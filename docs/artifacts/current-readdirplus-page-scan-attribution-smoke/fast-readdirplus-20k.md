# ScreenFS benchmark result

- timestamp: `2026-06-24T00:25:29.054090+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.svg --dir-entries 20000 --workload readdirplus_basic`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `2251a0c12c055bc7d667d81ed3bc6b49e0c7a1e8e8c4cff045b6f4cc19f1da4c`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+11 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.031523 | 1.542030 | 48.918 | 1.554336 | 1.555874 | 1.557105 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=52210 avg_ns=52210 max_ns=52210
  fuse_op.getattr: count=80005 total_ns=244631940 avg_ns=3057 max_ns=74766
  fuse_op.lookup: count=240013 total_ns=910542168 avg_ns=3793 max_ns=94779
  fuse_op.opendir: count=4 total_ns=134470 avg_ns=33617 max_ns=61682
  fuse_op.readdir: count=98 total_ns=3106439337 avg_ns=31698360 max_ns=79134194
  fuse_op.readdirplus: count=24 total_ns=770662316 avg_ns=32110929 max_ns=74889679
  fuse_op.releasedir: count=4 total_ns=7937589 avg_ns=1984397 max_ns=3256869
  fuse_op.statfs: count=2 total_ns=5743 avg_ns=2871 max_ns=3665
  policy_decision: count=1543941 total_ns=813371067 avg_ns=526 max_ns=285580
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1543941 total_ns=285465882 avg_ns=184 max_ns=44101
  matcher_candidate_order.path: count=4631823 total_ns=885851228 avg_ns=191 max_ns=2020535
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=23742372
  matcher_candidate_order_ancestor_steps.descendant: count=5935593
  matcher_candidate_order_ancestor_steps.path: count=17806779
  state_read_lock_wait: count=320145 total_ns=6786491 avg_ns=21 max_ns=3749
  state_read_lock_hold: count=320145 total_ns=19893081 avg_ns=62 max_ns=7430
  state_write_lock_wait: count=240145 total_ns=4920379 avg_ns=20 max_ns=4257
  state_write_lock_hold: count=240145 total_ns=317293047 avg_ns=1321 max_ns=3256314
  open_confined_openat2: count=324658 total_ns=214571780 avg_ns=660 max_ns=71521
  open_like.pre_open_guard.access: count=1 total_ns=9661 avg_ns=9661 max_ns=9661
  open_like.pre_open_guard.opendir: count=4 total_ns=26070 avg_ns=6517 max_ns=13965
  open_like.post_open_revalidation.access: count=1 total_ns=38100 avg_ns=38100 max_ns=38100
  open_like.post_open_revalidation.opendir: count=4 total_ns=95290 avg_ns=23822 max_ns=42342
  stat_child_no_follow: count=324531 total_ns=378285433 avg_ns=1165 max_ns=89594
  stat_child_no_follow.attr_conversion: count=324529 total_ns=4946128 avg_ns=15 max_ns=49826
  stat_child_no_follow.host_fstat: count=324529 total_ns=62468244 avg_ns=192 max_ns=87556
  stat_child_no_follow_context.path_guard_or_metadata: count=324531 total_ns=378285433 avg_ns=1165 max_ns=89594
  source_root_path: count=127 total_ns=2161370 avg_ns=17018 max_ns=42188
  resolved_virtual_path: count=127 total_ns=456385 avg_ns=3593 max_ns=12277
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=127 total_ns=456385 avg_ns=3593 max_ns=12277
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=98 total_ns=2934158596 avg_ns=29940393 max_ns=77592212
  readdir_scan.name_child_path_materialization: count=98 total_ns=271767432 avg_ns=2773137 max_ns=7029396
  readdir_scan.scan_fallback_attr: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=98 total_ns=2124904515 avg_ns=21682699 max_ns=60891977
  readdir_attr_generation_scan: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=98 total_ns=215572706 avg_ns=2199721 max_ns=5641134
  readdir_page_commit: count=98 total_ns=147636421 avg_ns=1506494 max_ns=3236757
  readdirplus_directory_scan: count=24 total_ns=736738341 avg_ns=30697430 max_ns=73943529
  readdirplus_scan.name_child_path_materialization: count=24 total_ns=66324685 avg_ns=2763528 max_ns=6466468
  readdirplus_scan.returned_attr_hydration: count=4500 total_ns=7046498 avg_ns=1565 max_ns=24047
  readdirplus_scan.returned_policy_recheck: count=4500 total_ns=8702585 avg_ns=1933 max_ns=7493
  readdirplus_scan.scan_fallback_attr: count=24 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=24 total_ns=528955527 avg_ns=22039813 max_ns=58433351
  readdirplus_attr_generation_scan: count=4524 total_ns=7046498 avg_ns=1557 max_ns=24047
  readdirplus_attr_generation_entries: count=4500
  readdirplus_symlink_visibility: count=24 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=24 total_ns=51262968 avg_ns=2135957 max_ns=4693962
  readdirplus_page_commit: count=24 total_ns=15608409 avg_ns=650350 max_ns=1434634
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
