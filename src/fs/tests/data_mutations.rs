use super::*;

#[test]
fn hidden_xattr_queries_return_enoent() {
    let dir = test_dir("hidden-xattr");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .lock()
        .expect("state mutex poisoned")
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
        .lock()
        .expect("state mutex poisoned")
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
        .lock()
        .expect("state mutex poisoned")
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
    let a_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        a,
        VirtualPath::new("/a"),
        OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
    );
    let b_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
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
    let a_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        a,
        VirtualPath::new("/a"),
        OpenOptions::new().read(true).open(dir.join("a")).unwrap(),
    );
    let b_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
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
        .lock()
        .expect("state mutex poisoned")
        .inode_for_path(VirtualPath::new("/hidden"));
    let visible = fs
        .reply_entry_for_path(VirtualPath::new("/visible"))
        .unwrap()
        .attr
        .ino;
    let hidden_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
        hidden,
        VirtualPath::new("/hidden"),
        OpenOptions::new()
            .read(true)
            .open(dir.join("hidden"))
            .unwrap(),
    );
    let visible_fh = fs.state.lock().expect("state mutex poisoned").insert_file(
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
fn hidden_multi_path_mutations_return_enoent_before_readonly() {
    let dir = test_dir("hidden-mutators");
    std::fs::write(dir.join("hidden"), b"secret").unwrap();
    std::fs::write(dir.join("visible"), b"ok").unwrap();
    let fs = fs_for_root_readonly(&dir, vec!["/hidden".to_string()]);
    let hidden = fs
        .state
        .lock()
        .expect("state mutex poisoned")
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
