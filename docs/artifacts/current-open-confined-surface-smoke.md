# ScreenFS benchmark result

- timestamp: `2026-06-20T16:53:33.408545+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --workload-set open-confined-surface --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-open-confined-surface-smoke.json --output-md docs/artifacts/current-open-confined-surface-smoke.md --output-svg docs/artifacts/current-open-confined-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+12 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+12 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_open | 0.000868 | 0.015074 | 17.359 | 0.016181 | 0.016319 | 0.016430 |
| metadata_opendir | 0.000377 | 0.011120 | 29.460 | 0.012400 | 0.012561 | 0.012689 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=36130 avg_ns=36130 max_ns=36130
  fuse_op.getattr: count=801 total_ns=7449397 avg_ns=9300 max_ns=29320
  fuse_op.lookup: count=4005 total_ns=52793580 avg_ns=13181 max_ns=7394118
  fuse_op.open: count=800 total_ns=13708575 avg_ns=17135 max_ns=81814
  fuse_op.opendir: count=800 total_ns=9789048 avg_ns=12236 max_ns=104828
  fuse_op.release: count=800 total_ns=397872 avg_ns=497 max_ns=5369
  fuse_op.releasedir: count=800 total_ns=190317 avg_ns=237 max_ns=1821
  fuse_op.statfs: count=2 total_ns=4630 avg_ns=2315 max_ns=3276
  policy_decision: count=14414 total_ns=4148806 avg_ns=287 max_ns=25841
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=14414 total_ns=1251689 avg_ns=86 max_ns=14845
  matcher_candidate_order.path: count=43242 total_ns=4864964 avg_ns=112 max_ns=4068
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=14414
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=14414
  matcher_candidate_order_ancestor_steps: count=150520
  matcher_candidate_order_ancestor_steps.descendant: count=37630
  matcher_candidate_order_ancestor_steps.path: count=112890
  state_read_lock_wait: count=6407 total_ns=113315 avg_ns=17 max_ns=149
  state_read_lock_hold: count=6407 total_ns=380941 avg_ns=59 max_ns=693
  state_write_lock_wait: count=7203 total_ns=123461 avg_ns=17 max_ns=166
  state_write_lock_hold: count=7203 total_ns=1670644 avg_ns=231 max_ns=27838
  open_confined_openat2: count=8008 total_ns=16427361 avg_ns=2051 max_ns=7371742
  open_like.pre_open_guard.access: count=1 total_ns=27259 avg_ns=27259 max_ns=27259
  open_like.pre_open_guard.open: count=800 total_ns=10621590 avg_ns=13276 max_ns=70718
  open_like.pre_open_guard.opendir: count=800 total_ns=7234014 avg_ns=9042 max_ns=100315
  open_like.post_open_revalidation.access: count=1 total_ns=5423 avg_ns=5423 max_ns=5423
  open_like.post_open_revalidation.open: count=800 total_ns=2228309 avg_ns=2785 max_ns=10240
  open_like.post_open_revalidation.opendir: count=800 total_ns=1784020 avg_ns=2230 max_ns=3864
  stat_child_no_follow: count=6407 total_ns=49443423 avg_ns=7717 max_ns=7388151
  source_root_path: count=6407 total_ns=8136058 avg_ns=1269 max_ns=21964
  resolved_virtual_path: count=20818 total_ns=30345770 avg_ns=1457 max_ns=19775
  resolved_virtual_path_from_path: count=12811 total_ns=23563239 avg_ns=1839 max_ns=19775
  resolved_virtual_path_from_path_component_walk: count=12811 total_ns=19788638 avg_ns=1544 max_ns=19231
  resolved_virtual_path_from_path_canonicalize: count=19214 total_ns=15648097 avg_ns=814 max_ns=18679
  resolved_virtual_path_from_path_source_root_confinement: count=19214 total_ns=2378760 avg_ns=123 max_ns=1441
  resolved_virtual_path_from_path_virtual_conversion: count=12811 total_ns=3177946 avg_ns=248 max_ns=1793
  resolved_virtual_path_from_open_fd: count=8007 total_ns=6782531 avg_ns=847 max_ns=16001
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
