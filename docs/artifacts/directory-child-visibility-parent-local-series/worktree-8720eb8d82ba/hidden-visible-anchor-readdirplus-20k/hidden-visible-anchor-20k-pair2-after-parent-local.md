# ScreenFS benchmark result

- timestamp: `2026-06-24T22:20:24.770256+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 20000 --workload readdirplus_basic --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-20k --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-pair2-after-parent-local.svg`
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
| readdirplus_basic | 0.032022 | 1.913781 | 59.765 | 2.073829 | 2.088188 | 2.099675 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=60845 avg_ns=60845 max_ns=60845
  fuse_op.getattr: count=260014 total_ns=2606220496 avg_ns=10023 max_ns=1976317
  fuse_op.lookup: count=780029 total_ns=7167259505 avg_ns=9188 max_ns=1515379
  fuse_op.opendir: count=13 total_ns=305510 avg_ns=23500 max_ns=49176
  fuse_op.readdir: count=332 total_ns=9506273163 avg_ns=28633352 max_ns=71860862
  fuse_op.readdirplus: count=33 total_ns=342507984 avg_ns=10379029 max_ns=14931141
  fuse_op.releasedir: count=13 total_ns=24704655 avg_ns=1900358 max_ns=3144628
  fuse_op.statfs: count=2 total_ns=4227 avg_ns=2113 max_ns=2212
  policy_decision: count=4243888 total_ns=2446414746 avg_ns=576 max_ns=997194
  matcher_candidates: count=4764349
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=780491
  matcher_candidates_by_source.visible.path: count=3983858
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=4764349
  matcher_candidate_order.descendant: count=4243888 total_ns=1053626120 avg_ns=248 max_ns=325624
  matcher_candidate_order.path: count=12731664 total_ns=2138198877 avg_ns=167 max_ns=337663
  matcher_candidate_order_by_source.hidden.path: count=4243888 total_ns=512781936 avg_ns=120 max_ns=317128
  matcher_candidate_order_by_source.internal_hidden.path: count=4243888 total_ns=550925586 avg_ns=129 max_ns=337663
  matcher_candidate_order_by_source.visible.descendant: count=4243888 total_ns=1053626120 avg_ns=248 max_ns=325624
  matcher_candidate_order_by_source.visible.path: count=4243888 total_ns=1074491355 avg_ns=253 max_ns=303866
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=16975552
  matcher_candidate_order_seen_slots.descendant: count=8487776
  matcher_candidate_order_seen_slots.path: count=8487776
  matcher_candidate_order_ancestor_steps: count=64780244
  matcher_candidate_order_ancestor_steps.descendant: count=16195061
  matcher_candidate_order_ancestor_steps.path: count=48585183
  state_read_lock_wait: count=1040422 total_ns=22470151 avg_ns=21 max_ns=76594
  state_read_lock_hold: count=1040422 total_ns=72388469 avg_ns=69 max_ns=116407
  state_write_lock_wait: count=780433 total_ns=16269798 avg_ns=20 max_ns=68694
  state_write_lock_hold: count=780433 total_ns=880525819 avg_ns=1128 max_ns=4625004
  open_confined_openat2: count=1046708 total_ns=767146733 avg_ns=732 max_ns=261249
  open_like.pre_open_guard.access: count=1 total_ns=49292 avg_ns=49292 max_ns=49292
  open_like.pre_open_guard.opendir: count=13 total_ns=209700 avg_ns=16130 max_ns=37435
  open_like.post_open_revalidation.access: count=1 total_ns=6846 avg_ns=6846 max_ns=6846
  open_like.post_open_revalidation.opendir: count=13 total_ns=59267 avg_ns=4559 max_ns=6706
  stat_child_no_follow: count=1046329 total_ns=1353388894 avg_ns=1293 max_ns=261702
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=16528160 avg_ns=15 max_ns=43116
  stat_child_no_follow.host_fstat: count=1046329 total_ns=223509337 avg_ns=213 max_ns=107502
  stat_child_no_follow_context.path_guard_or_metadata: count=1046329 total_ns=1353388894 avg_ns=1293 max_ns=261702
  source_root_path: count=1040448 total_ns=1674340407 avg_ns=1609 max_ns=1956301
  resolved_virtual_path: count=1040462 total_ns=3394198859 avg_ns=3262 max_ns=1493819
  resolved_virtual_path_from_path: count=1040083 total_ns=3393040383 avg_ns=3262 max_ns=1493819
  resolved_virtual_path_from_path_component_walk: count=1040083 total_ns=2877268147 avg_ns=2766 max_ns=1491082
  resolved_virtual_path_from_path_canonicalize: count=2340139 total_ns=2341315376 avg_ns=1000 max_ns=1488850
  resolved_virtual_path_from_path_source_root_confinement: count=2340139 total_ns=237675870 avg_ns=101 max_ns=346151
  resolved_virtual_path_from_path_virtual_conversion: count=1040083 total_ns=455861135 avg_ns=438 max_ns=172099
  resolved_virtual_path_from_open_fd: count=379 total_ns=1158476 avg_ns=3056 max_ns=38766
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9108696965 avg_ns=27435834 max_ns=67214619
  readdir_scan.name_child_path_materialization: count=332 total_ns=715507580 avg_ns=2155143 max_ns=5505745
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6484097878 avg_ns=19530415 max_ns=52526683
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=541441385 avg_ns=1630847 max_ns=4355392
  readdir_page_commit: count=332 total_ns=333397009 avg_ns=1004207 max_ns=4625276
  readdirplus_directory_scan: count=33 total_ns=301173476 avg_ns=9126468 max_ns=13649495
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=91597940 avg_ns=2775695 max_ns=4716444
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=8790273 avg_ns=1407 max_ns=16404
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=13460759 avg_ns=2155 max_ns=66469
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=7430693 avg_ns=225172 max_ns=426802
  readdirplus_attr_generation_scan: count=6279 total_ns=8790273 avg_ns=1399 max_ns=16404
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=58712888 avg_ns=1779178 max_ns=3098006
  readdirplus_page_commit: count=33 total_ns=16299011 avg_ns=493909 max_ns=1361614
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
