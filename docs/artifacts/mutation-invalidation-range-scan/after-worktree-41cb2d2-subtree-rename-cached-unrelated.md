# ScreenFS benchmark result

- timestamp: `2026-06-20T07:10:44.153166+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload subtree_rename_cached_unrelated --iterations 5 --warmups 1 --small-files 500 --output-json docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-subtree-rename-cached-unrelated.json --output-md docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-subtree-rename-cached-unrelated.md --output-svg docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-subtree-rename-cached-unrelated.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `42d11602a3f11535901c7b881c7ddb7ff8bab477697f48a19924c3d059d86505`
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
- screenfs_only_workloads: `subtree_rename_cached_unrelated`
- iterations: `5`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| subtree_rename_cached_unrelated | 0.060480 | 0.063563 | 0.064573 | 0.065381 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=55206 avg_ns=55206 max_ns=55206
  fuse_op.getattr: count=3037 total_ns=64937485 avg_ns=21382 max_ns=81200
  fuse_op.lookup: count=12250 total_ns=197505111 avg_ns=16122 max_ns=161930
  fuse_op.rename: count=12 total_ns=1857075 avg_ns=154756 max_ns=205253
  fuse_op.statfs: count=2 total_ns=3965 avg_ns=1982 max_ns=2196
  policy_decision: count=30912 total_ns=15781273 avg_ns=510 max_ns=58655
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=30840 total_ns=4585842 avg_ns=148 max_ns=27728
  matcher_candidate_order.path: count=92664 total_ns=18107193 avg_ns=195 max_ns=35843
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=30912
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=30912
  matcher_candidate_order_ancestor_steps: count=406388
  matcher_candidate_order_ancestor_steps.descendant: count=101477
  matcher_candidate_order_ancestor_steps.path: count=304911
  state_read_lock_wait: count=15312 total_ns=364423 avg_ns=23 max_ns=349
  state_read_lock_hold: count=15312 total_ns=1206394 avg_ns=78 max_ns=1235
  state_write_lock_wait: count=12236 total_ns=309500 avg_ns=25 max_ns=16989
  state_write_lock_hold: count=12236 total_ns=7119838 avg_ns=581 max_ns=47820
  open_confined_openat2: count=15409 total_ns=11018924 avg_ns=715 max_ns=113768
  stat_child_no_follow: count=15384 total_ns=146177440 avg_ns=9501 max_ns=130662
  source_root_path: count=15420 total_ns=29723052 avg_ns=1927 max_ns=33694
  resolved_virtual_path: count=46125 total_ns=128515063 avg_ns=2786 max_ns=60044
  resolved_virtual_path_from_path: count=30717 total_ns=110833965 avg_ns=3608 max_ns=60044
  resolved_virtual_path_from_path_component_walk: count=30717 total_ns=98317512 avg_ns=3200 max_ns=59713
  resolved_virtual_path_from_path_canonicalize: count=70299 total_ns=80211070 avg_ns=1140 max_ns=59413
  resolved_virtual_path_from_path_source_root_confinement: count=70299 total_ns=9950706 avg_ns=141 max_ns=21697
  resolved_virtual_path_from_path_virtual_conversion: count=30717 total_ns=10623439 avg_ns=345 max_ns=26468
  resolved_virtual_path_from_open_fd: count=15408 total_ns=17681098 avg_ns=1147 max_ns=55327
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
  invalidations: count=12 invalidated_entries=24 evicted_entries=0 scanned_entries=48
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
