//! Request-time policy guards for FUSE operations.
//!
//! Guards evaluate visibility before mutability, resolve symlink targets at the
//! point of use, and classify multi-path mutation coordinates.

use std::ffi::OsStr;
use std::fs::{self, File};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use fractal_fuse::{ENOENT, FileAttr, ReplyEntry};

use super::ScreenFs;
use crate::config::{MutabilityDecision, VisibilityDecision};
use crate::errors::{errno_from_io, open_has_write_intent};
use crate::path::VirtualPath;
#[cfg(feature = "perf-counters")]
use std::time::Instant;

// One mutation coordinate carries both direct-path and resolved-target decisions.
#[derive(Debug, Clone)]
struct MutationCoordinateEvaluation<'a> {
    path: &'a VirtualPath,
    follow_final_symlink: bool,
    path_visibility: VisibilityDecision,
    path_mutability: MutabilityDecision,
    resolved: Option<VirtualPath>,
}

// Request-local resolver avoids repeating source-root canonicalization inside one operation.
pub(super) struct RequestPathResolver<'a> {
    fs: &'a ScreenFs,
    source_root: Option<PathBuf>,
}

impl<'a> RequestPathResolver<'a> {
    pub(super) fn new(fs: &'a ScreenFs) -> Self {
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
        let source_root = self.source_root()?;
        #[cfg(feature = "perf-counters")]
        let start = Instant::now();
        #[cfg(feature = "perf-counters")]
        let (source, path_metrics) = path
            .resolve_host_path_from_canonical_source_root_with_metrics(
                source_root,
                follow_final_symlink,
            )
            .map_err(errno_from_io)?;
        #[cfg(not(feature = "perf-counters"))]
        let source = path
            .resolve_host_path_from_canonical_source_root(source_root, follow_final_symlink)
            .map_err(errno_from_io)?;
        #[cfg(feature = "perf-counters")]
        let (result, virtual_conversion) =
            virtual_path_from_source_path_with_metrics(source_root, &source);
        #[cfg(not(feature = "perf-counters"))]
        let result = virtual_path_from_source_path(source_root, &source);
        #[cfg(feature = "perf-counters")]
        {
            self.fs
                .perf
                .record_resolved_virtual_path_from_path(start.elapsed());
            self.fs
                .perf
                .record_resolved_virtual_path_from_path_details(path_metrics, virtual_conversion);
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
        let source_root = self.source_root()?;
        #[cfg(feature = "perf-counters")]
        let start = std::time::Instant::now();
        let fd_path = PathBuf::from(format!("/proc/self/fd/{}", file.as_raw_fd()));
        let source = fs::read_link(fd_path).map_err(errno_from_io)?;
        let result = virtual_path_from_source_path(source_root, &source);
        #[cfg(feature = "perf-counters")]
        self.fs
            .perf
            .record_resolved_virtual_path_from_open_fd(start.elapsed());
        result
    }
}

// Lazy resolution lets read-only checks avoid symlink work until a target decision is needed.
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

#[cfg(feature = "perf-counters")]
fn virtual_path_from_source_path_with_metrics(
    source_root: &Path,
    source: &Path,
) -> (Result<VirtualPath, i32>, std::time::Duration) {
    let start = Instant::now();
    let result = virtual_path_from_source_path(source_root, source);
    (result, start.elapsed())
}

// Guard entry points used by FUSE handlers; each returns a policy errno on denial.
impl ScreenFs {
    fn visible_for_entry_with_known_attr_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        known_child_attr: Option<&FileAttr>,
    ) -> bool {
        match known_child_attr {
            Some(attr) => self.entry_is_readable(path, (attr.mode & libc::S_IFMT) == libc::S_IFDIR),
            None => match self.stat_child_no_follow_with_resolver(resolver, path, 0) {
                Ok(attr) => {
                    self.entry_is_readable(path, (attr.mode & libc::S_IFMT) == libc::S_IFDIR)
                }
                Err(_) => self.is_fully_visible(path),
            },
        }
    }

    pub(super) fn guard_hidden_path(&self, path: &VirtualPath) -> Result<(), i32> {
        self.guard_hidden_path_with_known_attr(path, None)
    }

    fn guard_hidden_path_with_known_attr(
        &self,
        path: &VirtualPath,
        known_child_attr: Option<&FileAttr>,
    ) -> Result<(), i32> {
        let mut resolver = RequestPathResolver::new(self);
        self.guard_hidden_path_with_known_attr_with_resolver(&mut resolver, path, known_child_attr)
    }

    fn guard_hidden_path_with_known_attr_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        known_child_attr: Option<&FileAttr>,
    ) -> Result<(), i32> {
        if self.visible_for_entry_with_known_attr_with_resolver(resolver, path, known_child_attr) {
            Ok(())
        } else {
            Err(ENOENT)
        }
    }

    fn guard_hidden_path_after_child_stat_error(&self, path: &VirtualPath) -> Result<(), i32> {
        if self.is_fully_visible(path) {
            Ok(())
        } else {
            Err(ENOENT)
        }
    }

    #[allow(dead_code)]
    pub(super) fn resolved_virtual_path(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<VirtualPath, i32> {
        let source_root = self.source_root_path()?;
        #[cfg(feature = "perf-counters")]
        let start = Instant::now();
        #[cfg(feature = "perf-counters")]
        let (source, path_metrics) = path
            .resolve_host_path_from_canonical_source_root_with_metrics(
                &source_root,
                follow_final_symlink,
            )
            .map_err(errno_from_io)?;
        #[cfg(not(feature = "perf-counters"))]
        let source = path
            .resolve_host_path_from_canonical_source_root(&source_root, follow_final_symlink)
            .map_err(errno_from_io)?;
        #[cfg(feature = "perf-counters")]
        let (result, virtual_conversion) =
            virtual_path_from_source_path_with_metrics(&source_root, &source);
        #[cfg(not(feature = "perf-counters"))]
        let result = virtual_path_from_source_path(&source_root, &source);
        #[cfg(feature = "perf-counters")]
        {
            self.perf
                .record_resolved_virtual_path_from_path(start.elapsed());
            self.perf
                .record_resolved_virtual_path_from_path_details(path_metrics, virtual_conversion);
        }
        result
    }

    pub(super) fn guard_read_path(&self, path: &VirtualPath) -> Result<(), i32> {
        self.guard_read_path_with_known_attr(path, None)
    }

    pub(super) fn guard_read_path_with_known_attr(
        &self,
        path: &VirtualPath,
        known_child_attr: Option<&FileAttr>,
    ) -> Result<(), i32> {
        self.guard_hidden_path_with_known_attr(path, known_child_attr)?;
        self.guard_resolved_target_visibility_if_needed(path)
    }

    pub(super) fn guard_read_path_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        known_child_attr: Option<&FileAttr>,
    ) -> Result<(), i32> {
        self.guard_hidden_path_with_known_attr_with_resolver(resolver, path, known_child_attr)?;
        self.guard_resolved_target_visibility_if_needed_with_resolver(resolver, path)
    }

    fn guard_read_path_after_child_stat_error_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
    ) -> Result<(), i32> {
        self.guard_hidden_path_after_child_stat_error(path)?;
        self.guard_resolved_target_visibility_if_needed_with_resolver(resolver, path)
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
        self.guard_hidden_path_with_known_attr_with_resolver(resolver, path, None)?;
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
        self.guard_mutation_coordinates_with_resolver(&mut resolver, visible, writable)
    }

    fn guard_mutation_coordinates_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        visible: &[(&VirtualPath, bool)],
        writable: &[(&VirtualPath, bool)],
    ) -> Result<(), i32> {
        let mut coordinates = Vec::with_capacity(visible.len() + writable.len());
        for (path, follow_final_symlink) in visible.iter().copied().chain(writable.iter().copied())
        {
            coordinates.push(self.evaluate_mutation_coordinate_visibility(
                resolver,
                path,
                follow_final_symlink,
            )?);
        }
        for coordinate in coordinates.iter_mut().skip(visible.len()) {
            self.guard_mutation_coordinate_writable(resolver, coordinate)?;
        }
        Ok(())
    }

    pub(super) fn guard_mutation_path(
        &self,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<(), i32> {
        let mut resolver = RequestPathResolver::new(self);
        self.guard_mutation_path_with_resolver(&mut resolver, path, follow_final_symlink)
    }

    pub(super) fn guard_mutation_path_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        follow_final_symlink: bool,
    ) -> Result<(), i32> {
        self.guard_mutation_coordinates_with_resolver(
            resolver,
            &[],
            &[(path, follow_final_symlink)],
        )
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
        let mut resolver = RequestPathResolver::new(self);
        self.guard_resolved_target_visibility_if_needed_with_resolver(&mut resolver, path)
    }

    fn guard_resolved_target_visibility_if_needed_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
    ) -> Result<(), i32> {
        if self.cfg.can_skip_symlink_target_visibility_check() {
            return Ok(());
        }
        let resolved = match resolver.resolved_virtual_path(path, true) {
            Ok(resolved) => resolved,
            Err(ENOENT) => return Ok(()),
            Err(err) => return Err(err),
        };
        self.guard_resolved_target_fully_visible(path, &resolved)
    }

    pub(super) fn attr_for_path(&self, path: &VirtualPath, inode: u64) -> Result<FileAttr, i32> {
        let mut resolver = RequestPathResolver::new(self);
        match self.stat_child_no_follow_with_resolver(&mut resolver, path, inode) {
            Ok(attr) => {
                self.guard_read_path_with_resolver(&mut resolver, path, Some(&attr))?;
                Ok(attr)
            }
            Err(err) => {
                self.guard_read_path_after_child_stat_error_with_resolver(&mut resolver, path)?;
                Err(err)
            }
        }
    }

    pub(super) fn reply_entry_for_path(&self, path: VirtualPath) -> Result<ReplyEntry, i32> {
        let mut resolver = RequestPathResolver::new(self);
        let mut attr = match self.stat_child_no_follow_with_resolver(&mut resolver, &path, 0) {
            Ok(attr) => attr,
            Err(err) => {
                self.guard_read_path_after_child_stat_error_with_resolver(&mut resolver, &path)?;
                return Err(err);
            }
        };
        self.guard_read_path_with_resolver(&mut resolver, &path, Some(&attr))?;
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

    // open/opendir time this as the pre-open guard phase; fd-based revalidation
    // stays in the guard_opened_* helpers below.
    pub(super) fn guard_open_flags_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        flags: u32,
    ) -> Result<(), i32> {
        if open_has_write_intent(flags) {
            self.guard_mutation_path_with_resolver(resolver, path, true)
        } else {
            self.guard_read_path_with_resolver(resolver, path, None)
        }
    }

    // access times this as the pre-open guard phase; fd-based revalidation stays
    // in guard_opened_file_target_with_resolver.
    pub(super) fn guard_access_mask_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        mask: u32,
    ) -> Result<(), i32> {
        if mask & libc::W_OK as u32 != 0 {
            self.guard_mutation_path_with_resolver(resolver, path, true)
        } else {
            self.guard_read_path_with_resolver(resolver, path, None)
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

    #[allow(dead_code)]
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
        let mut resolver = RequestPathResolver::new(self);
        self.guard_opened_file_target_with_resolver(&mut resolver, path, file, mutation)
    }

    // open/access time this as the post-open fd revalidation phase.
    pub(super) fn guard_opened_file_target_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = resolver.resolved_virtual_path_for_open_file(file)?;
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
        let mut resolver = RequestPathResolver::new(self);
        self.guard_opened_directory_target_with_resolver(&mut resolver, path, file, mutation)
    }

    // opendir times this as the post-open fd revalidation phase.
    pub(super) fn guard_opened_directory_target_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = resolver.resolved_virtual_path_for_open_file(file)?;
        self.guard_resolved_target_fully_visible(path, &resolved)?;
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
        self.guard_opened_directory_at_path_with_resolver(&mut resolver, path, file, mutation)
    }

    pub(super) fn guard_opened_directory_at_path_with_resolver(
        &self,
        resolver: &mut RequestPathResolver<'_>,
        path: &VirtualPath,
        file: &File,
        mutation: bool,
    ) -> Result<(), i32> {
        let resolved = resolver.resolved_virtual_path_for_open_file(file)?;
        let expected = resolver.resolved_virtual_path(path, true)?;
        if resolved != expected {
            return Err(ENOENT);
        }
        self.guard_resolved_target_fully_visible(path, &resolved)?;
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
