# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:44.601154+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --cache-control warm --workload metadata_opendir --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 200 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-metadata-opendir-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-metadata-opendir-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-metadata-opendir-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+11 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
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
| metadata_opendir | 0.000500 | 0.012013 | 24.006 | 0.012393 | 0.012441 | 0.012479 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=43825 avg_ns=43825 max_ns=43825
  fuse_op.getattr: count=801 total_ns=6565355 avg_ns=8196 max_ns=43173
  fuse_op.lookup: count=1605 total_ns=12931079 avg_ns=8056 max_ns=65751
  fuse_op.opendir: count=800 total_ns=8524526 avg_ns=10655 max_ns=35787
  fuse_op.releasedir: count=800 total_ns=218738 avg_ns=273 max_ns=3329
  fuse_op.statfs: count=2 total_ns=7311 avg_ns=3655 max_ns=4719
  policy_decision: count=3208 total_ns=1365508 avg_ns=425 max_ns=18199
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3208 total_ns=366037 avg_ns=114 max_ns=562
  matcher_candidate_order.path: count=9624 total_ns=1547992 avg_ns=160 max_ns=4795
  matcher_candidate_order_by_source.hidden.path: count=3208 total_ns=757895 avg_ns=236 max_ns=832
  matcher_candidate_order_by_source.internal_hidden.path: count=3208 total_ns=421382 avg_ns=131 max_ns=4795
  matcher_candidate_order_by_source.visible.descendant: count=3208 total_ns=366037 avg_ns=114 max_ns=562
  matcher_candidate_order_by_source.visible.path: count=3208 total_ns=368715 avg_ns=114 max_ns=931
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3208
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3208
  matcher_candidate_order_ancestor_steps: count=35276
  matcher_candidate_order_ancestor_steps.descendant: count=8819
  matcher_candidate_order_ancestor_steps.path: count=26457
  state_read_lock_wait: count=3207 total_ns=79121 avg_ns=24 max_ns=330
  state_read_lock_hold: count=3207 total_ns=234495 avg_ns=73 max_ns=1088
  state_write_lock_wait: count=3203 total_ns=75142 avg_ns=23 max_ns=271
  state_write_lock_hold: count=3203 total_ns=519141 avg_ns=162 max_ns=25343
  open_confined_openat2: count=4008 total_ns=2831676 avg_ns=706 max_ns=19670
  open_like.pre_open_guard.access: count=1 total_ns=35415 avg_ns=35415 max_ns=35415
  open_like.pre_open_guard.opendir: count=800 total_ns=6441831 avg_ns=8052 max_ns=30725
  open_like.post_open_revalidation.access: count=1 total_ns=4217 avg_ns=4217 max_ns=4217
  open_like.post_open_revalidation.opendir: count=800 total_ns=1034653 avg_ns=1293 max_ns=7556
  stat_child_no_follow: count=3207 total_ns=4358139 avg_ns=1358 max_ns=26358
  stat_child_no_follow.attr_conversion: count=3205 total_ns=55230 avg_ns=17 max_ns=20
  stat_child_no_follow.host_fstat: count=3205 total_ns=673828 avg_ns=210 max_ns=7450
  stat_child_no_follow_context.path_guard_or_metadata: count=3207 total_ns=4358139 avg_ns=1358 max_ns=26358
  source_root_path: count=3207 total_ns=5863390 avg_ns=1828 max_ns=20546
  resolved_virtual_path: count=4006 total_ns=9027540 avg_ns=2253 max_ns=34997
  resolved_virtual_path_from_path: count=3205 total_ns=8091923 avg_ns=2524 max_ns=34997
  resolved_virtual_path_from_path_component_walk: count=3205 total_ns=6741531 avg_ns=2103 max_ns=34220
  resolved_virtual_path_from_path_canonicalize: count=5609 total_ns=5371510 avg_ns=957 max_ns=33674
  resolved_virtual_path_from_path_source_root_confinement: count=5609 total_ns=682261 avg_ns=121 max_ns=935
  resolved_virtual_path_from_path_virtual_conversion: count=3205 total_ns=1155862 avg_ns=360 max_ns=2290
  resolved_virtual_path_from_open_fd: count=801 total_ns=935617 avg_ns=1168 max_ns=7171
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
