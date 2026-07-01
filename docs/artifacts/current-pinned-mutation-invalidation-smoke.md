# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:09.254646+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload pinned_symlink_parent_mkdir_rmdir --iterations 1 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 20 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-pinned-mutation-invalidation-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-pinned-mutation-invalidation-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-pinned-mutation-invalidation-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+27 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+27 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `pinned_symlink_parent_mkdir_rmdir`
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
| pinned_symlink_parent_mkdir_rmdir | 0.020879 | 0.020879 | 0.020879 | 0.020879 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=50718 avg_ns=50718 max_ns=50718
  fuse_op.getattr: count=127 total_ns=1458162 avg_ns=11481 max_ns=20545
  fuse_op.lookup: count=1511 total_ns=15626736 avg_ns=10341 max_ns=136781
  fuse_op.mkdir: count=40 total_ns=2551643 avg_ns=63791 max_ns=83485
  fuse_op.opendir: count=42 total_ns=577696 avg_ns=13754 max_ns=62888
  fuse_op.readdirplus: count=42 total_ns=1370337 avg_ns=32627 max_ns=89603
  fuse_op.readlink: count=246 total_ns=5301648 avg_ns=21551 max_ns=57366
  fuse_op.releasedir: count=42 total_ns=24220 avg_ns=576 max_ns=2111
  fuse_op.rmdir: count=40 total_ns=2012958 avg_ns=50323 max_ns=77218
  fuse_op.statfs: count=2 total_ns=5911 avg_ns=2955 max_ns=3485
  policy_decision: count=3836 total_ns=2420271 avg_ns=630 max_ns=10105
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
  matcher_candidate_order.descendant: count=3596 total_ns=598916 avg_ns=166 max_ns=1469
  matcher_candidate_order.path: count=11268 total_ns=2467915 avg_ns=219 max_ns=6868
  matcher_candidate_order_by_source.hidden.path: count=3596 total_ns=1084558 avg_ns=301 max_ns=2789
  matcher_candidate_order_by_source.internal_hidden.path: count=3596 total_ns=650441 avg_ns=180 max_ns=6868
  matcher_candidate_order_by_source.readonly.path: count=240 total_ns=79066 avg_ns=329 max_ns=1151
  matcher_candidate_order_by_source.visible.descendant: count=3596 total_ns=598916 avg_ns=166 max_ns=1469
  matcher_candidate_order_by_source.visible.path: count=3596 total_ns=609791 avg_ns=169 max_ns=440
  matcher_candidate_order_by_source.writable.path: count=240 total_ns=44059 avg_ns=183 max_ns=1238
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3836
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3836
  matcher_candidate_order_ancestor_steps: count=57076
  matcher_candidate_order_ancestor_steps.descendant: count=13749
  matcher_candidate_order_ancestor_steps.path: count=43327
  state_read_lock_wait: count=2049 total_ns=46127 avg_ns=22 max_ns=277
  state_read_lock_hold: count=2049 total_ns=140455 avg_ns=68 max_ns=2113
  state_write_lock_wait: count=1637 total_ns=35936 avg_ns=21 max_ns=203
  state_write_lock_hold: count=1637 total_ns=630496 avg_ns=385 max_ns=7954
  open_confined_openat2: count=2618 total_ns=1967437 avg_ns=751 max_ns=26825
  open_like.pre_open_guard.access: count=1 total_ns=42158 avg_ns=42158 max_ns=42158
  open_like.pre_open_guard.opendir: count=42 total_ns=461572 avg_ns=10989 max_ns=60556
  open_like.post_open_revalidation.access: count=1 total_ns=4993 avg_ns=4993 max_ns=4993
  open_like.post_open_revalidation.opendir: count=42 total_ns=57819 avg_ns=1376 max_ns=2708
  stat_child_no_follow: count=2453 total_ns=5134531 avg_ns=2093 max_ns=53516
  stat_child_no_follow.attr_conversion: count=2171 total_ns=35518 avg_ns=16 max_ns=30
  stat_child_no_follow.directory_revalidation: count=246 total_ns=1893744 avg_ns=7698 max_ns=23760
  stat_child_no_follow.host_fstat: count=1925 total_ns=361397 avg_ns=187 max_ns=4642
  stat_child_no_follow.host_fstatat: count=246 total_ns=88974 avg_ns=361 max_ns=3268
  stat_child_no_follow.parent_open: count=246 total_ns=235394 avg_ns=956 max_ns=3451
  stat_child_no_follow_context.path_guard_or_metadata: count=2207 total_ns=2856690 avg_ns=1294 max_ns=53516
  stat_child_no_follow_context.readlink_pre_open: count=246 total_ns=2277841 avg_ns=9259 max_ns=31398
  source_root_path: count=2253 total_ns=3792132 avg_ns=1683 max_ns=45359
  resolved_virtual_path: count=2786 total_ns=9546063 avg_ns=3426 max_ns=26486
  resolved_virtual_path_from_path: count=2375 total_ns=9071692 avg_ns=3819 max_ns=26486
  resolved_virtual_path_from_path_component_walk: count=2375 total_ns=7962384 avg_ns=3352 max_ns=26028
  resolved_virtual_path_from_path_canonicalize: count=6097 total_ns=6535935 avg_ns=1071 max_ns=25196
  resolved_virtual_path_from_path_source_root_confinement: count=6097 total_ns=734731 avg_ns=120 max_ns=1756
  resolved_virtual_path_from_path_virtual_conversion: count=2375 total_ns=971494 avg_ns=409 max_ns=10438
  resolved_virtual_path_from_open_fd: count=411 total_ns=474371 avg_ns=1154 max_ns=3870
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
  readdirplus_directory_scan: count=42 total_ns=198069 avg_ns=4715 max_ns=25322
  readdirplus_scan.name_child_path_materialization: count=42 total_ns=35292 avg_ns=840 max_ns=4050
  readdirplus_scan.returned_attr_hydration: count=82 total_ns=97611 avg_ns=1190 max_ns=2910
  readdirplus_scan.returned_policy_recheck: count=82 total_ns=2533 avg_ns=30 max_ns=476
  readdirplus_scan.scan_fallback_attr: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=42 total_ns=2889 avg_ns=68 max_ns=899
  readdirplus_attr_generation_scan: count=124 total_ns=97611 avg_ns=787 max_ns=2910
  readdirplus_attr_generation_entries: count=82
  readdirplus_symlink_visibility: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=42 total_ns=21990 avg_ns=523 max_ns=4077
  readdirplus_page_commit: count=42 total_ns=67346 avg_ns=1603 max_ns=5695
  invalidations: count=80 invalidated_entries=40 evicted_entries=0 scanned_entries=360
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
