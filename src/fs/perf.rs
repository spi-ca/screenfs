use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::ops::AddAssign;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub(super) struct PerfCounters {
    fuse_operations: LabeledLatencyCounters,
    policy_decisions: LatencyCounter,
    matcher_candidates: AtomicU64,
    state_read_wait: LatencyCounter,
    state_read_hold: LatencyCounter,
    state_write_wait: LatencyCounter,
    state_write_hold: LatencyCounter,
    open_confined: LatencyCounter,
    resolved_virtual_path: LatencyCounter,
    read_size_buckets: LabeledLatencyCounters,
    write_size_buckets: LabeledLatencyCounters,
    readdir_attr_generation: LatencyCounter,
    readdir_attr_entries: AtomicU64,
    readdirplus_attr_generation: LatencyCounter,
    readdirplus_attr_entries: AtomicU64,
    invalidations: AtomicU64,
    invalidated_entries: AtomicU64,
    evicted_entries: AtomicU64,
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
    pub(super) state_read_wait: LatencySnapshot,
    pub(super) state_read_hold: LatencySnapshot,
    pub(super) state_write_wait: LatencySnapshot,
    pub(super) state_write_hold: LatencySnapshot,
    pub(super) open_confined: LatencySnapshot,
    pub(super) resolved_virtual_path: LatencySnapshot,
    pub(super) read_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) write_size_buckets: BTreeMap<&'static str, LatencySnapshot>,
    pub(super) readdir_attr_generation: LatencySnapshot,
    pub(super) readdir_attr_entries: u64,
    pub(super) readdirplus_attr_generation: LatencySnapshot,
    pub(super) readdirplus_attr_entries: u64,
    pub(super) invalidations: u64,
    pub(super) invalidated_entries: u64,
    pub(super) evicted_entries: u64,
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

    pub(super) fn record_matcher_candidates(&self, count: usize) {
        self.matcher_candidates
            .fetch_add(count as u64, Ordering::Relaxed);
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

    pub(super) fn record_resolved_virtual_path(&self, elapsed: Duration) {
        self.resolved_virtual_path.record(elapsed);
    }

    pub(super) fn record_read(&self, size: usize, elapsed: Duration) {
        self.read_size_buckets.record(size_bucket(size), elapsed);
    }

    pub(super) fn record_write(&self, size: usize, elapsed: Duration) {
        self.write_size_buckets.record(size_bucket(size), elapsed);
    }

    pub(super) fn record_readdir_attr_generation(&self, entries: u64, elapsed: Duration) {
        self.readdir_attr_generation.record(elapsed);
        self.readdir_attr_entries
            .fetch_add(entries, Ordering::Relaxed);
    }

    pub(super) fn record_readdirplus_attr_generation(&self, entries: u64, elapsed: Duration) {
        self.readdirplus_attr_generation.record(elapsed);
        self.readdirplus_attr_entries
            .fetch_add(entries, Ordering::Relaxed);
    }

    pub(super) fn record_invalidation(&self, stats: InvalidationStats) {
        self.invalidations.fetch_add(1, Ordering::Relaxed);
        self.invalidated_entries
            .fetch_add(stats.invalidated_entries, Ordering::Relaxed);
        self.evicted_entries
            .fetch_add(stats.evicted_entries, Ordering::Relaxed);
    }

    pub(super) fn snapshot(&self) -> PerfSnapshot {
        PerfSnapshot {
            fuse_operations: self.fuse_operations.snapshot(),
            policy_decisions: self.policy_decisions.snapshot(),
            matcher_candidates: self.matcher_candidates.load(Ordering::Relaxed),
            state_read_wait: self.state_read_wait.snapshot(),
            state_read_hold: self.state_read_hold.snapshot(),
            state_write_wait: self.state_write_wait.snapshot(),
            state_write_hold: self.state_write_hold.snapshot(),
            open_confined: self.open_confined.snapshot(),
            resolved_virtual_path: self.resolved_virtual_path.snapshot(),
            read_size_buckets: self.read_size_buckets.snapshot(),
            write_size_buckets: self.write_size_buckets.snapshot(),
            readdir_attr_generation: self.readdir_attr_generation.snapshot(),
            readdir_attr_entries: self.readdir_attr_entries.load(Ordering::Relaxed),
            readdirplus_attr_generation: self.readdirplus_attr_generation.snapshot(),
            readdirplus_attr_entries: self.readdirplus_attr_entries.load(Ordering::Relaxed),
            invalidations: self.invalidations.load(Ordering::Relaxed),
            invalidated_entries: self.invalidated_entries.load(Ordering::Relaxed),
            evicted_entries: self.evicted_entries.load(Ordering::Relaxed),
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
        write_latency(
            &mut output,
            "resolved_virtual_path",
            snapshot.resolved_virtual_path,
        );
        write_labeled_latency(&mut output, "read_size_bucket", &snapshot.read_size_buckets);
        write_labeled_latency(
            &mut output,
            "write_size_bucket",
            &snapshot.write_size_buckets,
        );
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
            "readdirplus_attr_generation_scan",
            snapshot.readdirplus_attr_generation,
        );
        writeln!(
            &mut output,
            "  readdirplus_attr_generation_entries: count={}",
            snapshot.readdirplus_attr_entries
        )
        .expect("write to string");
        writeln!(
            &mut output,
            "  invalidations: count={} invalidated_entries={} evicted_entries={}",
            snapshot.invalidations, snapshot.invalidated_entries, snapshot.evicted_entries
        )
        .expect("write to string");
        output
    }
}

impl LatencyCounter {
    fn record(&self, elapsed: Duration) {
        self.record_many(1, elapsed);
    }

    fn record_many(&self, count: u64, elapsed: Duration) {
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

fn size_bucket(size: usize) -> &'static str {
    match size {
        0..=4096 => "0_4k",
        4097..=65536 => "4k_64k",
        65537..=1048576 => "64k_1m",
        _ => "gt_1m",
    }
}

fn write_latency(output: &mut String, name: &str, snapshot: LatencySnapshot) {
    let avg_ns = if snapshot.count == 0 {
        0
    } else {
        snapshot.total_ns / snapshot.count
    };
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
