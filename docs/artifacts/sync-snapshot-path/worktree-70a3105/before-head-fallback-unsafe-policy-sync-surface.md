# ScreenFS benchmark result

- timestamp: `2026-06-21T03:11:40.582823+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-sync-head.AcV5Ch/target/release/screenfs --screenfs-source-root /tmp/screenfs-sync-head.AcV5Ch --policy-preset fallback-unsafe-policy --workload-set sync-surface --cache-control warm --iterations 10 --warmups 3 --sync-bytes 4096 --sync-ops 128 --perf-counters --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fallback-unsafe-policy-sync-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fallback-unsafe-policy-sync-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/sync-snapshot-path/worktree-70a3105/before-head-fallback-unsafe-policy-sync-surface.svg`
- harness_repo_root: `/tmp/screenfs-sync-head.AcV5Ch`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-sync-head.AcV5Ch/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-sync-head.AcV5Ch`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
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
| sync_flush_only | 0.000644 | 0.028571 | 44.360 | 0.029471 | 0.029617 | 0.029733 |
| sync_fsync_only | 0.000160 | 0.010259 | 64.182 | 0.011048 | 0.011096 | 0.011135 |
| sync_release_flush | 0.000327 | 0.025523 | 78.026 | 0.026757 | 0.026789 | 0.026814 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=57151 avg_ns=57151 max_ns=57151
  fuse_op.create: count=39 total_ns=4816214 avg_ns=123492 max_ns=189688
  fuse_op.flush: count=3341 total_ns=18167101 avg_ns=5437 max_ns=226960
  fuse_op.fsync: count=1664 total_ns=7339852 avg_ns=4410 max_ns=45639
  fuse_op.getattr: count=1665 total_ns=27011807 avg_ns=16223 max_ns=50421
  fuse_op.getxattr: count=8294 total_ns=201880143 avg_ns=24340 max_ns=63433
  fuse_op.lookup: count=10145 total_ns=134964034 avg_ns=13303 max_ns=71269
  fuse_op.open: count=3302 total_ns=95534016 avg_ns=28932 max_ns=54157
  fuse_op.release: count=3341 total_ns=1873192 avg_ns=560 max_ns=5086
  fuse_op.statfs: count=2 total_ns=4743 avg_ns=2371 max_ns=3042
  fuse_op.unlink: count=39 total_ns=3477940 avg_ns=89177 max_ns=169518
  fuse_op.write: count=4992 total_ns=142421242 avg_ns=28529 max_ns=236746
  policy_decision: count=107888 total_ns=54938142 avg_ns=509 max_ns=12131
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91027 total_ns=13743374 avg_ns=150 max_ns=7963
  matcher_candidate_order.path: count=306803 total_ns=60211629 avg_ns=196 max_ns=9718
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=107888
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=107888
  matcher_candidate_order_ancestor_steps: count=1393096
  matcher_candidate_order_ancestor_steps.descendant: count=314630
  matcher_candidate_order_ancestor_steps.path: count=1078466
  state_read_lock_wait: count=33482 total_ns=801304 avg_ns=23 max_ns=532
  state_read_lock_hold: count=33482 total_ns=2672770 avg_ns=79 max_ns=1868
  state_write_lock_wait: count=16825 total_ns=403682 avg_ns=23 max_ns=565
  state_write_lock_hold: count=16825 total_ns=3376052 avg_ns=200 max_ns=41351
  open_confined_openat2: count=40464 total_ns=28779466 avg_ns=711 max_ns=16759
  open_like.pre_open_guard.access: count=1 total_ns=41299 avg_ns=41299 max_ns=41299
  open_like.pre_open_guard.open: count=3302 total_ns=64700931 avg_ns=19594 max_ns=40442
  open_like.post_open_revalidation.access: count=1 total_ns=7207 avg_ns=7207 max_ns=7207
  open_like.post_open_revalidation.open: count=3302 total_ns=22340506 avg_ns=6765 max_ns=15239
  stat_child_no_follow: count=28789 total_ns=256364230 avg_ns=8904 max_ns=59812
  source_root_path: count=45416 total_ns=79003800 avg_ns=1739 max_ns=30326
  resolved_virtual_path: count=102913 total_ns=236380844 avg_ns=2296 max_ns=30091
  resolved_virtual_path_from_path: count=57419 total_ns=183239755 avg_ns=3191 max_ns=28974
  resolved_virtual_path_from_path_component_walk: count=57419 total_ns=160072563 avg_ns=2787 max_ns=28451
  resolved_virtual_path_from_path_canonicalize: count=122864 total_ns=130248844 avg_ns=1060 max_ns=27443
  resolved_virtual_path_from_path_source_root_confinement: count=122864 total_ns=15890498 avg_ns=129 max_ns=13161
  resolved_virtual_path_from_path_virtual_conversion: count=57419 total_ns=19574812 avg_ns=340 max_ns=11032
  resolved_virtual_path_from_open_fd: count=45494 total_ns=53141089 avg_ns=1168 max_ns=30091
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=4992 total_ns=1158174 avg_ns=232 max_ns=2290
  write_guard_mutation: count=4992 total_ns=132186872 avg_ns=26479 max_ns=58692
  write_io: count=4992 total_ns=8095848 avg_ns=1621 max_ns=210701
  file_sync.flush: count=3341 total_ns=17191834 avg_ns=5145 max_ns=225195
  file_sync.fsync: count=1664 total_ns=6830793 avg_ns=4105 max_ns=44894
  write_size_bucket.0_4k: count=4992 total_ns=8095848 avg_ns=1621 max_ns=210701
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
