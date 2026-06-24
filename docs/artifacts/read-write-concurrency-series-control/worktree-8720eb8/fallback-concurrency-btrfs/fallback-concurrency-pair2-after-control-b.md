# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:46.013950+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-after-control-b.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-after-control-b.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-after-control-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-rw-after-control`
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
| concurrent_rand_read_write_4k | 0.087505 | 0.420824 | 4.809 | 0.429174 | 0.430218 | 0.431053 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=88813 avg_ns=88813 max_ns=88813
  fuse_op.create: count=16 total_ns=2265485 avg_ns=141592 max_ns=223628
  fuse_op.flush: count=16 total_ns=1431640383 avg_ns=89477523 max_ns=126176846
  fuse_op.getattr: count=32769 total_ns=618098870 avg_ns=18862 max_ns=2054362
  fuse_op.getxattr: count=32784 total_ns=981187383 avg_ns=29928 max_ns=2057454
  fuse_op.lookup: count=149 total_ns=6175361 avg_ns=41445 max_ns=1510295
  fuse_op.open: count=16 total_ns=615192 avg_ns=38449 max_ns=74229
  fuse_op.read: count=32240 total_ns=841652704 avg_ns=26105 max_ns=2052954
  fuse_op.release: count=32 total_ns=70367 avg_ns=2198 max_ns=5047
  fuse_op.setattr: count=16 total_ns=991846 avg_ns=61990 max_ns=119463
  fuse_op.statfs: count=2 total_ns=7554 avg_ns=3777 max_ns=5249
  fuse_op.unlink: count=16 total_ns=46589807 avg_ns=2911862 max_ns=6999852
  fuse_op.write: count=32768 total_ns=1381615389 avg_ns=42163 max_ns=2260752
  policy_decision: count=360104 total_ns=195271971 avg_ns=542 max_ns=1145963
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
  matcher_candidate_order.descendant: count=294424 total_ns=47782266 avg_ns=162 max_ns=389127
  matcher_candidate_order.path: count=1014632 total_ns=224518223 avg_ns=221 max_ns=626324
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=88044608 avg_ns=299 max_ns=430653
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=57938676 avg_ns=196 max_ns=626324
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=19455885 avg_ns=296 max_ns=270754
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=47782266 avg_ns=162 max_ns=389127
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=47892265 avg_ns=162 max_ns=54238
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=11186789 avg_ns=170 max_ns=45241
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=10888386 avg_ns=83 max_ns=314812
  state_read_lock_hold: count=130791 total_ns=14190634 avg_ns=108 max_ns=148315
  state_write_lock_wait: count=211 total_ns=14441 avg_ns=68 max_ns=1230
  state_write_lock_hold: count=211 total_ns=436699 avg_ns=2069 max_ns=30403
  open_confined_openat2: count=163768 total_ns=184949640 avg_ns=1129 max_ns=1446373
  open_like.pre_open_guard.access: count=1 total_ns=64076 avg_ns=64076 max_ns=64076
  open_like.pre_open_guard.open: count=16 total_ns=415742 avg_ns=25983 max_ns=57824
  open_like.post_open_revalidation.access: count=1 total_ns=13106 avg_ns=13106 max_ns=13106
  open_like.post_open_revalidation.open: count=16 total_ns=129032 avg_ns=8064 max_ns=17093
  stat_child_no_follow: count=130919 total_ns=253660118 avg_ns=1937 max_ns=1455551
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2060257 avg_ns=15 max_ns=17309
  stat_child_no_follow.host_fstat: count=130837 total_ns=34592963 avg_ns=264 max_ns=379179
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=253660118 avg_ns=1937 max_ns=1455551
  source_root_path: count=163655 total_ns=544156103 avg_ns=3325 max_ns=2190094
  resolved_virtual_path: count=228726 total_ns=1224002426 avg_ns=5351 max_ns=2026267
  resolved_virtual_path_from_path: count=130853 total_ns=1067836615 avg_ns=8160 max_ns=2026267
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=988736442 avg_ns=7556 max_ns=2024330
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=878736282 avg_ns=2239 max_ns=2022789
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=67712801 avg_ns=172 max_ns=1977584
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=69966360 avg_ns=534 max_ns=468167
  resolved_virtual_path_from_open_fd: count=97873 total_ns=156165811 avg_ns=1595 max_ns=1524612
  read_handle_snapshot: count=32240 total_ns=11618392 avg_ns=360 max_ns=315118
  read_guard_path: count=32240 total_ns=781734672 avg_ns=24247 max_ns=2050412
  read_io: count=32240 total_ns=37802357 avg_ns=1172 max_ns=2011230
  write_handle_snapshot: count=32768 total_ns=11742002 avg_ns=358 max_ns=57955
  write_guard_mutation: count=32768 total_ns=1222474623 avg_ns=37306 max_ns=2246392
  write_io: count=32768 total_ns=135029732 avg_ns=4120 max_ns=547621
  file_sync.flush: count=16 total_ns=1431622389 avg_ns=89476399 max_ns=126174506
  read_size_bucket.0_4k: count=32236 total_ns=37795007 avg_ns=1172 max_ns=2011230
  read_size_bucket.4k_64k: count=4 total_ns=7350 avg_ns=1837 max_ns=2107
  write_size_bucket.0_4k: count=32768 total_ns=135029732 avg_ns=4120 max_ns=547621
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
