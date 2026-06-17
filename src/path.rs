//! Virtual path handling for the ScreenFS view.
//!
//! Matching and policy decisions use lexical virtual absolute paths, not host
//! canonical paths. Rule normalization may canonicalize launch-time host paths
//! to rebase cwd/HOME forms into the virtual view; request-time host paths are
//! then confined beneath `source_root` before delegation.

use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};
#[cfg(feature = "perf-counters")]
use std::time::{Duration, Instant};

/// Launch-time context used to normalize policy rules into virtual paths.
#[derive(Debug, Clone)]
pub struct RuleNormalizationContext {
    source_root: PathBuf,
    current_dir: PathBuf,
    home_dir: Option<PathBuf>,
}

/// Lexically normalized absolute path inside the ScreenFS virtual view.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VirtualPath(PathBuf);

#[cfg(feature = "perf-counters")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ResolveHostPathMetrics {
    pub(crate) component_walk: Duration,
    pub(crate) canonicalize_total: Duration,
    pub(crate) canonicalize_count: u64,
    pub(crate) source_root_confinement_total: Duration,
    pub(crate) source_root_confinement_count: u64,
}

// VirtualPath methods keep path math lexical and independent from host canonicalization.
impl VirtualPath {
    pub fn root() -> Self {
        Self(PathBuf::from("/"))
    }

    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(normalize_absolute(path.as_ref()))
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn join_child(&self, name: &OsStr) -> Self {
        let mut raw = self.0.clone();
        raw.push(name);
        Self::new(raw)
    }

    pub fn starts_with(&self, other: &VirtualPath) -> bool {
        if other.0 == Path::new("/") {
            return true;
        }
        self.0 == other.0 || self.0.starts_with(&other.0)
    }

    pub fn resolve_symlink_target(&self, raw_target: &OsStr) -> Option<Self> {
        let target = Path::new(raw_target);
        let parent = self.0.parent().unwrap_or(Path::new("/"));
        Some(Self(normalize_absolute_from(target, parent)))
    }

    pub fn to_source_relative_path(&self) -> PathBuf {
        let mut out = PathBuf::new();
        for component in self.0.components() {
            if let Component::Normal(part) = component {
                out.push(part);
            }
        }
        out
    }

    pub fn to_source_path(&self, source_root: &Path) -> PathBuf {
        let relative = self.to_source_relative_path();
        if relative.as_os_str().is_empty() {
            source_root.to_path_buf()
        } else {
            source_root.join(relative)
        }
    }

    pub fn resolve_host_path(
        &self,
        source_root: &Path,
        follow_final_symlink: bool,
    ) -> std::io::Result<PathBuf> {
        let source_root = source_root.canonicalize()?;
        self.resolve_host_path_from_canonical_source_root(&source_root, follow_final_symlink)
    }

    pub(crate) fn resolve_host_path_from_canonical_source_root(
        &self,
        source_root: &Path,
        follow_final_symlink: bool,
    ) -> std::io::Result<PathBuf> {
        #[cfg(feature = "perf-counters")]
        {
            self.resolve_host_path_from_canonical_source_root_with_metrics(
                source_root,
                follow_final_symlink,
            )
            .map(|(path, _metrics)| path)
        }
        #[cfg(not(feature = "perf-counters"))]
        {
            let mut current = source_root.to_path_buf();
            let mut parts = self
                .0
                .components()
                .filter_map(|component| match component {
                    Component::Normal(part) => Some(part),
                    _ => None,
                })
                .peekable();

            while let Some(part) = parts.next() {
                if !follow_final_symlink && parts.peek().is_none() {
                    return Ok(current.join(part));
                }
                current.push(part);
                current = current.canonicalize()?;
                if !current.starts_with(source_root) {
                    return Err(std::io::Error::from_raw_os_error(libc::ENOENT));
                }
            }

            Ok(current)
        }
    }

    #[cfg(feature = "perf-counters")]
    pub(crate) fn resolve_host_path_from_canonical_source_root_with_metrics(
        &self,
        source_root: &Path,
        follow_final_symlink: bool,
    ) -> std::io::Result<(PathBuf, ResolveHostPathMetrics)> {
        let mut metrics = ResolveHostPathMetrics::default();
        let walk_start = Instant::now();
        let mut current = source_root.to_path_buf();
        let mut parts = self
            .0
            .components()
            .filter_map(|component| match component {
                Component::Normal(part) => Some(part),
                _ => None,
            })
            .peekable();

        let result = (|| {
            while let Some(part) = parts.next() {
                if !follow_final_symlink && parts.peek().is_none() {
                    return Ok(current.join(part));
                }
                current.push(part);

                let canonicalize_start = Instant::now();
                current = current.canonicalize()?;
                metrics.canonicalize_total += canonicalize_start.elapsed();
                metrics.canonicalize_count += 1;

                let confinement_start = Instant::now();
                let confined = current.starts_with(source_root);
                metrics.source_root_confinement_total += confinement_start.elapsed();
                metrics.source_root_confinement_count += 1;
                if !confined {
                    return Err(std::io::Error::from_raw_os_error(libc::ENOENT));
                }
            }

            Ok(current)
        })();
        metrics.component_walk = walk_start.elapsed();
        result.map(|path| (path, metrics))
    }
}

impl std::fmt::Display for VirtualPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl AsRef<Path> for VirtualPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Borrow<Path> for VirtualPath {
    fn borrow(&self) -> &Path {
        self.as_path()
    }
}

// RuleNormalizationContext captures launch environment needed for cwd/HOME rebasing.
impl RuleNormalizationContext {
    pub fn from_environment(source_root: &Path) -> Result<Self, String> {
        let current_dir = std::env::current_dir()
            .map_err(|err| format!("failed to read current directory: {err}"))?;
        Self::new(
            source_root,
            &current_dir,
            std::env::var_os("HOME").map(PathBuf::from),
        )
    }

    pub fn new(
        source_root: &Path,
        current_dir: &Path,
        home_dir: Option<PathBuf>,
    ) -> Result<Self, String> {
        let current_dir = canonicalize_or_normalize_absolute_with_base(current_dir, Path::new("/"))
            .ok_or_else(|| {
                format!(
                    "failed to normalize current directory: {}",
                    current_dir.display()
                )
            })?;
        let source_root = canonicalize_or_normalize_absolute_with_base(source_root, &current_dir)
            .ok_or_else(|| {
            format!("failed to normalize source root: {}", source_root.display())
        })?;
        let home_dir = match home_dir {
            Some(path) => Some(
                canonicalize_or_normalize_absolute_with_base(&path, &current_dir)
                    .ok_or_else(|| "failed to normalize HOME".to_string())?,
            ),
            None => None,
        };
        Ok(Self {
            source_root,
            current_dir,
            home_dir,
        })
    }
}

// Normalize a user-facing policy rule into the virtual path space.
pub fn normalize_rule_path(
    raw: &str,
    ctx: &RuleNormalizationContext,
) -> Result<VirtualPath, String> {
    if raw.is_empty() {
        return Err("empty rule path".to_string());
    }
    if raw.starts_with('/') {
        return Ok(VirtualPath::new(raw));
    }

    let host_path = expand_rule_host_path(raw, ctx)?;
    rebase_host_path_into_virtual(&host_path, &ctx.source_root)
        .map_err(|_| format!("rule path resolves outside source_root: {raw}"))
}

// Expand cwd/HOME-relative rule forms as host paths before rebasing to virtual paths.
fn expand_rule_host_path(raw: &str, ctx: &RuleNormalizationContext) -> Result<PathBuf, String> {
    if raw == "~" || raw.starts_with("~/") {
        let Some(home_dir) = &ctx.home_dir else {
            return Err(format!("HOME is not set for rule: {raw}"));
        };
        let suffix = raw.strip_prefix('~').expect("tilde-prefixed rule");
        let suffix = suffix.strip_prefix('/').unwrap_or("");
        let path = if suffix.is_empty() {
            home_dir.clone()
        } else {
            normalize_absolute_host_path(&home_dir.join(suffix))
        };
        return Ok(path);
    }
    if raw.starts_with('~') {
        return Err(format!("unsupported home expansion: {raw}"));
    }
    Ok(normalize_absolute_host_path(&ctx.current_dir.join(raw)))
}

fn rebase_host_path_into_virtual(host_path: &Path, source_root: &Path) -> Result<VirtualPath, ()> {
    let relative = host_path.strip_prefix(source_root).map_err(|_| ())?;
    if relative.as_os_str().is_empty() {
        Ok(VirtualPath::root())
    } else {
        let mut virtual_path = PathBuf::from("/");
        virtual_path.push(relative);
        Ok(VirtualPath::new(virtual_path))
    }
}

// Lexical normalization clamps parent traversal at the virtual root.
pub fn normalize_absolute(path: &Path) -> PathBuf {
    normalize_absolute_from(path, Path::new("/"))
}

pub fn canonicalize_or_normalize_absolute(path: &Path) -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    canonicalize_or_normalize_absolute_with_base(path, &cwd)
}

fn canonicalize_or_normalize_absolute_with_base(path: &Path, base: &Path) -> Option<PathBuf> {
    if let Ok(canonical) = path.canonicalize() {
        return Some(canonical);
    }

    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    };
    Some(normalize_absolute_host_path(&absolute))
}

pub fn normalize_absolute_host_path(path: &Path) -> PathBuf {
    normalize_absolute_from(path, Path::new("/"))
}

fn normalize_absolute_from(path: &Path, base: &Path) -> PathBuf {
    let mut parts: Vec<OsString> = Vec::new();

    if !path.is_absolute() {
        push_normalized_components(&mut parts, base.components());
    }
    push_normalized_components(&mut parts, path.components());

    let mut out = PathBuf::from("/");
    for part in parts {
        out.push(part);
    }
    out
}

fn push_normalized_components<'a>(
    parts: &mut Vec<OsString>,
    components: impl Iterator<Item = Component<'a>>,
) {
    for component in components {
        match component {
            Component::RootDir | Component::Prefix(_) | Component::CurDir => {}
            Component::ParentDir => {
                parts.pop();
            }
            Component::Normal(part) => parts.push(part.to_os_string()),
        }
    }
}

#[cfg(test)]
pub(crate) fn process_env_lock() -> &'static std::sync::Mutex<()> {
    use std::sync::{Mutex, OnceLock};

    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(test)]
// ProcessEnvGuard serializes environment-sensitive tests that mutate cwd/HOME.
pub(crate) struct ProcessEnvGuard {
    _lock: std::sync::MutexGuard<'static, ()>,
    original_cwd: PathBuf,
    original_home: Option<OsString>,
}

#[cfg(test)]
impl ProcessEnvGuard {
    pub(crate) fn new(current_dir: &Path, home_dir: Option<&Path>) -> Self {
        let lock = process_env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let original_cwd = std::env::current_dir().expect("current dir available");
        let original_home = std::env::var_os("HOME");
        std::env::set_current_dir(current_dir).expect("set current dir");
        match home_dir {
            Some(path) => unsafe {
                std::env::set_var("HOME", path);
            },
            None => unsafe {
                std::env::remove_var("HOME");
            },
        }
        Self {
            _lock: lock,
            original_cwd,
            original_home,
        }
    }
}

#[cfg(test)]
impl Drop for ProcessEnvGuard {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.original_cwd).expect("restore current dir");
        match &self.original_home {
            Some(path) => unsafe {
                std::env::set_var("HOME", path);
            },
            None => unsafe {
                std::env::remove_var("HOME");
            },
        }
    }
}

#[cfg(test)]
#[path = "path_tests.rs"]
mod tests;
