use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::File;
use std::os::unix::ffi::OsStrExt;

use fractal_fuse::abi::FUSE_ROOT_ID;
use fractal_fuse::{ENOENT, FileAttr, FileType};

use crate::path::VirtualPath;

use super::ScreenFs;
use super::backing::{file_type_from_metadata, metadata_to_attr};

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
    pub(super) file: File,
}

#[derive(Debug, Clone)]
pub(super) struct DirectorySnapshotEntry {
    pub(super) ino: u64,
    pub(super) offset: u64,
    pub(super) kind: FileType,
    pub(super) name: Vec<u8>,
    pub(super) attr: FileAttr,
}

#[derive(Debug)]
pub(super) struct DirectoryHandle {
    pub(super) inode: u64,
    pub(super) path: VirtualPath,
    pub(super) entries: Vec<DirectorySnapshotEntry>,
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
        if inode != FUSE_ROOT_ID
            && let Some(record) = self.inodes.get_mut(&inode)
        {
            record.lookup_refs += 1;
        }
        inode
    }

    fn next_handle(&mut self) -> u64 {
        let fh = self.next_fh;
        self.next_fh += 1;
        fh
    }

    fn try_evict_inode(&mut self, inode: u64) {
        if inode == FUSE_ROOT_ID {
            return;
        }
        let should_remove = self
            .inodes
            .get(&inode)
            .map(|record| record.lookup_refs == 0 && record.open_refs == 0)
            .unwrap_or(false);
        if !should_remove {
            return;
        }
        if let Some(record) = self.inodes.remove(&inode)
            && let Some(path) = record.path
            && self.path_inodes.get(&path) == Some(&inode)
        {
            self.path_inodes.remove(&path);
        }
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
        self.files.insert(fh, FileHandle { inode, path, file });
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

    pub(super) fn insert_directory(
        &mut self,
        inode: u64,
        path: VirtualPath,
        entries: Vec<DirectorySnapshotEntry>,
    ) -> u64 {
        let fh = self.next_handle();
        self.directories.insert(
            fh,
            DirectoryHandle {
                inode,
                path,
                entries,
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
}

impl ScreenFs {
    pub(super) fn path_for_inode(&self, inode: u64) -> Result<VirtualPath, i32> {
        let state = self.state.lock().expect("state mutex poisoned");
        state.path_for_inode(inode).ok_or(ENOENT)
    }

    pub(super) fn child_path(&self, parent: u64, name: &OsStr) -> Result<VirtualPath, i32> {
        let parent = self.path_for_inode(parent)?;
        Ok(parent.join_child(name))
    }

    pub(super) fn track_path(&self, path: VirtualPath) -> u64 {
        self.state
            .lock()
            .expect("state mutex poisoned")
            .lookup_path(path)
    }

    pub(super) fn insert_open_file(&self, inode: u64, path: VirtualPath, file: File) -> u64 {
        self.state
            .lock()
            .expect("state mutex poisoned")
            .insert_file(inode, path, file)
    }

    pub(super) fn insert_open_directory(
        &self,
        inode: u64,
        path: VirtualPath,
        entries: Vec<DirectorySnapshotEntry>,
    ) -> u64 {
        self.state
            .lock()
            .expect("state mutex poisoned")
            .insert_directory(inode, path, entries)
    }

    pub(super) fn remove_open_directory(&self, fh: u64) {
        self.state
            .lock()
            .expect("state mutex poisoned")
            .remove_directory(fh);
    }

    pub(super) fn with_file_handle<R>(
        &self,
        inode: u64,
        fh: u64,
        f: impl FnOnce(&FileHandle) -> Result<R, i32>,
    ) -> Result<R, i32> {
        let state = self.state.lock().expect("state mutex poisoned");
        let handle = state.files.get(&fh).ok_or(ENOENT)?;
        if handle.inode != inode {
            return Err(ENOENT);
        }
        f(handle)
    }

    pub(super) fn with_file_handle_mut<R>(
        &self,
        inode: u64,
        fh: u64,
        f: impl FnOnce(&mut FileHandle) -> Result<R, i32>,
    ) -> Result<R, i32> {
        let mut state = self.state.lock().expect("state mutex poisoned");
        let handle = state.files.get_mut(&fh).ok_or(ENOENT)?;
        if handle.inode != inode {
            return Err(ENOENT);
        }
        f(handle)
    }

    pub(super) fn directory_snapshot(
        &self,
        dir: &VirtualPath,
        inode: u64,
    ) -> Result<Vec<DirectorySnapshotEntry>, i32> {
        let parent = Self::parent_path(dir);
        let children = self.dir_entries(dir)?;
        let (parent_ino, child_inos) = {
            let mut state = self.state.lock().expect("state mutex poisoned");
            let parent_ino = state.inode_for_path(parent.clone());
            let child_inos = children
                .iter()
                .map(|(_, child, _)| state.inode_for_path(child.clone()))
                .collect::<Vec<_>>();
            (parent_ino, child_inos)
        };
        let mut snapshot = vec![
            DirectorySnapshotEntry {
                ino: inode,
                offset: 1,
                kind: FileType::Directory,
                name: b".".to_vec(),
                attr: self.attr_for_path(dir, inode)?,
            },
            DirectorySnapshotEntry {
                ino: parent_ino,
                offset: 2,
                kind: FileType::Directory,
                name: b"..".to_vec(),
                attr: self.attr_for_path(&parent, parent_ino)?,
            },
        ];
        for ((name, _child, metadata), child_ino) in children.into_iter().zip(child_inos) {
            snapshot.push(DirectorySnapshotEntry {
                ino: child_ino,
                offset: snapshot.len() as u64 + 1,
                kind: file_type_from_metadata(&metadata),
                name: name.as_bytes().to_vec(),
                attr: metadata_to_attr(&metadata, child_ino),
            });
        }
        Ok(snapshot)
    }

    pub(super) fn opendir_snapshot(
        &self,
        inode: u64,
        fh: u64,
    ) -> Result<Vec<DirectorySnapshotEntry>, i32> {
        let state = self.state.lock().expect("state mutex poisoned");
        let handle = state.directories.get(&fh).ok_or(ENOENT)?;
        if handle.inode != inode {
            return Err(ENOENT);
        }
        Ok(handle.entries.clone())
    }

    pub(super) fn finalize_created_file(
        &self,
        parent_path: &VirtualPath,
        path: VirtualPath,
        file: File,
    ) -> (u64, u64) {
        let mut state = self.state.lock().expect("state mutex poisoned");
        state.invalidate_directory_snapshots(std::slice::from_ref(parent_path));
        state.invalidate_exact_path(&path);
        let inode = state.lookup_path(path.clone());
        let fh = state.insert_file(inode, path, file);
        (inode, fh)
    }

    pub(super) fn invalidate_after_mutation(
        &self,
        parents: &[VirtualPath],
        exact_paths: &[VirtualPath],
        tree_paths: &[VirtualPath],
    ) {
        let mut state = self.state.lock().expect("state mutex poisoned");
        state.invalidate_directory_snapshots(parents);
        for path in exact_paths {
            state.invalidate_exact_path(path);
        }
        for path in tree_paths {
            state.invalidate_path_tree(path);
        }
    }

    pub(super) fn forget_tracked_inode(&self, inode: u64, nlookup: u64) {
        self.state
            .lock()
            .expect("state mutex poisoned")
            .forget_inode(inode, nlookup);
    }
}
