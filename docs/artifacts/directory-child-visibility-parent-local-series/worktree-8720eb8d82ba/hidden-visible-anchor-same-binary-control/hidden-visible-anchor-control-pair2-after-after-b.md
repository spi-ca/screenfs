# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:31.496784+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-after-after-b.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-after-after-b.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-after-after-b.svg`
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
| readdir_basic | 0.001153 | 0.067339 | 58.382 | 0.069006 | 0.069016 | 0.069023 |
| readdirplus_basic | 0.006609 | 0.331659 | 50.182 | 0.367491 | 0.377522 | 0.385546 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=77326 avg_ns=77326 max_ns=77326
  fuse_op.getattr: count=65027 total_ns=640052613 avg_ns=9842 max_ns=329060
  fuse_op.lookup: count=195055 total_ns=1743653696 avg_ns=8939 max_ns=1938081
  fuse_op.opendir: count=26 total_ns=493318 avg_ns=18973 max_ns=43239
  fuse_op.readdir: count=180 total_ns=1460886338 avg_ns=8116035 max_ns=17010381
  fuse_op.readdirplus: count=31 total_ns=115030158 avg_ns=3710650 max_ns=4939210
  fuse_op.releasedir: count=26 total_ns=15846235 avg_ns=609470 max_ns=1436515
  fuse_op.statfs: count=2 total_ns=8351 avg_ns=4175 max_ns=6309
  policy_decision: count=694047 total_ns=412518841 avg_ns=594 max_ns=254445
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=178418941 avg_ns=257 max_ns=437199
  matcher_candidate_order.path: count=2082141 total_ns=362764239 avg_ns=174 max_ns=265706
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=86732664 avg_ns=124 max_ns=265706
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=93862862 avg_ns=135 max_ns=92117
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=178418941 avg_ns=257 max_ns=437199
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=182168713 avg_ns=262 max_ns=88388
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5765824 avg_ns=22 max_ns=166542
  state_read_lock_hold: count=260320 total_ns=17528551 avg_ns=67 max_ns=34550
  state_write_lock_wait: count=195344 total_ns=3988932 avg_ns=20 max_ns=953
  state_write_lock_hold: count=195344 total_ns=268044308 avg_ns=1372 max_ns=2729723
  open_confined_openat2: count=266227 total_ns=200427007 avg_ns=752 max_ns=1886609
  open_like.pre_open_guard.access: count=1 total_ns=62399 avg_ns=62399 max_ns=62399
  open_like.pre_open_guard.opendir: count=26 total_ns=318264 avg_ns=12240 max_ns=34669
  open_like.post_open_revalidation.access: count=1 total_ns=4838 avg_ns=4838 max_ns=4838
  open_like.post_open_revalidation.opendir: count=26 total_ns=110506 avg_ns=4250 max_ns=7182
  stat_child_no_follow: count=265989 total_ns=345737696 avg_ns=1299 max_ns=1892287
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4132546 avg_ns=15 max_ns=2254
  stat_child_no_follow.host_fstat: count=265989 total_ns=56388612 avg_ns=211 max_ns=63053
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=345737696 avg_ns=1299 max_ns=1892287
  source_root_path: count=260372 total_ns=416534273 avg_ns=1599 max_ns=150698
  resolved_virtual_path: count=260399 total_ns=820402443 avg_ns=3150 max_ns=360370
  resolved_virtual_path_from_path: count=260161 total_ns=819600181 avg_ns=3150 max_ns=360370
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=703092617 avg_ns=2702 max_ns=258533
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=579243111 avg_ns=989 max_ns=258036
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=58518496 avg_ns=99 max_ns=58363
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=101280228 avg_ns=389 max_ns=40809
  resolved_virtual_path_from_open_fd: count=238 total_ns=802262 avg_ns=3370 max_ns=22917
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1294843856 avg_ns=7193576 max_ns=15889982
  readdir_scan.name_child_path_materialization: count=180 total_ns=105791451 avg_ns=587730 max_ns=1435134
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=910799916 avg_ns=5059999 max_ns=11712644
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=70442121 avg_ns=391345 max_ns=1042409
  readdir_page_commit: count=180 total_ns=136235163 avg_ns=756862 max_ns=2730246
  readdirplus_directory_scan: count=31 total_ns=85833114 avg_ns=2768810 max_ns=3420099
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=29693015 avg_ns=957839 max_ns=1177140
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=8506491 avg_ns=1459 max_ns=51236
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=13075763 avg_ns=2243 max_ns=76124
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2393526 avg_ns=77210 max_ns=95939
  readdirplus_attr_generation_scan: count=5859 total_ns=8506491 avg_ns=1451 max_ns=51236
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=18192703 avg_ns=586861 max_ns=780199
  readdirplus_page_commit: count=31 total_ns=5338728 avg_ns=172217 max_ns=405620
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
