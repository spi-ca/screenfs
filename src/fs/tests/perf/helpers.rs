use super::super::*;

// Small accessors keep assertions focused on the perf split being tested.
pub(super) fn file_sync_count(snapshot: &PerfSnapshot, label: &'static str) -> u64 {
    snapshot
        .file_sync
        .get(label)
        .map_or(0, |counter| counter.count)
}

pub(super) fn labeled_latency_count(
    counters: &std::collections::BTreeMap<&'static str, crate::fs::perf::LatencySnapshot>,
    label: &'static str,
) -> u64 {
    counters.get(label).map_or(0, |counter| counter.count)
}
