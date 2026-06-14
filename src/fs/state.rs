use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::File;
use std::sync::Arc;

use fractal_fuse::abi::FUSE_ROOT_ID;
use fractal_fuse::{ENOENT, FileAttr, FileType};

use crate::path::VirtualPath;

use super::ScreenFs;
#[cfg(feature = "perf-counters")]
use super::perf::InvalidationStats;

#[derive(Debug)]
pub(super) struct State {
    next_ino: u64,
    next_fh: u64,
    pub(super) inodes: HashMap<u64, InodeRecord>,
    pub(super) path_inodes: BTreeMap<VirtualPath, u64>,
    pub(super) files: HashMap<u64, FileHandle>,
    pub(super) directories: HashMap<u64, DirectoryHandle>,
}

#[derive(Debug)]
pub(super) struct InodeRecord {
    pub(super) path: Option<VirtualPath>,
    pub(super) lookup_refs: u64,
    pub(super) open_refs: u64,
}

#[derive(Debug)]
pub(super) struct FileHandle {
    pub(super) inode: u64,
    pub(super) path: VirtualPath,
    pub(super) file: Arc<File>,
}

pub(super) type FileSnapshot = (VirtualPath, Arc<File>);
pub(super) type CopyFileRangeSnapshot = (FileSnapshot, FileSnapshot);

#[derive(Debug, Clone)]
pub(super) struct DirectorySnapshotEntry {
    pub(super) ino: u64,
    pub(super) offset: u64,
    pub(super) kind: FileType,
    pub(super) name: Vec<u8>,
    pub(super) attr: FileAttr,
    pub(super) child: Option<VirtualPath>,
}

#[derive(Debug, Clone)]
pub(super) enum DirectoryResume {
    Start,
    AfterDot,
    AfterDotDot,
    AfterChild(Vec<u8>),
    End,
}

#[derive(Debug)]
pub(super) struct DirectoryHandle {
    pub(super) inode: u64,
    pub(super) path: VirtualPath,
    next_cookie: u64,
    child_cookies: BTreeMap<Vec<u8>, u64>,
    page_cookies: BTreeMap<u64, Vec<u8>>,
}

impl State {
    pub(super) fn new() -> Self {
        let root = VirtualPath::root();
        let mut inodes = HashMap::new();
        let mut path_inodes = BTreeMap::new();
        inodes.insert(
            FUSE_ROOT_ID,
            InodeRecord {
                path: Some(root.clone()),
                lookup_refs: 0,
                open_refs: 0,
            },
        );
        path_inodes.insert(root, FUSE_ROOT_ID);
        Self {
            next_ino: FUSE_ROOT_ID + 1,
            next_fh: 1,
            inodes,
            path_inodes,
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub(super) fn path_for_inode(&self, inode: u64) -> Option<VirtualPath> {
        self.inodes
            .get(&inode)
            .and_then(|record| record.path.clone())
    }

    pub(super) fn inode_for_path(&mut self, path: VirtualPath) -> u64 {
        if let Some(inode) = self.path_inodes.get(&path) {
            return *inode;
        }
        let inode = self.next_ino;
        self.next_ino += 1;
        self.path_inodes.insert(path.clone(), inode);
        self.inodes.insert(
            inode,
            InodeRecord {
                path: Some(path),
                lookup_refs: 0,
                open_refs: 0,
            },
        );
        inode
    }

    pub(super) fn lookup_path(&mut self, path: VirtualPath) -> u64 {
        let inode = self.inode_for_path(path);
        self.add_lookup_ref(inode);
        inode
    }

    pub(super) fn add_lookup_ref(&mut self, inode: u64) {
        if inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&inode)
        {
            record.lookup_refs += 1;
        }
    }

    pub(super) fn add_lookup_refs(&mut self, inodes: impl IntoIterator<Item = u64>) {
        for inode in inodes {
            self.add_lookup_ref(inode);
        }
    }

    fn next_handle(&mut self) -> u64 {
        let fh = self.next_fh;
        self.next_fh += 1;
        fh
    }

    fn try_evict_inode(&mut self, inode: u64) -> bool {
        if inode == FUSE_ROOT_ID {
            return false;
        }
        let should_remove = self
            .inodes
            .get(&inode)
            .map(|record| record.lookup_refs == 0 && record.open_refs == 0)
            .unwrap_or(false);
        if !should_remove {
            return false;
        }
        let mut evicted = false;
        if let Some(record) = self.inodes.remove(&inode) {
            evicted = true;
            if let Some(path) = record.path
                && self.path_inodes.get(&path) == Some(&inode)
            {
                self.path_inodes.remove(&path);
            }
        }
        evicted
    }

    pub(super) fn forget_inode(&mut self, inode: u64, nlookup: u64) {
        if inode == FUSE_ROOT_ID {
            return;
        }
        if let Some(record) = self.inodes.get_mut(&inode) {
            record.lookup_refs = record.lookup_refs.saturating_sub(nlookup);
        }
        self.try_evict_inode(inode);
    }

    pub(super) fn insert_file(&mut self, inode: u64, path: VirtualPath, file: File) -> u64 {
        let fh = self.next_handle();
        self.files.insert(
            fh,
            FileHandle {
                inode,
                path,
                file: Arc::new(file),
            },
        );
        if inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&inode)
        {
            record.open_refs += 1;
        }
        fh
    }

    pub(super) fn remove_file(&mut self, fh: u64) -> Option<FileHandle> {
        let handle = self.files.remove(&fh)?;
        if handle.inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&handle.inode)
        {
            record.open_refs = record.open_refs.saturating_sub(1);
        }
        self.try_evict_inode(handle.inode);
        Some(handle)
    }

    pub(super) fn insert_directory(&mut self, inode: u64, path: VirtualPath) -> u64 {
        let fh = self.next_handle();
        self.directories.insert(
            fh,
            DirectoryHandle {
                inode,
                path,
                next_cookie: 3,
                child_cookies: BTreeMap::new(),
                page_cookies: BTreeMap::new(),
            },
        );
        if inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&inode)
        {
            record.open_refs += 1;
        }
        fh
    }

    pub(super) fn remove_directory(&mut self, fh: u64) -> Option<DirectoryHandle> {
        let handle = self.directories.remove(&fh)?;
        if handle.inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&handle.inode)
        {
            record.open_refs = record.open_refs.saturating_sub(1);
        }
        self.try_evict_inode(handle.inode);
        Some(handle)
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn invalidate_directory_snapshots(&mut self, parents: &[VirtualPath]) {
        let handles = self
            .directories
            .iter()
            .filter_map(|(fh, handle)| parents.contains(&handle.path).then_some(*fh))
            .collect::<Vec<_>>();
        for fh in handles {
            self.remove_directory(fh);
        }
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn invalidate_directory_snapshots(
        &mut self,
        parents: &[VirtualPath],
    ) -> InvalidationStats {
        let handles = self
            .directories
            .iter()
            .filter_map(|(fh, handle)| parents.contains(&handle.path).then_some(*fh))
            .collect::<Vec<_>>();
        let mut stats = InvalidationStats::default();
        for fh in handles {
            if self.remove_directory(fh).is_some() {
                stats.invalidated_entries += 1;
            }
        }
        stats
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn invalidate_exact_path(&mut self, path: &VirtualPath) {
        let Some(inode) = self.path_inodes.remove(path) else {
            return;
        };
        if let Some(record) = self.inodes.get_mut(&inode)
            && record.path.as_ref() == Some(path)
        {
            record.path = None;
        }
        self.try_evict_inode(inode);
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn invalidate_exact_path(&mut self, path: &VirtualPath) -> InvalidationStats {
        let Some(inode) = self.path_inodes.remove(path) else {
            return InvalidationStats::default();
        };
        let mut stats = InvalidationStats {
            invalidated_entries: 1,
            evicted_entries: 0,
        };
        if let Some(record) = self.inodes.get_mut(&inode)
            && record.path.as_ref() == Some(path)
        {
            record.path = None;
        }
        if self.try_evict_inode(inode) {
            stats.evicted_entries += 1;
        }
        stats
    }

    #[cfg(not(feature = "perf-counters"))]
    pub(super) fn invalidate_path_tree(&mut self, root: &VirtualPath) {
        let paths = self
            .path_inodes
            .keys()
            .filter(|path| path.starts_with(root))
            .cloned()
            .collect::<Vec<_>>();
        for path in paths {
            self.invalidate_exact_path(&path);
        }
    }

    #[cfg(feature = "perf-counters")]
    pub(super) fn invalidate_path_tree(&mut self, root: &VirtualPath) -> InvalidationStats {
        let paths = self
            .path_inodes
            .keys()
            .filter(|path| path.starts_with(root))
            .cloned()
            .collect::<Vec<_>>();
        let mut stats = InvalidationStats::default();
        for path in paths {
            stats += self.invalidate_exact_path(&path);
        }
        stats
    }

    pub(super) fn directory_resume(
        &self,
        inode: u64,
        fh: u64,
        offset: u64,
    ) -> Result<(VirtualPath, DirectoryResume), i32> {
        let handle = self.directories.get(&fh).ok_or(ENOENT)?;
        if handle.inode != inode {
            return Err(ENOENT);
        }
        let resume = match offset {
            0 => DirectoryResume::Start,
            1 => DirectoryResume::AfterDot,
            2 => DirectoryResume::AfterDotDot,
            offset => handle
                .page_cookies
                .get(&offset)
                .cloned()
                .map(DirectoryResume::AfterChild)
                .unwrap_or(DirectoryResume::End),
        };
        Ok((handle.path.clone(), resume))
    }

    pub(super) fn commit_directory_page(
        &mut self,
        inode: u64,
        fh: u64,
        mut entries: Vec<DirectorySnapshotEntry>,
        pin_lookup_refs: bool,
    ) -> Result<Vec<DirectorySnapshotEntry>, i32> {
        {
            let handle = self.directories.get(&fh).ok_or(ENOENT)?;
            if handle.inode != inode {
                return Err(ENOENT);
            }
        }

        for entry in &mut entries {
            if let Some(child) = &entry.child {
                let ino = self.inode_for_path(child.clone());
                entry.ino = ino;
                entry.attr.ino = ino;
            }
        }

        if pin_lookup_refs {
            let inodes = entries
                .iter()
                .filter(|entry| entry.child.is_some())
                .map(|entry| entry.ino)
                .collect::<Vec<_>>();
            self.add_lookup_refs(inodes);
        }

        let handle = self.directories.get_mut(&fh).ok_or(ENOENT)?;
        if handle.inode != inode {
            return Err(ENOENT);
        }
        for entry in &mut entries {
            if entry.child.is_some() {
                let cookie = if let Some(cookie) = handle.child_cookies.get(&entry.name) {
                    *cookie
                } else {
                    let cookie = handle.next_cookie;
                    handle.next_cookie += 1;
                    handle.child_cookies.insert(entry.name.clone(), cookie);
                    cookie
                };
                entry.offset = cookie;
                handle.page_cookies.insert(cookie, entry.name.clone());
            }
        }
        Ok(entries)
    }
}

impl ScreenFs {
    pub(super) fn path_for_inode(&self, inode: u64) -> Result<VirtualPath, i32> {
        self.with_state_read(|state| state.path_for_inode(inode).ok_or(ENOENT))
    }

    pub(super) fn child_path(&self, parent: u64, name: &OsStr) -> Result<VirtualPath, i32> {
        let parent = self.path_for_inode(parent)?;
        Ok(parent.join_child(name))
    }

    pub(super) fn track_path(&self, path: VirtualPath) -> u64 {
        self.with_state_write(|state| state.lookup_path(path))
    }

    pub(super) fn inode_for_visible_path(&self, path: VirtualPath) -> u64 {
        self.with_state_write(|state| state.inode_for_path(path))
    }

    pub(super) fn insert_open_file(&self, inode: u64, path: VirtualPath, file: File) -> u64 {
        self.with_state_write(|state| state.insert_file(inode, path, file))
    }

    pub(super) fn insert_open_directory(&self, inode: u64, path: VirtualPath) -> u64 {
        self.with_state_write(|state| state.insert_directory(inode, path))
    }

    pub(super) fn remove_open_directory(&self, fh: u64) {
        self.with_state_write(|state| {
            state.remove_directory(fh);
        });
    }

    pub(super) fn file_handle_snapshot(&self, inode: u64, fh: u64) -> Result<FileSnapshot, i32> {
        self.with_state_read(|state| {
            let handle = state.files.get(&fh).ok_or(ENOENT)?;
            if handle.inode != inode {
                return Err(ENOENT);
            }
            Ok((handle.path.clone(), Arc::clone(&handle.file)))
        })
    }

    pub(super) fn file_snapshot_for_handle(&self, fh: u64) -> Result<Arc<File>, i32> {
        self.with_state_read(|state| {
            let handle = state.files.get(&fh).ok_or(ENOENT)?;
            Ok(Arc::clone(&handle.file))
        })
    }

    pub(super) fn copy_file_range_snapshot(
        &self,
        inode_in: u64,
        fh_in: u64,
        inode_out: u64,
        fh_out: u64,
    ) -> Result<CopyFileRangeSnapshot, i32> {
        self.with_state_read(|state| {
            let input = state.files.get(&fh_in).ok_or(ENOENT)?;
            let output = state.files.get(&fh_out).ok_or(ENOENT)?;
            if input.inode != inode_in || output.inode != inode_out {
                return Err(ENOENT);
            }
            Ok((
                (input.path.clone(), Arc::clone(&input.file)),
                (output.path.clone(), Arc::clone(&output.file)),
            ))
        })
    }

    pub(super) fn directory_resume(
        &self,
        inode: u64,
        fh: u64,
        offset: u64,
    ) -> Result<(VirtualPath, DirectoryResume), i32> {
        self.with_state_read(|state| state.directory_resume(inode, fh, offset))
    }

    pub(super) fn commit_directory_page(
        &self,
        inode: u64,
        fh: u64,
        entries: Vec<DirectorySnapshotEntry>,
        pin_lookup_refs: bool,
    ) -> Result<Vec<DirectorySnapshotEntry>, i32> {
        self.with_state_write(|state| {
            state.commit_directory_page(inode, fh, entries, pin_lookup_refs)
        })
    }

    pub(super) fn finalize_created_file(
        &self,
        parent_path: &VirtualPath,
        path: VirtualPath,
        file: File,
    ) -> (u64, u64) {
        self.with_state_write(|state| {
            #[cfg(feature = "perf-counters")]
            let mut stats = state.invalidate_directory_snapshots(std::slice::from_ref(parent_path));
            #[cfg(not(feature = "perf-counters"))]
            state.invalidate_directory_snapshots(std::slice::from_ref(parent_path));
            #[cfg(feature = "perf-counters")]
            {
                stats += state.invalidate_exact_path(&path);
            }
            #[cfg(not(feature = "perf-counters"))]
            state.invalidate_exact_path(&path);
            let inode = state.lookup_path(path.clone());
            let fh = state.insert_file(inode, path, file);
            #[cfg(feature = "perf-counters")]
            if let Some(perf) = self.perf.as_ref() {
                perf.record_invalidation(stats);
            }
            (inode, fh)
        })
    }

    pub(super) fn invalidate_after_mutation(
        &self,
        parents: &[VirtualPath],
        exact_paths: &[VirtualPath],
        tree_paths: &[VirtualPath],
    ) {
        self.with_state_write(|state| {
            #[cfg(feature = "perf-counters")]
            let mut stats = state.invalidate_directory_snapshots(parents);
            #[cfg(not(feature = "perf-counters"))]
            state.invalidate_directory_snapshots(parents);
            for path in exact_paths {
                #[cfg(feature = "perf-counters")]
                {
                    stats += state.invalidate_exact_path(path);
                }
                #[cfg(not(feature = "perf-counters"))]
                state.invalidate_exact_path(path);
            }
            for path in tree_paths {
                #[cfg(feature = "perf-counters")]
                {
                    stats += state.invalidate_path_tree(path);
                }
                #[cfg(not(feature = "perf-counters"))]
                state.invalidate_path_tree(path);
            }
            #[cfg(feature = "perf-counters")]
            if let Some(perf) = self.perf.as_ref() {
                perf.record_invalidation(stats);
            }
        });
    }

    pub(super) fn forget_tracked_inode(&self, inode: u64, nlookup: u64) {
        self.with_state_write(|state| state.forget_inode(inode, nlookup));
    }
}
