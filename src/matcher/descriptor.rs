use std::cmp::Ordering;
use std::path::{Component, Path};

use crate::path::VirtualPath;

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
    pub(super) fn exact_or_prefix(path: &VirtualPath, exact: bool) -> Self {
        Self {
            prefix_components: component_count(path),
            kind: if exact {
                RuleSpecificityKind::Exact
            } else {
                RuleSpecificityKind::PrefixSubtree
            },
        }
    }

    pub(super) fn glob(
        prefix: Option<&VirtualPath>,
        recursive: bool,
        pattern: &GlobPattern,
    ) -> Self {
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

    pub(super) fn recursive_literal_subtree(
        prefix: Option<&VirtualPath>,
        tail: &LiteralPathTail,
    ) -> Self {
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
    pub(super) anchor: VirtualPath,
    pub(super) specificity: RuleSpecificity,
    pub(super) target: RuleTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum RuleTarget {
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
pub(super) enum GlobPattern {
    Any,
    Basename(String),
    Prefix(String),
    Suffix(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LiteralPathTail(Vec<String>);

impl LiteralPathTail {
    pub(super) fn parse(rule: &str, raw_tail: &str) -> Result<Self, String> {
        let mut components = Vec::new();
        for component in raw_tail.split('/') {
            if component.is_empty()
                || component == "."
                || component == ".."
                || component.contains('*')
                || component.contains('?')
                || component.contains('[')
                || component.contains(']')
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

    pub(super) fn component_count(&self) -> usize {
        self.0.len()
    }

    pub(super) fn total_len(&self) -> usize {
        self.0.iter().map(String::len).sum()
    }

    pub(super) fn contains_name_matching(&self, pattern: &GlobPattern) -> bool {
        self.0.iter().any(|name| pattern.matches_name(name))
    }

    pub(super) fn matches_path(&self, path: &Path) -> bool {
        let components = path_component_names(path);
        let tail = self.0.as_slice();
        components.len() >= tail.len()
            && components.windows(tail.len()).any(|window| window == tail)
    }

    pub(super) fn is_contiguous_subsequence_of(&self, other: &Self) -> bool {
        let tail = self.0.as_slice();
        other.0.len() >= tail.len() && other.0.windows(tail.len()).any(|window| window == tail)
    }

    pub(super) fn could_match_descendant_under(
        &self,
        path: &VirtualPath,
        anchor: &VirtualPath,
    ) -> bool {
        path.starts_with(anchor) || anchor.starts_with(path)
    }
}

impl GlobPattern {
    pub(super) fn matches_component(&self, component: Component<'_>) -> bool {
        match component {
            Component::Normal(part) => self.matches_name(&part.to_string_lossy()),
            _ => false,
        }
    }

    pub(super) fn matches_name(&self, name: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Basename(expected) => name == expected.as_str(),
            Self::Prefix(name_prefix) => name.starts_with(name_prefix),
            Self::Suffix(suffix) => name.ends_with(suffix),
        }
    }

    pub(super) fn contains_pattern(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) => true,
            (Self::Basename(left), Self::Basename(right)) => left == right,
            (Self::Prefix(prefix), Self::Basename(name)) => name.starts_with(prefix),
            (Self::Prefix(prefix), Self::Prefix(other_prefix)) => other_prefix.starts_with(prefix),
            (Self::Suffix(suffix), Self::Basename(name)) => name.ends_with(suffix),
            (Self::Suffix(suffix), Self::Suffix(other_suffix)) => other_suffix.ends_with(suffix),
            _ => false,
        }
    }

    pub(super) fn specificity_tail(&self) -> (u8, usize) {
        match self {
            Self::Any => (0, 0),
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

    pub fn requires_recursive_bridge_discovery(&self) -> bool {
        matches!(
            self.target,
            RuleTarget::Glob {
                recursive: true,
                ..
            } | RuleTarget::RecursiveLiteralSubtree { .. }
        )
    }

    pub(super) fn may_match_descendant_under_any_path(&self) -> bool {
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

    pub(super) fn matches_path(&self, path: &VirtualPath) -> bool {
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

pub(super) fn glob_contains_recursive_literal_subtree(
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

pub(super) fn recursive_literal_subtree_contains_recursive_literal_subtree(
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
