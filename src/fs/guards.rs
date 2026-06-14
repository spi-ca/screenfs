use std::ffi::OsStr;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use fractal_fuse::{ENOENT, FileAttr, ReplyEntry};

use super::ScreenFs;
use crate::config::{MutabilityDecision, VisibilityDecision};
use crate::errors::{errno_from_io, open_has_write_intent};
use crate::path::VirtualPath;

#[derive(Debug, Clone)]
struct MutationCoordinateEvaluation<'a> {
    path: &'a VirtualPath,
    follow_final_symlink: bool,
    path_visibility: VisibilityDecision,
    path_mutability: MutabilityDecision,
    resolved: Option<VirtualPath>,
}

struct RequestPathResolver<'a> {
    fs: &'a ScreenFs,
    source_root: Option<PathBuf>,
}

impl<'a> RequestPathResolver<'a> {
    fn new(fs: &'a ScreenFs) -> Self {
        Self {
            fs,
            source_root: None,
        }
    }

    fn source_root(&mut self) -> Result<&Path, i32> {
        if self.source_root.is_none() {
            self.source_root = Some(self.fs.source_root_path()?);
        }
        Ok(self
            .source_root
            .as_deref()
            .expect("request-local source root cached"))
    }

    fn resolved_virtual_path(
        &mut self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<VirtualPath, i32> {
        #[cfg(feature = "perf-counters")]
        let start = self.fs.perf.as_ref().map(|_| std::time::Instant::now());
        let source_root = self.source_root()?;
        let source = path
            .resolve_host_path(source_root, follow_final_symlink)
            .map_err(errno_from_io)?;
        let result = virtual_path_from_source_path(source_root, &source);
        #[cfg(feature = "perf-counters")]
        if let (Some(perf), Some(start)) = (self.fs.perf.as_ref(), start) {
            perf.record_resolved_virtual_path(start.elapsed());
        }
        result
    }

    fn resolve_mutation_coordinate_target(
        &mut self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<VirtualPath, i32> {
        match self.resolved_virtual_path(path, follow_final_symlink) {
            Ok(resolved) => Ok(resolved),
            Err(ENOENT) if !follow_final_symlink => Ok(path.clone()),
            Err(err) => Err(err),
        }
    }

    fn resolved_virtual_path_for_open_file(&mut self, file: &File) -> Result<VirtualPath, i32> {
        #[cfg(feature = "perf-counters")]
        let start = self.fs.perf.as_ref().map(|_| std::time::Instant::now());
        let fd_path = PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()));
        let source = fs::read_link(fd_path).map_err(errno_from_io)?;
        let source_root = self.source_root()?;
        let result = virtual_path_from_source_path(source_root, &source);
        #[cfg(feature = "perf-counters")]
        if let (Some(perf), Some(start)) = (self.fs.perf.as_ref(), start) {
            perf.record_resolved_virtual_path(start.elapsed());
        }
        result
    }
}

impl<'a> MutationCoordinateEvaluation<'a> {
    fn resolved_path<'b>(
        &'b mut self,
        resolver: &mut RequestPathResolver<'_>,
    ) -> Result<&'b VirtualPath, i32> {
        if self.resolved.is_none() {
            self.resolved = Some(
                resolver
                    .resolve_mutation_coordinate_target(self.path, self.follow_final_symlink)?,
            );
        }
        Ok(self.resolved.as_ref().expect("resolved path cached"))
    }
}

fn virtual_path_from_source_path(source_root: &Path, source: &Path) -> Result<VirtualPath, i32> {
    let relative = source.strip_prefix(source_root).map_err(|_| ENOENT)?;
    if relative.as_os_str().is_empty() {
        Ok(VirtualPath::root())
    } else {
        let mut resolved = PathBuf::from("/");
        resolved.push(relative);
        Ok(VirtualPath::new(resolved))
    }
}

impl ScreenFs {
    pub(super) fn visible_for_entry(&self, path: &VirtualPath) -> bool {
        match self.stat_child_no_follow(path, 0) {
            Ok(attr) => self.entry_is_readable(path, (attr.mode & libc::S_IFMT) == libc::S_IFDIR),
            Err(_) => self.is_fully_visible(path),
        }
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
        #[cfg(feature = "perf-counters")]
        let start = self.perf.as_ref().map(|_| std::time::Instant::now());
        let source_root = self.source_root_path()?;
        let source = path
            .resolve_host_path(&source_root, follow_final_symlink)
            .map_err(errno_from_io)?;
        let result = virtual_path_from_source_path(&source_root, &source);
        #[cfg(feature = "perf-counters")]
        if let (Some(perf), Some(start)) = (self.perf.as_ref(), start) {
            perf.record_resolved_virtual_path(start.elapsed());
        }
        result
    }

    pub(super) fn guard_read_path(&self, path: &VirtualPath) -> Result<(), i32> {
        self.guard_hidden_path(path)?;
        self.guard_resolved_target_visibility_if_needed(path)
    }

    fn guard_resolved_target_fully_visible(
        &self,
        path: &VirtualPath,
        resolved: &VirtualPath,
    ) -> Result<(), i32> {
        if resolved != path && !self.is_fully_visible(resolved) {
            Err(ENOENT)
        } else {
            Ok(())
        }
    }

    fn evaluate_mutation_coordinate_visibility<'a>(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &'a VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<MutationCoordinateEvaluation<'a>, i32> {
        self.guard_hidden_path(path)?;
        let path_visibility = self.visibility_decision(path);
        let path_mutability = self.mutability_decision(path);
        let mut coordinate = MutationCoordinateEvaluation {
            path,
            follow_final_symlink,
            path_visibility,
            path_mutability,
            resolved: None,
        };
        if !self.cfg.can_skip_symlink_target_visibility_check() {
            let resolved =
                resolver.resolve_mutation_coordinate_target(path, follow_final_symlink)?;
            self.guard_resolved_target_fully_visible(path, &resolved)?;
            coordinate.resolved = Some(resolved);
        }
        Ok(coordinate)
    }

    fn guard_mutation_coordinate_writable(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        coordinate: &mut MutationCoordinateEvaluation<'_>,
    ) -> Result<(), i32> {
        if matches!(
            coordinate.path_visibility,
            VisibilityDecision::BridgeVisible
        ) {
            return Err(libc::EROFS);
        }
        if coordinate.path_mutability.is_readonly() {
            return Err(libc::EROFS);
        }
        if self
            .cfg
            .can_skip_resolved_target_mutability_check(coordinate.path_mutability)
        {
            return Ok(());
        }

        let path = coordinate.path.clone();
        let resolved = coordinate.resolved_path(resolver)?;
        if *resolved == path {
            return Ok(());
        }
        if self.mutability_decision(resolved).is_readonly() {
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
        let mut resolver = RequestPathResolver::new(self);
        let mut coordinates = Vec::with_capacity(visible.len() + writable.len());
        for (path, follow_final_symlink) in visible.iter().copied().chain(writable.iter().copied())
        {
            coordinates.push(self.evaluate_mutation_coordinate_visibility(
                &mut resolver,
                path,
                follow_final_symlink,
            )?);
        }
        for coordinate in coordinates.iter_mut().skip(visible.len()) {
            self.guard_mutation_coordinate_writable(&mut resolver, coordinate)?;
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

    pub(super) fn guard_existing_entry_target_visibility(
        &self,
        path: &VirtualPath,
    ) -> Result<(), i32> {
        self.guard_hidden_path(path)?;
        if self.cfg.can_skip_symlink_target_visibility_check() {
            return Ok(());
        }
        match self.stat_child_no_follow(path, 0) {
            Ok(attr) if attr.mode & libc::S_IFMT == libc::S_IFLNK => {
                self.guard_resolved_target_visibility_if_needed(path)
            }
            Ok(_) | Err(ENOENT) => Ok(()),
            Err(err) => Err(err),
        }
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
        self.guard_resolved_target_fully_visible(path, &resolved)
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
        if self.is_hidden_symlink_target(path, raw_target) {
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
        self.guard_existing_entry_target_visibility(path)?;
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
        RequestPathResolver::new(self).resolved_virtual_path_for_open_file(file)
    }

    pub(super) fn guard_opened_file_target(
        &self,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = self.resolved_virtual_path_for_open_file(file)?;
        if !self.is_fully_visible(&resolved) {
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
        if !self.entry_is_readable(&resolved, true) {
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
        let mut resolver = RequestPathResolver::new(self);
        let resolved = resolver.resolved_virtual_path_for_open_file(file)?;
        let expected = resolver.resolved_virtual_path(path, true)?;
        if resolved != expected {
            return Err(ENOENT);
        }
        if !self.entry_is_readable(&resolved, true) {
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
        if !mutation {
            return Ok(());
        }
        if resolved != path && !self.is_fully_visible(resolved) {
            return Err(ENOENT);
        }

        let path_visibility = self.visibility_decision(path);
        if matches!(path_visibility, VisibilityDecision::BridgeVisible) {
            return Err(libc::EROFS);
        }

        let path_mutability = self.mutability_decision(path);
        if path_mutability.is_readonly() {
            return Err(libc::EROFS);
        }
        if resolved == path
            || self
                .cfg
                .can_skip_resolved_target_mutability_check(path_mutability)
        {
            return Ok(());
        }
        if self.mutability_decision(resolved).is_readonly() {
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
