use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::{LatencySnapshot, PerfSnapshot};

// Snapshot writers produce stable text blocks for benchmark artifacts.
pub(super) fn render_summary(snapshot: &PerfSnapshot) -> String {
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
        "matcher_candidates_by_source",
        &snapshot.matcher_candidates_by_source,
    );
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
    write_labeled_latency(
        &mut output,
        "matcher_candidate_order_by_source",
        &snapshot.matcher_candidate_order_by_source,
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
