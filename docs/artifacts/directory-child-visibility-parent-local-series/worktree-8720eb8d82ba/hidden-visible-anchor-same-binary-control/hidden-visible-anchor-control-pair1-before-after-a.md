# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:18.864033+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-before-after-a.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-before-after-a.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-before-after-a.svg`
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
| readdir_basic | 0.000940 | 0.046385 | 49.330 | 0.047983 | 0.051590 | 0.054476 |
| readdirplus_basic | 0.003362 | 0.366892 | 109.130 | 0.421531 | 0.430981 | 0.438541 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=74211 avg_ns=74211 max_ns=74211
  fuse_op.getattr: count=65027 total_ns=667922955 avg_ns=10271 max_ns=345033
  fuse_op.lookup: count=195055 total_ns=1824463306 avg_ns=9353 max_ns=279679
  fuse_op.opendir: count=26 total_ns=486543 avg_ns=18713 max_ns=54056
  fuse_op.readdir: count=180 total_ns=1243415950 avg_ns=6907866 max_ns=15720148
  fuse_op.readdirplus: count=31 total_ns=93874301 avg_ns=3028203 max_ns=4615894
  fuse_op.releasedir: count=26 total_ns=11165024 avg_ns=429424 max_ns=946319
  fuse_op.statfs: count=2 total_ns=6427 avg_ns=3213 max_ns=3300
  policy_decision: count=694047 total_ns=383951826 avg_ns=553 max_ns=191432
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=162910116 avg_ns=234 max_ns=57507
  matcher_candidate_order.path: count=2082141 total_ns=337328749 avg_ns=162 max_ns=283494
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=78961019 avg_ns=113 max_ns=251342
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=86841056 avg_ns=125 max_ns=106574
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=162910116 avg_ns=234 max_ns=57507
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=171526674 avg_ns=247 max_ns=283494
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5938423 avg_ns=22 max_ns=44222
  state_read_lock_hold: count=260320 total_ns=18856511 avg_ns=72 max_ns=75902
  state_write_lock_wait: count=195344 total_ns=4444536 avg_ns=22 max_ns=172082
  state_write_lock_hold: count=195344 total_ns=256145925 avg_ns=1311 max_ns=2195961
  open_confined_openat2: count=266227 total_ns=205508314 avg_ns=771 max_ns=261399
  open_like.pre_open_guard.access: count=1 total_ns=55354 avg_ns=55354 max_ns=55354
  open_like.pre_open_guard.opendir: count=26 total_ns=321685 avg_ns=12372 max_ns=45139
  open_like.post_open_revalidation.access: count=1 total_ns=11745 avg_ns=11745 max_ns=11745
  open_like.post_open_revalidation.opendir: count=26 total_ns=99983 avg_ns=3845 max_ns=7009
  stat_child_no_follow: count=265989 total_ns=357928763 avg_ns=1345 max_ns=262905
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4344950 avg_ns=16 max_ns=19776
  stat_child_no_follow.host_fstat: count=265989 total_ns=59726575 avg_ns=224 max_ns=68788
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=357928763 avg_ns=1345 max_ns=262905
  source_root_path: count=260372 total_ns=432879035 avg_ns=1662 max_ns=164845
  resolved_virtual_path: count=260399 total_ns=857740281 avg_ns=3293 max_ns=338639
  resolved_virtual_path_from_path: count=260161 total_ns=857146622 avg_ns=3294 max_ns=338639
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=737029052 avg_ns=2832 max_ns=338047
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=604494507 avg_ns=1032 max_ns=337281
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=61593092 avg_ns=105 max_ns=128753
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=104427604 avg_ns=401 max_ns=111495
  resolved_virtual_path_from_open_fd: count=238 total_ns=593659 avg_ns=2494 max_ns=9420
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1097230715 avg_ns=6095726 max_ns=14860160
  readdir_scan.name_child_path_materialization: count=180 total_ns=92617355 avg_ns=514540 max_ns=1277315
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=808447751 avg_ns=4491376 max_ns=11335839
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=56392181 avg_ns=313289 max_ns=957231
  readdir_page_commit: count=180 total_ns=123418285 avg_ns=685657 max_ns=2196263
  readdirplus_directory_scan: count=31 total_ns=70549693 avg_ns=2275796 max_ns=3477637
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27553754 avg_ns=888830 max_ns=1281367
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6463441 avg_ns=1109 max_ns=13613
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=10815960 avg_ns=1855 max_ns=10744
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2075119 avg_ns=66939 max_ns=128707
  readdirplus_attr_generation_scan: count=5859 total_ns=6463441 avg_ns=1103 max_ns=13613
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16103484 avg_ns=519467 max_ns=735278
  readdirplus_page_commit: count=31 total_ns=4263675 avg_ns=137537 max_ns=234072
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
