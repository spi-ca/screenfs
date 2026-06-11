use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;

use fractal_fuse::{
    DirectoryEntry, DirectoryEntryPlus, ENOENT, Filesystem, FsResult, ReplyAttr, ReplyCreate,
    ReplyEntry, ReplyOpen, ReplyReadlink, ReplyStatfs, ReplyXattr, Request, SetAttr,
};

use crate::config::RuntimeConfig;
use crate::errors::errno_from_io;
use crate::path::VirtualPath;

mod backing;
mod guards;
mod state;

use self::backing::{
    apply_setattr, cstring_os, cstring_path, faccessat2_empty, read_xattr_reply,
    sanitize_open_flags, source_root_statfs,
};
use self::state::State;

#[derive(Debug)]
pub struct ScreenFs {
    cfg: RuntimeConfig,
    state: Mutex<State>,
    bridge_visible_dirs: HashSet<VirtualPath>,
}

impl ScreenFs {
    pub fn new(cfg: RuntimeConfig) -> Self {
        let bridge_visible_dirs = build_bridge_visible_dirs(&cfg);
        Self {
            cfg,
            state: Mutex::new(State::new()),
            bridge_visible_dirs,
        }
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.cfg
    }
}

fn build_bridge_visible_dirs(cfg: &RuntimeConfig) -> HashSet<VirtualPath> {
    let mut bridge_dirs = HashSet::new();
    if !cfg.needs_dynamic_bridge_index() {
        return bridge_dirs;
    }
    let mut stack = cfg.dynamic_bridge_scan_roots();
    while let Some(dir) = stack.pop() {
        let Ok(source) = cfg
            .source_root
            .join(dir.to_source_relative_path())
            .canonicalize()
        else {
            continue;
        };
        let Ok(entries) = fs::read_dir(source) else {
            continue;
        };
        for entry in entries.flatten() {
            let child = dir.join_child(&entry.file_name());
            let Ok(metadata) = fs::symlink_metadata(entry.path()) else {
                continue;
            };
            let fully_visible = cfg.is_fully_visible(&child)
                && (!metadata.file_type().is_symlink()
                    || fs::read_link(entry.path()).ok().is_some_and(|target| {
                        !cfg.is_hidden_symlink_target(&child, target.as_os_str())
                    }));
            if fully_visible {
                add_bridge_ancestors(&mut bridge_dirs, &child);
            }
            if metadata.is_dir() {
                stack.push(child);
            }
        }
    }
    bridge_dirs
}

fn add_bridge_ancestors(bridge_dirs: &mut HashSet<VirtualPath>, path: &VirtualPath) {
    let mut current = ScreenFs::parent_path(path);
    loop {
        if !current.as_path().as_os_str().is_empty() {
            bridge_dirs.insert(current.clone());
        }
        if current == VirtualPath::root() {
            break;
        }
        current = ScreenFs::parent_path(&current);
    }
}

impl Filesystem for ScreenFs {
    async fn lookup(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<ReplyEntry> {
        let path = self.child_path(parent, name)?;
        self.reply_entry_for_path(path)
    }

    fn forget(&self, _req: Request, inode: u64, nlookup: u64) {
        self.forget_tracked_inode(inode, nlookup);
    }

    async fn getattr(
        &self,
        _req: Request,
        inode: u64,
        _fh: Option<u64>,
        _flags: u32,
    ) -> FsResult<ReplyAttr> {
        let path = self.path_for_inode(inode)?;
        Ok(ReplyAttr {
            ttl: self.cfg.attr_ttl,
            attr: self.attr_for_path(&path, inode)?,
        })
    }

    async fn readlink(&self, _req: Request, inode: u64) -> FsResult<ReplyReadlink> {
        let path = self.path_for_inode(inode)?;
        self.guard_read_path(&path)?;
        let target = fs::read_link(self.host_path(&path, false)?).map_err(errno_from_io)?;
        self.check_hidden_symlink_target(&path, target.as_os_str())?;
        Ok(ReplyReadlink {
            data: target.as_os_str().as_bytes().to_vec(),
        })
    }

    async fn open(&self, _req: Request, inode: u64, flags: u32) -> FsResult<ReplyOpen> {
        let path = self.path_for_inode(inode)?;
        self.guard_open_flags(&path, flags)?;
        let file = self.open_confined(&path, sanitize_open_flags(flags, false), None)?;
        let fh = self.insert_open_file(inode, path, file);
        Ok(ReplyOpen {
            fh,
            flags: 0,
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
        let (path, mut file) = self.file_handle_snapshot(inode, fh)?;
        self.guard_read_path(&path)?;
        file.seek(SeekFrom::Start(offset)).map_err(errno_from_io)?;
        file.read(buf).map_err(errno_from_io)
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
        let (path, mut file) = self.file_handle_snapshot(inode, fh)?;
        self.guard_mutation_path(&path, true)?;
        file.seek(SeekFrom::Start(offset)).map_err(errno_from_io)?;
        file.write(data).map_err(errno_from_io)
    }

    async fn flush(&self, _req: Request, inode: u64, fh: u64, _lock_owner: u64) -> FsResult<()> {
        let (_path, file) = self.file_handle_snapshot(inode, fh)?;
        file.sync_all().map_err(errno_from_io)
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
        let handle = {
            let mut state = self.state.lock().expect("state mutex poisoned");
            state.remove_file(fh)
        };
        if flush && let Some(handle) = handle {
            handle.file.sync_all().map_err(errno_from_io)?;
        }
        Ok(())
    }

    async fn fsync(&self, _req: Request, inode: u64, fh: u64, datasync: bool) -> FsResult<()> {
        let (_path, file) = self.file_handle_snapshot(inode, fh)?;
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

    async fn opendir(&self, _req: Request, inode: u64, flags: u32) -> FsResult<ReplyOpen> {
        let path = self.path_for_inode(inode)?;
        self.guard_open_flags(&path, flags)?;
        self.open_confined(&path, libc::O_PATH | libc::O_DIRECTORY, None)?;
        let snapshot = self.directory_snapshot(&path, inode)?;
        let fh = self.insert_open_directory(inode, path, snapshot);
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
        _size: u32,
    ) -> FsResult<Vec<DirectoryEntry>> {
        Ok(self
            .opendir_snapshot(inode, fh)?
            .into_iter()
            .filter(|entry| entry.offset > offset)
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
        _size: u32,
    ) -> FsResult<Vec<DirectoryEntryPlus>> {
        let entries = self
            .opendir_snapshot(inode, fh)?
            .into_iter()
            .filter(|entry| entry.offset > offset)
            .collect::<Vec<_>>();
        self.add_lookup_refs_for_readdirplus(
            entries
                .iter()
                .filter(|entry| entry.name != b"." && entry.name != b"..")
                .map(|entry| entry.ino),
        );
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
        self.remove_open_directory(fh);
        Ok(())
    }

    async fn access(&self, _req: Request, inode: u64, mask: u32) -> FsResult<()> {
        let path = self.path_for_inode(inode)?;
        self.guard_access_mask(&path, mask)?;
        let file = self.open_confined(&path, libc::O_PATH, None)?;
        faccessat2_empty(&file, mask)
    }

    async fn statfs(&self, _req: Request, _inode: u64) -> FsResult<ReplyStatfs> {
        source_root_statfs(&self.cfg.source_root)
    }

    async fn setattr(
        &self,
        _req: Request,
        inode: u64,
        _fh: Option<u64>,
        set_attr: SetAttr,
    ) -> FsResult<ReplyAttr> {
        let path = self.path_for_inode(inode)?;
        self.guard_mutation_path(&path, true)?;
        let source = self.host_path(&path, true)?;
        apply_setattr(&source, set_attr)?;
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
        let path = self.child_path(parent, name)?;
        if self.cfg.is_hidden_symlink_target(&path, link) {
            return Err(ENOENT);
        }
        let parent_path = self.guard_child_mutation_path(&path)?;
        std::os::unix::fs::symlink(link, self.host_path(&path, false)?).map_err(errno_from_io)?;
        self.invalidate_after_mutation(&[parent_path], std::slice::from_ref(&path), &[]);
        self.reply_entry_for_path(path)
    }

    async fn mknod(
        &self,
        _req: Request,
        parent: u64,
        name: &OsStr,
        mode: u32,
        rdev: u32,
    ) -> FsResult<ReplyEntry> {
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let source = self.host_path(&path, false)?;
        let c_path = cstring_path(&source)?;
        let result =
            unsafe { libc::mknod(c_path.as_ptr(), mode as libc::mode_t, rdev as libc::dev_t) };
        if result != 0 {
            return Err(errno_from_io(std::io::Error::last_os_error()));
        }
        self.invalidate_after_mutation(&[parent_path], std::slice::from_ref(&path), &[]);
        self.reply_entry_for_path(path)
    }

    async fn unlink(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<()> {
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        fs::remove_file(self.host_path(&path, false)?).map_err(errno_from_io)?;
        self.invalidate_after_mutation(&[parent_path], &[], &[path]);
        Ok(())
    }

    async fn rmdir(&self, _req: Request, parent: u64, name: &OsStr) -> FsResult<()> {
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        fs::remove_dir(self.host_path(&path, false)?).map_err(errno_from_io)?;
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
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let source = self.host_path(&path, false)?;
        fs::create_dir(&source).map_err(errno_from_io)?;
        fs::set_permissions(source, fs::Permissions::from_mode(mode & 0o7777))
            .map_err(errno_from_io)?;
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
        let from = self.child_path(parent, name)?;
        let to = self.child_path(new_parent, new_name)?;
        let from_parent = Self::parent_path(&from);
        let to_parent = Self::parent_path(&to);
        self.guard_multi_path_mutation(&[
            (&from, false),
            (&to, false),
            (&from_parent, true),
            (&to_parent, true),
        ])?;
        fs::rename(self.host_path(&from, false)?, self.host_path(&to, false)?)
            .map_err(errno_from_io)?;
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
        let source = self.path_for_inode(inode)?;
        let target = self.child_path(new_parent, new_name)?;
        let target_parent = Self::parent_path(&target);
        self.guard_multi_path_mutation(&[
            (&source, false),
            (&target, false),
            (&target_parent, true),
        ])?;
        fs::hard_link(
            self.host_path(&source, false)?,
            self.host_path(&target, false)?,
        )
        .map_err(errno_from_io)?;
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
        let (path, parent_path) = self.guarded_child_mutation(parent, name)?;
        let file =
            self.open_confined(&path, sanitize_open_flags(flags, true), Some(mode & 0o7777))?;
        let (inode, fh) = self.finalize_created_file(&parent_path, path.clone(), file);
        let attr = self.attr_for_path(&path, inode)?;
        Ok(ReplyCreate {
            ttl: self.cfg.entry_ttl,
            attr,
            generation: 0,
            fh,
            flags,
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
        let (path, file) = self.file_handle_snapshot(inode, fh)?;
        self.guard_mutation_path(&path, true)?;
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
        let state = self.state.lock().expect("state mutex poisoned");
        let handle = state.files.get(&fh).ok_or(ENOENT)?;
        let result = unsafe {
            libc::lseek(
                handle.file.as_raw_fd(),
                offset as libc::off_t,
                whence as i32,
            )
        };
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
        let state = self.state.lock().expect("state mutex poisoned");
        let input = state.files.get(&fh_in).ok_or(ENOENT)?;
        let output = state.files.get(&fh_out).ok_or(ENOENT)?;
        if input.inode != inode_in || output.inode != inode_out {
            return Err(ENOENT);
        }
        self.guard_read_path(&input.path)?;
        let output_parent = Self::parent_path(&output.path);
        self.guard_mutation_coordinates(&[], &[(&output.path, true), (&output_parent, true)])?;
        let mut in_off = off_in as libc::off64_t;
        let mut out_off = off_out as libc::off64_t;
        let copied = unsafe {
            libc::copy_file_range(
                input.file.as_raw_fd(),
                &mut in_off,
                output.file.as_raw_fd(),
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
        let source = self.xattr_host_path(inode, false)?;
        let c_path = cstring_path(&source)?;
        let c_name = cstring_os(name)?;
        read_xattr_reply(
            size,
            || {
                let needed = unsafe {
                    libc::getxattr(c_path.as_ptr(), c_name.as_ptr(), std::ptr::null_mut(), 0)
                };
                if needed < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(needed as usize)
                }
            },
            |data| {
                let read = unsafe {
                    libc::getxattr(
                        c_path.as_ptr(),
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
        let source = self.xattr_host_path(inode, false)?;
        let c_path = cstring_path(&source)?;
        read_xattr_reply(
            size,
            || {
                let needed = unsafe { libc::listxattr(c_path.as_ptr(), std::ptr::null_mut(), 0) };
                if needed < 0 {
                    Err(errno_from_io(std::io::Error::last_os_error()))
                } else {
                    Ok(needed as usize)
                }
            },
            |data| {
                let read = unsafe {
                    libc::listxattr(c_path.as_ptr(), data.as_mut_ptr().cast(), data.len())
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
        let source = self.xattr_host_path(inode, true)?;
        let c_path = cstring_path(&source)?;
        let c_name = cstring_os(name)?;
        let result = unsafe {
            libc::setxattr(
                c_path.as_ptr(),
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
        let source = self.xattr_host_path(inode, true)?;
        let c_path = cstring_path(&source)?;
        let c_name = cstring_os(name)?;
        let result = unsafe { libc::removexattr(c_path.as_ptr(), c_name.as_ptr()) };
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
