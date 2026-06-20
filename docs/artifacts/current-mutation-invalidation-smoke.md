# ScreenFS benchmark result

- timestamp: `2026-06-20T07:23:18.405636+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload symlink_parent_mkdir_rmdir --iterations 1 --warmups 1 --symlink-parent-mutations 20 --output-json docs/artifacts/current-mutation-invalidation-smoke.json --output-md docs/artifacts/current-mutation-invalidation-smoke.md --output-svg docs/artifacts/current-mutation-invalidation-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.015455 | 0.015455 | 0.015455 | 0.015455 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=28624 avg_ns=28624 max_ns=28624
  fuse_op.getattr: count=43 total_ns=771343 avg_ns=17938 max_ns=29066
  fuse_op.lookup: count=1093 total_ns=14886442 avg_ns=13619 max_ns=322828
  fuse_op.mkdir: count=40 total_ns=3817692 avg_ns=95442 max_ns=164309
  fuse_op.readlink: count=122 total_ns=2329097 avg_ns=19090 max_ns=85481
  fuse_op.rmdir: count=40 total_ns=3018542 avg_ns=75463 max_ns=123888
  fuse_op.statfs: count=2 total_ns=3547 avg_ns=1773 max_ns=2834
  policy_decision: count=4084 total_ns=1896184 avg_ns=464 max_ns=3221
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3844 total_ns=522383 avg_ns=135 max_ns=6068
  matcher_candidate_order.path: count=12012 total_ns=2082280 avg_ns=173 max_ns=1333
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=4084
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=4084
  matcher_candidate_order_ancestor_steps: count=55704
  matcher_candidate_order_ancestor_steps.descendant: count=13406
  matcher_candidate_order_ancestor_steps.path: count=42298
  state_read_lock_wait: count=1339 total_ns=25093 avg_ns=18 max_ns=128
  state_read_lock_hold: count=1339 total_ns=106816 avg_ns=79 max_ns=661
  state_write_lock_wait: count=1051 total_ns=19176 avg_ns=18 max_ns=116
  state_write_lock_hold: count=1051 total_ns=370616 avg_ns=352 max_ns=3474
  open_confined_openat2: count=1700 total_ns=820371 avg_ns=482 max_ns=5910
  stat_child_no_follow: count=1619 total_ns=11467949 avg_ns=7083 max_ns=66602
  source_root_path: count=1779 total_ns=2313449 avg_ns=1300 max_ns=20954
  resolved_virtual_path: count=4694 total_ns=10387749 avg_ns=2212 max_ns=71360
  resolved_virtual_path_from_path: count=2995 total_ns=8832115 avg_ns=2948 max_ns=71360
  resolved_virtual_path_from_path_component_walk: count=2995 total_ns=7614270 avg_ns=2542 max_ns=45160
  resolved_virtual_path_from_path_canonicalize: count=6622 total_ns=6056587 avg_ns=914 max_ns=7392
  resolved_virtual_path_from_path_source_root_confinement: count=6622 total_ns=921196 avg_ns=139 max_ns=42039
  resolved_virtual_path_from_path_virtual_conversion: count=2995 total_ns=1062141 avg_ns=354 max_ns=67800
  resolved_virtual_path_from_open_fd: count=1699 total_ns=1555634 avg_ns=915 max_ns=7214
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
  invalidations: count=80 invalidated_entries=40 evicted_entries=0 scanned_entries=320
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
