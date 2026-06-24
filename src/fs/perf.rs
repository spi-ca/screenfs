//! Optional performance counters for the `perf-counters` feature.
//!
//! Counters are grouped by request path, policy/matcher work, state locking, and
//! backing I/O splits so benchmark artifacts can attribute overhead.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::ops::AddAssign;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use crate::matcher::MatcherCandidateMetrics;
use crate::path::ResolveHostPathMetrics;

// PerfCounters stores raw measurements; formatting into artifacts happens on snapshot output.
#[derive(Debug, Default)]
pub(super) struct PerfCounters {
    fuse_operations: LabeledLatencyCounters,
    policy_decisions: LatencyCounter,
    matcher_candidates: AtomicU64,
    matcher_family_candidates: LabeledCountCounters,
    matcher_candidate_order: LabeledLatencyCounters,
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

#[derive(Debug, Default)]
struct LatencyCounter {
    count: AtomicU64,
    total_ns: AtomicU64,
    max_ns: AtomicU64,
}

#[derive(Debug, Default)]
struct LabeledLatencyCounters {
    counters: Mutex<BTreeMap<&'static str, LatencyTotals>>,
}

#[derive(Debug, Default)]
struct LabeledCountCounters {
    counters: Mutex<BTreeMap<&'static str, u64>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LatencyTotals {
    count: u64,
    total_ns: u64,
    max_ns: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PerfSnapshot {
    pub(super) fuse_operations: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) policy_decisions: LatencySnapshot,
    pub(super) matcher_candidates: u64,
    pub(super) matcher_family_candidates: BTreeMap<&'static str, u64>,
    pub(super) matcher_candidate_order: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) matcher_candidate_order_duplicates: u64,
    pub(super) matcher_candidate_order_duplicates_by_order: BTreeMap<&'static str, u64>,
    pub(super) matcher_candidate_order_seen_slots: u64,
    pub(super) matcher_candidate_order_seen_slots_by_order: BTreeMap<&'static str, u64>,
    pub(super) matcher_candidate_order_ancestor_steps: u64,
    pub(super) matcher_candidate_order_ancestor_steps_by_order: BTreeMap<&'static str, u64>,
    pub(super) state_read_wait: LatencySnapshot,
    pub(super) state_read_hold: LatencySnapshot,
    pub(super) state_write_wait: LatencySnapshot,
    pub(super) state_write_hold: LatencySnapshot,
    pub(super) open_confined: LatencySnapshot,
    pub(super) open_like_pre_open_guard: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) open_like_post_open_revalidation: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) stat_child_no_follow: LatencySnapshot,
    pub(super) stat_child_no_follow_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) stat_child_no_follow_context: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) source_root_path: LatencySnapshot,
    pub(super) resolved_virtual_path: LatencySnapshot,
    pub(super) resolved_virtual_path_from_path: LatencySnapshot,
    pub(super) resolved_virtual_path_from_path_component_walk: LatencySnapshot,
    pub(super) resolved_virtual_path_from_path_canonicalize: LatencySnapshot,
    pub(super) resolved_virtual_path_from_path_source_root_confinement: LatencySnapshot,
    pub(super) resolved_virtual_path_from_path_virtual_conversion: LatencySnapshot,
    pub(super) resolved_virtual_path_from_open_fd: LatencySnapshot,
    pub(super) read_handle_snapshot: LatencySnapshot,
    pub(super) read_guard_path: LatencySnapshot,
    pub(super) read_io: LatencySnapshot,
    pub(super) write_handle_snapshot: LatencySnapshot,
    pub(super) write_guard_mutation: LatencySnapshot,
    pub(super) write_io: LatencySnapshot,
    pub(super) file_sync: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) read_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) write_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) readdir_directory_scan: LatencySnapshot,
    pub(super) readdir_scan_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) readdir_attr_generation: LatencySnapshot,
    pub(super) readdir_attr_entries: u64,
    pub(super) readdir_symlink_visibility: LatencySnapshot,
    pub(super) readdir_candidate_selection: LatencySnapshot,
    pub(super) readdir_page_commit: LatencySnapshot,
    pub(super) readdirplus_directory_scan: LatencySnapshot,
    pub(super) readdirplus_scan_splits: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) readdirplus_attr_generation: LatencySnapshot,
    pub(super) readdirplus_attr_entries: u64,
    pub(super) readdirplus_symlink_visibility: LatencySnapshot,
    pub(super) readdirplus_candidate_selection: LatencySnapshot,
    pub(super) readdirplus_page_commit: LatencySnapshot,
    pub(super) invalidations: u64,
    pub(super) invalidated_entries: u64,
    pub(super) evicted_entries: u64,
    pub(super) invalidation_scanned_entries: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LatencySnapshot {
    pub(super) count: u64,
    pub(super) total_ns: u64,
    pub(super) max_ns: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct InvalidationStats {
    pub(super) invalidated_entries: u64,
    pub(super) evicted_entries: u64,
    pub(super) scanned_entries: u64,
}

pub(super) struct FuseOpTimer<'a> {
    perf: &'a PerfCounters,
    name: &'static str,
    start: Instant,
}

impl AddAssign for InvalidationStats {
    fn add_assign(&mut self, rhs: Self) {
        self.invalidated_entries += rhs.invalidated_entries;
        self.evicted_entries += rhs.evicted_entries;
        self.scanned_entries += rhs.scanned_entries;
    }
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
            matcher_family_candidates: self.matcher_family_candidates.snapshot(),
            matcher_candidate_order: self.matcher_candidate_order.snapshot(),
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
        let snapshot = self.snapshot();
        let mut output = String::new();
        writeln!(&mut output, "screenfs perf counters:").expect("write to string");
        write_labeled_latency(&mut output, "fuse_op", &snapshot.fuse_operations);
        write_latency(&mut output, "policy_decision", snapshot.policy_decisions);
        writeln!(
            &mut output,
            "  matcher_candidates: count={}",
            snapshot.matcher_candidates
        )
        .expect("write to string");
        write_labeled_counts(
            &mut output,
            "matcher_family_candidates",
            &snapshot.matcher_family_candidates,
        );
        write_labeled_latency(
            &mut output,
            "matcher_candidate_order",
            &snapshot.matcher_candidate_order,
        );
        writeln!(
            &mut output,
            "  matcher_candidate_order_duplicates: count={}",
            snapshot.matcher_candidate_order_duplicates
        )
        .expect("write to string");
        write_labeled_counts(
            &mut output,
            "matcher_candidate_order_duplicates",
            &snapshot.matcher_candidate_order_duplicates_by_order,
        );
        writeln!(
            &mut output,
            "  matcher_candidate_order_seen_slots: count={}",
            snapshot.matcher_candidate_order_seen_slots
        )
        .expect("write to string");
        write_labeled_counts(
            &mut output,
            "matcher_candidate_order_seen_slots",
            &snapshot.matcher_candidate_order_seen_slots_by_order,
        );
        writeln!(
            &mut output,
            "  matcher_candidate_order_ancestor_steps: count={}",
            snapshot.matcher_candidate_order_ancestor_steps
        )
        .expect("write to string");
        write_labeled_counts(
            &mut output,
            "matcher_candidate_order_ancestor_steps",
            &snapshot.matcher_candidate_order_ancestor_steps_by_order,
        );
        write_latency(
            &mut output,
            "state_read_lock_wait",
            snapshot.state_read_wait,
        );
        write_latency(
            &mut output,
            "state_read_lock_hold",
            snapshot.state_read_hold,
        );
        write_latency(
            &mut output,
            "state_write_lock_wait",
            snapshot.state_write_wait,
        );
        write_latency(
            &mut output,
            "state_write_lock_hold",
            snapshot.state_write_hold,
        );
        write_latency(&mut output, "open_confined_openat2", snapshot.open_confined);
        write_labeled_latency(
            &mut output,
            "open_like.pre_open_guard",
            &snapshot.open_like_pre_open_guard,
        );
        write_labeled_latency(
            &mut output,
            "open_like.post_open_revalidation",
            &snapshot.open_like_post_open_revalidation,
        );
        write_latency(
            &mut output,
            "stat_child_no_follow",
            snapshot.stat_child_no_follow,
        );
        write_labeled_latency(
            &mut output,
            "stat_child_no_follow",
            &snapshot.stat_child_no_follow_splits,
        );
        write_labeled_latency(
            &mut output,
            "stat_child_no_follow_context",
            &snapshot.stat_child_no_follow_context,
        );
        write_latency(&mut output, "source_root_path", snapshot.source_root_path);
        write_latency(
            &mut output,
            "resolved_virtual_path",
            snapshot.resolved_virtual_path,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_path",
            snapshot.resolved_virtual_path_from_path,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_path_component_walk",
            snapshot.resolved_virtual_path_from_path_component_walk,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_path_canonicalize",
            snapshot.resolved_virtual_path_from_path_canonicalize,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_path_source_root_confinement",
            snapshot.resolved_virtual_path_from_path_source_root_confinement,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_path_virtual_conversion",
            snapshot.resolved_virtual_path_from_path_virtual_conversion,
        );
        write_latency(
            &mut output,
            "resolved_virtual_path_from_open_fd",
            snapshot.resolved_virtual_path_from_open_fd,
        );
        write_latency(
            &mut output,
            "read_handle_snapshot",
            snapshot.read_handle_snapshot,
        );
        write_latency(&mut output, "read_guard_path", snapshot.read_guard_path);
        write_latency(&mut output, "read_io", snapshot.read_io);
        write_latency(
            &mut output,
            "write_handle_snapshot",
            snapshot.write_handle_snapshot,
        );
        write_latency(
            &mut output,
            "write_guard_mutation",
            snapshot.write_guard_mutation,
        );
        write_latency(&mut output, "write_io", snapshot.write_io);
        write_labeled_latency(&mut output, "file_sync", &snapshot.file_sync);
        write_labeled_latency(&mut output, "read_size_bucket", &snapshot.read_size_buckets);
        write_labeled_latency(
            &mut output,
            "write_size_bucket",
            &snapshot.write_size_buckets,
        );
        write_latency(
            &mut output,
            "readdir_directory_scan",
            snapshot.readdir_directory_scan,
        );
        write_labeled_latency(&mut output, "readdir_scan", &snapshot.readdir_scan_splits);
        write_latency(
            &mut output,
            "readdir_attr_generation_scan",
            snapshot.readdir_attr_generation,
        );
        writeln!(
            &mut output,
            "  readdir_attr_generation_entries: count={}",
            snapshot.readdir_attr_entries
        )
        .expect("write to string");
        write_latency(
            &mut output,
            "readdir_symlink_visibility",
            snapshot.readdir_symlink_visibility,
        );
        write_latency(
            &mut output,
            "readdir_candidate_selection",
            snapshot.readdir_candidate_selection,
        );
        write_latency(
            &mut output,
            "readdir_page_commit",
            snapshot.readdir_page_commit,
        );
        write_latency(
            &mut output,
            "readdirplus_directory_scan",
            snapshot.readdirplus_directory_scan,
        );
        write_labeled_latency(
            &mut output,
            "readdirplus_scan",
            &snapshot.readdirplus_scan_splits,
        );
        write_latency(
            &mut output,
            "readdirplus_attr_generation_scan",
            snapshot.readdirplus_attr_generation,
        );
        writeln!(
            &mut output,
            "  readdirplus_attr_generation_entries: count={}",
            snapshot.readdirplus_attr_entries
        )
        .expect("write to string");
        write_latency(
            &mut output,
            "readdirplus_symlink_visibility",
            snapshot.readdirplus_symlink_visibility,
        );
        write_latency(
            &mut output,
            "readdirplus_candidate_selection",
            snapshot.readdirplus_candidate_selection,
        );
        write_latency(
            &mut output,
            "readdirplus_page_commit",
            snapshot.readdirplus_page_commit,
        );
        writeln!(
            &mut output,
            "  invalidations: count={} invalidated_entries={} evicted_entries={} scanned_entries={}",
            snapshot.invalidations,
            snapshot.invalidated_entries,
            snapshot.evicted_entries,
            snapshot.invalidation_scanned_entries
        )
        .expect("write to string");
        output
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

// Counter primitives keep aggregation lock scope small.
impl LatencyCounter {
    fn record(&self, elapsed: Duration) {
        self.record_many(1, elapsed);
    }

    fn record_many(&self, count: u64, elapsed: Duration) {
        if count == 0 {
            return;
        }
        self.count.fetch_add(count, Ordering::Relaxed);
        let ns = elapsed.as_nanos().min(u128::from(u64::MAX)) as u64;
        self.total_ns.fetch_add(ns, Ordering::Relaxed);
        self.max_ns.fetch_max(ns, Ordering::Relaxed);
    }

    fn snapshot(&self) -> LatencySnapshot {
        LatencySnapshot {
            count: self.count.load(Ordering::Relaxed),
            total_ns: self.total_ns.load(Ordering::Relaxed),
            max_ns: self.max_ns.load(Ordering::Relaxed),
        }
    }
}

impl LabeledLatencyCounters {
    fn record(&self, label: &'static str, elapsed: Duration) {
        let ns = elapsed.as_nanos().min(u128::from(u64::MAX)) as u64;
        let mut counters = self.counters.lock().expect("perf counter mutex poisoned");
        let totals = counters.entry(label).or_default();
        totals.count += 1;
        totals.total_ns = totals.total_ns.saturating_add(ns);
        totals.max_ns = totals.max_ns.max(ns);
    }

    fn snapshot(&self) -> BTreeMap<&'static str, LatencySnapshot> {
        self.counters
            .lock()
            .expect("perf counter mutex poisoned")
            .iter()
            .map(|(label, totals)| {
                (
                    *label,
                    LatencySnapshot {
                        count: totals.count,
                        total_ns: totals.total_ns,
                        max_ns: totals.max_ns,
                    },
                )
            })
            .collect()
    }
}

impl LabeledCountCounters {
    fn record(&self, label: &'static str, count: u64) {
        let mut counters = self.counters.lock().expect("perf counter mutex poisoned");
        let total = counters.entry(label).or_default();
        *total = total.saturating_add(count);
    }

    fn snapshot(&self) -> BTreeMap<&'static str, u64> {
        self.counters
            .lock()
            .expect("perf counter mutex poisoned")
            .clone()
    }
}

fn size_bucket(size: usize) -> &'static str {
    match size {
        0..=4096 => "0_4k",
        4097..=65536 => "4k_64k",
        65537..=1048576 => "64k_1m",
        _ => "gt_1m",
    }
}

// Snapshot writers produce stable text blocks for benchmark artifacts.
fn write_latency(output: &mut String, name: &str, snapshot: LatencySnapshot) {
    let avg_ns = snapshot.total_ns.checked_div(snapshot.count).unwrap_or(0);
    writeln!(
        output,
        "  {name}: count={} total_ns={} avg_ns={} max_ns={}",
        snapshot.count, snapshot.total_ns, avg_ns, snapshot.max_ns
    )
    .expect("write to string");
}

fn write_labeled_latency(
    output: &mut String,
    prefix: &str,
    snapshots: &BTreeMap<&'static str, LatencySnapshot>,
) {
    for (label, snapshot) in snapshots {
        write_latency(output, &format!("{prefix}.{label}"), *snapshot);
    }
}

fn write_labeled_counts(
    output: &mut String,
    prefix: &str,
    snapshots: &BTreeMap<&'static str, u64>,
) {
    for (label, count) in snapshots {
        writeln!(output, "  {prefix}.{label}: count={count}").expect("write to string");
    }
}
