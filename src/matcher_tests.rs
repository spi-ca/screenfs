//! Regression tests for path-rule matcher grammar, specificity, and indexing.

use super::*;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

// Test cases are grouped by the behavior named in each function.
#[test]
fn matches_exact_rules_directory_prefixes_and_cwd_anchored_recursive_globs() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, Some(source.join("home")));
    let matcher = PathRuleMatcher::new(
        [
            "/secret",
            "/config/auth.json",
            "**/.env",
            "**/.env.*",
            "**/*.pem",
            "**/*.key",
            "/**/*.crt",
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

    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/.env")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/.env/local")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/.env.local")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/.env.production/secrets")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/.environment")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/certs/a.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/certs/a.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/keys/id.key")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/keys/id.key/public")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/peer/.env")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/peer/certs/a.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/peer/certs/a.crt")));
    assert!(!matcher.matches_path(&VirtualPath::new("/public/a.txt")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn matcher_descendant_query_can_exclude_same_anchor_subtree() {
    let root = test_dir();
    let source = root.join("source");
    fs::create_dir_all(&source).unwrap();
    let context = test_context(&source, &source, None);

    let matcher = PathRuleMatcher::new(
        [
            "/workspace/secret",
            "/workspace/secret/project",
            "/workspace/certs/*.pem",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    let same_anchor_only = PathRuleMatcher::new(["/same-anchor"], Vec::new(), &context).unwrap();
    assert!(
        !same_anchor_only
            .may_match_descendant_of_other_than_same_subtree(&VirtualPath::new("/same-anchor"))
    );
    assert!(
        !matcher
            .may_match_descendant_of_other_than_same_subtree(&VirtualPath::new("/workspace/other"))
    );
    assert!(
        matcher.may_match_descendant_of_other_than_same_subtree(&VirtualPath::new(
            "/workspace/secret"
        ))
    );
    assert!(
        matcher
            .may_match_descendant_of_other_than_same_subtree(&VirtualPath::new("/workspace/certs"))
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn matcher_indexes_candidates_by_family_and_normalized_anchor() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    let matcher = PathRuleMatcher::new(
        [
            "/etc",
            "/etc/**",
            "./fixtures/**",
            "/a/*.txt",
            "/b/*.pem",
            "/a/**/*.lock",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert_eq!(matcher.descriptors().len(), 5);
    assert!(matcher.matches_path(&VirtualPath::new("/a/file.txt")));
    assert!(!matcher.matches_path(&VirtualPath::new("/b/file.txt")));
    assert!(matcher.matches_path(&VirtualPath::new("/a/nested/file.lock")));

    let a_txt_path = VirtualPath::new("/a/file.txt");
    let a_txt_candidates = matcher.candidate_descriptor_count(&a_txt_path);
    let b_txt_candidates = matcher.candidate_descriptor_count(&VirtualPath::new("/b/file.txt"));
    assert!(a_txt_candidates < matcher.descriptors().len());
    assert!(b_txt_candidates < matcher.descriptors().len());
    assert_eq!(a_txt_candidates, b_txt_candidates);

    let a_txt_metrics = matcher.candidate_descriptor_metrics(&a_txt_path);
    assert_eq!(a_txt_metrics.count, a_txt_candidates);
    assert_eq!(a_txt_metrics.family_counts.direct_child_glob, 1);
    assert_eq!(a_txt_metrics.family_counts.recursive, 1);
    assert!(a_txt_metrics.candidate_order.ancestor_steps > 0);
    assert_eq!(
        a_txt_metrics.candidate_order.seen_slots,
        matcher.descriptors().len()
    );

    let a_descendant_candidates =
        matcher.descendant_candidate_descriptor_count(&VirtualPath::new("/a"));
    let unrelated_descendant_candidates =
        matcher.descendant_candidate_descriptor_count(&VirtualPath::new("/unrelated"));
    let a_descendant_metrics =
        matcher.descendant_candidate_descriptor_metrics(&VirtualPath::new("/a"));
    assert_eq!(a_descendant_metrics.count, a_descendant_candidates);
    assert!(a_descendant_metrics.family_counts.direct_child_glob > 0);
    assert!(a_descendant_metrics.family_counts.recursive > 0);
    assert!(a_descendant_metrics.candidate_order.duplicates_skipped > 0);
    assert!(a_descendant_candidates < matcher.descriptors().len());
    assert!(unrelated_descendant_candidates < a_descendant_candidates);
    assert!(matcher.may_match_descendant_of(&VirtualPath::new("/a")));
    assert!(!matcher.may_match_descendant_of(&VirtualPath::new("/unrelated")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn descendant_candidate_descriptor_visit_streams_parent_frontier_subtrees() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    let matcher = PathRuleMatcher::new(
        [
            "/workspace/project",
            "/workspace/project/docs",
            "/workspace/other",
            "/outside",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    let parent = VirtualPath::new("/workspace");
    let mut visited = Vec::new();
    matcher.visit_descendant_candidate_descriptors(&parent, |descriptor| {
        visited.push(descriptor.anchor().as_path().display().to_string());
    });
    visited.sort();

    assert_eq!(
        matcher.descendant_candidate_descriptor_count(&parent),
        visited.len()
    );
    assert_eq!(
        visited,
        vec![
            "/workspace/other".to_string(),
            "/workspace/project".to_string(),
            "/workspace/project/docs".to_string(),
        ]
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn canonical_glob_families_preserve_match_ranges_and_specificity() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    fs::create_dir_all(source.join("workspace/app/nested")).unwrap();
    fs::create_dir_all(source.join("workspace/peer")).unwrap();
    fs::create_dir_all(source.join("a/nested")).unwrap();
    let context = test_context(&source, &cwd, None);

    let recursive_pem = PathRuleMatcher::new(["**/*.pem"], Vec::new(), &context).unwrap();
    let root_recursive_pem = PathRuleMatcher::new(["/**/*.pem"], Vec::new(), &context).unwrap();
    let fixture_recursive_pem =
        PathRuleMatcher::new(["./fixtures/**/*.pem"], Vec::new(), &context).unwrap();
    let direct_pem = PathRuleMatcher::new(["*.pem"], Vec::new(), &context).unwrap();
    let direct_txt = PathRuleMatcher::new(["/a/*.txt"], Vec::new(), &context).unwrap();
    let recursive_txt = PathRuleMatcher::new(["/a/**/*.txt"], Vec::new(), &context).unwrap();

    let cwd_direct = VirtualPath::new("/workspace/app/local.pem");
    let cwd_direct_descendant = VirtualPath::new("/workspace/app/local.pem/chain");
    let cwd_nested = VirtualPath::new("/workspace/app/nested/local.pem");
    let cwd_nested_descendant = VirtualPath::new("/workspace/app/nested/local.pem/chain");
    let fixture_direct = VirtualPath::new("/workspace/app/fixtures/local.pem");
    let fixture_nested = VirtualPath::new("/workspace/app/fixtures/nested/local.pem");
    let outside_cwd = VirtualPath::new("/workspace/peer/local.pem");

    assert!(recursive_pem.matches_path(&cwd_direct));
    assert!(recursive_pem.matches_path(&cwd_direct_descendant));
    assert!(recursive_pem.matches_path(&cwd_nested));
    assert!(recursive_pem.matches_path(&cwd_nested_descendant));
    assert!(recursive_pem.matches_path(&fixture_direct));
    assert!(recursive_pem.matches_path(&fixture_nested));
    assert!(!recursive_pem.matches_path(&outside_cwd));
    assert!(root_recursive_pem.matches_path(&outside_cwd));

    assert!(fixture_recursive_pem.matches_path(&fixture_direct));
    assert!(fixture_recursive_pem.matches_path(&fixture_nested));
    assert!(!fixture_recursive_pem.matches_path(&cwd_direct));
    assert!(
        recursive_pem.descriptors()[0]
            .has_less_specific_ancestor_of(&fixture_recursive_pem.descriptors()[0])
    );
    assert!(
        recursive_pem.best_specificity(&fixture_direct).unwrap()
            < fixture_recursive_pem
                .best_specificity(&fixture_direct)
                .unwrap()
    );

    assert!(direct_pem.matches_path(&cwd_direct));
    assert!(direct_pem.matches_path(&cwd_direct_descendant));
    assert!(!direct_pem.matches_path(&cwd_nested));
    assert!(!direct_pem.matches_path(&fixture_direct));
    assert!(
        recursive_pem.descriptors()[0].has_less_specific_ancestor_of(&direct_pem.descriptors()[0])
    );
    assert!(
        recursive_pem.best_specificity(&cwd_direct).unwrap()
            < direct_pem.best_specificity(&cwd_direct).unwrap()
    );

    let direct_txt_path = VirtualPath::new("/a/file.txt");
    let direct_txt_descendant = VirtualPath::new("/a/file.txt/child");
    let nested_txt_path = VirtualPath::new("/a/nested/file.txt");
    let nested_txt_descendant = VirtualPath::new("/a/nested/file.txt/child");
    assert!(direct_txt.matches_path(&direct_txt_path));
    assert!(direct_txt.matches_path(&direct_txt_descendant));
    assert!(!direct_txt.matches_path(&nested_txt_path));
    assert!(!direct_txt.matches_path(&nested_txt_descendant));
    assert!(recursive_txt.matches_path(&direct_txt_path));
    assert!(recursive_txt.matches_path(&direct_txt_descendant));
    assert!(recursive_txt.matches_path(&nested_txt_path));
    assert!(recursive_txt.matches_path(&nested_txt_descendant));
    assert!(
        recursive_txt.descriptors()[0].has_less_specific_ancestor_of(&direct_txt.descriptors()[0])
    );
    assert!(
        recursive_txt.best_specificity(&direct_txt_path).unwrap()
            < direct_txt.best_specificity(&direct_txt_path).unwrap()
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
fn normalizes_direct_child_basename_prefix_and_suffix_glob_rules() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    fs::create_dir_all(&home).unwrap();
    let context = test_context(&source, &cwd, Some(home));
    let matcher = PathRuleMatcher::new(
        [
            "~/*.pem",
            "~/.env.*",
            "./fixtures/*.pem",
            "./fixtures/id_*",
            "/home/tester/*.pem",
            "/home/tester/id_*",
        ],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/user.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/user.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/.env.local")));
    assert!(matcher.matches_path(&VirtualPath::new("/home/tester/id_ed25519")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/id_rsa")));
    assert!(!matcher.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/nested/local.pem"
    )));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/nested/id_rsa")));
    assert!(!matcher.matches_path(&VirtualPath::new("/home/tester/nested/.env.local")));
    assert!(!matcher.matches_path(&VirtualPath::new("/home/tester/nested/id_ed25519")));
    assert!(!matcher.matches_path(&VirtualPath::new("/home/tester/nested/user.pem")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.key")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn bare_basename_glob_shorthand_matches_current_dir_immediate_children_and_descendants() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, Some(source.join("home")));
    let matcher =
        PathRuleMatcher::new(["*.pem", "*.key", ".env.*", "id_*"], Vec::new(), &context).unwrap();

    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/cert.pem")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/cert.pem/chain")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/id.key/public")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/.env.local/secrets")));
    assert!(matcher.matches_path(&VirtualPath::new("/workspace/app/id_ed25519/private")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/nested/cert.pem")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/nested/id.key/public")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/nested/.env.local")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/nested/id_ed25519")));
    assert!(!matcher.matches_path(&VirtualPath::new("/workspace/app/identity")));
    assert!(!matcher.matches_path(&VirtualPath::new("/notes/pem.txt")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn bare_basename_glob_shorthand_compiles_equivalent_to_current_dir_direct_child_form() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    for (shorthand, direct, recursive) in [
        ("*.pem", "./*.pem", "**/*.pem"),
        (".env.*", "./.env.*", "**/.env.*"),
        ("id_*", "./id_*", "**/id_*"),
    ] {
        let shorthand = PathRuleMatcher::new([shorthand], Vec::new(), &context).unwrap();
        let direct = PathRuleMatcher::new([direct], Vec::new(), &context).unwrap();
        let recursive = PathRuleMatcher::new([recursive], Vec::new(), &context).unwrap();
        assert_eq!(shorthand.descriptors(), direct.descriptors());
        assert_ne!(shorthand.descriptors(), recursive.descriptors());
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn prefixless_recursive_glob_shorthand_compiles_equivalent_to_current_dir_recursive_form() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    for (shorthand, recursive) in [
        ("**/*.pem", "./**/*.pem"),
        ("**/.env.*", "./**/.env.*"),
        ("**/id_*", "./**/id_*"),
    ] {
        let shorthand = PathRuleMatcher::new([shorthand], Vec::new(), &context).unwrap();
        let recursive = PathRuleMatcher::new([recursive], Vec::new(), &context).unwrap();
        assert_eq!(shorthand.descriptors(), recursive.descriptors());
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn anchored_wildcard_all_and_trailing_subtree_shorthand_preserve_normalized_semantics() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    fs::create_dir_all(home.join("sandbox/bin")).unwrap();
    let context = test_context(&source, &cwd, Some(home));

    for (shorthand, subtree) in [
        ("/vault/**", "/vault"),
        ("./fixtures/**", "./fixtures"),
        ("~/sandbox/**", "~/sandbox"),
        ("~/aa/**", "~/aa"),
    ] {
        let shorthand = PathRuleMatcher::new([shorthand], Vec::new(), &context).unwrap();
        let subtree = PathRuleMatcher::new([subtree], Vec::new(), &context).unwrap();
        assert_eq!(shorthand.descriptors(), subtree.descriptors());
    }

    let wildcard_all = PathRuleMatcher::new(
        ["/vault/*", "./fixtures/*", "~/sandbox/*"],
        Vec::new(),
        &context,
    )
    .unwrap();

    assert!(!wildcard_all.matches_path(&VirtualPath::new("/vault")));
    assert!(wildcard_all.matches_path(&VirtualPath::new("/vault/child")));
    assert!(wildcard_all.matches_path(&VirtualPath::new("/vault/child/grand")));
    assert!(!wildcard_all.matches_path(&VirtualPath::new("/other/child")));

    assert!(!wildcard_all.matches_path(&VirtualPath::new("/workspace/app/fixtures")));
    assert!(wildcard_all.matches_path(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
    assert!(wildcard_all.matches_path(&VirtualPath::new(
        "/workspace/app/fixtures/nested/local.pem"
    )));
    assert!(!wildcard_all.matches_path(&VirtualPath::new("/workspace/app/other/local.pem")));

    assert!(!wildcard_all.matches_path(&VirtualPath::new("/home/tester/sandbox")));
    assert!(wildcard_all.matches_path(&VirtualPath::new("/home/tester/sandbox/bin")));
    assert!(wildcard_all.matches_path(&VirtualPath::new("/home/tester/sandbox/bin/tool")));
    assert!(!wildcard_all.matches_path(&VirtualPath::new("/home/other/sandbox/bin")));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn anchored_wildcard_all_preserves_direct_child_containment_and_conservative_overlap_checks() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    fs::create_dir_all(&cwd).unwrap();
    let context = test_context(&source, &cwd, None);

    let subtree = PathRuleMatcher::new(["/a"], Vec::new(), &context).unwrap();
    let wildcard_all = PathRuleMatcher::new(["/a/*"], Vec::new(), &context).unwrap();
    let direct = PathRuleMatcher::new(["/a/*.txt"], Vec::new(), &context).unwrap();
    let recursive = PathRuleMatcher::new(["/a/**/*.txt"], Vec::new(), &context).unwrap();

    let subtree_desc = &subtree.descriptors()[0];
    let wildcard_all_desc = &wildcard_all.descriptors()[0];
    let direct_desc = &direct.descriptors()[0];
    let recursive_desc = &recursive.descriptors()[0];

    assert!(wildcard_all_desc.contains_target_set(direct_desc));
    assert!(!direct_desc.contains_target_set(wildcard_all_desc));
    assert!(subtree_desc.contains_target_set(wildcard_all_desc));
    assert!(!wildcard_all_desc.contains_target_set(subtree_desc));
    assert!(wildcard_all_desc.has_less_specific_ancestor_of(direct_desc));
    assert!(wildcard_all_desc.has_unproven_overlap_with(recursive_desc));
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
fn recursive_literal_directory_shorthand_matches_canonical_descendant_subtree_forms() {
    let root = test_dir();
    let source = root.join("source");
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    fs::create_dir_all(source.join("workspace/app/repo/nested")).unwrap();
    fs::create_dir_all(home.join("repo/nested")).unwrap();
    let context = test_context(&source, &cwd, Some(home));

    for (shorthand, canonical, matching_path, non_matching_path) in [
        (
            "**/.git/hooks",
            "**/.git/hooks/**",
            "/workspace/app/repo/.git/hooks/pre-commit",
            "/workspace/app/repo/.git/x/hooks/pre-commit",
        ),
        (
            "/repo/**/.git/hooks",
            "/repo/**/.git/hooks/**",
            "/repo/nested/.git/hooks/pre-commit",
            "/other/nested/.git/hooks/pre-commit",
        ),
        (
            "./repo/**/.git/hooks",
            "./repo/**/.git/hooks/**",
            "/workspace/app/repo/nested/.git/hooks/pre-commit",
            "/workspace/app/other/.git/hooks/pre-commit",
        ),
        (
            "~/repo/**/.git/hooks",
            "~/repo/**/.git/hooks/**",
            "/home/tester/repo/nested/.git/hooks/pre-commit",
            "/home/tester/other/.git/hooks/pre-commit",
        ),
        (
            "**/.git",
            "**/.git/**",
            "/workspace/app/repo/.git/config",
            "/workspace/app/repo/.gitignore",
        ),
        (
            "**/node_modules",
            "**/node_modules/**",
            "/workspace/app/repo/node_modules/pkg/index.js",
            "/workspace/app/repo/node_module/pkg/index.js",
        ),
        (
            "~/**/aaa/hook",
            "~/**/aaa/hook/**",
            "/home/tester/repo/nested/aaa/hook/pre-commit",
            "/workspace/app/repo/nested/aaa/hook/pre-commit",
        ),
    ] {
        let shorthand =
            PathRuleMatcher::compile(MatcherScope::Hidden, [shorthand], Vec::new(), &context)
                .unwrap();
        let canonical =
            PathRuleMatcher::compile(MatcherScope::Hidden, [canonical], Vec::new(), &context)
                .unwrap();
        assert_eq!(
            shorthand.descriptors(),
            canonical.descriptors(),
            "{shorthand:?}"
        );
        assert_eq!(
            shorthand.matches_path(&VirtualPath::new(matching_path)),
            canonical.matches_path(&VirtualPath::new(matching_path)),
            "{matching_path}"
        );
        assert!(
            shorthand.matches_path(&VirtualPath::new(matching_path)),
            "{matching_path}"
        );
        assert_eq!(
            shorthand.matches_path(&VirtualPath::new(non_matching_path)),
            canonical.matches_path(&VirtualPath::new(non_matching_path)),
            "{non_matching_path}"
        );
        assert!(
            !shorthand.matches_path(&VirtualPath::new(non_matching_path)),
            "{non_matching_path}"
        );
    }

    let dedup = PathRuleMatcher::compile(
        MatcherScope::Hidden,
        ["**/.git/hooks", "**/.git/hooks/**"],
        Vec::new(),
        &context,
    )
    .unwrap();
    assert_eq!(dedup.descriptors().len(), 1);
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
    let root_hooks = PathRuleMatcher::new(["/**/hooks"], Vec::new(), &context).unwrap();

    let broad_desc = &broad.descriptors()[0];
    let narrow_desc = &narrow.descriptors()[0];
    let anchored_desc = &anchored.descriptors()[0];
    let hooks_desc = &hooks.descriptors()[0];
    let root_hooks_desc = &root_hooks.descriptors()[0];

    assert!(broad_desc.has_less_specific_ancestor_of(narrow_desc));
    assert!(!hooks_desc.has_less_specific_ancestor_of(narrow_desc));
    assert!(root_hooks_desc.has_less_specific_ancestor_of(narrow_desc));
    assert!(narrow_desc.has_less_specific_ancestor_of(anchored_desc));

    let sample = VirtualPath::new("/workspace/app/fixtures/nested/.git/hooks/pre-commit");
    let outside_cwd_sample = VirtualPath::new("/workspace/peer/.git/hooks/pre-commit");
    assert!(hooks.matches_path(&sample));
    assert!(!hooks.matches_path(&outside_cwd_sample));
    assert!(root_hooks.matches_path(&outside_cwd_sample));
    assert!(narrow.matches_path(&outside_cwd_sample));
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
fn prefixless_recursive_glob_fails_fast_when_current_dir_is_outside_source_root() {
    let root = test_dir();
    let source = root.join("source");
    let outside = root.join("outside");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&outside).unwrap();
    let context = test_context(&source, &outside, None);

    let err = PathRuleMatcher::new(["**/*.pem"], Vec::new(), &context).unwrap_err();
    assert!(
        err.contains("rule path resolves outside source_root: ."),
        "{err}"
    );
    fs::remove_dir_all(root).unwrap();
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
    assert!(PathRuleMatcher::new(["**/foo?"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/[abc]"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["*"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/*"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["a*b"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["*secret*"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["/pre*fix/*.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["foo/*/bar.pem"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["~user/.ssh"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/.git/*/hooks"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/.git/**/hooks"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["**/.git/**/hooks/**"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["~/**/bbb/**/ccc"], Vec::new(), &context).is_err());
    assert!(PathRuleMatcher::new(["~/**/bbb/**/ccc/**"], Vec::new(), &context).is_err());
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
    let matcher = PathRuleMatcher::new(["/hidden", "*.pem"], Vec::new(), &context).unwrap();
    let exact_link = VirtualPath::new("/visible/link");
    let glob_link = VirtualPath::new("/cwd/nested/link");

    assert!(matcher.matches_symlink_target(&exact_link, std::ffi::OsStr::new("../hidden")));
    assert!(
        matcher.matches_symlink_target(&glob_link, std::ffi::OsStr::new("../secret.pem/private"))
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
