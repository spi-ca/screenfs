use std::path::{Component, Path, PathBuf};

use crate::path::VirtualPath;

#[derive(Debug, Clone)]
pub struct HideMatcher {
    exact: Vec<VirtualPath>,
    prefixes: Vec<VirtualPath>,
    glob_rules: Vec<CompiledGlob>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompiledGlob {
    Basename(String),
    Suffix(String),
}

impl CompiledGlob {
    fn compile(rule: &str) -> Result<Self, String> {
        let Some(pattern) = rule.strip_prefix("**/") else {
            return Err(format!("unsupported glob: {rule}"));
        };
        if pattern.is_empty() || pattern.contains('/') || pattern.contains('?') {
            return Err(format!("unsupported glob: {rule}"));
        }
        if let Some(extension) = pattern.strip_prefix("*.") {
            if extension.is_empty() || extension.contains('*') {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok(Self::Suffix(format!(".{extension}")));
        }
        if pattern.contains('*') {
            return Err(format!("unsupported glob: {rule}"));
        }
        Ok(Self::Basename(pattern.to_string()))
    }

    fn matches_path_or_ancestor(&self, path: &Path) -> bool {
        path.components().any(|component| match component {
            Component::Normal(part) => {
                let part = part.to_string_lossy();
                match self {
                    Self::Basename(name) => part == name.as_str(),
                    Self::Suffix(suffix) => part.ends_with(suffix),
                }
            }
            _ => false,
        })
    }
}

impl HideMatcher {
    pub fn new<I, P>(rules: I, internal_prefixes: Vec<VirtualPath>) -> Result<Self, String>
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
                glob_rules.push(CompiledGlob::compile(rule)?);
            } else {
                if !rule.starts_with('/') {
                    return Err(format!("exact rules must be absolute: {rule}"));
                }
                let path = VirtualPath::new(rule);
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

    pub fn empty() -> Self {
        Self::new(std::iter::empty::<&str>(), Vec::new()).expect("empty matcher is valid")
    }

    pub fn is_match(&self, path: &VirtualPath) -> bool {
        self.exact.iter().any(|p| p == path)
            || self.prefixes.iter().any(|prefix| path.starts_with(prefix))
            || self
                .glob_rules
                .iter()
                .any(|rule| rule.matches_path_or_ancestor(path.as_path()))
    }

    pub fn matches_symlink_target(
        &self,
        link_path: &VirtualPath,
        raw_target: &std::ffi::OsStr,
    ) -> bool {
        link_path
            .resolve_symlink_target(raw_target)
            .is_some_and(|target| self.is_match(&target))
    }

    pub fn is_hidden(&self, path: &VirtualPath) -> bool {
        self.is_match(path)
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

fn canonicalize_or_normalize_absolute(path: &Path) -> Option<PathBuf> {
    if let Ok(canonical) = path.canonicalize() {
        return Some(canonical);
    }

    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };
    Some(normalize_absolute_host_path(&absolute))
}

fn normalize_absolute_host_path(path: &Path) -> PathBuf {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Prefix(_) | Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_os_string()),
        }
    }

    let mut out = PathBuf::from("/");
    for part in parts {
        out.push(part);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn matches_exact_rules_directory_prefixes_and_simple_globs() {
        let matcher = HideMatcher::new(
            [
                "/secret",
                "/config/auth.json",
                "**/.env",
                "**/*.pem",
                "**/*.key",
            ],
            Vec::new(),
        )
        .unwrap();

        assert!(matcher.is_hidden(&VirtualPath::new("/secret")));
        assert!(matcher.is_hidden(&VirtualPath::new("/secret/file")));
        assert!(!matcher.is_hidden(&VirtualPath::new("/secretish")));

        assert!(matcher.is_hidden(&VirtualPath::new("/config/auth.json")));
        assert!(!matcher.is_hidden(&VirtualPath::new("/config/auth.json.bak")));

        assert!(matcher.is_hidden(&VirtualPath::new("/app/.env")));
        assert!(matcher.is_hidden(&VirtualPath::new("/app/.env/local")));
        assert!(matcher.is_hidden(&VirtualPath::new("/certs/a.pem")));
        assert!(matcher.is_hidden(&VirtualPath::new("/certs/a.pem/chain")));
        assert!(matcher.is_hidden(&VirtualPath::new("/keys/id.key")));
        assert!(matcher.is_hidden(&VirtualPath::new("/keys/id.key/public")));
        assert!(!matcher.is_hidden(&VirtualPath::new("/public/a.txt")));
    }

    #[test]
    fn rejects_relative_exact_rules_and_unsupported_globs() {
        assert!(HideMatcher::new(["secret"], Vec::new()).is_err());
        assert!(HideMatcher::new(["**/secret?.pem"], Vec::new()).is_err());
        assert!(HideMatcher::new(["*.pem"], Vec::new()).is_err());
    }

    #[test]
    fn hides_mount_root_subtree_as_internal_prefix() {
        let tmp = test_dir();
        let source = tmp.as_path();
        let mount = source.join("mnt");
        fs::create_dir(&mount).unwrap();
        let prefix = mount_root_internal_prefix(source, &mount).unwrap();
        let matcher = HideMatcher::new(std::iter::empty::<&str>(), vec![prefix]).unwrap();
        assert!(matcher.is_hidden(&VirtualPath::new("/mnt")));
        assert!(matcher.is_hidden(&VirtualPath::new("/mnt/child")));
        assert!(!matcher.is_hidden(&VirtualPath::new("/other")));
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
        let matcher = HideMatcher::new(["/hidden", "**/*.pem"], Vec::new()).unwrap();
        let exact_link = VirtualPath::new("/visible/link");
        let glob_link = VirtualPath::new("/visible/nested/link");

        assert!(matcher.matches_symlink_target(&exact_link, std::ffi::OsStr::new("../hidden")));
        assert!(matcher.matches_symlink_target(
            &glob_link,
            std::ffi::OsStr::new("../../secret.pem/private")
        ));
    }

    fn test_dir() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("holefs-test-{id}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
