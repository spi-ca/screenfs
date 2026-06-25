use super::super::*;

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
