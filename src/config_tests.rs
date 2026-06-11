use super::*;
use crate::path::ProcessEnvGuard;
use std::time::{SystemTime, UNIX_EPOCH};

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
    })
    .unwrap()
}

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
fn includes_mount_root_internal_hidden_rule() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(&mount).unwrap();
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
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn mount_root_internal_hidden_rule_cannot_be_made_visible() {
    let source = test_dir();
    let mount = source.join("mnt");
    std::fs::create_dir_all(mount.join("public")).unwrap();
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
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
fn descendant_subtree_visible_rule_bridges_all_existing_ancestors() {
    let source = test_dir();
    let cfg = launch(
        &source,
        Some(VisibilityDefault::Hidden),
        vec![],
        vec!["**/.git/hooks/**".to_string()],
        None,
        vec![],
        vec![],
    );
    for path in ["/", "/workspace", "/workspace/repo", "/workspace/repo/.git"] {
        assert_eq!(
            cfg.visibility_decision(&VirtualPath::new(path)),
            VisibilityDecision::BridgeVisible,
            "{path}"
        );
    }
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/.git/hooks")),
        VisibilityDecision::Visible
    );
    assert_eq!(
        cfg.visibility_decision(&VirtualPath::new("/workspace/repo/.git/hooks/pre-commit")),
        VisibilityDecision::Visible
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
    let err = RuntimeConfig::from_launch(LaunchArgs {
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
    })
    .unwrap_err();
    assert!(
        err.contains(
            "readonly and writable rules have overlapping glob targets without provable containment"
        ),
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
                visibility_visible_rules: vec!["./fixtures/**/*.pem".to_string()],
                mutability_readonly_rules: vec!["~/locks/**/*.lock".to_string()],
                mutability_writable_rules: vec!["~/.env.*".to_string()],
            },
            config_path: None,
            visibility_default: Some(VisibilityDefault::Hidden),
            mutability_default: Some(MutabilityDefault::Readonly),
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
    let cfg = RuntimeConfig::from_launch(LaunchArgs {
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
    let err = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount.clone(),
            visibility_hidden_rules: vec![],
            visibility_visible_rules: vec![],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: Some(config_path),
        visibility_default: None,
        mutability_default: None,
    })
    .unwrap_err();
    assert!(err.contains("failed to parse config"));

    let err = RuntimeConfig::from_launch(LaunchArgs {
        cli: CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            visibility_hidden_rules: vec!["/same".to_string()],
            visibility_visible_rules: vec!["/same".to_string()],
            mutability_readonly_rules: vec![],
            mutability_writable_rules: vec![],
        },
        config_path: None,
        visibility_default: None,
        mutability_default: None,
    })
    .unwrap_err();
    assert!(err.contains("hidden and visible rules conflict"));
    std::fs::remove_dir_all(source).unwrap();
}

#[test]
fn reports_shared_normalization_failures_per_surface() {
    let source = test_dir();
    for (rules, expected) in [
        (
            (vec!["*.pem".to_string()], vec![], vec![], vec![]),
            "invalid hidden pattern",
        ),
        (
            (vec![], vec!["*.pem".to_string()], vec![], vec![]),
            "invalid visible pattern",
        ),
        (
            (vec![], vec![], vec!["*.pem".to_string()], vec![]),
            "invalid readonly pattern",
        ),
        (
            (vec![], vec![], vec![], vec!["*.pem".to_string()]),
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

fn test_dir() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("screenfs-config-test-{nanos}"));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
