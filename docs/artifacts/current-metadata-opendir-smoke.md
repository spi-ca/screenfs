# ScreenFS benchmark result

- timestamp: `2026-06-20T16:53:34.181886+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --workload metadata_opendir --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-metadata-opendir-smoke.json --output-md docs/artifacts/current-metadata-opendir-smoke.md --output-svg docs/artifacts/current-metadata-opendir-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json;  M docs/artifacts/current-open-confined-surface-smoke.md;  M docs/artifacts/current-open-confined-surface-smoke.svg;  M docs/artifacts/current-policy-heavy-matrix-smoke.json; ... (+15 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json;  M docs/artifacts/current-open-confined-surface-smoke.md;  M docs/artifacts/current-open-confined-surface-smoke.svg;  M docs/artifacts/current-policy-heavy-matrix-smoke.json; ... (+15 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_opendir | 0.000220 | 0.012301 | 55.826 | 0.013341 | 0.013472 | 0.013576 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=110636 avg_ns=110636 max_ns=110636
  fuse_op.getattr: count=801 total_ns=8069306 avg_ns=10074 max_ns=32402
  fuse_op.lookup: count=1605 total_ns=14464818 avg_ns=9012 max_ns=85782
  fuse_op.opendir: count=800 total_ns=10616012 avg_ns=13270 max_ns=51638
  fuse_op.releasedir: count=800 total_ns=164950 avg_ns=206 max_ns=1149
  fuse_op.statfs: count=2 total_ns=3886 avg_ns=1943 max_ns=2815
  policy_decision: count=7214 total_ns=1873607 avg_ns=259 max_ns=17887
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=7214 total_ns=557576 avg_ns=77 max_ns=749
  matcher_candidate_order.path: count=21642 total_ns=2235436 avg_ns=103 max_ns=12853
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7214
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=7214
  matcher_candidate_order_ancestor_steps: count=67320
  matcher_candidate_order_ancestor_steps.descendant: count=16830
  matcher_candidate_order_ancestor_steps.path: count=50490
  state_read_lock_wait: count=3207 total_ns=59172 avg_ns=18 max_ns=131
  state_read_lock_hold: count=3207 total_ns=191489 avg_ns=59 max_ns=2709
  state_write_lock_wait: count=3203 total_ns=57816 avg_ns=18 max_ns=286
  state_write_lock_hold: count=3203 total_ns=458292 avg_ns=143 max_ns=65136
  open_confined_openat2: count=4008 total_ns=1725574 avg_ns=430 max_ns=16051
  open_like.pre_open_guard.access: count=1 total_ns=95994 avg_ns=95994 max_ns=95994
  open_like.pre_open_guard.opendir: count=800 total_ns=7817744 avg_ns=9772 max_ns=48342
  open_like.post_open_revalidation.access: count=1 total_ns=5710 avg_ns=5710 max_ns=5710
  open_like.post_open_revalidation.opendir: count=800 total_ns=1952671 avg_ns=2440 max_ns=28507
  stat_child_no_follow: count=3207 total_ns=17482880 avg_ns=5451 max_ns=80972
  source_root_path: count=3207 total_ns=4364875 avg_ns=1361 max_ns=52766
  resolved_virtual_path: count=10418 total_ns=13483499 avg_ns=1294 max_ns=74032
  resolved_virtual_path_from_path: count=6411 total_ns=10135035 avg_ns=1580 max_ns=9863
  resolved_virtual_path_from_path_component_walk: count=6411 total_ns=8225414 avg_ns=1283 max_ns=9566
  resolved_virtual_path_from_path_canonicalize: count=8014 total_ns=6407227 avg_ns=799 max_ns=9155
  resolved_virtual_path_from_path_source_root_confinement: count=8014 total_ns=1028294 avg_ns=128 max_ns=3634
  resolved_virtual_path_from_path_virtual_conversion: count=6411 total_ns=1592082 avg_ns=248 max_ns=3215
  resolved_virtual_path_from_open_fd: count=4007 total_ns=3348464 avg_ns=835 max_ns=74032
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_entries: count=0
  readdirplus_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
