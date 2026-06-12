use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn normalizes_lexical_absolute_virtual_paths() {
    assert_eq!(VirtualPath::new("/a//b/./c").as_path(), Path::new("/a/b/c"));
    assert_eq!(VirtualPath::new("/a/b/../c").as_path(), Path::new("/a/c"));
    assert_eq!(VirtualPath::new("a/./b/../c").as_path(), Path::new("/a/c"));
}

#[test]
fn clamps_parent_escape_at_virtual_root() {
    assert_eq!(
        VirtualPath::new("../../etc/passwd").as_path(),
        Path::new("/etc/passwd")
    );
    assert_eq!(
        VirtualPath::new("/../../etc/passwd").as_path(),
        Path::new("/etc/passwd")
    );
}

#[test]
fn joins_child_and_resolves_symlink_targets_lexically() {
    let link = VirtualPath::new("/a/b/link");
    assert_eq!(
        link.join_child(OsStr::new("child")).as_path(),
        Path::new("/a/b/link/child")
    );
    assert_eq!(
        link.resolve_symlink_target(OsStr::new("../secret"))
            .unwrap()
            .as_path(),
        Path::new("/a/secret")
    );
    assert_eq!(
        link.resolve_symlink_target(OsStr::new("../../../secret"))
            .unwrap()
            .as_path(),
        Path::new("/secret")
    );
    assert_eq!(
        link.resolve_symlink_target(OsStr::new("/safe/../secret"))
            .unwrap()
            .as_path(),
        Path::new("/secret")
    );
}

#[test]
fn resolves_root_symlink_targets_without_escaping_virtual_root() {
    let link = VirtualPath::new("/link");
    assert_eq!(
        link.resolve_symlink_target(OsStr::new("../secret"))
            .unwrap()
            .as_path(),
        Path::new("/secret")
    );
}

#[test]
fn converts_virtual_path_to_source_relative_and_host_paths_without_root_escape() {
    assert_eq!(
        VirtualPath::root().to_source_relative_path(),
        PathBuf::new()
    );
    assert_eq!(
        VirtualPath::new("/a/../b").to_source_relative_path(),
        PathBuf::from("b")
    );
    assert_eq!(
        VirtualPath::root().to_source_path(Path::new("/src")),
        PathBuf::from("/src")
    );
    assert_eq!(
        VirtualPath::new("/a/../b").to_source_path(Path::new("/src")),
        PathBuf::from("/src/b")
    );
}

#[test]
fn resolves_host_paths_within_source_root_without_changing_virtual_lexical_join() {
    let root = test_dir("resolve-host-safe");
    let source = root.join("source");
    std::fs::create_dir_all(source.join("real")).unwrap();
    std::fs::write(source.join("real/file.txt"), b"ok").unwrap();
    std::os::unix::fs::symlink("real", source.join("linkdir")).unwrap();

    assert_eq!(
        VirtualPath::new("/linkdir/file.txt")
            .to_source_path(&source)
            .as_path(),
        source.join("linkdir/file.txt").as_path()
    );
    assert_eq!(
        VirtualPath::new("/linkdir/file.txt")
            .resolve_host_path(&source, true)
            .unwrap(),
        source.join("real/file.txt").canonicalize().unwrap()
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn rejects_following_symlinks_outside_source_root_but_allows_link_itself() {
    let root = test_dir("resolve-host-escape");
    let source = root.join("source");
    let outside = root.join("outside");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("secret.txt"), b"secret").unwrap();
    std::os::unix::fs::symlink("../outside/secret.txt", source.join("escape-file")).unwrap();
    std::os::unix::fs::symlink("../outside", source.join("escape-dir")).unwrap();

    assert_eq!(
        VirtualPath::new("/escape-file")
            .resolve_host_path(&source, false)
            .unwrap(),
        source.join("escape-file")
    );
    let err = VirtualPath::new("/escape-file")
        .resolve_host_path(&source, true)
        .unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

    let err = VirtualPath::new("/escape-dir/secret.txt")
        .resolve_host_path(&source, true)
        .unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn normalizes_relative_and_tilde_rule_paths_inside_source_root() {
    let root = test_dir("rule-normalization-inside");
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(home.join("config")).unwrap();
    let ctx = RuleNormalizationContext::new(&source, &cwd, Some(home.clone())).unwrap();

    assert_eq!(
        normalize_rule_path("../secrets", &ctx).unwrap().as_path(),
        Path::new("/workspace/secrets")
    );
    assert_eq!(
        normalize_rule_path("~/config", &ctx).unwrap().as_path(),
        Path::new("/home/tester/config")
    );
    assert_eq!(
        normalize_rule_path("~", &ctx).unwrap().as_path(),
        Path::new("/home/tester")
    );
    assert_eq!(
        normalize_rule_path("/already/virtual", &ctx)
            .unwrap()
            .as_path(),
        Path::new("/already/virtual")
    );

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn rejects_rule_paths_outside_source_root_or_without_home() {
    let root = test_dir("rule-normalization-reject");
    let source = root.join("source");
    let cwd = source.join("workspace");
    std::fs::create_dir_all(&cwd).unwrap();
    let outside_home = root.join("outside-home");
    std::fs::create_dir_all(&outside_home).unwrap();
    let ctx = RuleNormalizationContext::new(&source, &cwd, Some(outside_home)).unwrap();

    let err = normalize_rule_path("../../outside", &ctx).unwrap_err();
    assert!(err.contains("outside source_root"));
    let err = normalize_rule_path("~/.ssh", &ctx).unwrap_err();
    assert!(err.contains("outside source_root"));
    let err = normalize_rule_path("~user/.ssh", &ctx).unwrap_err();
    assert!(err.contains("unsupported home expansion"));

    let no_home = RuleNormalizationContext::new(&source, &cwd, None).unwrap();
    let err = normalize_rule_path("~/.ssh", &no_home).unwrap_err();
    assert!(err.contains("HOME is not set"));

    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn loads_environment_based_rule_context() {
    let root = test_dir("rule-normalization-env");
    let source = root.join("source");
    let cwd = source.join("docs/guides");
    let home = source.join("home/tester");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        let ctx = RuleNormalizationContext::from_environment(&source).unwrap();
        assert_eq!(
            normalize_rule_path("../secrets", &ctx).unwrap().as_path(),
            Path::new("/docs/secrets")
        );
        assert_eq!(
            normalize_rule_path("~/token", &ctx).unwrap().as_path(),
            Path::new("/home/tester/token")
        );
    }

    std::fs::remove_dir_all(root).unwrap();
}

fn test_dir(label: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("screenfs-path-{label}-{id}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
