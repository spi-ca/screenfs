# ScreenFS benchmark result

- timestamp: `2026-06-20T07:23:19.142337+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload pinned_symlink_parent_mkdir_rmdir --iterations 1 --warmups 1 --symlink-parent-mutations 20 --output-json docs/artifacts/current-pinned-mutation-invalidation-smoke.json --output-md docs/artifacts/current-pinned-mutation-invalidation-smoke.md --output-svg docs/artifacts/current-pinned-mutation-invalidation-smoke.svg`
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
- screenfs_only_workloads: `pinned_symlink_parent_mkdir_rmdir`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| pinned_symlink_parent_mkdir_rmdir | 0.023637 | 0.023637 | 0.023637 | 0.023637 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=60662 avg_ns=60662 max_ns=60662
  fuse_op.getattr: count=127 total_ns=2113111 avg_ns=16638 max_ns=33206
  fuse_op.lookup: count=1511 total_ns=20514937 avg_ns=13577 max_ns=92944
  fuse_op.mkdir: count=40 total_ns=3761489 avg_ns=94037 max_ns=109633
  fuse_op.opendir: count=42 total_ns=789381 avg_ns=18794 max_ns=24359
  fuse_op.readdirplus: count=42 total_ns=1734786 avg_ns=41304 max_ns=62517
  fuse_op.readlink: count=246 total_ns=4511137 avg_ns=18337 max_ns=51239
  fuse_op.releasedir: count=42 total_ns=32241 avg_ns=767 max_ns=3595
  fuse_op.rmdir: count=40 total_ns=3038262 avg_ns=75956 max_ns=125283
  fuse_op.statfs: count=2 total_ns=3829 avg_ns=1914 max_ns=3222
  policy_decision: count=6126 total_ns=2840680 avg_ns=463 max_ns=21417
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=5886 total_ns=782873 avg_ns=133 max_ns=372
  matcher_candidate_order.path: count=18138 total_ns=3128600 avg_ns=172 max_ns=36308
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=6126
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=6126
  matcher_candidate_order_ancestor_steps: count=84016
  matcher_candidate_order_ancestor_steps.descendant: count=20484
  matcher_candidate_order_ancestor_steps.path: count=63532
  state_read_lock_wait: count=2049 total_ns=38802 avg_ns=18 max_ns=194
  state_read_lock_hold: count=2049 total_ns=170684 avg_ns=83 max_ns=1777
  state_write_lock_wait: count=1637 total_ns=30613 avg_ns=18 max_ns=125
  state_write_lock_hold: count=1637 total_ns=615027 avg_ns=375 max_ns=6781
  open_confined_openat2: count=2536 total_ns=1269249 avg_ns=500 max_ns=10166
  stat_child_no_follow: count=2371 total_ns=15989585 avg_ns=6743 max_ns=50784
  source_root_path: count=2615 total_ns=3517827 avg_ns=1345 max_ns=48142
  resolved_virtual_path: count=7034 total_ns=15522660 avg_ns=2206 max_ns=85354
  resolved_virtual_path_from_path: count=4499 total_ns=13200266 avg_ns=2934 max_ns=85354
  resolved_virtual_path_from_path_component_walk: count=4499 total_ns=11471980 avg_ns=2549 max_ns=84971
  resolved_virtual_path_from_path_canonicalize: count=9962 total_ns=9188865 avg_ns=922 max_ns=84420
  resolved_virtual_path_from_path_source_root_confinement: count=9962 total_ns=1336482 avg_ns=134 max_ns=17714
  resolved_virtual_path_from_path_virtual_conversion: count=4499 total_ns=1497483 avg_ns=332 max_ns=8803
  resolved_virtual_path_from_open_fd: count=2535 total_ns=2322394 avg_ns=916 max_ns=4029
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
  readdirplus_directory_scan: count=42 total_ns=405302 avg_ns=9650 max_ns=17300
  readdirplus_attr_generation_scan: count=42 total_ns=66289 avg_ns=1578 max_ns=6974
  readdirplus_attr_generation_entries: count=82
  readdirplus_symlink_visibility: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=42 total_ns=28506 avg_ns=678 max_ns=6281
  readdirplus_page_commit: count=42 total_ns=63971 avg_ns=1523 max_ns=2473
  invalidations: count=80 invalidated_entries=40 evicted_entries=0 scanned_entries=360
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
