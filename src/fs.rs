//! FUSE filesystem implementation for ScreenFS.
//!
//! This module orchestrates FUSE requests, state snapshots, policy guards, and
//! backing filesystem calls while keeping blocking host I/O outside the state lock.

use std::any::Any;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::File;
use std::future::Future;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileExt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll, Waker};
#[cfg(feature = "perf-counters")]
use std::time::Instant;

use fractal_fuse::abi::{FOPEN_NOFLUSH, fuse_dirent_size, fuse_direntplus_size};
use fractal_fuse::{
    DirectoryEntry, DirectoryEntryPlus, ENOENT, Filesystem, FsResult, ReplyAttr, ReplyCreate,
    ReplyEntry, ReplyOpen, ReplyReadlink, ReplyStatfs, ReplyXattr, Request, SetAttr,
};

use crate::config::{MutabilityDecision, RuntimeConfig};
use crate::errors::{errno_from_io, open_has_write_intent};
mod backing;
mod guards;
#[cfg(feature = "perf-counters")]
mod perf;
mod state;

use self::backing::{
    apply_setattr, cstring_os, faccessat2_empty, open_child_at, open_dir_handle, read_xattr_reply,
    sanitize_open_flags, source_root_statfs,
};
#[cfg(feature = "perf-counters")]
use self::perf::PerfCounters;
#[cfg(all(test, feature = "perf-counters"))]
use self::perf::PerfSnapshot;
use self::state::{DirectoryResume, DirectorySnapshotEntry, FileIoGuardCache, State};

#[cfg(feature = "perf-counters")]
macro_rules! fuse_op_timer {
    ($fs:expr, $name:literal) => {
        $fs.perf.fuse_op_timer($name)
    };
}

#[cfg(not(feature = "perf-counters"))]
macro_rules! fuse_op_timer {
    ($fs:expr, $name:literal) => {
        ()
    };
}

// ScreenFs owns long-lived config, source-root fd, and the single state domain.
#[derive(Debug)]
pub struct ScreenFs {
    cfg: RuntimeConfig,
    source_root: File,
    state: RwLock<State>,
    #[cfg(feature = "perf-counters")]
    perf: Arc<PerfCounters>,
}

type BlockingSyncResult = Result<FsResult<()>, Box<dyn Any + Send>>;

// Blocking sync operations are offloaded without holding the filesystem state lock.
struct ThreadOffload {
    state: Arc<Mutex<ThreadOffloadState>>,
}

struct ThreadOffloadState {
    result: Option<BlockingSyncResult>,
    waker: Option<Waker>,
}

impl ThreadOffload {
    fn spawn(sync: impl FnOnce() -> FsResult<()> + Send + 'static) -> std::io::Result<Self> {
        let state = Arc::new(Mutex::new(ThreadOffloadState {
            result: None,
            waker: None,
        }));
        let thread_state = Arc::clone(&state);
        std::thread::Builder::new().spawn(move || {
            let result = catch_unwind(AssertUnwindSafe(sync));
            let mut state = thread_state.lock().expect("thread offload mutex poisoned");
            state.result = Some(result);
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        })?;
        Ok(Self { state })
    }
}

impl Future for ThreadOffload {
    type Output = BlockingSyncResult;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().expect("thread offload mutex poisoned");
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// Construction and shared helpers used by request handlers.
impl ScreenFs {
    pub fn new(cfg: RuntimeConfig) -> Self {
        crate::ensure_non_root_user().expect("screenfs must be run as a non-root user");
        let source_root = open_dir_handle(&cfg.source_root).expect("source root must be openable");
        #[cfg(feature = "perf-counters")]
        let perf = Arc::new(PerfCounters::default());
        Self {
            cfg,
            source_root,
            state: RwLock::new(State::new()),
            #[cfg(feature = "perf-counters")]
            perf,
        }
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.cfg
    }

    fn open_file_io_guard_cache(&self) -> FileIoGuardCache {
        let skip_read_guard = self.cfg.can_skip_symlink_target_visibility_check();
        let skip_write_guard = skip_read_guard
            && self
                .cfg
                .can_skip_resolved_target_mutability_check(MutabilityDecision::Writable);
        FileIoGuardCache::new(skip_read_guard, skip_write_guard)
    }

    #[cfg(all(test, feature = "perf-counters"))]
    pub(super) fn perf_snapshot(&self) -> Option<PerfSnapshot> {
        Some(self.perf.snapshot())
    }

    #[cfg(not(feature = "perf-counters"))]
    fn with_state_read<T>(&self, f: impl FnOnce(&State) -> T) -> T {
        let state = self.state.read().expect("state rwlock poisoned");
        f(&state)
    }

    #[cfg(feature = "perf-counters")]
    fn with_state_read<T>(&self, f: impl FnOnce(&State) -> T) -> T {
        let perf = &self.perf;
        let wait_start = Instant::now();
        let state = self.state.read().expect("state rwlock poisoned");
        let wait = wait_start.elapsed();
        let hold_start = Instant::now();
        let result = f(&state);
        drop(state);
        perf.record_state_read_lock(wait, hold_start.elapsed());
        result
    }

    #[cfg(not(feature = "perf-counters"))]
    fn with_state_write<T>(&self, f: impl FnOnce(&mut State) -> T) -> T {
        let mut state = self.state.write().expect("state rwlock poisoned");
        f(&mut state)
    }

    #[cfg(feature = "perf-counters")]
    fn with_state_write<T>(&self, f: impl FnOnce(&mut State) -> T) -> T {
        let perf = &self.perf;
        let wait_start = Instant::now();
        let mut state = self.state.write().expect("state rwlock poisoned");
        let wait = wait_start.elapsed();
        let hold_start = Instant::now();
        let result = f(&mut state);
        drop(state);
        perf.record_state_write_lock(wait, hold_start.elapsed());
        result
    }

    #[cfg(feature = "perf-counters")]
    fn record_matcher_candidates_for_visibility(&self, path: &crate::path::VirtualPath) {
        self.perf.record_matcher_candidates(
            "path",
            self.cfg
                .internal_hidden_matcher
                .candidate_descriptor_metrics(path),
        );
        self.perf.record_matcher_candidates(
            "path",
            self.cfg.hidden_matcher.candidate_descriptor_metrics(path),
        );
        self.perf.record_matcher_candidates(
            "path",
            self.cfg.visible_matcher.candidate_descriptor_metrics(path),
        );
        self.perf.record_matcher_candidates(
            "descendant",
            self.cfg
                .visible_matcher
                .descendant_candidate_descriptor_metrics(path),
        );
    }

    #[cfg(feature = "perf-counters")]
    fn record_matcher_candidates_for_mutability(&self, path: &crate::path::VirtualPath) {
        self.perf.record_matcher_candidates(
            "path",
            self.cfg.readonly_matcher.candidate_descriptor_metrics(path),
        );
        self.perf.record_matcher_candidates(
            "path",
            self.cfg.writable_matcher.candidate_descriptor_metrics(path),
        );
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn visibility_decision(
        &self,
        path: &crate::path::VirtualPath,
    ) -> crate::config::VisibilityDecision {
        self.cfg.visibility_decision(path)
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn visibility_decision(
        &self,
        path: &crate::path::VirtualPath,
    ) -> crate::config::VisibilityDecision {
        let perf = &self.perf;
        self.record_matcher_candidates_for_visibility(path);
        let start = Instant::now();
        let decision = self.cfg.visibility_decision(path);
        perf.record_policy_decision(start.elapsed());
        decision
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn mutability_decision(
        &self,
        path: &crate::path::VirtualPath,
    ) -> crate::config::MutabilityDecision {
        self.cfg.mutability_decision(path)
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn mutability_decision(
        &self,
        path: &crate::path::VirtualPath,
    ) -> crate::config::MutabilityDecision {
        let perf = &self.perf;
        self.record_matcher_candidates_for_mutability(path);
        let start = Instant::now();
        let decision = self.cfg.mutability_decision(path);
        perf.record_policy_decision(start.elapsed());
        decision
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn is_fully_visible(&self, path: &crate::path::VirtualPath) -> bool {
        self.cfg.is_fully_visible(path)
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn is_fully_visible(&self, path: &crate::path::VirtualPath) -> bool {
        let perf = &self.perf;
        self.record_matcher_candidates_for_visibility(path);
        let start = Instant::now();
        let result = self.cfg.is_fully_visible(path);
        perf.record_policy_decision(start.elapsed());
        result
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn entry_is_readable(
        &self,
        path: &crate::path::VirtualPath,
        is_directory: bool,
    ) -> bool {
        self.cfg.entry_is_readable(path, is_directory)
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn entry_is_readable(
        &self,
        path: &crate::path::VirtualPath,
        is_directory: bool,
    ) -> bool {
        let perf = &self.perf;
        self.record_matcher_candidates_for_visibility(path);
        let start = Instant::now();
        let result = self.cfg.entry_is_readable(path, is_directory);
        perf.record_policy_decision(start.elapsed());
        result
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn is_hidden_symlink_target(
        &self,
        path: &crate::path::VirtualPath,
        target: &OsStr,
    ) -> bool {
        self.cfg.is_hidden_symlink_target(path, target)
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn is_hidden_symlink_target(
        &self,
        path: &crate::path::VirtualPath,
        target: &OsStr,
    ) -> bool {
        let perf = &self.perf;
        self.record_matcher_candidates_for_visibility(path);
        let start = Instant::now();
        let result = self.cfg.is_hidden_symlink_target(path, target);
        perf.record_policy_decision(start.elapsed());
        result
    }

    fn apply_deferred_truncate(&self, file: &File, flags: u32) -> FsResult<()> {
        if flags & libc::O_TRUNC as u32 == 0
            || flags & libc::O_ACCMODE as u32 == libc::O_RDONLY as u32
        {
            return Ok(());
        }
        let result = unsafe { libc::ftruncate(file.as_raw_fd(), 0) };
        if result == 0 {
            Ok(())
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }

    async fn offload_file_sync(
        file: Arc<File>,
        sync: impl FnOnce(Arc<File>) -> FsResult<()> + Send + 'static,
    ) -> FsResult<()> {
        let result = if compio_runtime::Runtime::try_with_current(|_| ()).is_ok() {
            compio_runtime::spawn_blocking(move || sync(file)).await
        } else {
            ThreadOffload::spawn(move || sync(file))
                .map_err(errno_from_io)?
                .await
        };
        result.map_err(|_| libc::EIO)?
    }

    fn fsync_fd(file: &File, datasync: bool) -> FsResult<()> {
        let result = unsafe {
            if datasync {
                libc::fdatasync(file.as_raw_fd())
            } else {
                libc::fsync(file.as_raw_fd())
            }
        };
        if result == 0 {
            Ok(())
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }
}

#[cfg(feature = "perf-counters")]
// Drop emits the optional perf summary; normal Rust ownership closes handles.
impl Drop for ScreenFs {
    fn drop(&mut self) {
        eprint!("{}", self.perf.summary());
    }
}

impl ScreenFs {
    fn directory_entry_wire_size(name_len: usize, with_plus: bool) -> usize {
        if with_plus {
            fuse_direntplus_size(name_len)
        } else {
            fuse_dirent_size(name_len)
        }
    }

    fn push_directory_page_entry(
        page: &mut Vec<DirectorySnapshotEntry>,
        remaining: &mut usize,
        entry: DirectorySnapshotEntry,
        with_plus: bool,
    ) -> bool {
        let entry_size = Self::directory_entry_wire_size(entry.name.len(), with_plus);
        if entry_size > *remaining {
            return false;
        }
        *remaining -= entry_size;
        page.push(entry);
        true
    }

    fn directory_page(
        &self,
        inode: u64,
        fh: u64,
        offset: u64,
        size: u32,
        with_plus: bool,
    ) -> FsResult<Vec<DirectorySnapshotEntry>> {
        let (path, resume) = self.directory_resume(inode, fh, offset)?;
        let mut remaining = size as usize;
        let mut page = Vec::new();

        if matches!(resume, DirectoryResume::Start) {
            let dot = DirectorySnapshotEntry {
                ino: inode,
                offset: 1,
                kind: fractal_fuse::FileType::Directory,
                name: b".".to_vec(),
                attr: self.attr_for_path(&path, inode)?,
                child: None,
            };
            if !Self::push_directory_page_entry(&mut page, &mut remaining, dot, with_plus) {
                return Ok(page);
            }
        }

        if matches!(resume, DirectoryResume::Start | DirectoryResume::AfterDot) {
            let parent = Self::parent_path(&path);
            let parent_ino = self.inode_for_visible_path(parent.clone());
            let dotdot = DirectorySnapshotEntry {
                ino: parent_ino,
                offset: 2,
                kind: fractal_fuse::FileType::Directory,
                name: b"..".to_vec(),
                attr: self.attr_for_path(&parent, parent_ino)?,
                child: None,
            };
            if !Self::push_directory_page_entry(&mut page, &mut remaining, dotdot, with_plus) {
                return Ok(page);
            }
        }

        let resume_name = match resume {
            DirectoryResume::Start | DirectoryResume::AfterDot | DirectoryResume::AfterDotDot => {
                None
            }
            DirectoryResume::AfterChild(name) => Some(name),
            DirectoryResume::End => return Ok(page),
        };
        if remaining > 0 {
            self.collect_child_directory_page(
                &path,
                resume_name.as_deref(),
                remaining,
                with_plus,
                &mut page,
            )?;
        }

        #[cfg(feature = "perf-counters")]
        let commit_start = Instant::now();
        let committed = self.commit_directory_page(inode, fh, page, with_plus)?;
        #[cfg(feature = "perf-counters")]
        {
            let elapsed = commit_start.elapsed();
            if with_plus {
                self.perf.record_readdirplus_page_commit(elapsed);
            } else {
                self.perf.record_readdir_page_commit(elapsed);
            }
        }
        Ok(committed)
    }

    fn collect_child_directory_page(
        &self,
        path: &crate::path::VirtualPath,
        resume_name: Option<&[u8]>,
        remaining: usize,
        with_plus: bool,
        page: &mut Vec<DirectorySnapshotEntry>,
    ) -> FsResult<()> {
        let min_entry_size = Self::directory_entry_wire_size(1, with_plus).max(1);
        let candidate_limit = (remaining / min_entry_size).saturating_add(1).max(1);
        let dir_file = self.open_confined(path, libc::O_RDONLY | libc::O_DIRECTORY, None)?;
        self.guard_opened_directory_target(path, &dir_file, false)?;
        let mut candidates = BTreeMap::new();
        #[cfg(feature = "perf-counters")]
        let mut symlink_visibility_elapsed = std::time::Duration::default();
        #[cfg(feature = "perf-counters")]
        let mut candidate_selection_elapsed = std::time::Duration::default();
        #[cfg(feature = "perf-counters")]
        let directory_scan_start = Instant::now();
        let scan = backing::visit_dir_entries(
            dir_file,
            path,
            3,
            with_plus,
            |name_bytes| resume_name.is_none_or(|resume| name_bytes > resume),
            |entry| {
                let name_bytes = entry.name.as_bytes();
                if !self.entry_is_readable(&entry.child, entry.is_dir) {
                    return Ok(());
                }
                if entry.is_symlink {
                    #[cfg(feature = "perf-counters")]
                    let symlink_visibility_start = Instant::now();
                    let visible = self
                        .guard_resolved_target_visibility_if_needed(&entry.child)
                        .is_ok();
                    #[cfg(feature = "perf-counters")]
                    {
                        symlink_visibility_elapsed += symlink_visibility_start.elapsed();
                    }
                    if !visible {
                        return Ok(());
                    }
                }
                #[cfg(feature = "perf-counters")]
                let candidate_selection_start = Instant::now();
                candidates.insert(name_bytes.to_vec(), entry);
                if candidates.len() > candidate_limit {
                    candidates.pop_last();
                }
                #[cfg(feature = "perf-counters")]
                {
                    candidate_selection_elapsed += candidate_selection_start.elapsed();
                }
                Ok(())
            },
        )?;
        #[cfg(not(feature = "perf-counters"))]
        let _ = scan;
        #[cfg(feature = "perf-counters")]
        {
            let directory_scan_elapsed = directory_scan_start.elapsed();
            if with_plus {
                self.perf
                    .record_readdirplus_directory_scan(directory_scan_elapsed);
                self.perf
                    .record_readdirplus_attr_generation(scan.attr_entries, scan.attr_generation);
                self.perf
                    .record_readdirplus_symlink_visibility(symlink_visibility_elapsed);
                self.perf
                    .record_readdirplus_candidate_selection(candidate_selection_elapsed);
            } else {
                self.perf
                    .record_readdir_directory_scan(directory_scan_elapsed);
                self.perf
                    .record_readdir_attr_generation(scan.attr_entries, scan.attr_generation);
                self.perf
                    .record_readdir_symlink_visibility(symlink_visibility_elapsed);
                self.perf
                    .record_readdir_candidate_selection(candidate_selection_elapsed);
            }
        }

        let mut remaining = remaining;
        for (_, entry) in candidates {
            let child = entry.child.clone();
            let snapshot_entry = DirectorySnapshotEntry {
                ino: 0,
                offset: 0,
                kind: entry.kind,
                name: entry.name.as_bytes().to_vec(),
                attr: entry.attr,
                child: Some(child),
            };
            if !Self::push_directory_page_entry(page, &mut remaining, snapshot_entry, with_plus) {
                break;
            }
        }
        Ok(())
    }
}

// FUSE request handlers live here; each method documents its own mix of state,
// guard, and backing-filesystem work because not all operations need every step.
impl Filesystem for ScreenFs {
    async fn lookup(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<ReplyEntry> {
        let _timer = fuse_op_timer!(self, "lookup");
        let path = self.child_path(parent, name)?;
        self.reply_entry_for_path(path)
    }

    fn forget(&self, _req: Request, inode: u64, nlookup: u64) {
        let _timer = fuse_op_timer!(self, "forget");
        self.forget_tracked_inode(inode, nlookup);
    }

    async fn getattr(
        &self,
        _req: Request,
        inode: u64,
        _fh: Option<u64>,
        _flags: u32,
    ) -> FsResult<ReplyAttr> {
        let _timer = fuse_op_timer!(self, "getattr");
        let path = self.path_for_inode(inode)?;
        Ok(ReplyAttr {
            ttl: self.cfg.attr_ttl,
            attr: self.attr_for_path(&path, inode)?,
        })
    }

    async fn readlink(&self, _req: Request, inode: u64) -> FsResult<ReplyReadlink> {
        let _timer = fuse_op_timer!(self, "readlink");
        let path = self.path_for_inode(inode)?;
        let mut resolver = guards::RequestPathResolver::new(self);
        let child = match self.prepare_readlink_child(&mut resolver, &path, inode) {
            Ok(child) => child,
            Err(err) => {
                self.guard_read_path_with_resolver(&mut resolver, &path, None)?;
                return Err(err);
            }
        };
        self.guard_read_path_with_resolver(&mut resolver, &path, Some(&child.attr))?;
        let target = self.readlink_prepared_child(&child)?;
        self.check_hidden_symlink_target(&path, target.as_os_str())?;
        Ok(ReplyReadlink {
            data: target.as_os_str().as_bytes().to_vec(),
        })
    }

    async fn open(&self, _req: Request, inode: u64, flags: u32) -> FsResult<ReplyOpen> {
        let _timer = fuse_op_timer!(self, "open");
        let path = self.path_for_inode(inode)?;
        let mut resolver = guards::RequestPathResolver::new(self);
        self.guard_open_flags_with_resolver(&mut resolver, &path, flags)?;
        let open_flags = sanitize_open_flags(flags, false) & !libc::O_TRUNC;
        let file = self.open_confined(&path, open_flags, None)?;
        self.guard_opened_file_target_with_resolver(
            &mut resolver,
            &path,
            &file,
            open_has_write_intent(flags),
        )?;
        self.apply_deferred_truncate(&file, flags)?;
        let write_intent = open_has_write_intent(flags);
        let fh = self.insert_open_file(
            inode,
            path,
            file,
            self.open_file_io_guard_cache(),
            write_intent,
        );
        Ok(ReplyOpen {
            fh,
            flags: if write_intent { 0 } else { FOPEN_NOFLUSH },
            backing_id: 0,
        })
    }

    async fn read(
        &self,
        _req: Request,
        inode: u64,
        fh: u64,
        offset: u64,
        buf: &mut [u8],
    ) -> FsResult<usize> {
        let _timer = fuse_op_timer!(self, "read");
        #[cfg(feature = "perf-counters")]
        let snapshot_start = Instant::now();
        let snapshot = self.file_data_path_snapshot(inode, fh);
        #[cfg(feature = "perf-counters")]
        self.perf
            .record_read_handle_snapshot(snapshot_start.elapsed());
        let (path, file, io_guard_cache) = snapshot?;
        #[cfg(feature = "perf-counters")]
        let guard_start = Instant::now();
        let guard = if io_guard_cache.skip_read_guard {
            Ok(())
        } else {
            self.guard_read_path(&path)
                .and_then(|()| self.guard_opened_file_target(&path, &file, false))
        };
        #[cfg(feature = "perf-counters")]
        self.perf.record_read_guard_path(guard_start.elapsed());
        guard?;
        #[cfg(feature = "perf-counters")]
        let io_start = Instant::now();
        let result = file.read_at(buf, offset).map_err(errno_from_io);
        #[cfg(feature = "perf-counters")]
        {
            let io_elapsed = io_start.elapsed();
            self.perf.record_read_io(io_elapsed);
            if let Ok(size) = result {
                self.perf.record_read_size_bucket(size, io_elapsed);
            }
        }
        result
    }

    async fn write(
        &self,
        _req: Request,
        inode: u64,
        fh: u64,
        offset: u64,
        data: &[u8],
        _write_flags: u32,
        _flags: u32,
    ) -> FsResult<usize> {
        let _timer = fuse_op_timer!(self, "write");
        #[cfg(feature = "perf-counters")]
        let snapshot_start = Instant::now();
        let snapshot = self.file_data_path_snapshot(inode, fh);
        #[cfg(feature = "perf-counters")]
        self.perf
            .record_write_handle_snapshot(snapshot_start.elapsed());
        let (path, file, io_guard_cache) = snapshot?;
        #[cfg(feature = "perf-counters")]
        let guard_start = Instant::now();
        let guard = if io_guard_cache.skip_write_guard {
            Ok(())
        } else {
            self.guard_mutation_path(&path, true)
                .and_then(|()| self.guard_opened_file_target(&path, &file, true))
        };
        #[cfg(feature = "perf-counters")]
        self.perf.record_write_guard_mutation(guard_start.elapsed());
        guard?;
        #[cfg(feature = "perf-counters")]
        let io_start = Instant::now();
        let result = file.write_at(data, offset).map_err(errno_from_io);
        #[cfg(feature = "perf-counters")]
        {
            let io_elapsed = io_start.elapsed();
            self.perf.record_write_io(io_elapsed);
            if let Ok(size) = result {
                self.perf.record_write_size_bucket(size, io_elapsed);
            }
        }
        result
    }

    async fn flush(&self, _req: Request, inode: u64, fh: u64, _lock_owner: u64) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "flush");
        let (_path, file, flush_needs_sync) = self.file_flush_snapshot(inode, fh)?;
        if !flush_needs_sync {
            return Ok(());
        }
        #[cfg(feature = "perf-counters")]
        let sync_start = Instant::now();
        let result =
            Self::offload_file_sync(file, |file| file.sync_all().map_err(errno_from_io)).await;
        #[cfg(feature = "perf-counters")]
        self.perf.record_file_sync("flush", sync_start.elapsed());
        result
    }

    async fn release(
        &self,
        _req: Request,
        _inode: u64,
        fh: u64,
        _flags: u32,
        _lock_owner: u64,
        flush: bool,
        _flock_release: bool,
    ) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "release");
        let handle = self.with_state_write(|state| state.remove_file(fh));
        if flush
            && let Some(handle) = handle
            && handle.flush_needs_sync
        {
            #[cfg(feature = "perf-counters")]
            let sync_start = Instant::now();
            let result =
                Self::offload_file_sync(handle.file, |file| file.sync_all().map_err(errno_from_io))
                    .await;
            #[cfg(feature = "perf-counters")]
            self.perf
                .record_file_sync("release_flush", sync_start.elapsed());
            result?;
        }
        Ok(())
    }

    async fn fsync(&self, _req: Request, inode: u64, fh: u64, datasync: bool) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "fsync");
        let (_path, file) = self.file_handle_snapshot(inode, fh)?;
        #[cfg(feature = "perf-counters")]
        let sync_start = Instant::now();
        let result =
            Self::offload_file_sync(file, move |file| Self::fsync_fd(file.as_ref(), datasync))
                .await;
        #[cfg(feature = "perf-counters")]
        self.perf.record_file_sync("fsync", sync_start.elapsed());
        result
    }

    async fn opendir(&self, _req: Request, inode: u64, flags: u32) -> FsResult<ReplyOpen> {
        let _timer = fuse_op_timer!(self, "opendir");
        let path = self.path_for_inode(inode)?;
        let mut resolver = guards::RequestPathResolver::new(self);
        self.guard_open_flags_with_resolver(&mut resolver, &path, flags)?;
        let dir_file = self.open_confined(&path, libc::O_PATH | libc::O_DIRECTORY, None)?;
        self.guard_opened_directory_target_with_resolver(&mut resolver, &path, &dir_file, false)?;
        let fh = self.insert_open_directory(inode, path);
        Ok(ReplyOpen {
            fh,
            flags: 0,
            backing_id: 0,
        })
    }

    async fn readdir(
        &self,
        _req: Request,
        inode: u64,
        fh: u64,
        offset: u64,
        size: u32,
    ) -> FsResult<Vec<DirectoryEntry>> {
        let _timer = fuse_op_timer!(self, "readdir");
        Ok(self
            .directory_page(inode, fh, offset, size, false)?
            .into_iter()
            .map(|entry| DirectoryEntry {
                ino: entry.ino,
                offset: entry.offset,
                kind: entry.kind,
                name: entry.name,
            })
            .collect())
    }

    async fn readdirplus(
        &self,
        _req: Request,
        inode: u64,
        fh: u64,
        offset: u64,
        size: u32,
    ) -> FsResult<Vec<DirectoryEntryPlus>> {
        let _timer = fuse_op_timer!(self, "readdirplus");
        let entries = self.directory_page(inode, fh, offset, size, true)?;
        Ok(entries
            .into_iter()
            .map(|entry| DirectoryEntryPlus {
                ino: entry.ino,
                offset: entry.offset,
                kind: entry.kind,
                name: entry.name,
                entry_ttl: self.cfg.entry_ttl,
                attr: entry.attr,
                generation: 0,
            })
            .collect())
    }

    async fn releasedir(&self, _req: Request, _inode: u64, fh: u64, _flags: u32) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "releasedir");
        self.remove_open_directory(fh);
        Ok(())
    }

    async fn access(&self, _req: Request, inode: u64, mask: u32) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "access");
        let path = self.path_for_inode(inode)?;
        let mut resolver = guards::RequestPathResolver::new(self);
        self.guard_access_mask_with_resolver(&mut resolver, &path, mask)?;
        let file = self.open_confined(&path, libc::O_PATH, None)?;
        self.guard_opened_file_target_with_resolver(
            &mut resolver,
            &path,
            &file,
            mask & libc::W_OK as u32 != 0,
        )?;
        faccessat2_empty(&file, mask)
    }

    async fn statfs(&self, _req: Request, _inode: u64) -> FsResult<ReplyStatfs> {
        let _timer = fuse_op_timer!(self, "statfs");
        source_root_statfs(&self.source_root)
    }

    async fn setattr(
        &self,
        _req: Request,
        inode: u64,
        _fh: Option<u64>,
        set_attr: SetAttr,
    ) -> FsResult<ReplyAttr> {
        let _timer = fuse_op_timer!(self, "setattr");
        let path = self.path_for_inode(inode)?;
        self.guard_mutation_path(&path, true)?;
        let flags = if set_attr.size.is_some() {
            libc::O_WRONLY
        } else {
            libc::O_RDONLY
        };
        let file = self.open_confined(&path, flags, None)?;
        self.guard_opened_file_target(&path, &file, true)?;
        apply_setattr(&file, set_attr)?;
        Ok(ReplyAttr {
            ttl: self.cfg.attr_ttl,
            attr: self.attr_for_path(&path, inode)?,
        })
    }

    async fn symlink(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        link: &OsStr,
    ) -> FsResult<ReplyEntry> {
        let _timer = fuse_op_timer!(self, "symlink");
        let path = self.child_path(parent, name)?;
        if self.is_hidden_symlink_target(&path, link) {
            return Err(ENOENT);
        }
        let parent_path = self.guard_child_mutation_path(&path)?;
        let (_parent, parent_dir, name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let link = cstring_os(link)?;
        let result =
            unsafe { libc::symlinkat(link.as_ptr(), parent_dir.as_raw_fd(), name.as_ptr()) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[parent_path], std::slice::from_ref(&path), &[]);
        match self.reply_entry_for_path(path.clone()) {
            Ok(entry) => Ok(entry),
            Err(err) => {
                let _ = unsafe { libc::unlinkat(parent_dir.as_raw_fd(), name.as_ptr(), 0) };
                self.invalidate_after_mutation(&[], std::slice::from_ref(&path), &[]);
                Err(err)
            }
        }
    }

    async fn mknod(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        rdev: u32,
    ) -> FsResult<ReplyEntry> {
        let _timer = fuse_op_timer!(self, "mknod");
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let (_parent, parent_dir, name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let result = unsafe {
            libc::mknodat(
                parent_dir.as_raw_fd(),
                name.as_ptr(),
                mode as libc::mode_t,
                rdev as libc::dev_t,
            )
        };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[parent_path], std::slice::from_ref(&path), &[]);
        self.reply_entry_for_path(path)
    }

    async fn unlink(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "unlink");
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let (_parent, parent_dir, name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let result = unsafe { libc::unlinkat(parent_dir.as_raw_fd(), name.as_ptr(), 0) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[parent_path], &[], &[path]);
        Ok(())
    }

    async fn rmdir(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "rmdir");
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let (_parent, parent_dir, name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let result =
            unsafe { libc::unlinkat(parent_dir.as_raw_fd(), name.as_ptr(), libc::AT_REMOVEDIR) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[parent_path], &[], &[path]);
        Ok(())
    }

    async fn mkdir(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        _umask: u32,
    ) -> FsResult<ReplyEntry> {
        let _timer = fuse_op_timer!(self, "mkdir");
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let (_parent, parent_dir, name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let result = unsafe {
            libc::mkdirat(
                parent_dir.as_raw_fd(),
                name.as_ptr(),
                ((mode & 0o7777) | 0o700) as libc::mode_t,
            )
        };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        let child_fd = unsafe {
            libc::openat(
                parent_dir.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if child_fd < 0 {
            let err = errno_from_io(std::io::Error::last_os_error());
            let _ = unsafe {
                libc::unlinkat(parent_dir.as_raw_fd(), name.as_ptr(), libc::AT_REMOVEDIR)
            };
            self.invalidate_after_mutation(&[parent_path], &[], std::slice::from_ref(&path));
            return Err(err);
        }
        let chmod_result = unsafe { libc::fchmod(child_fd, (mode & 0o7777) as libc::mode_t) };
        let close_result = unsafe { libc::close(child_fd) };
        if chmod_result != 0 || close_result != 0 {
            let err = errno_from_io(std::io::Error::last_os_error());
            let _ = unsafe {
                libc::unlinkat(parent_dir.as_raw_fd(), name.as_ptr(), libc::AT_REMOVEDIR)
            };
            self.invalidate_after_mutation(&[parent_path], &[], std::slice::from_ref(&path));
            return Err(err);
        }
        self.invalidate_after_mutation(&[parent_path], std::slice::from_ref(&path), &[]);
        self.reply_entry_for_path(path)
    }

    async fn rename(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        new_parent: u64,
        new_name: &OsStr,
        _flags: u32,
    ) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "rename");
        let from = self.child_path(parent, name)?;
        let to = self.child_path(new_parent, new_name)?;
        self.guard_existing_entry_target_visibility(&from)?;
        self.guard_existing_entry_target_visibility(&to)?;
        let from_parent = Self::parent_path(&from);
        let to_parent = Self::parent_path(&to);
        self.guard_multi_path_mutation(&[
            (&from, false),
            (&to, false),
            (&from_parent, true),
            (&to_parent, true),
        ])?;
        let (_from_parent, from_parent_dir, from_name) = self.open_parent_dir(&from)?;
        let (_to_parent, to_parent_dir, to_name) = self.open_parent_dir(&to)?;
        self.guard_opened_directory_at_path(&from_parent, &from_parent_dir, true)?;
        self.guard_opened_directory_at_path(&to_parent, &to_parent_dir, true)?;
        let result = unsafe {
            libc::syscall(
                libc::SYS_renameat2,
                from_parent_dir.as_raw_fd(),
                from_name.as_ptr(),
                to_parent_dir.as_raw_fd(),
                to_name.as_ptr(),
                0,
            ) as libc::c_int
        };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[from_parent, to_parent], &[], &[from, to]);
        Ok(())
    }

    async fn link(
        &self,
        _req: Request,
        inode: u64,
        new_parent: u64,
        new_name: &OsStr,
    ) -> FsResult<ReplyEntry> {
        let _timer = fuse_op_timer!(self, "link");
        let source = self.path_for_inode(inode)?;
        let target = self.child_path(new_parent, new_name)?;
        self.guard_existing_entry_target_visibility(&source)?;
        self.guard_existing_entry_target_visibility(&target)?;
        let target_parent = Self::parent_path(&target);
        self.guard_multi_path_mutation(&[
            (&source, false),
            (&target, false),
            (&target_parent, true),
        ])?;
        let source_parent = Self::parent_path(&source);
        let (_source_parent, source_parent_dir, source_name) = self.open_parent_dir(&source)?;
        let (_target_parent, target_parent_dir, target_name) = self.open_parent_dir(&target)?;
        self.guard_opened_directory_at_path(&source_parent, &source_parent_dir, false)?;
        self.guard_opened_directory_at_path(&target_parent, &target_parent_dir, true)?;
        let result = unsafe {
            libc::linkat(
                source_parent_dir.as_raw_fd(),
                source_name.as_ptr(),
                target_parent_dir.as_raw_fd(),
                target_name.as_ptr(),
                0,
            )
        };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[target_parent], std::slice::from_ref(&target), &[]);
        self.reply_entry_for_path(target)
    }

    async fn create(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32,
    ) -> FsResult<ReplyCreate> {
        let _timer = fuse_op_timer!(self, "create");
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let existed = self.stat_child_no_follow(&path, 0).is_ok();
        if existed {
            self.guard_mutation_path(&path, true)?;
        }
        let (_parent, parent_dir, child_name) = self.open_parent_dir(&path)?;
        self.guard_opened_directory_at_path(&parent_path, &parent_dir, true)?;
        let open_flags = (sanitize_open_flags(flags, true) & !libc::O_TRUNC) | libc::O_NOFOLLOW;
        let file = match open_child_at(&parent_dir, &child_name, open_flags, Some(mode & 0o7777)) {
            Ok(file) => file,
            Err(libc::ELOOP) => return Err(ENOENT),
            Err(err) => return Err(err),
        };
        if let Err(err) = self.guard_opened_file_target(&path, &file, true) {
            if !existed {
                let _ = unsafe { libc::unlinkat(parent_dir.as_raw_fd(), child_name.as_ptr(), 0) };
            }
            return Err(err);
        }
        if let Err(err) = self.apply_deferred_truncate(&file, flags) {
            if !existed {
                let _ = unsafe { libc::unlinkat(parent_dir.as_raw_fd(), child_name.as_ptr(), 0) };
            }
            return Err(err);
        }
        let (inode, fh) = self.finalize_created_file(&parent_path, path.clone(), file);
        let attr = self.attr_for_path(&path, inode)?;
        Ok(ReplyCreate {
            ttl: self.cfg.entry_ttl,
            attr,
            generation: 0,
            fh,
            // FUSE create replies use FOPEN_* reply flags, not the original
            // open(2) flags. ScreenFS does not request special handle behavior.
            flags: 0,
        })
    }

    async fn fallocate(
        &self,
        _req: Request,
        inode: u64,
        fh: u64,
        offset: u64,
        length: u64,
        mode: u32,
    ) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "fallocate");
        let (path, file) = self.file_handle_snapshot(inode, fh)?;
        self.guard_mutation_path(&path, true)?;
        self.guard_opened_file_target(&path, &file, true)?;
        let result = unsafe {
            libc::fallocate(
                file.as_raw_fd(),
                mode as i32,
                offset as libc::off_t,
                length as libc::off_t,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }

    async fn lseek(
        &self,
        _req: Request,
        _inode: u64,
        fh: u64,
        offset: u64,
        whence: u32,
    ) -> FsResult<u64> {
        let _timer = fuse_op_timer!(self, "lseek");
        let file = self.file_snapshot_for_handle(fh)?;
        let result = unsafe { libc::lseek(file.as_raw_fd(), offset as libc::off_t, whence as i32) };
        if result >= 0 {
            Ok(result as u64)
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }

    async fn copy_file_range(
        &self,
        _req: Request,
        inode_in: u64,
        fh_in: u64,
        off_in: u64,
        inode_out: u64,
        fh_out: u64,
        off_out: u64,
        length: u64,
        flags: u64,
    ) -> FsResult<usize> {
        let _timer = fuse_op_timer!(self, "copy_file_range");
        let ((input_path, input_file), (output_path, output_file)) =
            self.copy_file_range_snapshot(inode_in, fh_in, inode_out, fh_out)?;
        self.guard_read_path(&input_path)?;
        self.guard_opened_file_target(&input_path, &input_file, false)?;
        let output_parent = Self::parent_path(&output_path);
        self.guard_mutation_coordinates(&[], &[(&output_path, true), (&output_parent, true)])?;
        self.guard_opened_file_target(&output_path, &output_file, true)?;
        let mut in_off = off_in as libc::off64_t;
        let mut out_off = off_out as libc::off64_t;
        let copied = unsafe {
            libc::copy_file_range(
                input_file.as_raw_fd(),
                &mut in_off,
                output_file.as_raw_fd(),
                &mut out_off,
                length as usize,
                flags as u32,
            )
        };
        if copied >= 0 {
            Ok(copied as usize)
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }

    async fn getxattr(
        &self,
        _req: Request,
        inode: u64,
        name: &OsStr,
        size: u32,
    ) -> FsResult<ReplyXattr> {
        let _timer = fuse_op_timer!(self, "getxattr");
        let file = self.xattr_file(inode, false)?;
        let c_name = cstring_os(name)?;
        read_xattr_reply(
            size,
            || {
                let needed = unsafe {
                    libc::fgetxattr(file.as_raw_fd(), c_name.as_ptr(), std::ptr::null_mut(), 0)
                };
                if needed < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(needed as usize)
                }
            },
            |data| {
                let read = unsafe {
                    libc::fgetxattr(
                        file.as_raw_fd(),
                        c_name.as_ptr(),
                        data.as_mut_ptr().cast(),
                        data.len(),
                    )
                };
                if read < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(read as usize)
                }
            },
        )
    }

    async fn listxattr(&self, _req: Request, inode: u64, size: u32) -> FsResult<ReplyXattr> {
        let _timer = fuse_op_timer!(self, "listxattr");
        let file = self.xattr_file(inode, false)?;
        read_xattr_reply(
            size,
            || {
                let needed = unsafe { libc::flistxattr(file.as_raw_fd(), std::ptr::null_mut(), 0) };
                if needed < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(needed as usize)
                }
            },
            |data| {
                let read = unsafe {
                    libc::flistxattr(file.as_raw_fd(), data.as_mut_ptr().cast(), data.len())
                };
                if read < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(read as usize)
                }
            },
        )
    }

    async fn setxattr(
        &self,
        _req: Request,
        inode: u64,
        name: &OsStr,
        value: &[u8],
        flags: u32,
    ) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "setxattr");
        let file = self.xattr_file(inode, true)?;
        let c_name = cstring_os(name)?;
        let result = unsafe {
            libc::fsetxattr(
                file.as_raw_fd(),
                c_name.as_ptr(),
                value.as_ptr().cast(),
                value.len(),
                flags as i32,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }

    async fn removexattr(&self, _req: Request, inode: u64, name: &OsStr) -> FsResult<()> {
        let _timer = fuse_op_timer!(self, "removexattr");
        let file = self.xattr_file(inode, true)?;
        let c_name = cstring_os(name)?;
        let result = unsafe { libc::fremovexattr(file.as_raw_fd(), c_name.as_ptr()) };
        if result == 0 {
            Ok(())
        } else {
            Err(errno_from_io(std::io::Error::last_os_error()))
        }
    }
}

#[cfg(test)]
#[path = "fs/tests.rs"]
mod tests;
