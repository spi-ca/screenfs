use super::*;
use std::ffi::OsStr;

#[test]
fn perf_counters_are_enabled_by_feature() {
    let source = test_dir("perf-enabled-by-feature");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    let fs = fs_for(&source, vec![], vec![]);

    let inode = lookup_root_inode(&fs, "file.txt");
    let _ = block_on(fs.getattr(dummy_req(), inode, None, 0)).unwrap();

    assert!(fs.perf_snapshot().is_some());
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_policy_state_open_and_readdirplus_attr_work() {
    let source = test_dir("perf-readdirplus");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    std::fs::create_dir(source.join("dir")).unwrap();
    std::os::unix::fs::symlink("file.txt", source.join("link.txt")).unwrap();
    let fs = fs_for(&source, vec!["*.tmp".to_string()], vec![]);

    let names = root_listing_names(&fs);
    assert!(names.iter().any(|name| name == "file.txt"));
    assert!(names.iter().any(|name| name == "dir"));
    assert!(names.iter().any(|name| name == "link.txt"));

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        snapshot.fuse_operations.contains_key("opendir"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.fuse_operations.contains_key("readdirplus"),
        "{snapshot:?}"
    );
    assert!(snapshot.policy_decisions.count > 0, "{snapshot:?}");
    assert!(snapshot.matcher_candidates > 0, "{snapshot:?}");
    assert!(
        snapshot
            .matcher_family_candidates
            .get("subtree")
            .copied()
            .unwrap_or(0)
            > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot
            .matcher_family_candidates
            .get("direct_child_glob")
            .copied()
            .unwrap_or(0)
            > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.matcher_candidate_order.contains_key("path"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.matcher_candidate_order_seen_slots > 0,
        "{snapshot:?}"
    );
    assert!(snapshot.state_read_wait.count > 0, "{snapshot:?}");
    assert!(snapshot.state_read_hold.count > 0, "{snapshot:?}");
    assert!(snapshot.state_write_wait.count > 0, "{snapshot:?}");
    assert!(snapshot.state_write_hold.count > 0, "{snapshot:?}");
    assert!(snapshot.open_confined.count > 0, "{snapshot:?}");
    assert!(
        snapshot.readdirplus_attr_generation.count > 0,
        "{snapshot:?}"
    );
    assert!(snapshot.readdirplus_attr_entries > 0, "{snapshot:?}");
    assert!(
        snapshot.readdirplus_symlink_visibility.count > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.readdirplus_candidate_selection.count > 0,
        "{snapshot:?}"
    );
    assert!(snapshot.readdirplus_page_commit.count > 0, "{snapshot:?}");

    let fh = open_directory_handle(&fs, FUSE_ROOT_ID);
    let _ = block_on(fs.readdir(dummy_req(), FUSE_ROOT_ID, fh, 0, 4096)).unwrap();
    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(snapshot.readdir_attr_generation.count > 0, "{snapshot:?}");
    assert!(snapshot.readdir_attr_entries > 0, "{snapshot:?}");
    assert!(
        snapshot.readdir_symlink_visibility.count > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.readdir_candidate_selection.count > 0,
        "{snapshot:?}"
    );
    assert!(snapshot.readdir_page_commit.count > 0, "{snapshot:?}");
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_split_resolved_virtual_path_sources() {
    let root = test_dir("perf-resolved-path-sources");
    let source = root.join("source");
    let source_alias = root.join("source-alias");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("file.txt"), "hello world").unwrap();
    std::os::unix::fs::symlink(&source, &source_alias).unwrap();
    let fs = fs_for_perf(&source_alias);

    assert_eq!(
        fs.resolved_virtual_path(&vpath("/file.txt"), true).unwrap(),
        vpath("/file.txt")
    );
    let file = fs
        .open_confined(&vpath("/file.txt"), libc::O_RDONLY, None)
        .unwrap();
    assert_eq!(
        fs.resolved_virtual_path_for_open_file(&file).unwrap(),
        vpath("/file.txt")
    );

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(snapshot.source_root_path.count > 0, "{snapshot:?}");
    assert!(snapshot.resolved_virtual_path.count > 0, "{snapshot:?}");
    assert!(
        snapshot
            .resolved_virtual_path_from_path_component_walk
            .count
            > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.resolved_virtual_path_from_path_canonicalize.count > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot
            .resolved_virtual_path_from_path_source_root_confinement
            .count
            > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot
            .resolved_virtual_path_from_path_virtual_conversion
            .count
            > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.resolved_virtual_path_from_path.count > 0,
        "{snapshot:?}"
    );
    assert!(
        snapshot.resolved_virtual_path_from_open_fd.count > 0,
        "{snapshot:?}"
    );
    assert_eq!(
        snapshot.resolved_virtual_path.count,
        snapshot.resolved_virtual_path_from_path.count
            + snapshot.resolved_virtual_path_from_open_fd.count,
        "{snapshot:?}"
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn perf_counters_resume_filter_skips_repeated_attr_work() {
    let dir = test_dir("perf-readdir-resume-filter");
    std::fs::create_dir(dir.join("listing")).unwrap();
    for name in ["alpha", "beta", "gamma"] {
        std::fs::write(dir.join("listing").join(name), b"data").unwrap();
    }
    let fs = fs_for_perf(&dir);

    let listing = lookup_root_inode(&fs, "listing");
    let fh = open_directory_handle(&fs, listing);
    let first = block_on(fs.readdir(
        dummy_req(),
        listing,
        fh,
        0,
        (fractal_fuse::abi::fuse_dirent_size(1)
            + fractal_fuse::abi::fuse_dirent_size(2)
            + fractal_fuse::abi::fuse_dirent_size("alpha".len())) as u32,
    ))
    .unwrap();
    let first_snapshot = fs.perf_snapshot().expect("perf counters enabled");
    let alpha_cookie = first.last().unwrap().offset;

    let _second = block_on(fs.readdir(dummy_req(), listing, fh, alpha_cookie, 4096)).unwrap();
    let second_snapshot = fs.perf_snapshot().expect("perf counters enabled");

    assert_eq!(
        second_snapshot.readdir_attr_entries - first_snapshot.readdir_attr_entries,
        2,
        "{first_snapshot:?}\n{second_snapshot:?}"
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn perf_counters_record_data_size_buckets() {
    let source = test_dir("perf-data-buckets");
    std::fs::write(source.join("file.txt"), "hello world").unwrap();
    let fs = fs_for_perf(&source);

    let inode = lookup_root_inode(&fs, "file.txt");
    let opened = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();
    let mut buf = [0_u8; 5];
    let read = block_on(fs.read(dummy_req(), inode, opened.fh, 0, &mut buf)).unwrap();
    assert_eq!(read, 5);
    let written = block_on(fs.write(dummy_req(), inode, opened.fh, 6, b"pi", 0, 0)).unwrap();
    assert_eq!(written, 2);

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        snapshot.read_size_buckets.contains_key("0_4k"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.write_size_buckets.contains_key("0_4k"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.fuse_operations.contains_key("read"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.fuse_operations.contains_key("write"),
        "{snapshot:?}"
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_invalidation_and_eviction_work() {
    let source = test_dir("perf-invalidation");
    std::fs::write(source.join("victim.txt"), "hello").unwrap();
    let fs = fs_for_perf(&source);

    let _victim = fs.inode_for_visible_path(vpath("/victim.txt"));
    block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("victim.txt"))).unwrap();

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(snapshot.invalidations > 0, "{snapshot:?}");
    assert!(snapshot.invalidated_entries > 0, "{snapshot:?}");
    assert!(snapshot.evicted_entries > 0, "{snapshot:?}");
    std::fs::remove_dir_all(source).unwrap();
}
