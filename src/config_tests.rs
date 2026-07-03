//! Regression tests for config loading, rule conflict handling, and policy decisions.

use super::*;
use crate::path::ProcessEnvGuard;
use std::time::{SystemTime, UNIX_EPOCH};

// Helper builds a full RuntimeConfig while letting each test vary one policy axis.
fn launch(
    source: &Path,
    visibility_default: Option<VisibilityDefault>,
    hidden: Vec<String>,
    visible: Vec<String>,
    mutability_default: Option<MutabilityDefault>,
    readonly: Vec<String>,
    writable: Vec<String>,
) -> RuntimeConfig {
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let _env = ProcessEnvGuard::new(source, None);
    RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.to_path_buf(),
            mount_root: mount,
            visibility_hidden_rules: hidden,
            visibility_visible_rules: visible,
            mutability_readonly_rules: readonly,
            mutability_writable_rules: writable,
        },
        config_path: None,
        visibility_default,
        mutability_default,
        experimental_writeback_cache: false,
    })
    .unwrap()
}

fn launch_from_args(args: LaunchArgs) -> Result<RuntimeConfig, String> {
    let _env = ProcessEnvGuard::new(&args.cli.source_root, None);
    RuntimeConfig::from_launch(args)
}

// Test cases are grouped by the behavior named in each function.
#[test]
fn defaults_to_visible_and_writable_without_policy() {
    let source = test_dir();
    let cfg = launch(&source, None, vec![], vec![], None, vec![], vec![]);
    assert_eq!(cfg.visibility_default(), VisibilityDefault::Visible);
    assert_eq!(cfg.mutability_default(), MutabilityDefault::Writable);
    assert_eq!(cfg.visibility_source(), PolicySource::Default);
    assert_eq!(cfg.mutability_source(), PolicySource::Default);
    assert!(cfg.is_visible(&VirtualPath::new("/anything")));
    assert!(!cfg.is_readonly(&VirtualPath::new("/anything")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn directory_child_visibility_batch_fast_path_requires_no_hidden_rules() {
    let root = test_dir();
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&mount).unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source,
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: Some(VisibilityDefault::Visible),
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    let batch = cfg.directory_child_visibility_batch(&VirtualPath::root());
    assert!(!batch.uses_per_entry_visibility());
    for (path, is_dir) in [
        (VirtualPath::new("/file.txt"), false),
        (VirtualPath::new("/dir"), true),
    ] {
        assert_eq!(
            batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?}"
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn directory_child_visibility_batch_parent_scopes_hidden_subtree_frontier() {
    let root = test_dir();
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&mount).unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source,
            mount_root: mount,
            visibility_hidden_rules: vec![
                "/workspace/secret".to_string(),
                "/workspace/nested/secret".to_string(),
                "/workspace/certs".to_string(),
                "/other/hidden".to_string(),
            ],
            visibility_visible_rules: vec![
                "/workspace/secret/project".to_string(),
                "/workspace/certs/*.pem".to_string(),
            ],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: Some(VisibilityDefault::Visible),
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    let workspace_batch = cfg.directory_child_visibility_batch(&VirtualPath::new("/workspace"));
    assert!(!workspace_batch.uses_per_entry_visibility());
    assert!(!workspace_batch.can_assume_all_visible());
    for (path, is_dir) in [
        (VirtualPath::new("/workspace/secret"), true),
        (VirtualPath::new("/workspace/secret"), false),
        (VirtualPath::new("/workspace/certs"), true),
        (VirtualPath::new("/workspace/certs"), false),
        (VirtualPath::new("/workspace/nested"), true),
        (VirtualPath::new("/workspace/file.txt"), false),
    ] {
        assert_eq!(
            workspace_batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?}"
        );
    }

    let dir_entries_batch =
        cfg.directory_child_visibility_batch(&VirtualPath::new("/.screenfs-bench/dir-entries"));
    assert!(!dir_entries_batch.uses_per_entry_visibility());
    assert!(dir_entries_batch.can_assume_all_visible());
    assert_eq!(
        dir_entries_batch.entry_is_readable(
            &VirtualPath::new("/.screenfs-bench/dir-entries/file.txt"),
            false
        ),
        cfg.entry_is_readable(
            &VirtualPath::new("/.screenfs-bench/dir-entries/file.txt"),
            false
        )
    );
    assert_eq!(
        workspace_batch.entry_is_readable(&VirtualPath::new("/other/secret"), true),
        cfg.entry_is_readable(&VirtualPath::new("/other/secret"), true)
    );
    assert_eq!(
        workspace_batch.entry_is_readable(&VirtualPath::new("/workspace/secret/project"), true),
        cfg.entry_is_readable(&VirtualPath::new("/workspace/secret/project"), true)
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn directory_child_visibility_batch_parent_scopes_visible_subtree_frontier() {
    let root = test_dir();
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&mount).unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source,
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec!["/workspace/project".to_string()],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: Some(VisibilityDefault::Hidden),
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    let root_batch = cfg.directory_child_visibility_batch(&VirtualPath::root());
    assert!(!root_batch.uses_per_entry_visibility());
    assert_eq!(
        root_batch.entry_is_readable(&VirtualPath::new("/workspace"), true),
        cfg.entry_is_readable(&VirtualPath::new("/workspace"), true)
    );
    assert_eq!(
        root_batch.entry_is_readable(&VirtualPath::new("/workspace"), false),
        cfg.entry_is_readable(&VirtualPath::new("/workspace"), false)
    );
    assert_eq!(
        root_batch.entry_is_readable(&VirtualPath::new("/other"), true),
        cfg.entry_is_readable(&VirtualPath::new("/other"), true)
    );

    let workspace_batch = cfg.directory_child_visibility_batch(&VirtualPath::new("/workspace"));
    assert!(!workspace_batch.uses_per_entry_visibility());
    for (path, is_dir) in [
        (VirtualPath::new("/workspace/project"), false),
        (VirtualPath::new("/workspace/other"), true),
    ] {
        assert_eq!(
            workspace_batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?}"
        );
    }

    let project_batch =
        cfg.directory_child_visibility_batch(&VirtualPath::new("/workspace/project"));
    assert!(project_batch.can_assume_all_visible());
    assert_eq!(
        project_batch.entry_is_readable(&VirtualPath::new("/workspace/project/file.txt"), false),
        cfg.entry_is_readable(&VirtualPath::new("/workspace/project/file.txt"), false)
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn directory_child_visibility_batch_parent_scope_matches_per_entry_for_indexed_subtrees() {
    let root = test_dir();
    let source = root.join("source");
    let mount = root.join("mount");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&mount).unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source,
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![
                "/workspace/project".to_string(),
                "/workspace/shared/assets".to_string(),
                "/workspace/tools".to_string(),
                "/other/place".to_string(),
            ],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: Some(VisibilityDefault::Hidden),
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    let workspace_batch = cfg.directory_child_visibility_batch(&VirtualPath::new("/workspace"));
    assert!(!workspace_batch.uses_per_entry_visibility());
    for (path, is_dir) in [
        (VirtualPath::new("/workspace/project"), true),
        (VirtualPath::new("/workspace/shared"), true),
        (VirtualPath::new("/workspace/shared"), false),
        (VirtualPath::new("/workspace/tools"), false),
        (VirtualPath::new("/workspace/other"), true),
    ] {
        assert_eq!(
            workspace_batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?} dir={is_dir}"
        );
    }

    let shared_batch = cfg.directory_child_visibility_batch(&VirtualPath::new("/workspace/shared"));
    assert!(!shared_batch.uses_per_entry_visibility());
    for (path, is_dir) in [
        (VirtualPath::new("/workspace/shared/assets"), true),
        (VirtualPath::new("/workspace/shared/other"), true),
    ] {
        assert_eq!(
            shared_batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?}"
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn directory_child_visibility_batch_falls_back_for_rule_sensitive_shapes() {
    let source = test_dir();
    let cfg = launch(
        &source,
        Some(VisibilityDefault::Hidden),
        vec!["/home/secret".to_string()],
        vec!["/home/me/project".to_string(), "/home/*.pem".to_string()],
        None,
        vec![],
        vec![],
    );
    let batch = cfg.directory_child_visibility_batch(&VirtualPath::new("/home"));
    assert!(batch.uses_per_entry_visibility());
    for (path, is_dir) in [
        (VirtualPath::new("/home/me"), true),
        (VirtualPath::new("/home/other"), true),
        (VirtualPath::new("/home/local.pem"), false),
        (VirtualPath::new("/home/secret"), true),
    ] {
        assert_eq!(
            batch.entry_is_readable(&path, is_dir),
            cfg.entry_is_readable(&path, is_dir),
            "{path:?}"
        );
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn includes_mount_root_internal_hidden_rule() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    {
        let _env = ProcessEnvGuard::new(&source, None);
        let cfg = RuntimeConfig::from_cli(CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        })
        .unwrap();
        assert!(cfg.is_hidden(&VirtualPath::new("/mnt")));
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn mount_root_internal_hidden_rule_cannot_be_made_visible() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(mount.join("public")).unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec!["/mnt".to_string(), "/mnt/pub*".to_string()],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: Some(VisibilityDefault::Hidden),
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/mnt")),
        VisibilityDecision::Hidden
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/mnt/public")),
        VisibilityDecision::Hidden
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn visibility_hidden_and_visible_use_most_specific_match_with_bridge() {
    let source = test_dir();
    let cfg = launch(
        &source,
        None,
        vec!["/home".to_string()],
        vec!["/home/me/project".to_string()],
        None,
        vec![],
        vec![],
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/home")),
        VisibilityDecision::BridgeVisible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/home/me")),
        VisibilityDecision::BridgeVisible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/home/me/project")),
        VisibilityDecision::Visible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/home/other")),
        VisibilityDecision::Hidden
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn default_hidden_supports_visible_carve_outs() {
    let source = test_dir();
    let cfg = launch(
        &source,
        Some(VisibilityDefault::Hidden),
        vec![],
        vec!["/workspace".to_string()],
        None,
        vec![],
        vec![],
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/")),
        VisibilityDecision::BridgeVisible
    );
    assert!(cfg.is_fully_visible(&VirtualPath::new("/workspace/file")));
    assert!(cfg.is_hidden(&VirtualPath::new("/other")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn mutability_readonly_and_writable_use_most_specific_match() {
    let source = test_dir();
    let cfg = launch(
        &source,
        None,
        vec![],
        vec![],
        Some(MutabilityDefault::Readonly),
        vec!["/workspace/**/.git/hooks/**".to_string()],
        vec!["/workspace".to_string()],
    );
    assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/src/lib.rs")));
    assert!(cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/hooks/pre-commit")));
    assert!(cfg.is_readonly(&VirtualPath::new("/other")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn direct_child_visible_rule_bridges_only_normalized_anchor_ancestors() {
    let source = test_dir();
    let cfg = launch(
        &source,
        Some(VisibilityDefault::Hidden),
        vec![],
        vec!["/workspace/repo/*.pem".to_string()],
        None,
        vec![],
        vec![],
    );
    for path in ["/", "/workspace", "/workspace/repo"] {
        assert_eq!(
            cfg.visibility_decision(&VirtualPath::new(path)),
            VisibilityDecision::BridgeVisible,
            "{path}"
        );
    }
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/local.pem")),
        VisibilityDecision::Visible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/local.pem/chain")),
        VisibilityDecision::Visible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/nested")),
        VisibilityDecision::Hidden
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/nested/local.pem")),
        VisibilityDecision::Hidden
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn broader_visible_rule_does_not_override_more_specific_hidden_descendant() {
    let source = test_dir();
    let cfg = launch(
        &source,
        None,
        vec!["/secret".to_string()],
        vec!["/".to_string()],
        None,
        vec![],
        vec![],
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/secret/file")),
        VisibilityDecision::Hidden
    );
    assert!(cfg.is_hidden(&VirtualPath::new("/secret/file")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn unprovable_opposite_glob_overlap_fails_fast() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let err = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec!["/workspace/**/*.json".to_string()],
            mutability_writable_rules: vec!["/workspace/tmp/**/*.lock".to_string()],
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap_err();
    assert!(
        err.contains(
            "readonly and writable rules have overlapping glob targets without provable containment"
        ),
        "{err}"
    );
    assert!(
        err.contains("readonly rule [recursive glob anchor=/workspace pattern=*.json]"),
        "{err}"
    );
    assert!(
        err.contains("writable rule [recursive glob anchor=/workspace/tmp pattern=*.lock]"),
        "{err}"
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn hidden_precedence_over_mutability() {
    let source = test_dir();
    let cfg = launch(
        &source,
        None,
        vec!["/secret".to_string()],
        vec![],
        Some(MutabilityDefault::Writable),
        vec![],
        vec!["/secret".to_string()],
    );
    assert!(cfg.is_hidden(&VirtualPath::new("/secret")));
    assert!(cfg.is_readonly(&VirtualPath::new("/secret")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn four_policy_surfaces_share_normalized_virtual_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(home.join("locks")).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["../secrets".to_string()],
                visibility_visible_rules: vec!["./fixtures/*.pem".to_string()],
                mutability_readonly_rules: vec!["~/locks/**/*.lock".to_string()],
                mutability_writable_rules: vec!["~/.env.*".to_string()],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: Some(MutabilityDefault::Readonly),
            experimental_writeback_cache: false,
        })
        .unwrap();

        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/workspace/secrets")));
        assert!(cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/fixtures/key.pem")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/home/tester/locks/app.lock")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/home/tester/.env.local")));
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn four_policy_surfaces_share_bare_basename_glob_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    std::fs::create_dir_all(&cwd).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, None);

        let hidden_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["*.pem".to_string()],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(hidden_cfg.matches_hidden_rule(&VirtualPath::new("/workspace/app/cert.pem/chain")));
        assert!(
            !hidden_cfg.matches_hidden_rule(&VirtualPath::new("/workspace/app/nested/cert.pem"))
        );

        let visible_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec![".env.*".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(
            visible_cfg
                .matches_visible_rule(&VirtualPath::new("/workspace/app/.env.local/secrets"))
        );
        assert!(
            !visible_cfg
                .matches_visible_rule(&VirtualPath::new("/workspace/app/nested/.env.local"))
        );

        let readonly_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec!["id_*".to_string()],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(
            readonly_cfg
                .matches_readonly_rule(&VirtualPath::new("/workspace/app/id_ed25519/private"))
        );
        assert!(
            !readonly_cfg
                .matches_readonly_rule(&VirtualPath::new("/workspace/app/nested/id_ed25519"))
        );

        let writable_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec!["*.lock".to_string()],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: Some(MutabilityDefault::Readonly),
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(
            writable_cfg.matches_writable_rule(&VirtualPath::new("/workspace/app/state.lock/data"))
        );
        assert!(
            !writable_cfg
                .matches_writable_rule(&VirtualPath::new("/workspace/app/nested/state.lock"))
        );
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn four_policy_surfaces_share_direct_child_glob_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["./fixtures/*.pem".to_string()],
                visibility_visible_rules: vec!["~/.env.*".to_string()],
                mutability_readonly_rules: vec!["/locks/id_*".to_string()],
                mutability_writable_rules: vec!["./build/*.lock".to_string()],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: Some(MutabilityDefault::Readonly),
            experimental_writeback_cache: false,
        })
        .unwrap();

        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
        assert!(!cfg.matches_hidden_rule(&VirtualPath::new(
            "/workspace/app/fixtures/nested/local.pem"
        )));
        assert!(cfg.matches_visible_rule(&VirtualPath::new("/home/tester/.env.local")));
        assert!(!cfg.matches_visible_rule(&VirtualPath::new("/home/tester/nested/.env.local")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/locks/id_ed25519")));
        assert!(!cfg.matches_readonly_rule(&VirtualPath::new("/locks/nested/id_ed25519")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/workspace/app/build/state.lock")));
        assert!(
            !cfg.matches_writable_rule(&VirtualPath::new("/workspace/app/build/nested/state.lock"))
        );
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn four_policy_surfaces_share_anchored_shorthand_normalization_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    std::fs::create_dir_all(source.join("workspace/app/fixtures-subtree/nested")).unwrap();
    std::fs::create_dir_all(home.join("locks/nested")).unwrap();
    std::fs::create_dir_all(home.join("sandbox/bin")).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["/vault/*".to_string(), "/archive/**".to_string()],
                visibility_visible_rules: vec![
                    "./fixtures/*".to_string(),
                    "./fixtures-subtree/**".to_string(),
                ],
                mutability_readonly_rules: vec!["~/locks/*".to_string()],
                mutability_writable_rules: vec!["~/sandbox/**".to_string()],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: Some(MutabilityDefault::Readonly),
            experimental_writeback_cache: false,
        })
        .unwrap();

        assert!(!cfg.matches_hidden_rule(&VirtualPath::new("/vault")));
        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/vault/child")));
        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/vault/child/grand")));
        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/archive")));
        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/archive/child")));

        assert!(!cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/fixtures")));
        assert!(cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
        assert!(cfg.matches_visible_rule(&VirtualPath::new(
            "/workspace/app/fixtures/nested/local.pem"
        )));
        assert!(cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/fixtures-subtree")));
        assert!(cfg.matches_visible_rule(&VirtualPath::new(
            "/workspace/app/fixtures-subtree/nested/local.pem"
        )));

        assert!(!cfg.matches_readonly_rule(&VirtualPath::new("/home/tester/locks")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/home/tester/locks/item")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/home/tester/locks/nested/item")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/home/tester/sandbox")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/home/tester/sandbox/bin/tool")));
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn config_file_accepts_recursive_literal_directory_shorthand_for_hidden_readonly_and_writable() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let config_path = source.join("screenfs.yaml");
    std::fs::write(
        &config_path,
        "visibility:\n  hidden:\n    - \"/repo/**/.git/hooks\"\nmutability:\n  default: readonly\n  readonly:\n    - \"**/aaa\"\n  writable:\n    - \"**/aaa/hook\"\n",
    )
    .unwrap();

    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: Some(config_path),
        visibility_default: None,
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    assert!(cfg.matches_hidden_rule(&VirtualPath::new("/repo/nested/.git/hooks/pre-commit")));
    assert!(cfg.matches_readonly_rule(&VirtualPath::new("/other/aaa/file")));
    assert!(cfg.matches_writable_rule(&VirtualPath::new("/workspace/repo/aaa/hook/file")));
    assert!(cfg.is_readonly(&VirtualPath::new("/other/aaa/file")));
    assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/repo/aaa/hook/file")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn visible_recursive_literal_directory_shorthand_rejects_recursive_bridge_discovery_from_config_file()
 {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let config_path = source.join("screenfs.yaml");
    std::fs::write(
        &config_path,
        "visibility:\n  default: hidden\n  visible:\n    - \"**/.git/hooks\"\n",
    )
    .unwrap();

    let err = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: Some(config_path),
        visibility_default: None,
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap_err();
    assert!(
        err.contains("recursive visible globs are unsupported"),
        "{err}"
    );
    assert!(err.contains("recursive bridge discovery"), "{err}");
    assert!(err.contains("visible rule ["), "{err}");
    assert!(
        err.contains("prefer an explicit subtree visible rule such as /dir or /dir/**"),
        "{err}"
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn shorthand_and_trailing_descendant_forms_conflict_at_same_normalized_specificity() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();

    let err = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec!["**/.git/hooks".to_string()],
            mutability_writable_rules: vec!["**/.git/hooks/**".to_string()],
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap_err();
    assert!(
        err.contains("readonly and writable rules conflict at the same normalized specificity"),
        "{err}"
    );
    assert!(
        err.contains("readonly rule [recursive literal subtree anchor=/ tail=.git/hooks]"),
        "{err}"
    );
    assert!(
        err.contains("writable rule [recursive literal subtree anchor=/ tail=.git/hooks]"),
        "{err}"
    );
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn visible_rules_reject_recursive_bridge_discovery_but_keep_discovery_free_forms() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    let home = source.join("home/tester");
    std::fs::create_dir_all(&cwd).unwrap();
    std::fs::create_dir_all(&home).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, Some(&home));
        for rule in ["/a", "/a/**", "/a/*", "/a/*.txt", "./fixtures/*", "~/*.pem"] {
            let _cfg = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    visibility_hidden_rules: vec![],
                    visibility_visible_rules: vec![rule.to_string()],
                    mutability_readonly_rules: vec![],
                    mutability_writable_rules: vec![],
                },
                config_path: None,
                visibility_default: Some(VisibilityDefault::Hidden),
                mutability_default: None,
                experimental_writeback_cache: false,
            })
            .unwrap();
        }

        for rule in [
            "**/*.pem",
            "/**/*.pem",
            "/dir/**/*.pem",
            "**/.git",
            "**/.git/**",
            "**/.git/hooks",
            "**/.git/hooks/**",
            "/repo/**/.git/hooks",
            "/repo/**/.git/hooks/**",
            "./fixtures/**/*.pem",
            "~/project/**/.git/hooks",
            "~/project/**/.git/hooks/**",
            "~/**/aaa/hook",
            "~/**/aaa/hook/**",
        ] {
            let err = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    visibility_hidden_rules: vec![],
                    visibility_visible_rules: vec![rule.to_string()],
                    mutability_readonly_rules: vec![],
                    mutability_writable_rules: vec![],
                },
                config_path: None,
                visibility_default: Some(VisibilityDefault::Hidden),
                mutability_default: None,
                experimental_writeback_cache: false,
            })
            .unwrap_err();
            assert!(
                err.contains("recursive visible globs are unsupported"),
                "{rule}: {err}"
            );
            assert!(err.contains("recursive bridge discovery"), "{rule}: {err}");
            assert!(err.contains("visible rule ["), "{rule}: {err}");
            assert!(
                err.contains("prefer an explicit subtree visible rule such as /dir or /dir/**"),
                "{rule}: {err}"
            );
        }
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn canonical_glob_families_share_visibility_and_mutability_axis_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
    std::fs::create_dir_all(source.join("workspace/peer")).unwrap();
    std::fs::create_dir_all(source.join("a/nested")).unwrap();
    let mount = source.join("mount");
    std::fs::create_dir(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, None);
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["**/*.pem".to_string()],
                visibility_visible_rules: vec!["./fixtures/*.pem".to_string()],
                mutability_readonly_rules: vec!["/a/*.txt".to_string()],
                mutability_writable_rules: vec!["/a/**/*.txt".to_string()],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: Some(MutabilityDefault::Readonly),
            experimental_writeback_cache: false,
        })
        .unwrap();

        assert!(cfg.matches_hidden_rule(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
        assert!(cfg.matches_hidden_rule(&VirtualPath::new(
            "/workspace/app/fixtures/nested/local.pem"
        )));
        assert!(!cfg.matches_hidden_rule(&VirtualPath::new("/workspace/peer/local.pem")));
        assert!(cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
        assert!(!cfg.matches_visible_rule(&VirtualPath::new(
            "/workspace/app/fixtures/nested/local.pem"
        )));
        assert!(!cfg.matches_visible_rule(&VirtualPath::new("/workspace/app/local.pem")));
        assert!(!cfg.is_hidden(&VirtualPath::new("/workspace/app/fixtures/local.pem")));
        assert!(cfg.is_hidden(&VirtualPath::new(
            "/workspace/app/fixtures/nested/local.pem"
        )));
        assert!(cfg.is_hidden(&VirtualPath::new("/workspace/app/local.pem")));
        assert!(!cfg.is_hidden(&VirtualPath::new("/workspace/peer/local.pem")));

        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/a/file.txt")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/a/file.txt/child")));
        assert!(!cfg.matches_readonly_rule(&VirtualPath::new("/a/nested/file.txt")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/a/file.txt")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/a/nested/file.txt")));
        assert!(cfg.matches_writable_rule(&VirtualPath::new("/a/nested/file.txt/child")));
        assert!(cfg.is_readonly(&VirtualPath::new("/a/file.txt")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/a/nested/file.txt")));
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn config_uses_two_axis_schema_and_cli_replaces_axis_independently() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let config_path = source.join("screenfs.yaml");
    std::fs::write(
            &config_path,
            "visibility:\n  default: hidden\n  visible:\n    - /from-config\nmutability:\n  default: readonly\n  writable:\n    - /writable-config\n",
        )
        .unwrap();
    let cfg = launch_from_args(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec!["/from-cli".to_string()],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: Some(config_path),
        visibility_default: None,
        mutability_default: None,
        experimental_writeback_cache: false,
    })
    .unwrap();

    assert_eq!(cfg.visibility_source(), PolicySource::Cli);
    assert_eq!(cfg.mutability_source(), PolicySource::Config);
    assert!(cfg.is_hidden(&VirtualPath::new("/from-cli")));
    assert!(!cfg.is_hidden(&VirtualPath::new("/from-config")));
    assert!(!cfg.is_readonly(&VirtualPath::new("/writable-config")));
    assert!(cfg.is_readonly(&VirtualPath::new("/other")));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn rejects_unknown_config_fields_and_same_specificity_conflicts() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
    let config_path = source.join("unknown.yaml");
    std::fs::write(&config_path, "mutability:\n  unsupported_field: nope\n").unwrap();

    {
        let _env = ProcessEnvGuard::new(&source, None);

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: Some(config_path.clone()),
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(
            err.contains(&format!("failed to parse config {}", config_path.display())),
            "{err}"
        );
        assert!(err.contains("unsupported_field"), "{err}");

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["/same".to_string()],
                visibility_visible_rules: vec!["/same".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(err.contains("hidden and visible rules conflict"), "{err}");
        assert!(err.contains("hidden rule [subtree /same]"), "{err}");
        assert!(err.contains("visible rule [subtree /same]"), "{err}");

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["*.pem".to_string()],
                visibility_visible_rules: vec!["./*.pem".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(err.contains("hidden and visible rules conflict"), "{err}");
        assert!(
            err.contains("hidden rule [direct-child glob anchor=/ pattern=*.pem]"),
            "{err}"
        );
        assert!(
            err.contains("visible rule [direct-child glob anchor=/ pattern=*.pem]"),
            "{err}"
        );

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["**/*.pem".to_string()],
                visibility_visible_rules: vec!["./**/*.pem".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(
            err.contains("recursive visible globs are unsupported"),
            "{err}"
        );
        assert!(err.contains("visible rule ["), "{err}");
        assert!(
            err.contains("prefer an explicit subtree visible rule such as /dir or /dir/**"),
            "{err}"
        );

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["/same/**".to_string()],
                visibility_visible_rules: vec!["/same".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(err.contains("hidden and visible rules conflict"), "{err}");
        assert!(err.contains("hidden rule [subtree /same]"), "{err}");
        assert!(err.contains("visible rule [subtree /same]"), "{err}");

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["/same/*".to_string()],
                visibility_visible_rules: vec!["/same/**/*.pem".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap_err();
        assert!(
            err.contains("recursive visible globs are unsupported"),
            "{err}"
        );
        assert!(err.contains("visible rule ["), "{err}");
        assert!(
            err.contains("prefer an explicit subtree visible rule such as /dir or /dir/**"),
            "{err}"
        );
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn bare_and_recursive_globs_keep_containment_override_semantics() {
    let source = test_dir();
    let cwd = source.join("workspace/app");
    std::fs::create_dir_all(&cwd).unwrap();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&cwd, None);
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                visibility_hidden_rules: vec!["**/*.pem".to_string()],
                visibility_visible_rules: vec!["*.pem".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();

        assert!(!cfg.is_hidden(&VirtualPath::new("/workspace/app/local.pem")));
        assert!(!cfg.is_hidden(&VirtualPath::new("/workspace/app/local.pem/chain")));
        assert!(cfg.is_hidden(&VirtualPath::new("/workspace/app/nested/local.pem")));
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn cwd_sensitive_glob_shorthands_fail_fast_when_current_dir_is_outside_source_root() {
    let root = test_dir();
    let source = root.join("source");
    let outside = root.join("outside");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&outside, None);
        let absolute_only = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                visibility_hidden_rules: vec!["/absolute".to_string()],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: None,
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(absolute_only.matches_hidden_rule(&VirtualPath::new("/absolute")));

        for rule in ["*.pem", "**/*.pem"] {
            let err = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    visibility_hidden_rules: vec![rule.to_string()],
                    visibility_visible_rules: vec![],
                    mutability_readonly_rules: vec![],
                    mutability_writable_rules: vec![],
                },
                config_path: None,
                visibility_default: None,
                mutability_default: None,
                experimental_writeback_cache: false,
            })
            .unwrap_err();

            assert!(
                err.contains("rule path resolves outside source_root: ."),
                "{rule}: {err}"
            );
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn reports_shared_normalization_failures_per_surface() {
    let source = test_dir();
    for (rules, expected) in [
        (
            (vec!["*secret*".to_string()], vec![], vec![], vec![]),
            "invalid hidden pattern",
        ),
        (
            (vec![], vec!["*secret*".to_string()], vec![], vec![]),
            "invalid visible pattern",
        ),
        (
            (vec![], vec![], vec!["*secret*".to_string()], vec![]),
            "invalid readonly pattern",
        ),
        (
            (vec![], vec![], vec![], vec!["*secret*".to_string()]),
            "invalid writable pattern",
        ),
        (
            (vec!["**/*".to_string()], vec![], vec![], vec![]),
            "invalid hidden pattern",
        ),
        (
            (vec![], vec!["**/*".to_string()], vec![], vec![]),
            "invalid visible pattern",
        ),
        (
            (vec![], vec![], vec!["*".to_string()], vec![]),
            "invalid readonly pattern",
        ),
        (
            (vec![], vec![], vec![], vec!["*".to_string()]),
            "invalid writable pattern",
        ),
    ] {
        let (hidden, visible, readonly, writable) = rules;
        let err = std::panic::catch_unwind(|| {
            launch(&source, None, hidden, visible, None, readonly, writable)
        })
        .err();
        let message = if let Some(payload) = err {
            if let Some(message) = payload.downcast_ref::<String>() {
                message.clone()
            } else if let Some(message) = payload.downcast_ref::<&str>() {
                message.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        assert!(message.contains(expected), "{message}");
    }
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn mutability_fast_path_helpers_follow_default_policies() {
    let source = test_dir();

    let writable_default = launch(
        &source,
        None,
        vec![],
        vec![],
        Some(MutabilityDefault::Writable),
        vec![],
        vec![],
    );
    let writable_path = VirtualPath::new("/workspace/file.txt");
    assert_eq!(
        writable_default.mutability_decision(&writable_path),
        MutabilityDecision::Writable
    );
    assert!(writable_default.can_skip_resolved_target_mutability_check(
        writable_default.mutability_decision(&writable_path)
    ));
    assert!(!writable_default.is_readonly(&writable_path));

    let writable_with_readonly = launch(
        &source,
        None,
        vec![],
        vec![],
        Some(MutabilityDefault::Writable),
        vec!["/locked".to_string()],
        vec![],
    );
    let locked_path = VirtualPath::new("/locked");
    let free_path = VirtualPath::new("/free");
    assert_eq!(
        writable_with_readonly.mutability_decision(&locked_path),
        MutabilityDecision::Readonly
    );
    assert!(
        writable_with_readonly.can_skip_resolved_target_mutability_check(
            writable_with_readonly.mutability_decision(&locked_path)
        )
    );
    assert!(writable_with_readonly.is_readonly(&locked_path));
    assert_eq!(
        writable_with_readonly.mutability_decision(&free_path),
        MutabilityDecision::Writable
    );
    assert!(
        !writable_with_readonly.can_skip_resolved_target_mutability_check(
            writable_with_readonly.mutability_decision(&free_path)
        )
    );
    assert!(!writable_with_readonly.is_readonly(&free_path));

    let readonly_default = launch(
        &source,
        None,
        vec![],
        vec![],
        Some(MutabilityDefault::Readonly),
        vec![],
        vec![],
    );
    let readonly_path = VirtualPath::new("/readonly-default");
    assert_eq!(
        readonly_default.mutability_decision(&readonly_path),
        MutabilityDecision::Readonly
    );
    assert!(readonly_default.can_skip_resolved_target_mutability_check(
        readonly_default.mutability_decision(&readonly_path)
    ));
    assert!(readonly_default.is_readonly(&readonly_path));

    let readonly_with_writable = launch(
        &source,
        None,
        vec![],
        vec![],
        Some(MutabilityDefault::Readonly),
        vec![],
        vec!["/workspace".to_string()],
    );
    let writable_carve_out = VirtualPath::new("/workspace/file.txt");
    let blocked_elsewhere = VirtualPath::new("/elsewhere");
    assert_eq!(
        readonly_with_writable.mutability_decision(&writable_carve_out),
        MutabilityDecision::Writable
    );
    assert!(
        !readonly_with_writable.can_skip_resolved_target_mutability_check(
            readonly_with_writable.mutability_decision(&writable_carve_out)
        )
    );
    assert!(!readonly_with_writable.is_readonly(&writable_carve_out));
    assert_eq!(
        readonly_with_writable.mutability_decision(&blocked_elsewhere),
        MutabilityDecision::Readonly
    );
    assert!(
        readonly_with_writable.can_skip_resolved_target_mutability_check(
            readonly_with_writable.mutability_decision(&blocked_elsewhere)
        )
    );
    assert!(readonly_with_writable.is_readonly(&blocked_elsewhere));

    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn symlink_target_fast_path_is_enabled_only_when_visibility_policy_cannot_hide_targets() {
    let root = test_dir();
    let source = root.join("source");
    let outside_mount = root.join("mount");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::create_dir_all(&outside_mount).unwrap();

    {
        let _env = ProcessEnvGuard::new(&source, None);
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: outside_mount.clone(),
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Visible),
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(cfg.can_skip_symlink_target_visibility_check());

        let hidden_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: outside_mount.clone(),
                visibility_hidden_rules: vec!["/hidden".to_string()],
                visibility_visible_rules: vec![],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Visible),
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(!hidden_cfg.can_skip_symlink_target_visibility_check());

        let default_hidden_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: outside_mount,
                visibility_hidden_rules: vec![],
                visibility_visible_rules: vec!["/visible".to_string()],
                mutability_readonly_rules: vec![],
                mutability_writable_rules: vec![],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: None,
            experimental_writeback_cache: false,
        })
        .unwrap();
        assert!(!default_hidden_cfg.can_skip_symlink_target_visibility_check());
    }
    std::fs::remove_dir_all(root).unwrap();
}

fn test_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("screenfs-config-test-{nanos}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
