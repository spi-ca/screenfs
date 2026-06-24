# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:48.097057+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-opendir.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-opendir.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-opendir.svg --workload metadata_opendir`
- harness_repo_root: `/tmp/screenfs-bench-before-open-confined-168995`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `ca7372519260f85471e13bfc19189bf974d6fc0a973eb7e02270315c8d89cb97`
- screenfs_source_root: `/tmp/screenfs-bench-before-open-confined-168995`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_worktree_clean: `True`
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
| metadata_opendir | 0.001433 | 0.040722 | 28.420 | 0.041616 | 0.042180 | 0.042631 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=79182 avg_ns=79182 max_ns=79182
  fuse_op.getattr: count=6657 total_ns=83855739 avg_ns=12596 max_ns=62498
  fuse_op.lookup: count=13317 total_ns=156772546 avg_ns=11772 max_ns=80454
  fuse_op.opendir: count=6656 total_ns=112731092 avg_ns=16936 max_ns=283227
  fuse_op.releasedir: count=6656 total_ns=2423236 avg_ns=364 max_ns=8734
  fuse_op.statfs: count=2 total_ns=4999 avg_ns=2499 max_ns=2663
  policy_decision: count=59918 total_ns=18951637 avg_ns=316 max_ns=11738
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=5804828 avg_ns=96 max_ns=8374
  matcher_candidate_order.path: count=179754 total_ns=22666654 avg_ns=126 max_ns=264808
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=559224
  matcher_candidate_order_ancestor_steps.descendant: count=139806
  matcher_candidate_order_ancestor_steps.path: count=419418
  state_read_lock_wait: count=26631 total_ns=696994 avg_ns=26 max_ns=417
  state_read_lock_hold: count=26631 total_ns=2009194 avg_ns=75 max_ns=8438
  state_write_lock_wait: count=26627 total_ns=634808 avg_ns=23 max_ns=760
  state_write_lock_hold: count=26627 total_ns=5086284 avg_ns=191 max_ns=45633
  open_confined_openat2: count=33288 total_ns=22778014 avg_ns=684 max_ns=16446
  open_like.pre_open_guard.access: count=1 total_ns=63120 avg_ns=63120 max_ns=63120
  open_like.pre_open_guard.opendir: count=6656 total_ns=84114530 avg_ns=12637 max_ns=278019
  open_like.post_open_revalidation.access: count=1 total_ns=9396 avg_ns=9396 max_ns=9396
  open_like.post_open_revalidation.opendir: count=6656 total_ns=19482488 avg_ns=2927 max_ns=70828
  stat_child_no_follow: count=26631 total_ns=197813981 avg_ns=7427 max_ns=77671
  stat_child_no_follow.attr_conversion: count=26629 total_ns=463788 avg_ns=17 max_ns=157
  stat_child_no_follow.directory_revalidation: count=26630 total_ns=150148130 avg_ns=5638 max_ns=73385
  stat_child_no_follow.host_fstat: count=1 total_ns=1850 avg_ns=1850 max_ns=1850
  stat_child_no_follow.host_fstatat: count=26630 total_ns=9802312 avg_ns=368 max_ns=48587
  stat_child_no_follow.parent_open: count=26630 total_ns=24784380 avg_ns=930 max_ns=16749
  stat_child_no_follow_context.path_guard_or_metadata: count=26631 total_ns=197813981 avg_ns=7427 max_ns=77671
  source_root_path: count=26631 total_ns=51592244 avg_ns=1937 max_ns=67502
  resolved_virtual_path: count=86546 total_ns=138329045 avg_ns=1598 max_ns=69171
  resolved_virtual_path_from_path: count=53259 total_ns=102256072 avg_ns=1919 max_ns=65399
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=83986110 avg_ns=1576 max_ns=65095
  resolved_virtual_path_from_path_canonicalize: count=66574 total_ns=66178364 avg_ns=994 max_ns=64801
  resolved_virtual_path_from_path_source_root_confinement: count=66574 total_ns=8877770 avg_ns=133 max_ns=46300
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=15020989 avg_ns=282 max_ns=8209
  resolved_virtual_path_from_open_fd: count=33287 total_ns=36072973 avg_ns=1083 max_ns=69171
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
