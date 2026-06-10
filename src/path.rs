use std::ffi::{OsStr, OsString};
use std::path::{Component, Path, PathBuf};

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

pub fn normalize_absolute(path: &Path) -> PathBuf {
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
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn normalizes_lexical_absolute_virtual_paths() {
        assert_eq!(VirtualPath::new("/a//b/./c").as_path(), Path::new("/a/b/c"));
        assert_eq!(VirtualPath::new("/a/b/../c").as_path(), Path::new("/a/c"));
        assert_eq!(VirtualPath::new("a/./b/../c").as_path(), Path::new("/a/c"));
    }

    #[test]
    fn clamps_parent_escape_at_virtual_root() {
        assert_eq!(
            VirtualPath::new("../../etc/passwd").as_path(),
            Path::new("/etc/passwd")
        );
        assert_eq!(
            VirtualPath::new("/../../etc/passwd").as_path(),
            Path::new("/etc/passwd")
        );
    }

    #[test]
    fn joins_child_and_resolves_symlink_targets_lexically() {
        let link = VirtualPath::new("/a/b/link");
        assert_eq!(
            link.join_child(OsStr::new("child")).as_path(),
            Path::new("/a/b/link/child")
        );
        assert_eq!(
            link.resolve_symlink_target(OsStr::new("../secret"))
                .unwrap()
                .as_path(),
            Path::new("/a/secret")
        );
        assert_eq!(
            link.resolve_symlink_target(OsStr::new("../../../secret"))
                .unwrap()
                .as_path(),
            Path::new("/secret")
        );
        assert_eq!(
            link.resolve_symlink_target(OsStr::new("/safe/../secret"))
                .unwrap()
                .as_path(),
            Path::new("/secret")
        );
    }

    #[test]
    fn resolves_root_symlink_targets_without_escaping_virtual_root() {
        let link = VirtualPath::new("/link");
        assert_eq!(
            link.resolve_symlink_target(OsStr::new("../secret"))
                .unwrap()
                .as_path(),
            Path::new("/secret")
        );
    }

    #[test]
    fn converts_virtual_path_to_source_relative_and_host_paths_without_root_escape() {
        assert_eq!(
            VirtualPath::root().to_source_relative_path(),
            PathBuf::new()
        );
        assert_eq!(
            VirtualPath::new("/a/../b").to_source_relative_path(),
            PathBuf::from("b")
        );
        assert_eq!(
            VirtualPath::root().to_source_path(Path::new("/src")),
            PathBuf::from("/src")
        );
        assert_eq!(
            VirtualPath::new("/a/../b").to_source_path(Path::new("/src")),
            PathBuf::from("/src/b")
        );
    }

    #[test]
    fn resolves_host_paths_within_source_root_without_changing_virtual_lexical_join() {
        let root = test_dir("resolve-host-safe");
        let source = root.join("source");
        std::fs::create_dir_all(source.join("real")).unwrap();
        std::fs::write(source.join("real/file.txt"), b"ok").unwrap();
        std::os::unix::fs::symlink("real", source.join("alias")).unwrap();

        assert_eq!(
            VirtualPath::new("/alias/file.txt")
                .to_source_path(&source)
                .as_path(),
            source.join("alias/file.txt").as_path()
        );
        assert_eq!(
            VirtualPath::new("/alias/file.txt")
                .resolve_host_path(&source, true)
                .unwrap(),
            source.join("real/file.txt").canonicalize().unwrap()
        );

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_following_symlinks_outside_source_root_but_allows_link_itself() {
        let root = test_dir("resolve-host-escape");
        let source = root.join("source");
        let outside = root.join("outside");
        std::fs::create_dir_all(&source).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("secret.txt"), b"secret").unwrap();
        std::os::unix::fs::symlink("../outside/secret.txt", source.join("escape-file")).unwrap();
        std::os::unix::fs::symlink("../outside", source.join("escape-dir")).unwrap();

        assert_eq!(
            VirtualPath::new("/escape-file")
                .resolve_host_path(&source, false)
                .unwrap(),
            source.join("escape-file")
        );
        let err = VirtualPath::new("/escape-file")
            .resolve_host_path(&source, true)
            .unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

        let err = VirtualPath::new("/escape-dir/secret.txt")
            .resolve_host_path(&source, true)
            .unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);

        std::fs::remove_dir_all(root).unwrap();
    }

    fn test_dir(label: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("holefs-path-{label}-{id}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
