# ScreenFS benchmark result

- timestamp: `2026-07-01T05:16:33.579230+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload readdirplus_basic --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 20000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+106 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+106 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
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
| readdirplus_basic | 0.017209 | 1.469284 | 85.378 | 1.487062 | 1.512055 | 1.532051 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=71930 avg_ns=71930 max_ns=71930
  fuse_op.getattr: count=260014 total_ns=2631005461 avg_ns=10118 max_ns=266688
  fuse_op.lookup: count=780031 total_ns=7230586748 avg_ns=9269 max_ns=633846
  fuse_op.opendir: count=13 total_ns=233299 avg_ns=17946 max_ns=27813
  fuse_op.readdir: count=332 total_ns=3304165574 avg_ns=9952305 max_ns=15442600
  fuse_op.readdirplus: count=33 total_ns=297060900 avg_ns=9001845 max_ns=14038558
  fuse_op.releasedir: count=13 total_ns=27509938 avg_ns=2116149 max_ns=3014715
  fuse_op.statfs: count=2 total_ns=7548 avg_ns=3774 max_ns=5199
  policy_decision: count=1040086 total_ns=518609671 avg_ns=498 max_ns=47763
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1040086 total_ns=146157783 avg_ns=140 max_ns=21349
  matcher_candidate_order.path: count=3120258 total_ns=577229861 avg_ns=184 max_ns=49757
  matcher_candidate_order_by_source.hidden.path: count=1040086 total_ns=271354527 avg_ns=260 max_ns=49757
  matcher_candidate_order_by_source.internal_hidden.path: count=1040086 total_ns=157033671 avg_ns=150 max_ns=48467
  matcher_candidate_order_by_source.visible.descendant: count=1040086 total_ns=146157783 avg_ns=140 max_ns=21349
  matcher_candidate_order_by_source.visible.path: count=1040086 total_ns=148841663 avg_ns=143 max_ns=44974
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1040086
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1040086
  matcher_candidate_order_ancestor_steps: count=13520908
  matcher_candidate_order_ancestor_steps.descendant: count=3380227
  matcher_candidate_order_ancestor_steps.path: count=10140681
  state_read_lock_wait: count=1040424 total_ns=24292463 avg_ns=23 max_ns=5322
  state_read_lock_hold: count=1040424 total_ns=81475623 avg_ns=78 max_ns=11135
  state_write_lock_wait: count=780433 total_ns=17223049 avg_ns=22 max_ns=7415
  state_write_lock_hold: count=780433 total_ns=912424945 avg_ns=1169 max_ns=3014089
  open_confined_openat2: count=1046710 total_ns=848584979 avg_ns=810 max_ns=119827
  open_like.pre_open_guard.access: count=1 total_ns=55137 avg_ns=55137 max_ns=55137
  open_like.pre_open_guard.opendir: count=13 total_ns=161205 avg_ns=12400 max_ns=22172
  open_like.post_open_revalidation.access: count=1 total_ns=11758 avg_ns=11758 max_ns=11758
  open_like.post_open_revalidation.opendir: count=13 total_ns=36495 avg_ns=2807 max_ns=5020
  stat_child_no_follow: count=1046331 total_ns=1461830108 avg_ns=1397 max_ns=120696
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=17538642 avg_ns=16 max_ns=5765
  stat_child_no_follow.host_fstat: count=1046329 total_ns=243100192 avg_ns=232 max_ns=54409
  stat_child_no_follow_context.path_guard_or_metadata: count=1046331 total_ns=1461830108 avg_ns=1397 max_ns=120696
  source_root_path: count=1040450 total_ns=1709534742 avg_ns=1643 max_ns=86523
  resolved_virtual_path: count=1040462 total_ns=3435604460 avg_ns=3301 max_ns=255014
  resolved_virtual_path_from_path: count=1040083 total_ns=3434565054 avg_ns=3302 max_ns=255014
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=2940785900 avg_ns=2827 max_ns=254510
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2370542142 avg_ns=1012 max_ns=139350
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=272607322 avg_ns=116 max_ns=183166
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=433059419 avg_ns=416 max_ns=47163
  resolved_virtual_path_from_open_fd: count=379 total_ns=1039406 avg_ns=2742 max_ns=9387
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=2918250947 avg_ns=8789912 max_ns=14568507
  readdir_scan.name_child_path_materialization: count=332 total_ns=721204294 avg_ns=2172302 max_ns=4420960
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=56225574 avg_ns=169354 max_ns=375868
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=493500329 avg_ns=1486446 max_ns=3105381
  readdir_page_commit: count=332 total_ns=333171382 avg_ns=1003528 max_ns=2512221
  readdirplus_directory_scan: count=33 total_ns=275107957 avg_ns=8336604 max_ns=13503475
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=87247205 avg_ns=2643854 max_ns=4403744
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=7133421 avg_ns=1142 max_ns=25998
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=93008 avg_ns=14 max_ns=182
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=6851579 avg_ns=207623 max_ns=374056
  readdirplus_attr_generation_scan: count=6279 total_ns=7133421 avg_ns=1136 max_ns=25998
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=56808636 avg_ns=1721473 max_ns=2735303
  readdirplus_page_commit: count=33 total_ns=12953719 avg_ns=392536 max_ns=1003306
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
