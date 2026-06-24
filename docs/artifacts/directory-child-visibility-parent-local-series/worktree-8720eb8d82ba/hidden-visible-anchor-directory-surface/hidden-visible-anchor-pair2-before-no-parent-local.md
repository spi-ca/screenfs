# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:39.470898+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-before-no-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-before-no-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-before-no-parent-local.svg`
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
| readdir_basic | 0.000909 | 0.068700 | 75.611 | 0.078226 | 0.085683 | 0.091649 |
| readdirplus_basic | 0.004337 | 0.288894 | 66.618 | 0.332927 | 0.355927 | 0.374326 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=70964 avg_ns=70964 max_ns=70964
  fuse_op.getattr: count=65027 total_ns=587719872 avg_ns=9038 max_ns=641250
  fuse_op.lookup: count=195055 total_ns=1584017976 avg_ns=8120 max_ns=391071
  fuse_op.opendir: count=26 total_ns=451339 avg_ns=17359 max_ns=32095
  fuse_op.readdir: count=180 total_ns=1299526811 avg_ns=7219593 max_ns=26928034
  fuse_op.readdirplus: count=31 total_ns=388146162 avg_ns=12520843 max_ns=22826112
  fuse_op.releasedir: count=26 total_ns=10102341 avg_ns=388551 max_ns=784959
  fuse_op.statfs: count=2 total_ns=2540 avg_ns=1270 max_ns=1299
  policy_decision: count=833915 total_ns=450376772 avg_ns=540 max_ns=1520915
  matcher_candidates: count=964313
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=768859
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=964313
  matcher_candidate_order.descendant: count=833915 total_ns=193189123 avg_ns=231 max_ns=1050809
  matcher_candidate_order.path: count=2501745 total_ns=398956667 avg_ns=159 max_ns=315696
  matcher_candidate_order_by_source.hidden.path: count=833915 total_ns=93522638 avg_ns=112 max_ns=294889
  matcher_candidate_order_by_source.internal_hidden.path: count=833915 total_ns=101908245 avg_ns=122 max_ns=100477
  matcher_candidate_order_by_source.visible.descendant: count=833915 total_ns=193189123 avg_ns=231 max_ns=1050809
  matcher_candidate_order_by_source.visible.path: count=833915 total_ns=203525784 avg_ns=244 max_ns=315696
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3335660
  matcher_candidate_order_seen_slots.descendant: count=1667830
  matcher_candidate_order_seen_slots.path: count=1667830
  matcher_candidate_order_ancestor_steps: count=12560824
  matcher_candidate_order_ancestor_steps.descendant: count=3140206
  matcher_candidate_order_ancestor_steps.path: count=9420618
  state_read_lock_wait: count=260320 total_ns=5159078 avg_ns=19 max_ns=26881
  state_read_lock_hold: count=260320 total_ns=17725474 avg_ns=68 max_ns=151988
  state_write_lock_wait: count=195344 total_ns=3681907 avg_ns=18 max_ns=31546
  state_write_lock_hold: count=195344 total_ns=242999239 avg_ns=1243 max_ns=3025780
  open_confined_openat2: count=266227 total_ns=167874846 avg_ns=630 max_ns=610109
  open_like.pre_open_guard.access: count=1 total_ns=53500 avg_ns=53500 max_ns=53500
  open_like.pre_open_guard.opendir: count=26 total_ns=287223 avg_ns=11047 max_ns=24260
  open_like.post_open_revalidation.access: count=1 total_ns=13463 avg_ns=13463 max_ns=13463
  open_like.post_open_revalidation.opendir: count=26 total_ns=104993 avg_ns=4038 max_ns=11698
  stat_child_no_follow: count=265989 total_ns=296619962 avg_ns=1115 max_ns=614168
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3841059 avg_ns=14 max_ns=7349
  stat_child_no_follow.host_fstat: count=265989 total_ns=45557854 avg_ns=171 max_ns=88739
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=296619962 avg_ns=1115 max_ns=614168
  source_root_path: count=260372 total_ns=361880990 avg_ns=1389 max_ns=323521
  resolved_virtual_path: count=260399 total_ns=753586735 avg_ns=2893 max_ns=385540
  resolved_virtual_path_from_path: count=260161 total_ns=752894378 avg_ns=2893 max_ns=385540
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=648036874 avg_ns=2490 max_ns=385111
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=529800429 avg_ns=905 max_ns=382149
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=59038691 avg_ns=100 max_ns=64842
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=90880201 avg_ns=349 max_ns=80224
  resolved_virtual_path_from_open_fd: count=238 total_ns=692357 avg_ns=2909 max_ns=13193
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1148277676 avg_ns=6379320 max_ns=25601639
  readdir_scan.name_child_path_materialization: count=180 total_ns=101268422 avg_ns=562602 max_ns=2405467
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=863432799 avg_ns=4796848 max_ns=20490079
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=60779841 avg_ns=337665 max_ns=1513539
  readdir_page_commit: count=180 total_ns=127978751 avg_ns=710993 max_ns=3026045
  readdirplus_directory_scan: count=31 total_ns=361779950 avg_ns=11670320 max_ns=20604491
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=30717238 avg_ns=990878 max_ns=1785203
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6913257 avg_ns=1186 max_ns=93019
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=12366522 avg_ns=2121 max_ns=29836
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=288257610 avg_ns=9298632 max_ns=16397661
  readdirplus_attr_generation_scan: count=5859 total_ns=6913257 avg_ns=1179 max_ns=93019
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=19225363 avg_ns=620173 max_ns=1280208
  readdirplus_page_commit: count=31 total_ns=4850570 avg_ns=156470 max_ns=342082
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
