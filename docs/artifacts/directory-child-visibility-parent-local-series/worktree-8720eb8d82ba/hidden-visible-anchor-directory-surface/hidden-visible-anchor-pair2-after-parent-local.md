# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:33.324222+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-after-parent-local.svg`
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
| readdir_basic | 0.001411 | 0.046254 | 32.775 | 0.050374 | 0.050914 | 0.051345 |
| readdirplus_basic | 0.007735 | 0.396655 | 51.282 | 0.514494 | 0.519889 | 0.524206 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=72062 avg_ns=72062 max_ns=72062
  fuse_op.getattr: count=65027 total_ns=739061135 avg_ns=11365 max_ns=423070
  fuse_op.lookup: count=195055 total_ns=2021302213 avg_ns=10362 max_ns=889135
  fuse_op.opendir: count=26 total_ns=524898 avg_ns=20188 max_ns=35671
  fuse_op.readdir: count=180 total_ns=1338935687 avg_ns=7438531 max_ns=17842056
  fuse_op.readdirplus: count=31 total_ns=108152616 avg_ns=3488794 max_ns=5829903
  fuse_op.releasedir: count=26 total_ns=14076549 avg_ns=541405 max_ns=3000754
  fuse_op.statfs: count=2 total_ns=5108 avg_ns=2554 max_ns=3475
  policy_decision: count=694047 total_ns=413546603 avg_ns=595 max_ns=912651
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=176267506 avg_ns=253 max_ns=353371
  matcher_candidate_order.path: count=2082141 total_ns=361754791 avg_ns=173 max_ns=356728
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=83034657 avg_ns=119 max_ns=356728
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=95984731 avg_ns=138 max_ns=250494
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=176267506 avg_ns=253 max_ns=353371
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=182735403 avg_ns=263 max_ns=351658
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=6272930 avg_ns=24 max_ns=92177
  state_read_lock_hold: count=260320 total_ns=20100327 avg_ns=77 max_ns=85279
  state_write_lock_wait: count=195344 total_ns=4412942 avg_ns=22 max_ns=21415
  state_write_lock_hold: count=195344 total_ns=279665646 avg_ns=1431 max_ns=3249273
  open_confined_openat2: count=266227 total_ns=226098186 avg_ns=849 max_ns=1293705
  open_like.pre_open_guard.access: count=1 total_ns=57461 avg_ns=57461 max_ns=57461
  open_like.pre_open_guard.opendir: count=26 total_ns=335751 avg_ns=12913 max_ns=26028
  open_like.post_open_revalidation.access: count=1 total_ns=11231 avg_ns=11231 max_ns=11231
  open_like.post_open_revalidation.opendir: count=26 total_ns=103062 avg_ns=3963 max_ns=6471
  stat_child_no_follow: count=265989 total_ns=395501465 avg_ns=1486 max_ns=184371
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4520083 avg_ns=16 max_ns=28995
  stat_child_no_follow.host_fstat: count=265989 total_ns=64789078 avg_ns=243 max_ns=123630
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=395501465 avg_ns=1486 max_ns=184371
  source_root_path: count=260372 total_ns=494701994 avg_ns=1899 max_ns=634067
  resolved_virtual_path: count=260399 total_ns=933110328 avg_ns=3583 max_ns=879694
  resolved_virtual_path_from_path: count=260161 total_ns=932463247 avg_ns=3584 max_ns=879694
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=791723992 avg_ns=3043 max_ns=877899
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=644877663 avg_ns=1101 max_ns=876557
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=66893464 avg_ns=114 max_ns=225202
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=123685884 avg_ns=475 max_ns=248929
  resolved_virtual_path_from_open_fd: count=238 total_ns=647081 avg_ns=2718 max_ns=23868
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1180304985 avg_ns=6557249 max_ns=16821199
  readdir_scan.name_child_path_materialization: count=180 total_ns=100920341 avg_ns=560668 max_ns=1783450
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=855575679 avg_ns=4753198 max_ns=12316755
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=65406680 avg_ns=363370 max_ns=1211186
  readdir_page_commit: count=180 total_ns=132371100 avg_ns=735395 max_ns=3249883
  readdirplus_directory_scan: count=31 total_ns=79808737 avg_ns=2574475 max_ns=4451661
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=29043877 avg_ns=936899 max_ns=1510658
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=8038512 avg_ns=1379 max_ns=125298
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=13043227 avg_ns=2238 max_ns=72958
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2252435 avg_ns=72659 max_ns=172050
  readdirplus_attr_generation_scan: count=5859 total_ns=8038512 avg_ns=1371 max_ns=125298
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=18232475 avg_ns=588144 max_ns=1189053
  readdirplus_page_commit: count=31 total_ns=5062415 avg_ns=163303 max_ns=316974
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
