# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:50.941386+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-after-parent-local.svg`
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
- policy_label: `parent-local-hidden-visible-anchor`
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
| readdir_basic | 0.000770 | 0.048093 | 62.445 | 0.050559 | 0.051003 | 0.051358 |
| readdirplus_basic | 0.003456 | 0.318552 | 92.174 | 0.402465 | 0.415402 | 0.425753 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=84506 avg_ns=84506 max_ns=84506
  fuse_op.getattr: count=65027 total_ns=604707075 avg_ns=9299 max_ns=1131809
  fuse_op.lookup: count=195055 total_ns=1632187348 avg_ns=8367 max_ns=731411
  fuse_op.opendir: count=26 total_ns=553446 avg_ns=21286 max_ns=58318
  fuse_op.readdir: count=180 total_ns=1205873849 avg_ns=6699299 max_ns=15077568
  fuse_op.readdirplus: count=31 total_ns=91175443 avg_ns=2941143 max_ns=4702634
  fuse_op.releasedir: count=26 total_ns=10250800 avg_ns=394261 max_ns=748976
  fuse_op.statfs: count=2 total_ns=3268 avg_ns=1634 max_ns=2038
  policy_decision: count=694047 total_ns=357240863 avg_ns=514 max_ns=419339
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=151013389 avg_ns=217 max_ns=70965
  matcher_candidate_order.path: count=2082141 total_ns=313083464 avg_ns=150 max_ns=340880
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=73565700 avg_ns=105 max_ns=340880
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=80284292 avg_ns=115 max_ns=81822
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=151013389 avg_ns=217 max_ns=70965
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=159233472 avg_ns=229 max_ns=316356
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5202890 avg_ns=19 max_ns=20113
  state_read_lock_hold: count=260320 total_ns=16011008 avg_ns=61 max_ns=23159
  state_write_lock_wait: count=195344 total_ns=3712846 avg_ns=19 max_ns=19116
  state_write_lock_hold: count=195344 total_ns=240499460 avg_ns=1231 max_ns=4356941
  open_confined_openat2: count=266227 total_ns=178550368 avg_ns=670 max_ns=688969
  open_like.pre_open_guard.access: count=1 total_ns=73855 avg_ns=73855 max_ns=73855
  open_like.pre_open_guard.opendir: count=26 total_ns=385278 avg_ns=14818 max_ns=47797
  open_like.post_open_revalidation.access: count=1 total_ns=6824 avg_ns=6824 max_ns=6824
  open_like.post_open_revalidation.opendir: count=26 total_ns=106285 avg_ns=4087 max_ns=6446
  stat_child_no_follow: count=265989 total_ns=310972653 avg_ns=1169 max_ns=695744
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3897791 avg_ns=14 max_ns=25473
  stat_child_no_follow.host_fstat: count=265989 total_ns=49174425 avg_ns=184 max_ns=63898
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=310972653 avg_ns=1169 max_ns=695744
  source_root_path: count=260372 total_ns=386188786 avg_ns=1483 max_ns=1112143
  resolved_virtual_path: count=260399 total_ns=775802874 avg_ns=2979 max_ns=276409
  resolved_virtual_path_from_path: count=260161 total_ns=775197581 avg_ns=2979 max_ns=276409
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=667197330 avg_ns=2564 max_ns=275771
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=549304127 avg_ns=938 max_ns=274744
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=58430454 avg_ns=99 max_ns=67311
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=93694807 avg_ns=360 max_ns=160338
  resolved_virtual_path_from_open_fd: count=238 total_ns=605293 avg_ns=2543 max_ns=7822
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1060964031 avg_ns=5894244 max_ns=14128629
  readdir_scan.name_child_path_materialization: count=180 total_ns=91734130 avg_ns=509634 max_ns=1228672
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=788853406 avg_ns=4382518 max_ns=10639622
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=53932011 avg_ns=299622 max_ns=1038843
  readdir_page_commit: count=180 total_ns=123147235 avg_ns=684151 max_ns=4357393
  readdirplus_directory_scan: count=31 total_ns=68092106 avg_ns=2196519 max_ns=3760461
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27897895 avg_ns=899932 max_ns=1863897
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6018207 avg_ns=1032 max_ns=38558
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=10853557 avg_ns=1862 max_ns=23598
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1924513 avg_ns=62081 max_ns=92942
  readdirplus_attr_generation_scan: count=5859 total_ns=6018207 avg_ns=1027 max_ns=38558
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=15744445 avg_ns=507885 max_ns=730260
  readdirplus_page_commit: count=31 total_ns=4382419 avg_ns=141368 max_ns=248415
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
