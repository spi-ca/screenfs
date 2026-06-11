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
}

impl ScreenFs {
    pub fn new(cfg: RuntimeConfig) -> Self {
        Self {
            cfg,
            state: Mutex::new(State::new()),
        }
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.cfg
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
        self.with_file_handle_mut(inode, fh, |handle| {
            self.guard_read_path(&handle.path)?;
            handle
                .file
                .seek(SeekFrom::Start(offset))
                .map_err(errno_from_io)?;
            handle.file.read(buf).map_err(errno_from_io)
        })
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
        self.with_file_handle_mut(inode, fh, |handle| {
            self.guard_mutation_path(&handle.path, true)?;
            handle
                .file
                .seek(SeekFrom::Start(offset))
                .map_err(errno_from_io)?;
            handle.file.write(data).map_err(errno_from_io)
        })
    }

    async fn flush(&self, _req: Request, inode: u64, fh: u64, _lock_owner: u64) -> FsResult<()> {
        self.with_file_handle(inode, fh, |handle| {
            handle.file.sync_all().map_err(errno_from_io)
        })
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
        let mut state = self.state.lock().expect("state mutex poisoned");
        if flush && let Some(handle) = state.files.get(&fh) {
            handle.file.sync_all().map_err(errno_from_io)?;
        }
        state.remove_file(fh);
        Ok(())
    }

    async fn fsync(&self, _req: Request, inode: u64, fh: u64, datasync: bool) -> FsResult<()> {
        self.with_file_handle(inode, fh, |handle| {
            let result = unsafe {
                if datasync {
                    libc::fdatasync(handle.file.as_raw_fd())
                } else {
                    libc::fsync(handle.file.as_raw_fd())
                }
            };
            if result == 0 {
                Ok(())
            } else {
                Err(errno_from_io(std::io::Error::last_os_error()))
            }
        })
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
        Ok(self
            .opendir_snapshot(inode, fh)?
            .into_iter()
            .filter(|entry| entry.offset > offset)
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
        if self.cfg.matcher.is_hidden_symlink_target(&path, link) {
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
        self.with_file_handle(inode, fh, |handle| {
            self.guard_mutation_path(&handle.path, true)?;
            let result = unsafe {
                libc::fallocate(
                    handle.file.as_raw_fd(),
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
        })
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
mod tests {
    use super::*;
    use crate::cli::{CliArgs, LaunchArgs, MutabilityFamily};
    use crate::config::RuntimeConfig;
    use crate::path::{ProcessEnvGuard, VirtualPath};
    use fractal_fuse::abi::FUSE_ROOT_ID;
    use std::fs::{File, OpenOptions};
    use std::future::Future;
    use std::path::Path;
    use std::pin::Pin;
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    };
    use std::task::{Context, Poll, Wake, Waker};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn fs_for_policy(
        source: &Path,
        hide_rules: Vec<String>,
        readonly_rules: Vec<String>,
        policy_family: Option<MutabilityFamily>,
        allow_write_rules: Vec<String>,
    ) -> ScreenFs {
        let mount = source.join("mount");
        std::fs::create_dir_all(&mount).unwrap();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.to_path_buf(),
                mount_root: mount,
                hide_rules,
                readonly_rules,
            },
            config_path: None,
            policy_family,
            allow_write_rules,
        })
        .unwrap();
        ScreenFs::new(cfg)
    }

    fn fs_for(source: &Path, hide_rules: Vec<String>, readonly_rules: Vec<String>) -> ScreenFs {
        fs_for_policy(source, hide_rules, readonly_rules, None, Vec::new())
    }

    fn fs_for_root_readonly(source: &Path, hide_rules: Vec<String>) -> ScreenFs {
        fs_for_policy(
            source,
            hide_rules,
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            Vec::new(),
        )
    }

    #[test]
    fn hidden_read_and_list_operations_return_enoent_and_filter_entries() {
        let dir = test_dir("hidden-read-list");
        std::fs::write(dir.join("visible"), b"ok").unwrap();
        std::fs::write(dir.join("hidden.pem"), b"secret").unwrap();
        std::fs::create_dir(dir.join("private")).unwrap();
        std::fs::write(dir.join("private/note.txt"), b"nope").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["**/*.pem".to_string(), "/private".to_string()]);

        let hidden_inode = tracked_inode(&fs, "/hidden.pem");
        let fh = insert_tracked_file_handle(
            &fs,
            hidden_inode,
            "/hidden.pem",
            File::open(dir.join("hidden.pem")).unwrap(),
        );
        let mut buf = [0_u8; 16];
        let err = block_on(fs.read(dummy_req(), hidden_inode, fh, 0, &mut buf)).unwrap_err();
        assert_eq!(err, ENOENT);
        block_on(fs.release(dummy_req(), hidden_inode, fh, 0, 0, false, false)).unwrap();

        let hidden_dir_inode = tracked_inode(&fs, "/private");
        let err =
            block_on(fs.opendir(dummy_req(), hidden_dir_inode, libc::O_RDONLY as u32)).unwrap_err();
        assert_eq!(err, ENOENT);

        let names = root_listing_names(&fs);
        assert!(names.contains(&"visible".to_string()));
        assert!(!names.contains(&"hidden.pem".to_string()));
        assert!(!names.contains(&"private".to_string()));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_direct_child_suffix_rules_keep_enoent_precedence_over_readonly() {
        let dir = test_dir("hidden-direct-child-suffix-precedence");
        let source = dir.join("src");
        let cwd = source.join("workspace/app");
        let home = source.join("home/tester");
        std::fs::create_dir_all(source.join("home/tester/nested")).unwrap();
        std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
        std::fs::create_dir_all(&cwd).unwrap();
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(source.join("home/tester/user.pem"), b"secret").unwrap();
        std::fs::write(source.join("home/tester/nested/user.pem"), b"nested").unwrap();
        std::fs::write(source.join("workspace/app/fixtures/local.pem"), b"ok").unwrap();
        std::fs::write(
            source.join("workspace/app/fixtures/nested/local.pem"),
            b"nested",
        )
        .unwrap();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();

        let fs = {
            let _env = ProcessEnvGuard::new(&cwd, Some(&home));
            let cfg = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount,
                    hide_rules: vec!["~/*.pem".to_string()],
                    readonly_rules: vec!["~/*.pem".to_string()],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap();
            ScreenFs::new(cfg)
        };

        let hidden_path = vpath("/home/tester/user.pem");
        assert!(fs.config().is_hidden(&hidden_path));
        assert!(fs.config().is_readonly(&hidden_path));
        let hidden_inode = tracked_inode(&fs, "/home/tester/user.pem");
        let err = block_on(fs.open(dummy_req(), hidden_inode, libc::O_WRONLY as u32)).unwrap_err();
        assert_eq!(err, ENOENT);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn selective_readonly_descendant_subtree_rule_only_locks_git_hooks_subtree() {
        let dir = test_dir("selective-readonly-git-hooks-subtree");
        std::fs::create_dir_all(dir.join("project/repo/.git/hooks")).unwrap();
        std::fs::write(dir.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
        std::fs::write(dir.join("project/repo/.git/config"), b"config").unwrap();
        let fs = fs_for(
            &dir,
            Vec::new(),
            vec!["/project/**/.git/hooks/**".to_string()],
        );

        let hook = fs
            .reply_entry_for_path(VirtualPath::new("/project/repo/.git/hooks/pre-commit"))
            .unwrap()
            .attr
            .ino;
        let hooks_dir = fs
            .reply_entry_for_path(VirtualPath::new("/project/repo/.git/hooks"))
            .unwrap()
            .attr
            .ino;
        let config = fs
            .reply_entry_for_path(VirtualPath::new("/project/repo/.git/config"))
            .unwrap()
            .attr
            .ino;

        assert_eq!(
            block_on(fs.open(dummy_req(), hook, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.opendir(dummy_req(), hooks_dir, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );

        let config_handle = block_on(fs.open(dummy_req(), config, libc::O_WRONLY as u32)).unwrap();
        block_on(fs.release(dummy_req(), config, config_handle.fh, 0, 0, false, false)).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_descendant_subtree_rules_keep_enoent_precedence_over_readonly() {
        let dir = test_dir("hidden-git-hooks-precedence");
        std::fs::create_dir_all(dir.join("project/repo/.git/hooks")).unwrap();
        std::fs::write(dir.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
        std::fs::write(dir.join("project/repo/.git/config"), b"config").unwrap();
        let fs = fs_for(
            &dir,
            vec!["/project/**/.git/hooks/**".to_string()],
            vec!["/project/**/.git/**".to_string()],
        );

        let hidden_hook = tracked_inode(&fs, "/project/repo/.git/hooks/pre-commit");
        let hidden_hooks_dir = tracked_inode(&fs, "/project/repo/.git/hooks");
        let readonly_config = fs
            .reply_entry_for_path(VirtualPath::new("/project/repo/.git/config"))
            .unwrap()
            .attr
            .ino;

        assert_eq!(
            block_on(fs.open(dummy_req(), hidden_hook, libc::O_WRONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.opendir(dummy_req(), hidden_hooks_dir, libc::O_WRONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.open(dummy_req(), readonly_config, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn visible_read_and_nested_listing_preserve_access_and_parent_entries() {
        let dir = test_dir("visible-read-list");
        std::fs::create_dir_all(dir.join("a/b")).unwrap();
        std::fs::write(dir.join("a/b/hello.txt"), b"hello world").unwrap();
        let fs = fs_for_root_readonly(&dir, Vec::new());

        let file_inode = fs
            .reply_entry_for_path(VirtualPath::new("/a/b/hello.txt"))
            .unwrap()
            .attr
            .ino;
        let handle = block_on(fs.open(dummy_req(), file_inode, libc::O_RDONLY as u32)).unwrap();
        let mut buf = [0_u8; 5];
        let len = block_on(fs.read(dummy_req(), file_inode, handle.fh, 0, &mut buf)).unwrap();
        assert_eq!(&buf[..len], b"hello");
        block_on(fs.release(dummy_req(), file_inode, handle.fh, 0, 0, false, false)).unwrap();

        let parent_inode = fs
            .reply_entry_for_path(VirtualPath::new("/a"))
            .unwrap()
            .attr
            .ino;
        let dir_inode = fs
            .reply_entry_for_path(VirtualPath::new("/a/b"))
            .unwrap()
            .attr
            .ino;
        let handle = block_on(fs.opendir(dummy_req(), dir_inode, libc::O_RDONLY as u32)).unwrap();
        let entries = block_on(fs.readdir(dummy_req(), dir_inode, handle.fh, 0, 4096)).unwrap();
        let dotdot = entries
            .iter()
            .find(|entry| entry.name.as_slice() == b"..")
            .unwrap();
        assert_eq!(dotdot.ino, parent_inode);
        assert!(
            entries
                .iter()
                .any(|entry| entry.name.as_slice() == b"hello.txt")
        );

        let plus_entries =
            block_on(fs.readdirplus(dummy_req(), dir_inode, handle.fh, 0, 4096)).unwrap();
        let dotdot = plus_entries
            .iter()
            .find(|entry| entry.name.as_slice() == b"..")
            .unwrap();
        assert_eq!(dotdot.ino, parent_inode);
        assert_eq!(dotdot.attr.ino, parent_inode);
        block_on(fs.releasedir(dummy_req(), dir_inode, handle.fh, 0)).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn opendir_returns_real_handle_and_readdir_variants_share_stable_snapshot() {
        let dir = test_dir("dir-handle-snapshot");
        std::fs::create_dir(dir.join("listing")).unwrap();
        std::fs::write(dir.join("listing/alpha"), b"a").unwrap();
        std::fs::write(dir.join("listing/gamma"), b"g").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());
        let listing = fs
            .reply_entry_for_path(VirtualPath::new("/listing"))
            .unwrap()
            .attr
            .ino;

        let handle = block_on(fs.opendir(dummy_req(), listing, libc::O_RDONLY as u32)).unwrap();
        assert_ne!(handle.fh, 0);
        assert!(
            fs.state
                .lock()
                .expect("state mutex poisoned")
                .directories
                .contains_key(&handle.fh)
        );

        let first_page = block_on(fs.readdir(dummy_req(), listing, handle.fh, 0, 4096)).unwrap();
        let alpha = first_page
            .iter()
            .find(|entry| entry.name.as_slice() == b"alpha")
            .unwrap();
        assert_eq!(alpha.offset, 3);

        std::fs::write(dir.join("listing/beta"), b"b").unwrap();
        let remaining =
            block_on(fs.readdirplus(dummy_req(), listing, handle.fh, alpha.offset, 4096)).unwrap();
        let remaining_names = remaining
            .iter()
            .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(remaining_names, vec!["gamma".to_string()]);
        assert_eq!(remaining[0].offset, 4);

        block_on(fs.releasedir(dummy_req(), listing, handle.fh, 0)).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn releasedir_removes_directory_handle_and_snapshot() {
        let dir = test_dir("releasedir-cleanup");
        std::fs::create_dir(dir.join("listing")).unwrap();
        std::fs::write(dir.join("listing/file"), b"ok").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());
        let listing = fs
            .reply_entry_for_path(VirtualPath::new("/listing"))
            .unwrap()
            .attr
            .ino;

        let handle = block_on(fs.opendir(dummy_req(), listing, libc::O_RDONLY as u32)).unwrap();
        assert!(
            fs.state
                .lock()
                .expect("state mutex poisoned")
                .directories
                .contains_key(&handle.fh)
        );

        block_on(fs.releasedir(dummy_req(), listing, handle.fh, 0)).unwrap();
        assert!(
            !fs.state
                .lock()
                .expect("state mutex poisoned")
                .directories
                .contains_key(&handle.fh)
        );
        assert_eq!(
            block_on(fs.readdir(dummy_req(), listing, handle.fh, 0, 4096)).unwrap_err(),
            ENOENT
        );

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_write_intent_open_and_opendir_return_erofs_but_hidden_stays_enoent() {
        let dir = test_dir("readonly-open");
        std::fs::write(dir.join("visible"), b"ok").unwrap();
        std::fs::create_dir(dir.join("docs")).unwrap();
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

        let visible = fs
            .reply_entry_for_path(VirtualPath::new("/visible"))
            .unwrap()
            .attr
            .ino;
        let docs = fs
            .reply_entry_for_path(VirtualPath::new("/docs"))
            .unwrap()
            .attr
            .ino;
        let hidden = fs.reply_entry_for_path(VirtualPath::new("/hidden"));
        assert_eq!(hidden.unwrap_err(), ENOENT);

        let err = block_on(fs.open(dummy_req(), visible, libc::O_WRONLY as u32)).unwrap_err();
        assert_eq!(err, libc::EROFS);
        let err = block_on(fs.opendir(dummy_req(), docs, libc::O_WRONLY as u32)).unwrap_err();
        assert_eq!(err, libc::EROFS);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn symlink_to_hidden_target_returns_enoent_before_readonly_for_write_intent() {
        let dir = test_dir("symlink-hidden-readonly-mutation");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let inode = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/link"));

        assert_eq!(
            block_on(fs.open(dummy_req(), inode, libc::O_WRONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), inode, libc::W_OK as u32)).unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn symlink_to_hidden_target_returns_enoent_for_lookup_open_and_readlink() {
        let dir = test_dir("symlink-hidden");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

        let lookup_err = fs
            .reply_entry_for_path(VirtualPath::new("/link"))
            .unwrap_err();
        assert_eq!(lookup_err, ENOENT);

        let inode = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/link"));
        let open_err = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err();
        assert_eq!(open_err, ENOENT);
        let readlink_err = block_on(fs.readlink(dummy_req(), inode)).unwrap_err();
        assert_eq!(readlink_err, ENOENT);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent() {
        let root = test_dir("symlink-source-root-escape");
        let source = root.join("source");
        let outside = root.join("outside");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("secret.txt"), b"secret").unwrap();
        std::os::unix::fs::symlink("../outside/secret.txt", source.join("escape")).unwrap();
        let fs = fs_for_root_readonly(&source, Vec::new());

        let inode = fs
            .reply_entry_for_path(VirtualPath::new("/escape"))
            .unwrap()
            .attr
            .ino;
        let readlink = block_on(fs.readlink(dummy_req(), inode)).unwrap();
        assert_eq!(readlink.data, b"../outside/secret.txt");
        assert_eq!(
            block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
            ENOENT
        );

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects() {
        let root = test_dir("symlink-dir-source-root-escape");
        let source = root.join("source");
        let outside = root.join("outside");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::os::unix::fs::symlink("../outside", source.join("escape-dir")).unwrap();
        let fs = fs_for(&source, Vec::new(), Vec::new());

        let inode = fs
            .reply_entry_for_path(VirtualPath::new("/escape-dir"))
            .unwrap()
            .attr
            .ino;
        assert_eq!(
            block_on(fs.opendir(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.create(
                dummy_req(),
                inode,
                OsStr::new("created.txt"),
                0o644,
                libc::O_WRONLY as u32,
            ))
            .unwrap_err(),
            ENOENT
        );
        assert!(!outside.join("created.txt").exists());

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn access_passes_through_host_permissions_and_readonly_write_mask() {
        let dir = test_dir("access");
        let file = dir.join("file");
        std::fs::write(&file, b"data").unwrap();
        let fs = fs_for_root_readonly(&dir, Vec::new());
        let inode = fs
            .reply_entry_for_path(VirtualPath::new("/file"))
            .unwrap()
            .attr
            .ino;

        let write_err = block_on(fs.access(dummy_req(), inode, libc::W_OK as u32)).unwrap_err();
        assert_eq!(write_err, libc::EROFS);

        if unsafe { libc::getuid() } != 0 {
            std::fs::set_permissions(&file, fs::Permissions::from_mode(0o0)).unwrap();
            let read_err = block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err();
            assert_eq!(read_err, libc::EACCES);
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_access_returns_enoent_for_read_and_write_masks() {
        let dir = test_dir("hidden-access");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden"));

        assert_eq!(
            block_on(fs.access(dummy_req(), hidden, libc::R_OK as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), hidden, libc::W_OK as u32)).unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_create_returns_erofs_instead_of_enosys() {
        let dir = test_dir("readonly-create");
        let fs = fs_for_root_readonly(&dir, Vec::new());
        let err = block_on(fs.create(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("new-file"),
            0o644,
            libc::O_CREAT as u32,
        ))
        .unwrap_err();
        assert_eq!(err, libc::EROFS);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn statfs_reflects_backing_filesystem_instead_of_placeholder_values() {
        let dir = test_dir("statfs");
        std::fs::write(dir.join("visible"), b"ok").unwrap();
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        std::fs::create_dir(dir.join("nested")).unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let visible = fs
            .reply_entry_for_path(VirtualPath::new("/visible"))
            .unwrap()
            .attr
            .ino;
        fs.state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/nested"));

        let stats = block_on(fs.statfs(dummy_req(), visible)).unwrap();
        let host = host_statfs(&dir);
        let tracked_inodes = fs.state.lock().expect("state mutex poisoned").inodes.len() as u64;

        assert_eq!(stats.blocks, host.blocks);
        assert_eq!(stats.files, host.files);
        assert_eq!(stats.bsize, host.bsize);
        assert_eq!(stats.frsize, host.frsize);
        assert_eq!(stats.namelen, host.namelen);
        assert!(stats.blocks > 0);
        assert!(stats.bfree <= stats.blocks);
        assert!(stats.bavail <= stats.bfree);
        if host.files > 0 {
            assert!(stats.ffree <= stats.files);
            assert!(stats.files > tracked_inodes);
        } else {
            assert_eq!(stats.files, 0);
        }

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn open_honors_host_access_mode_truncate_and_append_flags() {
        let dir = test_dir("open-host-flags");
        let file = dir.join("file");
        std::fs::write(&file, b"abc").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());
        let inode = fs
            .reply_entry_for_path(VirtualPath::new("/file"))
            .unwrap()
            .attr
            .ino;

        let truncated =
            block_on(fs.open(dummy_req(), inode, (libc::O_WRONLY | libc::O_TRUNC) as u32)).unwrap();
        let mut buf = [0_u8; 1];
        let read_err =
            block_on(fs.read(dummy_req(), inode, truncated.fh, 0, &mut buf)).unwrap_err();
        assert_eq!(read_err, libc::EBADF);
        block_on(fs.release(dummy_req(), inode, truncated.fh, 0, 0, false, false)).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), Vec::<u8>::new());

        std::fs::write(&file, b"abc").unwrap();
        let appended =
            block_on(fs.open(dummy_req(), inode, (libc::O_RDWR | libc::O_APPEND) as u32)).unwrap();
        assert_eq!(
            block_on(fs.write(dummy_req(), inode, appended.fh, 0, b"z", 0, 0)).unwrap(),
            1
        );
        let mut buf = [0_u8; 4];
        let len = block_on(fs.read(dummy_req(), inode, appended.fh, 0, &mut buf)).unwrap();
        assert_eq!(&buf[..len], b"abcz");
        block_on(fs.release(dummy_req(), inode, appended.fh, 0, 0, false, false)).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn create_honors_host_exclusive_and_readonly_flags() {
        let dir = test_dir("create-host-flags");
        std::fs::write(dir.join("existing"), b"abc").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());

        let err = block_on(fs.create(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("existing"),
            0o644,
            (libc::O_CREAT | libc::O_EXCL | libc::O_WRONLY) as u32,
        ))
        .unwrap_err();
        assert_eq!(err, libc::EEXIST);

        let created = block_on(fs.create(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("new"),
            0o644,
            libc::O_RDONLY as u32,
        ))
        .unwrap();
        let write_err =
            block_on(fs.write(dummy_req(), created.attr.ino, created.fh, 0, b"x", 0, 0))
                .unwrap_err();
        assert_eq!(write_err, libc::EBADF);
        block_on(fs.release(
            dummy_req(),
            created.attr.ino,
            created.fh,
            0,
            0,
            false,
            false,
        ))
        .unwrap();
        assert!(dir.join("new").is_file());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_create_returns_enoent_before_readonly() {
        let dir = test_dir("hidden-create");
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

        let err = block_on(fs.create(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("hidden"),
            0o644,
            libc::O_CREAT as u32,
        ))
        .unwrap_err();
        assert_eq!(err, ENOENT);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn selective_readonly_rules_are_scoped_to_matching_paths() {
        let dir = test_dir("selective-readonly-scoped");
        std::fs::create_dir(dir.join("locked")).unwrap();
        std::fs::create_dir_all(dir.join("free/nested")).unwrap();
        std::fs::write(dir.join("locked/file.txt"), b"locked").unwrap();
        std::fs::write(dir.join("free/file.txt"), b"free").unwrap();
        std::fs::write(dir.join("free/state.lock"), b"state").unwrap();
        std::fs::write(dir.join("free/nested/app.lock"), b"nested").unwrap();
        let fs = fs_for(
            &dir,
            Vec::new(),
            vec!["/locked".to_string(), "**/*.lock".to_string()],
        );

        let locked = fs
            .reply_entry_for_path(VirtualPath::new("/locked/file.txt"))
            .unwrap()
            .attr
            .ino;
        let free = fs
            .reply_entry_for_path(VirtualPath::new("/free/file.txt"))
            .unwrap()
            .attr
            .ino;
        let top_level_lock = fs
            .reply_entry_for_path(VirtualPath::new("/free/state.lock"))
            .unwrap()
            .attr
            .ino;
        let nested_lock = fs
            .reply_entry_for_path(VirtualPath::new("/free/nested/app.lock"))
            .unwrap()
            .attr
            .ino;

        assert_eq!(
            block_on(fs.open(dummy_req(), locked, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.open(dummy_req(), top_level_lock, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), nested_lock, libc::W_OK as u32)).unwrap_err(),
            libc::EROFS
        );

        let free_handle =
            block_on(fs.open(dummy_req(), free, (libc::O_WRONLY | libc::O_APPEND) as u32)).unwrap();
        assert_eq!(
            block_on(fs.write(dummy_req(), free, free_handle.fh, 0, b"!", 0, 0)).unwrap(),
            1
        );
        block_on(fs.release(dummy_req(), free, free_handle.fh, 0, 0, false, false)).unwrap();

        let err = block_on(
            fs.create(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/locked"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("new.txt"),
                0o644,
                libc::O_CREAT as u32,
            ),
        )
        .unwrap_err();
        assert_eq!(err, libc::EROFS);

        let err = block_on(
            fs.create(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/free"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("new.lock"),
                0o644,
                libc::O_CREAT as u32,
            ),
        )
        .unwrap_err();
        assert_eq!(err, libc::EROFS);
        assert!(!dir.join("free/new.lock").exists());

        let created = block_on(
            fs.create(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/free"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("new.txt"),
                0o644,
                libc::O_CREAT as u32,
            ),
        )
        .unwrap();
        block_on(fs.release(
            dummy_req(),
            created.attr.ino,
            created.fh,
            0,
            0,
            false,
            false,
        ))
        .unwrap();
        assert!(dir.join("free/new.txt").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn selective_readonly_symlink_returns_erofs_and_hidden_precedence_remains_enoent() {
        let dir = test_dir("selective-readonly-symlink");
        std::fs::write(dir.join("locked.lock"), b"locked").unwrap();
        std::fs::write(dir.join("hidden.lock"), b"hidden").unwrap();
        std::fs::write(dir.join("free.txt"), b"free").unwrap();
        std::os::unix::fs::symlink("locked.lock", dir.join("locked-link")).unwrap();
        std::os::unix::fs::symlink("hidden.lock", dir.join("hidden-link")).unwrap();
        let fs = fs_for(
            &dir,
            vec![
                "/hidden.lock".to_string(),
                "/hidden-link-create".to_string(),
            ],
            vec!["**/*.lock".to_string(), "/locked-link-create".to_string()],
        );

        let locked = fs
            .reply_entry_for_path(VirtualPath::new("/locked-link"))
            .unwrap()
            .attr
            .ino;
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden-link"));

        assert_eq!(
            block_on(fs.open(dummy_req(), locked, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), locked, libc::W_OK as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.open(dummy_req(), hidden, libc::O_WRONLY as u32)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.access(dummy_req(), hidden, libc::W_OK as u32)).unwrap_err(),
            ENOENT
        );

        assert_eq!(
            block_on(fs.symlink(
                dummy_req(),
                FUSE_ROOT_ID,
                OsStr::new("locked-link-create"),
                OsStr::new("free.txt"),
            ))
            .unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.symlink(
                dummy_req(),
                FUSE_ROOT_ID,
                OsStr::new("hidden-link-create"),
                OsStr::new("free.txt"),
            ))
            .unwrap_err(),
            ENOENT
        );

        block_on(fs.symlink(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("free-link-create"),
            OsStr::new("free.txt"),
        ))
        .unwrap();
        assert_eq!(
            std::fs::read_link(dir.join("free-link-create")).unwrap(),
            std::path::PathBuf::from("free.txt")
        );
        assert!(
            std::fs::symlink_metadata(dir.join("free-link-create"))
                .unwrap()
                .file_type()
                .is_symlink()
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_root_allowwrite_match_non_match_and_hidden_precedence() {
        let dir = test_dir("allowwrite-match-hidden");
        std::fs::write(dir.join("allowed.txt"), b"ok").unwrap();
        std::fs::write(dir.join("blocked.bin"), b"no").unwrap();
        std::fs::write(dir.join("hidden.txt"), b"secret").unwrap();
        let fs = fs_for_policy(
            &dir,
            vec!["/hidden.txt".to_string()],
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            vec!["**/*.txt".to_string()],
        );

        let allowed = fs
            .reply_entry_for_path(VirtualPath::new("/allowed.txt"))
            .unwrap()
            .attr
            .ino;
        let blocked = fs
            .reply_entry_for_path(VirtualPath::new("/blocked.bin"))
            .unwrap()
            .attr
            .ino;
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden.txt"));

        let allowed_handle =
            block_on(fs.open(dummy_req(), allowed, libc::O_WRONLY as u32)).unwrap();
        block_on(fs.release(dummy_req(), allowed, allowed_handle.fh, 0, 0, false, false)).unwrap();
        assert_eq!(
            block_on(fs.open(dummy_req(), blocked, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.open(dummy_req(), hidden, libc::O_WRONLY as u32)).unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_root_allowwrite_requires_writable_parent_for_path_only_and_multi_path_mutation() {
        let dir = test_dir("allowwrite-parent-multi");
        std::fs::create_dir(dir.join("sandbox")).unwrap();
        std::fs::create_dir(dir.join("src")).unwrap();
        std::fs::create_dir(dir.join("dst")).unwrap();
        std::fs::write(dir.join("src/item.txt"), b"item").unwrap();

        let fs = fs_for_policy(
            &dir,
            Vec::new(),
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            vec!["**/*.txt".to_string()],
        );
        assert_eq!(
            block_on(
                fs.create(
                    dummy_req(),
                    fs.reply_entry_for_path(VirtualPath::new("/sandbox"))
                        .unwrap()
                        .attr
                        .ino,
                    OsStr::new("new.txt"),
                    0o644,
                    libc::O_CREAT as u32,
                )
            )
            .unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(
                fs.rename(
                    dummy_req(),
                    fs.reply_entry_for_path(VirtualPath::new("/src"))
                        .unwrap()
                        .attr
                        .ino,
                    OsStr::new("item.txt"),
                    fs.reply_entry_for_path(VirtualPath::new("/dst"))
                        .unwrap()
                        .attr
                        .ino,
                    OsStr::new("renamed.txt"),
                    0,
                )
            )
            .unwrap_err(),
            libc::EROFS
        );
        assert!(dir.join("src/item.txt").exists());

        let fs = fs_for_policy(
            &dir,
            Vec::new(),
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            vec!["/src".to_string(), "/dst".to_string()],
        );
        block_on(
            fs.rename(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/src"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("item.txt"),
                fs.reply_entry_for_path(VirtualPath::new("/dst"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("renamed.txt"),
                0,
            ),
        )
        .unwrap();
        assert!(!dir.join("src/item.txt").exists());
        assert_eq!(std::fs::read(dir.join("dst/renamed.txt")).unwrap(), b"item");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_root_allowwrite_copy_file_range_requires_writable_destination_parent() {
        let dir = test_dir("allowwrite-copy-parent");
        std::fs::create_dir(dir.join("blocked")).unwrap();
        std::fs::write(dir.join("input.txt"), b"abcdef").unwrap();
        std::fs::write(dir.join("blocked/out.txt"), b"------").unwrap();

        let fs = fs_for_policy(
            &dir,
            Vec::new(),
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            vec!["**/*.txt".to_string()],
        );
        let input = fs
            .reply_entry_for_path(VirtualPath::new("/input.txt"))
            .unwrap()
            .attr
            .ino;
        let output = fs
            .reply_entry_for_path(VirtualPath::new("/blocked/out.txt"))
            .unwrap()
            .attr
            .ino;
        let input_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            input,
            VirtualPath::new("/input.txt"),
            OpenOptions::new()
                .read(true)
                .open(dir.join("input.txt"))
                .unwrap(),
        );
        let output_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            output,
            VirtualPath::new("/blocked/out.txt"),
            OpenOptions::new()
                .write(true)
                .open(dir.join("blocked/out.txt"))
                .unwrap(),
        );
        assert_eq!(
            block_on(fs.copy_file_range(
                dummy_req(),
                input,
                input_fh,
                0,
                output,
                output_fh,
                0,
                3,
                0,
            ))
            .unwrap_err(),
            libc::EROFS
        );

        std::fs::write(dir.join("blocked/out.txt"), b"------").unwrap();
        let fs = fs_for_policy(
            &dir,
            Vec::new(),
            Vec::new(),
            Some(MutabilityFamily::ReadonlyRootAllowwrite),
            vec!["/blocked".to_string()],
        );
        let input = fs
            .reply_entry_for_path(VirtualPath::new("/input.txt"))
            .unwrap()
            .attr
            .ino;
        let output = fs
            .reply_entry_for_path(VirtualPath::new("/blocked/out.txt"))
            .unwrap()
            .attr
            .ino;
        let input_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            input,
            VirtualPath::new("/input.txt"),
            OpenOptions::new()
                .read(true)
                .open(dir.join("input.txt"))
                .unwrap(),
        );
        let output_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            output,
            VirtualPath::new("/blocked/out.txt"),
            OpenOptions::new()
                .write(true)
                .open(dir.join("blocked/out.txt"))
                .unwrap(),
        );
        let copied = block_on(fs.copy_file_range(
            dummy_req(),
            input,
            input_fh,
            1,
            output,
            output_fh,
            2,
            3,
            0,
        ))
        .unwrap();
        assert_eq!(copied, 3);
        assert_eq!(
            std::fs::read(dir.join("blocked/out.txt")).unwrap(),
            b"--bcd-"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_xattr_queries_return_enoent() {
        let dir = test_dir("hidden-xattr");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden"));

        assert_eq!(
            block_on(fs.getxattr(dummy_req(), hidden, OsStr::new("user.test"), 0)).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.listxattr(dummy_req(), hidden, 0)).unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn readonly_multi_path_xattr_and_fallocate_mutations_return_erofs() {
        let dir = test_dir("readonly-mutators");
        std::fs::write(dir.join("a"), b"aaaa").unwrap();
        std::fs::write(dir.join("b"), b"bbbb").unwrap();
        let fs = fs_for_root_readonly(&dir, Vec::new());
        let a = fs
            .reply_entry_for_path(VirtualPath::new("/a"))
            .unwrap()
            .attr
            .ino;
        let b = fs
            .reply_entry_for_path(VirtualPath::new("/b"))
            .unwrap()
            .attr
            .ino;
        let a_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            a,
            VirtualPath::new("/a"),
            OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
        );
        let b_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            b,
            VirtualPath::new("/b"),
            OpenOptions::new().write(true).open(dir.join("b")).unwrap(),
        );

        assert_eq!(
            block_on(fs.link(dummy_req(), a, FUSE_ROOT_ID, OsStr::new("a-link"))).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.symlink(
                dummy_req(),
                FUSE_ROOT_ID,
                OsStr::new("sym"),
                OsStr::new("a")
            ))
            .unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.setxattr(dummy_req(), a, OsStr::new("user.test"), b"v", 0)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.removexattr(dummy_req(), a, OsStr::new("user.test"))).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.fallocate(dummy_req(), b, b_fh, 0, 1, 0)).unwrap_err(),
            libc::EROFS
        );
        assert_eq!(
            block_on(fs.copy_file_range(dummy_req(), a, a_fh, 0, b, b_fh, 0, 1, 0)).unwrap_err(),
            libc::EROFS
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn visible_copy_file_range_copies_data() {
        let dir = test_dir("copy-visible");
        std::fs::write(dir.join("a"), b"abcdef").unwrap();
        std::fs::write(dir.join("b"), b"------").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());
        let a = fs
            .reply_entry_for_path(VirtualPath::new("/a"))
            .unwrap()
            .attr
            .ino;
        let b = fs
            .reply_entry_for_path(VirtualPath::new("/b"))
            .unwrap()
            .attr
            .ino;
        let a_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            a,
            VirtualPath::new("/a"),
            OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
        );
        let b_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            b,
            VirtualPath::new("/b"),
            OpenOptions::new().write(true).open(dir.join("b")).unwrap(),
        );

        let copied =
            block_on(fs.copy_file_range(dummy_req(), a, a_fh, 1, b, b_fh, 2, 3, 0)).unwrap();
        assert_eq!(copied, 3);
        assert_eq!(std::fs::read(dir.join("b")).unwrap(), b"--bcd-");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_copy_file_range_paths_return_enoent_before_readonly() {
        let dir = test_dir("copy-hidden");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        std::fs::write(dir.join("visible"), b"------").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden"));
        let visible = fs
            .reply_entry_for_path(VirtualPath::new("/visible"))
            .unwrap()
            .attr
            .ino;
        let hidden_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            hidden,
            VirtualPath::new("/hidden"),
            OpenOptions::new()
                .read(true)
                .open(dir.join("hidden"))
                .unwrap(),
        );
        let visible_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
            visible,
            VirtualPath::new("/visible"),
            OpenOptions::new()
                .write(true)
                .open(dir.join("visible"))
                .unwrap(),
        );

        assert_eq!(
            block_on(fs.copy_file_range(
                dummy_req(),
                hidden,
                hidden_fh,
                0,
                visible,
                visible_fh,
                0,
                1,
                0,
            ))
            .unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.copy_file_range(
                dummy_req(),
                visible,
                visible_fh,
                0,
                hidden,
                hidden_fh,
                0,
                1,
                0,
            ))
            .unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn hidden_multi_path_mutations_return_enoent_before_readonly() {
        let dir = test_dir("hidden-mutators");
        std::fs::write(dir.join("hidden"), b"secret").unwrap();
        std::fs::write(dir.join("visible"), b"ok").unwrap();
        let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
        let hidden = fs
            .state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(VirtualPath::new("/hidden"));
        let visible = fs
            .reply_entry_for_path(VirtualPath::new("/visible"))
            .unwrap()
            .attr
            .ino;

        assert_eq!(
            block_on(fs.link(dummy_req(), hidden, FUSE_ROOT_ID, OsStr::new("copy"))).unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.rename(
                dummy_req(),
                FUSE_ROOT_ID,
                OsStr::new("hidden"),
                FUSE_ROOT_ID,
                OsStr::new("renamed"),
                0,
            ))
            .unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.rename(
                dummy_req(),
                FUSE_ROOT_ID,
                OsStr::new("visible"),
                FUSE_ROOT_ID,
                OsStr::new("hidden"),
                0,
            ))
            .unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.setxattr(dummy_req(), hidden, OsStr::new("user.test"), b"v", 0))
                .unwrap_err(),
            ENOENT
        );
        assert_eq!(
            block_on(fs.link(dummy_req(), visible, FUSE_ROOT_ID, OsStr::new("hidden")))
                .unwrap_err(),
            ENOENT
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn forget_evicts_non_root_mapping_after_lookup_refs_drop_and_handles_close() {
        let dir = test_dir("forget-eviction");
        std::fs::write(dir.join("file"), b"data").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());

        let entry = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("file"))).unwrap();
        let inode = entry.attr.ino;
        let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();

        fs.forget(dummy_req(), inode, 1);
        {
            let state = fs.state.lock().expect("state mutex poisoned");
            let record = state
                .inodes
                .get(&inode)
                .expect("inode kept while handle open");
            assert_eq!(record.lookup_refs, 0);
            assert_eq!(record.open_refs, 1);
            assert_eq!(
                state.path_inodes.get(&VirtualPath::new("/file")),
                Some(&inode)
            );
        }

        block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
        {
            let state = fs.state.lock().expect("state mutex poisoned");
            assert!(!state.inodes.contains_key(&inode));
            assert!(!state.path_inodes.contains_key(&VirtualPath::new("/file")));
        }

        let replacement =
            block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("file"))).unwrap();
        assert_ne!(replacement.attr.ino, inode);

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn successful_mutations_invalidate_parent_snapshots_and_reused_exact_paths() {
        let dir = test_dir("mutation-invalidation");
        std::fs::write(dir.join("source"), b"src").unwrap();
        std::fs::write(dir.join("stale"), b"old").unwrap();
        std::fs::create_dir(dir.join("again")).unwrap();
        std::fs::write(dir.join("node"), b"old").unwrap();
        std::os::unix::fs::symlink("source", dir.join("sym")).unwrap();
        std::fs::write(dir.join("target"), b"old").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());

        assert_root_recreation_replaces_inode(
            &fs,
            "stale",
            |fs| {
                block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("stale"))).unwrap();
            },
            |fs| {
                let created = block_on(fs.create(
                    dummy_req(),
                    FUSE_ROOT_ID,
                    OsStr::new("stale"),
                    0o644,
                    libc::O_RDWR as u32,
                ))
                .unwrap();
                let inode = created.attr.ino;
                block_on(fs.release(
                    dummy_req(),
                    created.attr.ino,
                    created.fh,
                    0,
                    0,
                    false,
                    false,
                ))
                .unwrap();
                inode
            },
        );

        assert_root_recreation_replaces_inode(
            &fs,
            "again",
            |fs| {
                block_on(fs.rmdir(dummy_req(), FUSE_ROOT_ID, OsStr::new("again"))).unwrap();
            },
            |fs| {
                block_on(fs.mkdir(dummy_req(), FUSE_ROOT_ID, OsStr::new("again"), 0o755, 0))
                    .unwrap()
                    .attr
                    .ino
            },
        );

        assert_root_recreation_replaces_inode(
            &fs,
            "node",
            |fs| {
                block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("node"))).unwrap();
            },
            |fs| {
                block_on(fs.mknod(
                    dummy_req(),
                    FUSE_ROOT_ID,
                    OsStr::new("node"),
                    libc::S_IFREG | 0o644,
                    0,
                ))
                .unwrap()
                .attr
                .ino
            },
        );

        assert_root_recreation_replaces_inode(
            &fs,
            "sym",
            |fs| {
                block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("sym"))).unwrap();
            },
            |fs| {
                block_on(fs.symlink(
                    dummy_req(),
                    FUSE_ROOT_ID,
                    OsStr::new("sym"),
                    OsStr::new("source"),
                ))
                .unwrap()
                .attr
                .ino
            },
        );

        let source = lookup_root_inode(&fs, "source");
        assert_root_recreation_replaces_inode(
            &fs,
            "target",
            |fs| {
                block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("target"))).unwrap();
            },
            |fs| {
                block_on(fs.link(dummy_req(), source, FUSE_ROOT_ID, OsStr::new("target")))
                    .unwrap()
                    .attr
                    .ino
            },
        );

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn successful_rename_invalidates_source_tree_and_parent_snapshot() {
        let dir = test_dir("rename-invalidation");
        std::fs::create_dir(dir.join("rename-src")).unwrap();
        std::fs::write(dir.join("rename-src/child"), b"old").unwrap();
        let fs = fs_for(&dir, Vec::new(), Vec::new());

        let src_dir = lookup_root_inode(&fs, "rename-src");
        let old_child = lookup_child_inode(&fs, src_dir, "child");
        let root_fh = open_directory_handle(&fs, FUSE_ROOT_ID);

        block_on(fs.rename(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("rename-src"),
            FUSE_ROOT_ID,
            OsStr::new("rename-dst"),
            0,
        ))
        .unwrap();
        assert_readdir_handle_invalidated(&fs, FUSE_ROOT_ID, root_fh);
        assert_eq!(
            block_on(fs.getattr(dummy_req(), old_child, None, 0)).unwrap_err(),
            ENOENT
        );

        std::fs::create_dir(dir.join("rename-src")).unwrap();
        std::fs::write(dir.join("rename-src/child"), b"new").unwrap();
        let new_src_dir = lookup_root_inode(&fs, "rename-src");
        assert_ne!(new_src_dir, src_dir);

        let dst_dir = lookup_root_inode(&fs, "rename-dst");
        let new_child = lookup_child_inode(&fs, dst_dir, "child");
        assert_ne!(new_child, old_child);

        std::fs::remove_dir_all(dir).unwrap();
    }

    fn vpath(path: &str) -> VirtualPath {
        VirtualPath::new(path)
    }

    fn tracked_inode(fs: &ScreenFs, path: &str) -> u64 {
        fs.state
            .lock()
            .expect("state mutex poisoned")
            .inode_for_path(vpath(path))
    }

    fn insert_tracked_file_handle(fs: &ScreenFs, inode: u64, path: &str, file: File) -> u64 {
        fs.state
            .lock()
            .expect("state mutex poisoned")
            .insert_file(inode, vpath(path), file)
    }

    fn lookup_child_inode(fs: &ScreenFs, parent: u64, name: &str) -> u64 {
        block_on(fs.lookup(dummy_req(), parent, OsStr::new(name)))
            .unwrap()
            .attr
            .ino
    }

    fn lookup_root_inode(fs: &ScreenFs, name: &str) -> u64 {
        lookup_child_inode(fs, FUSE_ROOT_ID, name)
    }

    fn open_directory_handle(fs: &ScreenFs, inode: u64) -> u64 {
        block_on(fs.opendir(dummy_req(), inode, libc::O_RDONLY as u32))
            .unwrap()
            .fh
    }

    fn root_listing_names(fs: &ScreenFs) -> Vec<String> {
        let fh = open_directory_handle(fs, FUSE_ROOT_ID);
        let names = block_on(fs.readdirplus(dummy_req(), FUSE_ROOT_ID, fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
        block_on(fs.releasedir(dummy_req(), FUSE_ROOT_ID, fh, 0)).unwrap();
        names
    }

    fn assert_root_recreation_replaces_inode(
        fs: &ScreenFs,
        name: &str,
        remove: impl FnOnce(&ScreenFs),
        recreate: impl FnOnce(&ScreenFs) -> u64,
    ) {
        let old_inode = lookup_root_inode(fs, name);
        let root_fh = open_directory_handle(fs, FUSE_ROOT_ID);
        remove(fs);
        assert_readdir_handle_invalidated(fs, FUSE_ROOT_ID, root_fh);

        let root_fh = open_directory_handle(fs, FUSE_ROOT_ID);
        let new_inode = recreate(fs);
        assert_readdir_handle_invalidated(fs, FUSE_ROOT_ID, root_fh);
        assert_ne!(new_inode, old_inode);
    }

    fn assert_readdir_handle_invalidated(fs: &ScreenFs, inode: u64, fh: u64) {
        assert_eq!(
            block_on(fs.readdir(dummy_req(), inode, fh, 0, 4096)).unwrap_err(),
            ENOENT
        );
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        struct Noop;
        impl Wake for Noop {
            fn wake(self: Arc<Self>) {}
        }
        let waker = Waker::from(Arc::new(Noop));
        let mut cx = Context::from_waker(&waker);
        let mut future = Box::pin(future);
        match Pin::new(&mut future).poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("test future unexpectedly pending"),
        }
    }

    fn test_dir(label: &str) -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let mut dir = std::path::PathBuf::from("/tmp/screenfs-tests");
        std::fs::create_dir_all(&dir).unwrap();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let pid = std::process::id();
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        dir.push(format!("screenfs-{label}-{pid}-{id}-{counter}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn dummy_req() -> Request {
        Request {
            unique: 1,
            uid: 0,
            gid: 0,
            pid: 0,
        }
    }

    fn host_statfs(path: &Path) -> ReplyStatfs {
        let c_path = cstring_path(path).unwrap();
        let mut stats = std::mem::MaybeUninit::<libc::statvfs>::zeroed();
        let result = unsafe { libc::statvfs(c_path.as_ptr(), stats.as_mut_ptr()) };
        assert_eq!(result, 0, "statvfs failed for {}", path.display());
        let stats = unsafe { stats.assume_init() };
        let bsize = u32::try_from(stats.f_bsize)
            .ok()
            .filter(|value| *value != 0)
            .unwrap_or(512);
        let frsize = u32::try_from(stats.f_frsize)
            .ok()
            .filter(|value| *value != 0)
            .unwrap_or(bsize);
        let namelen = u32::try_from(stats.f_namemax)
            .ok()
            .filter(|value| *value != 0)
            .unwrap_or(255);
        ReplyStatfs {
            blocks: stats.f_blocks,
            bfree: stats.f_bfree,
            bavail: stats.f_bavail,
            files: stats.f_files,
            ffree: stats.f_ffree,
            bsize,
            namelen,
            frsize,
        }
    }
}
