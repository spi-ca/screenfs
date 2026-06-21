# ScreenFS benchmark result

- timestamp: `2026-06-21T02:31:49.810638+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --symlink-parent-mutations 20 --small-files 200 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-mutation-invalidation.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-mutation-invalidation.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-mutation-invalidation.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/state.rs;  M src/fs/tests/perf.rs; ... (+6 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `a31930e4cb63e4d8821e98a1383ddfd1b67dfee11d1a45dddd871aab69ae1c0d`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/state.rs;  M src/fs/tests/perf.rs; ... (+6 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir, subtree_rename_cached_unrelated`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.018786 | 0.021404 | 0.022058 | 0.022582 |
| pinned_symlink_parent_mkdir_rmdir | 0.029608 | 0.030816 | 0.031702 | 0.032411 |
| subtree_rename_cached_unrelated | 0.024639 | 0.026062 | 0.026102 | 0.026133 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=50292 avg_ns=50292 max_ns=50292
  fuse_op.getattr: count=3771 total_ns=79655782 avg_ns=21123 max_ns=50807
  fuse_op.lookup: count=27798 total_ns=451135585 avg_ns=16229 max_ns=77452
  fuse_op.mkdir: count=520 total_ns=57982857 avg_ns=111505 max_ns=188617
  fuse_op.opendir: count=273 total_ns=5963258 avg_ns=21843 max_ns=35947
  fuse_op.readdirplus: count=273 total_ns=20636153 avg_ns=75590 max_ns=120826
  fuse_op.readlink: count=2392 total_ns=52658504 avg_ns=22014 max_ns=89605
  fuse_op.releasedir: count=273 total_ns=202549 avg_ns=741 max_ns=2283
  fuse_op.rename: count=26 total_ns=3781067 avg_ns=145425 max_ns=170093
  fuse_op.rmdir: count=520 total_ns=44250031 avg_ns=85096 max_ns=130296
  fuse_op.statfs: count=2 total_ns=4876 avg_ns=2438 max_ns=2891
  policy_decision: count=95211 total_ns=50371314 avg_ns=529 max_ns=19330
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91935 total_ns=13947239 avg_ns=151 max_ns=11244
  matcher_candidate_order.path: count=282357 total_ns=55641795 avg_ns=197 max_ns=30688
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=95211
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=95211
  matcher_candidate_order_ancestor_steps: count=1291408
  matcher_candidate_order_ancestor_steps.descendant: count=315832
  matcher_candidate_order_ancestor_steps.path: count=975576
  state_read_lock_wait: count=35600 total_ns=859487 avg_ns=24 max_ns=530
  state_read_lock_hold: count=35600 total_ns=2761933 avg_ns=77 max_ns=3461
  state_write_lock_wait: count=28342 total_ns=690752 avg_ns=24 max_ns=639
  state_write_lock_hold: count=28342 total_ns=12050311 avg_ns=425 max_ns=30942
  open_confined_openat2: count=41841 total_ns=28800659 avg_ns=688 max_ns=16207
  open_like.pre_open_guard.access: count=1 total_ns=42199 avg_ns=42199 max_ns=42199
  open_like.pre_open_guard.opendir: count=273 total_ns=4565575 avg_ns=16723 max_ns=28821
  open_like.post_open_revalidation.access: count=1 total_ns=4133 avg_ns=4133 max_ns=4133
  open_like.post_open_revalidation.opendir: count=273 total_ns=959595 avg_ns=3515 max_ns=5094
  stat_child_no_follow: count=40202 total_ns=359965973 avg_ns=8953 max_ns=65883
  source_root_path: count=40449 total_ns=78384979 avg_ns=1937 max_ns=34224
  resolved_virtual_path: count=118484 total_ns=321471722 avg_ns=2713 max_ns=66135
  resolved_virtual_path_from_path: count=76644 total_ns=272692682 avg_ns=3557 max_ns=66135
  resolved_virtual_path_from_path_component_walk: count=76644 total_ns=241718304 avg_ns=3153 max_ns=56027
  resolved_virtual_path_from_path_canonicalize: count=171817 total_ns=199125614 avg_ns=1158 max_ns=22584
  resolved_virtual_path_from_path_source_root_confinement: count=171817 total_ns=23149405 avg_ns=134 max_ns=7235
  resolved_virtual_path_from_path_virtual_conversion: count=76644 total_ns=26253043 avg_ns=342 max_ns=10039
  resolved_virtual_path_from_open_fd: count=41840 total_ns=48779040 avg_ns=1165 max_ns=45707
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
  readdirplus_directory_scan: count=273 total_ns=2578652 avg_ns=9445 max_ns=43397
  readdirplus_attr_generation_scan: count=806 total_ns=6297532 avg_ns=7813 max_ns=16330
  readdirplus_attr_generation_entries: count=533
  readdirplus_symlink_visibility: count=273 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=273 total_ns=169171 avg_ns=619 max_ns=23736
  readdirplus_page_commit: count=273 total_ns=449269 avg_ns=1645 max_ns=4619
  invalidations: count=1066 invalidated_entries=572 evicted_entries=0 scanned_entries=884
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
