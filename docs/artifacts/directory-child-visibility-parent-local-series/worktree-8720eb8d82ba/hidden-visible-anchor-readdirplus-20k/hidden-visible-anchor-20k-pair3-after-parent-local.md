# ScreenFS benchmark result

- timestamp: `2026-06-24T22:21:48.275119+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-after-parent-local.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+42 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- screenfs_bin_sha256: `84b20a30b24c0da8ff470158a100e6124373c07d54ce8d584558e17e17a99605`
- screenfs_source_root: `/tmp/screenfs-parent-local-8720eb8d82ba/after`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `unknown`
- screenfs_source_git_worktree_clean: `unknown`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `parent-local-hidden-visible-anchor-20k`
- fast_path_cache_eligible: `False`
- policy_notes: `extra_screenfs_arg included policy-shaping flags (--visibility-default, --visible, --visible); treat the run as a named custom unsafe matrix entry unless independently reviewed`
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
| readdirplus_basic | 0.027392 | 1.840307 | 67.185 | 2.002744 | 2.031894 | 2.055214 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61179 avg_ns=61179 max_ns=61179
  fuse_op.getattr: count=260014 total_ns=2506375975 avg_ns=9639 max_ns=2263889
  fuse_op.lookup: count=780029 total_ns=6857053552 avg_ns=8790 max_ns=1945240
  fuse_op.opendir: count=13 total_ns=254863 avg_ns=19604 max_ns=38130
  fuse_op.readdir: count=332 total_ns=9344153241 avg_ns=28145039 max_ns=68888685
  fuse_op.readdirplus: count=33 total_ns=324819397 avg_ns=9843012 max_ns=14517790
  fuse_op.releasedir: count=13 total_ns=24940386 avg_ns=1918491 max_ns=3156794
  fuse_op.statfs: count=2 total_ns=6619 avg_ns=3309 max_ns=3687
  policy_decision: count=4243888 total_ns=2373242517 avg_ns=559 max_ns=739042
  matcher_candidates: count=4764349
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=3983858
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=4764349
  matcher_candidate_order.descendant: count=4243888 total_ns=1026486727 avg_ns=241 max_ns=292707
  matcher_candidate_order.path: count=12731664 total_ns=2088755134 avg_ns=164 max_ns=355374
  matcher_candidate_order_by_source.hidden.path: count=4243888 total_ns=500547171 avg_ns=117 max_ns=244788
  matcher_candidate_order_by_source.internal_hidden.path: count=4243888 total_ns=532306583 avg_ns=125 max_ns=316805
  matcher_candidate_order_by_source.visible.descendant: count=4243888 total_ns=1026486727 avg_ns=241 max_ns=292707
  matcher_candidate_order_by_source.visible.path: count=4243888 total_ns=1055901380 avg_ns=248 max_ns=355374
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=16975552
  matcher_candidate_order_seen_slots.descendant: count=8487776
  matcher_candidate_order_seen_slots.path: count=8487776
  matcher_candidate_order_ancestor_steps: count=64780244
  matcher_candidate_order_ancestor_steps.descendant: count=16195061
  matcher_candidate_order_ancestor_steps.path: count=48585183
  state_read_lock_wait: count=1040422 total_ns=21697291 avg_ns=20 max_ns=28469
  state_read_lock_hold: count=1040422 total_ns=69952406 avg_ns=67 max_ns=164680
  state_write_lock_wait: count=780433 total_ns=15613021 avg_ns=20 max_ns=25224
  state_write_lock_hold: count=780433 total_ns=846352416 avg_ns=1084 max_ns=3156302
  open_confined_openat2: count=1046708 total_ns=747175762 avg_ns=713 max_ns=640983
  open_like.pre_open_guard.access: count=1 total_ns=53628 avg_ns=53628 max_ns=53628
  open_like.pre_open_guard.opendir: count=13 total_ns=167517 avg_ns=12885 max_ns=27310
  open_like.post_open_revalidation.access: count=1 total_ns=4373 avg_ns=4373 max_ns=4373
  open_like.post_open_revalidation.opendir: count=13 total_ns=56310 avg_ns=4331 max_ns=6384
  stat_child_no_follow: count=1046329 total_ns=1298899080 avg_ns=1241 max_ns=642734
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=15993339 avg_ns=15 max_ns=43284
  stat_child_no_follow.host_fstat: count=1046329 total_ns=209257543 avg_ns=199 max_ns=77622
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1298899080 avg_ns=1241 max_ns=642734
  source_root_path: count=1040448 total_ns=1575178495 avg_ns=1513 max_ns=1558166
  resolved_virtual_path: count=1040462 total_ns=3261484058 avg_ns=3134 max_ns=966987
  resolved_virtual_path_from_path: count=1040083 total_ns=3260375079 avg_ns=3134 max_ns=966987
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=2771133924 avg_ns=2664 max_ns=322462
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2252295274 avg_ns=962 max_ns=264648
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=232559369 avg_ns=99 max_ns=207473
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=430766756 avg_ns=414 max_ns=962416
  resolved_virtual_path_from_open_fd: count=379 total_ns=1108979 avg_ns=2926 max_ns=24282
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=8963019825 avg_ns=26997047 max_ns=65890608
  readdir_scan.name_child_path_materialization: count=332 total_ns=706681513 avg_ns=2128558 max_ns=4796380
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6362521004 avg_ns=19164219 max_ns=49672100
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=532394448 avg_ns=1603597 max_ns=4697551
  readdir_page_commit: count=332 total_ns=319482022 avg_ns=962295 max_ns=3092330
  readdirplus_directory_scan: count=33 total_ns=286835184 avg_ns=8691975 max_ns=13505080
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=90613216 avg_ns=2745855 max_ns=4659210
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=8091074 avg_ns=1295 max_ns=82582
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=12876728 avg_ns=2061 max_ns=43843
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=7239939 avg_ns=219392 max_ns=392677
  readdirplus_attr_generation_scan: count=6279 total_ns=8091074 avg_ns=1288 max_ns=82582
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=56028265 avg_ns=1697826 max_ns=2891272
  readdirplus_page_commit: count=33 total_ns=14783907 avg_ns=447997 max_ns=1170774
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
