use std::collections::BTreeMap;
use std::path::Path;

use crate::path::VirtualPath;

use super::descriptor::{RuleDescriptor, RuleTarget};

#[derive(Debug, Clone)]
pub(super) struct MatcherIndex {
    subtree_by_anchor: BTreeMap<VirtualPath, Vec<usize>>,
    direct_child_glob_by_anchor: BTreeMap<VirtualPath, Vec<usize>>,
    recursive_order: Vec<usize>,
    bridge_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>>,
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
        let mut bridge_descendant_by_path: BTreeMap<VirtualPath, Vec<usize>> = BTreeMap::new();
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
                    for ancestor in path_ancestors(&descriptor.anchor) {
                        bridge_descendant_by_path
                            .entry(ancestor)
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
                    for ancestor in path_ancestors(&descriptor.anchor) {
                        bridge_descendant_by_path
                            .entry(ancestor)
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
            bridge_descendant_by_path,
            order_rank,
            has_global_descendant_match,
        }
    }

    pub(super) fn candidate_order(&self, path: &VirtualPath) -> Vec<usize> {
        let mut candidates = Vec::new();
        for ancestor in path_ancestors(path) {
            if let Some(indices) = self.subtree_by_anchor.get(&ancestor) {
                push_unique(&mut candidates, indices);
            }
            if let Some(indices) = self.direct_child_glob_by_anchor.get(&ancestor) {
                push_unique(&mut candidates, indices);
            }
        }
        push_unique(&mut candidates, &self.recursive_order);
        self.sort_candidates_by_match_order(candidates)
    }

    pub(super) fn descendant_candidate_order(&self, path: &VirtualPath) -> Vec<usize> {
        let mut candidates = Vec::new();
        if let Some(indices) = self.bridge_descendant_by_path.get(path) {
            push_unique(&mut candidates, indices);
        }
        for ancestor in path_ancestors(path) {
            if let Some(indices) = self.direct_child_glob_by_anchor.get(&ancestor) {
                push_unique(&mut candidates, indices);
            }
        }
        push_unique(&mut candidates, &self.recursive_order);
        self.sort_candidates_by_match_order(candidates)
    }

    fn sort_candidates_by_match_order(&self, mut candidates: Vec<usize>) -> Vec<usize> {
        candidates.sort_by_key(|index| self.order_rank[*index]);
        candidates
    }
}

fn push_unique(out: &mut Vec<usize>, indices: &[usize]) {
    for index in indices {
        if !out.contains(index) {
            out.push(*index);
        }
    }
}

fn path_ancestors(path: &VirtualPath) -> Vec<VirtualPath> {
    let mut ancestors = Vec::new();
    let mut current = Some(path.as_path());
    while let Some(path) = current {
        ancestors.push(VirtualPath::new(path));
        if path == Path::new("/") {
            break;
        }
        current = path.parent();
    }
    ancestors
}
