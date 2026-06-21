# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:37.124848+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_opendir.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_opendir.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_opendir.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `77d2fc749459704dab16352ddbe5567c35d8ce313519dbc39fdf43a471081280`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
- screenfs_only_workloads: `(none)`
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
| metadata_opendir | 0.000959 | 0.042805 | 44.639 | 0.044315 | 0.044433 | 0.044528 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=85069 avg_ns=85069 max_ns=85069
  fuse_op.getattr: count=6657 total_ns=81989937 avg_ns=12316 max_ns=42370
  fuse_op.lookup: count=13317 total_ns=154389105 avg_ns=11593 max_ns=95130
  fuse_op.opendir: count=6656 total_ns=111807272 avg_ns=16797 max_ns=84685
  fuse_op.releasedir: count=6656 total_ns=2376831 avg_ns=357 max_ns=6536
  fuse_op.statfs: count=2 total_ns=5829 avg_ns=2914 max_ns=4134
  policy_decision: count=59918 total_ns=19619117 avg_ns=327 max_ns=6695
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=5829693 avg_ns=97 max_ns=4766
  matcher_candidate_order.path: count=179754 total_ns=23541891 avg_ns=130 max_ns=7696
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=559224
  matcher_candidate_order_ancestor_steps.descendant: count=139806
  matcher_candidate_order_ancestor_steps.path: count=419418
  state_read_lock_wait: count=26631 total_ns=675974 avg_ns=25 max_ns=396
  state_read_lock_hold: count=26631 total_ns=1918022 avg_ns=72 max_ns=14680
  state_write_lock_wait: count=26627 total_ns=641205 avg_ns=24 max_ns=426
  state_write_lock_hold: count=26627 total_ns=5116465 avg_ns=192 max_ns=60162
  open_confined_openat2: count=33288 total_ns=22451168 avg_ns=674 max_ns=17361
  open_like.pre_open_guard.access: count=1 total_ns=73895 avg_ns=73895 max_ns=73895
  open_like.pre_open_guard.opendir: count=6656 total_ns=83032366 avg_ns=12474 max_ns=55833
  open_like.post_open_revalidation.access: count=1 total_ns=4515 avg_ns=4515 max_ns=4515
  open_like.post_open_revalidation.opendir: count=6656 total_ns=19656600 avg_ns=2953 max_ns=23152
  stat_child_no_follow: count=26631 total_ns=193837366 avg_ns=7278 max_ns=77284
  source_root_path: count=26631 total_ns=52565842 avg_ns=1973 max_ns=40674
  resolved_virtual_path: count=86546 total_ns=136913074 avg_ns=1581 max_ns=24224
  resolved_virtual_path_from_path: count=53259 total_ns=101349227 avg_ns=1902 max_ns=16561
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=82955929 avg_ns=1557 max_ns=15710
  resolved_virtual_path_from_path_canonicalize: count=66574 total_ns=64795580 avg_ns=973 max_ns=15062
  resolved_virtual_path_from_path_source_root_confinement: count=66574 total_ns=9153939 avg_ns=137 max_ns=9328
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=15157455 avg_ns=284 max_ns=6759
  resolved_virtual_path_from_open_fd: count=33287 total_ns=35563847 avg_ns=1068 max_ns=24224
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
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
