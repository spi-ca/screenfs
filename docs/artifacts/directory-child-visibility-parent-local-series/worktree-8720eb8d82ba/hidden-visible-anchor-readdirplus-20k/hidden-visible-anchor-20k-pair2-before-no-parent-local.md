# ScreenFS benchmark result

- timestamp: `2026-06-24T22:20:53.303672+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-before-no-parent-local.svg`
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
| readdirplus_basic | 0.027007 | 2.077706 | 76.932 | 2.162586 | 2.169910 | 2.175769 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=60066 avg_ns=60066 max_ns=60066
  fuse_op.getattr: count=260014 total_ns=2709192156 avg_ns=10419 max_ns=295348
  fuse_op.lookup: count=780029 total_ns=7471363137 avg_ns=9578 max_ns=328001
  fuse_op.opendir: count=13 total_ns=231456 avg_ns=17804 max_ns=34925
  fuse_op.readdir: count=332 total_ns=9936489158 avg_ns=29929184 max_ns=62588425
  fuse_op.readdirplus: count=33 total_ns=1329552846 avg_ns=40289480 max_ns=69472173
  fuse_op.releasedir: count=13 total_ns=32023686 avg_ns=2463360 max_ns=3591391
  fuse_op.statfs: count=2 total_ns=4755 avg_ns=2377 max_ns=2900
  policy_decision: count=4691266 total_ns=2818246639 avg_ns=600 max_ns=233205
  matcher_candidates: count=5211727
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=4431236
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=5211727
  matcher_candidate_order.descendant: count=4691266 total_ns=1190537485 avg_ns=253 max_ns=467831
  matcher_candidate_order.path: count=14073798 total_ns=2500958505 avg_ns=177 max_ns=486738
  matcher_candidate_order_by_source.hidden.path: count=4691266 total_ns=608298425 avg_ns=129 max_ns=180846
  matcher_candidate_order_by_source.internal_hidden.path: count=4691266 total_ns=648078624 avg_ns=138 max_ns=162883
  matcher_candidate_order_by_source.visible.descendant: count=4691266 total_ns=1190537485 avg_ns=253 max_ns=467831
  matcher_candidate_order_by_source.visible.path: count=4691266 total_ns=1244581456 avg_ns=265 max_ns=486738
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=18765064
  matcher_candidate_order_seen_slots.descendant: count=9382532
  matcher_candidate_order_seen_slots.path: count=9382532
  matcher_candidate_order_ancestor_steps: count=71938292
  matcher_candidate_order_ancestor_steps.descendant: count=17984573
  matcher_candidate_order_ancestor_steps.path: count=53953719
  state_read_lock_wait: count=1040422 total_ns=23417659 avg_ns=22 max_ns=140688
  state_read_lock_hold: count=1040422 total_ns=72753511 avg_ns=69 max_ns=111949
  state_write_lock_wait: count=780433 total_ns=16839259 avg_ns=21 max_ns=51304
  state_write_lock_hold: count=780433 total_ns=916264739 avg_ns=1174 max_ns=3590724
  open_confined_openat2: count=1046708 total_ns=815421894 avg_ns=779 max_ns=261337
  open_like.pre_open_guard.access: count=1 total_ns=48712 avg_ns=48712 max_ns=48712
  open_like.pre_open_guard.opendir: count=13 total_ns=137322 avg_ns=10563 max_ns=25051
  open_like.post_open_revalidation.access: count=1 total_ns=7293 avg_ns=7293 max_ns=7293
  open_like.post_open_revalidation.opendir: count=13 total_ns=61508 avg_ns=4731 max_ns=8043
  stat_child_no_follow: count=1046329 total_ns=1436972507 avg_ns=1373 max_ns=261847
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=17108295 avg_ns=16 max_ns=14748
  stat_child_no_follow.host_fstat: count=1046329 total_ns=242235785 avg_ns=231 max_ns=82820
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1436972507 avg_ns=1373 max_ns=261847
  source_root_path: count=1040448 total_ns=1787026046 avg_ns=1717 max_ns=163791
  resolved_virtual_path: count=1040462 total_ns=3546576925 avg_ns=3408 max_ns=321949
  resolved_virtual_path_from_path: count=1040083 total_ns=3545411209 avg_ns=3408 max_ns=321949
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=3005240714 avg_ns=2889 max_ns=321374
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2428640516 avg_ns=1037 max_ns=320897
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=247278236 avg_ns=105 max_ns=317097
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=479859040 avg_ns=461 max_ns=306876
  resolved_virtual_path_from_open_fd: count=379 total_ns=1165716 avg_ns=3075 max_ns=28745
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9530531006 avg_ns=28706418 max_ns=60332102
  readdir_scan.name_child_path_materialization: count=332 total_ns=730723036 avg_ns=2200973 max_ns=4821931
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6660524459 avg_ns=20061820 max_ns=44341798
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=581838836 avg_ns=1752526 max_ns=4867624
  readdir_page_commit: count=332 total_ns=333612819 avg_ns=1004857 max_ns=3057523
  readdirplus_directory_scan: count=33 total_ns=1287032421 avg_ns=39000982 max_ns=68485705
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=99180667 avg_ns=3005474 max_ns=5169492
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=10083052 avg_ns=1614 max_ns=26920
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=13447194 avg_ns=2152 max_ns=18349
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=942691392 avg_ns=28566405 max_ns=50498622
  readdirplus_attr_generation_scan: count=6279 total_ns=10083052 avg_ns=1605 max_ns=26920
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=72586608 avg_ns=2199594 max_ns=4412109
  readdirplus_page_commit: count=33 total_ns=16180620 avg_ns=490321 max_ns=1420545
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
