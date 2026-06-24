# ScreenFS benchmark result

- timestamp: `2026-06-24T00:25:20.773414+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.svg --workload-set directory-surface`
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
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.001256 | 0.070500 | 56.112 | 0.070777 | 0.070811 | 0.070839 |
| readdirplus_basic | 0.006896 | 0.245208 | 35.556 | 0.245620 | 0.245671 | 0.245712 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=86636 avg_ns=86636 max_ns=86636
  fuse_op.getattr: count=20009 total_ns=67495795 avg_ns=3373 max_ns=322542
  fuse_op.lookup: count=60021 total_ns=246949756 avg_ns=4114 max_ns=320894
  fuse_op.opendir: count=8 total_ns=213323 avg_ns=26665 max_ns=40241
  fuse_op.readdir: count=54 total_ns=413650066 avg_ns=7660186 max_ns=14944716
  fuse_op.readdirplus: count=13 total_ns=139808598 avg_ns=10754507 max_ns=14354674
  fuse_op.releasedir: count=8 total_ns=4556229 avg_ns=569528 max_ns=706507
  fuse_op.statfs: count=2 total_ns=7590 avg_ns=3795 max_ns=5421
  policy_decision: count=262129 total_ns=122188246 avg_ns=466 max_ns=31843
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=262129 total_ns=43699303 avg_ns=166 max_ns=315599
  matcher_candidate_order.path: count=786387 total_ns=135010250 avg_ns=171 max_ns=21862
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=3953456
  matcher_candidate_order_ancestor_steps.descendant: count=988364
  matcher_candidate_order_ancestor_steps.path: count=2965092
  state_read_lock_wait: count=80106 total_ns=1871698 avg_ns=23 max_ns=7587
  state_read_lock_hold: count=80106 total_ns=5389317 avg_ns=67 max_ns=8501
  state_write_lock_wait: count=60110 total_ns=1374038 avg_ns=22 max_ns=3487
  state_write_lock_hold: count=60110 total_ns=95913530 avg_ns=1595 max_ns=3050809
  open_confined_openat2: count=82467 total_ns=65452622 avg_ns=793 max_ns=320375
  open_like.pre_open_guard.access: count=1 total_ns=30973 avg_ns=30973 max_ns=30973
  open_like.pre_open_guard.opendir: count=8 total_ns=35720 avg_ns=4465 max_ns=10391
  open_like.post_open_revalidation.access: count=1 total_ns=44212 avg_ns=44212 max_ns=44212
  open_like.post_open_revalidation.opendir: count=8 total_ns=156139 avg_ns=19517 max_ns=28137
  stat_child_no_follow: count=82391 total_ns=112590970 avg_ns=1366 max_ns=320770
  stat_child_no_follow.attr_conversion: count=82389 total_ns=1385205 avg_ns=16 max_ns=497
  stat_child_no_follow.host_fstat: count=82389 total_ns=19073216 avg_ns=231 max_ns=8742
  stat_child_no_follow_context.path_guard_or_metadata: count=82391 total_ns=112590970 avg_ns=1366 max_ns=320770
  source_root_path: count=76 total_ns=1423049 avg_ns=18724 max_ns=51888
  resolved_virtual_path: count=76 total_ns=303792 avg_ns=3997 max_ns=8069
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=76 total_ns=303792 avg_ns=3997 max_ns=8069
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=351175315 avg_ns=6503246 max_ns=13395144
  readdir_scan.name_child_path_materialization: count=54 total_ns=35586495 avg_ns=659009 max_ns=1417715
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=238601912 avg_ns=4418553 max_ns=9363566
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=26189426 avg_ns=484989 max_ns=1264660
  readdir_page_commit: count=54 total_ns=51720119 avg_ns=957779 max_ns=3051202
  readdirplus_directory_scan: count=13 total_ns=127695685 avg_ns=9822745 max_ns=13349861
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=12262231 avg_ns=943248 max_ns=1325958
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=4016348 avg_ns=1719 max_ns=18271
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=4353735 avg_ns=1863 max_ns=6028
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=91984803 avg_ns=7075754 max_ns=9741366
  readdirplus_attr_generation_scan: count=2349 total_ns=4016348 avg_ns=1709 max_ns=18271
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=9390370 avg_ns=722336 max_ns=1048703
  readdirplus_page_commit: count=13 total_ns=2645048 avg_ns=203465 max_ns=329427
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
