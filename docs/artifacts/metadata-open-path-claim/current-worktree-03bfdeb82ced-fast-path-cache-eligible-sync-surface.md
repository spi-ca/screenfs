# ScreenFS benchmark result

- timestamp: `2026-06-15T09:18:47.206460+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --iterations 10 --warmups 3 --perf-counters --small-files 1024 --dir-entries 2048 --metadata-ops 1024 --sync-ops 64 --hidden-misses 512 --matcher-misses 512 --symlink-parent-mutations 64 --policy-preset fast-path-cache-eligible --workload-set sync-surface --output-json docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json --output-md docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.md --output-svg docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+83 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `1b9b5de4a9f56ef6ff65a27c0a01f690a922ab5d9e75de913edf39db4ee4f94d`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+83 more)`
- screenfs_source_git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `sync-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `sync_flush_only, sync_fsync_only, sync_release_flush`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| sync_flush_only | 0.000384 | 0.008478 | 22.080 | 0.008733 | 0.008772 | 0.008803 |
| sync_fsync_only | 0.000160 | 0.002052 | 12.843 | 0.002332 | 0.002700 | 0.002994 |
| sync_release_flush | 0.000287 | 0.006665 | 23.263 | 0.008009 | 0.008181 | 0.008318 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=84457 avg_ns=84457 max_ns=84457
  fuse_op.create: count=39 total_ns=2393702 avg_ns=61376 max_ns=120192
  fuse_op.flush: count=1677 total_ns=5754933 avg_ns=3431 max_ns=129519
  fuse_op.fsync: count=832 total_ns=2326042 avg_ns=2795 max_ns=28414
  fuse_op.getattr: count=833 total_ns=8840452 avg_ns=10612 max_ns=43561
  fuse_op.getxattr: count=4134 total_ns=45142471 avg_ns=10919 max_ns=81763
  fuse_op.lookup: count=5153 total_ns=52046737 avg_ns=10100 max_ns=128693
  fuse_op.open: count=1638 total_ns=26468368 avg_ns=16158 max_ns=67347
  fuse_op.release: count=1677 total_ns=912419 avg_ns=544 max_ns=4804
  fuse_op.statfs: count=2 total_ns=2645 avg_ns=1322 max_ns=1338
  fuse_op.unlink: count=39 total_ns=1567538 avg_ns=40193 max_ns=46809
  fuse_op.write: count=2496 total_ns=4205733 avg_ns=1684 max_ns=12253
  policy_decision: count=43115 total_ns=13406449 avg_ns=310 max_ns=5508
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=39566 total_ns=4675572 avg_ns=118 max_ns=37961
  matcher_candidate_order.path: count=125796 total_ns=15162221 avg_ns=120 max_ns=18403
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=525312
  matcher_candidate_order_ancestor_steps.descendant: count=124308
  matcher_candidate_order_ancestor_steps.path: count=401004
  state_read_lock_wait: count=16842 total_ns=316799 avg_ns=18 max_ns=328
  state_read_lock_hold: count=16842 total_ns=1317648 avg_ns=78 max_ns=39184
  state_write_lock_wait: count=8505 total_ns=165050 avg_ns=19 max_ns=1968
  state_write_lock_hold: count=8505 total_ns=1678411 avg_ns=197 max_ns=7627
  open_confined_openat2: count=23947 total_ns=10043799 avg_ns=419 max_ns=12610
  source_root_path: count=23985 total_ns=27260525 avg_ns=1136 max_ns=54495
  resolved_virtual_path: count=24063 total_ns=19716387 avg_ns=819 max_ns=39967
  resolved_virtual_path_from_path: count=78 total_ns=224309 avg_ns=2875 max_ns=7816
  resolved_virtual_path_from_path_component_walk: count=78 total_ns=193962 avg_ns=2486 max_ns=7163
  resolved_virtual_path_from_path_canonicalize: count=156 total_ns=137695 avg_ns=882 max_ns=2651
  resolved_virtual_path_from_path_source_root_confinement: count=156 total_ns=20834 avg_ns=133 max_ns=604
  resolved_virtual_path_from_path_virtual_conversion: count=78 total_ns=24173 avg_ns=309 max_ns=501
  resolved_virtual_path_from_open_fd: count=23985 total_ns=19492078 avg_ns=812 max_ns=39967
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=2496 total_ns=444752 avg_ns=178 max_ns=1513
  write_guard_mutation: count=2496 total_ns=35725 avg_ns=14 max_ns=243
  write_io: count=2496 total_ns=3339815 avg_ns=1338 max_ns=11983
  file_sync.flush: count=1677 total_ns=5251919 avg_ns=3131 max_ns=128524
  file_sync.fsync: count=832 total_ns=2094678 avg_ns=2517 max_ns=26631
  write_size_bucket.0_4k: count=2496 total_ns=3339815 avg_ns=1338 max_ns=11983
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
  invalidations: count=78 invalidated_entries=39 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
