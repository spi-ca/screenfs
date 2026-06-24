# ScreenFS benchmark result

- timestamp: `2026-06-24T22:19:29.405292+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-before-no-parent-local.svg`
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
| readdirplus_basic | 0.016352 | 1.947213 | 119.080 | 2.158875 | 2.211720 | 2.253996 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=76055 avg_ns=76055 max_ns=76055
  fuse_op.getattr: count=260014 total_ns=2516634676 avg_ns=9678 max_ns=666694
  fuse_op.lookup: count=780029 total_ns=6912539377 avg_ns=8861 max_ns=1612551
  fuse_op.opendir: count=13 total_ns=268670 avg_ns=20666 max_ns=33128
  fuse_op.readdir: count=332 total_ns=9223844157 avg_ns=27782663 max_ns=61417130
  fuse_op.readdirplus: count=33 total_ns=1167741841 avg_ns=35386116 max_ns=59405110
  fuse_op.releasedir: count=13 total_ns=27275318 avg_ns=2098101 max_ns=3208482
  fuse_op.statfs: count=2 total_ns=4429 avg_ns=2214 max_ns=2887
  policy_decision: count=4691266 total_ns=2632109429 avg_ns=561 max_ns=286954
  matcher_candidates: count=5211727
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=4431236
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=5211727
  matcher_candidate_order.descendant: count=4691266 total_ns=1114778309 avg_ns=237 max_ns=310972
  matcher_candidate_order.path: count=14073798 total_ns=2324486630 avg_ns=165 max_ns=482250
  matcher_candidate_order_by_source.hidden.path: count=4691266 total_ns=558701742 avg_ns=119 max_ns=275006
  matcher_candidate_order_by_source.internal_hidden.path: count=4691266 total_ns=597544413 avg_ns=127 max_ns=263692
  matcher_candidate_order_by_source.visible.descendant: count=4691266 total_ns=1114778309 avg_ns=237 max_ns=310972
  matcher_candidate_order_by_source.visible.path: count=4691266 total_ns=1168240475 avg_ns=249 max_ns=482250
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=18765064
  matcher_candidate_order_seen_slots.descendant: count=9382532
  matcher_candidate_order_seen_slots.path: count=9382532
  matcher_candidate_order_ancestor_steps: count=71938292
  matcher_candidate_order_ancestor_steps.descendant: count=17984573
  matcher_candidate_order_ancestor_steps.path: count=53953719
  state_read_lock_wait: count=1040422 total_ns=21704657 avg_ns=20 max_ns=12293
  state_read_lock_hold: count=1040422 total_ns=68405274 avg_ns=65 max_ns=438560
  state_write_lock_wait: count=780433 total_ns=15655699 avg_ns=20 max_ns=20483
  state_write_lock_hold: count=780433 total_ns=847464407 avg_ns=1085 max_ns=4031598
  open_confined_openat2: count=1046708 total_ns=740974277 avg_ns=707 max_ns=180951
  open_like.pre_open_guard.access: count=1 total_ns=59442 avg_ns=59442 max_ns=59442
  open_like.pre_open_guard.opendir: count=13 total_ns=179175 avg_ns=13782 max_ns=26337
  open_like.post_open_revalidation.access: count=1 total_ns=11916 avg_ns=11916 max_ns=11916
  open_like.post_open_revalidation.opendir: count=13 total_ns=57328 avg_ns=4409 max_ns=5741
  stat_child_no_follow: count=1046329 total_ns=1295941933 avg_ns=1238 max_ns=314524
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=16140653 avg_ns=15 max_ns=48143
  stat_child_no_follow.host_fstat: count=1046329 total_ns=208805247 avg_ns=199 max_ns=111427
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1295941933 avg_ns=1238 max_ns=314524
  source_root_path: count=1040448 total_ns=1581018547 avg_ns=1519 max_ns=356989
  resolved_virtual_path: count=1040462 total_ns=3296481880 avg_ns=3168 max_ns=421620
  resolved_virtual_path_from_path: count=1040083 total_ns=3295418773 avg_ns=3168 max_ns=421620
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=2803139648 avg_ns=2695 max_ns=385629
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2279933401 avg_ns=974 max_ns=383440
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=238979945 avg_ns=102 max_ns=152814
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=434682813 avg_ns=417 max_ns=416747
  resolved_virtual_path_from_open_fd: count=379 total_ns=1063107 avg_ns=2805 max_ns=23437
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=8848250493 avg_ns=26651356 max_ns=60367968
  readdir_scan.name_child_path_materialization: count=332 total_ns=691485864 avg_ns=2082788 max_ns=4868418
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6366477261 avg_ns=19176136 max_ns=45759542
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=519619627 avg_ns=1565119 max_ns=4709657
  readdir_page_commit: count=332 total_ns=317565702 avg_ns=956523 max_ns=4031830
  readdirplus_directory_scan: count=33 total_ns=1131182102 avg_ns=34278245 max_ns=58443044
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=89279178 avg_ns=2705429 max_ns=4687695
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=7665024 avg_ns=1227 max_ns=26245
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=12457591 avg_ns=1994 max_ns=73194
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=852282989 avg_ns=25826757 max_ns=44997762
  readdirplus_attr_generation_scan: count=6279 total_ns=7665024 avg_ns=1220 max_ns=26245
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=58563969 avg_ns=1774665 max_ns=3474190
  readdirplus_page_commit: count=33 total_ns=14263926 avg_ns=432240 max_ns=1256493
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
