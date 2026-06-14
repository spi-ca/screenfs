use std::collections::BTreeMap;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::path::VirtualPath;

use super::descriptor::{RuleDescriptor, RuleTarget};

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
        candidates.sort_by_key(|index| self.order_rank[*index]);
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
