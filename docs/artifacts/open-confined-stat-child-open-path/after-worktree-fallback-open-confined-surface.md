# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:42.045611+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-open-confined-surface.svg --workload-set open-confined-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
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
| metadata_open | 0.005332 | 0.029540 | 5.540 | 0.030333 | 0.030578 | 0.030774 |
| metadata_opendir | 0.001648 | 0.024635 | 14.951 | 0.025188 | 0.025353 | 0.025485 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=92685 avg_ns=92685 max_ns=92685
  fuse_op.getattr: count=6657 total_ns=42901408 avg_ns=6444 max_ns=87216
  fuse_op.lookup: count=33285 total_ns=292351697 avg_ns=8783 max_ns=25176989
  fuse_op.open: count=6656 total_ns=83281984 avg_ns=12512 max_ns=98485
  fuse_op.opendir: count=6656 total_ns=64934867 avg_ns=9755 max_ns=151842
  fuse_op.release: count=6656 total_ns=4110703 avg_ns=617 max_ns=33267
  fuse_op.releasedir: count=6656 total_ns=1515928 avg_ns=227 max_ns=3943
  fuse_op.statfs: count=2 total_ns=2930 avg_ns=1465 max_ns=1593
  policy_decision: count=66568 total_ns=24972494 avg_ns=375 max_ns=17588
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=66568 total_ns=7442482 avg_ns=111 max_ns=14999
  matcher_candidate_order.path: count=199704 total_ns=29782478 avg_ns=149 max_ns=14349
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=66568
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=66568
  matcher_candidate_order_ancestor_steps: count=825420
  matcher_candidate_order_ancestor_steps.descendant: count=206355
  matcher_candidate_order_ancestor_steps.path: count=619065
  state_read_lock_wait: count=53255 total_ns=985848 avg_ns=18 max_ns=1063
  state_read_lock_hold: count=53255 total_ns=3199944 avg_ns=60 max_ns=3781
  state_write_lock_wait: count=59907 total_ns=1083245 avg_ns=18 max_ns=1000
  state_write_lock_hold: count=59907 total_ns=14398810 avg_ns=240 max_ns=36187
  open_confined_openat2: count=66568 total_ns=87018046 avg_ns=1307 max_ns=25126222
  open_like.pre_open_guard.access: count=1 total_ns=70202 avg_ns=70202 max_ns=70202
  open_like.pre_open_guard.open: count=6656 total_ns=54499790 avg_ns=8188 max_ns=72133
  open_like.pre_open_guard.opendir: count=6656 total_ns=42205381 avg_ns=6340 max_ns=147512
  open_like.post_open_revalidation.access: count=1 total_ns=13365 avg_ns=13365 max_ns=13365
  open_like.post_open_revalidation.open: count=6656 total_ns=21942319 avg_ns=3296 max_ns=23296
  open_like.post_open_revalidation.opendir: count=6656 total_ns=15966286 avg_ns=2398 max_ns=16694
  stat_child_no_follow: count=53255 total_ns=105089579 avg_ns=1973 max_ns=25132495
  stat_child_no_follow.attr_conversion: count=53253 total_ns=732488 avg_ns=13 max_ns=230
  stat_child_no_follow.host_fstat: count=53253 total_ns=7703842 avg_ns=144 max_ns=5052
  stat_child_no_follow_context.path_guard_or_metadata: count=53255 total_ns=105089579 avg_ns=1973 max_ns=25132495
  source_root_path: count=53255 total_ns=70418137 avg_ns=1322 max_ns=65922
  resolved_virtual_path: count=66566 total_ns=150093311 avg_ns=2254 max_ns=321315
  resolved_virtual_path_from_path: count=53253 total_ns=134645531 avg_ns=2528 max_ns=321315
  resolved_virtual_path_from_path_component_walk: count=53253 total_ns=114809554 avg_ns=2155 max_ns=320930
  resolved_virtual_path_from_path_canonicalize: count=106505 total_ns=89948580 avg_ns=844 max_ns=320443
  resolved_virtual_path_from_path_source_root_confinement: count=106505 total_ns=14803885 avg_ns=138 max_ns=10024
  resolved_virtual_path_from_path_virtual_conversion: count=53253 total_ns=17170597 avg_ns=322 max_ns=31197
  resolved_virtual_path_from_open_fd: count=13313 total_ns=15447780 avg_ns=1160 max_ns=15963
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
