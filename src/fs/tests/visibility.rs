use super::*;

#[test]
fn hidden_read_and_list_operations_return_enoent_and_filter_entries() {
    let dir = test_dir("hidden-read-list");
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    std::fs::write(dir.join("hidden.pem"), b"secret").unwrap();
    std::fs::create_dir(dir.join("private")).unwrap();
    std::fs::write(dir.join("private/note.txt"), b"nope").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["*.pem".to_string(), "/private".to_string()]);

    let hidden_inode = tracked_inode(&fs, "/hidden.pem");
    let fh = insert_tracked_file_handle(
        &fs,
        hidden_inode,
        "/hidden.pem",
        File::open(dir.join("hidden.pem")).unwrap(),
    );
    let mut buf = [0_u8; 16];
    let err = block_on(fs.read(dummy_req(), hidden_inode, fh, 0, &mut buf)).unwrap_err();
    assert_eq!(err, ENOENT);
    block_on(fs.release(dummy_req(), hidden_inode, fh, 0, 0, false, false)).unwrap();

    let hidden_dir_inode = tracked_inode(&fs, "/private");
    let err =
        block_on(fs.opendir(dummy_req(), hidden_dir_inode, libc::O_RDONLY as u32)).unwrap_err();
    assert_eq!(err, ENOENT);

    let names = root_listing_names(&fs);
    assert!(names.contains(&"visible".to_string()));
    assert!(!names.contains(&"hidden.pem".to_string()));
    assert!(!names.contains(&"private".to_string()));
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn visible_carve_out_exposes_only_bridge_ancestors_and_target_subtree() {
    let dir = test_dir("visible-bridge-carve-out");
    std::fs::create_dir_all(dir.join("home/me/project")).unwrap();
    std::fs::create_dir_all(dir.join("home/other")).unwrap();
    std::fs::write(dir.join("home/me/project/file.txt"), b"ok").unwrap();
    std::fs::write(dir.join("home/me/secret.txt"), b"nope").unwrap();
    let fs = fs_for_axes(
        &dir,
        None,
        vec!["/home".to_string()],
        vec!["/home/me/project".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"home".to_string()));

    let home_inode = lookup_root_inode(&fs, "home");
    let home_fh = open_directory_handle(&fs, home_inode);
    let home_names: Vec<String> =
        block_on(fs.readdirplus(dummy_req(), home_inode, home_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(home_names.contains(&"me".to_string()));
    assert!(!home_names.contains(&"other".to_string()));
    block_on(fs.releasedir(dummy_req(), home_inode, home_fh, 0)).unwrap();

    let me_inode = lookup_child_inode(&fs, home_inode, "me");
    let me_fh = open_directory_handle(&fs, me_inode);
    let me_names: Vec<String> = block_on(fs.readdirplus(dummy_req(), me_inode, me_fh, 0, 4096))
        .unwrap()
        .into_iter()
        .map(|entry| String::from_utf8(entry.name).unwrap())
        .collect();
    assert!(me_names.contains(&"project".to_string()));
    assert!(!me_names.contains(&"secret.txt".to_string()));
    block_on(fs.releasedir(dummy_req(), me_inode, me_fh, 0)).unwrap();

    let project_inode = lookup_child_inode(&fs, me_inode, "project");
    let file_inode = lookup_child_inode(&fs, project_inode, "file.txt");
    let fh = block_on(fs.open(dummy_req(), file_inode, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), file_inode, fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(fs.lookup(dummy_req(), me_inode, OsStr::new("secret.txt"))).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.setattr(
            dummy_req(),
            me_inode,
            None,
            SetAttr {
                mode: Some(0o755),
                ..Default::default()
            },
        ))
        .unwrap_err(),
        libc::EROFS
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn exact_visible_absent_target_still_exposes_existing_bridge_ancestors() {
    let dir = test_dir("exact-visible-absent-target-bridge");
    std::fs::create_dir_all(dir.join("a/b")).unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/a/b/c".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"a".to_string()));
    let a_inode = lookup_root_inode(&fs, "a");
    let a_fh = open_directory_handle(&fs, a_inode);
    let a_names: Vec<String> = block_on(fs.readdirplus(dummy_req(), a_inode, a_fh, 0, 4096))
        .unwrap()
        .into_iter()
        .map(|entry| String::from_utf8(entry.name).unwrap())
        .collect();
    assert!(a_names.contains(&"b".to_string()));
    block_on(fs.releasedir(dummy_req(), a_inode, a_fh, 0)).unwrap();

    let b_inode = lookup_child_inode(&fs, a_inode, "b");
    let b_fh = block_on(fs.opendir(dummy_req(), b_inode, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.releasedir(dummy_req(), b_inode, b_fh, 0)).unwrap();
    assert_eq!(
        block_on(fs.lookup(dummy_req(), b_inode, OsStr::new("c"))).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readdirplus_discovered_visible_file_survives_open_release_for_cached_path_reuse() {
    let dir = test_dir("readdirplus-cached-visible-file");
    std::fs::create_dir_all(dir.join("tmp")).unwrap();
    std::fs::write(dir.join("tmp/existing"), b"old").unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/tmp".to_string()],
        Some(crate::cli::MutabilityDefault::Readonly),
        Vec::new(),
        vec!["/tmp".to_string()],
    );

    let root_fh = open_directory_handle(&fs, FUSE_ROOT_ID);
    let root_entries =
        block_on(fs.readdirplus(dummy_req(), FUSE_ROOT_ID, root_fh, 0, 4096)).unwrap();
    let tmp_inode = root_entries
        .iter()
        .find(|entry| entry.name == b"tmp")
        .unwrap()
        .ino;
    block_on(fs.releasedir(dummy_req(), FUSE_ROOT_ID, root_fh, 0)).unwrap();

    let tmp_fh = open_directory_handle(&fs, tmp_inode);
    let tmp_entries = block_on(fs.readdirplus(dummy_req(), tmp_inode, tmp_fh, 0, 4096)).unwrap();
    let existing_inode = tmp_entries
        .iter()
        .find(|entry| entry.name == b"existing")
        .unwrap()
        .ino;
    block_on(fs.releasedir(dummy_req(), tmp_inode, tmp_fh, 0)).unwrap();

    let write_fh = block_on(fs.open(dummy_req(), existing_inode, libc::O_WRONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), existing_inode, write_fh, 0, 0, false, false)).unwrap();
    let read_fh = block_on(fs.open(dummy_req(), existing_inode, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), existing_inode, read_fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn default_hidden_visible_writable_subtree_allows_new_child_create() {
    let dir = test_dir("default-hidden-visible-create");
    std::fs::create_dir_all(dir.join("tmp")).unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/tmp".to_string()],
        Some(crate::cli::MutabilityDefault::Readonly),
        Vec::new(),
        vec!["/tmp".to_string()],
    );
    let tmp_inode = lookup_root_inode(&fs, "tmp");
    let created = block_on(fs.create(
        dummy_req(),
        tmp_inode,
        OsStr::new("write-ok"),
        0o644,
        libc::O_WRONLY as u32,
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
    assert!(dir.join("tmp/write-ok").exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn descendant_subtree_visible_rule_exposes_bridge_ancestors_to_target() {
    let dir = test_dir("visible-descendant-subtree-bridge");
    std::fs::create_dir_all(dir.join("workspace/repo/.git/hooks")).unwrap();
    std::fs::create_dir_all(dir.join("workspace/other")).unwrap();
    std::fs::write(dir.join("workspace/repo/.git/hooks/pre-commit"), b"ok").unwrap();
    std::fs::write(dir.join("workspace/repo/.git/config"), b"hidden").unwrap();
    std::fs::write(dir.join("workspace/other/readme.txt"), b"hidden").unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["**/.git/hooks/**".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"workspace".to_string()));
    assert!(!root_names.contains(&"other".to_string()));

    let workspace_inode = lookup_root_inode(&fs, "workspace");
    let workspace_fh = open_directory_handle(&fs, workspace_inode);
    let workspace_names: Vec<String> =
        block_on(fs.readdirplus(dummy_req(), workspace_inode, workspace_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(workspace_names.contains(&"repo".to_string()));
    assert!(!workspace_names.contains(&"other".to_string()));
    block_on(fs.releasedir(dummy_req(), workspace_inode, workspace_fh, 0)).unwrap();

    let repo_inode = lookup_child_inode(&fs, workspace_inode, "repo");
    let repo_fh = open_directory_handle(&fs, repo_inode);
    let repo_names: Vec<String> =
        block_on(fs.readdirplus(dummy_req(), repo_inode, repo_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(repo_names.contains(&".git".to_string()));
    block_on(fs.releasedir(dummy_req(), repo_inode, repo_fh, 0)).unwrap();

    let git_inode = lookup_child_inode(&fs, repo_inode, ".git");
    let git_fh = open_directory_handle(&fs, git_inode);
    let git_names: Vec<String> = block_on(fs.readdirplus(dummy_req(), git_inode, git_fh, 0, 4096))
        .unwrap()
        .into_iter()
        .map(|entry| String::from_utf8(entry.name).unwrap())
        .collect();
    assert!(git_names.contains(&"hooks".to_string()));
    assert!(!git_names.contains(&"config".to_string()));
    block_on(fs.releasedir(dummy_req(), git_inode, git_fh, 0)).unwrap();

    let hooks_inode = lookup_child_inode(&fs, git_inode, "hooks");
    let hook_inode = lookup_child_inode(&fs, hooks_inode, "pre-commit");
    let fh = block_on(fs.open(dummy_req(), hook_inode, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), hook_inode, fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(fs.lookup(dummy_req(), git_inode, OsStr::new("config"))).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn default_hidden_recursive_visible_glob_does_not_expose_unrelated_directories() {
    let dir = test_dir("visible-glob-actual-descendant");
    std::fs::create_dir_all(dir.join("with-secret/nested")).unwrap();
    std::fs::create_dir_all(dir.join("without-secret/nested")).unwrap();
    std::fs::write(dir.join("with-secret/nested/public.pem"), b"ok").unwrap();
    std::fs::write(dir.join("without-secret/nested/readme.txt"), b"nope").unwrap();
    std::fs::write(dir.join("secret.txt"), b"hidden target").unwrap();
    std::os::unix::fs::symlink("../secret.txt", dir.join("without-secret/link.pem")).unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["**/*.pem".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"with-secret".to_string()));
    assert!(!root_names.contains(&"without-secret".to_string()));
    let with_secret = lookup_root_inode(&fs, "with-secret");
    let nested = lookup_child_inode(&fs, with_secret, "nested");
    let pem = lookup_child_inode(&fs, nested, "public.pem");
    let fh = block_on(fs.open(dummy_req(), pem, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), pem, fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn default_hidden_cwd_rebased_direct_child_visible_glob_uses_dynamic_bridge_index() {
    let dir = test_dir("visible-cwd-direct-child-bridge");
    let source = dir.join("src");
    let cwd = source.join("workspace/app");
    std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    std::fs::write(source.join("workspace/app/fixtures/local.pem"), b"ok").unwrap();
    std::fs::write(
        source.join("workspace/app/fixtures/nested/local.pem"),
        b"nested",
    )
    .unwrap();
    std::fs::write(source.join("workspace/app/readme.txt"), b"hidden").unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    let fs = {
        let _env = ProcessEnvGuard::new(&cwd, None);
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: Vec::new(),
                visibility_visible_rules: vec!["./fixtures/*.pem".to_string()],
                mutability_readonly_rules: Vec::new(),
                mutability_writable_rules: Vec::new(),
            },
            config_path: None,
            visibility_default: Some(crate::cli::VisibilityDefault::Hidden),
            mutability_default: None,
        })
        .unwrap();
        ScreenFs::new(cfg)
    };

    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"workspace".to_string()));

    let workspace = lookup_root_inode(&fs, "workspace");
    let workspace_fh = open_directory_handle(&fs, workspace);
    let workspace_names: Vec<String> =
        block_on(fs.readdirplus(dummy_req(), workspace, workspace_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(workspace_names.contains(&"app".to_string()));
    block_on(fs.releasedir(dummy_req(), workspace, workspace_fh, 0)).unwrap();

    let app = lookup_child_inode(&fs, workspace, "app");
    let app_fh = open_directory_handle(&fs, app);
    let app_names: Vec<String> = block_on(fs.readdirplus(dummy_req(), app, app_fh, 0, 4096))
        .unwrap()
        .into_iter()
        .map(|entry| String::from_utf8(entry.name).unwrap())
        .collect();
    assert!(app_names.contains(&"fixtures".to_string()));
    assert!(!app_names.contains(&"readme.txt".to_string()));
    block_on(fs.releasedir(dummy_req(), app, app_fh, 0)).unwrap();

    let fixtures = lookup_child_inode(&fs, app, "fixtures");
    let fixtures_fh = open_directory_handle(&fs, fixtures);
    let fixture_names: Vec<String> =
        block_on(fs.readdirplus(dummy_req(), fixtures, fixtures_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(fixture_names.contains(&"local.pem".to_string()));
    assert!(!fixture_names.contains(&"nested".to_string()));
    block_on(fs.releasedir(dummy_req(), fixtures, fixtures_fh, 0)).unwrap();

    let pem = lookup_child_inode(&fs, fixtures, "local.pem");
    let fh = block_on(fs.open(dummy_req(), pem, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(fs.release(dummy_req(), pem, fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(fs.lookup(dummy_req(), fixtures, OsStr::new("nested"))).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn default_hidden_absolute_txt_visible_globs_distinguish_direct_child_and_recursive_matches() {
    let dir = test_dir("visible-absolute-txt-glob-families");
    std::fs::create_dir_all(dir.join("a/nested")).unwrap();
    std::fs::create_dir_all(dir.join("b")).unwrap();
    std::fs::write(dir.join("a/top.txt"), b"top").unwrap();
    std::fs::write(dir.join("a/nested/deep.txt"), b"deep").unwrap();
    std::fs::write(dir.join("b/other.txt"), b"other").unwrap();

    let direct_fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/a/*.txt".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );
    let direct_root_names = root_listing_names(&direct_fs);
    assert!(direct_root_names.contains(&"a".to_string()));
    assert!(!direct_root_names.contains(&"b".to_string()));
    let direct_a = lookup_root_inode(&direct_fs, "a");
    let direct_a_fh = open_directory_handle(&direct_fs, direct_a);
    let direct_a_names: Vec<String> =
        block_on(direct_fs.readdirplus(dummy_req(), direct_a, direct_a_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(direct_a_names.contains(&"top.txt".to_string()));
    assert!(!direct_a_names.contains(&"nested".to_string()));
    block_on(direct_fs.releasedir(dummy_req(), direct_a, direct_a_fh, 0)).unwrap();
    let top = lookup_child_inode(&direct_fs, direct_a, "top.txt");
    let fh = block_on(direct_fs.open(dummy_req(), top, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(direct_fs.release(dummy_req(), top, fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(direct_fs.lookup(dummy_req(), direct_a, OsStr::new("nested"))).unwrap_err(),
        ENOENT
    );

    let recursive_fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/a/**/*.txt".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );
    let recursive_root_names = root_listing_names(&recursive_fs);
    assert!(recursive_root_names.contains(&"a".to_string()));
    assert!(!recursive_root_names.contains(&"b".to_string()));
    let recursive_a = lookup_root_inode(&recursive_fs, "a");
    let recursive_a_fh = open_directory_handle(&recursive_fs, recursive_a);
    let recursive_a_names: Vec<String> =
        block_on(recursive_fs.readdirplus(dummy_req(), recursive_a, recursive_a_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(recursive_a_names.contains(&"top.txt".to_string()));
    assert!(recursive_a_names.contains(&"nested".to_string()));
    block_on(recursive_fs.releasedir(dummy_req(), recursive_a, recursive_a_fh, 0)).unwrap();
    let nested = lookup_child_inode(&recursive_fs, recursive_a, "nested");
    let nested_fh = open_directory_handle(&recursive_fs, nested);
    let nested_names: Vec<String> =
        block_on(recursive_fs.readdirplus(dummy_req(), nested, nested_fh, 0, 4096))
            .unwrap()
            .into_iter()
            .map(|entry| String::from_utf8(entry.name).unwrap())
            .collect();
    assert!(nested_names.contains(&"deep.txt".to_string()));
    block_on(recursive_fs.releasedir(dummy_req(), nested, nested_fh, 0)).unwrap();
    let deep = lookup_child_inode(&recursive_fs, nested, "deep.txt");
    let fh = block_on(recursive_fs.open(dummy_req(), deep, libc::O_RDONLY as u32))
        .unwrap()
        .fh;
    block_on(recursive_fs.release(dummy_req(), deep, fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hidden_direct_child_suffix_rules_keep_enoent_precedence_over_readonly() {
    let dir = test_dir("hidden-direct-child-suffix-precedence");
    let source = dir.join("src");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(source.join("home/tester/nested")).unwrap();
    std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    std::fs::write(source.join("home/tester/user.pem"), b"secret").unwrap();
    std::fs::write(source.join("home/tester/nested/user.pem"), b"nested").unwrap();
    std::fs::write(source.join("workspace/app/fixtures/local.pem"), b"ok").unwrap();
    std::fs::write(
        source.join("workspace/app/fixtures/nested/local.pem"),
        b"nested",
    )
    .unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    let fs = {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["~/*.pem".to_string()],
                visibility_visible_rules: Vec::new(),
                mutability_readonly_rules: vec!["~/*.pem".to_string()],
                mutability_writable_rules: Vec::new(),
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
        })
        .unwrap();
        ScreenFs::new(cfg)
    };

    let hidden_path = vpath("/home/tester/user.pem");
    assert!(fs.config().is_hidden(&hidden_path));
    assert!(fs.config().is_readonly(&hidden_path));
    let hidden_inode = tracked_inode(&fs, "/home/tester/user.pem");
    let err = block_on(fs.open(dummy_req(), hidden_inode, libc::O_WRONLY as u32)).unwrap_err();
    assert_eq!(err, ENOENT);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn writable_default_readonly_descendant_subtree_rule_only_locks_git_hooks_subtree() {
    let dir = test_dir("writable-default-git-hooks-subtree");
    std::fs::create_dir_all(dir.join("project/repo/.git/hooks")).unwrap();
    std::fs::write(dir.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
    std::fs::write(dir.join("project/repo/.git/config"), b"config").unwrap();
    let fs = fs_for(
        &dir,
        Vec::new(),
        vec!["/project/**/.git/hooks/**".to_string()],
    );

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
fn hidden_descendant_subtree_rules_keep_enoent_precedence_over_readonly() {
    let dir = test_dir("hidden-git-hooks-precedence");
    std::fs::create_dir_all(dir.join("project/repo/.git/hooks")).unwrap();
    std::fs::write(dir.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
    std::fs::write(dir.join("project/repo/.git/config"), b"config").unwrap();
    let fs = fs_for(
        &dir,
        vec!["/project/**/.git/hooks/**".to_string()],
        vec!["/project/**/.git/**".to_string()],
    );

    let hidden_hook = tracked_inode(&fs, "/project/repo/.git/hooks/pre-commit");
    let hidden_hooks_dir = tracked_inode(&fs, "/project/repo/.git/hooks");
    let readonly_config = fs
        .reply_entry_for_path(VirtualPath::new("/project/repo/.git/config"))
        .unwrap()
        .attr
        .ino;

    assert_eq!(
        block_on(fs.open(dummy_req(), hidden_hook, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.opendir(dummy_req(), hidden_hooks_dir, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), readonly_config, libc::O_WRONLY as u32)).unwrap_err(),
        libc::EROFS
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn visible_read_and_nested_listing_preserve_access_and_parent_entries() {
    let dir = test_dir("visible-read-list");
    std::fs::create_dir_all(dir.join("a/b")).unwrap();
    std::fs::write(dir.join("a/b/hello.txt"), b"hello world").unwrap();
    let fs = fs_for_root_readonly(&dir, Vec::new());

    let file_inode = fs
        .reply_entry_for_path(VirtualPath::new("/a/b/hello.txt"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), file_inode, libc::O_RDONLY as u32)).unwrap();
    let mut buf = [0_u8; 5];
    let len = block_on(fs.read(dummy_req(), file_inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"hello");
    block_on(fs.release(dummy_req(), file_inode, handle.fh, 0, 0, false, false)).unwrap();

    let parent_inode = fs
        .reply_entry_for_path(VirtualPath::new("/a"))
        .unwrap()
        .attr
        .ino;
    let dir_inode = fs
        .reply_entry_for_path(VirtualPath::new("/a/b"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.opendir(dummy_req(), dir_inode, libc::O_RDONLY as u32)).unwrap();
    let entries = block_on(fs.readdir(dummy_req(), dir_inode, handle.fh, 0, 4096)).unwrap();
    let dotdot = entries
        .iter()
        .find(|entry| entry.name.as_slice() == b"..")
        .unwrap();
    assert_eq!(dotdot.ino, parent_inode);
    assert!(
        entries
            .iter()
            .any(|entry| entry.name.as_slice() == b"hello.txt")
    );

    let plus_entries =
        block_on(fs.readdirplus(dummy_req(), dir_inode, handle.fh, 0, 4096)).unwrap();
    let dotdot = plus_entries
        .iter()
        .find(|entry| entry.name.as_slice() == b"..")
        .unwrap();
    assert_eq!(dotdot.ino, parent_inode);
    assert_eq!(dotdot.attr.ino, parent_inode);
    block_on(fs.releasedir(dummy_req(), dir_inode, handle.fh, 0)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn opendir_returns_real_handle_and_readdir_variants_share_stable_snapshot() {
    let dir = test_dir("dir-handle-snapshot");
    std::fs::create_dir(dir.join("listing")).unwrap();
    std::fs::write(dir.join("listing/alpha"), b"a").unwrap();
    std::fs::write(dir.join("listing/gamma"), b"g").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let listing = fs
        .reply_entry_for_path(VirtualPath::new("/listing"))
        .unwrap()
        .attr
        .ino;

    let handle = block_on(fs.opendir(dummy_req(), listing, libc::O_RDONLY as u32)).unwrap();
    assert_ne!(handle.fh, 0);
    assert!(
        fs.state
            .lock()
            .expect("state mutex poisoned")
            .directories
            .contains_key(&handle.fh)
    );

    let first_page = block_on(fs.readdir(dummy_req(), listing, handle.fh, 0, 4096)).unwrap();
    let alpha = first_page
        .iter()
        .find(|entry| entry.name.as_slice() == b"alpha")
        .unwrap();
    assert_eq!(alpha.offset, 3);

    std::fs::write(dir.join("listing/beta"), b"b").unwrap();
    let remaining =
        block_on(fs.readdirplus(dummy_req(), listing, handle.fh, alpha.offset, 4096)).unwrap();
    let remaining_names = remaining
        .iter()
        .map(|entry| String::from_utf8(entry.name.clone()).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(remaining_names, vec!["gamma".to_string()]);
    assert_eq!(remaining[0].offset, 4);

    block_on(fs.releasedir(dummy_req(), listing, handle.fh, 0)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn releasedir_removes_directory_handle_and_snapshot() {
    let dir = test_dir("releasedir-cleanup");
    std::fs::create_dir(dir.join("listing")).unwrap();
    std::fs::write(dir.join("listing/file"), b"ok").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let listing = fs
        .reply_entry_for_path(VirtualPath::new("/listing"))
        .unwrap()
        .attr
        .ino;

    let handle = block_on(fs.opendir(dummy_req(), listing, libc::O_RDONLY as u32)).unwrap();
    assert!(
        fs.state
            .lock()
            .expect("state mutex poisoned")
            .directories
            .contains_key(&handle.fh)
    );

    block_on(fs.releasedir(dummy_req(), listing, handle.fh, 0)).unwrap();
    assert!(
        !fs.state
            .lock()
            .expect("state mutex poisoned")
            .directories
            .contains_key(&handle.fh)
    );
    assert_eq!(
        block_on(fs.readdir(dummy_req(), listing, handle.fh, 0, 4096)).unwrap_err(),
        ENOENT
    );

    std::fs::remove_dir_all(dir).unwrap();
}
