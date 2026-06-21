# ScreenFS benchmark result

- timestamp: `2026-06-21T03:11:38.785860+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-sync-head.AcV5Ch/target/release/screenfs --screenfs-source-root /tmp/screenfs-sync-head.AcV5Ch --policy-preset fast-path-cache-eligible --workload-set sync-surface --cache-control warm --iterations 10 --warmups 3 --sync-bytes 4096 --sync-ops 128 --perf-counters --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fast-path-cache-eligible-sync-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fast-path-cache-eligible-sync-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fast-path-cache-eligible-sync-surface.svg`
- harness_repo_root: `/tmp/screenfs-sync-head.AcV5Ch`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-sync-head.AcV5Ch/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-sync-head.AcV5Ch`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
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
| sync_flush_only | 0.000273 | 0.021490 | 78.591 | 0.023377 | 0.024592 | 0.025564 |
| sync_fsync_only | 0.000096 | 0.005287 | 54.917 | 0.006088 | 0.006094 | 0.006099 |
| sync_release_flush | 0.000200 | 0.018267 | 91.299 | 0.019381 | 0.020073 | 0.020627 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47590 avg_ns=47590 max_ns=47590
  fuse_op.create: count=39 total_ns=3470849 avg_ns=88996 max_ns=119630
  fuse_op.flush: count=3341 total_ns=18175726 avg_ns=5440 max_ns=147754
  fuse_op.fsync: count=1664 total_ns=6811267 avg_ns=4093 max_ns=20835
  fuse_op.getattr: count=1665 total_ns=17748647 avg_ns=10659 max_ns=57605
  fuse_op.getxattr: count=8294 total_ns=142566009 avg_ns=17189 max_ns=174703
  fuse_op.lookup: count=10145 total_ns=98339668 avg_ns=9693 max_ns=46064
  fuse_op.open: count=3302 total_ns=75813679 avg_ns=22959 max_ns=1479129
  fuse_op.release: count=3341 total_ns=2103509 avg_ns=629 max_ns=4502
  fuse_op.statfs: count=2 total_ns=6120 avg_ns=3060 max_ns=3474
  fuse_op.unlink: count=39 total_ns=2509676 avg_ns=64350 max_ns=91193
  fuse_op.write: count=4992 total_ns=8443795 avg_ns=1691 max_ns=12842
  policy_decision: count=72866 total_ns=28030571 avg_ns=384 max_ns=20029
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=65989 total_ns=9490707 avg_ns=143 max_ns=6256
  matcher_candidate_order.path: count=211721 total_ns=30947392 avg_ns=146 max_ns=14253
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=932896
  matcher_candidate_order_ancestor_steps.descendant: count=219548
  matcher_candidate_order_ancestor_steps.path: count=713348
  state_read_lock_wait: count=33482 total_ns=783601 avg_ns=23 max_ns=685
  state_read_lock_hold: count=33482 total_ns=2610056 avg_ns=77 max_ns=1851
  state_write_lock_wait: count=16825 total_ns=401610 avg_ns=23 max_ns=1324
  state_write_lock_hold: count=16825 total_ns=3629292 avg_ns=215 max_ns=30949
  open_confined_openat2: count=35394 total_ns=23969059 avg_ns=677 max_ns=80273
  open_like.pre_open_guard.access: count=1 total_ns=16537 avg_ns=16537 max_ns=16537
  open_like.pre_open_guard.open: count=3302 total_ns=47763077 avg_ns=14464 max_ns=1435544
  open_like.post_open_revalidation.access: count=1 total_ns=27652 avg_ns=27652 max_ns=27652
  open_like.post_open_revalidation.open: count=3302 total_ns=19307658 avg_ns=5847 max_ns=18487
  stat_child_no_follow: count=23719 total_ns=202371575 avg_ns=8532 max_ns=1423109
  source_root_path: count=32052 total_ns=57023164 avg_ns=1779 max_ns=72917
  resolved_virtual_path: count=59228 total_ns=95331660 avg_ns=1609 max_ns=1315089
  resolved_virtual_path_from_path: count=23796 total_ns=55853978 avg_ns=2347 max_ns=1315089
  resolved_virtual_path_from_path_component_walk: count=23796 total_ns=47499988 avg_ns=1996 max_ns=1312538
  resolved_virtual_path_from_path_canonicalize: count=37367 total_ns=38046564 avg_ns=1018 max_ns=1309985
  resolved_virtual_path_from_path_source_root_confinement: count=37367 total_ns=4896039 avg_ns=131 max_ns=1785
  resolved_virtual_path_from_path_virtual_conversion: count=23796 total_ns=6874173 avg_ns=288 max_ns=5668
  resolved_virtual_path_from_open_fd: count=35432 total_ns=39477682 avg_ns=1114 max_ns=17017
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=4992 total_ns=1095057 avg_ns=219 max_ns=1707
  write_guard_mutation: count=4992 total_ns=98627 avg_ns=19 max_ns=435
  write_io: count=4992 total_ns=6365483 avg_ns=1275 max_ns=12488
  file_sync.flush: count=3341 total_ns=17130874 avg_ns=5127 max_ns=143929
  file_sync.fsync: count=1664 total_ns=6344643 avg_ns=3812 max_ns=19352
  write_size_bucket.0_4k: count=4992 total_ns=6365483 avg_ns=1275 max_ns=12488
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
