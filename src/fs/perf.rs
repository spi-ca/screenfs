//! Optional performance counters for the `perf-counters` feature.
//!
//! Counters are grouped by request path, policy/matcher work, state locking, and
//! backing I/O splits so benchmark artifacts can attribute overhead.

mod counters;
mod snapshot;
mod summary;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::matcher::MatcherCandidateMetrics;
use crate::path::ResolveHostPathMetrics;

use self::counters::{LabeledCountCounters, LabeledLatencyCounters, LatencyCounter, size_bucket};
pub(super) use self::snapshot::{InvalidationStats, LatencySnapshot, PerfSnapshot};
use self::summary::render_summary;

// PerfCounters stores raw measurements; formatting into artifacts happens on snapshot output.
#[derive(Debug, Default)]
pub(super) struct PerfCounters {
    fuse_operations: LabeledLatencyCounters,
    policy_decisions: LatencyCounter,
    matcher_candidates: AtomicU64,
    matcher_candidates_by_source: LabeledCountCounters,
    matcher_family_candidates: LabeledCountCounters,
    matcher_candidate_order: LabeledLatencyCounters,
    matcher_candidate_order_by_source: LabeledLatencyCounters,
    matcher_candidate_order_duplicates: AtomicU64,
    matcher_candidate_order_duplicates_by_order: LabeledCountCounters,
    matcher_candidate_order_seen_slots: AtomicU64,
    matcher_candidate_order_seen_slots_by_order: LabeledCountCounters,
    matcher_candidate_order_ancestor_steps: AtomicU64,
    matcher_candidate_order_ancestor_steps_by_order: LabeledCountCounters,
    state_read_wait: LatencyCounter,
    state_read_hold: LatencyCounter,
    state_write_wait: LatencyCounter,
    state_write_hold: LatencyCounter,
    open_confined: LatencyCounter,
    open_like_pre_open_guard: LabeledLatencyCounters,
    open_like_post_open_revalidation: LabeledLatencyCounters,
    stat_child_no_follow: LatencyCounter,
    stat_child_no_follow_splits: LabeledLatencyCounters,
    stat_child_no_follow_context: LabeledLatencyCounters,
    source_root_path: LatencyCounter,
    resolved_virtual_path: LatencyCounter,
    resolved_virtual_path_from_path: LatencyCounter,
    resolved_virtual_path_from_path_component_walk: LatencyCounter,
    resolved_virtual_path_from_path_canonicalize: LatencyCounter,
    resolved_virtual_path_from_path_source_root_confinement: LatencyCounter,
    resolved_virtual_path_from_path_virtual_conversion: LatencyCounter,
    resolved_virtual_path_from_open_fd: LatencyCounter,
    read_handle_snapshot: LatencyCounter,
    read_guard_path: LatencyCounter,
    read_io: LatencyCounter,
    write_handle_snapshot: LatencyCounter,
    write_guard_mutation: LatencyCounter,
    write_io: LatencyCounter,
    file_sync: LabeledLatencyCounters,
    read_size_buckets: LabeledLatencyCounters,
    write_size_buckets: LabeledLatencyCounters,
    readdir_directory_scan: LatencyCounter,
    readdir_scan_splits: LabeledLatencyCounters,
    readdir_attr_generation: LatencyCounter,
    readdir_attr_entries: AtomicU64,
    readdir_symlink_visibility: LatencyCounter,
    readdir_candidate_selection: LatencyCounter,
    readdir_page_commit: LatencyCounter,
    readdirplus_directory_scan: LatencyCounter,
    readdirplus_scan_splits: LabeledLatencyCounters,
    readdirplus_attr_generation: LatencyCounter,
    readdirplus_attr_entries: AtomicU64,
    readdirplus_symlink_visibility: LatencyCounter,
    readdirplus_candidate_selection: LatencyCounter,
    readdirplus_page_commit: LatencyCounter,
    invalidations: AtomicU64,
    invalidated_entries: AtomicU64,
    evicted_entries: AtomicU64,
    invalidation_scanned_entries: AtomicU64,
}

pub(super) struct FuseOpTimer<'a> {
    perf: &'a PerfCounters,
    name: &'static str,
    start: Instant,
}

impl<'a> FuseOpTimer<'a> {
    pub(super) fn new(perf: &'a PerfCounters, name: &'static str) -> Self {
        Self {
            perf,
            name,
            start: Instant::now(),
        }
    }
}

impl Drop for FuseOpTimer<'_> {
    fn drop(&mut self) {
        self.perf
            .record_fuse_operation(self.name, self.start.elapsed());
    }
}

// Recording helpers are intentionally tiny so call sites can mark exact request phases.
impl PerfCounters {
    pub(super) fn fuse_op_timer(&self, name: &'static str) -> FuseOpTimer<'_> {
        FuseOpTimer::new(self, name)
    }

    pub(super) fn record_fuse_operation(&self, name: &'static str, elapsed: Duration) {
        self.fuse_operations.record(name, elapsed);
    }

    pub(super) fn record_policy_decision(&self, elapsed: Duration) {
        self.policy_decisions.record(elapsed);
    }

    pub(super) fn record_matcher_candidates_by_source(
        &self,
        source: &'static str,
        order: &'static str,
        metrics: MatcherCandidateMetrics,
    ) {
        self.matcher_candidates_by_source
            .record(source, metrics.count as u64);
        self.matcher_candidate_order_by_source
            .record(source, metrics.candidate_order.elapsed);
        self.record_matcher_candidates(order, metrics);
    }

    pub(super) fn record_matcher_candidates(
        &self,
        order: &'static str,
        metrics: MatcherCandidateMetrics,
    ) {
        self.matcher_candidates
            .fetch_add(metrics.count as u64, Ordering::Relaxed);
        self.matcher_family_candidates
            .record("subtree", metrics.family_counts.subtree as u64);
        self.matcher_family_candidates.record(
            "direct_child_glob",
            metrics.family_counts.direct_child_glob as u64,
        );
        self.matcher_family_candidates
            .record("recursive", metrics.family_counts.recursive as u64);
        self.matcher_candidate_order
            .record(order, metrics.candidate_order.elapsed);
        let duplicates = metrics.candidate_order.duplicates_skipped as u64;
        self.matcher_candidate_order_duplicates
            .fetch_add(duplicates, Ordering::Relaxed);
        self.matcher_candidate_order_duplicates_by_order
            .record(order, duplicates);
        let seen_slots = metrics.candidate_order.seen_slots as u64;
        self.matcher_candidate_order_seen_slots
            .fetch_add(seen_slots, Ordering::Relaxed);
        self.matcher_candidate_order_seen_slots_by_order
            .record(order, seen_slots);
        let ancestor_steps = metrics.candidate_order.ancestor_steps as u64;
        self.matcher_candidate_order_ancestor_steps
            .fetch_add(ancestor_steps, Ordering::Relaxed);
        self.matcher_candidate_order_ancestor_steps_by_order
            .record(order, ancestor_steps);
    }

    pub(super) fn record_state_read_lock(&self, wait: Duration, hold: Duration) {
        self.state_read_wait.record(wait);
        self.state_read_hold.record(hold);
    }

    pub(super) fn record_state_write_lock(&self, wait: Duration, hold: Duration) {
        self.state_write_wait.record(wait);
        self.state_write_hold.record(hold);
    }

    pub(super) fn record_open_confined(&self, elapsed: Duration) {
        self.open_confined.record(elapsed);
    }

    pub(super) fn record_open_like_pre_open_guard(
        &self,
        operation: &'static str,
        elapsed: Duration,
    ) {
        self.open_like_pre_open_guard.record(operation, elapsed);
    }

    pub(super) fn record_open_like_post_open_revalidation(
        &self,
        operation: &'static str,
        elapsed: Duration,
    ) {
        self.open_like_post_open_revalidation
            .record(operation, elapsed);
    }

    pub(super) fn record_stat_child_no_follow(&self, elapsed: Duration) {
        self.stat_child_no_follow.record(elapsed);
    }

    pub(super) fn record_stat_child_no_follow_split(&self, label: &'static str, elapsed: Duration) {
        self.stat_child_no_follow_splits.record(label, elapsed);
    }

    pub(super) fn record_stat_child_no_follow_context(
        &self,
        label: &'static str,
        elapsed: Duration,
    ) {
        self.stat_child_no_follow_context.record(label, elapsed);
    }

    pub(super) fn record_source_root_path(&self, elapsed: Duration) {
        self.source_root_path.record(elapsed);
    }

    pub(super) fn record_resolved_virtual_path_from_path(&self, elapsed: Duration) {
        self.resolved_virtual_path.record(elapsed);
        self.resolved_virtual_path_from_path.record(elapsed);
    }

    pub(super) fn record_resolved_virtual_path_from_path_details(
        &self,
        metrics: ResolveHostPathMetrics,
        virtual_conversion: Duration,
    ) {
        self.resolved_virtual_path_from_path_component_walk
            .record(metrics.component_walk);
        self.resolved_virtual_path_from_path_canonicalize
            .record_many(metrics.canonicalize_count, metrics.canonicalize_total);
        self.resolved_virtual_path_from_path_source_root_confinement
            .record_many(
                metrics.source_root_confinement_count,
                metrics.source_root_confinement_total,
            );
        self.resolved_virtual_path_from_path_virtual_conversion
            .record(virtual_conversion);
    }

    pub(super) fn record_resolved_virtual_path_from_open_fd(&self, elapsed: Duration) {
        self.resolved_virtual_path.record(elapsed);
        self.resolved_virtual_path_from_open_fd.record(elapsed);
    }

    pub(super) fn record_read_handle_snapshot(&self, elapsed: Duration) {
        self.read_handle_snapshot.record(elapsed);
    }

    pub(super) fn record_read_guard_path(&self, elapsed: Duration) {
        self.read_guard_path.record(elapsed);
    }

    pub(super) fn record_read_io(&self, elapsed: Duration) {
        self.read_io.record(elapsed);
    }

    pub(super) fn record_read_size_bucket(&self, size: usize, elapsed: Duration) {
        self.read_size_buckets.record(size_bucket(size), elapsed);
    }

    pub(super) fn record_write_handle_snapshot(&self, elapsed: Duration) {
        self.write_handle_snapshot.record(elapsed);
    }

    pub(super) fn record_write_guard_mutation(&self, elapsed: Duration) {
        self.write_guard_mutation.record(elapsed);
    }

    pub(super) fn record_write_io(&self, elapsed: Duration) {
        self.write_io.record(elapsed);
    }

    pub(super) fn record_write_size_bucket(&self, size: usize, elapsed: Duration) {
        self.write_size_buckets.record(size_bucket(size), elapsed);
    }

    pub(super) fn record_file_sync(&self, label: &'static str, elapsed: Duration) {
        self.file_sync.record(label, elapsed);
    }

    pub(super) fn record_readdir_directory_scan(&self, elapsed: Duration) {
        self.readdir_directory_scan.record(elapsed);
    }

    pub(super) fn record_readdirplus_directory_scan(&self, elapsed: Duration) {
        self.readdirplus_directory_scan.record(elapsed);
    }

    pub(super) fn record_readdir_scan_split(&self, label: &'static str, elapsed: Duration) {
        self.readdir_scan_splits.record(label, elapsed);
    }

    pub(super) fn record_readdirplus_scan_split(&self, label: &'static str, elapsed: Duration) {
        self.readdirplus_scan_splits.record(label, elapsed);
    }

    pub(super) fn record_readdir_attr_generation(&self, entries: u64, elapsed: Duration) {
        self.record_directory_attr_generation(false, entries, elapsed);
    }

    pub(super) fn record_readdirplus_attr_generation(&self, entries: u64, elapsed: Duration) {
        self.record_directory_attr_generation(true, entries, elapsed);
    }

    pub(super) fn record_readdir_symlink_visibility(&self, elapsed: Duration) {
        self.readdir_symlink_visibility.record(elapsed);
    }

    pub(super) fn record_readdirplus_symlink_visibility(&self, elapsed: Duration) {
        self.readdirplus_symlink_visibility.record(elapsed);
    }

    pub(super) fn record_readdir_candidate_selection(&self, elapsed: Duration) {
        self.readdir_candidate_selection.record(elapsed);
    }

    pub(super) fn record_readdirplus_candidate_selection(&self, elapsed: Duration) {
        self.readdirplus_candidate_selection.record(elapsed);
    }

    pub(super) fn record_readdir_page_commit(&self, elapsed: Duration) {
        self.readdir_page_commit.record(elapsed);
    }

    pub(super) fn record_readdirplus_page_commit(&self, elapsed: Duration) {
        self.readdirplus_page_commit.record(elapsed);
    }

    pub(super) fn record_invalidation(&self, stats: InvalidationStats) {
        self.invalidations.fetch_add(1, Ordering::Relaxed);
        self.invalidated_entries
            .fetch_add(stats.invalidated_entries, Ordering::Relaxed);
        self.evicted_entries
            .fetch_add(stats.evicted_entries, Ordering::Relaxed);
        self.invalidation_scanned_entries
            .fetch_add(stats.scanned_entries, Ordering::Relaxed);
    }

    pub(super) fn snapshot(&self) -> PerfSnapshot {
        PerfSnapshot {
            fuse_operations: self.fuse_operations.snapshot(),
            policy_decisions: self.policy_decisions.snapshot(),
            matcher_candidates: self.matcher_candidates.load(Ordering::Relaxed),
            matcher_candidates_by_source: self.matcher_candidates_by_source.snapshot(),
            matcher_family_candidates: self.matcher_family_candidates.snapshot(),
            matcher_candidate_order: self.matcher_candidate_order.snapshot(),
            matcher_candidate_order_by_source: self.matcher_candidate_order_by_source.snapshot(),
            matcher_candidate_order_duplicates: self
                .matcher_candidate_order_duplicates
                .load(Ordering::Relaxed),
            matcher_candidate_order_duplicates_by_order: self
                .matcher_candidate_order_duplicates_by_order
                .snapshot(),
            matcher_candidate_order_seen_slots: self
                .matcher_candidate_order_seen_slots
                .load(Ordering::Relaxed),
            matcher_candidate_order_seen_slots_by_order: self
                .matcher_candidate_order_seen_slots_by_order
                .snapshot(),
            matcher_candidate_order_ancestor_steps: self
                .matcher_candidate_order_ancestor_steps
                .load(Ordering::Relaxed),
            matcher_candidate_order_ancestor_steps_by_order: self
                .matcher_candidate_order_ancestor_steps_by_order
                .snapshot(),
            state_read_wait: self.state_read_wait.snapshot(),
            state_read_hold: self.state_read_hold.snapshot(),
            state_write_wait: self.state_write_wait.snapshot(),
            state_write_hold: self.state_write_hold.snapshot(),
            open_confined: self.open_confined.snapshot(),
            open_like_pre_open_guard: self.open_like_pre_open_guard.snapshot(),
            open_like_post_open_revalidation: self.open_like_post_open_revalidation.snapshot(),
            stat_child_no_follow: self.stat_child_no_follow.snapshot(),
            stat_child_no_follow_splits: self.stat_child_no_follow_splits.snapshot(),
            stat_child_no_follow_context: self.stat_child_no_follow_context.snapshot(),
            source_root_path: self.source_root_path.snapshot(),
            resolved_virtual_path: self.resolved_virtual_path.snapshot(),
            resolved_virtual_path_from_path: self.resolved_virtual_path_from_path.snapshot(),
            resolved_virtual_path_from_path_component_walk: self
                .resolved_virtual_path_from_path_component_walk
                .snapshot(),
            resolved_virtual_path_from_path_canonicalize: self
                .resolved_virtual_path_from_path_canonicalize
                .snapshot(),
            resolved_virtual_path_from_path_source_root_confinement: self
                .resolved_virtual_path_from_path_source_root_confinement
                .snapshot(),
            resolved_virtual_path_from_path_virtual_conversion: self
                .resolved_virtual_path_from_path_virtual_conversion
                .snapshot(),
            resolved_virtual_path_from_open_fd: self.resolved_virtual_path_from_open_fd.snapshot(),
            read_handle_snapshot: self.read_handle_snapshot.snapshot(),
            read_guard_path: self.read_guard_path.snapshot(),
            read_io: self.read_io.snapshot(),
            write_handle_snapshot: self.write_handle_snapshot.snapshot(),
            write_guard_mutation: self.write_guard_mutation.snapshot(),
            write_io: self.write_io.snapshot(),
            file_sync: self.file_sync.snapshot(),
            read_size_buckets: self.read_size_buckets.snapshot(),
            write_size_buckets: self.write_size_buckets.snapshot(),
            readdir_directory_scan: self.readdir_directory_scan.snapshot(),
            readdir_scan_splits: self.readdir_scan_splits.snapshot(),
            readdir_attr_generation: self.readdir_attr_generation.snapshot(),
            readdir_attr_entries: self.readdir_attr_entries.load(Ordering::Relaxed),
            readdir_symlink_visibility: self.readdir_symlink_visibility.snapshot(),
            readdir_candidate_selection: self.readdir_candidate_selection.snapshot(),
            readdir_page_commit: self.readdir_page_commit.snapshot(),
            readdirplus_directory_scan: self.readdirplus_directory_scan.snapshot(),
            readdirplus_scan_splits: self.readdirplus_scan_splits.snapshot(),
            readdirplus_attr_generation: self.readdirplus_attr_generation.snapshot(),
            readdirplus_attr_entries: self.readdirplus_attr_entries.load(Ordering::Relaxed),
            readdirplus_symlink_visibility: self.readdirplus_symlink_visibility.snapshot(),
            readdirplus_candidate_selection: self.readdirplus_candidate_selection.snapshot(),
            readdirplus_page_commit: self.readdirplus_page_commit.snapshot(),
            invalidations: self.invalidations.load(Ordering::Relaxed),
            invalidated_entries: self.invalidated_entries.load(Ordering::Relaxed),
            evicted_entries: self.evicted_entries.load(Ordering::Relaxed),
            invalidation_scanned_entries: self.invalidation_scanned_entries.load(Ordering::Relaxed),
        }
    }

    pub(super) fn summary(&self) -> String {
        render_summary(&self.snapshot())
    }

    fn record_directory_attr_generation(&self, with_plus: bool, entries: u64, elapsed: Duration) {
        let (counter, entry_counter) = if with_plus {
            (
                &self.readdirplus_attr_generation,
                &self.readdirplus_attr_entries,
            )
        } else {
            (&self.readdir_attr_generation, &self.readdir_attr_entries)
        };
        counter.record(elapsed);
        entry_counter.fetch_add(entries, Ordering::Relaxed);
    }
}
