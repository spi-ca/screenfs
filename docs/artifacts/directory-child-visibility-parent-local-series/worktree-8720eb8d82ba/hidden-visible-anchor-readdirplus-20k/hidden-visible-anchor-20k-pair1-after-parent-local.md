# ScreenFS benchmark result

- timestamp: `2026-06-24T22:19:57.863500+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair1-after-parent-local.svg`
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
| readdirplus_basic | 0.017683 | 2.119913 | 119.883 | 2.181513 | 2.195395 | 2.206501 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=58167 avg_ns=58167 max_ns=58167
  fuse_op.getattr: count=260014 total_ns=2879584228 avg_ns=11074 max_ns=673004
  fuse_op.lookup: count=780029 total_ns=7881882373 avg_ns=10104 max_ns=2041902
  fuse_op.opendir: count=13 total_ns=264231 avg_ns=20325 max_ns=63673
  fuse_op.readdir: count=332 total_ns=10283948592 avg_ns=30975748 max_ns=67416000
  fuse_op.readdirplus: count=33 total_ns=355964146 avg_ns=10786792 max_ns=16737297
  fuse_op.releasedir: count=13 total_ns=29888844 avg_ns=2299141 max_ns=2903305
  fuse_op.statfs: count=2 total_ns=8134 avg_ns=4067 max_ns=5308
  policy_decision: count=4243888 total_ns=2670681347 avg_ns=629 max_ns=925881
  matcher_candidates: count=4764349
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=3983858
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=4764349
  matcher_candidate_order.descendant: count=4243888 total_ns=1160362583 avg_ns=273 max_ns=478349
  matcher_candidate_order.path: count=12731664 total_ns=2347960376 avg_ns=184 max_ns=2011219
  matcher_candidate_order_by_source.hidden.path: count=4243888 total_ns=564371396 avg_ns=132 max_ns=315788
  matcher_candidate_order_by_source.internal_hidden.path: count=4243888 total_ns=598697182 avg_ns=141 max_ns=922417
  matcher_candidate_order_by_source.visible.descendant: count=4243888 total_ns=1160362583 avg_ns=273 max_ns=478349
  matcher_candidate_order_by_source.visible.path: count=4243888 total_ns=1184891798 avg_ns=279 max_ns=2011219
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=16975552
  matcher_candidate_order_seen_slots.descendant: count=8487776
  matcher_candidate_order_seen_slots.path: count=8487776
  matcher_candidate_order_ancestor_steps: count=64780244
  matcher_candidate_order_ancestor_steps.descendant: count=16195061
  matcher_candidate_order_ancestor_steps.path: count=48585183
  state_read_lock_wait: count=1040422 total_ns=24954244 avg_ns=23 max_ns=336791
  state_read_lock_hold: count=1040422 total_ns=75894533 avg_ns=72 max_ns=110333
  state_write_lock_wait: count=780433 total_ns=17591827 avg_ns=22 max_ns=8621
  state_write_lock_hold: count=780433 total_ns=950952445 avg_ns=1218 max_ns=4000640
  open_confined_openat2: count=1046708 total_ns=877460823 avg_ns=838 max_ns=308747
  open_like.pre_open_guard.access: count=1 total_ns=48954 avg_ns=48954 max_ns=48954
  open_like.pre_open_guard.opendir: count=13 total_ns=170216 avg_ns=13093 max_ns=50773
  open_like.post_open_revalidation.access: count=1 total_ns=6220 avg_ns=6220 max_ns=6220
  open_like.post_open_revalidation.opendir: count=13 total_ns=58909 avg_ns=4531 max_ns=7213
  stat_child_no_follow: count=1046329 total_ns=1526563114 avg_ns=1458 max_ns=309356
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=18073388 avg_ns=17 max_ns=90538
  stat_child_no_follow.host_fstat: count=1046329 total_ns=252526467 avg_ns=241 max_ns=201177
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1526563114 avg_ns=1458 max_ns=309356
  source_root_path: count=1040448 total_ns=1876383350 avg_ns=1803 max_ns=2016460
  resolved_virtual_path: count=1040462 total_ns=3726285921 avg_ns=3581 max_ns=363978
  resolved_virtual_path_from_path: count=1040083 total_ns=3725146429 avg_ns=3581 max_ns=363978
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=3158565306 avg_ns=3036 max_ns=363007
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2563842613 avg_ns=1095 max_ns=362477
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=252339615 avg_ns=107 max_ns=357460
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=502435782 avg_ns=483 max_ns=164343
  resolved_virtual_path_from_open_fd: count=379 total_ns=1139492 avg_ns=3006 max_ns=45636
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9857086887 avg_ns=29690020 max_ns=66500889
  readdir_scan.name_child_path_materialization: count=332 total_ns=773400729 avg_ns=2329520 max_ns=5727100
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6986699632 avg_ns=21044276 max_ns=49719005
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=611490697 avg_ns=1841839 max_ns=4919949
  readdir_page_commit: count=332 total_ns=353356962 avg_ns=1064328 max_ns=4001415
  readdirplus_directory_scan: count=33 total_ns=312588659 avg_ns=9472383 max_ns=14917352
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=98653122 avg_ns=2989488 max_ns=5161295
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=9385298 avg_ns=1502 max_ns=47900
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=14164826 avg_ns=2267 max_ns=32119
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=7800309 avg_ns=236373 max_ns=415368
  readdirplus_attr_generation_scan: count=6279 total_ns=9385298 avg_ns=1494 max_ns=47900
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=62019463 avg_ns=1879377 max_ns=3281118
  readdirplus_page_commit: count=33 total_ns=16926079 avg_ns=512911 max_ns=1468738
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
