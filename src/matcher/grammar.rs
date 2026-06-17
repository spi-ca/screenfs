//! Supported rule grammar parsing for path matchers.
//!
//! This module recognizes exact/subtree, direct-child glob, recursive glob, and
//! recursive literal subtree forms before descriptors are indexed.

use crate::path::{RuleNormalizationContext, VirtualPath, normalize_rule_path};

use super::descriptor::{
    GlobPattern, LiteralPathTail, RuleDescriptor, RuleSpecificity, RuleTarget,
};

// CompiledGlob keeps only the supported glob shapes accepted by the current contract.
pub(super) enum CompiledGlob {
    Any {
        prefix: Option<VirtualPath>,
    },
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

// Compilation validates syntax and normalizes glob prefixes into virtual anchors.
impl CompiledGlob {
    pub(super) fn compile(
        rule: &str,
        context: &RuleNormalizationContext,
        allow_generic_single_component_directory_shorthand: bool,
    ) -> Result<Self, String> {
        if let Some((prefix, tail)) = split_supported_recursive_literal_subtree_glob(
            rule,
            allow_generic_single_component_directory_shorthand,
        )? {
            let prefix = prefix
                .map(|raw| normalize_rule_path(raw, context))
                .transpose()?;
            return Ok(Self::RecursiveLiteralSubtree { prefix, tail });
        }

        let (prefix, pattern, recursive) = split_supported_glob(rule)?;
        let prefix = prefix
            .map(|raw| normalize_rule_path(raw, context))
            .transpose()?;
        if pattern.is_empty()
            || pattern.contains('/')
            || pattern.contains('?')
            || pattern.contains('[')
            || pattern.contains(']')
        {
            return Err(format!("unsupported glob: {rule}"));
        }
        if pattern == "*" {
            if recursive {
                return Err(format!("unsupported glob: {rule}"));
            }
            return Ok(Self::Any { prefix });
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

    pub(super) fn descriptor(&self) -> RuleDescriptor {
        match self {
            Self::Any { prefix } => {
                let pattern = self.pattern().expect("pattern-backed glob");
                RuleDescriptor {
                    anchor: prefix.clone().unwrap_or_else(VirtualPath::root),
                    specificity: RuleSpecificity::glob(prefix.as_ref(), false, &pattern),
                    target: RuleTarget::Glob {
                        recursive: false,
                        pattern,
                    },
                }
            }
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
            Self::Any { .. } => Some(GlobPattern::Any),
            Self::Basename { name, .. } => Some(GlobPattern::Basename(name.clone())),
            Self::Prefix { name_prefix, .. } => Some(GlobPattern::Prefix(name_prefix.clone())),
            Self::Suffix { suffix, .. } => Some(GlobPattern::Suffix(suffix.clone())),
            Self::RecursiveLiteralSubtree { .. } => None,
        }
    }
}

// Lightweight detection lets non-glob exact/subtree rules take the simpler path.
pub(super) fn looks_like_glob(rule: &str) -> bool {
    rule.contains('*') || rule.contains('?')
}

fn split_supported_recursive_literal_subtree_glob(
    rule: &str,
    allow_generic_single_component_directory_shorthand: bool,
) -> Result<Option<(Option<&str>, LiteralPathTail)>, String> {
    let (without_descendants, require_literal_tail) =
        if let Some(without_descendants) = rule.strip_suffix("/**") {
            (without_descendants, true)
        } else {
            (rule, false)
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
        return Ok(None);
    };

    if !require_literal_tail
        && !looks_like_recursive_literal_directory_shorthand_tail(
            raw_tail,
            allow_generic_single_component_directory_shorthand,
        )
    {
        return Ok(None);
    }

    Ok(Some((prefix, LiteralPathTail::parse(rule, raw_tail)?)))
}

fn looks_like_recursive_literal_directory_shorthand_tail(
    raw_tail: &str,
    allow_generic_single_component_directory_shorthand: bool,
) -> bool {
    let mut components = raw_tail.split('/').peekable();
    let Some(first) = components.next() else {
        return false;
    };
    if !is_literal_tail_component(first) {
        return false;
    }
    if components.peek().is_none() {
        return allow_generic_single_component_directory_shorthand;
    }
    components.all(is_literal_tail_component)
}

fn is_literal_tail_component(component: &str) -> bool {
    !component.is_empty()
        && component != "."
        && component != ".."
        && !component.contains('*')
        && !component.contains('?')
        && !component.contains('[')
        && !component.contains(']')
}

fn split_supported_glob(rule: &str) -> Result<(Option<&str>, &str, bool), String> {
    if let Some(pattern) = rule.strip_prefix("**/") {
        return Ok((Some("."), pattern, true));
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
        if is_supported_direct_child_glob_pattern(pattern) {
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

// Directory shorthand is split before generic glob parsing so recursive literal
// directories can canonicalize to subtree descriptors.
pub(super) fn split_supported_subtree_shorthand(rule: &str) -> Option<&str> {
    let prefix = rule.strip_suffix("/**")?;
    if prefix.is_empty()
        || prefix.contains('*')
        || prefix.contains('?')
        || !(prefix.starts_with('/')
            || prefix == "."
            || prefix.starts_with("./")
            || prefix == "~"
            || prefix.starts_with("~/"))
    {
        return None;
    }
    Some(prefix)
}

fn is_supported_direct_child_glob_pattern(pattern: &str) -> bool {
    pattern == "*" || is_supported_basename_glob_pattern(pattern)
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
