# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:43.569270+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --cache-control warm --workload metadata_lookup --workload metadata_getattr --workload metadata_access --workload matcher_hidden_stat_miss --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 200 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 200 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+8 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
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
| metadata_lookup | 0.001362 | 0.011546 | 8.480 | 0.012106 | 0.012176 | 0.012232 |
| metadata_getattr | 0.001276 | 0.014747 | 11.558 | 0.014873 | 0.014889 | 0.014902 |
| metadata_access | 0.001074 | 0.015789 | 14.703 | 0.015854 | 0.015862 | 0.015868 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009505 | 0.009786 | 0.009821 | 0.009849 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=16818553 avg_ns=20996 max_ns=80567
  fuse_op.getattr: count=801 total_ns=10767029 avg_ns=13441 max_ns=37460
  fuse_op.lookup: count=9605 total_ns=98917232 avg_ns=10298 max_ns=73768
  fuse_op.statfs: count=2 total_ns=5136 avg_ns=2568 max_ns=3776
  policy_decision: count=12008 total_ns=16471196 avg_ns=1371 max_ns=7709
  matcher_candidates: count=128896
  matcher_candidates_by_source.hidden.path: count=800
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=128096
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=128896
  matcher_candidate_order.descendant: count=12008 total_ns=7629455 avg_ns=635 max_ns=20744
  matcher_candidate_order.path: count=36024 total_ns=17116378 avg_ns=475 max_ns=18536
  matcher_candidate_order_by_source.hidden.path: count=12008 total_ns=7995586 avg_ns=665 max_ns=4852
  matcher_candidate_order_by_source.internal_hidden.path: count=12008 total_ns=1866840 avg_ns=155 max_ns=18536
  matcher_candidate_order_by_source.visible.descendant: count=12008 total_ns=7629455 avg_ns=635 max_ns=20744
  matcher_candidate_order_by_source.visible.path: count=12008 total_ns=7253952 avg_ns=604 max_ns=4686
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1176784
  matcher_candidate_order_seen_slots.descendant: count=384256
  matcher_candidate_order_seen_slots.path: count=792528
  matcher_candidate_order_ancestor_steps: count=153676
  matcher_candidate_order_ancestor_steps.descendant: count=38419
  matcher_candidate_order_ancestor_steps.path: count=115257
  state_read_lock_wait: count=11207 total_ns=269453 avg_ns=24 max_ns=5626
  state_read_lock_hold: count=11207 total_ns=799718 avg_ns=71 max_ns=1314
  state_write_lock_wait: count=8003 total_ns=183016 avg_ns=22 max_ns=456
  state_write_lock_hold: count=8003 total_ns=2608365 avg_ns=325 max_ns=11040
  open_confined_openat2: count=12008 total_ns=8710305 avg_ns=725 max_ns=18540
  open_like.pre_open_guard.access: count=801 total_ns=10487029 avg_ns=13092 max_ns=59686
  open_like.post_open_revalidation.access: count=801 total_ns=5253127 avg_ns=6558 max_ns=11994
  stat_child_no_follow: count=11207 total_ns=14470912 avg_ns=1291 max_ns=18953
  stat_child_no_follow.attr_conversion: count=10405 total_ns=180287 avg_ns=17 max_ns=28
  stat_child_no_follow.host_fstat: count=10405 total_ns=2227450 avg_ns=214 max_ns=7733
  stat_child_no_follow_context.path_guard_or_metadata: count=11207 total_ns=14470912 avg_ns=1291 max_ns=18953
  source_root_path: count=10407 total_ns=18694619 avg_ns=1796 max_ns=34293
  resolved_virtual_path: count=10406 total_ns=28556506 avg_ns=2744 max_ns=12266
  resolved_virtual_path_from_path: count=9605 total_ns=27642937 avg_ns=2877 max_ns=12266
  resolved_virtual_path_from_path_component_walk: count=9605 total_ns=23764225 avg_ns=2474 max_ns=11740
  resolved_virtual_path_from_path_canonicalize: count=19209 total_ns=19335948 avg_ns=1006 max_ns=10717
  resolved_virtual_path_from_path_source_root_confinement: count=19209 total_ns=2305585 avg_ns=120 max_ns=1481
  resolved_virtual_path_from_path_virtual_conversion: count=9605 total_ns=3295449 avg_ns=343 max_ns=4359
  resolved_virtual_path_from_open_fd: count=801 total_ns=913569 avg_ns=1140 max_ns=5919
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
