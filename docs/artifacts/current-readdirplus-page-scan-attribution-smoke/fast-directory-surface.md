# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:15.850174+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --cache-control warm --workload-set directory-surface --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+39 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+39 more)`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001591 | 0.013471 | 8.468 | 0.013691 | 0.013718 | 0.013740 |
| readdirplus_basic | 0.003543 | 0.156110 | 44.066 | 0.157232 | 0.157372 | 0.157484 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=43216 avg_ns=43216 max_ns=43216
  fuse_op.getattr: count=20009 total_ns=58320979 avg_ns=2914 max_ns=54918
  fuse_op.lookup: count=60021 total_ns=203779215 avg_ns=3395 max_ns=54806
  fuse_op.opendir: count=8 total_ns=107852 avg_ns=13481 max_ns=38962
  fuse_op.readdir: count=54 total_ns=94718150 avg_ns=1754039 max_ns=3212664
  fuse_op.readdirplus: count=13 total_ns=18204873 avg_ns=1400374 max_ns=1708903
  fuse_op.releasedir: count=8 total_ns=2784170 avg_ns=348021 max_ns=507588
  fuse_op.statfs: count=2 total_ns=3844 avg_ns=1922 max_ns=2430
  policy_decision: count=80047 total_ns=27165652 avg_ns=339 max_ns=10089
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=80047 total_ns=9441986 avg_ns=117 max_ns=48560
  matcher_candidate_order.path: count=240141 total_ns=31964327 avg_ns=133 max_ns=52442
  matcher_candidate_order_by_source.hidden.path: count=80047 total_ns=10665358 avg_ns=133 max_ns=52442
  matcher_candidate_order_by_source.internal_hidden.path: count=80047 total_ns=10890565 avg_ns=136 max_ns=11845
  matcher_candidate_order_by_source.visible.descendant: count=80047 total_ns=9441986 avg_ns=117 max_ns=48560
  matcher_candidate_order_by_source.visible.path: count=80047 total_ns=10408404 avg_ns=130 max_ns=16553
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1040488
  matcher_candidate_order_ancestor_steps.descendant: count=260122
  matcher_candidate_order_ancestor_steps.path: count=780366
  state_read_lock_wait: count=80106 total_ns=1437519 avg_ns=17 max_ns=6656
  state_read_lock_hold: count=80106 total_ns=4495224 avg_ns=56 max_ns=12447
  state_write_lock_wait: count=60110 total_ns=1032971 avg_ns=17 max_ns=283
  state_write_lock_hold: count=60110 total_ns=68872055 avg_ns=1145 max_ns=2265934
  open_confined_openat2: count=82458 total_ns=40528749 avg_ns=491 max_ns=17080
  open_like.pre_open_guard.access: count=1 total_ns=298 avg_ns=298 max_ns=298
  open_like.pre_open_guard.opendir: count=8 total_ns=2256 avg_ns=282 max_ns=629
  open_like.post_open_revalidation.access: count=1 total_ns=33052 avg_ns=33052 max_ns=33052
  open_like.post_open_revalidation.opendir: count=8 total_ns=80581 avg_ns=10072 max_ns=28494
  stat_child_no_follow: count=82382 total_ns=74818849 avg_ns=908 max_ns=18306
  stat_child_no_follow.attr_conversion: count=82380 total_ns=1094761 avg_ns=13 max_ns=104
  stat_child_no_follow.host_fstat: count=82380 total_ns=12014135 avg_ns=145 max_ns=17597
  stat_child_no_follow_context.path_guard_or_metadata: count=82382 total_ns=74818849 avg_ns=908 max_ns=18306
  source_root_path: count=76 total_ns=528416 avg_ns=6952 max_ns=32282
  resolved_virtual_path: count=76 total_ns=152923 avg_ns=2012 max_ns=7198
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=76 total_ns=152923 avg_ns=2012 max_ns=7198
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=45580730 avg_ns=844087 max_ns=1335190
  readdir_scan.name_child_path_materialization: count=54 total_ns=2743793 avg_ns=50810 max_ns=104843
  readdir_scan.returned_child_path_materialization: count=37715 total_ns=6377765 avg_ns=169 max_ns=21941
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=1640335 avg_ns=30376 max_ns=67004
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=13248334 avg_ns=245339 max_ns=583163
  readdir_page_commit: count=54 total_ns=38039733 avg_ns=704439 max_ns=2266112
  readdirplus_directory_scan: count=13 total_ns=13076821 avg_ns=1005909 max_ns=1306088
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=746242 avg_ns=57403 max_ns=79800
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=2180599 avg_ns=933 max_ns=4722
  readdirplus_scan.returned_child_path_materialization: count=2336 total_ns=478364 avg_ns=204 max_ns=1720
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=30245 avg_ns=12 max_ns=40
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=610868 avg_ns=46989 max_ns=67483
  readdirplus_attr_generation_scan: count=2349 total_ns=2180599 avg_ns=928 max_ns=4722
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=4480739 avg_ns=344672 max_ns=478501
  readdirplus_page_commit: count=13 total_ns=1770413 avg_ns=136185 max_ns=230813
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
