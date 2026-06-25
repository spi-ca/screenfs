use super::super::*;

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
