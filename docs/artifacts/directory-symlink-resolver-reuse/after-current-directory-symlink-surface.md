# ScreenFS benchmark result

- timestamp: `2026-06-20T07:38:18.385961+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-symlink-surface --iterations 10 --warmups 3 --dir-entries 200 --output-json docs/artifacts/directory-symlink-resolver-reuse/after-current-directory-symlink-surface.json --output-md docs/artifacts/directory-symlink-resolver-reuse/after-current-directory-symlink-surface.md --output-svg docs/artifacts/directory-symlink-resolver-reuse/after-current-directory-symlink-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `459757d51c2497af9800fd465a5579433f31f4334d19dce3152a804e8a63dde4`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+20 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-symlink-surface`
- comparable_workloads: `readdir_symlink_visibility, readdirplus_symlink_visibility`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_symlink_visibility | 0.000123 | 0.003009 | 24.391 | 0.003320 | 0.003333 | 0.003344 |
| readdirplus_symlink_visibility | 0.000362 | 0.021127 | 58.362 | 0.021482 | 0.021546 | 0.021596 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=59947 avg_ns=59947 max_ns=59947
  fuse_op.getattr: count=2653 total_ns=50922233 avg_ns=19194 max_ns=92650
  fuse_op.lookup: count=7935 total_ns=116604078 avg_ns=14694 max_ns=158145
  fuse_op.opendir: count=26 total_ns=546655 avg_ns=21025 max_ns=43581
  fuse_op.readdir: count=51 total_ns=5717512 avg_ns=112108 max_ns=261266
  fuse_op.readdirplus: count=27 total_ns=61205293 avg_ns=2266862 max_ns=2789343
  fuse_op.releasedir: count=26 total_ns=569692 avg_ns=21911 max_ns=47947
  fuse_op.statfs: count=2 total_ns=4777 avg_ns=2388 max_ns=2622
  policy_decision: count=37584 total_ns=17906000 avg_ns=476 max_ns=33675
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=37584 total_ns=5362066 avg_ns=142 max_ns=4414
  matcher_candidate_order.path: count=112752 total_ns=20395995 avg_ns=180 max_ns=19459
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=37584
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=37584
  matcher_candidate_order_ancestor_steps: count=493496
  matcher_candidate_order_ancestor_steps.descendant: count=123374
  matcher_candidate_order_ancestor_steps.path: count=370122
  state_read_lock_wait: count=10693 total_ns=268992 avg_ns=25 max_ns=387
  state_read_lock_hold: count=10693 total_ns=820692 avg_ns=76 max_ns=3689
  state_write_lock_wait: count=8089 total_ns=193295 avg_ns=23 max_ns=584
  state_write_lock_hold: count=8089 total_ns=7977733 avg_ns=986 max_ns=332927
  open_confined_openat2: count=10772 total_ns=7131603 avg_ns=662 max_ns=30864
  stat_child_no_follow: count=10667 total_ns=89793507 avg_ns=8417 max_ns=142591
  source_root_path: count=10823 total_ns=22391580 avg_ns=2068 max_ns=34244
  resolved_virtual_path: count=37536 total_ns=100154421 avg_ns=2668 max_ns=77742
  resolved_virtual_path_from_path: count=26765 total_ns=87743030 avg_ns=3278 max_ns=77742
  resolved_virtual_path_from_path_component_walk: count=26765 total_ns=77398933 avg_ns=2891 max_ns=77056
  resolved_virtual_path_from_path_canonicalize: count=53444 total_ns=63479248 avg_ns=1187 max_ns=76122
  resolved_virtual_path_from_path_source_root_confinement: count=53444 total_ns=7222882 avg_ns=135 max_ns=9334
  resolved_virtual_path_from_path_virtual_conversion: count=26765 total_ns=8734877 avg_ns=326 max_ns=11450
  resolved_virtual_path_from_open_fd: count=10771 total_ns=12411391 avg_ns=1152 max_ns=10512
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=51 total_ns=4492943 avg_ns=88096 max_ns=217573
  readdir_attr_generation_scan: count=51 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=51 total_ns=1784507 avg_ns=34990 max_ns=120864
  readdir_candidate_selection: count=51 total_ns=47389 avg_ns=929 max_ns=3700
  readdir_page_commit: count=51 total_ns=332682 avg_ns=6523 max_ns=21554
  readdirplus_directory_scan: count=27 total_ns=55751145 avg_ns=2064857 max_ns=2369259
  readdirplus_attr_generation_scan: count=27 total_ns=5011062 avg_ns=185594 max_ns=254746
  readdirplus_attr_generation_entries: count=5252
  readdirplus_symlink_visibility: count=27 total_ns=38407344 avg_ns=1422494 max_ns=1651007
  readdirplus_candidate_selection: count=27 total_ns=975957 avg_ns=36146 max_ns=89046
  readdirplus_page_commit: count=27 total_ns=3633728 avg_ns=134582 max_ns=333327
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
