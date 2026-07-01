# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:47.363444+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --cache-control warm --workload-set open-confined-surface --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 200 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-open-confined-surface-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-open-confined-surface-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-open-confined-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+20 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
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
| metadata_open | 0.000880 | 0.011364 | 12.910 | 0.011546 | 0.011569 | 0.011587 |
| metadata_opendir | 0.000355 | 0.009076 | 25.530 | 0.009161 | 0.009172 | 0.009180 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=42847 avg_ns=42847 max_ns=42847
  fuse_op.getattr: count=801 total_ns=5258902 avg_ns=6565 max_ns=72904
  fuse_op.lookup: count=4005 total_ns=60747857 avg_ns=15168 max_ns=16718293
  fuse_op.open: count=800 total_ns=10430248 avg_ns=13037 max_ns=85766
  fuse_op.opendir: count=800 total_ns=6537048 avg_ns=8171 max_ns=26355
  fuse_op.release: count=800 total_ns=369567 avg_ns=461 max_ns=1943
  fuse_op.releasedir: count=800 total_ns=162901 avg_ns=203 max_ns=1351
  fuse_op.statfs: count=2 total_ns=2042 avg_ns=1021 max_ns=1228
  policy_decision: count=7208 total_ns=2911010 avg_ns=403 max_ns=3499
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=7208 total_ns=841822 avg_ns=116 max_ns=14783
  matcher_candidate_order.path: count=21624 total_ns=3388114 avg_ns=156 max_ns=48510
  matcher_candidate_order_by_source.hidden.path: count=7208 total_ns=1512463 avg_ns=209 max_ns=48510
  matcher_candidate_order_by_source.internal_hidden.path: count=7208 total_ns=993241 avg_ns=137 max_ns=31499
  matcher_candidate_order_by_source.visible.descendant: count=7208 total_ns=841822 avg_ns=116 max_ns=14783
  matcher_candidate_order_by_source.visible.path: count=7208 total_ns=882410 avg_ns=122 max_ns=15311
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7208
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=7208
  matcher_candidate_order_ancestor_steps: count=89676
  matcher_candidate_order_ancestor_steps.descendant: count=22419
  matcher_candidate_order_ancestor_steps.path: count=67257
  state_read_lock_wait: count=6407 total_ns=117771 avg_ns=18 max_ns=134
  state_read_lock_hold: count=6407 total_ns=390126 avg_ns=60 max_ns=3324
  state_write_lock_wait: count=7203 total_ns=127652 avg_ns=17 max_ns=136
  state_write_lock_hold: count=7203 total_ns=1668077 avg_ns=231 max_ns=19893
  open_confined_openat2: count=8008 total_ns=35960765 avg_ns=4490 max_ns=16701162
  open_like.pre_open_guard.access: count=1 total_ns=35468 avg_ns=35468 max_ns=35468
  open_like.pre_open_guard.open: count=800 total_ns=6806488 avg_ns=8508 max_ns=70354
  open_like.pre_open_guard.opendir: count=800 total_ns=4953305 avg_ns=6191 max_ns=22451
  open_like.post_open_revalidation.access: count=1 total_ns=4224 avg_ns=4224 max_ns=4224
  open_like.post_open_revalidation.open: count=800 total_ns=2777731 avg_ns=3472 max_ns=35682
  open_like.post_open_revalidation.opendir: count=800 total_ns=772254 avg_ns=965 max_ns=3584
  stat_child_no_follow: count=6407 total_ns=38154081 avg_ns=5955 max_ns=16703659
  stat_child_no_follow.attr_conversion: count=6405 total_ns=87933 avg_ns=13 max_ns=75
  stat_child_no_follow.host_fstat: count=6405 total_ns=915301 avg_ns=142 max_ns=2041
  stat_child_no_follow_context.path_guard_or_metadata: count=6407 total_ns=38154081 avg_ns=5955 max_ns=16703659
  source_root_path: count=6407 total_ns=8185342 avg_ns=1277 max_ns=67599
  resolved_virtual_path: count=8006 total_ns=17622777 avg_ns=2201 max_ns=65614
  resolved_virtual_path_from_path: count=6405 total_ns=15811575 avg_ns=2468 max_ns=65614
  resolved_virtual_path_from_path_component_walk: count=6405 total_ns=13386298 avg_ns=2089 max_ns=65081
  resolved_virtual_path_from_path_canonicalize: count=12809 total_ns=10713304 avg_ns=836 max_ns=64382
  resolved_virtual_path_from_path_source_root_confinement: count=12809 total_ns=1473264 avg_ns=115 max_ns=27900
  resolved_virtual_path_from_path_virtual_conversion: count=6405 total_ns=2111485 avg_ns=329 max_ns=20666
  resolved_virtual_path_from_open_fd: count=1601 total_ns=1811202 avg_ns=1131 max_ns=9661
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
