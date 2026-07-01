# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:35.181898+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set directory-symlink-surface --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 200 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-directory-symlink-surface-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-directory-symlink-surface-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-directory-symlink-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `?? docs/artifacts/current-benchmark-rerun.log`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `?? docs/artifacts/current-benchmark-rerun.log`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-symlink-surface`
- comparable_workloads: `readdir_symlink_visibility, readdirplus_symlink_visibility`
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
| readdir_symlink_visibility | 0.000114 | 0.004300 | 37.734 | 0.004712 | 0.004764 | 0.004805 |
| readdirplus_symlink_visibility | 0.000370 | 0.017542 | 47.428 | 0.017979 | 0.018034 | 0.018077 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=68795 avg_ns=68795 max_ns=68795
  fuse_op.getattr: count=817 total_ns=10298603 avg_ns=12605 max_ns=22774
  fuse_op.lookup: count=2445 total_ns=24450178 avg_ns=10000 max_ns=69920
  fuse_op.opendir: count=8 total_ns=146282 avg_ns=18285 max_ns=49347
  fuse_op.readdir: count=15 total_ns=1507158 avg_ns=100477 max_ns=390115
  fuse_op.readdirplus: count=9 total_ns=32585694 avg_ns=3620632 max_ns=4622005
  fuse_op.releasedir: count=8 total_ns=194377 avg_ns=24297 max_ns=43033
  fuse_op.statfs: count=2 total_ns=15653 avg_ns=7826 max_ns=10742
  policy_decision: count=8120 total_ns=4639233 avg_ns=571 max_ns=6364
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=8120 total_ns=1304966 avg_ns=160 max_ns=11064
  matcher_candidate_order.path: count=24360 total_ns=4979034 avg_ns=204 max_ns=10472
  matcher_candidate_order_by_source.hidden.path: count=8120 total_ns=2305994 avg_ns=283 max_ns=3701
  matcher_candidate_order_by_source.internal_hidden.path: count=8120 total_ns=1373600 avg_ns=169 max_ns=10472
  matcher_candidate_order_by_source.visible.descendant: count=8120 total_ns=1304966 avg_ns=160 max_ns=11064
  matcher_candidate_order_by_source.visible.path: count=8120 total_ns=1299440 avg_ns=160 max_ns=1495
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=8120
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=8120
  matcher_candidate_order_ancestor_steps: count=119916
  matcher_candidate_order_ancestor_steps.descendant: count=29979
  matcher_candidate_order_ancestor_steps.path: count=89937
  state_read_lock_wait: count=3295 total_ns=79937 avg_ns=24 max_ns=295
  state_read_lock_hold: count=3295 total_ns=247650 avg_ns=75 max_ns=2490
  state_write_lock_wait: count=2491 total_ns=59988 avg_ns=24 max_ns=457
  state_write_lock_hold: count=2491 total_ns=2425045 avg_ns=973 max_ns=341106
  open_confined_openat2: count=4872 total_ns=3506105 avg_ns=719 max_ns=17166
  open_like.pre_open_guard.access: count=1 total_ns=56132 avg_ns=56132 max_ns=56132
  open_like.pre_open_guard.opendir: count=8 total_ns=101832 avg_ns=12729 max_ns=37818
  open_like.post_open_revalidation.access: count=1 total_ns=6786 avg_ns=6786 max_ns=6786
  open_like.post_open_revalidation.opendir: count=8 total_ns=18746 avg_ns=2343 max_ns=5384
  stat_child_no_follow: count=4839 total_ns=6315447 avg_ns=1305 max_ns=18283
  stat_child_no_follow.attr_conversion: count=4837 total_ns=84086 avg_ns=17 max_ns=35
  stat_child_no_follow.host_fstat: count=4837 total_ns=1083732 avg_ns=224 max_ns=4171
  stat_child_no_follow_context.path_guard_or_metadata: count=4839 total_ns=6315447 avg_ns=1305 max_ns=18283
  source_root_path: count=6535 total_ns=10957594 avg_ns=1676 max_ns=34601
  resolved_virtual_path: count=6542 total_ns=27219868 avg_ns=4160 max_ns=23299
  resolved_virtual_path_from_path: count=6509 total_ns=27154229 avg_ns=4171 max_ns=23299
  resolved_virtual_path_from_path_component_walk: count=6509 total_ns=24267972 avg_ns=3728 max_ns=22504
  resolved_virtual_path_from_path_canonicalize: count=17033 total_ns=20336073 avg_ns=1193 max_ns=21933
  resolved_virtual_path_from_path_source_root_confinement: count=17033 total_ns=2067161 avg_ns=121 max_ns=13182
  resolved_virtual_path_from_path_virtual_conversion: count=6509 total_ns=2497144 avg_ns=383 max_ns=6734
  resolved_virtual_path_from_open_fd: count=33 total_ns=65639 avg_ns=1989 max_ns=5766
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=15 total_ns=1155614 avg_ns=77040 max_ns=275606
  readdir_scan.name_child_path_materialization: count=15 total_ns=26835 avg_ns=1789 max_ns=4733
  readdir_scan.scan_fallback_attr: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=15 total_ns=2637 avg_ns=175 max_ns=836
  readdir_attr_generation_scan: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=15 total_ns=632493 avg_ns=42166 max_ns=118663
  readdir_candidate_selection: count=15 total_ns=11294 avg_ns=752 max_ns=2734
  readdir_page_commit: count=15 total_ns=109306 avg_ns=7287 max_ns=29279
  readdirplus_directory_scan: count=9 total_ns=15442417 avg_ns=1715824 max_ns=2112702
  readdirplus_scan.name_child_path_materialization: count=9 total_ns=469469 avg_ns=52163 max_ns=67527
  readdirplus_scan.returned_attr_hydration: count=1552 total_ns=1973413 avg_ns=1271 max_ns=5502
  readdirplus_scan.returned_policy_recheck: count=1552 total_ns=28086 avg_ns=18 max_ns=154
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=13281007 avg_ns=8557 max_ns=21453
  readdirplus_scan.scan_fallback_attr: count=9 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=9 total_ns=32760 avg_ns=3640 max_ns=4977
  readdirplus_attr_generation_scan: count=1561 total_ns=1973413 avg_ns=1264 max_ns=5502
  readdirplus_attr_generation_entries: count=1552
  readdirplus_symlink_visibility: count=9 total_ns=14117893 avg_ns=1568654 max_ns=1954369
  readdirplus_candidate_selection: count=9 total_ns=296463 avg_ns=32940 max_ns=62828
  readdirplus_page_commit: count=9 total_ns=1211022 avg_ns=134558 max_ns=341250
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
