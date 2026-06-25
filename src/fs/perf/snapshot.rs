use std::collections::BTreeMap;
use std::ops::AddAssign;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::fs) struct PerfSnapshot {
    pub(in crate::fs) fuse_operations: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) policy_decisions: LatencySnapshot,
    pub(in crate::fs) matcher_candidates: u64,
    pub(in crate::fs) matcher_candidates_by_source: BTreeMap<&'static str, u64>,
    pub(in crate::fs) matcher_family_candidates: BTreeMap<&'static str, u64>,
    pub(in crate::fs) matcher_candidate_order: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) matcher_candidate_order_by_source: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) matcher_candidate_order_duplicates: u64,
    pub(in crate::fs) matcher_candidate_order_duplicates_by_order: BTreeMap<&'static str, u64>,
    pub(in crate::fs) matcher_candidate_order_seen_slots: u64,
    pub(in crate::fs) matcher_candidate_order_seen_slots_by_order: BTreeMap<&'static str, u64>,
    pub(in crate::fs) matcher_candidate_order_ancestor_steps: u64,
    pub(in crate::fs) matcher_candidate_order_ancestor_steps_by_order: BTreeMap<&'static str, u64>,
    pub(in crate::fs) state_read_wait: LatencySnapshot,
    pub(in crate::fs) state_read_hold: LatencySnapshot,
    pub(in crate::fs) state_write_wait: LatencySnapshot,
    pub(in crate::fs) state_write_hold: LatencySnapshot,
    pub(in crate::fs) open_confined: LatencySnapshot,
    pub(in crate::fs) open_like_pre_open_guard: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) open_like_post_open_revalidation: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) stat_child_no_follow: LatencySnapshot,
    pub(in crate::fs) stat_child_no_follow_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) stat_child_no_follow_context: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) source_root_path: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_path: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_path_component_walk: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_path_canonicalize: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_path_source_root_confinement: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_path_virtual_conversion: LatencySnapshot,
    pub(in crate::fs) resolved_virtual_path_from_open_fd: LatencySnapshot,
    pub(in crate::fs) read_handle_snapshot: LatencySnapshot,
    pub(in crate::fs) read_guard_path: LatencySnapshot,
    pub(in crate::fs) read_io: LatencySnapshot,
    pub(in crate::fs) write_handle_snapshot: LatencySnapshot,
    pub(in crate::fs) write_guard_mutation: LatencySnapshot,
    pub(in crate::fs) write_io: LatencySnapshot,
    pub(in crate::fs) file_sync: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) read_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) write_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) readdir_directory_scan: LatencySnapshot,
    pub(in crate::fs) readdir_scan_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) readdir_attr_generation: LatencySnapshot,
    pub(in crate::fs) readdir_attr_entries: u64,
    pub(in crate::fs) readdir_symlink_visibility: LatencySnapshot,
    pub(in crate::fs) readdir_candidate_selection: LatencySnapshot,
    pub(in crate::fs) readdir_page_commit: LatencySnapshot,
    pub(in crate::fs) readdirplus_directory_scan: LatencySnapshot,
    pub(in crate::fs) readdirplus_scan_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(in crate::fs) readdirplus_attr_generation: LatencySnapshot,
    pub(in crate::fs) readdirplus_attr_entries: u64,
    pub(in crate::fs) readdirplus_symlink_visibility: LatencySnapshot,
    pub(in crate::fs) readdirplus_candidate_selection: LatencySnapshot,
    pub(in crate::fs) readdirplus_page_commit: LatencySnapshot,
    pub(in crate::fs) invalidations: u64,
    pub(in crate::fs) invalidated_entries: u64,
    pub(in crate::fs) evicted_entries: u64,
    pub(in crate::fs) invalidation_scanned_entries: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::fs) struct LatencySnapshot {
    pub(in crate::fs) count: u64,
    pub(in crate::fs) total_ns: u64,
    pub(in crate::fs) max_ns: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::fs) struct InvalidationStats {
    pub(in crate::fs) invalidated_entries: u64,
    pub(in crate::fs) evicted_entries: u64,
    pub(in crate::fs) scanned_entries: u64,
}

impl AddAssign for InvalidationStats {
    fn add_assign(&mut self, rhs: Self) {
        self.invalidated_entries += rhs.invalidated_entries;
        self.evicted_entries += rhs.evicted_entries;
        self.scanned_entries += rhs.scanned_entries;
    }
}
