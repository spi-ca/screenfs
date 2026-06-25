use super::super::*;
use super::helpers::file_sync_count;
use fractal_fuse::abi::FOPEN_NOFLUSH;
use std::sync::Arc;

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
