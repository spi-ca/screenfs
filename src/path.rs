use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RuleNormalizationContext {
    source_root: PathBuf,
    current_dir: PathBuf,
    home_dir: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VirtualPath(PathBuf);

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
        let mut current = source_root.clone();
        let parts = self
            .0
            .components()
            .filter_map(|component| match component {
                Component::Normal(part) => Some(part.to_os_string()),
                _ => None,
            })
            .collect::<Vec<_>>();
        let follow_count = if follow_final_symlink {
            parts.len()
        } else {
            parts.len().saturating_sub(1)
        };

        for part in parts.iter().take(follow_count) {
            current.push(part);
            current = current.canonicalize()?;
            if !current.starts_with(&source_root) {
                return Err(std::io::Error::from_raw_os_error(libc::ENOENT));
            }
        }

        if follow_final_symlink || parts.is_empty() {
            Ok(current)
        } else {
            Ok(current.join(&parts[parts.len() - 1]))
        }
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
