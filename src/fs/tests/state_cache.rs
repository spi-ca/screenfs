//! State, lookup-ref, directory-cookie, and invalidation behavior tests.

use super::*;

// Test cases are grouped by the behavior named in each function.
#[test]
fn failed_lookup_does_not_pin_unknown_inode() {
    let dir = test_dir("failed-lookup-no-pin");
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    assert_eq!(
        block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("missing"))).unwrap_err(),
        ENOENT
    );

    let state = fs.state.read().expect("state rwlock poisoned");
    assert!(
        !state
            .path_inodes
            .contains_key(&VirtualPath::new("/missing"))
    );
    assert_eq!(state.inodes.len(), 1);
    drop(state);
    std::fs::remove_dir_all(dir).unwrap();
}

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
        let state = fs.state.read().expect("state rwlock poisoned");
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
        let state = fs.state.read().expect("state rwlock poisoned");
        assert!(!state.inodes.contains_key(&inode));
        assert!(!state.path_inodes.contains_key(&VirtualPath::new("/file")));
    }

    let replacement = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("file"))).unwrap();
    assert_ne!(replacement.attr.ino, inode);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn mutation_invalidation_detaches_path_and_evicts_after_forget_without_touching_sibling() {
    let dir = test_dir("mutation-invalidation-forget-eviction");
    std::fs::write(dir.join("victim"), b"old").unwrap();
    std::fs::write(dir.join("sibling"), b"keep").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let victim = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("victim"))).unwrap();
    let sibling = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("sibling"))).unwrap();

    block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("victim"))).unwrap();
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        let victim_record = state
            .inodes
            .get(&victim.attr.ino)
            .expect("lookup ref keeps invalidated inode until forget");
        assert_eq!(victim_record.path, None);
        assert_eq!(victim_record.lookup_refs, 1);
        assert!(!state.path_inodes.contains_key(&VirtualPath::new("/victim")));
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    fs.forget(dummy_req(), victim.attr.ino, 1);
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        assert!(!state.inodes.contains_key(&victim.attr.ino));
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    fs.forget(dummy_req(), sibling.attr.ino, 1);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rename_invalidation_detaches_subtree_and_keeps_unrelated_sibling_mapping() {
    let dir = test_dir("rename-invalidation-subtree");
    std::fs::create_dir(dir.join("old")).unwrap();
    std::fs::write(dir.join("old/child"), b"child").unwrap();
    std::fs::create_dir(dir.join("oldish")).unwrap();
    std::fs::write(dir.join("oldish/child"), b"keep-prefix-neighbor").unwrap();
    std::fs::write(dir.join("sibling"), b"keep").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let old_dir = lookup_root_inode(&fs, "old");
    let old_child = lookup_child_inode(&fs, old_dir, "child");
    let oldish = lookup_root_inode(&fs, "oldish");
    let oldish_child = lookup_child_inode(&fs, oldish, "child");
    let sibling = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("sibling"))).unwrap();

    block_on(fs.rename(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("old"),
        FUSE_ROOT_ID,
        OsStr::new("new"),
        0,
    ))
    .unwrap();

    {
        let state = fs.state.read().expect("state rwlock poisoned");
        let old_record = state
            .inodes
            .get(&old_dir)
            .expect("lookup ref keeps invalidated source dir until forget");
        let child_record = state
            .inodes
            .get(&old_child)
            .expect("lookup ref keeps invalidated source child until forget");
        assert_eq!(old_record.path, None);
        assert_eq!(child_record.path, None);
        assert!(!state.path_inodes.contains_key(&VirtualPath::new("/old")));
        assert!(
            !state
                .path_inodes
                .contains_key(&VirtualPath::new("/old/child"))
        );
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/oldish")),
            Some(&oldish)
        );
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/oldish/child")),
            Some(&oldish_child)
        );
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    fs.forget(dummy_req(), old_child, 1);
    fs.forget(dummy_req(), old_dir, 1);
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        assert!(!state.inodes.contains_key(&old_child));
        assert!(!state.inodes.contains_key(&old_dir));
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/oldish")),
            Some(&oldish)
        );
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/oldish/child")),
            Some(&oldish_child)
        );
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    fs.forget(dummy_req(), oldish_child, 1);
    fs.forget(dummy_req(), oldish, 1);
    fs.forget(dummy_req(), sibling.attr.ino, 1);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rename_overwrite_invalidates_source_and_target_but_not_sibling() {
    let dir = test_dir("rename-overwrite-invalidation");
    std::fs::write(dir.join("source"), b"new").unwrap();
    std::fs::write(dir.join("target"), b"old").unwrap();
    std::fs::write(dir.join("sibling"), b"keep").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let source = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("source"))).unwrap();
    let target = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("target"))).unwrap();
    let sibling = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("sibling"))).unwrap();

    block_on(fs.rename(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("source"),
        FUSE_ROOT_ID,
        OsStr::new("target"),
        0,
    ))
    .unwrap();

    {
        let state = fs.state.read().expect("state rwlock poisoned");
        let source_record = state
            .inodes
            .get(&source.attr.ino)
            .expect("source lookup ref keeps invalidated inode until forget");
        let target_record = state
            .inodes
            .get(&target.attr.ino)
            .expect("target lookup ref keeps overwritten inode until forget");
        assert_eq!(source_record.path, None);
        assert_eq!(target_record.path, None);
        assert!(!state.path_inodes.contains_key(&VirtualPath::new("/source")));
        assert!(!state.path_inodes.contains_key(&VirtualPath::new("/target")));
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    fs.forget(dummy_req(), source.attr.ino, 1);
    fs.forget(dummy_req(), target.attr.ino, 1);
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        assert!(!state.inodes.contains_key(&source.attr.ino));
        assert!(!state.inodes.contains_key(&target.attr.ino));
        assert_eq!(
            state.path_inodes.get(&VirtualPath::new("/sibling")),
            Some(&sibling.attr.ino)
        );
    }

    let replacement = block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("target"))).unwrap();
    assert_ne!(replacement.attr.ino, source.attr.ino);
    assert_ne!(replacement.attr.ino, target.attr.ino);

    fs.forget(dummy_req(), replacement.attr.ino, 1);
    fs.forget(dummy_req(), sibling.attr.ino, 1);
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
fn readdirplus_pins_returned_child_lookup_refs() {
    let dir = test_dir("readdirplus-pins-child");
    std::fs::create_dir(dir.join("listing")).unwrap();
    std::fs::write(dir.join("listing/child"), b"data").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let listing = lookup_root_inode(&fs, "listing");
    let fh = open_directory_handle(&fs, listing);
    let entries = block_on(fs.readdirplus(dummy_req(), listing, fh, 0, 4096)).unwrap();
    let child = entries
        .iter()
        .find(|entry| entry.name == b"child")
        .expect("child entry present")
        .ino;

    {
        let state = fs.state.read().expect("state rwlock poisoned");
        let record = state.inodes.get(&child).expect("child inode pinned");
        assert_eq!(record.lookup_refs, 1);
    }

    fs.forget(dummy_req(), child, 1);
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        assert!(!state.inodes.contains_key(&child));
        assert!(
            !state
                .path_inodes
                .contains_key(&VirtualPath::new("/listing/child"))
        );
    }

    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readdirplus_pins_only_returned_page_child_lookup_refs() {
    let dir = test_dir("readdirplus-page-pins-only");
    std::fs::create_dir(dir.join("listing")).unwrap();
    for name in ["alpha", "beta", "gamma"] {
        std::fs::write(dir.join("listing").join(name), b"data").unwrap();
    }
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let listing = lookup_root_inode(&fs, "listing");
    let fh = open_directory_handle(&fs, listing);
    let entries = block_on(fs.readdirplus(
        dummy_req(),
        listing,
        fh,
        0,
        (fractal_fuse::abi::fuse_direntplus_size(1) * 2
            + fractal_fuse::abi::fuse_direntplus_size("alpha".len())) as u32,
    ))
    .unwrap();
    let names = entries
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, vec![".", "..", "alpha"]);

    let alpha = entries
        .iter()
        .find(|entry| entry.name == b"alpha")
        .expect("alpha entry returned")
        .ino;
    {
        let state = fs.state.read().expect("state rwlock poisoned");
        assert_eq!(state.inodes.get(&alpha).unwrap().lookup_refs, 1);
        assert!(
            !state
                .path_inodes
                .contains_key(&VirtualPath::new("/listing/beta"))
        );
        assert!(
            !state
                .path_inodes
                .contains_key(&VirtualPath::new("/listing/gamma"))
        );
    }

    fs.forget(dummy_req(), alpha, 1);
    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readdirplus_honors_size_budget_and_continues_from_last_cookie() {
    let dir = test_dir("readdirplus-size-continuation");
    std::fs::create_dir(dir.join("listing")).unwrap();
    for name in ["alpha", "beta", "gamma"] {
        std::fs::write(dir.join("listing").join(name), b"data").unwrap();
    }
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let listing = lookup_root_inode(&fs, "listing");
    let fh = open_directory_handle(&fs, listing);
    let first = block_on(fs.readdirplus(
        dummy_req(),
        listing,
        fh,
        0,
        (fractal_fuse::abi::fuse_direntplus_size(1) * 2
            + fractal_fuse::abi::fuse_direntplus_size("alpha".len())) as u32,
    ))
    .unwrap();
    let first_names = first
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(first_names, vec![".", "..", "alpha"]);

    let second =
        block_on(fs.readdirplus(dummy_req(), listing, fh, first.last().unwrap().offset, 4096))
            .unwrap();
    let second_names = second
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(second_names, vec!["beta", "gamma"]);

    for entry in first.into_iter().chain(second) {
        if entry.name != b"." && entry.name != b".." {
            fs.forget(dummy_req(), entry.ino, 1);
        }
    }
    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn read_only_state_snapshots_can_run_concurrently() {
    let dir = test_dir("state-read-concurrent");
    std::fs::write(dir.join("file"), b"data").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let inode = lookup_root_inode(&fs, "file");
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();

    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..100 {
                    assert_eq!(fs.path_for_inode(inode).unwrap(), VirtualPath::new("/file"));
                    let (path, _file) = fs.file_handle_snapshot(inode, handle.fh).unwrap();
                    assert_eq!(path, VirtualPath::new("/file"));
                }
            });
        }
    });

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
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
