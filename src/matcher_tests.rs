use super::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn matches_exact_rules_directory_prefixes_and_simple_globs() {
    let root = test_dir();
    let source = root.join("source");
    fs::create_dir_all(source.join("cwd")).unwrap();
    let context = test_context(&source, &source.join("cwd"), Some(source.join("home")));
    let matcher = PathRuleMatcher::new(
        [
            "/secret",
            "/config/auth.json",
            "**/.env",
            "**/.env.*",
            "**/*.pem",
            "**/*.key",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(matcher.matches_path(&VirtualPath::new("/secret")));
    assert!(matcher.matches_path(&VirtualPath::new("/secret/file")));
    assert!(!matcher.matches_path(&VirtualPath::new("/secretish")));

    assert!(matcher.matches_path(&VirtualPath::new("/config/auth.json")));
    assert!(!matcher.matches_path(&VirtualPath::new("/config/auth.json.bak")));

    assert!(matcher.matches_path(&VirtualPath::new("/app/.env")));
    assert!(matcher.matches_path(&VirtualPath::new("/app/.env/local")));
    assert!(matcher.matches_path(&VirtualPath::new("/app/.env.local")));
    assert!(matcher.matches_path(&VirtualPath::new("/app/.env.production/secrets")));
    assert!(!matcher.matches_path(&VirtualPath::new("/app/.environment")));
    assert!(matcher.matches_path(&VirtualPath::new("/certs/a.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/certs/a.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/keys/id.key")));
    assert!(matcher.matches_path(&VirtualPath::new("/keys/id.key/public")));
    assert!(!matcher.matches_path(&VirtualPath::new("/public/a.txt")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn dynamic_bridge_scan_roots_use_normalized_rule_anchors() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(&cwd).unwrap();
    fs::create_dir_all(&home).unwrap();
    let context = test_context(&source, &cwd, Some(home));

    let exact_only = PathRuleMatcher::new(["/etc"], Vec::new(), &context).unwrap();
    assert!(!exact_only.needs_dynamic_bridge_index());
    assert!(exact_only.dynamic_bridge_scan_roots().is_empty());

    let matcher = PathRuleMatcher::new(
        ["/etc/**/*.conf", "./fixtures/*.pem", "**/.git/hooks/**"],
        Vec::new(),
        &context,
    )
    .unwrap();
    assert!(matcher.needs_dynamic_bridge_index());
    assert_eq!(
        matcher.dynamic_bridge_scan_roots(),
        vec![
            VirtualPath::root(),
            VirtualPath::new("/etc"),
            VirtualPath::new("/workspace/app/fixtures"),
        ]
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn normalizes_relative_exact_and_prefixed_glob_rules() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(&cwd).unwrap();
    fs::create_dir_all(home.join("certs")).unwrap();
    let context = test_context(&source, &cwd, Some(home));
    let matcher = PathRuleMatcher::new(
        [
            "../secrets",
            "./fixtures/**/*.pem",
            "~/certs/**/*.lock",
            "/system/**/*.pem",
            "~/.env.*",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(matcher.matches_path(&VirtualPath::new("/workspace/secrets")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/key.pem")));
    assert!(matcher.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/nested/key.pem/chain"
    )));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/certs/app.lock")));
    assert!(matcher.matches_path(&VirtualPath::new("/system/keys/root.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/.env.local")));
    assert!(!matcher.matches_path(&VirtualPath::new("/home/tester/nested/.env.local")));
    assert!(!matcher.matches_path(&VirtualPath::new("/other/key.pem")));
    assert!(!matcher.matches_path(&VirtualPath::new("/system/keys/root.key")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/key.key")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn normalizes_direct_child_suffix_glob_rules() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    fs::create_dir_all(&home).unwrap();
    let context = test_context(&source, &cwd, Some(home));
    let matcher = PathRuleMatcher::new(
        ["~/*.pem", "./fixtures/*.pem", "/home/tester/*.pem"],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/user.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/user.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.pem/chain")));
    assert!(!matcher.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/nested/local.pem"
    )));
    assert!(!matcher.matches_path(&VirtualPath::new("/home/tester/nested/user.pem")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.key")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn matches_recursive_literal_descendant_subtree_globs() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    fs::create_dir_all(home.join("project/nested")).unwrap();
    let context = test_context(&source, &cwd, Some(home));

    let broad = PathRuleMatcher::new(["**/.git/**"], Vec::new(), &context).unwrap();
    assert!(broad.matches_path(&VirtualPath::new("/repo/.git")));
    assert!(broad.matches_path(&VirtualPath::new("/repo/.git/config")));
    assert!(!broad.matches_path(&VirtualPath::new("/repo/.gitignore")));

    let hooks = PathRuleMatcher::new(["**/.git/hooks/**"], Vec::new(), &context).unwrap();
    let prefixed = PathRuleMatcher::new(
        [
            "/home/tester/project/**/.git/hooks/**",
            "./fixtures/**/.git/hooks/**",
            "~/project/**/.git/hooks/**",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(hooks.matches_path(&VirtualPath::new("/repo/.git/hooks/pre-commit")));
    assert!(!hooks.matches_path(&VirtualPath::new("/repo/.git/x/hooks/pre-commit")));
    assert!(prefixed.matches_path(&VirtualPath::new(
        "/home/tester/project/.git/hooks/pre-commit"
    )));
    assert!(prefixed.matches_path(&VirtualPath::new(
        "/home/tester/project/nested/.git/hooks/pre-commit"
    )));
    assert!(!prefixed.matches_path(&VirtualPath::new(
        "/home/tester/other/.git/hooks/pre-commit"
    )));
    assert!(prefixed.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/.git/hooks/pre-commit"
    )));
    assert!(prefixed.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/nested/.git/hooks/pre-commit"
    )));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn recursive_literal_descendant_subtree_rules_preserve_specificity_and_containment() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(source.join("workspace/app")).unwrap();
    let context = test_context(&source, &cwd, Some(source.join("home/tester")));

    let broad = PathRuleMatcher::new(["**/.git/**"], Vec::new(), &context).unwrap();
    let narrow = PathRuleMatcher::new(["**/.git/hooks/**"], Vec::new(), &context).unwrap();
    let anchored =
        PathRuleMatcher::new(["./fixtures/**/.git/hooks/**"], Vec::new(), &context).unwrap();
    let hooks = PathRuleMatcher::new(["**/hooks"], Vec::new(), &context).unwrap();

    let broad_desc = &broad.descriptors()[0];
    let narrow_desc = &narrow.descriptors()[0];
    let anchored_desc = &anchored.descriptors()[0];
    let hooks_desc = &hooks.descriptors()[0];

    assert!(broad_desc.has_less_specific_ancestor_of(narrow_desc));
    assert!(hooks_desc.has_less_specific_ancestor_of(narrow_desc));
    assert!(narrow_desc.has_less_specific_ancestor_of(anchored_desc));

    let sample = VirtualPath::new("/workspace/app/fixtures/nested/.git/hooks/pre-commit");
    assert!(broad.best_specificity(&sample).unwrap() < narrow.best_specificity(&sample).unwrap());
    assert!(
        narrow.best_specificity(&sample).unwrap() < anchored.best_specificity(&sample).unwrap()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn recursive_literal_descendant_subtree_specificity_prefers_longer_tail_over_longer_prefix() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    let broader_prefix =
        PathRuleMatcher::new(["/workspace/**/.git/**"], Vec::new(), &context).unwrap();
    let longer_tail = PathRuleMatcher::new(["**/.git/hooks/**"], Vec::new(), &context).unwrap();
    let sample = VirtualPath::new("/workspace/repo/.git/hooks/pre-commit");

    assert!(broader_prefix.matches_path(&sample));
    assert!(longer_tail.matches_path(&sample));
    assert!(
        broader_prefix.best_specificity(&sample).unwrap()
            < longer_tail.best_specificity(&sample).unwrap()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn matches_requested_absolute_descendant_subtree_glob_pattern() {
    let context = test_context(Path::new("/"), Path::new("/"), None);
    let matcher = PathRuleMatcher::new(
        ["/home/spi-ca/Codebase/the-onion/palgong/**/.git/hooks/**"],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(matcher.matches_path(&VirtualPath::new(
        "/home/spi-ca/Codebase/the-onion/palgong/repo/.git/hooks/pre-commit"
    )));
    assert!(!matcher.matches_path(&VirtualPath::new(
        "/home/spi-ca/Codebase/the-onion/other/.git/hooks/pre-commit"
    )));
}

#[test]
fn rejects_unsupported_globs_and_paths_outside_source_root() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    assert!(PathRuleMatcher::new(["../../secret"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/secret?.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["*.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["foo*"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["/pre*fix/*.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["foo/*/bar.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["~user/.ssh"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/.git/**/hooks/**"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/*/.git/**"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/./hooks/**"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["foo/**"], Vec::new(), &context).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn hides_mount_root_subtree_as_internal_prefix() {
    let tmp = test_dir();
    let source = tmp.as_path();
    let mount = source.join("mnt");
    fs::create_dir(&mount).unwrap();
    let prefix = mount_root_internal_prefix(source, &mount).unwrap();
    let context = test_context(source, source, None);
    let matcher = PathRuleMatcher::new(std::iter::empty::<&str>(), vec![prefix], &context).unwrap();
    assert!(matcher.matches_path(&VirtualPath::new("/mnt")));
    assert!(matcher.matches_path(&VirtualPath::new("/mnt/child")));
    assert!(!matcher.matches_path(&VirtualPath::new("/other")));
    fs::remove_dir_all(tmp).unwrap();
}

#[test]
fn computes_mount_root_internal_prefix_before_mount_exists() {
    let tmp = test_dir();
    let source = tmp.as_path();
    let mount = source.join("nested/../mnt/child");
    let prefix = mount_root_internal_prefix(source, &mount).unwrap();
    assert_eq!(prefix.as_path(), Path::new("/mnt/child"));
    fs::remove_dir_all(tmp).unwrap();
}

#[test]
fn matches_symlink_target_without_caching_target_decision() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("cwd");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);
    let matcher = PathRuleMatcher::new(["/hidden", "**/*.pem"], Vec::new(), &context).unwrap();
    let exact_link = VirtualPath::new("/visible/link");
    let glob_link = VirtualPath::new("/visible/nested/link");

    assert!(matcher.matches_symlink_target(&exact_link, std::ffi::OsStr::new("../hidden")));
    assert!(
        matcher
            .matches_symlink_target(&glob_link, std::ffi::OsStr::new("../../secret.pem/private"))
    );
    fs::remove_dir_all(root).unwrap();
}

fn test_context(
    source_root: &Path,
    current_dir: &Path,
    home_dir: Option<PathBuf>,
) -> RuleNormalizationContext {
    RuleNormalizationContext::new(source_root, current_dir, home_dir).unwrap()
}

fn test_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    dir.push(format!("screenfs-test-{id}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}
