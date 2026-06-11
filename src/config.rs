use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;

use crate::cli::{
    CliArgs, LaunchArgs, MutabilityFamily, MutabilitySurface, validate_future_mutability_surface,
};
use crate::matcher::{MatcherScope, PathRuleMatcher, RuleDescriptor, mount_root_internal_prefix};
use crate::path::{RuleNormalizationContext, VirtualPath};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutabilitySource {
    Default,
    Cli,
    Config,
}

impl MutabilitySource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Cli => "cli",
            Self::Config => "config",
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub matcher: PathRuleMatcher,
    pub readonly_matcher: PathRuleMatcher,
    pub allow_write_matcher: PathRuleMatcher,
    mutability_family: MutabilityFamily,
    mutability_source: MutabilitySource,
    readonly_rule_count: usize,
    allow_write_rule_count: usize,
    pub attr_ttl: Duration,
    pub entry_ttl: Duration,
}

impl RuntimeConfig {
    #[cfg(test)]
    pub(crate) fn from_cli(args: CliArgs) -> Result<Self, String> {
        Self::from_launch(LaunchArgs {
            cli: args,
            config_path: None,
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
    }

    pub fn from_launch(args: LaunchArgs) -> Result<Self, String> {
        let LaunchArgs {
            cli,
            config_path,
            policy_family,
            allow_write_rules,
        } = args;
        let context = RuleNormalizationContext::from_environment(&cli.source_root)?;
        let file_config = match config_path.as_ref() {
            Some(path) => load_file_config(path)?,
            None => FileConfig::default(),
        };

        let mut internal_prefixes = Vec::new();
        if let Some(prefix) = mount_root_internal_prefix(&cli.source_root, &cli.mount_root) {
            internal_prefixes.push(prefix);
        }
        let matcher = compile_matcher(
            MatcherScope::Hide,
            cli.hide_rules.iter().map(String::as_str),
            internal_prefixes,
            &context,
        )?;
        let mutability = resolve_mutability(
            &cli,
            policy_family,
            &allow_write_rules,
            file_config.mutability,
        )?;
        let readonly_matcher = compile_matcher(
            MatcherScope::Readonly,
            mutability.readonly_rules.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;
        let allow_write_matcher = compile_matcher(
            MatcherScope::AllowWrite,
            mutability.allow_write_rules.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;
        validate_nested_mutability_rules(
            mutability.family,
            &readonly_matcher,
            &allow_write_matcher,
        )?;

        Ok(Self {
            source_root: cli.source_root,
            mount_root: cli.mount_root,
            matcher,
            readonly_matcher,
            allow_write_matcher,
            mutability_family: mutability.family,
            mutability_source: mutability.source,
            readonly_rule_count: mutability.readonly_rules.len(),
            allow_write_rule_count: mutability.allow_write_rules.len(),
            attr_ttl: Duration::from_secs(1),
            entry_ttl: Duration::from_secs(1),
        })
    }

    pub fn is_hidden(&self, path: &VirtualPath) -> bool {
        self.matcher.matches_path(path)
    }

    pub fn is_hidden_symlink_target(&self, link_path: &VirtualPath, raw_target: &OsStr) -> bool {
        self.matcher.matches_symlink_target(link_path, raw_target)
    }

    pub fn matches_readonly_rule(&self, path: &VirtualPath) -> bool {
        self.readonly_matcher.matches_path(path)
    }

    pub fn matches_allow_write_rule(&self, path: &VirtualPath) -> bool {
        self.allow_write_matcher.matches_path(path)
    }

    pub fn is_readonly(&self, path: &VirtualPath) -> bool {
        let readonly = self.readonly_matcher.best_specificity(path);
        let allow_write = self.allow_write_matcher.best_specificity(path);
        match self.mutability_family {
            MutabilityFamily::SelectiveReadonly => match (readonly, allow_write) {
                (Some(readonly), Some(allow_write)) => readonly >= allow_write,
                (Some(_), None) => true,
                _ => false,
            },
            MutabilityFamily::ReadonlyRootAllowwrite => match (readonly, allow_write) {
                (Some(readonly), Some(allow_write)) => readonly >= allow_write,
                (None, Some(_)) => false,
                _ => true,
            },
        }
    }

    pub fn mutability_family(&self) -> MutabilityFamily {
        self.mutability_family
    }

    pub fn mutability_source(&self) -> MutabilitySource {
        self.mutability_source
    }

    pub fn readonly_rule_count(&self) -> usize {
        self.readonly_rule_count
    }

    pub fn allow_write_rule_count(&self) -> usize {
        self.allow_write_rule_count
    }
}

#[derive(Debug)]
struct ResolvedMutability {
    family: MutabilityFamily,
    source: MutabilitySource,
    readonly_rules: Vec<String>,
    allow_write_rules: Vec<String>,
}

fn resolve_mutability(
    cli: &CliArgs,
    policy_family: Option<MutabilityFamily>,
    allow_write_rules: &[String],
    config_mutability: Option<MutabilityConfig>,
) -> Result<ResolvedMutability, String> {
    if LaunchArgs::has_future_mutability_options(
        policy_family,
        &cli.readonly_rules,
        allow_write_rules,
    ) {
        let family = validate_future_mutability_surface(
            MutabilitySurface::Cli,
            policy_family,
            &cli.readonly_rules,
            allow_write_rules,
        )?;
        return Ok(ResolvedMutability {
            family,
            source: MutabilitySource::Cli,
            readonly_rules: cli.readonly_rules.clone(),
            allow_write_rules: allow_write_rules.to_vec(),
        });
    }

    if let Some(config_mutability) = config_mutability {
        let family = validate_future_mutability_surface(
            MutabilitySurface::Config,
            config_mutability.family,
            &config_mutability.readonly_rules,
            &config_mutability.allow_write,
        )?;
        return Ok(ResolvedMutability {
            family,
            source: MutabilitySource::Config,
            readonly_rules: config_mutability.readonly_rules,
            allow_write_rules: config_mutability.allow_write,
        });
    }

    Ok(ResolvedMutability {
        family: MutabilityFamily::SelectiveReadonly,
        source: MutabilitySource::Default,
        readonly_rules: Vec::new(),
        allow_write_rules: Vec::new(),
    })
}

fn compile_matcher<'a, I>(
    scope: MatcherScope,
    rules: I,
    internal_prefixes: Vec<VirtualPath>,
    context: &RuleNormalizationContext,
) -> Result<PathRuleMatcher, String>
where
    I: IntoIterator<Item = &'a str>,
{
    PathRuleMatcher::compile(scope, rules, internal_prefixes, context)
}

fn validate_nested_mutability_rules(
    family: MutabilityFamily,
    readonly: &PathRuleMatcher,
    allow_write: &PathRuleMatcher,
) -> Result<(), String> {
    for readonly_rule in readonly.descriptors() {
        for allow_write_rule in allow_write.descriptors() {
            if readonly_rule.specificity() == allow_write_rule.specificity()
                && same_anchor(readonly_rule, allow_write_rule)
            {
                return Err(
                    "readonly and allow-write rules conflict at the same normalized specificity"
                        .to_string(),
                );
            }
        }
    }

    match family {
        MutabilityFamily::SelectiveReadonly => {
            validate_secondary_rules(
                allow_write.descriptors(),
                readonly.descriptors(),
                "allow-write",
                "readonly",
            )?;
        }
        MutabilityFamily::ReadonlyRootAllowwrite => {
            validate_secondary_rules(
                readonly.descriptors(),
                allow_write.descriptors(),
                "readonly",
                "allow-write",
            )?;
        }
    }
    Ok(())
}

fn validate_secondary_rules(
    secondary_rules: &[RuleDescriptor],
    primary_rules: &[RuleDescriptor],
    secondary_label: &str,
    primary_label: &str,
) -> Result<(), String> {
    for secondary in secondary_rules {
        if !primary_rules
            .iter()
            .any(|primary| primary.has_less_specific_ancestor_of(secondary))
        {
            return Err(format!(
                "{secondary_label} rule requires a less-specific ancestor {primary_label} rule"
            ));
        }
    }
    Ok(())
}

fn same_anchor(left: &RuleDescriptor, right: &RuleDescriptor) -> bool {
    left.anchor() == right.anchor()
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    #[serde(default)]
    mutability: Option<MutabilityConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MutabilityConfig {
    family: Option<MutabilityFamily>,
    #[serde(default)]
    readonly_rules: Vec<String>,
    #[serde(default)]
    allow_write: Vec<String>,
}

fn load_file_config(path: &Path) -> Result<FileConfig, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read config {}: {err}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(FileConfig::default());
    }
    serde_yaml::from_str(&content)
        .map_err(|err| format!("failed to parse config {}: {err}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::ProcessEnvGuard;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn readonly_root_allowwrite_without_allow_write_makes_visible_paths_readonly() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec!["/secret".to_string(), "/secret".to_string()],
                readonly_rules: vec![],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: Vec::new(),
        })
        .unwrap();
        assert_eq!(cfg.mutability_source(), MutabilitySource::Cli);
        assert_eq!(
            cfg.mutability_family(),
            MutabilityFamily::ReadonlyRootAllowwrite
        );
        assert!(cfg.is_hidden(&VirtualPath::new("/secret")));
        assert!(cfg.is_readonly(&VirtualPath::new("/logs/app")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn includes_mount_root_internal_hide_rule() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_cli(CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            hide_rules: vec![],
            readonly_rules: vec![],
        })
        .unwrap();
        assert!(cfg.is_hidden(&VirtualPath::new("/mnt")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn hide_and_readonly_rules_share_normalized_virtual_semantics() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        let home = source.join("home/tester");
        std::fs::create_dir_all(&cwd).unwrap();
        std::fs::create_dir_all(source.join("workspace/secrets")).unwrap();
        std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
        std::fs::create_dir_all(home.join("locks")).unwrap();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();

        {
            let _env = ProcessEnvGuard::new(&cwd, Some(&home));
            let cfg = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    hide_rules: vec![
                        "../secrets".to_string(),
                        "./fixtures/**/*.pem".to_string(),
                        "~/locks/**/*.lock".to_string(),
                        "~/.env.*".to_string(),
                    ],
                    readonly_rules: vec![
                        "../secrets".to_string(),
                        "./fixtures/**/*.pem".to_string(),
                        "~/locks/**/*.lock".to_string(),
                        "~/.env.*".to_string(),
                    ],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap();

            for path in [
                VirtualPath::new("/workspace/secrets"),
                VirtualPath::new("/workspace/app/fixtures/key.pem"),
                VirtualPath::new("/workspace/app/fixtures/nested/key.pem/chain"),
                VirtualPath::new("/home/tester/locks/app.lock"),
                VirtualPath::new("/home/tester/.env.local"),
            ] {
                assert!(cfg.is_hidden(&path));
                assert!(cfg.matches_readonly_rule(&path));
            }
            let visible = VirtualPath::new("/workspace/app/fixtures/key.key");
            assert!(!cfg.is_hidden(&visible));
            assert!(!cfg.matches_readonly_rule(&visible));
            let nested_env = VirtualPath::new("/home/tester/nested/.env.local");
            assert!(!cfg.is_hidden(&nested_env));
            assert!(!cfg.matches_readonly_rule(&nested_env));
        }

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn allow_write_rules_share_normalized_virtual_semantics() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        let home = source.join("home/tester");
        std::fs::create_dir_all(&cwd).unwrap();
        std::fs::create_dir_all(source.join("workspace/secrets")).unwrap();
        std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
        std::fs::create_dir_all(home.join("locks")).unwrap();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();

        {
            let _env = ProcessEnvGuard::new(&cwd, Some(&home));
            let cfg = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    hide_rules: vec![],
                    readonly_rules: vec![],
                },
                config_path: None,
                policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
                allow_write_rules: vec![
                    "../secrets".to_string(),
                    "./fixtures/**/*.pem".to_string(),
                    "~/locks/**/*.lock".to_string(),
                    "~/.env.*".to_string(),
                ],
            })
            .unwrap();

            for path in [
                VirtualPath::new("/workspace/secrets"),
                VirtualPath::new("/workspace/app/fixtures/key.pem"),
                VirtualPath::new("/workspace/app/fixtures/nested/key.pem/chain"),
                VirtualPath::new("/home/tester/locks/app.lock"),
                VirtualPath::new("/home/tester/.env.local"),
            ] {
                assert!(cfg.matches_allow_write_rule(&path));
                assert!(!cfg.is_readonly(&path));
            }
            let readonly = VirtualPath::new("/workspace/app/fixtures/key.key");
            assert!(!cfg.matches_allow_write_rule(&readonly));
            assert!(cfg.is_readonly(&readonly));
            let nested_env = VirtualPath::new("/home/tester/nested/.env.local");
            assert!(!cfg.matches_allow_write_rule(&nested_env));
            assert!(cfg.is_readonly(&nested_env));
        }

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn readonly_rule_reports_shared_normalization_failures() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        let mount = source.join("mount");
        std::fs::create_dir_all(&cwd).unwrap();
        std::fs::create_dir(&mount).unwrap();

        let err = {
            let _env = ProcessEnvGuard::new(&cwd, None);
            RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount,
                    hide_rules: vec![],
                    readonly_rules: vec!["~/.ssh".to_string()],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap_err()
        };
        assert!(err.contains("invalid readonly pattern"));
        assert!(err.contains("HOME is not set"));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn loads_mutability_from_config_when_cli_mutability_is_absent() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: readonly-root-allowwrite\n  allow_write:\n    - /tmp\n    - '**/*.lock'\n",
        )
        .unwrap();

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap();

        assert_eq!(cfg.mutability_source(), MutabilitySource::Config);
        assert_eq!(
            cfg.mutability_family(),
            MutabilityFamily::ReadonlyRootAllowwrite
        );
        assert_eq!(cfg.readonly_rule_count(), 0);
        assert_eq!(cfg.allow_write_rule_count(), 2);
        assert!(cfg.matches_allow_write_rule(&VirtualPath::new("/tmp")));
        assert!(cfg.matches_allow_write_rule(&VirtualPath::new("/var/app.lock")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/tmp")));
        assert!(cfg.is_readonly(&VirtualPath::new("/var/app.log")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn cli_mutability_replaces_config_mutability() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: selective-readonly\n  readonly_rules:\n    - /from-config\n",
        )
        .unwrap();

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: vec!["/from-cli".to_string()],
        })
        .unwrap();

        assert_eq!(cfg.mutability_source(), MutabilitySource::Cli);
        assert_eq!(
            cfg.mutability_family(),
            MutabilityFamily::ReadonlyRootAllowwrite
        );
        assert!(!cfg.matches_readonly_rule(&VirtualPath::new("/from-config")));
        assert!(cfg.matches_allow_write_rule(&VirtualPath::new("/from-cli")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn config_without_family_defaults_to_selective_readonly() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  readonly_rules:\n    - /logs\n",
        )
        .unwrap();

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap();

        assert_eq!(cfg.mutability_source(), MutabilitySource::Config);
        assert_eq!(cfg.mutability_family(), MutabilityFamily::SelectiveReadonly);
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/logs")));
        assert!(cfg.is_readonly(&VirtualPath::new("/logs")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn defaults_to_selective_readonly_without_cli_or_config_mutability() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: None,
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap();

        assert_eq!(cfg.mutability_source(), MutabilitySource::Default);
        assert_eq!(cfg.mutability_family(), MutabilityFamily::SelectiveReadonly);
        assert_eq!(cfg.readonly_rule_count(), 0);
        assert_eq!(cfg.allow_write_rule_count(), 0);
        assert!(!cfg.is_readonly(&VirtualPath::new("/visible")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn rejects_unknown_fields_in_config() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: selective-readonly\n  extra: nope\nunknown: value\n",
        )
        .unwrap();

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap_err();

        assert!(err.contains("failed to parse config"));
        assert!(err.contains("unknown field"));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn hide_and_readonly_rules_accept_direct_child_suffix_forms_through_runtime_config() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        let home = source.join("home/tester");
        std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
        std::fs::create_dir_all(source.join("home/tester/nested")).unwrap();
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
                    hide_rules: vec!["~/*.pem".to_string(), "./fixtures/*.pem".to_string()],
                    readonly_rules: vec!["~/*.pem".to_string(), "./fixtures/*.pem".to_string()],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap();

            for path in [
                VirtualPath::new("/home/tester/user.pem"),
                VirtualPath::new("/home/tester/user.pem/chain"),
                VirtualPath::new("/workspace/app/fixtures/local.pem"),
                VirtualPath::new("/workspace/app/fixtures/local.pem/chain"),
            ] {
                assert!(cfg.is_hidden(&path));
                assert!(cfg.matches_readonly_rule(&path));
            }

            for path in [
                VirtualPath::new("/home/tester/nested/user.pem"),
                VirtualPath::new("/workspace/app/fixtures/nested/local.pem"),
                VirtualPath::new("/home/tester/user.key"),
                VirtualPath::new("/workspace/app/fixtures/local.key"),
            ] {
                assert!(!cfg.is_hidden(&path));
                assert!(!cfg.matches_readonly_rule(&path));
            }
        }

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn allow_write_rules_accept_direct_child_suffix_forms_through_runtime_config() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        let home = source.join("home/tester");
        std::fs::create_dir_all(source.join("workspace/app/fixtures/nested")).unwrap();
        std::fs::create_dir_all(source.join("home/tester/nested")).unwrap();
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
                    hide_rules: vec![],
                    readonly_rules: vec![],
                },
                config_path: None,
                policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
                allow_write_rules: vec!["~/*.pem".to_string(), "./fixtures/*.pem".to_string()],
            })
            .unwrap();

            for path in [
                VirtualPath::new("/home/tester/user.pem"),
                VirtualPath::new("/home/tester/user.pem/chain"),
                VirtualPath::new("/workspace/app/fixtures/local.pem"),
                VirtualPath::new("/workspace/app/fixtures/local.pem/chain"),
            ] {
                assert!(cfg.matches_allow_write_rule(&path));
                assert!(!cfg.is_readonly(&path));
            }

            for path in [
                VirtualPath::new("/home/tester/nested/user.pem"),
                VirtualPath::new("/workspace/app/fixtures/nested/local.pem"),
                VirtualPath::new("/home/tester/user.key"),
                VirtualPath::new("/workspace/app/fixtures/local.key"),
            ] {
                assert!(!cfg.matches_allow_write_rule(&path));
                assert!(cfg.is_readonly(&path));
            }
        }

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn descendant_subtree_globs_share_runtime_config_grammar_across_surfaces() {
        let source = test_dir();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();
        std::fs::create_dir_all(source.join("project/repo/.git/hooks")).unwrap();
        std::fs::write(source.join("project/repo/.git/hooks/pre-commit"), b"hook").unwrap();
        std::fs::write(source.join("project/repo/.git/config"), b"config").unwrap();

        let hook = VirtualPath::new("/project/repo/.git/hooks/pre-commit");
        let hook_dir = VirtualPath::new("/project/repo/.git/hooks");
        let config = VirtualPath::new("/project/repo/.git/config");

        let cli_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec!["/project/**/.git/hooks/**".to_string()],
                readonly_rules: vec!["/project/**/.git/hooks/**".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec![],
        })
        .unwrap();
        assert!(cli_cfg.is_hidden(&hook));
        assert!(cli_cfg.matches_readonly_rule(&hook));
        assert!(cli_cfg.matches_readonly_rule(&hook_dir));
        assert!(!cli_cfg.matches_readonly_rule(&config));

        let allow_write_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: vec!["/project/**/.git/hooks/**".to_string()],
        })
        .unwrap();
        assert!(allow_write_cfg.matches_allow_write_rule(&hook));
        assert!(allow_write_cfg.matches_allow_write_rule(&hook_dir));
        assert!(!allow_write_cfg.matches_allow_write_rule(&config));
        assert!(!allow_write_cfg.is_readonly(&hook));
        assert!(allow_write_cfg.is_readonly(&config));

        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: readonly-root-allowwrite\n  allow_write:\n    - /project/**/.git/hooks/**\n",
        )
        .unwrap();
        let file_cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap();
        assert_eq!(file_cfg.mutability_source(), MutabilitySource::Config);
        assert!(file_cfg.matches_allow_write_rule(&hook));
        assert!(!file_cfg.matches_allow_write_rule(&config));

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn selective_readonly_nested_allow_write_uses_most_specific_match() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec!["/workspace".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/workspace/tmp".to_string()],
        })
        .unwrap();

        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/docs/a.txt")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/tmp/out.txt")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/free/out.txt")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn selective_readonly_descendant_subtree_nested_allow_write_uses_literal_tail_specificity() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec!["/workspace/**/.git/**".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/workspace/**/.git/hooks/**".to_string()],
        })
        .unwrap();

        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/config")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/hooks/pre-commit")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn runtime_config_accepts_requested_absolute_descendant_subtree_pattern() {
        let source = test_dir();
        let mount = source.join("mount");
        std::fs::create_dir_all(&mount).unwrap();
        let pattern = "/home/spi-ca/Codebase/the-onion/palgong/**/.git/hooks/**".to_string();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: PathBuf::from("/"),
                mount_root: mount,
                hide_rules: vec![pattern.clone()],
                readonly_rules: vec![pattern.clone()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec![],
        })
        .unwrap();
        let hook =
            VirtualPath::new("/home/spi-ca/Codebase/the-onion/palgong/repo/.git/hooks/pre-commit");
        let outside =
            VirtualPath::new("/home/spi-ca/Codebase/the-onion/other/.git/hooks/pre-commit");

        assert!(cfg.is_hidden(&hook));
        assert!(cfg.matches_readonly_rule(&hook));
        assert!(!cfg.is_hidden(&outside));
        assert!(!cfg.matches_readonly_rule(&outside));

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn readonly_root_allowwrite_nested_readonly_uses_most_specific_match() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec!["/workspace/vendor".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: vec!["/workspace".to_string()],
        })
        .unwrap();

        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/out.txt")));
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/vendor/lock.json")));
        assert!(cfg.is_readonly(&VirtualPath::new("/free/out.txt")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn nested_mutability_rules_require_primary_ancestor_and_no_equal_conflict() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["/workspace".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/other/tmp".to_string()],
        })
        .unwrap_err();
        assert!(err.contains("allow-write rule requires a less-specific ancestor readonly rule"));

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["/workspace".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/workspace".to_string()],
        })
        .unwrap_err();
        assert!(err.contains("conflict at the same normalized specificity"));

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["/workspace/**/*.json".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/workspace/tmp/**/*.json".to_string()],
        })
        .unwrap();
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/app/config.json")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/tmp/config.json")));

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["**/.env*".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["**/.env.local".to_string()],
        })
        .unwrap();
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/.env.prod")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/.env.local")));

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["**/.git/**".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["**/.git/hooks/**".to_string()],
        })
        .unwrap();
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/config")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/hooks/pre-commit")));

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["**/.git/hooks/**".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: vec!["**/.git/**".to_string()],
        })
        .unwrap();
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/config")));
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/repo/.git/hooks/pre-commit")));

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec!["/workspace/**/*.json".to_string()],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::SelectiveReadonly),
            allow_write_rules: vec!["/workspace/tmp/**/*.lock".to_string()],
        })
        .unwrap_err();
        assert!(err.contains("allow-write rule requires a less-specific ancestor readonly rule"));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn config_option_b_requires_family_when_both_rule_sets_are_present() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  readonly_rules:\n    - /workspace\n  allow_write:\n    - /workspace/tmp\n",
        )
        .unwrap();

        let err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap_err();
        assert!(err.contains("require explicit mutability.family"));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn config_option_b_uses_config_as_source_of_truth_without_cli_mutability() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: selective-readonly\n  readonly_rules:\n    - /workspace\n  allow_write:\n    - /workspace/tmp\n",
        )
        .unwrap();

        let cfg = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap();

        assert_eq!(cfg.mutability_source(), MutabilitySource::Config);
        assert!(cfg.is_readonly(&VirtualPath::new("/workspace/docs/a.txt")));
        assert!(!cfg.is_readonly(&VirtualPath::new("/workspace/tmp/out.txt")));
        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn bare_suffix_globs_stay_unsupported_on_hide_and_mutability_surfaces() {
        let source = test_dir();
        let cwd = source.join("workspace/app");
        std::fs::create_dir_all(&cwd).unwrap();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();

        {
            let _env = ProcessEnvGuard::new(&cwd, None);
            let hide_err = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    hide_rules: vec!["*.pem".to_string()],
                    readonly_rules: vec![],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap_err();
            assert!(hide_err.contains("unsupported glob: *.pem"));

            let readonly_err = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount.clone(),
                    hide_rules: vec![],
                    readonly_rules: vec!["*.pem".to_string()],
                },
                config_path: None,
                policy_family: None,
                allow_write_rules: Vec::new(),
            })
            .unwrap_err();
            assert!(readonly_err.contains("unsupported glob: *.pem"));

            let allow_write_err = RuntimeConfig::from_launch(LaunchArgs {
                cli: CliArgs {
                    source_root: source.clone(),
                    mount_root: mount,
                    hide_rules: vec![],
                    readonly_rules: vec![],
                },
                config_path: None,
                policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
                allow_write_rules: vec!["*.pem".to_string()],
            })
            .unwrap_err();
            assert!(allow_write_err.contains("unsupported glob: *.pem"));
        }

        std::fs::remove_dir_all(source).unwrap();
    }

    #[test]
    fn broader_descendant_wildcard_forms_stay_unsupported_across_runtime_config_surfaces() {
        let source = test_dir();
        let mount = source.join("mount");
        std::fs::create_dir(&mount).unwrap();

        let hide_err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec!["**/.git/**/hooks/**".to_string()],
                readonly_rules: vec![],
            },
            config_path: None,
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap_err();
        assert!(hide_err.contains("unsupported glob: **/.git/**/hooks/**"));

        let readonly_err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec!["**/*/.git/**".to_string()],
            },
            config_path: None,
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap_err();
        assert!(readonly_err.contains("unsupported glob: **/*/.git/**"));

        let allow_write_err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: None,
            policy_family: Some(MutabilityFamily::ReadonlyRootAllowwrite),
            allow_write_rules: vec!["/project/**/.git/**/hooks/**".to_string()],
        })
        .unwrap_err();
        assert!(allow_write_err.contains("unsupported glob: /project/**/.git/**/hooks/**"));

        let config_path = source.join("screenfs.yaml");
        std::fs::write(
            &config_path,
            "mutability:\n  family: readonly-root-allowwrite\n  allow_write:\n    - '**/.git/**/hooks/**'\n",
        )
        .unwrap();
        let config_err = RuntimeConfig::from_launch(LaunchArgs {
            cli: CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                hide_rules: vec![],
                readonly_rules: vec![],
            },
            config_path: Some(config_path),
            policy_family: None,
            allow_write_rules: Vec::new(),
        })
        .unwrap_err();
        assert!(config_err.contains("unsupported glob: **/.git/**/hooks/**"));

        std::fs::remove_dir_all(source).unwrap();
    }

    fn test_dir() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("screenfs-config-test-{id}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
