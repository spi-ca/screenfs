//! Perf-counter feature tests for data-path and state/matcher attribution.

use super::*;
use fractal_fuse::abi::FOPEN_NOFLUSH;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::sync::Arc;

// Small accessors keep assertions focused on the perf split being tested.
fn file_sync_count(snapshot: &PerfSnapshot, label: &'static str) -> u64 {
    snapshot
        .file_sync
        .get(label)
        .map_or(0, |counter| counter.count)
}

fn labeled_latency_count(
    counters: &std::collections::BTreeMap<&'static str, crate::fs::perf::LatencySnapshot>,
    label: &'static str,
) -> u64 {
    counters.get(label).map_or(0, |counter| counter.count)
}

// Test cases are grouped by the behavior named in each function.
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
fn perf_counters_record_lookup_getattr_stat_reuse() {
    let source = test_dir("perf-lookup-getattr-stat-reuse");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    let fs = fs_for_perf(&source);

    let inode = lookup_root_inode(&fs, "file.txt");
    let after_lookup = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        after_lookup.stat_child_no_follow.count, 1,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_lookup.stat_child_no_follow_splits, "parent_open"),
        0,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_lookup.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        0,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_lookup.stat_child_no_follow_splits, "host_fstat"),
        1,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_lookup.stat_child_no_follow_splits, "host_fstatat"),
        0,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_lookup.stat_child_no_follow_splits, "attr_conversion"),
        1,
        "{after_lookup:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_lookup.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        1,
        "{after_lookup:?}"
    );

    let _ = block_on(fs.getattr(dummy_req(), inode, None, 0)).unwrap();
    let after_getattr = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        after_getattr.stat_child_no_follow.count, 2,
        "{after_getattr:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_getattr.stat_child_no_follow_splits, "parent_open"),
        0,
        "{after_getattr:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_getattr.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        0,
        "{after_getattr:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_getattr.stat_child_no_follow_splits, "host_fstat"),
        2,
        "{after_getattr:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_getattr.stat_child_no_follow_splits, "host_fstatat"),
        0,
        "{after_getattr:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_getattr.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        2,
        "{after_getattr:?}"
    );
    assert!(after_getattr.open_confined.count > 0, "{after_getattr:?}");

    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_missing_lookup_stat_failure_reuse() {
    let source = test_dir("perf-missing-lookup-stat-reuse");
    let fs = fs_for_perf(&source);

    assert_eq!(
        block_on(fs.lookup(dummy_req(), FUSE_ROOT_ID, OsStr::new("missing.txt"))).unwrap_err(),
        ENOENT
    );
    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(snapshot.stat_child_no_follow.count, 1, "{snapshot:?}");
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "parent_open"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &snapshot.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "host_fstat"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "host_fstatat"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "attr_conversion"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &snapshot.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        1,
        "{snapshot:?}"
    );

    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_root_stat_child_no_follow_split() {
    let source = test_dir("perf-root-stat-child-no-follow-split");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    let fs = fs_for_perf(&source);

    let root_attr = block_on(fs.getattr(dummy_req(), FUSE_ROOT_ID, None, 0)).unwrap();
    assert_eq!(root_attr.attr.ino, FUSE_ROOT_ID);

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(snapshot.stat_child_no_follow.count, 1, "{snapshot:?}");
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "host_fstat"),
        1,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "parent_open"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &snapshot.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "host_fstatat"),
        0,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(&snapshot.stat_child_no_follow_splits, "attr_conversion"),
        1,
        "{snapshot:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &snapshot.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        1,
        "{snapshot:?}"
    );

    let summary = fs.perf.summary();
    assert!(
        summary.contains("stat_child_no_follow.host_fstat"),
        "{summary}"
    );

    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_readlink_stat_reuse() {
    let source = test_dir("perf-readlink-stat-reuse");
    std::fs::write(source.join("target.txt"), "hello").unwrap();
    std::os::unix::fs::symlink("target.txt", source.join("link.txt")).unwrap();
    let fs = fs_for_perf(&source);

    let inode = lookup_root_inode(&fs, "link.txt");
    let after_lookup = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        after_lookup.stat_child_no_follow.count, 1,
        "{after_lookup:?}"
    );

    let reply = block_on(fs.readlink(dummy_req(), inode)).unwrap();
    assert_eq!(reply.data, OsStr::new("target.txt").as_bytes());
    let after_readlink = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        after_readlink.stat_child_no_follow.count, 2,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_readlink.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        1,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_readlink.stat_child_no_follow_context,
            "readlink_pre_open",
        ),
        1,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_readlink.stat_child_no_follow_splits, "parent_open"),
        1,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_readlink.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        1,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_readlink.stat_child_no_follow_splits, "host_fstat"),
        1,
        "{after_readlink:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_readlink.stat_child_no_follow_splits, "host_fstatat"),
        1,
        "{after_readlink:?}"
    );
    let summary = fs.perf.summary();
    assert!(
        summary.contains("stat_child_no_follow.parent_open"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.directory_revalidation"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.host_fstat"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.host_fstatat"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.attr_conversion"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow_context.readlink_pre_open"),
        "{summary}"
    );

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
        2,
        "trivial visible policy should skip scan-time child policy decisions but keep directory guard and returned-entry recheck: {before:?}\n{after:?}"
    );
    assert!(
        after
            .readdirplus_scan_splits
            .contains_key("scan_visibility"),
        "{after:?}"
    );

    block_on(fs.releasedir(dummy_req(), listing, fh, 0)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn perf_counters_record_open_like_guard_and_revalidation_splits_on_success() {
    let source = test_dir("perf-open-like-splits");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    std::fs::create_dir(source.join("dir")).unwrap();
    let fs = fs_for_perf(&source);

    let file_inode = lookup_root_inode(&fs, "file.txt");
    let dir_inode = lookup_root_inode(&fs, "dir");

    let before_open = fs.perf_snapshot().expect("perf counters enabled");
    let file_handle = block_on(fs.open(dummy_req(), file_inode, libc::O_RDWR as u32)).unwrap();
    let after_open = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        labeled_latency_count(&after_open.open_like_pre_open_guard, "open"),
        labeled_latency_count(&before_open.open_like_pre_open_guard, "open") + 1,
        "{before_open:?}\n{after_open:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_open.open_like_post_open_revalidation, "open"),
        labeled_latency_count(&before_open.open_like_post_open_revalidation, "open") + 1,
        "{before_open:?}\n{after_open:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_open.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        labeled_latency_count(
            &before_open.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ) + 1,
        "{before_open:?}\n{after_open:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_open.stat_child_no_follow_splits, "host_fstat"),
        labeled_latency_count(&before_open.stat_child_no_follow_splits, "host_fstat") + 1,
        "{before_open:?}\n{after_open:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_open.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        labeled_latency_count(
            &before_open.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        "{before_open:?}\n{after_open:?}"
    );
    assert!(after_open.open_confined.count > before_open.open_confined.count);
    assert!(
        after_open.resolved_virtual_path_from_open_fd.count
            > before_open.resolved_virtual_path_from_open_fd.count,
        "{before_open:?}\n{after_open:?}"
    );

    let before_opendir = fs.perf_snapshot().expect("perf counters enabled");
    let dir_handle = block_on(fs.opendir(dummy_req(), dir_inode, libc::O_RDONLY as u32)).unwrap();
    let after_opendir = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        labeled_latency_count(&after_opendir.open_like_pre_open_guard, "opendir"),
        labeled_latency_count(&before_opendir.open_like_pre_open_guard, "opendir") + 1,
        "{before_opendir:?}\n{after_opendir:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_opendir.open_like_post_open_revalidation, "opendir",),
        labeled_latency_count(&before_opendir.open_like_post_open_revalidation, "opendir",) + 1,
        "{before_opendir:?}\n{after_opendir:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_opendir.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        labeled_latency_count(
            &before_opendir.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ) + 1,
        "{before_opendir:?}\n{after_opendir:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_opendir.stat_child_no_follow_splits, "host_fstat"),
        labeled_latency_count(&before_opendir.stat_child_no_follow_splits, "host_fstat") + 1,
        "{before_opendir:?}\n{after_opendir:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_opendir.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        labeled_latency_count(
            &before_opendir.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        "{before_opendir:?}\n{after_opendir:?}"
    );

    let before_access = fs.perf_snapshot().expect("perf counters enabled");
    block_on(fs.access(dummy_req(), file_inode, libc::W_OK as u32)).unwrap();
    let after_access = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        labeled_latency_count(&after_access.open_like_pre_open_guard, "access"),
        labeled_latency_count(&before_access.open_like_pre_open_guard, "access") + 1,
        "{before_access:?}\n{after_access:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_access.open_like_post_open_revalidation, "access"),
        labeled_latency_count(&before_access.open_like_post_open_revalidation, "access") + 1,
        "{before_access:?}\n{after_access:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_access.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ),
        labeled_latency_count(
            &before_access.stat_child_no_follow_context,
            "path_guard_or_metadata",
        ) + 1,
        "{before_access:?}\n{after_access:?}"
    );
    assert_eq!(
        labeled_latency_count(&after_access.stat_child_no_follow_splits, "host_fstat"),
        labeled_latency_count(&before_access.stat_child_no_follow_splits, "host_fstat") + 1,
        "{before_access:?}\n{after_access:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after_access.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        labeled_latency_count(
            &before_access.stat_child_no_follow_splits,
            "directory_revalidation",
        ),
        "{before_access:?}\n{after_access:?}"
    );

    let summary = fs.perf.summary();
    assert!(summary.contains("open_confined_openat2"), "{summary}");
    assert!(
        summary.contains("open_like.pre_open_guard.open"),
        "{summary}"
    );
    assert!(
        summary.contains("open_like.post_open_revalidation.open"),
        "{summary}"
    );
    assert!(
        summary.contains("open_like.pre_open_guard.opendir"),
        "{summary}"
    );
    assert!(
        summary.contains("open_like.post_open_revalidation.opendir"),
        "{summary}"
    );
    assert!(
        summary.contains("open_like.pre_open_guard.access"),
        "{summary}"
    );
    assert!(
        summary.contains("open_like.post_open_revalidation.access"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.host_fstat"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow.attr_conversion"),
        "{summary}"
    );
    assert!(
        summary.contains("stat_child_no_follow_context.path_guard_or_metadata"),
        "{summary}"
    );
    assert!(
        after_access.fuse_operations.contains_key("open"),
        "{after_access:?}"
    );
    assert!(
        after_access.fuse_operations.contains_key("opendir"),
        "{after_access:?}"
    );
    assert!(
        after_access.fuse_operations.contains_key("access"),
        "{after_access:?}"
    );

    block_on(fs.release(dummy_req(), file_inode, file_handle.fh, 0, 0, false, false)).unwrap();
    block_on(fs.releasedir(dummy_req(), dir_inode, dir_handle.fh, 0)).unwrap();
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

#[test]
fn perf_counters_record_data_path_splits_on_success() {
    let root = test_dir("perf-data-buckets");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::create_dir(&mount).unwrap();
    std::fs::write(source.join("file.txt"), "hello world").unwrap();
    let _env = ProcessEnvGuard::new(&source, None);
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: Vec::new(),
            visibility_visible_rules: Vec::new(),
            mutability_readonly_rules: Vec::new(),
            mutability_writable_rules: Vec::new(),
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
    })
    .unwrap();
    let fs = ScreenFs::new(cfg);

    let inode = lookup_root_inode(&fs, "file.txt");
    let opened = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();
    let before_data_io = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        before_data_io.policy_decisions.count > 0,
        "{before_data_io:?}"
    );

    let mut buf = [0_u8; 5];
    let read = block_on(fs.read(dummy_req(), inode, opened.fh, 0, &mut buf)).unwrap();
    assert_eq!(read, 5);
    let written = block_on(fs.write(dummy_req(), inode, opened.fh, 6, b"pi", 0, 0)).unwrap();
    assert_eq!(written, 2);

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        snapshot.policy_decisions.count, before_data_io.policy_decisions.count,
        "cache-safe read/write should skip per-I/O policy rechecks: {before_data_io:?}\n{snapshot:?}"
    );
    assert_eq!(snapshot.read_handle_snapshot.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.read_guard_path.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.read_io.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_handle_snapshot.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_guard_mutation.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_io.count, 1, "{snapshot:?}");
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
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe() {
    let read_source = test_dir("perf-data-split-read-guard-fallback");
    std::fs::write(read_source.join("file.txt"), "hello world").unwrap();
    std::fs::write(read_source.join("hidden.txt"), "secret").unwrap();
    let read_fs = fs_for_axes(
        &read_source,
        None,
        vec!["/hidden.txt".to_string()],
        vec![],
        None,
        vec![],
        vec![],
    );

    let read_inode = lookup_root_inode(&read_fs, "file.txt");
    let read_opened =
        block_on(read_fs.open(dummy_req(), read_inode, libc::O_RDONLY as u32)).unwrap();
    let before_read = read_fs.perf_snapshot().expect("perf counters enabled");
    let mut buf = [0_u8; 5];
    let read =
        block_on(read_fs.read(dummy_req(), read_inode, read_opened.fh, 0, &mut buf)).unwrap();
    assert_eq!(read, 5);
    let after_read = read_fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_read.policy_decisions.count > before_read.policy_decisions.count,
        "read guard should still recheck policy when visibility cache is unsafe: {before_read:?}\n{after_read:?}"
    );
    assert_eq!(after_read.read_handle_snapshot.count, 1, "{after_read:?}");
    assert_eq!(after_read.read_guard_path.count, 1, "{after_read:?}");
    assert_eq!(after_read.read_io.count, 1, "{after_read:?}");
    assert_eq!(
        after_read.source_root_path.count - before_read.source_root_path.count,
        1,
        "unsafe read should reuse one request-local resolver across path and opened-fd guards: {before_read:?}\n{after_read:?}"
    );
    assert_eq!(
        after_read.resolved_virtual_path_from_path.count
            - before_read.resolved_virtual_path_from_path.count,
        1,
        "read should still re-resolve the requested path even without a parent-walk stat path: {before_read:?}\n{after_read:?}"
    );
    assert_eq!(
        after_read.resolved_virtual_path_from_open_fd.count
            - before_read.resolved_virtual_path_from_open_fd.count,
        1,
        "read should still revalidate the opened file fd even without parent-dir revalidation: {before_read:?}\n{after_read:?}"
    );

    let write_source = test_dir("perf-data-split-write-guard-fallback");
    std::fs::write(write_source.join("file.txt"), "hello world").unwrap();
    std::fs::write(write_source.join("hidden.txt"), "secret").unwrap();
    let write_fs = fs_for_axes(
        &write_source,
        None,
        vec!["/hidden.txt".to_string()],
        vec![],
        None,
        vec![],
        vec![],
    );
    let write_inode = lookup_root_inode(&write_fs, "file.txt");
    let write_opened =
        block_on(write_fs.open(dummy_req(), write_inode, libc::O_RDWR as u32)).unwrap();
    let before_write = write_fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        block_on(write_fs.write(dummy_req(), write_inode, write_opened.fh, 0, b"x", 0, 0)).unwrap(),
        1
    );
    let after_write = write_fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_write.policy_decisions.count > before_write.policy_decisions.count,
        "write guard should still recheck policy when cache is unsafe: {before_write:?}\n{after_write:?}"
    );
    assert_eq!(
        after_write.write_handle_snapshot.count, 1,
        "{after_write:?}"
    );
    assert_eq!(after_write.write_guard_mutation.count, 1, "{after_write:?}");
    assert_eq!(after_write.write_io.count, 1, "{after_write:?}");
    assert_eq!(
        after_write.source_root_path.count - before_write.source_root_path.count,
        1,
        "unsafe write should reuse one request-local resolver across path and opened-fd guards: {before_write:?}\n{after_write:?}"
    );
    assert_eq!(
        after_write.resolved_virtual_path_from_path.count
            - before_write.resolved_virtual_path_from_path.count,
        1,
        "write should still re-resolve the requested path even without a parent-walk stat path: {before_write:?}\n{after_write:?}"
    );
    assert_eq!(
        after_write.resolved_virtual_path_from_open_fd.count
            - before_write.resolved_virtual_path_from_open_fd.count,
        1,
        "write should still revalidate the opened file fd even without parent-dir revalidation: {before_write:?}\n{after_write:?}"
    );

    std::fs::remove_dir_all(read_source).unwrap();
    std::fs::remove_dir_all(write_source).unwrap();
}

#[test]
fn perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures() {
    let source = test_dir("perf-data-split-failures");
    std::fs::write(source.join("file.txt"), "hello world").unwrap();
    let fs = fs_for_perf(&source);

    let inode = lookup_root_inode(&fs, "file.txt");
    let mut buf = [0_u8; 5];
    assert_eq!(
        block_on(fs.read(dummy_req(), inode, 999_999, 0, &mut buf)).unwrap_err(),
        ENOENT
    );
    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(snapshot.read_handle_snapshot.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.read_guard_path.count, 0, "{snapshot:?}");
    assert_eq!(snapshot.read_io.count, 0, "{snapshot:?}");

    let opened = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, opened.fh, 0, b"x", 0, 0)).unwrap_err(),
        libc::EBADF
    );
    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(snapshot.write_handle_snapshot.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_guard_mutation.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_io.count, 1, "{snapshot:?}");
    assert!(
        !snapshot.write_size_buckets.contains_key("0_4k"),
        "failed writes do not update success-only size buckets: {snapshot:?}"
    );

    let readonly_source = test_dir("perf-data-split-guard-failure");
    std::fs::write(readonly_source.join("file.txt"), "hello world").unwrap();
    let readonly_fs = fs_for_root_readonly(&readonly_source, vec![]);
    let readonly_inode = lookup_root_inode(&readonly_fs, "file.txt");
    let readonly_opened =
        block_on(readonly_fs.open(dummy_req(), readonly_inode, libc::O_RDONLY as u32)).unwrap();
    assert_eq!(
        block_on(readonly_fs.write(
            dummy_req(),
            readonly_inode,
            readonly_opened.fh,
            0,
            b"x",
            0,
            0
        ))
        .unwrap_err(),
        libc::EROFS
    );
    let snapshot = readonly_fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(snapshot.write_handle_snapshot.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_guard_mutation.count, 1, "{snapshot:?}");
    assert_eq!(snapshot.write_io.count, 0, "{snapshot:?}");

    std::fs::remove_dir_all(source).unwrap();
    std::fs::remove_dir_all(readonly_source).unwrap();
}

#[test]
fn perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path() {
    let source = test_dir("perf-data-path-noncached-mutators");
    std::fs::write(source.join("input.txt"), b"abcdef").unwrap();
    std::fs::write(source.join("output.txt"), b"------").unwrap();
    let fs = fs_for_perf(&source);

    let input = lookup_root_inode(&fs, "input.txt");
    let output = lookup_root_inode(&fs, "output.txt");
    let input_handle = block_on(fs.open(dummy_req(), input, libc::O_RDONLY as u32)).unwrap();
    let output_handle = block_on(fs.open(dummy_req(), output, libc::O_RDWR as u32)).unwrap();

    let before = fs.perf_snapshot().expect("perf counters enabled");
    block_on(fs.fallocate(dummy_req(), output, output_handle.fh, 0, 8, 0)).unwrap();
    let after_first_fallocate = fs.perf_snapshot().expect("perf counters enabled");
    block_on(fs.fallocate(dummy_req(), output, output_handle.fh, 0, 12, 0)).unwrap();
    let after_second_fallocate = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_first_fallocate.policy_decisions.count > before.policy_decisions.count,
        "fallocate should still re-run policy guards: {before:?}\n{after_first_fallocate:?}"
    );
    assert!(
        after_second_fallocate.policy_decisions.count
            > after_first_fallocate.policy_decisions.count,
        "repeated fallocate should stay on the per-call policy path: {after_first_fallocate:?}\n{after_second_fallocate:?}"
    );

    assert_eq!(
        block_on(fs.copy_file_range(
            dummy_req(),
            input,
            input_handle.fh,
            0,
            output,
            output_handle.fh,
            0,
            2,
            0,
        ))
        .unwrap(),
        2
    );
    let after_first_copy = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        block_on(fs.copy_file_range(
            dummy_req(),
            input,
            input_handle.fh,
            2,
            output,
            output_handle.fh,
            2,
            2,
            0,
        ))
        .unwrap(),
        2
    );
    let after_second_copy = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_first_copy.policy_decisions.count > after_second_fallocate.policy_decisions.count,
        "copy_file_range should still re-run policy guards: {after_second_fallocate:?}\n{after_first_copy:?}"
    );
    assert!(
        after_second_copy.policy_decisions.count > after_first_copy.policy_decisions.count,
        "repeated copy_file_range should stay on the per-call policy path: {after_first_copy:?}\n{after_second_copy:?}"
    );
    assert_eq!(
        after_second_copy.read_handle_snapshot.count, 0,
        "{after_second_copy:?}"
    );
    assert_eq!(
        after_second_copy.write_handle_snapshot.count, 0,
        "{after_second_copy:?}"
    );
    assert!(
        after_second_copy.fuse_operations.contains_key("fallocate"),
        "{after_second_copy:?}"
    );
    assert!(
        after_second_copy
            .fuse_operations
            .contains_key("copy_file_range"),
        "{after_second_copy:?}"
    );

    block_on(fs.release(dummy_req(), input, input_handle.fh, 0, 0, false, false)).unwrap();
    block_on(fs.release(dummy_req(), output, output_handle.fh, 0, 0, false, false)).unwrap();
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
    assert!(snapshot.invalidation_scanned_entries > 0, "{snapshot:?}");
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn perf_counters_record_file_sync_splits() {
    let source = test_dir("perf-file-sync-splits");
    std::fs::write(source.join("file.txt"), b"abcdef").unwrap();
    let fs = fs_for_perf(&source);
    let inode = lookup_root_inode(&fs, "file.txt");
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();
    let (flush_file, flush_needs_sync) = fs.file_flush_sync_snapshot(inode, handle.fh).unwrap();
    assert!(flush_needs_sync);
    let fsync_file = fs.file_sync_snapshot(inode, handle.fh).unwrap();
    assert!(Arc::ptr_eq(&flush_file, &fsync_file));

    let before = fs.perf_snapshot().expect("perf counters enabled");

    compio_block_on(fs.flush(dummy_req(), inode, handle.fh, 0)).unwrap();
    let after_flush = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        file_sync_count(&after_flush, "flush"),
        file_sync_count(&before, "flush") + 1,
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        file_sync_count(&after_flush, "fsync"),
        file_sync_count(&before, "fsync"),
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        file_sync_count(&after_flush, "release_flush"),
        file_sync_count(&before, "release_flush"),
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        after_flush.state_read_wait.count,
        before.state_read_wait.count + 1,
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        after_flush.state_read_hold.count,
        before.state_read_hold.count + 1,
        "{before:?}\n{after_flush:?}"
    );

    compio_block_on(fs.fsync(dummy_req(), inode, handle.fh, false)).unwrap();
    let after_fsync = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        file_sync_count(&after_fsync, "flush"),
        file_sync_count(&after_flush, "flush"),
        "{after_flush:?}\n{after_fsync:?}"
    );
    assert_eq!(
        file_sync_count(&after_fsync, "fsync"),
        file_sync_count(&after_flush, "fsync") + 1,
        "{after_flush:?}\n{after_fsync:?}"
    );
    assert_eq!(
        file_sync_count(&after_fsync, "release_flush"),
        file_sync_count(&after_flush, "release_flush"),
        "{after_flush:?}\n{after_fsync:?}"
    );
    assert_eq!(
        after_fsync.state_read_wait.count,
        after_flush.state_read_wait.count + 1,
        "{after_flush:?}\n{after_fsync:?}"
    );
    assert_eq!(
        after_fsync.state_read_hold.count,
        after_flush.state_read_hold.count + 1,
        "{after_flush:?}\n{after_fsync:?}"
    );

    compio_block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    let after_release_without_flush = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        file_sync_count(&after_release_without_flush, "release_flush"),
        file_sync_count(&after_fsync, "release_flush"),
        "{after_fsync:?}\n{after_release_without_flush:?}"
    );

    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();
    compio_block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, true, false)).unwrap();

    let snapshot = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        snapshot.fuse_operations.contains_key("flush"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.fuse_operations.contains_key("fsync"),
        "{snapshot:?}"
    );
    assert!(
        snapshot.fuse_operations.contains_key("release"),
        "{snapshot:?}"
    );
    assert_eq!(
        file_sync_count(&snapshot, "flush"),
        file_sync_count(&after_release_without_flush, "flush"),
        "{after_release_without_flush:?}\n{snapshot:?}"
    );
    assert_eq!(
        file_sync_count(&snapshot, "fsync"),
        file_sync_count(&after_release_without_flush, "fsync"),
        "{after_release_without_flush:?}\n{snapshot:?}"
    );
    assert_eq!(
        file_sync_count(&snapshot, "release_flush"),
        file_sync_count(&after_release_without_flush, "release_flush") + 1,
        "{after_release_without_flush:?}\n{snapshot:?}"
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn sync_snapshots_reject_missing_or_mismatched_handles() {
    let source = test_dir("perf-sync-snapshot-enoent");
    std::fs::write(source.join("file.txt"), b"abcdef").unwrap();
    let fs = fs_for_perf(&source);
    let inode = lookup_root_inode(&fs, "file.txt");
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    assert_eq!(
        compio_block_on(fs.flush(dummy_req(), inode, 999_999, 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        compio_block_on(fs.fsync(dummy_req(), inode, 999_999, false)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        compio_block_on(fs.flush(dummy_req(), FUSE_ROOT_ID, handle.fh, 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        compio_block_on(fs.fsync(dummy_req(), FUSE_ROOT_ID, handle.fh, false)).unwrap_err(),
        ENOENT
    );

    compio_block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn readonly_open_uses_noflush_and_flush_skips_sync() {
    let source = test_dir("perf-readonly-open-noflush");
    std::fs::write(source.join("file.txt"), b"abcdef").unwrap();
    let fs = fs_for_perf(&source);
    let inode = lookup_root_inode(&fs, "file.txt");
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();
    assert_ne!(handle.flags & FOPEN_NOFLUSH, 0);
    let (flush_file, flush_needs_sync) = fs.file_flush_sync_snapshot(inode, handle.fh).unwrap();
    assert!(!flush_needs_sync);
    let fsync_file = fs.file_sync_snapshot(inode, handle.fh).unwrap();
    assert!(Arc::ptr_eq(&flush_file, &fsync_file));

    let before = fs.perf_snapshot().expect("perf counters enabled");
    compio_block_on(fs.flush(dummy_req(), inode, handle.fh, 0)).unwrap();
    let after_flush = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_flush.fuse_operations.contains_key("flush"),
        "{after_flush:?}"
    );
    assert_eq!(
        file_sync_count(&after_flush, "flush"),
        file_sync_count(&before, "flush"),
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        after_flush.state_read_wait.count,
        before.state_read_wait.count + 1,
        "{before:?}\n{after_flush:?}"
    );
    assert_eq!(
        after_flush.state_read_hold.count,
        before.state_read_hold.count + 1,
        "{before:?}\n{after_flush:?}"
    );

    compio_block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, true, false)).unwrap();
    let after_release = fs.perf_snapshot().expect("perf counters enabled");
    assert!(
        after_release.fuse_operations.contains_key("release"),
        "{after_release:?}"
    );
    assert_eq!(
        file_sync_count(&after_release, "release_flush"),
        file_sync_count(&after_flush, "release_flush"),
        "{after_flush:?}\n{after_release:?}"
    );

    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn write_capable_open_flushes_and_does_not_use_noflush() {
    let source = test_dir("perf-write-open-flushes");
    std::fs::write(source.join("file.txt"), b"abcdef").unwrap();
    let fs = fs_for_perf(&source);
    let inode = lookup_root_inode(&fs, "file.txt");
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_WRONLY as u32)).unwrap();
    assert_eq!(handle.flags & FOPEN_NOFLUSH, 0);

    let before = fs.perf_snapshot().expect("perf counters enabled");
    compio_block_on(fs.flush(dummy_req(), inode, handle.fh, 0)).unwrap();
    let after_flush = fs.perf_snapshot().expect("perf counters enabled");
    assert_eq!(
        file_sync_count(&after_flush, "flush"),
        file_sync_count(&before, "flush") + 1,
        "{before:?}\n{after_flush:?}"
    );

    std::fs::remove_dir_all(source).unwrap();
}
