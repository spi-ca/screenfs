# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:44.356894+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fast-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fast-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fast-open-confined-surface.svg --workload-set open-confined-surface`
- harness_repo_root: `/tmp/screenfs-bench-before-open-confined-168995`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `ca7372519260f85471e13bfc19189bf974d6fc0a973eb7e02270315c8d89cb97`
- screenfs_source_root: `/tmp/screenfs-bench-before-open-confined-168995`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
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
| metadata_open | 0.005205 | 0.043695 | 8.395 | 0.045364 | 0.046218 | 0.046902 |
| metadata_opendir | 0.000957 | 0.027686 | 28.937 | 0.034107 | 0.034210 | 0.034293 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=74575 avg_ns=74575 max_ns=74575
  fuse_op.getattr: count=6657 total_ns=54911068 avg_ns=8248 max_ns=111192
  fuse_op.lookup: count=33285 total_ns=374355407 avg_ns=11246 max_ns=12977252
  fuse_op.open: count=6656 total_ns=110458832 avg_ns=16595 max_ns=137537
  fuse_op.opendir: count=6656 total_ns=78828548 avg_ns=11843 max_ns=163863
  fuse_op.release: count=6656 total_ns=9275469 avg_ns=1393 max_ns=47991
  fuse_op.releasedir: count=6656 total_ns=2280163 avg_ns=342 max_ns=15511
  fuse_op.statfs: count=2 total_ns=18599 avg_ns=9299 max_ns=16457
  policy_decision: count=119822 total_ns=33784036 avg_ns=281 max_ns=84818
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=12813785 avg_ns=106 max_ns=48212
  matcher_candidate_order.path: count=359466 total_ns=38798552 avg_ns=107 max_ns=50317
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=1552710 avg_ns=29 max_ns=1389
  state_read_lock_hold: count=53255 total_ns=4107912 avg_ns=77 max_ns=163248
  state_write_lock_wait: count=59907 total_ns=1352382 avg_ns=22 max_ns=4956
  state_write_lock_hold: count=59907 total_ns=21835277 avg_ns=364 max_ns=73609
  open_confined_openat2: count=66568 total_ns=84434364 avg_ns=1268 max_ns=12909615
  open_like.pre_open_guard.access: count=1 total_ns=31848 avg_ns=31848 max_ns=31848
  open_like.pre_open_guard.open: count=6656 total_ns=78094055 avg_ns=11732 max_ns=131012
  open_like.pre_open_guard.opendir: count=6656 total_ns=53837890 avg_ns=8088 max_ns=85835
  open_like.post_open_revalidation.access: count=1 total_ns=35364 avg_ns=35364 max_ns=35364
  open_like.post_open_revalidation.open: count=6656 total_ns=21648302 avg_ns=3252 max_ns=88108
  open_like.post_open_revalidation.opendir: count=6656 total_ns=16859900 avg_ns=2533 max_ns=155296
  stat_child_no_follow: count=53255 total_ns=444046671 avg_ns=8338 max_ns=12969263
  stat_child_no_follow.attr_conversion: count=53253 total_ns=875783 avg_ns=16 max_ns=1523
  stat_child_no_follow.directory_revalidation: count=53254 total_ns=309730549 avg_ns=5816 max_ns=126518
  stat_child_no_follow.host_fstat: count=1 total_ns=2236 avg_ns=2236 max_ns=2236
  stat_child_no_follow.host_fstatat: count=53254 total_ns=22908773 avg_ns=430 max_ns=81989
  stat_child_no_follow.parent_open: count=53254 total_ns=86798905 avg_ns=1629 max_ns=12914041
  stat_child_no_follow_context.path_guard_or_metadata: count=53255 total_ns=444046671 avg_ns=8338 max_ns=12969263
  source_root_path: count=53255 total_ns=93642859 avg_ns=1758 max_ns=124229
  resolved_virtual_path: count=119821 total_ns=155390715 avg_ns=1296 max_ns=153542
  resolved_virtual_path_from_path: count=53254 total_ns=80816629 avg_ns=1517 max_ns=104375
  resolved_virtual_path_from_path_component_walk: count=53254 total_ns=64438846 avg_ns=1210 max_ns=104005
  resolved_virtual_path_from_path_canonicalize: count=53253 total_ns=50401063 avg_ns=946 max_ns=103771
  resolved_virtual_path_from_path_source_root_confinement: count=53253 total_ns=6604447 avg_ns=124 max_ns=11482
  resolved_virtual_path_from_path_virtual_conversion: count=53254 total_ns=13064193 avg_ns=245 max_ns=19558
  resolved_virtual_path_from_open_fd: count=66567 total_ns=74574086 avg_ns=1120 max_ns=153542
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
