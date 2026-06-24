# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:45.192787+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-before-no-parent-local.svg`
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
| readdir_basic | 0.000894 | 0.054972 | 61.507 | 0.057477 | 0.060532 | 0.062975 |
| readdirplus_basic | 0.003221 | 0.282464 | 87.698 | 0.310643 | 0.354321 | 0.389263 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=69933 avg_ns=69933 max_ns=69933
  fuse_op.getattr: count=65027 total_ns=580401037 avg_ns=8925 max_ns=92842
  fuse_op.lookup: count=195055 total_ns=1573100290 avg_ns=8064 max_ns=1030814
  fuse_op.opendir: count=26 total_ns=442079 avg_ns=17003 max_ns=49186
  fuse_op.readdir: count=180 total_ns=1158011172 avg_ns=6433395 max_ns=15278671
  fuse_op.readdirplus: count=31 total_ns=355182331 avg_ns=11457494 max_ns=15380093
  fuse_op.releasedir: count=26 total_ns=9339743 avg_ns=359220 max_ns=643265
  fuse_op.statfs: count=2 total_ns=2621 avg_ns=1310 max_ns=1337
  policy_decision: count=833915 total_ns=419736086 avg_ns=503 max_ns=344857
  matcher_candidates: count=964313
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=768859
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=964313
  matcher_candidate_order.descendant: count=833915 total_ns=179557406 avg_ns=215 max_ns=262710
  matcher_candidate_order.path: count=2501745 total_ns=371561047 avg_ns=148 max_ns=322104
  matcher_candidate_order_by_source.hidden.path: count=833915 total_ns=87368550 avg_ns=104 max_ns=77668
  matcher_candidate_order_by_source.internal_hidden.path: count=833915 total_ns=95291423 avg_ns=114 max_ns=321546
  matcher_candidate_order_by_source.visible.descendant: count=833915 total_ns=179557406 avg_ns=215 max_ns=262710
  matcher_candidate_order_by_source.visible.path: count=833915 total_ns=188901074 avg_ns=226 max_ns=322104
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3335660
  matcher_candidate_order_seen_slots.descendant: count=1667830
  matcher_candidate_order_seen_slots.path: count=1667830
  matcher_candidate_order_ancestor_steps: count=12560824
  matcher_candidate_order_ancestor_steps.descendant: count=3140206
  matcher_candidate_order_ancestor_steps.path: count=9420618
  state_read_lock_wait: count=260320 total_ns=5081125 avg_ns=19 max_ns=19507
  state_read_lock_hold: count=260320 total_ns=16481049 avg_ns=63 max_ns=45428
  state_write_lock_wait: count=195344 total_ns=3639255 avg_ns=18 max_ns=5192
  state_write_lock_hold: count=195344 total_ns=229793048 avg_ns=1176 max_ns=2114836
  open_confined_openat2: count=266227 total_ns=172163206 avg_ns=646 max_ns=322098
  open_like.pre_open_guard.access: count=1 total_ns=58911 avg_ns=58911 max_ns=58911
  open_like.pre_open_guard.opendir: count=26 total_ns=293701 avg_ns=11296 max_ns=39462
  open_like.post_open_revalidation.access: count=1 total_ns=6842 avg_ns=6842 max_ns=6842
  open_like.post_open_revalidation.opendir: count=26 total_ns=95146 avg_ns=3659 max_ns=9182
  stat_child_no_follow: count=265989 total_ns=301263074 avg_ns=1132 max_ns=115320
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3881498 avg_ns=14 max_ns=54420
  stat_child_no_follow.host_fstat: count=265989 total_ns=46977843 avg_ns=176 max_ns=96830
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=301263074 avg_ns=1132 max_ns=115320
  source_root_path: count=260372 total_ns=364926120 avg_ns=1401 max_ns=990694
  resolved_virtual_path: count=260399 total_ns=748970038 avg_ns=2876 max_ns=331867
  resolved_virtual_path_from_path: count=260161 total_ns=748128083 avg_ns=2875 max_ns=108548
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=643819202 avg_ns=2474 max_ns=107921
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=528855078 avg_ns=903 max_ns=106655
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=57241170 avg_ns=97 max_ns=62677
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=90756614 avg_ns=348 max_ns=96796
  resolved_virtual_path_from_open_fd: count=238 total_ns=841955 avg_ns=3537 max_ns=331867
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1020477788 avg_ns=5669321 max_ns=14302159
  readdir_scan.name_child_path_materialization: count=180 total_ns=88678732 avg_ns=492659 max_ns=1209057
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=772372682 avg_ns=4290959 max_ns=10780805
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=51350807 avg_ns=285282 max_ns=1051995
  readdir_page_commit: count=180 total_ns=115596452 avg_ns=642202 max_ns=2115044
  readdirplus_directory_scan: count=31 total_ns=329571872 avg_ns=10631350 max_ns=14429975
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27816766 avg_ns=897315 max_ns=1543886
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=7121503 avg_ns=1221 max_ns=57717
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=11523404 avg_ns=1977 max_ns=71462
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=260722717 avg_ns=8410410 max_ns=11066053
  readdirplus_attr_generation_scan: count=5859 total_ns=7121503 avg_ns=1215 max_ns=57717
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16780196 avg_ns=541296 max_ns=691316
  readdirplus_page_commit: count=31 total_ns=4914322 avg_ns=158526 max_ns=370434
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
