use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use super::LatencySnapshot;

#[derive(Debug, Default)]
pub(super) struct LatencyCounter {
    count: AtomicU64,
    total_ns: AtomicU64,
    max_ns: AtomicU64,
}

#[derive(Debug, Default)]
pub(super) struct LabeledLatencyCounters {
    counters: Mutex<BTreeMap<&'static str, LatencyTotals>>,
}

#[derive(Debug, Default)]
pub(super) struct LabeledCountCounters {
    counters: Mutex<BTreeMap<&'static str, u64>>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LatencyTotals {
    count: u64,
    total_ns: u64,
    max_ns: u64,
}

// Counter primitives keep aggregation lock scope small.
impl LatencyCounter {
    pub(super) fn record(&self, elapsed: Duration) {
        self.record_many(1, elapsed);
    }

    pub(super) fn record_many(&self, count: u64, elapsed: Duration) {
        if count == 0 {
            return;
        }
        self.count.fetch_add(count, Ordering::Relaxed);
        let ns = elapsed.as_nanos().min(u128::from(u64::MAX)) as u64;
        self.total_ns.fetch_add(ns, Ordering::Relaxed);
        self.max_ns.fetch_max(ns, Ordering::Relaxed);
    }

    pub(super) fn snapshot(&self) -> LatencySnapshot {
        LatencySnapshot {
            count: self.count.load(Ordering::Relaxed),
            total_ns: self.total_ns.load(Ordering::Relaxed),
            max_ns: self.max_ns.load(Ordering::Relaxed),
        }
    }
}

impl LabeledLatencyCounters {
    pub(super) fn record(&self, label: &'static str, elapsed: Duration) {
        let ns = elapsed.as_nanos().min(u128::from(u64::MAX)) as u64;
        let mut counters = self.counters.lock().expect("perf counter mutex poisoned");
        let totals = counters.entry(label).or_default();
        totals.count += 1;
        totals.total_ns = totals.total_ns.saturating_add(ns);
        totals.max_ns = totals.max_ns.max(ns);
    }

    pub(super) fn snapshot(&self) -> BTreeMap<&'static str, LatencySnapshot> {
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
    pub(super) fn record(&self, label: &'static str, count: u64) {
        let mut counters = self.counters.lock().expect("perf counter mutex poisoned");
        let total = counters.entry(label).or_default();
        *total = total.saturating_add(count);
    }

    pub(super) fn snapshot(&self) -> BTreeMap<&'static str, u64> {
        self.counters
            .lock()
            .expect("perf counter mutex poisoned")
            .clone()
    }
}

pub(super) fn size_bucket(size: usize) -> &'static str {
    match size {
        0..=4096 => "0_4k",
        4097..=65536 => "4k_64k",
        65537..=1048576 => "64k_1m",
        _ => "gt_1m",
    }
}
