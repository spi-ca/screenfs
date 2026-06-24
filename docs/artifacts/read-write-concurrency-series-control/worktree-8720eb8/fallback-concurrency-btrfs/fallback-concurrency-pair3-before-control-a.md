# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:53.923844+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-before-control-a.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-before-control-a.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-before-control-a.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-rw-before-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `read-write-concurrency-control`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
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
| concurrent_rand_read_write_4k | 0.047108 | 0.391183 | 8.304 | 0.432624 | 0.437804 | 0.441948 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=90264 avg_ns=90264 max_ns=90264
  fuse_op.create: count=16 total_ns=2333917 avg_ns=145869 max_ns=305520
  fuse_op.flush: count=16 total_ns=1113865491 avg_ns=69616593 max_ns=115404390
  fuse_op.getattr: count=32769 total_ns=574641928 avg_ns=17536 max_ns=4073869
  fuse_op.getxattr: count=32784 total_ns=922927543 avg_ns=28151 max_ns=2583446
  fuse_op.lookup: count=149 total_ns=3710165 avg_ns=24900 max_ns=115454
  fuse_op.open: count=16 total_ns=499478 avg_ns=31217 max_ns=67637
  fuse_op.read: count=32240 total_ns=785727905 avg_ns=24371 max_ns=2239504
  fuse_op.release: count=32 total_ns=76526 avg_ns=2391 max_ns=6644
  fuse_op.setattr: count=16 total_ns=1011950 avg_ns=63246 max_ns=130206
  fuse_op.statfs: count=2 total_ns=7120 avg_ns=3560 max_ns=5349
  fuse_op.unlink: count=16 total_ns=31786509 avg_ns=1986656 max_ns=4187234
  fuse_op.write: count=32768 total_ns=1310080616 avg_ns=39980 max_ns=3310768
  policy_decision: count=360104 total_ns=183863870 avg_ns=510 max_ns=405486
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=294424 total_ns=45874070 avg_ns=155 max_ns=163589
  matcher_candidate_order.path: count=1014632 total_ns=214938858 avg_ns=211 max_ns=589064
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=83525271 avg_ns=283 max_ns=589064
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=55559732 avg_ns=188 max_ns=229126
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=17755804 avg_ns=270 max_ns=128567
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=45874070 avg_ns=155 max_ns=163589
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=47195597 avg_ns=160 max_ns=345916
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=10902454 avg_ns=165 max_ns=34962
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=9285645 avg_ns=70 max_ns=107642
  state_read_lock_hold: count=130791 total_ns=14401625 avg_ns=110 max_ns=58061
  state_write_lock_wait: count=211 total_ns=14181 avg_ns=67 max_ns=2183
  state_write_lock_hold: count=211 total_ns=325373 avg_ns=1542 max_ns=16323
  open_confined_openat2: count=163768 total_ns=162533540 avg_ns=992 max_ns=1908616
  open_like.pre_open_guard.access: count=1 total_ns=57668 avg_ns=57668 max_ns=57668
  open_like.pre_open_guard.open: count=16 total_ns=335438 avg_ns=20964 max_ns=43218
  open_like.post_open_revalidation.access: count=1 total_ns=12843 avg_ns=12843 max_ns=12843
  open_like.post_open_revalidation.open: count=16 total_ns=103097 avg_ns=6443 max_ns=17152
  stat_child_no_follow: count=130919 total_ns=226875350 avg_ns=1732 max_ns=2008439
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2005292 avg_ns=15 max_ns=25358
  stat_child_no_follow.host_fstat: count=130837 total_ns=32815070 avg_ns=250 max_ns=2006765
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=226875350 avg_ns=1732 max_ns=2008439
  source_root_path: count=163655 total_ns=496611344 avg_ns=3034 max_ns=4037802
  resolved_virtual_path: count=228726 total_ns=1148818600 avg_ns=5022 max_ns=1894668
  resolved_virtual_path_from_path: count=130853 total_ns=1007109538 avg_ns=7696 max_ns=1894668
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=931745827 avg_ns=7120 max_ns=1636855
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=828865279 avg_ns=2112 max_ns=1634668
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=62918740 avg_ns=160 max_ns=301437
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=66425811 avg_ns=507 max_ns=1886825
  resolved_virtual_path_from_open_fd: count=97873 total_ns=141709062 avg_ns=1447 max_ns=727622
  read_handle_snapshot: count=32240 total_ns=10454032 avg_ns=324 max_ns=39756
  read_guard_path: count=32240 total_ns=734707699 avg_ns=22788 max_ns=2237089
  read_io: count=32240 total_ns=31085421 avg_ns=964 max_ns=626690
  write_handle_snapshot: count=32768 total_ns=11044139 avg_ns=337 max_ns=190206
  write_guard_mutation: count=32768 total_ns=1163990352 avg_ns=35522 max_ns=3305600
  write_io: count=32768 total_ns=124033829 avg_ns=3785 max_ns=1110812
  file_sync.flush: count=16 total_ns=1113849092 avg_ns=69615568 max_ns=115403702
  read_size_bucket.0_4k: count=32236 total_ns=31078432 avg_ns=964 max_ns=626690
  read_size_bucket.4k_64k: count=4 total_ns=6989 avg_ns=1747 max_ns=2764
  write_size_bucket.0_4k: count=32768 total_ns=124033829 avg_ns=3785 max_ns=1110812
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
  invalidations: count=32 invalidated_entries=16 evicted_entries=0 scanned_entries=200
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
