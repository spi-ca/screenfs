# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:19.877728+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-open-confined-surface.svg --workload-set open-confined-surface`
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
| metadata_open | 0.004601 | 0.052874 | 11.492 | 0.054704 | 0.057468 | 0.059680 |
| metadata_opendir | 0.001599 | 0.041267 | 25.815 | 0.042982 | 0.044726 | 0.046121 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=97337 avg_ns=97337 max_ns=97337
  fuse_op.getattr: count=6657 total_ns=83634982 avg_ns=12563 max_ns=130129
  fuse_op.lookup: count=33285 total_ns=487487487 avg_ns=14645 max_ns=11822429
  fuse_op.open: count=6656 total_ns=143613991 avg_ns=21576 max_ns=153988
  fuse_op.opendir: count=6656 total_ns=111113343 avg_ns=16693 max_ns=178663
  fuse_op.release: count=6656 total_ns=7075632 avg_ns=1063 max_ns=30899
  fuse_op.releasedir: count=6656 total_ns=2656022 avg_ns=399 max_ns=7383
  fuse_op.statfs: count=2 total_ns=9945 avg_ns=4972 max_ns=6504
  policy_decision: count=119822 total_ns=43754584 avg_ns=365 max_ns=42787
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=13963331 avg_ns=116 max_ns=7447
  matcher_candidate_order.path: count=359466 total_ns=51068907 avg_ns=142 max_ns=42895
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=119822
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=119822
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=1498082 avg_ns=28 max_ns=1316
  state_read_lock_hold: count=53255 total_ns=4343140 avg_ns=81 max_ns=10823
  state_write_lock_wait: count=59907 total_ns=1391408 avg_ns=23 max_ns=4613
  state_write_lock_hold: count=59907 total_ns=20537160 avg_ns=342 max_ns=51944
  open_confined_openat2: count=66568 total_ns=86140402 avg_ns=1294 max_ns=11706838
  open_like.pre_open_guard.access: count=1 total_ns=67274 avg_ns=67274 max_ns=67274
  open_like.pre_open_guard.open: count=6656 total_ns=110727331 avg_ns=16635 max_ns=115691
  open_like.pre_open_guard.opendir: count=6656 total_ns=82621965 avg_ns=12413 max_ns=128839
  open_like.post_open_revalidation.access: count=1 total_ns=14027 avg_ns=14027 max_ns=14027
  open_like.post_open_revalidation.open: count=6656 total_ns=22762109 avg_ns=3419 max_ns=48599
  open_like.post_open_revalidation.opendir: count=6656 total_ns=19421412 avg_ns=2917 max_ns=31499
  stat_child_no_follow: count=53255 total_ns=468613478 avg_ns=8799 max_ns=11803161
  stat_child_no_follow.attr_conversion: count=53253 total_ns=903645 avg_ns=16 max_ns=5639
  stat_child_no_follow.directory_revalidation: count=53254 total_ns=328881543 avg_ns=6175 max_ns=122690
  stat_child_no_follow.host_fstat: count=1 total_ns=2248 avg_ns=2248 max_ns=2248
  stat_child_no_follow.host_fstatat: count=53254 total_ns=23700284 avg_ns=445 max_ns=55634
  stat_child_no_follow.parent_open: count=53254 total_ns=89369630 avg_ns=1678 max_ns=11711373
  stat_child_no_follow_context.path_guard_or_metadata: count=53255 total_ns=468613478 avg_ns=8799 max_ns=11803161
  source_root_path: count=53255 total_ns=100846369 avg_ns=1893 max_ns=117629
  resolved_virtual_path: count=173074 total_ns=322256424 avg_ns=1861 max_ns=127773
  resolved_virtual_path_from_path: count=106507 total_ns=243956892 avg_ns=2290 max_ns=127773
  resolved_virtual_path_from_path_component_walk: count=106507 total_ns=205234298 avg_ns=1926 max_ns=127431
  resolved_virtual_path_from_path_canonicalize: count=159758 total_ns=162274592 avg_ns=1015 max_ns=126869
  resolved_virtual_path_from_path_source_root_confinement: count=159758 total_ns=21748101 avg_ns=136 max_ns=7659
  resolved_virtual_path_from_path_virtual_conversion: count=106507 total_ns=32298177 avg_ns=303 max_ns=44281
  resolved_virtual_path_from_open_fd: count=66567 total_ns=78299532 avg_ns=1176 max_ns=62700
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
