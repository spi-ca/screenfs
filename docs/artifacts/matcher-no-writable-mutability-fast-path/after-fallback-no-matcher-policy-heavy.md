# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:38.813410+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/after-fallback-no-matcher-policy-heavy.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/after-fallback-no-matcher-policy-heavy.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/after-fallback-no-matcher-policy-heavy.svg --policy-preset fallback-unsafe-policy --policy-label fallback-no-matcher --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-no-matcher`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
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
| metadata_lookup | 0.001461 | 0.010488 | 7.178 | 0.011178 | 0.011548 | 0.011844 |
| metadata_getattr | 0.001655 | 0.013651 | 8.250 | 0.014616 | 0.014653 | 0.014682 |
| metadata_access | 0.001496 | 0.014599 | 9.761 | 0.015259 | 0.015513 | 0.015715 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Skipped workloads

- `matcher_hidden_stat_miss`: matcher_hidden_stat_miss requires --matcher-extra-rules > 0
- `matcher_readonly_access_wok`: matcher_readonly_access_wok requires --matcher-extra-rules > 0

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=38878135 avg_ns=14947 max_ns=115117
  fuse_op.getattr: count=2601 total_ns=27305675 avg_ns=10498 max_ns=37650
  fuse_op.lookup: count=23405 total_ns=220533334 avg_ns=9422 max_ns=71788
  fuse_op.statfs: count=2 total_ns=5429 avg_ns=2714 max_ns=3080
  policy_decision: count=31208 total_ns=15392616 avg_ns=493 max_ns=19255
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=31208 total_ns=4549708 avg_ns=145 max_ns=7193
  matcher_candidate_order.path: count=93624 total_ns=17618937 avg_ns=188 max_ns=23662
  matcher_candidate_order_by_source.hidden.path: count=31208 total_ns=8218040 avg_ns=263 max_ns=5822
  matcher_candidate_order_by_source.internal_hidden.path: count=31208 total_ns=5096663 avg_ns=163 max_ns=23662
  matcher_candidate_order_by_source.visible.descendant: count=31208 total_ns=4549708 avg_ns=145 max_ns=7193
  matcher_candidate_order_by_source.visible.path: count=31208 total_ns=4304234 avg_ns=137 max_ns=5105
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=31208
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=31208
  matcher_candidate_order_ancestor_steps: count=405676
  matcher_candidate_order_ancestor_steps.descendant: count=101419
  matcher_candidate_order_ancestor_steps.path: count=304257
  state_read_lock_wait: count=28607 total_ns=701315 avg_ns=24 max_ns=640
  state_read_lock_hold: count=28607 total_ns=1837896 avg_ns=64 max_ns=5991
  state_write_lock_wait: count=20803 total_ns=508633 avg_ns=24 max_ns=17093
  state_write_lock_hold: count=20803 total_ns=7607437 avg_ns=365 max_ns=14763
  open_confined_openat2: count=31208 total_ns=26562740 avg_ns=851 max_ns=63730
  open_like.pre_open_guard.access: count=2601 total_ns=26431129 avg_ns=10161 max_ns=110051
  open_like.post_open_revalidation.access: count=2601 total_ns=9072376 avg_ns=3488 max_ns=22228
  stat_child_no_follow: count=28607 total_ns=42253559 avg_ns=1477 max_ns=100605
  stat_child_no_follow.attr_conversion: count=26005 total_ns=454993 avg_ns=17 max_ns=4744
  stat_child_no_follow.host_fstat: count=26005 total_ns=6397486 avg_ns=246 max_ns=99530
  stat_child_no_follow_context.path_guard_or_metadata: count=28607 total_ns=42253559 avg_ns=1477 max_ns=100605
  source_root_path: count=28607 total_ns=54341365 avg_ns=1899 max_ns=51132
  resolved_virtual_path: count=28606 total_ns=86526007 avg_ns=3024 max_ns=21468
  resolved_virtual_path_from_path: count=26005 total_ns=83440977 avg_ns=3208 max_ns=21468
  resolved_virtual_path_from_path_component_walk: count=26005 total_ns=72324965 avg_ns=2781 max_ns=20877
  resolved_virtual_path_from_path_canonicalize: count=54609 total_ns=57847317 avg_ns=1059 max_ns=15943
  resolved_virtual_path_from_path_source_root_confinement: count=54609 total_ns=8030412 avg_ns=147 max_ns=5484
  resolved_virtual_path_from_path_virtual_conversion: count=26005 total_ns=9491617 avg_ns=364 max_ns=5779
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3085030 avg_ns=1186 max_ns=5426
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
