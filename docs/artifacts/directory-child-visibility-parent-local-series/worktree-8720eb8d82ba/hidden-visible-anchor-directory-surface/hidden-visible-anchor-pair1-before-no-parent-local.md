# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:20.262355+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-before-no-parent-local.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+41 more)`
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
| readdir_basic | 0.001259 | 0.075238 | 59.740 | 0.076275 | 0.076541 | 0.076754 |
| readdirplus_basic | 0.006462 | 0.380870 | 58.940 | 0.407077 | 0.416363 | 0.423792 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=79219 avg_ns=79219 max_ns=79219
  fuse_op.getattr: count=65027 total_ns=697926773 avg_ns=10732 max_ns=359359
  fuse_op.lookup: count=195055 total_ns=1911960570 avg_ns=9802 max_ns=2027509
  fuse_op.opendir: count=26 total_ns=478255 avg_ns=18394 max_ns=35977
  fuse_op.readdir: count=180 total_ns=1495756504 avg_ns=8309758 max_ns=25607339
  fuse_op.readdirplus: count=31 total_ns=427307421 avg_ns=13784110 max_ns=19378111
  fuse_op.releasedir: count=26 total_ns=13850297 avg_ns=532703 max_ns=758308
  fuse_op.statfs: count=2 total_ns=7155 avg_ns=3577 max_ns=3659
  policy_decision: count=833915 total_ns=528383417 avg_ns=633 max_ns=3152873
  matcher_candidates: count=964313
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=768859
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=964313
  matcher_candidate_order.descendant: count=833915 total_ns=222081009 avg_ns=266 max_ns=140751
  matcher_candidate_order.path: count=2501745 total_ns=468403382 avg_ns=187 max_ns=195963
  matcher_candidate_order_by_source.hidden.path: count=833915 total_ns=112243699 avg_ns=134 max_ns=139128
  matcher_candidate_order_by_source.internal_hidden.path: count=833915 total_ns=122994626 avg_ns=147 max_ns=195963
  matcher_candidate_order_by_source.visible.descendant: count=833915 total_ns=222081009 avg_ns=266 max_ns=140751
  matcher_candidate_order_by_source.visible.path: count=833915 total_ns=233165057 avg_ns=279 max_ns=108301
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3335660
  matcher_candidate_order_seen_slots.descendant: count=1667830
  matcher_candidate_order_seen_slots.path: count=1667830
  matcher_candidate_order_ancestor_steps: count=12560824
  matcher_candidate_order_ancestor_steps.descendant: count=3140206
  matcher_candidate_order_ancestor_steps.path: count=9420618
  state_read_lock_wait: count=260320 total_ns=6104604 avg_ns=23 max_ns=46793
  state_read_lock_hold: count=260320 total_ns=18901141 avg_ns=72 max_ns=78720
  state_write_lock_wait: count=195344 total_ns=4384649 avg_ns=22 max_ns=7984
  state_write_lock_hold: count=195344 total_ns=280150347 avg_ns=1434 max_ns=2717708
  open_confined_openat2: count=266227 total_ns=219594009 avg_ns=824 max_ns=2007608
  open_like.pre_open_guard.access: count=1 total_ns=71306 avg_ns=71306 max_ns=71306
  open_like.pre_open_guard.opendir: count=26 total_ns=305600 avg_ns=11753 max_ns=26154
  open_like.post_open_revalidation.access: count=1 total_ns=4467 avg_ns=4467 max_ns=4467
  open_like.post_open_revalidation.opendir: count=26 total_ns=109035 avg_ns=4193 max_ns=6923
  stat_child_no_follow: count=265989 total_ns=385571279 avg_ns=1449 max_ns=2009512
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4540154 avg_ns=17 max_ns=41403
  stat_child_no_follow.host_fstat: count=265989 total_ns=64947401 avg_ns=244 max_ns=153376
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=385571279 avg_ns=1449 max_ns=2009512
  source_root_path: count=260372 total_ns=460780979 avg_ns=1769 max_ns=350212
  resolved_virtual_path: count=260399 total_ns=892910736 avg_ns=3429 max_ns=354671
  resolved_virtual_path_from_path: count=260161 total_ns=892131944 avg_ns=3429 max_ns=354671
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=763675032 avg_ns=2935 max_ns=354201
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=625719958 avg_ns=1069 max_ns=231967
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=64307736 avg_ns=109 max_ns=64732
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=112494568 avg_ns=432 max_ns=74324
  resolved_virtual_path_from_open_fd: count=238 total_ns=778792 avg_ns=3272 max_ns=14733
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1324741243 avg_ns=7359673 max_ns=24049221
  readdir_scan.name_child_path_materialization: count=180 total_ns=106438026 avg_ns=591322 max_ns=1572119
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=940495244 avg_ns=5224973 max_ns=20125027
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=71469484 avg_ns=397052 max_ns=1115174
  readdir_page_commit: count=180 total_ns=139482548 avg_ns=774903 max_ns=2718205
  readdirplus_directory_scan: count=31 total_ns=397351423 avg_ns=12817787 max_ns=18364157
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=31464664 avg_ns=1014989 max_ns=1611088
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=8876918 avg_ns=1523 max_ns=68037
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=12980593 avg_ns=2227 max_ns=67466
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=304235336 avg_ns=9814043 max_ns=13978714
  readdirplus_attr_generation_scan: count=5859 total_ns=8876918 avg_ns=1515 max_ns=68037
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=21638238 avg_ns=698007 max_ns=1214747
  readdirplus_page_commit: count=31 total_ns=5766543 avg_ns=186017 max_ns=307686
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
