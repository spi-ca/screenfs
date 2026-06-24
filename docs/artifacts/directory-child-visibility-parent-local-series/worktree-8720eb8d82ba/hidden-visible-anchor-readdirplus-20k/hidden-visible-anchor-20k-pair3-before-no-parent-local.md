# ScreenFS benchmark result

- timestamp: `2026-06-24T22:21:22.698179+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair3-before-no-parent-local.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+42 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local`
- screenfs_bin_sha256: `ff65e7e4d5b9d9506096ae5b33185ee9b108263a272aed1a592f13dd87847511`
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
| readdirplus_basic | 0.017235 | 2.153692 | 124.963 | 2.267994 | 2.366914 | 2.446049 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=110885 avg_ns=110885 max_ns=110885
  fuse_op.getattr: count=260014 total_ns=2844265195 avg_ns=10938 max_ns=362197
  fuse_op.lookup: count=780029 total_ns=7862672219 avg_ns=10079 max_ns=327390
  fuse_op.opendir: count=13 total_ns=290935 avg_ns=22379 max_ns=35072
  fuse_op.readdir: count=332 total_ns=10281645239 avg_ns=30968810 max_ns=69274240
  fuse_op.readdirplus: count=33 total_ns=1340940709 avg_ns=40634566 max_ns=63540217
  fuse_op.releasedir: count=13 total_ns=36391334 avg_ns=2799333 max_ns=3654078
  fuse_op.statfs: count=2 total_ns=7861 avg_ns=3930 max_ns=5702
  policy_decision: count=4691266 total_ns=2938881795 avg_ns=626 max_ns=329601
  matcher_candidates: count=5211727
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=4431236
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=5211727
  matcher_candidate_order.descendant: count=4691266 total_ns=1239303855 avg_ns=264 max_ns=386577
  matcher_candidate_order.path: count=14073798 total_ns=2602045018 avg_ns=184 max_ns=402720
  matcher_candidate_order_by_source.hidden.path: count=4691266 total_ns=634480014 avg_ns=135 max_ns=402720
  matcher_candidate_order_by_source.internal_hidden.path: count=4691266 total_ns=678913219 avg_ns=144 max_ns=317435
  matcher_candidate_order_by_source.visible.descendant: count=4691266 total_ns=1239303855 avg_ns=264 max_ns=386577
  matcher_candidate_order_by_source.visible.path: count=4691266 total_ns=1288651785 avg_ns=274 max_ns=322060
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=18765064
  matcher_candidate_order_seen_slots.descendant: count=9382532
  matcher_candidate_order_seen_slots.path: count=9382532
  matcher_candidate_order_ancestor_steps: count=71938292
  matcher_candidate_order_ancestor_steps.descendant: count=17984573
  matcher_candidate_order_ancestor_steps.path: count=53953719
  state_read_lock_wait: count=1040422 total_ns=23994826 avg_ns=23 max_ns=80654
  state_read_lock_hold: count=1040422 total_ns=75315281 avg_ns=72 max_ns=180946
  state_write_lock_wait: count=780433 total_ns=17436029 avg_ns=22 max_ns=14906
  state_write_lock_hold: count=780433 total_ns=949672310 avg_ns=1216 max_ns=3653534
  open_confined_openat2: count=1046708 total_ns=905473735 avg_ns=865 max_ns=354001
  open_like.pre_open_guard.access: count=1 total_ns=81462 avg_ns=81462 max_ns=81462
  open_like.pre_open_guard.opendir: count=13 total_ns=182126 avg_ns=14009 max_ns=28144
  open_like.post_open_revalidation.access: count=1 total_ns=14458 avg_ns=14458 max_ns=14458
  open_like.post_open_revalidation.opendir: count=13 total_ns=68988 avg_ns=5306 max_ns=13176
  stat_child_no_follow: count=1046329 total_ns=1562019422 avg_ns=1492 max_ns=354396
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=17734784 avg_ns=16 max_ns=79665
  stat_child_no_follow.host_fstat: count=1046329 total_ns=259176842 avg_ns=247 max_ns=170136
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1562019422 avg_ns=1492 max_ns=354396
  source_root_path: count=1040448 total_ns=1867638468 avg_ns=1795 max_ns=289167
  resolved_virtual_path: count=1040462 total_ns=3724160254 avg_ns=3579 max_ns=309282
  resolved_virtual_path_from_path: count=1040083 total_ns=3722971980 avg_ns=3579 max_ns=309282
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=3106561047 avg_ns=2986 max_ns=305063
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2495031968 avg_ns=1066 max_ns=303687
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=253021659 avg_ns=108 max_ns=131044
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=553487964 avg_ns=532 max_ns=280404
  resolved_virtual_path_from_open_fd: count=379 total_ns=1188274 avg_ns=3135 max_ns=68722
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9863177788 avg_ns=29708366 max_ns=67814351
  readdir_scan.name_child_path_materialization: count=332 total_ns=759170628 avg_ns=2286658 max_ns=5701151
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6897703801 avg_ns=20776216 max_ns=49250825
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=646021755 avg_ns=1945848 max_ns=6222980
  readdir_page_commit: count=332 total_ns=342612903 avg_ns=1031966 max_ns=2989870
  readdirplus_directory_scan: count=33 total_ns=1298865088 avg_ns=39359548 max_ns=62529093
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=103192309 avg_ns=3127039 max_ns=6091269
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=9422662 avg_ns=1508 max_ns=18777
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=13618136 avg_ns=2180 max_ns=93041
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=953661822 avg_ns=28898843 max_ns=45997973
  readdirplus_attr_generation_scan: count=6279 total_ns=9422662 avg_ns=1500 max_ns=18777
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=76829485 avg_ns=2328166 max_ns=6152032
  readdirplus_page_commit: count=33 total_ns=16180257 avg_ns=490310 max_ns=1435823
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
