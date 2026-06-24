# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:26.291258+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-after-parent-local.svg`
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
| readdir_basic | 0.000679 | 0.047643 | 70.126 | 0.051046 | 0.051301 | 0.051506 |
| readdirplus_basic | 0.003521 | 0.350909 | 99.665 | 0.384181 | 0.385876 | 0.387232 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65085 avg_ns=65085 max_ns=65085
  fuse_op.getattr: count=65027 total_ns=627585690 avg_ns=9651 max_ns=482212
  fuse_op.lookup: count=195055 total_ns=1722251664 avg_ns=8829 max_ns=637201
  fuse_op.opendir: count=26 total_ns=475188 avg_ns=18276 max_ns=40940
  fuse_op.readdir: count=180 total_ns=1223543319 avg_ns=6797462 max_ns=15405113
  fuse_op.readdirplus: count=31 total_ns=93165382 avg_ns=3005334 max_ns=4262629
  fuse_op.releasedir: count=26 total_ns=10682713 avg_ns=410873 max_ns=847583
  fuse_op.statfs: count=2 total_ns=2265 avg_ns=1132 max_ns=1529
  policy_decision: count=694047 total_ns=366613115 avg_ns=528 max_ns=274335
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=156304438 avg_ns=225 max_ns=115924
  matcher_candidate_order.path: count=2082141 total_ns=322551305 avg_ns=154 max_ns=153443
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=75347912 avg_ns=108 max_ns=153443
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=83530296 avg_ns=120 max_ns=122787
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=156304438 avg_ns=225 max_ns=115924
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=163673097 avg_ns=235 max_ns=79184
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5538554 avg_ns=21 max_ns=21780
  state_read_lock_hold: count=260320 total_ns=17881278 avg_ns=68 max_ns=131289
  state_write_lock_wait: count=195344 total_ns=3893945 avg_ns=19 max_ns=3648
  state_write_lock_hold: count=195344 total_ns=248608305 avg_ns=1272 max_ns=2189962
  open_confined_openat2: count=266227 total_ns=194254311 avg_ns=729 max_ns=490701
  open_like.pre_open_guard.access: count=1 total_ns=50927 avg_ns=50927 max_ns=50927
  open_like.pre_open_guard.opendir: count=26 total_ns=295288 avg_ns=11357 max_ns=23540
  open_like.post_open_revalidation.access: count=1 total_ns=9949 avg_ns=9949 max_ns=9949
  open_like.post_open_revalidation.opendir: count=26 total_ns=120953 avg_ns=4652 max_ns=29022
  stat_child_no_follow: count=265989 total_ns=337370740 avg_ns=1268 max_ns=492187
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4134900 avg_ns=15 max_ns=84718
  stat_child_no_follow.host_fstat: count=265989 total_ns=54579829 avg_ns=205 max_ns=258604
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=337370740 avg_ns=1268 max_ns=492187
  source_root_path: count=260372 total_ns=411534513 avg_ns=1580 max_ns=246029
  resolved_virtual_path: count=260399 total_ns=797458402 avg_ns=3062 max_ns=301654
  resolved_virtual_path_from_path: count=260161 total_ns=796848284 avg_ns=3062 max_ns=301654
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=683022482 avg_ns=2625 max_ns=301034
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=560308021 avg_ns=957 max_ns=300435
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=58837046 avg_ns=100 max_ns=70764
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=99114124 avg_ns=380 max_ns=173358
  resolved_virtual_path_from_open_fd: count=238 total_ns=610118 avg_ns=2563 max_ns=26164
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1078813229 avg_ns=5993406 max_ns=14414300
  readdir_scan.name_child_path_materialization: count=180 total_ns=91128109 avg_ns=506267 max_ns=1235663
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=789018062 avg_ns=4383433 max_ns=10848734
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=55478714 avg_ns=308215 max_ns=1083393
  readdir_page_commit: count=180 total_ns=120822423 avg_ns=671235 max_ns=2190226
  readdirplus_directory_scan: count=31 total_ns=69786460 avg_ns=2251176 max_ns=3295087
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=26686641 avg_ns=860859 max_ns=1184634
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6159833 avg_ns=1056 max_ns=32579
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=10932552 avg_ns=1875 max_ns=35728
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1970463 avg_ns=63563 max_ns=97053
  readdirplus_attr_generation_scan: count=5859 total_ns=6159833 avg_ns=1051 max_ns=32579
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16010199 avg_ns=516458 max_ns=718767
  readdirplus_page_commit: count=31 total_ns=4398912 avg_ns=141900 max_ns=234080
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
