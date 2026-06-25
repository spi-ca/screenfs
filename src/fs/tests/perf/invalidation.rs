use super::super::*;
use std::ffi::OsStr;

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
