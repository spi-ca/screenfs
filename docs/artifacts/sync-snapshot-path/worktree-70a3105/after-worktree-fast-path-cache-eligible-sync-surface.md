# ScreenFS benchmark result

- timestamp: `2026-06-21T03:11:41.815109+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --workload-set sync-surface --cache-control warm --iterations 10 --warmups 3 --sync-bytes 4096 --sync-ops 128 --perf-counters --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fast-path-cache-eligible-sync-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fast-path-cache-eligible-sync-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fast-path-cache-eligible-sync-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `55bf6728cbc17f0c6632c800b5edb4412e497ac22d0f5e39932d935fafe3e6d4`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+9 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `sync-surface`
- comparable_workloads: `sync_flush_only, sync_fsync_only, sync_release_flush`
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
| sync_flush_only | 0.000366 | 0.015900 | 43.491 | 0.016488 | 0.016511 | 0.016530 |
| sync_fsync_only | 0.000191 | 0.004407 | 23.130 | 0.004662 | 0.004690 | 0.004713 |
| sync_release_flush | 0.000270 | 0.013251 | 49.001 | 0.014129 | 0.014155 | 0.014176 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=40628 avg_ns=40628 max_ns=40628
  fuse_op.create: count=39 total_ns=2678365 avg_ns=68676 max_ns=101440
  fuse_op.flush: count=3341 total_ns=10575965 avg_ns=3165 max_ns=164120
  fuse_op.fsync: count=1664 total_ns=4669815 avg_ns=2806 max_ns=20163
  fuse_op.getattr: count=1665 total_ns=14097949 avg_ns=8467 max_ns=27006
  fuse_op.getxattr: count=8294 total_ns=110569471 avg_ns=13331 max_ns=114897
  fuse_op.lookup: count=10145 total_ns=74831918 avg_ns=7376 max_ns=273791
  fuse_op.open: count=3302 total_ns=56515579 avg_ns=17115 max_ns=43409
  fuse_op.release: count=3341 total_ns=1754562 avg_ns=525 max_ns=3203
  fuse_op.statfs: count=2 total_ns=1303 avg_ns=651 max_ns=670
  fuse_op.unlink: count=39 total_ns=1871856 avg_ns=47996 max_ns=67866
  fuse_op.write: count=4992 total_ns=8245534 avg_ns=1651 max_ns=15767
  policy_decision: count=72866 total_ns=23504248 avg_ns=322 max_ns=20502
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=65989 total_ns=7736118 avg_ns=117 max_ns=15032
  matcher_candidate_order.path: count=211721 total_ns=26430987 avg_ns=124 max_ns=47732
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=932896
  matcher_candidate_order_ancestor_steps.descendant: count=219548
  matcher_candidate_order_ancestor_steps.path: count=713348
  state_read_lock_wait: count=33482 total_ns=600800 avg_ns=17 max_ns=339
  state_read_lock_hold: count=33482 total_ns=2219428 avg_ns=66 max_ns=1046
  state_write_lock_wait: count=16825 total_ns=298031 avg_ns=17 max_ns=349
  state_write_lock_hold: count=16825 total_ns=2785310 avg_ns=165 max_ns=9742
  open_confined_openat2: count=35394 total_ns=15410020 avg_ns=435 max_ns=100669
  open_like.pre_open_guard.access: count=1 total_ns=22268 avg_ns=22268 max_ns=22268
  open_like.pre_open_guard.open: count=3302 total_ns=35387152 avg_ns=10716 max_ns=37177
  open_like.post_open_revalidation.access: count=1 total_ns=14905 avg_ns=14905 max_ns=14905
  open_like.post_open_revalidation.open: count=3302 total_ns=16203256 avg_ns=4907 max_ns=19843
  stat_child_no_follow: count=23719 total_ns=149410479 avg_ns=6299 max_ns=271417
  source_root_path: count=32052 total_ns=36810040 avg_ns=1148 max_ns=69429
  resolved_virtual_path: count=59228 total_ns=74394558 avg_ns=1256 max_ns=96718
  resolved_virtual_path_from_path: count=23796 total_ns=44960451 avg_ns=1889 max_ns=96718
  resolved_virtual_path_from_path_component_walk: count=23796 total_ns=37707754 avg_ns=1584 max_ns=96388
  resolved_virtual_path_from_path_canonicalize: count=37367 total_ns=29895144 avg_ns=800 max_ns=95896
  resolved_virtual_path_from_path_source_root_confinement: count=37367 total_ns=4402582 avg_ns=117 max_ns=667
  resolved_virtual_path_from_path_virtual_conversion: count=23796 total_ns=6086028 avg_ns=255 max_ns=2300
  resolved_virtual_path_from_open_fd: count=35432 total_ns=29434107 avg_ns=830 max_ns=85261
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=4992 total_ns=867011 avg_ns=173 max_ns=1006
  write_guard_mutation: count=4992 total_ns=70295 avg_ns=14 max_ns=286
  write_io: count=4992 total_ns=6580272 avg_ns=1318 max_ns=15373
  file_sync.flush: count=3341 total_ns=9715341 avg_ns=2907 max_ns=163784
  file_sync.fsync: count=1664 total_ns=4245502 avg_ns=2551 max_ns=19873
  write_size_bucket.0_4k: count=4992 total_ns=6580272 avg_ns=1318 max_ns=15373
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
  invalidations: count=78 invalidated_entries=39 evicted_entries=0 scanned_entries=234
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
