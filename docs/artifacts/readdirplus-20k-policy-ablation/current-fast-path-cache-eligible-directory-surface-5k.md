# ScreenFS benchmark result

- timestamp: `2026-07-01T05:16:36.696692+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+109 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+109 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.001189 | 0.014054 | 11.823 | 0.014383 | 0.014464 | 0.014529 |
| readdirplus_basic | 0.003635 | 0.155473 | 42.766 | 0.157185 | 0.157412 | 0.157594 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=64569 avg_ns=64569 max_ns=64569
  fuse_op.getattr: count=65027 total_ns=183854486 avg_ns=2827 max_ns=273647
  fuse_op.lookup: count=195057 total_ns=648750035 avg_ns=3325 max_ns=286733
  fuse_op.opendir: count=26 total_ns=417761 avg_ns=16067 max_ns=52676
  fuse_op.readdir: count=180 total_ns=297910046 avg_ns=1655055 max_ns=3313159
  fuse_op.readdirplus: count=31 total_ns=47651751 avg_ns=1537153 max_ns=1896726
  fuse_op.releasedir: count=26 total_ns=7945461 avg_ns=305594 max_ns=446016
  fuse_op.statfs: count=2 total_ns=4569 avg_ns=2284 max_ns=2525
  policy_decision: count=260137 total_ns=86201944 avg_ns=331 max_ns=261309
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=260137 total_ns=29751363 avg_ns=114 max_ns=48927
  matcher_candidate_order.path: count=780411 total_ns=101380505 avg_ns=129 max_ns=258724
  matcher_candidate_order_by_source.hidden.path: count=260137 total_ns=33844520 avg_ns=130 max_ns=258724
  matcher_candidate_order_by_source.internal_hidden.path: count=260137 total_ns=34559228 avg_ns=132 max_ns=50747
  matcher_candidate_order_by_source.visible.descendant: count=260137 total_ns=29751363 avg_ns=114 max_ns=48927
  matcher_candidate_order_by_source.visible.path: count=260137 total_ns=32976757 avg_ns=126 max_ns=23598
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=3381424
  matcher_candidate_order_ancestor_steps.descendant: count=845356
  matcher_candidate_order_ancestor_steps.path: count=2536068
  state_read_lock_wait: count=260322 total_ns=4558076 avg_ns=17 max_ns=14153
  state_read_lock_hold: count=260322 total_ns=12816967 avg_ns=49 max_ns=50832
  state_write_lock_wait: count=195344 total_ns=3294658 avg_ns=16 max_ns=15339
  state_write_lock_hold: count=195344 total_ns=204862566 avg_ns=1048 max_ns=2229326
  open_confined_openat2: count=266202 total_ns=129664139 avg_ns=487 max_ns=275765
  open_like.pre_open_guard.access: count=1 total_ns=310 avg_ns=310 max_ns=310
  open_like.pre_open_guard.opendir: count=26 total_ns=5335 avg_ns=205 max_ns=542
  open_like.post_open_revalidation.access: count=1 total_ns=47462 avg_ns=47462 max_ns=47462
  open_like.post_open_revalidation.opendir: count=26 total_ns=322111 avg_ns=12388 max_ns=43187
  stat_child_no_follow: count=265964 total_ns=238040343 avg_ns=895 max_ns=276314
  stat_child_no_follow.attr_conversion: count=265962 total_ns=3445150 avg_ns=12 max_ns=1158
  stat_child_no_follow.host_fstat: count=265962 total_ns=37348110 avg_ns=140 max_ns=63049
  stat_child_no_follow_context.path_guard_or_metadata: count=265964 total_ns=238040343 avg_ns=895 max_ns=276314
  source_root_path: count=238 total_ns=1531222 avg_ns=6433 max_ns=39282
  resolved_virtual_path: count=238 total_ns=546157 avg_ns=2294 max_ns=13555
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=238 total_ns=546157 avg_ns=2294 max_ns=13555
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=154104624 avg_ns=856136 max_ns=1639774
  readdir_scan.name_child_path_materialization: count=180 total_ns=9212861 avg_ns=51182 max_ns=348336
  readdir_scan.returned_child_path_materialization: count=124331 total_ns=20506986 avg_ns=164 max_ns=47298
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=5370020 avg_ns=29833 max_ns=64638
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=41866385 avg_ns=232591 max_ns=547630
  readdir_page_commit: count=180 total_ns=109672687 avg_ns=609292 max_ns=2229552
  readdirplus_directory_scan: count=31 total_ns=35314568 avg_ns=1139179 max_ns=1478033
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=2083988 avg_ns=67225 max_ns=83350
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=5487655 avg_ns=941 max_ns=48574
  readdirplus_scan.returned_child_path_materialization: count=5828 total_ns=1188457 avg_ns=203 max_ns=2193
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=74632 avg_ns=12 max_ns=38
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1729488 avg_ns=55789 max_ns=69885
  readdirplus_attr_generation_scan: count=5859 total_ns=5487655 avg_ns=936 max_ns=48574
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=12574458 avg_ns=405627 max_ns=566773
  readdirplus_page_commit: count=31 total_ns=4121943 avg_ns=132965 max_ns=310906
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
