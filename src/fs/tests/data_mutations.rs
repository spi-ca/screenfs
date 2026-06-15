use super::*;
use std::os::unix::fs::PermissionsExt;
use std::sync::Arc;

#[test]
fn hidden_xattr_queries_return_enoent() {
    let dir = test_dir("hidden-xattr");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/hidden"));

    assert_eq!(
        block_on(fs.getxattr(dummy_req(), hidden, OsStr::new("user.test"), 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.listxattr(dummy_req(), hidden, 0)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn xattr_operations_use_confined_fd_and_preserve_symlink_visibility() {
    let dir = test_dir("fd-xattr");
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link2")).unwrap();
    std::os::unix::fs::symlink("link2", dir.join("link1")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    let hidden_link = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link1"));

    block_on(fs.setxattr(dummy_req(), visible, OsStr::new("user.test"), b"value", 0)).unwrap();
    let size = block_on(fs.getxattr(dummy_req(), visible, OsStr::new("user.test"), 0)).unwrap();
    assert!(matches!(size, ReplyXattr::Size(5)));
    let value = block_on(fs.getxattr(dummy_req(), visible, OsStr::new("user.test"), 5)).unwrap();
    assert!(matches!(value, ReplyXattr::Data(data) if data == b"value"));
    let names = block_on(fs.listxattr(dummy_req(), visible, 1024)).unwrap();
    assert!(
        matches!(names, ReplyXattr::Data(data) if data.windows(b"user.test".len()).any(|window| window == b"user.test"))
    );
    block_on(fs.removexattr(dummy_req(), visible, OsStr::new("user.test"))).unwrap();

    assert_eq!(
        block_on(fs.getxattr(dummy_req(), hidden_link, OsStr::new("user.test"), 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.setxattr(dummy_req(), hidden_link, OsStr::new("user.test"), b"x", 0))
            .unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn setattr_uses_confined_fd_and_preserves_symlink_visibility() {
    let dir = test_dir("fd-setattr");
    std::fs::write(dir.join("visible"), b"abcdef").unwrap();
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("link2")).unwrap();
    std::os::unix::fs::symlink("link2", dir.join("link1")).unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    let hidden_link = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/link1"));

    block_on(fs.setattr(
        dummy_req(),
        visible,
        None,
        SetAttr {
            size: Some(3),
            mode: Some(0o600),
            ..Default::default()
        },
    ))
    .unwrap();
    assert_eq!(std::fs::read(dir.join("visible")).unwrap(), b"abc");
    assert_eq!(
        std::fs::metadata(dir.join("visible"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );

    assert_eq!(
        block_on(fs.setattr(
            dummy_req(),
            hidden_link,
            None,
            SetAttr {
                mode: Some(0o600),
                ..Default::default()
            },
        ))
        .unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn readonly_multi_path_xattr_and_fallocate_mutations_return_erofs() {
    let dir = test_dir("readonly-mutators");
    std::fs::write(dir.join("a"), b"aaaa").unwrap();
    std::fs::write(dir.join("b"), b"bbbb").unwrap();
    let fs = fs_for_root_readonly(&dir, Vec::new());
    let a = fs
        .reply_entry_for_path(VirtualPath::new("/a"))
        .unwrap()
        .attr
        .ino;
    let b = fs
        .reply_entry_for_path(VirtualPath::new("/b"))
        .unwrap()
        .attr
        .ino;
    let a_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            a,
            VirtualPath::new("/a"),
            OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
        );
    let b_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            b,
            VirtualPath::new("/b"),
            OpenOptions::new().write(true).open(dir.join("b")).unwrap(),
        );

    assert_eq!(
        block_on(fs.link(dummy_req(), a, FUSE_ROOT_ID, OsStr::new("a-link"))).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.symlink(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("sym"),
            OsStr::new("a")
        ))
        .unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.setxattr(dummy_req(), a, OsStr::new("user.test"), b"v", 0)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.removexattr(dummy_req(), a, OsStr::new("user.test"))).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.fallocate(dummy_req(), b, b_fh, 0, 1, 0)).unwrap_err(),
        libc::EROFS
    );
    assert_eq!(
        block_on(fs.copy_file_range(dummy_req(), a, a_fh, 0, b, b_fh, 0, 1, 0)).unwrap_err(),
        libc::EROFS
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn lseek_uses_tracked_file_handle_and_released_handle_fails() {
    let dir = test_dir("lseek-handle");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let fh = insert_tracked_file_handle(
        &fs,
        inode,
        "/file",
        OpenOptions::new()
            .read(true)
            .open(dir.join("file"))
            .unwrap(),
    );

    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, fh, 2, libc::SEEK_SET as u32)).unwrap(),
        2
    );
    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, fh, -1_i64 as u64, libc::SEEK_END as u32)).unwrap(),
        5
    );
    block_on(fs.release(dummy_req(), inode, fh, 0, 0, false, false)).unwrap();
    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, fh, 0, libc::SEEK_SET as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn read_write_offsets_do_not_depend_on_shared_file_position() {
    let dir = test_dir("fileext-offsets");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, handle.fh, 4, libc::SEEK_SET as u32)).unwrap(),
        4
    );
    let mut buf = [0_u8; 3];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abc");

    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, handle.fh, 0, libc::SEEK_END as u32)).unwrap(),
        6
    );
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 2, b"ZZ", 0, 0)).unwrap(),
        2
    );
    assert_eq!(std::fs::read(dir.join("file")).unwrap(), b"abZZef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn cache_eligible_opened_file_read_write_keep_pinned_fd_after_host_rename() {
    let root = test_dir("cache-eligible-pinned-fd-after-host-rename");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("file"), b"abcdef").unwrap();
    let fs = fs_for_external_mount(&source, &mount);
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    std::fs::rename(source.join("file"), source.join("renamed")).unwrap();

    let mut buf = [0_u8; 3];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abc");
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 0, b"ZZ", 0, 0)).unwrap(),
        2
    );
    assert!(!source.join("file").exists());
    assert_eq!(std::fs::read(source.join("renamed")).unwrap(), b"ZZcdef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn cache_eligible_opened_file_read_write_keep_pinned_fd_after_ancestor_rename() {
    let root = test_dir("cache-eligible-pinned-fd-after-ancestor-rename");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(source.join("dir")).unwrap();
    std::fs::write(source.join("dir/file"), b"abcdef").unwrap();
    let fs = fs_for_external_mount(&source, &mount);
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/dir/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    std::fs::rename(source.join("dir"), source.join("moved")).unwrap();

    let mut buf = [0_u8; 3];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abc");
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 0, b"YY", 0, 0)).unwrap(),
        2
    );
    assert!(!source.join("dir/file").exists());
    assert_eq!(std::fs::read(source.join("moved/file")).unwrap(), b"YYcdef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn cache_eligible_opened_file_read_write_keep_pinned_fd_after_unlink() {
    let root = test_dir("cache-eligible-pinned-fd-after-unlink");
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("file"), b"abcdef").unwrap();
    let fs = fs_for_external_mount(&source, &mount);
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    std::fs::remove_file(source.join("file")).unwrap();

    let mut buf = [0_u8; 3];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abc");
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 0, b"XX", 0, 0)).unwrap(),
        2
    );
    assert!(!source.join("file").exists());

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn opened_file_read_write_fail_closed_after_host_rename_into_hidden_subtree() {
    let dir = test_dir("pinned-fd-renamed-hidden");
    std::fs::create_dir(dir.join("hidden")).unwrap();
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, vec!["/hidden".to_string()], Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    std::fs::rename(dir.join("file"), dir.join("hidden/file")).unwrap();

    let mut buf = [0_u8; 3];
    assert_eq!(
        block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 0, b"ZZ", 0, 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(std::fs::read(dir.join("hidden/file")).unwrap(), b"abcdef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn opened_file_read_keeps_pinned_fd_after_host_rename_but_write_fails_closed() {
    let dir = test_dir("pinned-fd-after-host-rename");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    std::fs::rename(dir.join("file"), dir.join("renamed")).unwrap();

    let mut buf = [0_u8; 3];
    let len = block_on(fs.read(dummy_req(), inode, handle.fh, 0, &mut buf)).unwrap();
    assert_eq!(&buf[..len], b"abc");
    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 0, b"ZZ", 0, 0)).unwrap_err(),
        ENOENT
    );
    assert!(!dir.join("file").exists());
    assert_eq!(std::fs::read(dir.join("renamed")).unwrap(), b"abcdef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn flush_and_fsync_complete_under_compio_runtime() {
    let dir = test_dir("compio-flush-fsync");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 2, b"ZZ", 0, 0)).unwrap(),
        2
    );
    compio_block_on(fs.flush(dummy_req(), inode, handle.fh, 0)).unwrap();
    compio_block_on(fs.fsync(dummy_req(), inode, handle.fh, false)).unwrap();
    compio_block_on(fs.fsync(dummy_req(), inode, handle.fh, true)).unwrap();
    assert_eq!(std::fs::read(dir.join("file")).unwrap(), b"abZZef");

    block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, false, false)).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn release_flush_true_completes_under_compio_runtime_and_removes_handle() {
    let dir = test_dir("compio-release-flush");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let inode = fs
        .reply_entry_for_path(VirtualPath::new("/file"))
        .unwrap()
        .attr
        .ino;
    let handle = block_on(fs.open(dummy_req(), inode, libc::O_RDWR as u32)).unwrap();

    assert_eq!(
        block_on(fs.write(dummy_req(), inode, handle.fh, 1, b"YY", 0, 0)).unwrap(),
        2
    );
    compio_block_on(fs.release(dummy_req(), inode, handle.fh, 0, 0, true, false)).unwrap();

    assert_eq!(std::fs::read(dir.join("file")).unwrap(), b"aYYdef");
    assert_eq!(
        block_on(fs.lseek(dummy_req(), inode, handle.fh, 0, libc::SEEK_SET as u32)).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn offload_file_sync_preserves_closure_errno() {
    let dir = test_dir("compio-offload-errno");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let file = Arc::new(
        OpenOptions::new()
            .read(true)
            .open(dir.join("file"))
            .unwrap(),
    );

    assert_eq!(
        compio_block_on(ScreenFs::offload_file_sync(file, |_file| Err(libc::ENOSPC))).unwrap_err(),
        libc::ENOSPC
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn offload_file_sync_completes_without_ambient_compio_runtime() {
    let dir = test_dir("thread-offload-no-compio");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let file = Arc::new(
        OpenOptions::new()
            .read(true)
            .open(dir.join("file"))
            .unwrap(),
    );

    waking_block_on(ScreenFs::offload_file_sync(file, |_file| Ok(()))).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn offload_file_sync_maps_closure_panic_to_eio() {
    let dir = test_dir("compio-offload-panic");
    std::fs::write(dir.join("file"), b"abcdef").unwrap();
    let file = Arc::new(
        OpenOptions::new()
            .read(true)
            .open(dir.join("file"))
            .unwrap(),
    );

    assert_eq!(
        compio_block_on(ScreenFs::offload_file_sync(file, |_file| panic!("boom"))).unwrap_err(),
        libc::EIO
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn visible_copy_file_range_copies_data() {
    let dir = test_dir("copy-visible");
    std::fs::write(dir.join("a"), b"abcdef").unwrap();
    std::fs::write(dir.join("b"), b"------").unwrap();
    let fs = fs_for(&dir, Vec::new(), Vec::new());
    let a = fs
        .reply_entry_for_path(VirtualPath::new("/a"))
        .unwrap()
        .attr
        .ino;
    let b = fs
        .reply_entry_for_path(VirtualPath::new("/b"))
        .unwrap()
        .attr
        .ino;
    let a_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            a,
            VirtualPath::new("/a"),
            OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
        );
    let b_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            b,
            VirtualPath::new("/b"),
            OpenOptions::new().write(true).open(dir.join("b")).unwrap(),
        );

    let copied = block_on(fs.copy_file_range(dummy_req(), a, a_fh, 1, b, b_fh, 2, 3, 0)).unwrap();
    assert_eq!(copied, 3);
    assert_eq!(std::fs::read(dir.join("b")).unwrap(), b"--bcd-");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hidden_copy_file_range_paths_return_enoent_before_readonly() {
    let dir = test_dir("copy-hidden");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::fs::write(dir.join("visible"), b"------").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/hidden"));
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    let hidden_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            hidden,
            VirtualPath::new("/hidden"),
            OpenOptions::new()
                .read(true)
                .open(dir.join("hidden"))
                .unwrap(),
        );
    let visible_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            visible,
            VirtualPath::new("/visible"),
            OpenOptions::new()
                .write(true)
                .open(dir.join("visible"))
                .unwrap(),
        );

    assert_eq!(
        block_on(fs.copy_file_range(
            dummy_req(),
            hidden,
            hidden_fh,
            0,
            visible,
            visible_fh,
            0,
            1,
            0,
        ))
        .unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.copy_file_range(
            dummy_req(),
            visible,
            visible_fh,
            0,
            hidden,
            hidden_fh,
            0,
            1,
            0,
        ))
        .unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn copy_file_range_symlink_destination_hidden_target_returns_enoent_before_readonly_fast_path() {
    let dir = test_dir("copy-hidden-symlink-destination");
    std::fs::write(dir.join("input"), b"abcdef").unwrap();
    std::fs::write(dir.join("hidden"), b"------").unwrap();
    std::os::unix::fs::symlink("hidden", dir.join("alias")).unwrap();
    let fs = fs_for_policy(
        &dir,
        vec!["/hidden".to_string()],
        Vec::new(),
        Some(MutabilityDefault::Readonly),
        Vec::new(),
    );
    let input = fs
        .reply_entry_for_path(VirtualPath::new("/input"))
        .unwrap()
        .attr
        .ino;
    let alias_path = VirtualPath::new("/alias");
    let alias = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(alias_path.clone());
    let input_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            input,
            VirtualPath::new("/input"),
            OpenOptions::new()
                .read(true)
                .open(dir.join("input"))
                .unwrap(),
        );
    let alias_fh = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .insert_file(
            alias,
            alias_path.clone(),
            OpenOptions::new()
                .write(true)
                .open(dir.join("hidden"))
                .unwrap(),
        );

    assert!(
        fs.config().can_skip_resolved_target_mutability_check(
            fs.config().mutability_decision(&alias_path)
        )
    );
    assert_eq!(
        block_on(fs.copy_file_range(dummy_req(), input, input_fh, 0, alias, alias_fh, 0, 1, 0,))
            .unwrap_err(),
        ENOENT
    );
    assert_eq!(std::fs::read(dir.join("hidden")).unwrap(), b"------");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn hidden_multi_path_mutations_return_enoent_before_readonly() {
    let dir = test_dir("hidden-mutators");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .write()
        .expect("state rwlock poisoned")
        .inode_for_path(VirtualPath::new("/hidden"));
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;

    assert_eq!(
        block_on(fs.link(dummy_req(), hidden, FUSE_ROOT_ID, OsStr::new("copy"))).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.rename(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("hidden"),
            FUSE_ROOT_ID,
            OsStr::new("renamed"),
            0,
        ))
        .unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.rename(
            dummy_req(),
            FUSE_ROOT_ID,
            OsStr::new("visible"),
            FUSE_ROOT_ID,
            OsStr::new("hidden"),
            0,
        ))
        .unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.setxattr(dummy_req(), hidden, OsStr::new("user.test"), b"v", 0)).unwrap_err(),
        ENOENT
    );
    assert_eq!(
        block_on(fs.link(dummy_req(), visible, FUSE_ROOT_ID, OsStr::new("hidden"))).unwrap_err(),
        ENOENT
    );
    std::fs::remove_dir_all(dir).unwrap();
}
