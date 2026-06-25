//! Candidate index for bounded matcher lookup.
//!
//! The index narrows descriptor scans by match family and anchor while preserving
//! the same winner as full candidate ordering.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::path::VirtualPath;

use super::descriptor::{RuleDescriptor, RuleTarget};

// Metrics structs expose how much candidate work a policy lookup performed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct MatcherCandidateFamilyCounts {
    pub(crate) subtree: usize,
    pub(crate) direct_child_glob: usize,
    pub(crate) recursive: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CandidateOrderMetrics {
    pub(crate) elapsed: Duration,
    pub(crate) duplicates_skipped: usize,
    pub(crate) seen_slots: usize,
    pub(crate) ancestor_steps: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct MatcherCandidateMetrics {
    pub(crate) count: usize,
    pub(crate) family_counts: MatcherCandidateFamilyCounts,
    pub(crate) candidate_order: CandidateOrderMetrics,
}

// The index partitions descriptors by family to avoid whole-list scans on hot paths.
#[derive(Debug, Clone)]
pub(super) struct MatcherIndex {
    subtree_by_anchor: BTreeMap<VirtualPath, Vec<usize>>,
    direct_child_glob_by_anchor: BTreeMap<VirtualPath, Vec<usize>>,
    recursive_order: Vec<usize>,
    bridge_subtree_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>>,
    bridge_direct_child_glob_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>>,
    order_rank: Vec<usize>,
    pub(super) has_global_descendant_match: bool,
}

// Query methods preserve full candidate-order semantics while narrowing the search space.
impl MatcherIndex {
    pub(super) fn build(descriptors: &[RuleDescriptor]) -> Self {
        let mut match_order = (0..descriptors.len()).collect::<Vec<_>>();
        match_order.sort_by(|left, right| {
            descriptors[*right]
                .specificity
                .cmp(&descriptors[*left].specificity)
                .then_with(|| right.cmp(left))
        });

        let mut subtree_by_anchor: BTreeMap<VirtualPath, Vec<usize>> = BTreeMap::new();
        let mut direct_child_glob_by_anchor: BTreeMap<VirtualPath, Vec<usize>> = BTreeMap::new();
        let mut recursive_order = Vec::new();
        let mut bridge_subtree_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>> =
            BTreeMap::new();
        let mut bridge_direct_child_glob_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>> =
            BTreeMap::new();
        let mut order_rank = vec![usize::MAX; descriptors.len()];
        for (rank, index) in match_order.iter().enumerate() {
            order_rank[*index] = rank;
        }
        for index in &match_order {
            let descriptor = &descriptors[*index];
            match &descriptor.target {
                RuleTarget::Subtree => {
                    subtree_by_anchor
                        .entry(descriptor.anchor.clone())
                        .or_default()
                        .push(*index);
                    for ancestor in ancestor_paths(descriptor.anchor.as_path()) {
                        bridge_subtree_descendant_by_path
                            .entry(VirtualPath::new(ancestor))
                            .or_default()
                            .push(*index);
                    }
                }
                RuleTarget::Glob {
                    recursive: false, ..
                } => {
                    direct_child_glob_by_anchor
                        .entry(descriptor.anchor.clone())
                        .or_default()
                        .push(*index);
                    for ancestor in ancestor_paths(descriptor.anchor.as_path()) {
                        bridge_direct_child_glob_descendant_by_path
                            .entry(VirtualPath::new(ancestor))
                            .or_default()
                            .push(*index);
                    }
                }
                RuleTarget::Glob {
                    recursive: true, ..
                }
                | RuleTarget::RecursiveLiteralSubtree { .. } => recursive_order.push(*index),
            }
        }

        let has_global_descendant_match = descriptors
            .iter()
            .any(RuleDescriptor::may_match_descendant_under_any_path);

        Self {
            subtree_by_anchor,
            direct_child_glob_by_anchor,
            recursive_order,
            bridge_subtree_descendant_by_path,
            bridge_direct_child_glob_descendant_by_path,
            order_rank,
            has_global_descendant_match,
        }
    }

    pub(super) fn candidate_order(&self, path: &VirtualPath) -> Vec<usize> {
        let mut candidates = Vec::new();
        self.extend_path_candidates(path, &mut candidates, None);
        self.sort_and_dedup_candidates_by_match_order(candidates)
    }

    pub(super) fn best_matching_candidate<F>(
        &self,
        path: &VirtualPath,
        mut matches: F,
    ) -> Option<usize>
    where
        F: FnMut(usize) -> bool,
    {
        let mut best = None;
        self.visit_path_candidates(path, |index| {
            if matches(index) {
                self.update_best_candidate(&mut best, index);
            }
        });
        best.map(|(_, index)| index)
    }

    pub(super) fn candidate_metrics(&self, path: &VirtualPath) -> MatcherCandidateMetricsResult {
        let start = Instant::now();
        let mut candidates = Vec::new();
        let mut metrics = MatcherCandidateMetrics::default();
        metrics.candidate_order.seen_slots = self.order_rank.len();
        self.extend_path_candidates(path, &mut candidates, Some(&mut metrics));
        let raw_count = candidates.len();
        let candidates = self.sort_and_dedup_candidates_by_match_order(candidates);
        metrics.candidate_order.duplicates_skipped = raw_count.saturating_sub(candidates.len());
        metrics.count = candidates.len();
        metrics.candidate_order.elapsed = start.elapsed();
        MatcherCandidateMetricsResult(metrics)
    }

    pub(super) fn descendant_candidate_order(&self, path: &VirtualPath) -> Vec<usize> {
        let mut candidates = Vec::new();
        self.extend_descendant_candidates(path, &mut candidates, None);
        self.sort_and_dedup_candidates_by_match_order(candidates)
    }

    pub(super) fn any_matching_descendant_candidate<F>(
        &self,
        path: &VirtualPath,
        mut matches: F,
    ) -> bool
    where
        F: FnMut(usize) -> bool,
    {
        let mut found = false;
        self.visit_descendant_candidates(path, |index| {
            if !found && matches(index) {
                found = true;
            }
        });
        found
    }

    pub(super) fn descendant_candidate_metrics(
        &self,
        path: &VirtualPath,
    ) -> MatcherCandidateMetricsResult {
        let start = Instant::now();
        let mut candidates = Vec::new();
        let mut metrics = MatcherCandidateMetrics::default();
        metrics.candidate_order.seen_slots = self.order_rank.len();
        self.extend_descendant_candidates(path, &mut candidates, Some(&mut metrics));
        let raw_count = candidates.len();
        let candidates = self.sort_and_dedup_candidates_by_match_order(candidates);
        metrics.candidate_order.duplicates_skipped = raw_count.saturating_sub(candidates.len());
        metrics.count = candidates.len();
        metrics.candidate_order.elapsed = start.elapsed();
        MatcherCandidateMetricsResult(metrics)
    }

    fn visit_path_candidates(&self, path: &VirtualPath, mut visit: impl FnMut(usize)) {
        for ancestor in ancestor_paths(path.as_path()) {
            if let Some(indices) = self.subtree_by_anchor.get(ancestor) {
                for index in indices {
                    visit(*index);
                }
            }
            if let Some(indices) = self.direct_child_glob_by_anchor.get(ancestor) {
                for index in indices {
                    visit(*index);
                }
            }
        }
        for index in &self.recursive_order {
            visit(*index);
        }
    }

    fn update_best_candidate(&self, best: &mut Option<(usize, usize)>, index: usize) {
        let rank = self.order_rank[index];
        match best {
            Some((best_rank, _)) if rank >= *best_rank => {}
            _ => *best = Some((rank, index)),
        }
    }

    fn extend_path_candidates(
        &self,
        path: &VirtualPath,
        candidates: &mut Vec<usize>,
        mut metrics: Option<&mut MatcherCandidateMetrics>,
    ) {
        for ancestor in ancestor_paths(path.as_path()) {
            if let Some(metrics) = metrics.as_deref_mut() {
                metrics.candidate_order.ancestor_steps += 1;
            }
            if let Some(indices) = self.subtree_by_anchor.get(ancestor) {
                if let Some(metrics) = metrics.as_deref_mut() {
                    metrics.family_counts.subtree += indices.len();
                }
                candidates.extend(indices);
            }
            if let Some(indices) = self.direct_child_glob_by_anchor.get(ancestor) {
                if let Some(metrics) = metrics.as_deref_mut() {
                    metrics.family_counts.direct_child_glob += indices.len();
                }
                candidates.extend(indices);
            }
        }
        if let Some(metrics) = metrics {
            metrics.family_counts.recursive += self.recursive_order.len();
        }
        candidates.extend(&self.recursive_order);
    }

    // This streams raw descendant candidates for hot-path callers; debug helpers
    // that need canonical candidate ordering should keep using
    // `descendant_candidate_order()`.
    pub(super) fn visit_descendant_candidates(
        &self,
        path: &VirtualPath,
        mut visit: impl FnMut(usize),
    ) {
        if let Some(indices) = self.bridge_subtree_descendant_by_path.get(path.as_path()) {
            for index in indices {
                visit(*index);
            }
        }
        if let Some(indices) = self
            .bridge_direct_child_glob_descendant_by_path
            .get(path.as_path())
        {
            for index in indices {
                visit(*index);
            }
        }
        for ancestor in ancestor_paths(path.as_path()) {
            if let Some(indices) = self.direct_child_glob_by_anchor.get(ancestor) {
                for index in indices {
                    visit(*index);
                }
            }
        }
        for index in &self.recursive_order {
            visit(*index);
        }
    }

    fn extend_descendant_candidates(
        &self,
        path: &VirtualPath,
        candidates: &mut Vec<usize>,
        mut metrics: Option<&mut MatcherCandidateMetrics>,
    ) {
        if let Some(indices) = self.bridge_subtree_descendant_by_path.get(path.as_path()) {
            if let Some(metrics) = metrics.as_deref_mut() {
                metrics.family_counts.subtree += indices.len();
            }
            candidates.extend(indices);
        }
        if let Some(indices) = self
            .bridge_direct_child_glob_descendant_by_path
            .get(path.as_path())
        {
            if let Some(metrics) = metrics.as_deref_mut() {
                metrics.family_counts.direct_child_glob += indices.len();
            }
            candidates.extend(indices);
        }
        for ancestor in ancestor_paths(path.as_path()) {
            if let Some(metrics) = metrics.as_deref_mut() {
                metrics.candidate_order.ancestor_steps += 1;
            }
            if let Some(indices) = self.direct_child_glob_by_anchor.get(ancestor) {
                if let Some(metrics) = metrics.as_deref_mut() {
                    metrics.family_counts.direct_child_glob += indices.len();
                }
                candidates.extend(indices);
            }
        }
        if let Some(metrics) = metrics {
            metrics.family_counts.recursive += self.recursive_order.len();
        }
        candidates.extend(&self.recursive_order);
    }

    fn sort_and_dedup_candidates_by_match_order(&self, mut candidates: Vec<usize>) -> Vec<usize> {
        candidates.sort_unstable_by_key(|index| self.order_rank[*index]);
        candidates.dedup();
        candidates
    }
}

#[derive(Debug, Clone)]
pub(super) struct MatcherCandidateMetricsResult(MatcherCandidateMetrics);

impl MatcherCandidateMetricsResult {
    #[allow(dead_code)]
    pub(super) fn metrics(&self) -> MatcherCandidateMetrics {
        self.0
    }
}

// Ancestor iteration feeds subtree and bridge-candidate lookup.
fn ancestor_paths(path: &Path) -> AncestorPaths<'_> {
    AncestorPaths {
        current: Some(path),
    }
}

struct AncestorPaths<'a> {
    current: Option<&'a Path>,
}

impl<'a> Iterator for AncestorPaths<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let path = self.current?;
        self.current = if path == Path::new("/") {
            None
        } else {
            path.parent()
        };
        Some(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::descriptor::{GlobPattern, LiteralPathTail, RuleSpecificity};

    fn subtree_descriptor(path: &str) -> RuleDescriptor {
        let anchor = VirtualPath::new(path);
        RuleDescriptor {
            anchor: anchor.clone(),
            specificity: RuleSpecificity::exact_or_prefix(&anchor, false),
            target: RuleTarget::Subtree,
        }
    }

    fn direct_child_suffix_descriptor(anchor: &str, suffix: &str) -> RuleDescriptor {
        let anchor = VirtualPath::new(anchor);
        let pattern = GlobPattern::Suffix(suffix.to_string());
        RuleDescriptor {
            anchor: anchor.clone(),
            specificity: RuleSpecificity::glob(Some(&anchor), false, &pattern),
            target: RuleTarget::Glob {
                recursive: false,
                pattern,
            },
        }
    }

    fn recursive_suffix_descriptor(anchor: &str, suffix: &str) -> RuleDescriptor {
        let anchor = VirtualPath::new(anchor);
        let pattern = GlobPattern::Suffix(suffix.to_string());
        RuleDescriptor {
            anchor: anchor.clone(),
            specificity: RuleSpecificity::glob(Some(&anchor), true, &pattern),
            target: RuleTarget::Glob {
                recursive: true,
                pattern,
            },
        }
    }

    fn recursive_literal_descriptor(anchor: &str, tail: &str) -> RuleDescriptor {
        let anchor = VirtualPath::new(anchor);
        let tail = LiteralPathTail::parse("test", tail).unwrap();
        RuleDescriptor {
            anchor: anchor.clone(),
            specificity: RuleSpecificity::recursive_literal_subtree(Some(&anchor), &tail),
            target: RuleTarget::RecursiveLiteralSubtree { tail },
        }
    }

    #[test]
    fn candidate_order_uses_match_order_after_unstable_sort() {
        let descriptors = vec![
            subtree_descriptor("/alpha/bravo"),
            subtree_descriptor("/alpha"),
            subtree_descriptor("/alpha/bravo/charlie"),
            subtree_descriptor("/unrelated"),
        ];
        let index = MatcherIndex::build(&descriptors);
        let path = VirtualPath::new("/alpha/bravo/charlie/file.txt");

        assert_eq!(index.candidate_order(&path), vec![2, 0, 1]);

        let metrics = index.candidate_metrics(&path).metrics();
        assert_eq!(metrics.count, 3);
        assert_eq!(metrics.family_counts.subtree, 3);
        assert_eq!(metrics.family_counts.direct_child_glob, 0);
        assert_eq!(metrics.family_counts.recursive, 0);
        assert_eq!(metrics.candidate_order.duplicates_skipped, 0);
        assert_eq!(metrics.candidate_order.seen_slots, descriptors.len());
        assert!(metrics.candidate_order.ancestor_steps > 0);
    }

    #[test]
    fn descendant_candidate_order_uses_match_order_after_unstable_sort() {
        let descriptors = vec![
            subtree_descriptor("/alpha/bravo"),
            subtree_descriptor("/alpha/bravo/charlie"),
            subtree_descriptor("/alpha"),
            subtree_descriptor("/unrelated"),
        ];
        let index = MatcherIndex::build(&descriptors);
        let path = VirtualPath::new("/alpha");

        assert_eq!(index.descendant_candidate_order(&path), vec![1, 0, 2]);

        let metrics = index.descendant_candidate_metrics(&path).metrics();
        assert_eq!(metrics.count, 3);
        assert_eq!(metrics.family_counts.subtree, 3);
        assert_eq!(metrics.family_counts.direct_child_glob, 0);
        assert_eq!(metrics.family_counts.recursive, 0);
        assert_eq!(metrics.candidate_order.duplicates_skipped, 0);
        assert_eq!(metrics.candidate_order.seen_slots, descriptors.len());
    }

    #[test]
    fn best_matching_candidate_matches_candidate_order_winner_for_mixed_families() {
        let descriptors = vec![
            recursive_suffix_descriptor("/alpha", ".log"),
            subtree_descriptor("/alpha/bravo"),
            direct_child_suffix_descriptor("/alpha/bravo/charlie", ".log"),
            recursive_literal_descriptor("/alpha", "charlie/audit.log"),
            subtree_descriptor("/unrelated"),
        ];
        let index = MatcherIndex::build(&descriptors);
        let path = VirtualPath::new("/alpha/bravo/charlie/audit.log");
        let expected = index
            .candidate_order(&path)
            .into_iter()
            .find(|index| descriptors[*index].matches_path(&path));

        assert_eq!(
            index.best_matching_candidate(&path, |index| descriptors[index].matches_path(&path)),
            expected
        );
    }

    #[test]
    fn descendant_streaming_candidate_matches_candidate_order_bool_for_mixed_families() {
        let descriptors = vec![
            subtree_descriptor("/alpha/bravo"),
            direct_child_suffix_descriptor("/alpha", ".log"),
            recursive_suffix_descriptor("/alpha", ".pem"),
            recursive_literal_descriptor("/alpha", "charlie/secret.txt"),
            subtree_descriptor("/unrelated"),
        ];
        let index = MatcherIndex::build(&descriptors);
        for path in ["/", "/alpha", "/alpha/bravo", "/alpha/charlie"] {
            let path = VirtualPath::new(path);
            let expected = index
                .descendant_candidate_order(&path)
                .into_iter()
                .any(|index| descriptors[index].may_match_descendant_of(&path));
            assert_eq!(
                index.any_matching_descendant_candidate(&path, |index| descriptors[index]
                    .may_match_descendant_of(&path)),
                expected,
                "{path:?}"
            );
        }
    }
}
