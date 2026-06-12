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
fn writable_default_direct_child_globs_lock_only_immediate_children() {
    let dir = test_dir("writable-default-direct-child-globs");
    std::fs::create_dir_all(dir.join("free/nested")).unwrap();
    std::fs::write(dir.join("free/state.lock"), b"locked").unwrap();
    std::fs::write(dir.join("free/id_ed25519"), b"locked").unwrap();
    std::fs::write(dir.join("free/nested/state.lock"), b"nested").unwrap();
    std::fs::write(dir.join("free/nested/id_ed25519"), b"nested").unwrap();
    let fs = fs_for(
        &dir,
        Vec::new(),
        vec!["/free/*.lock".to_string(), "/free/id_*".to_string()],
    );

    for path in ["/free/state.lock", "/free/id_ed25519"] {
        let ino = fs
            .reply_entry_for_path(VirtualPath::new(path))
            .unwrap()
            .attr
            .ino;
        assert_eq!(
            block_on(fs.open(dummy_req(), ino, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS
        );
    }

    for path in ["/free/nested/state.lock", "/free/nested/id_ed25519"] {
        let ino = fs
            .reply_entry_for_path(VirtualPath::new(path))
            .unwrap()
            .attr
            .ino;
        let handle = block_on(fs.open(dummy_req(), ino, libc::O_WRONLY as u32)).unwrap();
        block_on(fs.release(dummy_req(), ino, handle.fh, 0, 0, false, false)).unwrap();
    }

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn writable_default_direct_child_wildcard_all_locks_child_subtrees() {
    let dir = test_dir("writable-default-direct-child-wildcard-all");
    std::fs::create_dir_all(dir.join("free/nested")).unwrap();
    std::fs::write(dir.join("free/plain.txt"), b"plain").unwrap();
    std::fs::write(dir.join("free/nested/deep.txt"), b"deep").unwrap();
    std::fs::write(dir.join("other.txt"), b"other").unwrap();
    let fs = fs_for(&dir, Vec::new(), vec!["/free/*".to_string()]);

    for path in ["/free/plain.txt", "/free/nested/deep.txt"] {
        let ino = fs
            .reply_entry_for_path(VirtualPath::new(path))
            .unwrap()
            .attr
            .ino;
        assert_eq!(
            block_on(fs.open(dummy_req(), ino, libc::O_WRONLY as u32)).unwrap_err(),
            libc::EROFS,
            "{path}"
        );
    }

    let other = fs
        .reply_entry_for_path(VirtualPath::new("/other.txt"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), other, libc::O_WRONLY as u32)).unwrap();
    block_on(fs.release(dummy_req(), other, handle.fh, 0, 0, false, false)).unwrap();

    let free = fs
        .reply_entry_for_path(VirtualPath::new("/free"))
        .unwrap()
        .attr
        .ino;
    assert_eq!(
        block_on(fs.create(
            dummy_req(),
            free,
            OsStr::new("new.txt"),
            0o644,
            libc::O_WRONLY as u32 | libc::O_CREAT as u32,
        ))
        .unwrap_err(),
        libc::EROFS
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_default_canonical_absolute_txt_globs_preserve_direct_child_over_recursive_specificity()
{
    let dir = test_dir("readonly-default-absolute-txt-globs");
    std::fs::create_dir_all(dir.join("a/nested")).unwrap();
    std::fs::write(dir.join("a/top.txt"), b"top").unwrap();
    std::fs::write(dir.join("a/nested/deep.txt"), b"deep").unwrap();
    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        vec!["/a/*.txt".to_string()],
        Some(MutabilityDefault::Readonly),
        vec!["/a/**/*.txt".to_string()],
    );

    let top = fs
        .reply_entry_for_path(VirtualPath::new("/a/top.txt"))
        .unwrap()
        .attr
        .ino;
    let deep = fs
        .reply_entry_for_path(VirtualPath::new("/a/nested/deep.txt"))
        .unwrap()
        .attr
        .ino;

    assert_eq!(
        block_on(fs.open(dummy_req(), top, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    let handle = block_on(fs.open(dummy_req(), deep, libc::O_WRONLY as u32)).unwrap();
    block_on(fs.release(dummy_req(), deep, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn writable_default_recursive_literal_directory_shorthand_locks_git_hooks_subtree() {
    let dir = test_dir("writable-default-git-hooks-shorthand");
    std::fs::create_dir_all(dir.join("project/repo/.git/hooks")).unwrap();
    std::fs::write(dir.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
    std::fs::write(dir.join("project/repo/.git/config"), b"config").unwrap();
    let fs = fs_for(&dir, Vec::new(), vec!["/project/**/.git/hooks".to_string()]);

    let hook = fs
        .reply_entry_for_path(VirtualPath::new("/project/repo/.git/hooks/pre-commit"))
        .unwrap()
        .attr
        .ino;
    let hooks_dir = fs
        .reply_entry_for_path(VirtualPath::new("/project/repo/.git/hooks"))
        .unwrap()
        .attr
        .ino;
    let config = fs
        .reply_entry_for_path(VirtualPath::new("/project/repo/.git/config"))
        .unwrap()
        .attr
        .ino;

    assert_eq!(
        block_on(fs.open(dummy_req(), hook, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.opendir(dummy_req(), hooks_dir, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );

    let config_handle = block_on(fs.open(dummy_req(), config, libc::O_WRONLY as u32)).unwrap();
    block_on(fs.release(dummy_req(), config, config_handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_default_recursive_literal_directory_writable_shorthand_overrides_default() {
    let dir = test_dir("readonly-default-git-hooks-shorthand-override");
    std::fs::create_dir_all(dir.join("workspace/repo/.git/hooks")).unwrap();
    std::fs::create_dir_all(dir.join("other/repo/.git/hooks")).unwrap();
    std::fs::write(dir.join("workspace/repo/.git/hooks/pre-commit"), b"hook").unwrap();
    std::fs::write(dir.join("workspace/repo/.git/config"), b"config").unwrap();
    std::fs::write(dir.join("other/repo/.git/hooks/pre-commit"), b"hook").unwrap();
    let fs = fs_for_policy(
        &dir,
        Vec::new(),
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        vec!["/workspace/**/.git/hooks".to_string()],
    );

    let workspace_hook = fs
        .reply_entry_for_path(VirtualPath::new("/workspace/repo/.git/hooks/pre-commit"))
        .unwrap()
        .attr
        .ino;
    let workspace_hooks_dir = fs
        .reply_entry_for_path(VirtualPath::new("/workspace/repo/.git/hooks"))
        .unwrap()
        .attr
        .ino;
    let workspace_config = fs
        .reply_entry_for_path(VirtualPath::new("/workspace/repo/.git/config"))
        .unwrap()
        .attr
        .ino;
    let other_hook = fs
        .reply_entry_for_path(VirtualPath::new("/other/repo/.git/hooks/pre-commit"))
        .unwrap()
        .attr
        .ino;

    let hook_handle =
        block_on(fs.open(dummy_req(), workspace_hook, libc::O_WRONLY as u32)).unwrap();
    block_on(fs.release(
        dummy_req(),
        workspace_hook,
        hook_handle.fh,
        0,
        0,
        false,
        false,
    ))
    .unwrap();

    let created = block_on(fs.create(
        dummy_req(),
        workspace_hooks_dir,
        OsStr::new("post-commit"),
        0o755,
        libc::O_WRONLY as u32 | libc::O_CREAT as u32,
    ))
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
    assert!(dir.join("workspace/repo/.git/hooks/post-commit").exists());

    assert_eq!(
        block_on(fs.open(dummy_req(), workspace_config, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), other_hook, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
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
        vec!["*.lock".to_string(), "/locked-link-create".to_string()],
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
        vec!["*.txt".to_string()],
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
        vec!["*.txt".to_string()],
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
