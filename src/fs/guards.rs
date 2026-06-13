use std::ffi::OsStr;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::PathBuf;

use fractal_fuse::{ENOENT, FileAttr, ReplyEntry};

use super::ScreenFs;
use super::backing::DirEntryInfo;
use crate::errors::{errno_from_io, open_has_write_intent};
use crate::path::VirtualPath;

impl ScreenFs {
    pub(super) fn hidden(&self, path: &VirtualPath) -> bool {
        self.cfg.is_hidden(path)
    }

    pub(super) fn visible_for_entry(&self, path: &VirtualPath) -> bool {
        match self.stat_child_no_follow(path, 0) {
            Ok(attr) => self
                .cfg
                .entry_is_readable(path, (attr.mode & libc::S_IFMT) == libc::S_IFDIR),
            Err(_) => self.cfg.is_fully_visible(path),
        }
    }

    pub(super) fn readonly(&self, path: &VirtualPath) -> bool {
        self.cfg.is_readonly(path)
    }

    pub(super) fn guard_hidden_path(&self, path: &VirtualPath) -> Result<(), i32> {
        if self.visible_for_entry(path) {
            Ok(())
        } else {
            Err(ENOENT)
        }
    }

    pub(super) fn resolved_virtual_path(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<VirtualPath, i32> {
        let source = self.host_path(path, follow_final_symlink)?;
        let source_root = self.source_root_path()?;
        let relative = source.strip_prefix(&source_root).map_err(|_| ENOENT)?;
        if relative.as_os_str().is_empty() {
            Ok(VirtualPath::root())
        } else {
            let mut resolved = PathBuf::from("/");
            resolved.push(relative);
            Ok(VirtualPath::new(resolved))
        }
    }

    pub(super) fn guard_read_path(&self, path: &VirtualPath) -> Result<(), i32> {
        self.guard_hidden_path(path)?;
        self.guard_resolved_target_visibility_if_needed(path)
    }

    pub(super) fn guard_coordinate_hidden(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<(), i32> {
        self.guard_hidden_path(path)?;
        let resolved = match self.resolved_virtual_path(path, follow_final_symlink) {
            Ok(resolved) => resolved,
            Err(ENOENT) if !follow_final_symlink => path.clone(),
            Err(err) => return Err(err),
        };
        if self.hidden(&resolved) {
            Err(ENOENT)
        } else {
            Ok(())
        }
    }

    pub(super) fn guard_coordinate_writable(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<(), i32> {
        let resolved = match self.resolved_virtual_path(path, follow_final_symlink) {
            Ok(resolved) => resolved,
            Err(ENOENT) if !follow_final_symlink => path.clone(),
            Err(err) => return Err(err),
        };
        if self.cfg.visibility_blocks_mutation(path, &resolved)
            || self.readonly(path)
            || self.readonly(&resolved)
        {
            Err(libc::EROFS)
        } else {
            Ok(())
        }
    }

    pub(super) fn guard_mutation_coordinates(
        &self,
        visible: &[(&VirtualPath, bool)],
        writable: &[(&VirtualPath, bool)],
    ) -> Result<(), i32> {
        for (path, follow_final_symlink) in visible.iter().copied().chain(writable.iter().copied())
        {
            self.guard_coordinate_hidden(path, follow_final_symlink)?;
        }
        for (path, follow_final_symlink) in writable {
            self.guard_coordinate_writable(path, *follow_final_symlink)?;
        }
        Ok(())
    }

    pub(super) fn guard_mutation_path(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<(), i32> {
        self.guard_mutation_coordinates(&[], &[(path, follow_final_symlink)])
    }

    pub(super) fn guard_multi_path_mutation(
        &self,
        paths: &[(&VirtualPath, bool)],
    ) -> Result<(), i32> {
        self.guard_mutation_coordinates(&[], paths)
    }

    pub(super) fn guard_resolved_target_visibility_if_needed(
        &self,
        path: &VirtualPath,
    ) -> Result<(), i32> {
        if self.cfg.can_skip_symlink_target_visibility_check() {
            return Ok(());
        }
        let resolved = match self.resolved_virtual_path(path, true) {
            Ok(resolved) => resolved,
            Err(ENOENT) => return Ok(()),
            Err(err) => return Err(err),
        };
        if resolved != *path && !self.cfg.is_fully_visible(&resolved) {
            Err(ENOENT)
        } else {
            Ok(())
        }
    }

    pub(super) fn attr_for_path(&self, path: &VirtualPath, inode: u64) -> Result<FileAttr, i32> {
        self.guard_read_path(path)?;
        self.stat_child_no_follow(path, inode)
    }

    pub(super) fn reply_entry_for_path(&self, path: VirtualPath) -> Result<ReplyEntry, i32> {
        self.guard_read_path(&path)?;
        let mut attr = self.stat_child_no_follow(&path, 0)?;
        let inode = self.track_path(path);
        attr.ino = inode;
        Ok(ReplyEntry {
            ttl: self.cfg.entry_ttl,
            attr,
            generation: 0,
        })
    }

    pub(super) fn check_hidden_symlink_target(
        &self,
        path: &VirtualPath,
        raw_target: &OsStr,
    ) -> Result<(), i32> {
        if self.cfg.is_hidden_symlink_target(path, raw_target) {
            Err(ENOENT)
        } else {
            Ok(())
        }
    }

    pub(super) fn parent_path(path: &VirtualPath) -> VirtualPath {
        path.as_path()
            .parent()
            .map(VirtualPath::new)
            .unwrap_or_else(VirtualPath::root)
    }

    pub(super) fn dir_entries(&self, dir: &VirtualPath) -> Result<Vec<DirEntryInfo>, i32> {
        self.guard_read_path(dir)?;
        let dir_file = self.open_confined(dir, libc::O_RDONLY | libc::O_DIRECTORY, None)?;
        self.guard_opened_directory_target(dir, &dir_file, false)?;
        let mut entries = super::backing::read_dir_entries(dir_file, dir, 3)?;
        entries.retain(|entry| {
            self.cfg.entry_is_readable(&entry.child, entry.is_dir)
                && (!entry.is_symlink
                    || self
                        .guard_resolved_target_visibility_if_needed(&entry.child)
                        .is_ok())
        });
        entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries)
    }

    pub(super) fn guard_open_flags(&self, path: &VirtualPath, flags: u32) -> Result<(), i32> {
        if open_has_write_intent(flags) {
            self.guard_mutation_path(path, true)
        } else {
            self.guard_read_path(path)
        }
    }

    pub(super) fn guard_access_mask(&self, path: &VirtualPath, mask: u32) -> Result<(), i32> {
        if mask & libc::W_OK as u32 != 0 {
            self.guard_mutation_path(path, true)
        } else {
            self.guard_read_path(path)
        }
    }

    pub(super) fn guard_child_mutation_path(&self, path: &VirtualPath) -> Result<VirtualPath, i32> {
        let parent_path = Self::parent_path(path);
        self.guard_mutation_coordinates(&[], &[(path, false), (&parent_path, true)])?;
        Ok(parent_path)
    }

    pub(super) fn guarded_child_mutation(
        &self,
        parent: u64,
        name: &OsStr,
    ) -> Result<(VirtualPath, VirtualPath), i32> {
        let path = self.child_path(parent, name)?;
        let parent_path = self.guard_child_mutation_path(&path)?;
        Ok((path, parent_path))
    }

    pub(super) fn resolved_virtual_path_for_open_file(
        &self,
        file: &File,
    ) -> Result<VirtualPath, i32> {
        let fd_path = PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()));
        let source = fs::read_link(fd_path).map_err(errno_from_io)?;
        let source_root = self.source_root_path()?;
        let relative = source.strip_prefix(&source_root).map_err(|_| ENOENT)?;
        if relative.as_os_str().is_empty() {
            Ok(VirtualPath::root())
        } else {
            let mut resolved = PathBuf::from("/");
            resolved.push(relative);
            Ok(VirtualPath::new(resolved))
        }
    }

    pub(super) fn guard_opened_file_target(
        &self,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = self.resolved_virtual_path_for_open_file(file)?;
        if !self.cfg.is_fully_visible(&resolved) {
            return Err(ENOENT);
        }
        self.guard_opened_writable_target(path, &resolved, mutation)
    }

    pub(super) fn guard_opened_directory_target(
        &self,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = self.resolved_virtual_path_for_open_file(file)?;
        if !self.cfg.entry_is_readable(&resolved, true) {
            return Err(ENOENT);
        }
        self.guard_opened_writable_target(path, &resolved, mutation)
    }

    pub(super) fn guard_opened_directory_at_path(
        &self,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = self.resolved_virtual_path_for_open_file(file)?;
        let expected = self.resolved_virtual_path(path, true)?;
        if resolved != expected {
            return Err(ENOENT);
        }
        if !self.cfg.entry_is_readable(&resolved, true) {
            return Err(ENOENT);
        }
        self.guard_opened_writable_target(path, &resolved, mutation)
    }

    fn guard_opened_writable_target(
        &self,
        path: &VirtualPath,
        resolved: &VirtualPath,
        mutation: bool,
    ) -> Result<(), i32> {
        if mutation
            && (self.cfg.visibility_blocks_mutation(path, resolved)
                || self.readonly(path)
                || self.readonly(resolved))
        {
            return Err(libc::EROFS);
        }
        Ok(())
    }

    pub(super) fn xattr_file(&self, inode: u64, mutation: bool) -> Result<File, i32> {
        let path = self.path_for_inode(inode)?;
        if mutation {
            self.guard_mutation_path(&path, true)?;
        } else {
            self.guard_read_path(&path)?;
        }
        let file = self.open_confined(&path, libc::O_RDONLY, None)?;
        self.guard_opened_file_target(&path, &file, mutation)?;
        Ok(file)
    }
}
