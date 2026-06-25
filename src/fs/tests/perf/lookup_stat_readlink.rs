use super::super::*;
use super::helpers::labeled_latency_count;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

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
