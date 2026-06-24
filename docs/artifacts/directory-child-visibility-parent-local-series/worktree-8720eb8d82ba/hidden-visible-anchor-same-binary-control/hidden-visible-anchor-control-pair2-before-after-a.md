# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:37.103401+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-before-after-a.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-before-after-a.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-before-after-a.svg`
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
- policy_label: `parent-local-hidden-visible-anchor-control`
- fast_path_cache_eligible: `False`
- policy_notes: `extra_screenfs_arg included policy-shaping flags (--visibility-default, --visible, --visible); treat the run as a named custom unsafe matrix entry unless independently reviewed`
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
| readdir_basic | 0.000728 | 0.051566 | 70.790 | 0.055913 | 0.057948 | 0.059577 |
| readdirplus_basic | 0.004097 | 0.287403 | 70.151 | 0.304913 | 0.357741 | 0.400004 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=133478 avg_ns=133478 max_ns=133478
  fuse_op.getattr: count=65027 total_ns=561657098 avg_ns=8637 max_ns=653316
  fuse_op.lookup: count=195055 total_ns=1510203284 avg_ns=7742 max_ns=1509717
  fuse_op.opendir: count=26 total_ns=448301 avg_ns=17242 max_ns=46893
  fuse_op.readdir: count=180 total_ns=1221886320 avg_ns=6788257 max_ns=15183766
  fuse_op.readdirplus: count=31 total_ns=94257361 avg_ns=3040560 max_ns=4298469
  fuse_op.releasedir: count=26 total_ns=10318210 avg_ns=396854 max_ns=906914
  fuse_op.statfs: count=2 total_ns=6657 avg_ns=3328 max_ns=3767
  policy_decision: count=694047 total_ns=344057063 avg_ns=495 max_ns=356337
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=146721488 avg_ns=211 max_ns=131523
  matcher_candidate_order.path: count=2082141 total_ns=303697851 avg_ns=145 max_ns=355470
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=71306263 avg_ns=102 max_ns=329553
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=77256615 avg_ns=111 max_ns=275221
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=146721488 avg_ns=211 max_ns=131523
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=155134973 avg_ns=223 max_ns=355470
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=4925957 avg_ns=18 max_ns=17078
  state_read_lock_hold: count=260320 total_ns=15000419 avg_ns=57 max_ns=48405
  state_write_lock_wait: count=195344 total_ns=3493137 avg_ns=17 max_ns=26096
  state_write_lock_hold: count=195344 total_ns=232747136 avg_ns=1191 max_ns=2710263
  open_confined_openat2: count=266227 total_ns=162065777 avg_ns=608 max_ns=331151
  open_like.pre_open_guard.access: count=1 total_ns=105872 avg_ns=105872 max_ns=105872
  open_like.pre_open_guard.opendir: count=26 total_ns=298539 avg_ns=11482 max_ns=37013
  open_like.post_open_revalidation.access: count=1 total_ns=20803 avg_ns=20803 max_ns=20803
  open_like.post_open_revalidation.opendir: count=26 total_ns=95016 avg_ns=3654 max_ns=6869
  stat_child_no_follow: count=265989 total_ns=282627806 avg_ns=1062 max_ns=331770
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3691855 avg_ns=13 max_ns=15678
  stat_child_no_follow.host_fstat: count=265989 total_ns=42855344 avg_ns=161 max_ns=287191
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=282627806 avg_ns=1062 max_ns=331770
  source_root_path: count=260372 total_ns=341992532 avg_ns=1313 max_ns=169632
  resolved_virtual_path: count=260399 total_ns=723701472 avg_ns=2779 max_ns=400378
  resolved_virtual_path_from_path: count=260161 total_ns=723102942 avg_ns=2779 max_ns=400378
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=622406925 avg_ns=2392 max_ns=399923
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=512936918 avg_ns=876 max_ns=399559
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=55247171 avg_ns=94 max_ns=62805
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=86919320 avg_ns=334 max_ns=188711
  resolved_virtual_path_from_open_fd: count=238 total_ns=598530 avg_ns=2514 max_ns=18386
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1079160812 avg_ns=5995337 max_ns=14262736
  readdir_scan.name_child_path_materialization: count=180 total_ns=93110661 avg_ns=517281 max_ns=1281545
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=791998373 avg_ns=4399990 max_ns=10632176
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=56601670 avg_ns=314453 max_ns=1037813
  readdir_page_commit: count=180 total_ns=121718607 avg_ns=676214 max_ns=2710465
  readdirplus_directory_scan: count=31 total_ns=69865541 avg_ns=2253727 max_ns=3255910
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27360106 avg_ns=882584 max_ns=1176119
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6616313 avg_ns=1135 max_ns=100750
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=11325059 avg_ns=1943 max_ns=32943
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1964054 avg_ns=63356 max_ns=92334
  readdirplus_attr_generation_scan: count=5859 total_ns=6616313 avg_ns=1129 max_ns=100750
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16291033 avg_ns=525517 max_ns=721067
  readdirplus_page_commit: count=31 total_ns=4597877 avg_ns=148318 max_ns=273057
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
