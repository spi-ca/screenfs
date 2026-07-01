# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:46.404290+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload symlink_parent_mkdir_rmdir --iterations 1 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 20 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+17 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.011494 | 0.011494 | 0.011494 | 0.011494 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62923 avg_ns=62923 max_ns=62923
  fuse_op.getattr: count=43 total_ns=459512 avg_ns=10686 max_ns=20270
  fuse_op.lookup: count=1093 total_ns=9556116 avg_ns=8743 max_ns=61944
  fuse_op.mkdir: count=40 total_ns=2441478 avg_ns=61036 max_ns=331613
  fuse_op.readlink: count=122 total_ns=2369943 avg_ns=19425 max_ns=68378
  fuse_op.rmdir: count=40 total_ns=1727207 avg_ns=43180 max_ns=78181
  fuse_op.statfs: count=2 total_ns=4133 avg_ns=2066 max_ns=3150
  policy_decision: count=2588 total_ns=1418698 avg_ns=548 max_ns=2851
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
  matcher_candidate_order.descendant: count=2348 total_ns=354717 avg_ns=151 max_ns=742
  matcher_candidate_order.path: count=7524 total_ns=1466111 avg_ns=194 max_ns=3104
  matcher_candidate_order_by_source.hidden.path: count=2348 total_ns=603761 avg_ns=257 max_ns=1795
  matcher_candidate_order_by_source.internal_hidden.path: count=2348 total_ns=392583 avg_ns=167 max_ns=3104
  matcher_candidate_order_by_source.readonly.path: count=240 total_ns=63864 avg_ns=266 max_ns=653
  matcher_candidate_order_by_source.visible.descendant: count=2348 total_ns=354717 avg_ns=151 max_ns=742
  matcher_candidate_order_by_source.visible.path: count=2348 total_ns=366608 avg_ns=156 max_ns=507
  matcher_candidate_order_by_source.writable.path: count=240 total_ns=39295 avg_ns=163 max_ns=1280
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2588
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=2588
  matcher_candidate_order_ancestor_steps: count=38444
  matcher_candidate_order_ancestor_steps.descendant: count=9091
  matcher_candidate_order_ancestor_steps.path: count=29353
  state_read_lock_wait: count=1339 total_ns=24919 avg_ns=18 max_ns=93
  state_read_lock_hold: count=1339 total_ns=88449 avg_ns=66 max_ns=952
  state_write_lock_wait: count=1051 total_ns=18766 avg_ns=17 max_ns=73
  state_write_lock_hold: count=1051 total_ns=334052 avg_ns=317 max_ns=2619
  open_confined_openat2: count=1700 total_ns=1049157 avg_ns=617 max_ns=37107
  open_like.pre_open_guard.access: count=1 total_ns=51053 avg_ns=51053 max_ns=51053
  open_like.post_open_revalidation.access: count=1 total_ns=7113 avg_ns=7113 max_ns=7113
  stat_child_no_follow: count=1619 total_ns=2492865 avg_ns=1539 max_ns=39752
  stat_child_no_follow.attr_conversion: count=1337 total_ns=18319 avg_ns=13 max_ns=44
  stat_child_no_follow.directory_revalidation: count=122 total_ns=842636 avg_ns=6906 max_ns=31994
  stat_child_no_follow.host_fstat: count=1215 total_ns=185870 avg_ns=152 max_ns=1340
  stat_child_no_follow.host_fstatat: count=122 total_ns=32003 avg_ns=262 max_ns=1206
  stat_child_no_follow.parent_open: count=122 total_ns=96980 avg_ns=794 max_ns=5704
  stat_child_no_follow_context.path_guard_or_metadata: count=1497 total_ns=1490195 avg_ns=995 max_ns=9648
  stat_child_no_follow_context.readlink_pre_open: count=122 total_ns=1002670 avg_ns=8218 max_ns=39752
  source_root_path: count=1459 total_ns=1948346 avg_ns=1335 max_ns=51995
  resolved_virtual_path: count=1702 total_ns=5434856 avg_ns=3193 max_ns=289445
  resolved_virtual_path_from_path: count=1499 total_ns=5219069 avg_ns=3481 max_ns=289445
  resolved_virtual_path_from_path_component_walk: count=1499 total_ns=4296061 avg_ns=2865 max_ns=22467
  resolved_virtual_path_from_path_canonicalize: count=3803 total_ns=3475986 avg_ns=914 max_ns=21884
  resolved_virtual_path_from_path_source_root_confinement: count=3803 total_ns=441169 avg_ns=116 max_ns=1495
  resolved_virtual_path_from_path_virtual_conversion: count=1499 total_ns=842294 avg_ns=561 max_ns=285334
  resolved_virtual_path_from_open_fd: count=203 total_ns=215787 avg_ns=1062 max_ns=6655
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
  invalidations: count=80 invalidated_entries=40 evicted_entries=0 scanned_entries=320
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
