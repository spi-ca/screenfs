use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::Duration;

use crate::cli::CliArgs;
use crate::matcher::{HideMatcher, mount_root_internal_prefix};
use crate::path::{RuleNormalizationContext, VirtualPath};

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub readonly: bool,
    pub matcher: HideMatcher,
    pub readonly_matcher: HideMatcher,
    pub attr_ttl: Duration,
    pub entry_ttl: Duration,
}

impl RuntimeConfig {
    pub fn from_cli(args: CliArgs) -> Result<Self, String> {
        let context = RuleNormalizationContext::from_environment(&args.source_root)?;
        let mut internal_prefixes = Vec::new();
        if let Some(prefix) = mount_root_internal_prefix(&args.source_root, &args.mount_root) {
            internal_prefixes.push(prefix);
        }
        let matcher = HideMatcher::new(
            args.hide_rules.iter().map(String::as_str),
            internal_prefixes,
            &context,
        )
        .map_err(|err| format!("invalid hide pattern: {err}"))?;
        let readonly_matcher = HideMatcher::new(
            args.readonly_rules.iter().map(String::as_str),
            Vec::new(),
            &context,
        )
        .map_err(|err| format!("invalid readonly pattern: {err}"))?;
        Ok(Self {
            source_root: args.source_root,
            mount_root: args.mount_root,
            readonly: args.readonly,
            matcher,
            readonly_matcher,
            attr_ttl: Duration::from_secs(1),
            entry_ttl: Duration::from_secs(1),
        })
    }

    pub fn is_hidden(&self, path: &VirtualPath) -> bool {
        self.matcher.is_hidden(path)
    }

    pub fn is_hidden_symlink_target(&self, link_path: &VirtualPath, raw_target: &OsStr) -> bool {
        self.matcher.matches_symlink_target(link_path, raw_target)
    }

    pub fn matches_readonly_rule(&self, path: &VirtualPath) -> bool {
        self.readonly_matcher.is_match(path)
    }

    pub fn is_readonly(&self, path: &VirtualPath) -> bool {
        self.readonly || self.matches_readonly_rule(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::ProcessEnvGuard;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn preserves_readonly_and_compiles_hide_and_readonly_rules() {
        let source = test_dir();
        let mount = source.join("mnt");
        std::fs::create_dir(&mount).unwrap();
        let cfg = RuntimeConfig::from_cli(CliArgs {
            source_root: source.clone(),
            mount_root: mount,
            readonly: true,
            hide_rules: vec!["/secret".to_string(), "/secret".to_string()],
            readonly_rules: vec!["/logs".to_string(), "**/*.lock".to_string()],
        })
        .unwrap();
        assert!(cfg.readonly);
        assert!(cfg.is_hidden(&VirtualPath::new("/secret")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/logs/app")));
        assert!(cfg.matches_readonly_rule(&VirtualPath::new("/tmp/file.lock")));
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
            readonly: true,
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
            let cfg = RuntimeConfig::from_cli(CliArgs {
                source_root: source.clone(),
                mount_root: mount.clone(),
                readonly: false,
                hide_rules: vec![
                    "../secrets".to_string(),
                    "./fixtures/**/*.pem".to_string(),
                    "~/locks/**/*.lock".to_string(),
                ],
                readonly_rules: vec![
                    "../secrets".to_string(),
                    "./fixtures/**/*.pem".to_string(),
                    "~/locks/**/*.lock".to_string(),
                ],
            })
            .unwrap();

            for path in [
                VirtualPath::new("/workspace/secrets"),
                VirtualPath::new("/workspace/app/fixtures/key.pem"),
                VirtualPath::new("/workspace/app/fixtures/nested/key.pem/chain"),
                VirtualPath::new("/home/tester/locks/app.lock"),
            ] {
                assert!(cfg.is_hidden(&path));
                assert!(cfg.matches_readonly_rule(&path));
            }
            let visible = VirtualPath::new("/workspace/app/fixtures/key.key");
            assert!(!cfg.is_hidden(&visible));
            assert!(!cfg.matches_readonly_rule(&visible));
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
            RuntimeConfig::from_cli(CliArgs {
                source_root: source.clone(),
                mount_root: mount,
                readonly: false,
                hide_rules: vec![],
                readonly_rules: vec!["~/.ssh".to_string()],
            })
            .unwrap_err()
        };
        assert!(err.contains("invalid readonly pattern"));
        assert!(err.contains("HOME is not set"));
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
