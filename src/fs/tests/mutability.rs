use super::*;

#[test]
fn writable_default_readonly_rules_are_scoped_to_matching_paths() {
    let dir = test_dir("writable-default-readonly-scoped");
    std::fs::create_dir(dir.join("locked")).unwrap();
    std::fs::create_dir_all(dir.join("free/nested")).unwrap();
    std::fs::write(dir.join("locked/file.txt"), b"locked").unwrap();
    std::fs::write(dir.join("free/file.txt"), b"free").unwrap();
    std::fs::write(dir.join("free/state.lock"), b"state").unwrap();
    std::fs::write(dir.join("free/nested/app.lock"), b"nested").unwrap();
    let fs = fs_for(
        &dir,
        Vec::new(),
        vec!["/locked".to_string(), "**/*.lock".to_string()],
    );

    let locked = fs
        .reply_entry_for_path(VirtualPath::new("/locked/file.txt"))
        .unwrap()
        .attr
        .ino;
    let free = fs
        .reply_entry_for_path(VirtualPath::new("/free/file.txt"))
        .unwrap()
        .attr
        .ino;
    let top_level_lock = fs
        .reply_entry_for_path(VirtualPath::new("/free/state.lock"))
        .unwrap()
        .attr
        .ino;
    let nested_lock = fs
        .reply_entry_for_path(VirtualPath::new("/free/nested/app.lock"))
        .unwrap()
        .attr
        .ino;

    assert_eq!(
        block_on(fs.open(dummy_req(), locked, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), top_level_lock, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), nested_lock, libc::W_OK as u32)).unwrap_err(),
        libc::EROFS
    );

    let free_handle =
        block_on(fs.open(dummy_req(), free, (libc::O_WRONLY | libc::O_APPEND) as u32)).unwrap();
    assert_eq!(
        block_on(fs.write(dummy_req(), free, free_handle.fh, 0, b"!", 0, 0)).unwrap(),
        1
    );
    block_on(fs.release(dummy_req(), free, free_handle.fh, 0, 0, false, false)).unwrap();

    let err = block_on(
        fs.create(
            dummy_req(),
            fs.reply_entry_for_path(VirtualPath::new("/locked"))
                .unwrap()
                .attr
                .ino,
            OsStr::new("new.txt"),
            0o644,
            libc::O_CREAT as u32,
        ),
    )
    .unwrap_err();
    assert_eq!(err, libc::EROFS);

    let err = block_on(
        fs.create(
            dummy_req(),
            fs.reply_entry_for_path(VirtualPath::new("/free"))
                .unwrap()
                .attr
                .ino,
            OsStr::new("new.lock"),
            0o644,
            libc::O_CREAT as u32,
        ),
    )
    .unwrap_err();
    assert_eq!(err, libc::EROFS);
    assert!(!dir.join("free/new.lock").exists());

    let created = block_on(
        fs.create(
            dummy_req(),
            fs.reply_entry_for_path(VirtualPath::new("/free"))
                .unwrap()
                .attr
                .ino,
            OsStr::new("new.txt"),
            0o644,
            libc::O_CREAT as u32,
        ),
    )
    .unwrap();
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
    assert!(dir.join("free/new.txt").exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn writable_default_symlink_returns_erofs_and_hidden_precedence_remains_enoent() {
    let dir = test_dir("writable-default-symlink");
    std::fs::write(dir.join("locked.lock"), b"locked").unwrap();
    std::fs::write(dir.join("hidden.lock"), b"hidden").unwrap();
    std::fs::write(dir.join("free.txt"), b"free").unwrap();
    std::os::unix::fs::symlink("locked.lock", dir.join("locked-link")).unwrap();
    std::os::unix::fs::symlink("hidden.lock", dir.join("hidden-link")).unwrap();
    let fs = fs_for(
        &dir,
        vec![
            "/hidden.lock".to_string(),
            "/hidden-link-create".to_string(),
        ],
        vec!["**/*.lock".to_string(), "/locked-link-create".to_string()],
    );

    let locked = fs
        .reply_entry_for_path(VirtualPath::new("/locked-link"))
        .unwrap()
        .attr
        .ino;
    let hidden = fs
        .state
        .lock()
        .expect("state mutex poisoned")
        .inode_for_path(VirtualPath::new("/hidden-link"));

    assert_eq!(
        block_on(fs.open(dummy_req(), locked, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), locked, libc::W_OK as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), hidden, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), hidden, libc::W_OK as u32)).unwrap_err(),
        ENOENT
    );

    assert_eq!(
        block_on(fs.symlink(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("locked-link-create"),
            OsStr::new("free.txt"),
        ))
        .unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.symlink(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("hidden-link-create"),
            OsStr::new("free.txt"),
        ))
        .unwrap_err(),
        ENOENT
    );

    block_on(fs.symlink(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("free-link-create"),
        OsStr::new("free.txt"),
    ))
    .unwrap();
    assert_eq!(
        std::fs::read_link(dir.join("free-link-create")).unwrap(),
        std::path::PathBuf::from("free.txt")
    );
    assert!(
        std::fs::symlink_metadata(dir.join("free-link-create"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_default_writable_match_non_match_and_hidden_precedence() {
    let dir = test_dir("allowwrite-match-hidden");
    std::fs::write(dir.join("allowed.txt"), b"ok").unwrap();
    std::fs::write(dir.join("blocked.bin"), b"no").unwrap();
    std::fs::write(dir.join("hidden.txt"), b"secret").unwrap();
    let fs = fs_for_policy(
        &dir,
        vec!["/hidden.txt".to_string()],
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["**/*.txt".to_string()],
    );

    let allowed = fs
        .reply_entry_for_path(VirtualPath::new("/allowed.txt"))
        .unwrap()
        .attr
        .ino;
    let blocked = fs
        .reply_entry_for_path(VirtualPath::new("/blocked.bin"))
        .unwrap()
        .attr
        .ino;
    let hidden = fs
        .state
        .lock()
        .expect("state mutex poisoned")
        .inode_for_path(VirtualPath::new("/hidden.txt"));

    let allowed_handle = block_on(fs.open(dummy_req(), allowed, libc::O_WRONLY as u32)).unwrap();
    block_on(fs.release(dummy_req(), allowed, allowed_handle.fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(fs.open(dummy_req(), blocked, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), hidden, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_default_requires_writable_parent_for_path_only_and_multi_path_mutation() {
    let dir = test_dir("allowwrite-parent-multi");
    std::fs::create_dir(dir.join("sandbox")).unwrap();
    std::fs::create_dir(dir.join("src")).unwrap();
    std::fs::create_dir(dir.join("dst")).unwrap();
    std::fs::write(dir.join("src/item.txt"), b"item").unwrap();

    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["**/*.txt".to_string()],
    );
    assert_eq!(
        block_on(
            fs.create(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/sandbox"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("new.txt"),
                0o644,
                libc::O_CREAT as u32,
            )
        )
        .unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(
            fs.rename(
                dummy_req(),
                fs.reply_entry_for_path(VirtualPath::new("/src"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("item.txt"),
                fs.reply_entry_for_path(VirtualPath::new("/dst"))
                    .unwrap()
                    .attr
                    .ino,
                OsStr::new("renamed.txt"),
                0,
            )
        )
        .unwrap_err(),
        libc::EROFS
    );
    assert!(dir.join("src/item.txt").exists());

    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["/src".to_string(), "/dst".to_string()],
    );
    block_on(
        fs.rename(
            dummy_req(),
            fs.reply_entry_for_path(VirtualPath::new("/src"))
                .unwrap()
                .attr
                .ino,
            OsStr::new("item.txt"),
            fs.reply_entry_for_path(VirtualPath::new("/dst"))
                .unwrap()
                .attr
                .ino,
            OsStr::new("renamed.txt"),
            0,
        ),
    )
    .unwrap();
    assert!(!dir.join("src/item.txt").exists());
    assert_eq!(std::fs::read(dir.join("dst/renamed.txt")).unwrap(), b"item");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_default_copy_file_range_requires_writable_destination_parent() {
    let dir = test_dir("allowwrite-copy-parent");
    std::fs::create_dir(dir.join("blocked")).unwrap();
    std::fs::write(dir.join("input.txt"), b"abcdef").unwrap();
    std::fs::write(dir.join("blocked/out.txt"), b"------").unwrap();

    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["**/*.txt".to_string()],
    );
    let input = fs
        .reply_entry_for_path(VirtualPath::new("/input.txt"))
        .unwrap()
        .attr
        .ino;
    let output = fs
        .reply_entry_for_path(VirtualPath::new("/blocked/out.txt"))
        .unwrap()
        .attr
        .ino;
    let input_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        input,
        VirtualPath::new("/input.txt"),
        OpenOptions::new()
            .read(true)
            .open(dir.join("input.txt"))
            .unwrap(),
    );
    let output_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        output,
        VirtualPath::new("/blocked/out.txt"),
        OpenOptions::new()
            .write(true)
            .open(dir.join("blocked/out.txt"))
            .unwrap(),
    );
    assert_eq!(
        block_on(fs.copy_file_range(dummy_req(), input, input_fh, 0, output, output_fh, 0, 3, 0,))
            .unwrap_err(),
        libc::EROFS
    );

    std::fs::write(dir.join("blocked/out.txt"), b"------").unwrap();
    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["/blocked".to_string()],
    );
    let input = fs
        .reply_entry_for_path(VirtualPath::new("/input.txt"))
        .unwrap()
        .attr
        .ino;
    let output = fs
        .reply_entry_for_path(VirtualPath::new("/blocked/out.txt"))
        .unwrap()
        .attr
        .ino;
    let input_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        input,
        VirtualPath::new("/input.txt"),
        OpenOptions::new()
            .read(true)
            .open(dir.join("input.txt"))
            .unwrap(),
    );
    let output_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        output,
        VirtualPath::new("/blocked/out.txt"),
        OpenOptions::new()
            .write(true)
            .open(dir.join("blocked/out.txt"))
            .unwrap(),
    );
    let copied =
        block_on(fs.copy_file_range(dummy_req(), input, input_fh, 1, output, output_fh, 2, 3, 0))
            .unwrap();
    assert_eq!(copied, 3);
    assert_eq!(
        std::fs::read(dir.join("blocked/out.txt")).unwrap(),
        b"--bcd-"
    );
    std::fs::remove_dir_all(dir).unwrap();
}
