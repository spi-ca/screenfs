use std::path::{Component, Path, PathBuf};

use crate::path::{
    RuleNormalizationContext, VirtualPath, canonicalize_or_normalize_absolute, normalize_rule_path,
};

#[derive(Debug, Clone)]
pub struct PathRuleMatcher {
    exact: Vec<VirtualPath>,
    prefixes: Vec<VirtualPath>,
    glob_rules: Vec<CompiledGlob>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatcherScope {
    Hide,
    Readonly,
    AllowWrite,
}

impl MatcherScope {
    fn label(self) -> &'static str {
        match self {
            Self::Hide => "hide",
            Self::Readonly => "readonly",
            Self::AllowWrite => "allow-write",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompiledGlob {
    Basename {
        prefix: Option<VirtualPath>,
        recursive: bool,
        name: String,
    },
    Prefix {
        prefix: Option<VirtualPath>,
        recursive: bool,
        name_prefix: String,
    },
    Suffix {
        prefix: Option<VirtualPath>,
        recursive: bool,
        suffix: String,
    },
}

impl CompiledGlob {
    fn compile(rule: &str, context: &RuleNormalizationContext) -> Result<Self, String> {
        let (prefix, pattern, recursive) = split_supported_glob(rule)?;
        let prefix = prefix
            .map(|raw| normalize_rule_path(raw, context))
            .transpose()?;
        if pattern.is_empty() || pattern.contains('/') || pattern.contains('?') {
            return Err(format!("unsupported glob: {rule}"));
        }
        if let Some(extension) = pattern.strip_prefix("*.") {
            if extension.is_empty() || extension.contains('*') {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok(Self::Suffix {
                prefix,
                recursive,
                suffix: format!(".{extension}"),
            });
        }
        if let Some(name_prefix) = pattern.strip_suffix('*') {
            if name_prefix.is_empty() || name_prefix.contains('*') {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok(Self::Prefix {
                prefix,
                recursive,
                name_prefix: name_prefix.to_string(),
            });
        }
        if pattern.contains('*') {
            return Err(format!("unsupported glob: {rule}"));
        }
        Ok(Self::Basename {
            prefix,
            recursive,
            name: pattern.to_string(),
        })
    }

    fn matches_path_or_ancestor(&self, path: &VirtualPath) -> bool {
        let (candidate, recursive) = match self {
            Self::Basename {
                prefix, recursive, ..
            }
            | Self::Prefix {
                prefix, recursive, ..
            }
            | Self::Suffix {
                prefix, recursive, ..
            } => match prefix.as_ref() {
                Some(prefix) => match path.as_path().strip_prefix(prefix.as_path()) {
                    Ok(relative) => (relative, *recursive),
                    Err(_) => return false,
                },
                None => (path.as_path(), *recursive),
            },
        };

        let mut components = candidate.components();
        if recursive {
            components.any(|component| self.matches_component(component))
        } else {
            components
                .next()
                .is_some_and(|component| self.matches_component(component))
        }
    }

    fn matches_component(&self, component: Component<'_>) -> bool {
        match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                match self {
                    Self::Basename { name, .. } => part == name.as_str(),
                    Self::Prefix { name_prefix, .. } => part.starts_with(name_prefix),
                    Self::Suffix { suffix, .. } => part.ends_with(suffix),
                }
            }
            _ => false,
        }
    }
}

impl PathRuleMatcher {
    pub fn new<I, P>(
        rules: I,
        internal_prefixes: Vec<VirtualPath>,
        context: &RuleNormalizationContext,
    ) -> Result<Self, String>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        let mut exact = Vec::new();
        let mut prefixes = internal_prefixes;
        let mut glob_rules = Vec::new();

        for rule in rules {
            let rule = rule.as_ref();
            if looks_like_glob(rule) {
                glob_rules.push(CompiledGlob::compile(rule, context)?);
            } else {
                let path = normalize_rule_path(rule, context)?;
                exact.push(path.clone());
                prefixes.push(path);
            }
        }

        Ok(Self {
            exact,
            prefixes,
            glob_rules,
        })
    }

    pub fn compile<I, P>(
        scope: MatcherScope,
        rules: I,
        internal_prefixes: Vec<VirtualPath>,
        context: &RuleNormalizationContext,
    ) -> Result<Self, String>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        Self::new(rules, internal_prefixes, context)
            .map_err(|err| format!("invalid {} pattern: {err}", scope.label()))
    }

    pub fn empty() -> Self {
        Self {
            exact: Vec::new(),
            prefixes: Vec::new(),
            glob_rules: Vec::new(),
        }
    }

    pub fn matches_path(&self, path: &VirtualPath) -> bool {
        self.exact.iter().any(|p| p == path)
            || self.prefixes.iter().any(|prefix| path.starts_with(prefix))
            || self
                .glob_rules
                .iter()
                .any(|rule| rule.matches_path_or_ancestor(path))
    }

    pub fn matches_symlink_target(
        &self,
        link_path: &VirtualPath,
        raw_target: &std::ffi::OsStr,
    ) -> bool {
        link_path
            .resolve_symlink_target(raw_target)
            .is_some_and(|target| self.matches_path(&target))
    }

    pub fn is_hidden(&self, path: &VirtualPath) -> bool {
        self.matches_path(path)
    }

    pub fn is_hidden_symlink_target(
        &self,
        link_path: &VirtualPath,
        raw_target: &std::ffi::OsStr,
    ) -> bool {
        self.matches_symlink_target(link_path, raw_target)
    }
}

pub fn mount_root_internal_prefix(source_root: &Path, mount_root: &Path) -> Option<VirtualPath> {
    let source = source_root.canonicalize().ok()?;
    let mount = canonicalize_or_normalize_absolute(mount_root)?;
    let relative = mount.strip_prefix(&source).ok()?;
    if relative.as_os_str().is_empty() {
        return Some(VirtualPath::root());
    }
    let mut virtual_path = PathBuf::from("/");
    virtual_path.push(relative);
    Some(VirtualPath::new(virtual_path))
}

fn looks_like_glob(rule: &str) -> bool {
    rule.contains('*') || rule.contains('?')
}

fn split_supported_glob(rule: &str) -> Result<(Option<&str>, &str, bool), String> {
    if let Some(pattern) = rule.strip_prefix("**/") {
        return Ok((None, pattern, true));
    }
    if let Some(index) = rule.find("/**/") {
        let prefix = &rule[..index];
        if prefix.is_empty() || prefix.contains('*') || prefix.contains('?') {
            return Err(format!("unsupported glob: {rule}"));
        }
        let pattern = &rule[index + 4..];
        return Ok((Some(prefix), pattern, true));
    }
    if let Some(index) = rule.rfind('/') {
        let pattern = &rule[index + 1..];
        if is_supported_direct_child_glob(pattern) {
            let prefix = if index == 0 { "/" } else { &rule[..index] };
            if prefix.contains('*') || prefix.contains('?') {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok((Some(prefix), pattern, false));
        }
    }
    Err(format!("unsupported glob: {rule}"))
}

fn is_supported_direct_child_glob(pattern: &str) -> bool {
    is_supported_direct_basename_prefix_glob(pattern)
        || is_supported_direct_child_suffix_glob(pattern)
}

fn is_supported_direct_basename_prefix_glob(pattern: &str) -> bool {
    pattern
        .strip_suffix('*')
        .is_some_and(|prefix| !prefix.is_empty() && !prefix.contains('*') && !prefix.contains('?'))
        && !pattern.starts_with("*.")
}

fn is_supported_direct_child_suffix_glob(pattern: &str) -> bool {
    pattern
        .strip_prefix("*.")
        .is_some_and(|suffix| !suffix.is_empty() && !suffix.contains('*') && !suffix.contains('?'))
}

#[cfg(test)]
mod tests {
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
        let matcher =
            PathRuleMatcher::new(std::iter::empty::<&str>(), vec![prefix], &context).unwrap();
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
            matcher.matches_symlink_target(
                &glob_link,
                std::ffi::OsStr::new("../../secret.pem/private")
            )
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
}
