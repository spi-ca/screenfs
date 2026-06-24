# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:48.963993+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-after-after-b.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-after-after-b.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-after-after-b.svg`
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
| readdir_basic | 0.001077 | 0.065486 | 60.807 | 0.068265 | 0.068629 | 0.068921 |
| readdirplus_basic | 0.006703 | 0.284103 | 42.382 | 0.338837 | 0.360790 | 0.378352 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=54587 avg_ns=54587 max_ns=54587
  fuse_op.getattr: count=65027 total_ns=561897433 avg_ns=8640 max_ns=274588
  fuse_op.lookup: count=195055 total_ns=1523530971 avg_ns=7810 max_ns=975406
  fuse_op.opendir: count=26 total_ns=460898 avg_ns=17726 max_ns=43831
  fuse_op.readdir: count=180 total_ns=1389538392 avg_ns=7719657 max_ns=19960627
  fuse_op.readdirplus: count=31 total_ns=106267528 avg_ns=3427984 max_ns=4888927
  fuse_op.releasedir: count=26 total_ns=12774557 avg_ns=491329 max_ns=723894
  fuse_op.statfs: count=2 total_ns=1637 avg_ns=818 max_ns=985
  policy_decision: count=694047 total_ns=373533375 avg_ns=538 max_ns=255189
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=162524631 avg_ns=234 max_ns=297142
  matcher_candidate_order.path: count=2082141 total_ns=331479379 avg_ns=159 max_ns=144173
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=79237539 avg_ns=114 max_ns=108323
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=85144651 avg_ns=122 max_ns=144173
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=162524631 avg_ns=234 max_ns=297142
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=167097189 avg_ns=240 max_ns=86256
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=4962458 avg_ns=19 max_ns=6535
  state_read_lock_hold: count=260320 total_ns=15526521 avg_ns=59 max_ns=47300
  state_write_lock_wait: count=195344 total_ns=3520914 avg_ns=18 max_ns=4548
  state_write_lock_hold: count=195344 total_ns=258380287 avg_ns=1322 max_ns=11973384
  open_confined_openat2: count=266227 total_ns=165091624 avg_ns=620 max_ns=77772
  open_like.pre_open_guard.access: count=1 total_ns=46663 avg_ns=46663 max_ns=46663
  open_like.pre_open_guard.opendir: count=26 total_ns=300523 avg_ns=11558 max_ns=34779
  open_like.post_open_revalidation.access: count=1 total_ns=5040 avg_ns=5040 max_ns=5040
  open_like.post_open_revalidation.opendir: count=26 total_ns=102181 avg_ns=3930 max_ns=6013
  stat_child_no_follow: count=265989 total_ns=287244077 avg_ns=1079 max_ns=116420
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3827995 avg_ns=14 max_ns=49254
  stat_child_no_follow.host_fstat: count=265989 total_ns=44098485 avg_ns=165 max_ns=45854
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=287244077 avg_ns=1079 max_ns=116420
  source_root_path: count=260372 total_ns=348636694 avg_ns=1338 max_ns=323929
  resolved_virtual_path: count=260399 total_ns=729663667 avg_ns=2802 max_ns=288936
  resolved_virtual_path_from_path: count=260161 total_ns=728857343 avg_ns=2801 max_ns=288936
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=629446442 avg_ns=2419 max_ns=288313
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=519283267 avg_ns=887 max_ns=287905
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=54758822 avg_ns=93 max_ns=61975
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=86157146 avg_ns=331 max_ns=267404
  resolved_virtual_path_from_open_fd: count=238 total_ns=806324 avg_ns=3387 max_ns=17212
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1216423263 avg_ns=6757907 max_ns=15187819
  readdir_scan.name_child_path_materialization: count=180 total_ns=99397827 avg_ns=552210 max_ns=1282065
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=859437102 avg_ns=4774650 max_ns=11722013
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=63532573 avg_ns=352958 max_ns=991511
  readdir_page_commit: count=180 total_ns=142962714 avg_ns=794237 max_ns=11973798
  readdirplus_directory_scan: count=31 total_ns=79528631 avg_ns=2565439 max_ns=3393962
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=28375865 avg_ns=915350 max_ns=1186197
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=7754210 avg_ns=1330 max_ns=52067
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=12182950 avg_ns=2090 max_ns=32222
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2226152 avg_ns=71811 max_ns=94297
  readdirplus_attr_generation_scan: count=5859 total_ns=7754210 avg_ns=1323 max_ns=52067
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=17389735 avg_ns=560959 max_ns=787166
  readdirplus_page_commit: count=31 total_ns=4704968 avg_ns=151773 max_ns=301012
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
