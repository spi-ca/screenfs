use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;

#[test]
fn readonly_write_intent_open_and_opendir_return_erofs_but_hidden_stays_enoent() {
    let dir = test_dir("readonly-open");
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    std::fs::create_dir(dir.join("docs")).unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    let docs = fs
        .reply_entry_for_path(VirtualPath::new("/docs"))
        .unwrap()
        .attr
        .ino;
    let hidden = fs.reply_entry_for_path(VirtualPath::new("/hidden"));
    assert_eq!(hidden.unwrap_err(), ENOENT);

    let err = block_on(fs.open(dummy_req(), visible, libc::O_WRONLY as u32)).unwrap_err();
    assert_eq!(err, libc::EROFS);
    let err = block_on(fs.opendir(dummy_req(), docs, libc::O_WRONLY as u32)).unwrap_err();
    assert_eq!(err, libc::EROFS);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn symlink_to_hidden_target_returns_enoent_before_readonly_for_write_intent() {
    let dir = test_dir("symlink-hidden-readonly-mutation");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link"));

    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::W_OK as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn symlink_to_hidden_target_returns_enoent_for_lookup_open_and_readlink() {
    let dir = test_dir("symlink-hidden");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

    let lookup_err = fs
        .reply_entry_for_path(VirtualPath::new("/link"))
        .unwrap_err();
    assert_eq!(lookup_err, ENOENT);

    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link"));
    let open_err = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err();
    assert_eq!(open_err, ENOENT);
    let readlink_err = block_on(fs.readlink(dummy_req(), inode)).unwrap_err();
    assert_eq!(readlink_err, ENOENT);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn multi_hop_symlink_to_hidden_target_returns_enoent() {
    let dir = test_dir("symlink-hidden-multi-hop");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link2")).unwrap();
    std::os::unix::fs::symlink("link2", dir.join("link1")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

    assert_eq!(
        fs.reply_entry_for_path(VirtualPath::new("/link1"))
            .unwrap_err(),
        ENOENT
    );
    assert!(!root_listing_names(&fs).contains(&"link1".to_string()));

    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link1"));
    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.readlink(dummy_req(), inode)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn ancestor_symlink_into_hidden_subtree_returns_enoent_for_child() {
    let dir = test_dir("symlink-hidden-ancestor");
    std::fs::create_dir_all(dir.join("secret/subdir")).unwrap();
    std::fs::write(dir.join("secret/subdir/file"), b"secret").unwrap();
    std::os::unix::fs::symlink("secret", dir.join("visible-dir-link")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/secret".to_string()]);

    assert_eq!(
        fs.reply_entry_for_path(VirtualPath::new("/visible-dir-link/subdir/file"))
            .unwrap_err(),
        ENOENT
    );
    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/visible-dir-link/subdir/file"));
    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn symlink_to_bridge_visible_target_is_not_exported() {
    let dir = test_dir("symlink-bridge-visible-target");
    std::fs::create_dir_all(dir.join("target/child")).unwrap();
    std::fs::write(dir.join("target/child/file.txt"), b"ok").unwrap();
    std::os::unix::fs::symlink("target", dir.join("link-to-bridge")).unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/target/child".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    assert_eq!(
        fs.config()
            .visibility_decision(&VirtualPath::new("/target")),
        crate::config::VisibilityDecision::BridgeVisible
    );
    let root_names = root_listing_names(&fs);
    assert!(root_names.contains(&"target".to_string()));
    assert!(!root_names.contains(&"link-to-bridge".to_string()));
    assert_eq!(
        fs.reply_entry_for_path(VirtualPath::new("/link-to-bridge"))
            .unwrap_err(),
        ENOENT
    );
    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link-to-bridge"));
    assert_eq!(
        block_on(fs.readlink(dummy_req(), inode)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.opendir(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn visible_symlink_to_bridge_visible_target_returns_enoent_for_write_intent_ops() {
    let dir = test_dir("symlink-bridge-visible-write-intent");
    std::fs::create_dir_all(dir.join("target/child")).unwrap();
    std::os::unix::fs::symlink("target", dir.join("link-to-bridge")).unwrap();
    let fs = fs_for_axes(
        &dir,
        Some(crate::cli::VisibilityDefault::Hidden),
        Vec::new(),
        vec!["/target/child".to_string(), "/link-to-bridge".to_string()],
        Some(crate::cli::MutabilityDefault::Readonly),
        Vec::new(),
        Vec::new(),
    );
    let link_path = VirtualPath::new("/link-to-bridge");
    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(link_path.clone());

    assert_eq!(
        fs.config()
            .visibility_decision(&VirtualPath::new("/target")),
        crate::config::VisibilityDecision::BridgeVisible
    );
    assert_eq!(
        fs.config().visibility_decision(&link_path),
        crate::config::VisibilityDecision::Visible
    );
    assert!(
        fs.config()
            .can_skip_resolved_target_mutability_check(fs.config().mutability_decision(&link_path))
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::W_OK as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.opendir(dummy_req(), inode, libc::O_WRONLY as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn broader_visible_rule_does_not_expose_more_specific_hidden_symlink_target() {
    let dir = test_dir("symlink-hidden-under-broader-visible");
    std::fs::create_dir_all(dir.join("secret")).unwrap();
    std::fs::write(dir.join("secret/file"), b"secret").unwrap();
    std::os::unix::fs::symlink("secret/file", dir.join("link")).unwrap();
    let fs = fs_for_axes(
        &dir,
        None,
        vec!["/secret".to_string()],
        vec!["/".to_string()],
        None,
        Vec::new(),
        Vec::new(),
    );

    let lookup_err = fs
        .reply_entry_for_path(VirtualPath::new("/link"))
        .unwrap_err();
    assert_eq!(lookup_err, ENOENT);

    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link"));
    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.readlink(dummy_req(), inode)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn symlink_creation_to_hidden_final_target_rolls_back_side_effect() {
    let dir = test_dir("symlink-create-hidden-final");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link2")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());

    assert_eq!(
        block_on(fs.symlink(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("link1"),
            OsStr::new("link2"),
        ))
        .unwrap_err(),
        ENOENT
    );
    assert!(!dir.join("link1").exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent() {
    let root = test_dir("symlink-source-root-escape");
    let source = root.join("source");
    let outside = root.join("outside");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("secret.txt"), b"secret").unwrap();
    std::os::unix::fs::symlink("../outside/secret.txt", source.join("escape")).unwrap();
    let fs = fs_for_root_readonly(&source, Vec::new());

    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/escape"))
        .unwrap()
        .attr
        .ino;
    let readlink = block_on(fs.readlink(dummy_req(), inode)).unwrap();
    assert_eq!(readlink.data, b"../outside/secret.txt");
    assert_eq!(
        block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn opened_symlink_read_revalidates_hidden_target_after_retarget() {
    let dir = test_dir("opened-symlink-hidden-retarget");
    std::fs::write(dir.join("visible.txt"), b"public").unwrap();
    std::fs::write(dir.join("hidden.txt"), b"secret").unwrap();
    std::os::unix::fs::symlink("visible.txt", dir.join("alias")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden.txt".to_string()], Vec::new());
    assert!(!fs.config().can_skip_symlink_target_visibility_check());

    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/alias"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();

    std::fs::remove_file(dir.join("alias")).unwrap();
    std::os::unix::fs::symlink("hidden.txt", dir.join("alias")).unwrap();

    let mut buf = [0_u8; 6];
    assert_eq!(
        block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap_err(),
        ENOENT
    );
    assert_eq!(std::fs::read(dir.join("visible.txt")).unwrap(), b"public");
    assert_eq!(std::fs::read(dir.join("hidden.txt")).unwrap(), b"secret");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn cache_eligible_opened_symlink_read_keeps_pinned_fd_after_final_retarget() {
    let root = test_dir("cache-eligible-opened-symlink-final-retarget");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("first.txt"), b"first").unwrap();
    std::fs::write(source.join("second.txt"), b"second").unwrap();
    std::os::unix::fs::symlink("first.txt", source.join("alias")).unwrap();
    let fs = fs_for_external_mount(&source, &mount);
    assert!(fs.config().can_skip_symlink_target_visibility_check());

    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/alias"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();

    std::fs::remove_file(source.join("alias")).unwrap();
    std::os::unix::fs::symlink("second.txt", source.join("alias")).unwrap();

    let mut buf = [0_u8; 5];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"first");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn cache_eligible_opened_symlink_read_keeps_pinned_fd_after_ancestor_retarget() {
    let root = test_dir("cache-eligible-opened-symlink-ancestor-retarget");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::create_dir(source.join("first")).unwrap();
    std::fs::create_dir(source.join("second")).unwrap();
    std::fs::write(source.join("first/file.txt"), b"first").unwrap();
    std::fs::write(source.join("second/file.txt"), b"second").unwrap();
    std::os::unix::fs::symlink("first", source.join("alias-dir")).unwrap();
    let fs = fs_for_external_mount(&source, &mount);
    assert!(fs.config().can_skip_symlink_target_visibility_check());

    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/alias-dir/file.txt"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap();

    std::fs::remove_file(source.join("alias-dir")).unwrap();
    std::os::unix::fs::symlink("second", source.join("alias-dir")).unwrap();

    let mut buf = [0_u8; 5];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"first");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn deleted_source_root_resolved_path_fails_closed() {
    let source = test_dir("deleted-source-root-resolve");
    let fs = fs_for(&source, Vec::new(), Vec::new());
    std::fs::remove_dir_all(source.join("mount")).unwrap();
    std::fs::remove_dir(&source).unwrap();

    assert!(
        fs.resolved_virtual_path(&VirtualPath::root(), true)
            .is_err()
    );
}

#[test]
fn symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects() {
    let root = test_dir("symlink-dir-source-root-escape");
    let source = root.join("source");
    let outside = root.join("outside");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    std::os::unix::fs::symlink("../outside", source.join("escape-dir")).unwrap();
    let fs = fs_for(&source, Vec::new(), Vec::new());

    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/escape-dir"))
        .unwrap()
        .attr
        .ino;
    assert_eq!(
        block_on(fs.opendir(dummy_req(), inode, libc::O_RDONLY as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.create(
            dummy_req(),
            inode,
            OsStr::new("created.txt"),
            0o644,
            libc::O_WRONLY as u32,
        ))
        .unwrap_err(),
        ENOENT
    );
    assert!(!outside.join("created.txt").exists());

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn access_passes_through_host_permissions_and_readonly_write_mask() {
    let dir = test_dir("access");
    let file = dir.join("file");
    std::fs::write(&file, b"data").unwrap();
    let fs = fs_for_root_readonly(&dir, Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;

    let write_err = block_on(fs.access(dummy_req(), inode, libc::W_OK as u32)).unwrap_err();
    assert_eq!(write_err, libc::EROFS);

    if unsafe { libc::getuid() } != 0 {
        std::fs::set_permissions(&file, fs::Permissions::from_mode(0o0)).unwrap();
        let read_err = block_on(fs.access(dummy_req(), inode, libc::R_OK as u32)).unwrap_err();
        assert_eq!(read_err, libc::EACCES);
    }
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hidden_access_returns_enoent_for_read_and_write_masks() {
    let dir = test_dir("hidden-access");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/hidden"));

    assert_eq!(
        block_on(fs.access(dummy_req(), hidden, libc::R_OK as u32)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.access(dummy_req(), hidden, libc::W_OK as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_create_returns_erofs_instead_of_enosys() {
    let dir = test_dir("readonly-create");
    let fs = fs_for_root_readonly(&dir, Vec::new());
    let err = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("new-file"),
        0o644,
        libc::O_CREAT as u32,
    ))
    .unwrap_err();
    assert_eq!(err, libc::EROFS);
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn statfs_reflects_backing_filesystem_instead_of_placeholder_values() {
    let dir = test_dir("statfs");
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::fs::create_dir(dir.join("nested")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    fs.state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/nested"));

    let stats = block_on(fs.statfs(dummy_req(), visible)).unwrap();
    let host = host_statfs(&dir);
    let tracked_inodes = fs.state.read().expect("state rwlock poisoned").inodes.len() as u64;

    assert_eq!(stats.blocks, host.blocks);
    assert_eq!(stats.files, host.files);
    assert_eq!(stats.bsize, host.bsize);
    assert_eq!(stats.frsize, host.frsize);
    assert_eq!(stats.namelen, host.namelen);
    assert!(stats.blocks > 0);
    assert!(stats.bfree <= stats.blocks);
    assert!(stats.bavail <= stats.bfree);
    if host.files > 0 {
        assert!(stats.ffree <= stats.files);
        assert!(stats.files > tracked_inodes);
    } else {
        assert_eq!(stats.files, 0);
    }

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn open_honors_host_access_mode_truncate_and_append_flags() {
    let dir = test_dir("open-host-flags");
    let file = dir.join("file");
    std::fs::write(&file, b"abc").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;

    let truncated =
        block_on(fs.open(dummy_req(), inode, (libc::O_WRONLY | libc::O_TRUNC) as u32)).unwrap();
    let mut buf = [0_u8; 1];
    let read_err = block_on(fs.read(dummy_req(), inode, truncated.fh, 0, &mut buf)).unwrap_err();
    assert_eq!(read_err, libc::EBADF);
    block_on(fs.release(dummy_req(), inode, truncated.fh, 0, 0, false, false)).unwrap();
    assert_eq!(std::fs::read(&file).unwrap(), Vec::<u8>::new());

    std::fs::write(&file, b"abc").unwrap();
    let appended =
        block_on(fs.open(dummy_req(), inode, (libc::O_RDWR | libc::O_APPEND) as u32)).unwrap();
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, appended.fh, 0, b"z", 0, 0)).unwrap(),
        1
    );
    let mut buf = [0_u8; 4];
    let len = block_on(fs.read(dummy_req(), inode, appended.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abcz");
    block_on(fs.release(dummy_req(), inode, appended.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn write_intent_open_hidden_symlink_does_not_truncate_target() {
    let dir = test_dir("open-hidden-symlink-no-truncate");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());
    let inode = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link"));

    let err =
        block_on(fs.open(dummy_req(), inode, (libc::O_WRONLY | libc::O_TRUNC) as u32)).unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn create_existing_symlink_to_hidden_target_returns_enoent_before_side_effects() {
    let dir = test_dir("create-hidden-symlink-target");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());

    let err = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("link"),
        0o644,
        (libc::O_CREAT | libc::O_TRUNC | libc::O_RDWR) as u32,
    ))
    .unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn create_existing_visible_symlink_is_rejected_without_target_side_effect() {
    let dir = test_dir("create-visible-symlink-no-follow");
    std::fs::write(dir.join("target"), b"target").unwrap();
    std::os::unix::fs::symlink("target", dir.join("link")).unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let err = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("link"),
        0o644,
        (libc::O_CREAT | libc::O_TRUNC | libc::O_RDWR) as u32,
    ))
    .unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(std::fs::read(dir.join("target")).unwrap(), b"target");
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn unlink_hidden_target_symlink_returns_enoent_and_preserves_entry() {
    let dir = test_dir("unlink-hidden-target-symlink");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());

    let err = block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("link"))).unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(
        std::fs::read_link(dir.join("link")).unwrap(),
        std::path::PathBuf::from("hidden")
    );
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn unlink_hidden_target_symlink_returns_enoent_before_readonly() {
    let dir = test_dir("unlink-hidden-target-symlink-readonly");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

    let err = block_on(fs.unlink(dummy_req(), FUSE_ROOT_ID, OsStr::new("link"))).unwrap_err();

    assert_eq!(err, ENOENT);
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rename_hidden_target_symlink_source_returns_enoent_and_preserves_entry() {
    let dir = test_dir("rename-hidden-target-symlink-source");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());

    let err = block_on(fs.rename(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("link"),
        FUSE_ROOT_ID,
        OsStr::new("renamed"),
        0,
    ))
    .unwrap_err();

    assert_eq!(err, ENOENT);
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(!dir.join("renamed").exists());
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn rename_over_hidden_target_symlink_returns_enoent_without_overwrite() {
    let dir = test_dir("rename-over-hidden-target-symlink");
    std::fs::write(dir.join("visible"), b"public").unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());

    let err = block_on(fs.rename(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("visible"),
        FUSE_ROOT_ID,
        OsStr::new("link"),
        0,
    ))
    .unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(std::fs::read(dir.join("visible")).unwrap(), b"public");
    assert_eq!(
        std::fs::read_link(dir.join("link")).unwrap(),
        std::path::PathBuf::from("hidden")
    );
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hard_link_existing_hidden_target_symlink_returns_enoent_before_eexist() {
    let dir = test_dir("link-existing-hidden-target-symlink");
    std::fs::write(dir.join("visible"), b"public").unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;

    let err =
        block_on(fs.link(dummy_req(), visible, FUSE_ROOT_ID, OsStr::new("link"))).unwrap_err();

    assert_eq!(err, ENOENT);
    assert_eq!(std::fs::read(dir.join("visible")).unwrap(), b"public");
    assert_eq!(
        std::fs::read_link(dir.join("link")).unwrap(),
        std::path::PathBuf::from("hidden")
    );
    assert!(
        std::fs::symlink_metadata(dir.join("link"))
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"secret");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn mutation_under_visible_symlink_parent_uses_resolved_parent() {
    let dir = test_dir("symlink-parent-mutation");
    std::fs::create_dir(dir.join("target")).unwrap();
    std::os::unix::fs::symlink("target", dir.join("alias")).unwrap();
    std::fs::write(dir.join("target/remove-me"), b"old").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let alias = lookup_root_inode(&fs, "alias");

    block_on(fs.mkdir(dummy_req(), alias, OsStr::new("child"), 0o755, 0)).unwrap();
    assert!(dir.join("target/child").is_dir());

    block_on(fs.unlink(dummy_req(), alias, OsStr::new("remove-me"))).unwrap();
    assert!(!dir.join("target/remove-me").exists());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn mkdir_restrictive_mode_returns_success_after_creation() {
    let dir = test_dir("mkdir-restrictive-mode");
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    block_on(fs.mkdir(dummy_req(), FUSE_ROOT_ID, OsStr::new("private"), 0o000, 0)).unwrap();
    assert!(dir.join("private").is_dir());
    assert_eq!(
        std::fs::metadata(dir.join("private"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0
    );

    std::fs::remove_dir(dir.join("private")).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn create_honors_host_exclusive_and_readonly_flags() {
    let dir = test_dir("create-host-flags");
    std::fs::write(dir.join("existing"), b"abc").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());

    let err = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("existing"),
        0o644,
        (libc::O_CREAT | libc::O_EXCL | libc::O_WRONLY) as u32,
    ))
    .unwrap_err();
    assert_eq!(err, libc::EEXIST);

    let created = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("new"),
        0o644,
        libc::O_RDONLY as u32,
    ))
    .unwrap();
    let write_err =
        block_on(fs.write(dummy_req(), created.attr.ino, created.fh, 0, b"x", 0, 0)).unwrap_err();
    assert_eq!(write_err, libc::EBADF);
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
    assert!(dir.join("new").is_file());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hidden_create_returns_enoent_before_readonly() {
    let dir = test_dir("hidden-create");
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);

    let err = block_on(fs.create(
        dummy_req(),
        FUSE_ROOT_ID,
        OsStr::new("hidden"),
        0o644,
        libc::O_CREAT as u32,
    ))
    .unwrap_err();
    assert_eq!(err, ENOENT);
    std::fs::remove_dir_all(dir).unwrap();
}
