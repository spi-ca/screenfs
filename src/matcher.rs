//! Shared policy-rule matcher.
//!
//! This module owns rule normalization output, specificity comparison,
//! containment/overlap primitives, and candidate indexing for both visibility
//! and mutability axes. Cross-polarity conflict validation is assembled in
//! `config` from these matcher descriptors.

use std::path::{Path, PathBuf};

use crate::path::{
    RuleNormalizationContext, VirtualPath, canonicalize_or_normalize_absolute, normalize_rule_path,
};

mod descriptor;
mod grammar;
mod index;

use descriptor::RuleTarget;
pub use descriptor::{RuleDescriptor, RuleSpecificity};
use grammar::{CompiledGlob, looks_like_glob, split_supported_subtree_shorthand};
use index::MatcherIndex;

pub(crate) use index::MatcherCandidateMetrics;

/// Compiled path-rule set used by one policy list.
#[derive(Debug, Clone)]
pub struct PathRuleMatcher {
    descriptors: Vec<RuleDescriptor>,
    index: MatcherIndex,
}

/// Policy list being compiled, used for scope-specific rule validation.
pub enum MatcherScope {
    Hidden,
    Visible,
    Readonly,
    Writable,
}

// Scope labels make validation errors point to the policy list being compiled.
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

// Matcher construction normalizes rules, deduplicates descriptors, and builds the index.
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
        Self::new_with_directory_shorthand_mode(rules, internal_prefixes, context, false)
    }

    fn new_with_directory_shorthand_mode<I, P>(
        rules: I,
        internal_prefixes: Vec<VirtualPath>,
        context: &RuleNormalizationContext,
        allow_generic_single_component_directory_shorthand: bool,
    ) -> Result<Self, String>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<str>,
    {
        let mut descriptors = Vec::new();

        for prefix in &internal_prefixes {
            push_descriptor_dedup(
                &mut descriptors,
                RuleDescriptor {
                    anchor: prefix.clone(),
                    specificity: RuleSpecificity::exact_or_prefix(prefix, false),
                    target: RuleTarget::Subtree,
                },
            );
        }

        for rule in rules {
            let rule = rule.as_ref();
            if let Some(raw_prefix) = split_supported_subtree_shorthand(rule) {
                let path = normalize_rule_path(raw_prefix, context)?;
                push_descriptor_dedup(
                    &mut descriptors,
                    RuleDescriptor {
                        anchor: path.clone(),
                        specificity: RuleSpecificity::exact_or_prefix(&path, false),
                        target: RuleTarget::Subtree,
                    },
                );
            } else if looks_like_glob(rule) {
                let glob = CompiledGlob::compile(
                    rule,
                    context,
                    allow_generic_single_component_directory_shorthand,
                )?;
                push_descriptor_dedup(&mut descriptors, glob.descriptor());
            } else {
                let path = normalize_rule_path(rule, context)?;
                push_descriptor_dedup(
                    &mut descriptors,
                    RuleDescriptor {
                        anchor: path.clone(),
                        specificity: RuleSpecificity::exact_or_prefix(&path, false),
                        target: RuleTarget::Subtree,
                    },
                );
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
        let allow_generic_single_component_directory_shorthand = matches!(
            scope,
            MatcherScope::Hidden | MatcherScope::Readonly | MatcherScope::Writable
        );
        Self::new_with_directory_shorthand_mode(
            rules,
            internal_prefixes,
            context,
            allow_generic_single_component_directory_shorthand,
        )
        .map_err(|err| format!("invalid {} pattern: {err}", scope.label()))
    }

    pub fn best_specificity(&self, path: &VirtualPath) -> Option<RuleSpecificity> {
        self.best_descriptor(path)
            .map(|descriptor| descriptor.specificity)
    }

    pub fn best_descriptor(&self, path: &VirtualPath) -> Option<&RuleDescriptor> {
        self.index
            .best_matching_candidate(path, |index| self.descriptors[index].matches_path(path))
            .map(|index| &self.descriptors[index])
    }

    pub fn candidate_descriptor_count(&self, path: &VirtualPath) -> usize {
        self.index.candidate_order(path).len()
    }

    #[allow(dead_code)]
    pub(crate) fn candidate_descriptor_metrics(
        &self,
        path: &VirtualPath,
    ) -> MatcherCandidateMetrics {
        self.index.candidate_metrics(path).metrics()
    }

    pub fn descriptors(&self) -> &[RuleDescriptor] {
        &self.descriptors
    }

    pub fn matches_path(&self, path: &VirtualPath) -> bool {
        self.best_specificity(path).is_some()
    }

    pub fn may_match_descendant_of(&self, path: &VirtualPath) -> bool {
        self.index.has_global_descendant_match
            || self.index.any_matching_descendant_candidate(path, |index| {
                self.descriptors[index].may_match_descendant_of(path)
            })
    }

    pub fn descendant_candidate_descriptor_count(&self, path: &VirtualPath) -> usize {
        self.index.descendant_candidate_order(path).len()
    }

    #[allow(dead_code)]
    pub(crate) fn descendant_candidate_descriptor_metrics(
        &self,
        path: &VirtualPath,
    ) -> MatcherCandidateMetrics {
        self.index.descendant_candidate_metrics(path).metrics()
    }

    pub fn has_recursive_bridge_discovery_rule(&self) -> bool {
        self.descriptors
            .iter()
            .any(RuleDescriptor::requires_recursive_bridge_discovery)
    }

    pub fn can_skip_symlink_target_visibility_check(&self) -> bool {
        self.descriptors.is_empty()
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

// Internal mount-root exclusion is compiled as a hidden prefix before user rules.
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

fn push_descriptor_dedup(descriptors: &mut Vec<RuleDescriptor>, descriptor: RuleDescriptor) {
    if !descriptors.contains(&descriptor) {
        descriptors.push(descriptor);
    }
}

#[cfg(test)]
#[path = "matcher_tests.rs"]
mod tests;
