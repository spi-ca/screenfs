# ScreenFS benchmark result

- timestamp: `2026-06-20T07:48:58.518613+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload metadata_opendir --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-metadata-opendir-smoke.json --output-md docs/artifacts/current-metadata-opendir-smoke.md --output-svg docs/artifacts/current-metadata-opendir-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+21 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+21 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
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
| metadata_opendir | 0.000248 | 0.011116 | 44.751 | 0.011282 | 0.011303 | 0.011319 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=76366 avg_ns=76366 max_ns=76366
  fuse_op.getattr: count=801 total_ns=7295287 avg_ns=9107 max_ns=37470
  fuse_op.lookup: count=1605 total_ns=13257507 avg_ns=8260 max_ns=66902
  fuse_op.opendir: count=800 total_ns=10616895 avg_ns=13271 max_ns=35142
  fuse_op.releasedir: count=800 total_ns=190444 avg_ns=238 max_ns=1753
  fuse_op.statfs: count=2 total_ns=2562 avg_ns=1281 max_ns=1673
  policy_decision: count=7214 total_ns=1757737 avg_ns=243 max_ns=1042
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=7214 total_ns=536211 avg_ns=74 max_ns=4905
  matcher_candidate_order.path: count=21642 total_ns=2060406 avg_ns=95 max_ns=1009
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7214
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=7214
  matcher_candidate_order_ancestor_steps: count=67320
  matcher_candidate_order_ancestor_steps.descendant: count=16830
  matcher_candidate_order_ancestor_steps.path: count=50490
  state_read_lock_wait: count=3207 total_ns=55204 avg_ns=17 max_ns=286
  state_read_lock_hold: count=3207 total_ns=188236 avg_ns=58 max_ns=1277
  state_write_lock_wait: count=3203 total_ns=55152 avg_ns=17 max_ns=145
  state_write_lock_hold: count=3203 total_ns=388948 avg_ns=121 max_ns=23058
  open_confined_openat2: count=4008 total_ns=1605973 avg_ns=400 max_ns=10170
  stat_child_no_follow: count=3207 total_ns=15932049 avg_ns=4967 max_ns=46885
  source_root_path: count=4007 total_ns=4748150 avg_ns=1184 max_ns=27069
  resolved_virtual_path: count=10418 total_ns=12275324 avg_ns=1178 max_ns=7929
  resolved_virtual_path_from_path: count=6411 total_ns=9254469 avg_ns=1443 max_ns=4822
  resolved_virtual_path_from_path_component_walk: count=6411 total_ns=7476114 avg_ns=1166 max_ns=4399
  resolved_virtual_path_from_path_canonicalize: count=8014 total_ns=5769023 avg_ns=719 max_ns=3961
  resolved_virtual_path_from_path_source_root_confinement: count=8014 total_ns=967467 avg_ns=120 max_ns=794
  resolved_virtual_path_from_path_virtual_conversion: count=6411 total_ns=1471601 avg_ns=229 max_ns=1930
  resolved_virtual_path_from_open_fd: count=4007 total_ns=3020855 avg_ns=753 max_ns=7929
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
