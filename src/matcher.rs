use std::cmp::Ordering;
use std::path::{Component, Path, PathBuf};

use crate::path::{
    RuleNormalizationContext, VirtualPath, canonicalize_or_normalize_absolute, normalize_rule_path,
};

#[derive(Debug, Clone)]
pub struct PathRuleMatcher {
    exact: Vec<VirtualPath>,
    prefixes: Vec<VirtualPath>,
    glob_rules: Vec<CompiledGlob>,
    descriptors: Vec<RuleDescriptor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuleSpecificity {
    prefix_components: usize,
    kind: RuleSpecificityKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleSpecificityKind {
    Exact,
    PrefixSubtree,
    DirectChildGlob {
        tail_rank: u8,
        tail_len: usize,
    },
    RecursiveGlob {
        tail_rank: u8,
        tail_len: usize,
    },
    RecursiveLiteralSubtree {
        tail_components: usize,
        tail_len: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RuleSpecificitySortKey {
    category_rank: u8,
    primary_components: usize,
    secondary_components: usize,
    tail_rank: u8,
    tail_len: usize,
}

impl RuleSpecificity {
    fn exact_or_prefix(path: &VirtualPath, exact: bool) -> Self {
        Self {
            prefix_components: component_count(path),
            kind: if exact {
                RuleSpecificityKind::Exact
            } else {
                RuleSpecificityKind::PrefixSubtree
            },
        }
    }

    fn glob(prefix: Option<&VirtualPath>, recursive: bool, pattern: &GlobPattern) -> Self {
        let (tail_rank, tail_len) = pattern.specificity_tail();
        Self {
            prefix_components: prefix.map(component_count).unwrap_or(0),
            kind: if recursive {
                RuleSpecificityKind::RecursiveGlob {
                    tail_rank,
                    tail_len,
                }
            } else {
                RuleSpecificityKind::DirectChildGlob {
                    tail_rank,
                    tail_len,
                }
            },
        }
    }

    fn recursive_literal_subtree(prefix: Option<&VirtualPath>, tail: &LiteralPathTail) -> Self {
        Self {
            prefix_components: prefix.map(component_count).unwrap_or(0),
            kind: RuleSpecificityKind::RecursiveLiteralSubtree {
                tail_components: tail.component_count(),
                tail_len: tail.total_len(),
            },
        }
    }

    fn sort_key(self) -> RuleSpecificitySortKey {
        // Descendant-subtree globs compare by literal-tail depth before normalized prefix length
        // so `**/.git/hooks/**` outranks `/workspace/**/.git/**` for paths that match both.
        match self.kind {
            RuleSpecificityKind::Exact => RuleSpecificitySortKey {
                category_rank: 4,
                primary_components: self.prefix_components,
                secondary_components: 0,
                tail_rank: 0,
                tail_len: 0,
            },
            RuleSpecificityKind::PrefixSubtree => RuleSpecificitySortKey {
                category_rank: 3,
                primary_components: self.prefix_components,
                secondary_components: 0,
                tail_rank: 0,
                tail_len: 0,
            },
            RuleSpecificityKind::DirectChildGlob {
                tail_rank,
                tail_len,
            } => RuleSpecificitySortKey {
                category_rank: 2,
                primary_components: self.prefix_components,
                secondary_components: 0,
                tail_rank,
                tail_len,
            },
            RuleSpecificityKind::RecursiveGlob {
                tail_rank,
                tail_len,
            } => RuleSpecificitySortKey {
                category_rank: 1,
                primary_components: 1,
                secondary_components: self.prefix_components,
                tail_rank,
                tail_len,
            },
            RuleSpecificityKind::RecursiveLiteralSubtree {
                tail_components,
                tail_len,
            } => RuleSpecificitySortKey {
                category_rank: 1,
                primary_components: tail_components,
                secondary_components: self.prefix_components,
                tail_rank: 3,
                tail_len,
            },
        }
    }
}

impl Ord for RuleSpecificity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for RuleSpecificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleDescriptor {
    anchor: VirtualPath,
    specificity: RuleSpecificity,
    target: RuleTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleTarget {
    Subtree,
    Glob {
        recursive: bool,
        pattern: GlobPattern,
    },
    RecursiveLiteralSubtree {
        tail: LiteralPathTail,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GlobPattern {
    Basename(String),
    Prefix(String),
    Suffix(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LiteralPathTail(Vec<String>);

impl LiteralPathTail {
    fn parse(rule: &str, raw_tail: &str) -> Result<Self, String> {
        let mut components = Vec::new();
        for component in raw_tail.split('/') {
            if component.is_empty()
                || component == "."
                || component == ".."
                || component.contains('*')
                || component.contains('?')
            {
                return Err(format!("unsupported glob: {rule}"));
            }
            components.push(component.to_string());
        }
        if components.is_empty() {
            return Err(format!("unsupported glob: {rule}"));
        }
        Ok(Self(components))
    }

    fn component_count(&self) -> usize {
        self.0.len()
    }

    fn total_len(&self) -> usize {
        self.0.iter().map(String::len).sum()
    }

    fn first(&self) -> Option<&str> {
        self.0.first().map(String::as_str)
    }

    fn contains_name_matching(&self, pattern: &GlobPattern) -> bool {
        self.0.iter().any(|name| pattern.matches_name(name))
    }

    fn matches_path(&self, path: &Path) -> bool {
        let components = path_component_names(path);
        let tail = self.0.as_slice();
        components.len() >= tail.len()
            && components.windows(tail.len()).any(|window| window == tail)
    }

    fn is_contiguous_subsequence_of(&self, other: &Self) -> bool {
        let tail = self.0.as_slice();
        other.0.len() >= tail.len() && other.0.windows(tail.len()).any(|window| window == tail)
    }
}

impl GlobPattern {
    fn matches_component(&self, component: Component<'_>) -> bool {
        match component {
            Component::Normal(part) => self.matches_name(&part.to_string_lossy()),
            _ => false,
        }
    }

    fn matches_name(&self, name: &str) -> bool {
        match self {
            Self::Basename(expected) => name == expected.as_str(),
            Self::Prefix(name_prefix) => name.starts_with(name_prefix),
            Self::Suffix(suffix) => name.ends_with(suffix),
        }
    }

    fn contains_pattern(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Basename(left), Self::Basename(right)) => left == right,
            (Self::Prefix(prefix), Self::Basename(name)) => name.starts_with(prefix),
            (Self::Prefix(prefix), Self::Prefix(other_prefix)) => other_prefix.starts_with(prefix),
            (Self::Suffix(suffix), Self::Basename(name)) => name.ends_with(suffix),
            (Self::Suffix(suffix), Self::Suffix(other_suffix)) => other_suffix.ends_with(suffix),
            _ => false,
        }
    }

    fn specificity_tail(&self) -> (u8, usize) {
        match self {
            Self::Basename(name) => (3, name.len()),
            Self::Prefix(prefix) => (1, prefix.len()),
            Self::Suffix(suffix) => (1, suffix.len()),
        }
    }
}

impl RuleDescriptor {
    pub fn anchor(&self) -> &VirtualPath {
        &self.anchor
    }

    pub fn specificity(&self) -> RuleSpecificity {
        self.specificity
    }

    pub fn has_less_specific_ancestor_of(&self, other: &Self) -> bool {
        self.specificity < other.specificity && self.contains_target_set(other)
    }

    fn contains_target_set(&self, other: &Self) -> bool {
        match (&self.target, &other.target) {
            (RuleTarget::Subtree, _) => other.anchor.starts_with(&self.anchor),
            (RuleTarget::Glob { recursive, pattern }, RuleTarget::Subtree) => {
                subtree_is_inside_glob(&self.anchor, *recursive, pattern, &other.anchor)
            }
            (
                RuleTarget::Glob { recursive, pattern },
                RuleTarget::Glob {
                    recursive: other_recursive,
                    pattern: other_pattern,
                },
            ) => {
                pattern.contains_pattern(other_pattern)
                    && other.anchor.starts_with(&self.anchor)
                    && (*recursive || (!other_recursive && other.anchor == self.anchor))
            }
            (
                RuleTarget::Glob { recursive, pattern },
                RuleTarget::RecursiveLiteralSubtree { tail },
            ) => glob_contains_recursive_literal_subtree(
                &self.anchor,
                *recursive,
                pattern,
                &other.anchor,
                tail,
            ),
            (RuleTarget::RecursiveLiteralSubtree { tail }, RuleTarget::Subtree) => {
                subtree_is_inside_recursive_literal_subtree(&self.anchor, tail, &other.anchor)
            }
            (
                RuleTarget::RecursiveLiteralSubtree { tail },
                RuleTarget::RecursiveLiteralSubtree { tail: other_tail },
            ) => recursive_literal_subtree_contains_recursive_literal_subtree(
                &self.anchor,
                tail,
                &other.anchor,
                other_tail,
            ),
            (RuleTarget::RecursiveLiteralSubtree { .. }, RuleTarget::Glob { .. }) => false,
        }
    }
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
    RecursiveLiteralSubtree {
        prefix: Option<VirtualPath>,
        tail: LiteralPathTail,
    },
}

impl CompiledGlob {
    fn compile(rule: &str, context: &RuleNormalizationContext) -> Result<Self, String> {
        if let Some((prefix, tail)) = split_supported_recursive_literal_subtree_glob(rule)? {
            let prefix = prefix
                .map(|raw| normalize_rule_path(raw, context))
                .transpose()?;
            if let Some(name) = tail.first().filter(|_| tail.component_count() == 1) {
                return Ok(Self::Basename {
                    prefix,
                    recursive: true,
                    name: name.to_string(),
                });
            }
            return Ok(Self::RecursiveLiteralSubtree { prefix, tail });
        }

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

    fn descriptor(&self) -> RuleDescriptor {
        match self {
            Self::Basename {
                prefix, recursive, ..
            }
            | Self::Prefix {
                prefix, recursive, ..
            }
            | Self::Suffix {
                prefix, recursive, ..
            } => {
                let pattern = self.pattern().expect("pattern-backed glob");
                RuleDescriptor {
                    anchor: prefix.clone().unwrap_or_else(VirtualPath::root),
                    specificity: RuleSpecificity::glob(prefix.as_ref(), *recursive, &pattern),
                    target: RuleTarget::Glob {
                        recursive: *recursive,
                        pattern,
                    },
                }
            }
            Self::RecursiveLiteralSubtree { prefix, tail } => RuleDescriptor {
                anchor: prefix.clone().unwrap_or_else(VirtualPath::root),
                specificity: RuleSpecificity::recursive_literal_subtree(prefix.as_ref(), tail),
                target: RuleTarget::RecursiveLiteralSubtree { tail: tail.clone() },
            },
        }
    }

    fn match_specificity(&self, path: &VirtualPath) -> Option<RuleSpecificity> {
        match self {
            Self::Basename {
                prefix, recursive, ..
            }
            | Self::Prefix {
                prefix, recursive, ..
            }
            | Self::Suffix {
                prefix, recursive, ..
            } => {
                let candidate = match prefix.as_ref() {
                    Some(prefix) => match path.as_path().strip_prefix(prefix.as_path()) {
                        Ok(relative) => relative,
                        Err(_) => return None,
                    },
                    None => path.as_path(),
                };
                let mut components = candidate.components();
                let matched = if *recursive {
                    components.any(|component| self.matches_component(component))
                } else {
                    components
                        .next()
                        .is_some_and(|component| self.matches_component(component))
                };
                matched.then(|| self.descriptor().specificity)
            }
            Self::RecursiveLiteralSubtree { prefix, tail } => {
                let candidate = match prefix.as_ref() {
                    Some(prefix) => match path.as_path().strip_prefix(prefix.as_path()) {
                        Ok(relative) => relative,
                        Err(_) => return None,
                    },
                    None => path.as_path(),
                };
                tail.matches_path(candidate)
                    .then(|| RuleSpecificity::recursive_literal_subtree(prefix.as_ref(), tail))
            }
        }
    }

    fn matches_component(&self, component: Component<'_>) -> bool {
        self.pattern()
            .is_some_and(|pattern| pattern.matches_component(component))
    }

    fn pattern(&self) -> Option<GlobPattern> {
        match self {
            Self::Basename { name, .. } => Some(GlobPattern::Basename(name.clone())),
            Self::Prefix { name_prefix, .. } => Some(GlobPattern::Prefix(name_prefix.clone())),
            Self::Suffix { suffix, .. } => Some(GlobPattern::Suffix(suffix.clone())),
            Self::RecursiveLiteralSubtree { .. } => None,
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
        let mut descriptors = Vec::new();

        for prefix in &prefixes {
            descriptors.push(RuleDescriptor {
                anchor: prefix.clone(),
                specificity: RuleSpecificity::exact_or_prefix(prefix, false),
                target: RuleTarget::Subtree,
            });
        }

        for rule in rules {
            let rule = rule.as_ref();
            if looks_like_glob(rule) {
                let glob = CompiledGlob::compile(rule, context)?;
                descriptors.push(glob.descriptor());
                glob_rules.push(glob);
            } else {
                let path = normalize_rule_path(rule, context)?;
                descriptors.push(RuleDescriptor {
                    anchor: path.clone(),
                    specificity: RuleSpecificity::exact_or_prefix(&path, false),
                    target: RuleTarget::Subtree,
                });
                exact.push(path.clone());
                prefixes.push(path);
            }
        }

        Ok(Self {
            exact,
            prefixes,
            glob_rules,
            descriptors,
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
            descriptors: Vec::new(),
        }
    }

    pub fn best_specificity(&self, path: &VirtualPath) -> Option<RuleSpecificity> {
        let exact_matches = self
            .exact
            .iter()
            .filter(|p| *p == path)
            .map(|p| RuleSpecificity::exact_or_prefix(p, true));
        let prefix_matches = self
            .prefixes
            .iter()
            .filter(|prefix| path.starts_with(prefix))
            .map(|prefix| RuleSpecificity::exact_or_prefix(prefix, false));
        let glob_matches = self
            .glob_rules
            .iter()
            .filter_map(|rule| rule.match_specificity(path));
        exact_matches
            .chain(prefix_matches)
            .chain(glob_matches)
            .max()
    }

    pub fn descriptors(&self) -> &[RuleDescriptor] {
        &self.descriptors
    }

    pub fn matches_path(&self, path: &VirtualPath) -> bool {
        self.best_specificity(path).is_some()
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

fn component_count(path: &VirtualPath) -> usize {
    path.as_path()
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count()
}

fn path_component_names(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect()
}

fn subtree_is_inside_glob(
    glob_anchor: &VirtualPath,
    recursive: bool,
    pattern: &GlobPattern,
    subtree_anchor: &VirtualPath,
) -> bool {
    let Ok(relative) = subtree_anchor.as_path().strip_prefix(glob_anchor.as_path()) else {
        return false;
    };
    let mut components = relative.components();
    if recursive {
        components.any(|component| pattern.matches_component(component))
    } else {
        components
            .next()
            .is_some_and(|component| pattern.matches_component(component))
    }
}

fn glob_contains_recursive_literal_subtree(
    glob_anchor: &VirtualPath,
    recursive: bool,
    pattern: &GlobPattern,
    other_anchor: &VirtualPath,
    other_tail: &LiteralPathTail,
) -> bool {
    let Ok(relative_anchor) = other_anchor.as_path().strip_prefix(glob_anchor.as_path()) else {
        return false;
    };
    let mut relative_components = relative_anchor.components();
    if recursive {
        relative_components.any(|component| pattern.matches_component(component))
            || other_tail.contains_name_matching(pattern)
    } else {
        relative_components
            .next()
            .is_some_and(|component| pattern.matches_component(component))
    }
}

fn subtree_is_inside_recursive_literal_subtree(
    glob_anchor: &VirtualPath,
    tail: &LiteralPathTail,
    subtree_anchor: &VirtualPath,
) -> bool {
    let Ok(relative) = subtree_anchor.as_path().strip_prefix(glob_anchor.as_path()) else {
        return false;
    };
    tail.matches_path(relative)
}

fn recursive_literal_subtree_contains_recursive_literal_subtree(
    parent_anchor: &VirtualPath,
    parent_tail: &LiteralPathTail,
    child_anchor: &VirtualPath,
    child_tail: &LiteralPathTail,
) -> bool {
    let Ok(relative_anchor) = child_anchor.as_path().strip_prefix(parent_anchor.as_path()) else {
        return false;
    };
    parent_tail.matches_path(relative_anchor)
        || parent_tail.is_contiguous_subsequence_of(child_tail)
}

fn looks_like_glob(rule: &str) -> bool {
    rule.contains('*') || rule.contains('?')
}

fn split_supported_recursive_literal_subtree_glob(
    rule: &str,
) -> Result<Option<(Option<&str>, LiteralPathTail)>, String> {
    let Some(without_descendants) = rule.strip_suffix("/**") else {
        return Ok(None);
    };

    let (prefix, raw_tail) = if let Some(tail) = without_descendants.strip_prefix("**/") {
        (None, tail)
    } else if let Some(index) = without_descendants.find("/**/") {
        let prefix = if index == 0 {
            "/"
        } else {
            &without_descendants[..index]
        };
        if prefix.contains('*') || prefix.contains('?') {
            return Err(format!("unsupported glob: {rule}"));
        }
        (Some(prefix), &without_descendants[index + 4..])
    } else {
        return Err(format!("unsupported glob: {rule}"));
    };

    Ok(Some((prefix, LiteralPathTail::parse(rule, raw_tail)?)))
}

fn split_supported_glob(rule: &str) -> Result<(Option<&str>, &str, bool), String> {
    if let Some(pattern) = rule.strip_prefix("**/") {
        return Ok((None, pattern, true));
    }
    if let Some(index) = rule.find("/**/") {
        let prefix = if index == 0 { "/" } else { &rule[..index] };
        if prefix.contains('*') || prefix.contains('?') {
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

        let broad_desc = &broad.descriptors()[0];
        let narrow_desc = &narrow.descriptors()[0];
        let anchored_desc = &anchored.descriptors()[0];
        let hooks_desc = &hooks.descriptors()[0];

        assert!(broad_desc.has_less_specific_ancestor_of(narrow_desc));
        assert!(hooks_desc.has_less_specific_ancestor_of(narrow_desc));
        assert!(narrow_desc.has_less_specific_ancestor_of(anchored_desc));

        let sample = VirtualPath::new("/workspace/app/fixtures/nested/.git/hooks/pre-commit");
        assert!(
            broad.best_specificity(&sample).unwrap() < narrow.best_specificity(&sample).unwrap()
        );
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
        assert!(PathRuleMatcher::new(["**/.git/**/hooks/**"], Vec::new(), &context).is_err());
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
