use std::cmp::Ordering;
use std::path::{Component, Path, PathBuf};

use crate::path::{
    RuleNormalizationContext, VirtualPath, canonicalize_or_normalize_absolute, normalize_rule_path,
};

#[derive(Debug, Clone)]
pub struct PathRuleMatcher {
    descriptors: Vec<RuleDescriptor>,
    index: MatcherIndex,
}

#[derive(Debug, Clone)]
struct MatcherIndex {
    match_order: Vec<usize>,
    dynamic_bridge_scan_roots: Vec<VirtualPath>,
    needs_dynamic_bridge_index: bool,
    has_global_descendant_match: bool,
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

    fn could_match_descendant_under(&self, path: &VirtualPath, anchor: &VirtualPath) -> bool {
        path.starts_with(anchor) || anchor.starts_with(path)
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

    pub fn contains_target_set(&self, other: &Self) -> bool {
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

    pub fn may_match_descendant_of(&self, path: &VirtualPath) -> bool {
        match &self.target {
            RuleTarget::Subtree => self.anchor.starts_with(path),
            RuleTarget::Glob { recursive, pattern } => {
                if self.anchor.starts_with(path) {
                    return true;
                }
                let Ok(relative) = path.as_path().strip_prefix(self.anchor.as_path()) else {
                    return false;
                };
                if *recursive {
                    true
                } else {
                    relative
                        .components()
                        .next()
                        .is_none_or(|component| pattern.matches_component(component))
                }
            }
            RuleTarget::RecursiveLiteralSubtree { tail } => {
                if self.anchor.starts_with(path) {
                    return true;
                }
                path.starts_with(&self.anchor)
                    && tail.could_match_descendant_under(path, &self.anchor)
            }
        }
    }

    pub fn is_static_subtree_bridge_ancestor(&self, path: &VirtualPath) -> bool {
        matches!(self.target, RuleTarget::Subtree) && self.anchor.starts_with(path)
    }

    pub fn needs_dynamic_bridge_index(&self) -> bool {
        !matches!(self.target, RuleTarget::Subtree)
    }

    pub fn dynamic_bridge_scan_root(&self) -> Option<&VirtualPath> {
        self.needs_dynamic_bridge_index().then_some(&self.anchor)
    }

    fn may_match_descendant_under_any_path(&self) -> bool {
        self.anchor == VirtualPath::root()
            && matches!(
                self.target,
                RuleTarget::Glob {
                    recursive: true,
                    ..
                } | RuleTarget::RecursiveLiteralSubtree { .. }
            )
    }

    pub fn has_unproven_overlap_with(&self, other: &Self) -> bool {
        !self.contains_target_set(other)
            && !other.contains_target_set(self)
            && self.may_overlap_target_set(other)
    }

    fn may_overlap_target_set(&self, other: &Self) -> bool {
        match (&self.target, &other.target) {
            (RuleTarget::Subtree, RuleTarget::Subtree) => {
                self.anchor.starts_with(&other.anchor) || other.anchor.starts_with(&self.anchor)
            }
            (RuleTarget::Subtree, _) => other.may_match_descendant_of(&self.anchor),
            (_, RuleTarget::Subtree) => self.may_match_descendant_of(&other.anchor),
            (
                RuleTarget::Glob { recursive, .. },
                RuleTarget::Glob {
                    recursive: other_recursive,
                    ..
                },
            ) => anchors_may_overlap(*recursive, &self.anchor, *other_recursive, &other.anchor),
            (RuleTarget::Glob { recursive, .. }, RuleTarget::RecursiveLiteralSubtree { .. }) => {
                anchors_may_overlap(*recursive, &self.anchor, true, &other.anchor)
            }
            (RuleTarget::RecursiveLiteralSubtree { .. }, RuleTarget::Glob { recursive, .. }) => {
                anchors_may_overlap(true, &self.anchor, *recursive, &other.anchor)
            }
            (
                RuleTarget::RecursiveLiteralSubtree { .. },
                RuleTarget::RecursiveLiteralSubtree { .. },
            ) => anchors_may_overlap(true, &self.anchor, true, &other.anchor),
        }
    }

    fn matches_path(&self, path: &VirtualPath) -> bool {
        match &self.target {
            RuleTarget::Subtree => path.starts_with(&self.anchor),
            RuleTarget::Glob { recursive, pattern } => {
                let Ok(candidate) = path.as_path().strip_prefix(self.anchor.as_path()) else {
                    return false;
                };
                let mut components = candidate.components();
                if *recursive {
                    components.any(|component| pattern.matches_component(component))
                } else {
                    components
                        .next()
                        .is_some_and(|component| pattern.matches_component(component))
                }
            }
            RuleTarget::RecursiveLiteralSubtree { tail } => {
                let Ok(candidate) = path.as_path().strip_prefix(self.anchor.as_path()) else {
                    return false;
                };
                tail.matches_path(candidate)
            }
        }
    }
}

impl MatcherIndex {
    fn build(descriptors: &[RuleDescriptor]) -> Self {
        let mut match_order = (0..descriptors.len()).collect::<Vec<_>>();
        match_order.sort_by(|left, right| {
            descriptors[*right]
                .specificity
                .cmp(&descriptors[*left].specificity)
                .then_with(|| right.cmp(left))
        });

        let mut dynamic_bridge_scan_roots = descriptors
            .iter()
            .filter_map(|descriptor| descriptor.dynamic_bridge_scan_root().cloned())
            .collect::<Vec<_>>();
        dynamic_bridge_scan_roots.sort();
        dynamic_bridge_scan_roots.dedup();
        dynamic_bridge_scan_roots = prune_nested_scan_roots(dynamic_bridge_scan_roots);

        let needs_dynamic_bridge_index = !dynamic_bridge_scan_roots.is_empty();
        let has_global_descendant_match = descriptors
            .iter()
            .any(RuleDescriptor::may_match_descendant_under_any_path);

        Self {
            match_order,
            dynamic_bridge_scan_roots,
            needs_dynamic_bridge_index,
            has_global_descendant_match,
        }
    }
}

fn prune_nested_scan_roots(roots: Vec<VirtualPath>) -> Vec<VirtualPath> {
    let mut pruned: Vec<VirtualPath> = Vec::new();
    for root in roots {
        if pruned.iter().any(|existing| root.starts_with(existing)) {
            continue;
        }
        pruned.retain(|existing| !existing.starts_with(&root));
        pruned.push(root);
    }
    pruned
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatcherScope {
    Hidden,
    Visible,
    Readonly,
    Writable,
}

impl MatcherScope {
    fn label(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Visible => "visible",
            Self::Readonly => "readonly",
            Self::Writable => "writable",
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
        let mut descriptors = Vec::new();

        for prefix in &internal_prefixes {
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
            } else {
                let path = normalize_rule_path(rule, context)?;
                descriptors.push(RuleDescriptor {
                    anchor: path.clone(),
                    specificity: RuleSpecificity::exact_or_prefix(&path, false),
                    target: RuleTarget::Subtree,
                });
            }
        }

        let index = MatcherIndex::build(&descriptors);
        Ok(Self { descriptors, index })
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

    pub fn best_specificity(&self, path: &VirtualPath) -> Option<RuleSpecificity> {
        self.best_descriptor(path)
            .map(|descriptor| descriptor.specificity)
    }

    pub fn best_descriptor(&self, path: &VirtualPath) -> Option<&RuleDescriptor> {
        self.index
            .match_order
            .iter()
            .map(|index| &self.descriptors[*index])
            .find(|descriptor| descriptor.matches_path(path))
    }

    pub fn descriptors(&self) -> &[RuleDescriptor] {
        &self.descriptors
    }

    pub fn matches_path(&self, path: &VirtualPath) -> bool {
        self.best_specificity(path).is_some()
    }

    pub fn may_match_descendant_of(&self, path: &VirtualPath) -> bool {
        self.index.has_global_descendant_match
            || self
                .descriptors
                .iter()
                .any(|descriptor| descriptor.may_match_descendant_of(path))
    }

    pub fn has_static_subtree_bridge_ancestor(&self, path: &VirtualPath) -> bool {
        self.descriptors
            .iter()
            .any(|descriptor| descriptor.is_static_subtree_bridge_ancestor(path))
    }

    pub fn dynamic_bridge_scan_roots(&self) -> Vec<VirtualPath> {
        self.index.dynamic_bridge_scan_roots.clone()
    }

    pub fn needs_dynamic_bridge_index(&self) -> bool {
        self.index.needs_dynamic_bridge_index
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

fn anchors_may_overlap(
    left_recursive: bool,
    left_anchor: &VirtualPath,
    right_recursive: bool,
    right_anchor: &VirtualPath,
) -> bool {
    if left_anchor == right_anchor {
        return true;
    }
    if right_anchor.as_path().starts_with(left_anchor.as_path()) {
        return left_recursive;
    }
    if left_anchor.as_path().starts_with(right_anchor.as_path()) {
        return right_recursive;
    }
    false
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
        if is_supported_basename_glob_pattern(pattern) {
            let prefix = if index == 0 { "/" } else { &rule[..index] };
            if prefix.contains('*') || prefix.contains('?') {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok((Some(prefix), pattern, false));
        }
    } else if is_supported_basename_glob_pattern(rule) {
        return Ok((Some("."), rule, false));
    }
    Err(format!("unsupported glob: {rule}"))
}

fn is_supported_basename_glob_pattern(pattern: &str) -> bool {
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
#[path = "matcher_tests.rs"]
mod tests;
