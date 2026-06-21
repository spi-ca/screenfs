# ScreenFS benchmark result

- timestamp: `2026-06-21T03:11:43.588792+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --workload-set sync-surface --cache-control warm --iterations 10 --warmups 3 --sync-bytes 4096 --sync-ops 128 --perf-counters --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fallback-unsafe-policy-sync-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fallback-unsafe-policy-sync-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/after-worktree-fallback-unsafe-policy-sync-surface.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
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
| sync_flush_only | 0.000538 | 0.029342 | 54.544 | 0.031217 | 0.031223 | 0.031227 |
| sync_fsync_only | 0.000218 | 0.010136 | 46.476 | 0.010758 | 0.010826 | 0.010881 |
| sync_release_flush | 0.000399 | 0.026369 | 66.097 | 0.027077 | 0.027291 | 0.027462 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=52435 avg_ns=52435 max_ns=52435
  fuse_op.create: count=39 total_ns=4719956 avg_ns=121024 max_ns=169086
  fuse_op.flush: count=3341 total_ns=19279455 avg_ns=5770 max_ns=187022
  fuse_op.fsync: count=1664 total_ns=6932861 avg_ns=4166 max_ns=23489
  fuse_op.getattr: count=1665 total_ns=27238568 avg_ns=16359 max_ns=44027
  fuse_op.getxattr: count=8294 total_ns=202001867 avg_ns=24355 max_ns=67758
  fuse_op.lookup: count=10145 total_ns=136688438 avg_ns=13473 max_ns=66654
  fuse_op.open: count=3302 total_ns=96010888 avg_ns=29076 max_ns=86899
  fuse_op.release: count=3341 total_ns=2003013 avg_ns=599 max_ns=4248
  fuse_op.statfs: count=2 total_ns=6509 avg_ns=3254 max_ns=4073
  fuse_op.unlink: count=39 total_ns=3331504 avg_ns=85423 max_ns=108964
  fuse_op.write: count=4992 total_ns=143726485 avg_ns=28791 max_ns=86051
  policy_decision: count=107888 total_ns=55238061 avg_ns=511 max_ns=14927
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91027 total_ns=13910113 avg_ns=152 max_ns=12436
  matcher_candidate_order.path: count=306803 total_ns=61160633 avg_ns=199 max_ns=9810
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=107888
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=107888
  matcher_candidate_order_ancestor_steps: count=1393096
  matcher_candidate_order_ancestor_steps.descendant: count=314630
  matcher_candidate_order_ancestor_steps.path: count=1078466
  state_read_lock_wait: count=33482 total_ns=874702 avg_ns=26 max_ns=9797
  state_read_lock_hold: count=33482 total_ns=2486761 avg_ns=74 max_ns=2325
  state_write_lock_wait: count=16825 total_ns=414806 avg_ns=24 max_ns=378
  state_write_lock_hold: count=16825 total_ns=3630059 avg_ns=215 max_ns=38618
  open_confined_openat2: count=40464 total_ns=28836692 avg_ns=712 max_ns=22806
  open_like.pre_open_guard.access: count=1 total_ns=38289 avg_ns=38289 max_ns=38289
  open_like.pre_open_guard.open: count=3302 total_ns=65083886 avg_ns=19710 max_ns=72930
  open_like.post_open_revalidation.access: count=1 total_ns=6172 avg_ns=6172 max_ns=6172
  open_like.post_open_revalidation.open: count=3302 total_ns=22283122 avg_ns=6748 max_ns=20877
  stat_child_no_follow: count=28789 total_ns=258706733 avg_ns=8986 max_ns=60418
  source_root_path: count=45416 total_ns=80077040 avg_ns=1763 max_ns=35475
  resolved_virtual_path: count=102913 total_ns=237126417 avg_ns=2304 max_ns=37669
  resolved_virtual_path_from_path: count=57419 total_ns=184494609 avg_ns=3213 max_ns=18658
  resolved_virtual_path_from_path_component_walk: count=57419 total_ns=161582495 avg_ns=2814 max_ns=18208
  resolved_virtual_path_from_path_canonicalize: count=122864 total_ns=131040545 avg_ns=1066 max_ns=14213
  resolved_virtual_path_from_path_source_root_confinement: count=122864 total_ns=16163313 avg_ns=131 max_ns=5769
  resolved_virtual_path_from_path_virtual_conversion: count=57419 total_ns=19354225 avg_ns=337 max_ns=6627
  resolved_virtual_path_from_open_fd: count=45494 total_ns=52631808 avg_ns=1156 max_ns=37669
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=4992 total_ns=1065708 avg_ns=213 max_ns=2232
  write_guard_mutation: count=4992 total_ns=132218814 avg_ns=26486 max_ns=67234
  write_io: count=4992 total_ns=9502365 avg_ns=1903 max_ns=59587
  file_sync.flush: count=3341 total_ns=18339183 avg_ns=5489 max_ns=185778
  file_sync.fsync: count=1664 total_ns=6523780 avg_ns=3920 max_ns=23052
  write_size_bucket.0_4k: count=4992 total_ns=9502365 avg_ns=1903 max_ns=59587
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
