# ScreenFS benchmark result

- timestamp: `2026-06-20T07:43:41.759716+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set open-confined-surface --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-open-confined-surface-smoke.json --output-md docs/artifacts/current-open-confined-surface-smoke.md --output-svg docs/artifacts/current-open-confined-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+18 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
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
| metadata_open | 0.001127 | 0.021646 | 19.204 | 0.022433 | 0.022532 | 0.022611 |
| metadata_opendir | 0.000347 | 0.017759 | 51.247 | 0.017863 | 0.017876 | 0.017887 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=39328 avg_ns=39328 max_ns=39328
  fuse_op.getattr: count=801 total_ns=10229610 avg_ns=12771 max_ns=49728
  fuse_op.lookup: count=4005 total_ns=73506669 avg_ns=18353 max_ns=11961728
  fuse_op.open: count=800 total_ns=18578405 avg_ns=23223 max_ns=68658
  fuse_op.opendir: count=800 total_ns=14630289 avg_ns=18287 max_ns=78363
  fuse_op.release: count=800 total_ns=917098 avg_ns=1146 max_ns=9610
  fuse_op.releasedir: count=800 total_ns=329578 avg_ns=411 max_ns=2385
  fuse_op.statfs: count=2 total_ns=4839 avg_ns=2419 max_ns=2431
  policy_decision: count=14414 total_ns=5396264 avg_ns=374 max_ns=12120
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=14414 total_ns=1598769 avg_ns=110 max_ns=1485
  matcher_candidate_order.path: count=43242 total_ns=6350126 avg_ns=146 max_ns=1222
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=14414
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=14414
  matcher_candidate_order_ancestor_steps: count=150520
  matcher_candidate_order_ancestor_steps.descendant: count=37630
  matcher_candidate_order_ancestor_steps.path: count=112890
  state_read_lock_wait: count=6407 total_ns=157112 avg_ns=24 max_ns=371
  state_read_lock_hold: count=6407 total_ns=495688 avg_ns=77 max_ns=1072
  state_write_lock_wait: count=7203 total_ns=174200 avg_ns=24 max_ns=558
  state_write_lock_hold: count=7203 total_ns=2567505 avg_ns=356 max_ns=57925
  open_confined_openat2: count=8008 total_ns=24135625 avg_ns=3013 max_ns=11924626
  stat_child_no_follow: count=6407 total_ns=71140914 avg_ns=11103 max_ns=11954184
  source_root_path: count=8007 total_ns=16045172 avg_ns=2003 max_ns=58692
  resolved_virtual_path: count=20818 total_ns=39977424 avg_ns=1920 max_ns=90217
  resolved_virtual_path_from_path: count=12811 total_ns=29856923 avg_ns=2330 max_ns=90217
  resolved_virtual_path_from_path_component_walk: count=12811 total_ns=25202738 avg_ns=1967 max_ns=89352
  resolved_virtual_path_from_path_canonicalize: count=19214 total_ns=20003474 avg_ns=1041 max_ns=14715
  resolved_virtual_path_from_path_source_root_confinement: count=19214 total_ns=2630278 avg_ns=136 max_ns=1063
  resolved_virtual_path_from_path_virtual_conversion: count=12811 total_ns=3873526 avg_ns=302 max_ns=12218
  resolved_virtual_path_from_open_fd: count=8007 total_ns=10120501 avg_ns=1263 max_ns=39251
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
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
