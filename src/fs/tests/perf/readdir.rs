use super::super::*;

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
    for label in [
        "internal_hidden.path",
        "hidden.path",
        "visible.path",
        "visible.descendant",
    ] {
        assert!(
            snapshot.matcher_candidates_by_source.contains_key(label),
            "missing matcher_candidates_by_source.{label}: {snapshot:?}"
        );
        assert!(
            snapshot
                .matcher_candidate_order_by_source
                .contains_key(label),
            "missing matcher_candidate_order_by_source.{label}: {snapshot:?}"
        );
    }
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
    for label in [
        "name_child_path_materialization",
        "scan_visibility",
        "scan_fallback_attr",
        "returned_attr_hydration",
        "returned_policy_recheck",
        "returned_symlink_visibility",
    ] {
        assert!(
            snapshot.readdirplus_scan_splits.contains_key(label),
            "missing {label}: {snapshot:?}"
        );
    }
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
    assert!(
        snapshot
            .readdir_scan_splits
            .contains_key("name_child_path_materialization"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.readdir_scan_splits.contains_key("scan_visibility"),
        "{snapshot:?}"
    );
    assert!(
        snapshot
            .readdir_scan_splits
            .contains_key("scan_fallback_attr"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.readdir_attr_entries <= snapshot.readdirplus_attr_entries,
        "{snapshot:?}"
    );
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
fn perf_counters_record_matcher_sources_for_visibility_and_mutability() {
    let source = test_dir("perf-matcher-sources");
    std::fs::create_dir(source.join("carve")).unwrap();
    std::fs::write(source.join("carve/file.txt"), "hello").unwrap();
    let fs = fs_for_axes(
        &source,
        Some(crate::cli::VisibilityDefault::Hidden),
        vec!["./hidden/**".to_string()],
        vec!["./carve/file.txt".to_string()],
        Some(MutabilityDefault::Readonly),
        vec![],
        vec!["./carve/file.txt".to_string()],
    );

    let carve_inode = lookup_root_inode(&fs, "carve");
    let file_inode = lookup_child_inode(&fs, carve_inode, "file.txt");
    block_on(fs.access(dummy_req(), file_inode, libc::W_OK as u32)).unwrap();

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    for label in [
        "internal_hidden.path",
        "hidden.path",
        "visible.path",
        "visible.descendant",
        "readonly.path",
        "writable.path",
    ] {
        assert!(
            snapshot.matcher_candidates_by_source.contains_key(label),
            "missing matcher_candidates_by_source.{label}: {snapshot:?}"
        );
        assert!(
            snapshot
                .matcher_candidate_order_by_source
                .contains_key(label),
            "missing matcher_candidate_order_by_source.{label}: {snapshot:?}"
        );
    }

    let summary = fs.perf.summary();
    assert!(
        summary.contains("matcher_candidates_by_source.visible.path"),
        "{summary}"
    );
    assert!(
        summary.contains("matcher_candidate_order_by_source.writable.path"),
        "{summary}"
    );

    std::fs::remove_dir_all(source).unwrap();
}
#[test]
fn perf_counters_record_scan_visibility_batch_skips_trivial_policy_shape() {
    let root = test_dir("perf-scan-visibility-batch");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(source.join("listing")).unwrap();
    std::fs::write(source.join("listing/file.txt"), "hello").unwrap();
    let fs = fs_for_external_mount(&source, &mount);

    let listing = lookup_root_inode(&fs, "listing");
    let fh = open_directory_handle(&fs, listing);
    let before = fs.perf_snapshot().expect("perf counters enabled");

    let entries = block_on(fs.readdirplus(dummy_req(), listing, fh, 2, 4096)).unwrap();
    let after = fs.perf_snapshot().expect("perf counters enabled");
    let names = entries
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["file.txt"]);
    assert_eq!(
        after.policy_decisions.count - before.policy_decisions.count,
        0,
        "trivial visible policy should skip scan-time and returned-entry child policy decisions on the directory page path: {before:?}\n{after:?}"
    );
    assert!(
        after
            .readdirplus_scan_splits
            .contains_key("scan_visibility"),
        "{after:?}"
    );

    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();

    let fh = open_directory_handle(&fs, listing);
    let before = fs.perf_snapshot().expect("perf counters enabled");
    let entries = block_on(fs.readdir(dummy_req(), listing, fh, 2, 4096)).unwrap();
    let after = fs.perf_snapshot().expect("perf counters enabled");
    let names = entries
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(names, vec!["file.txt"]);
    assert_eq!(
        after.policy_decisions.count - before.policy_decisions.count,
        0,
        "trivial visible policy should skip readdir scan-time child policy decisions on the directory page path: {before:?}\n{after:?}"
    );
    assert!(
        after.readdir_scan_splits.contains_key("scan_visibility"),
        "{after:?}"
    );

    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();
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

    let second = block_on(fs.readdir(dummy_req(), listing, fh, alpha_cookie, 4096)).unwrap();
    let second_snapshot = fs.perf_snapshot().expect("perf counters enabled");
    let second_names = second
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(second_names, vec!["beta", "gamma"]);

    assert!(
        second_snapshot.readdir_attr_entries - first_snapshot.readdir_attr_entries <= 2,
        "{first_snapshot:?}\n{second_snapshot:?}"
    );
    std::fs::remove_dir_all(dir).unwrap();
}
