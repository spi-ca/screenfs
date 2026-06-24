# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:02.167722+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-before-no-parent-local.svg`
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
| readdir_basic | 0.000669 | 0.055296 | 82.613 | 0.056137 | 0.056185 | 0.056224 |
| readdirplus_basic | 0.003433 | 0.345061 | 100.509 | 0.389576 | 0.400995 | 0.410130 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=78513 avg_ns=78513 max_ns=78513
  fuse_op.getattr: count=65027 total_ns=591233915 avg_ns=9092 max_ns=163875
  fuse_op.lookup: count=195055 total_ns=1609358698 avg_ns=8250 max_ns=214226
  fuse_op.opendir: count=26 total_ns=388192 avg_ns=14930 max_ns=31195
  fuse_op.readdir: count=180 total_ns=1190205242 avg_ns=6612251 max_ns=17952541
  fuse_op.readdirplus: count=31 total_ns=344523397 avg_ns=11113657 max_ns=16802044
  fuse_op.releasedir: count=26 total_ns=11086739 avg_ns=426413 max_ns=1054739
  fuse_op.statfs: count=2 total_ns=9304 avg_ns=4652 max_ns=6884
  policy_decision: count=833915 total_ns=421239939 avg_ns=505 max_ns=489467
  matcher_candidates: count=964313
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=768859
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=964313
  matcher_candidate_order.descendant: count=833915 total_ns=180256112 avg_ns=216 max_ns=323725
  matcher_candidate_order.path: count=2501745 total_ns=373907169 avg_ns=149 max_ns=305137
  matcher_candidate_order_by_source.hidden.path: count=833915 total_ns=87941155 avg_ns=105 max_ns=305137
  matcher_candidate_order_by_source.internal_hidden.path: count=833915 total_ns=97048744 avg_ns=116 max_ns=62635
  matcher_candidate_order_by_source.visible.descendant: count=833915 total_ns=180256112 avg_ns=216 max_ns=323725
  matcher_candidate_order_by_source.visible.path: count=833915 total_ns=188917270 avg_ns=226 max_ns=79671
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3335660
  matcher_candidate_order_seen_slots.descendant: count=1667830
  matcher_candidate_order_seen_slots.path: count=1667830
  matcher_candidate_order_ancestor_steps: count=12560824
  matcher_candidate_order_ancestor_steps.descendant: count=3140206
  matcher_candidate_order_ancestor_steps.path: count=9420618
  state_read_lock_wait: count=260320 total_ns=5379239 avg_ns=20 max_ns=31538
  state_read_lock_hold: count=260320 total_ns=16149225 avg_ns=62 max_ns=27887
  state_write_lock_wait: count=195344 total_ns=3732330 avg_ns=19 max_ns=23171
  state_write_lock_hold: count=195344 total_ns=236732467 avg_ns=1211 max_ns=2203755
  open_confined_openat2: count=266227 total_ns=175462011 avg_ns=659 max_ns=97997
  open_like.pre_open_guard.access: count=1 total_ns=57319 avg_ns=57319 max_ns=57319
  open_like.pre_open_guard.opendir: count=26 total_ns=246384 avg_ns=9476 max_ns=24215
  open_like.post_open_revalidation.access: count=1 total_ns=17011 avg_ns=17011 max_ns=17011
  open_like.post_open_revalidation.opendir: count=26 total_ns=90623 avg_ns=3485 max_ns=5010
  stat_child_no_follow: count=265989 total_ns=308647601 avg_ns=1160 max_ns=138043
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4052938 avg_ns=15 max_ns=133152
  stat_child_no_follow.host_fstat: count=265989 total_ns=48648076 avg_ns=182 max_ns=61930
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=308647601 avg_ns=1160 max_ns=138043
  source_root_path: count=260372 total_ns=372291125 avg_ns=1429 max_ns=139925
  resolved_virtual_path: count=260399 total_ns=764719401 avg_ns=2936 max_ns=157840
  resolved_virtual_path_from_path: count=260161 total_ns=764146180 avg_ns=2937 max_ns=157840
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=659892683 avg_ns=2536 max_ns=156647
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=542148385 avg_ns=926 max_ns=155841
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=57621997 avg_ns=98 max_ns=71059
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=90319793 avg_ns=347 max_ns=68947
  resolved_virtual_path_from_open_fd: count=238 total_ns=573221 avg_ns=2408 max_ns=11660
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1048222938 avg_ns=5823460 max_ns=17034710
  readdir_scan.name_child_path_materialization: count=180 total_ns=89044664 avg_ns=494692 max_ns=1415534
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=773718156 avg_ns=4298434 max_ns=12711045
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=52155235 avg_ns=289751 max_ns=1002453
  readdir_page_commit: count=180 total_ns=119633735 avg_ns=664631 max_ns=2203877
  readdirplus_directory_scan: count=31 total_ns=321177185 avg_ns=10360554 max_ns=15841796
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27003778 avg_ns=871089 max_ns=1330168
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6430523 avg_ns=1103 max_ns=8910
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=10855586 avg_ns=1862 max_ns=25005
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=253286897 avg_ns=8170545 max_ns=12225006
  readdirplus_attr_generation_scan: count=5859 total_ns=6430523 avg_ns=1097 max_ns=8910
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16247634 avg_ns=524117 max_ns=811843
  readdirplus_page_commit: count=31 total_ns=4363267 avg_ns=140750 max_ns=244942
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
