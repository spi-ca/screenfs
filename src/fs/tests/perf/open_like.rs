use super::super::*;
use super::helpers::labeled_latency_count;

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
    let dir_handle = block_on(fs.opendir(dummy_req(), dir_inode, libc::O_RDWR as u32)).unwrap();
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
fn perf_counters_skip_pre_open_stat_for_read_only_open_like_success() {
    let root = test_dir("perf-open-like-read-only-fast-path");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    std::fs::create_dir(source.join("dir")).unwrap();
    let fs = fs_for_external_mount(&source, &mount);

    let file_inode = lookup_root_inode(&fs, "file.txt");
    let dir_inode = lookup_root_inode(&fs, "dir");

    let before_open = fs.perf_snapshot().expect("perf counters enabled");
    let file_handle = block_on(fs.open(dummy_req(), file_inode, libc::O_RDONLY as u32)).unwrap();
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
        ),
        "read-only open should rely on opened-fd revalidation instead of a pre-open stat guard: {before_open:?}\n{after_open:?}"
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
        ),
        "read-only opendir should rely on opened-fd revalidation instead of a pre-open stat guard: {before_opendir:?}\n{after_opendir:?}"
    );

    let before_access = fs.perf_snapshot().expect("perf counters enabled");
    block_on(fs.access(dummy_req(), file_inode, libc::R_OK as u32)).unwrap();
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
        ),
        "read-only access should rely on opened-fd revalidation instead of a pre-open stat guard: {before_access:?}\n{after_access:?}"
    );

    block_on(fs.release(dummy_req(), file_inode, file_handle.fh, 0, 0, false, false)).unwrap();
    block_on(fs.releasedir(dummy_req(), dir_inode, dir_handle.fh, 0)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn perf_counters_keep_read_only_pre_open_guard_when_visibility_can_hide_targets() {
    let source = test_dir("perf-open-like-read-only-rule-sensitive");
    std::fs::write(source.join("file.txt"), "hello").unwrap();
    let fs = fs_for(&source, vec!["/hidden".to_string()], vec![]);
    let file_inode = lookup_root_inode(&fs, "file.txt");
    let before = fs.perf_snapshot().expect("perf counters enabled");
    let file_handle = block_on(fs.open(dummy_req(), file_inode, libc::O_RDONLY as u32)).unwrap();
    let after = fs.perf_snapshot().expect("perf counters enabled");

    assert_eq!(
        labeled_latency_count(&after.open_like_pre_open_guard, "open"),
        labeled_latency_count(&before.open_like_pre_open_guard, "open") + 1,
        "{before:?}\n{after:?}"
    );
    assert_eq!(
        labeled_latency_count(
            &after.stat_child_no_follow_context,
            "path_guard_or_metadata"
        ),
        labeled_latency_count(
            &before.stat_child_no_follow_context,
            "path_guard_or_metadata"
        ) + 1,
        "rule-sensitive read-only open must keep the pre-open stat guard so hidden ENOENT masking happens before host open failure: {before:?}\n{after:?}"
    );

    block_on(fs.release(dummy_req(), file_inode, file_handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(source).unwrap();
}
