use super::*;

#[test]
fn readdirplus_dot_entries_do_not_pin_lookup_refs() {
    let dir = test_dir("readdirplus-dot-no-pin");
    std::fs::create_dir(dir.join("a")).unwrap();
    std::fs::create_dir(dir.join("a/b")).unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let a_inode = lookup_root_inode(&fs, "a");
    let b_inode = lookup_child_inode(&fs, a_inode, "b");
    let b_fh = open_directory_handle(&fs, b_inode);
    let entries = block_on(fs.readdirplus(dummy_req(), b_inode, b_fh, 0, 4096)).unwrap();
    assert!(entries.iter().any(|entry| entry.name == b"."));
    assert!(entries.iter().any(|entry| entry.name == b".."));
    block_on(fs.releasedir(dummy_req(), b_inode, b_fh, 0)).unwrap();

    fs.forget(dummy_req(), b_inode, 1);
    let replacement = lookup_child_inode(&fs, a_inode, "b");
    assert_ne!(replacement, b_inode);

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

    let replacement = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("file"))).unwrap();
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
