# ScreenFS benchmark result

- timestamp: `2026-06-24T22:17:56.261416+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-after-parent-local.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-after-parent-local.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-after-parent-local.svg`
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
| readdir_basic | 0.000889 | 0.047081 | 52.978 | 0.048786 | 0.050035 | 0.051035 |
| readdirplus_basic | 0.003485 | 0.274520 | 78.782 | 0.278753 | 0.287656 | 0.294779 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=72690 avg_ns=72690 max_ns=72690
  fuse_op.getattr: count=65027 total_ns=546844580 avg_ns=8409 max_ns=170436
  fuse_op.lookup: count=195055 total_ns=1482575154 avg_ns=7600 max_ns=365343
  fuse_op.opendir: count=26 total_ns=442095 avg_ns=17003 max_ns=33270
  fuse_op.readdir: count=180 total_ns=1152149634 avg_ns=6400831 max_ns=15635633
  fuse_op.readdirplus: count=31 total_ns=87305117 avg_ns=2816294 max_ns=4219254
  fuse_op.releasedir: count=26 total_ns=9770590 avg_ns=375791 max_ns=628655
  fuse_op.statfs: count=2 total_ns=4549 avg_ns=2274 max_ns=3278
  policy_decision: count=694047 total_ns=333404264 avg_ns=480 max_ns=83609
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=140840832 avg_ns=202 max_ns=96256
  matcher_candidate_order.path: count=2082141 total_ns=294868261 avg_ns=141 max_ns=149186
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=69176214 avg_ns=99 max_ns=56513
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=75069610 avg_ns=108 max_ns=62846
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=140840832 avg_ns=202 max_ns=96256
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=150622437 avg_ns=217 max_ns=149186
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=4846228 avg_ns=18 max_ns=8828
  state_read_lock_hold: count=260320 total_ns=15851140 avg_ns=60 max_ns=57006
  state_write_lock_wait: count=195344 total_ns=3427464 avg_ns=17 max_ns=4575
  state_write_lock_hold: count=195344 total_ns=225088500 avg_ns=1152 max_ns=2309905
  open_confined_openat2: count=266227 total_ns=157327750 avg_ns=590 max_ns=138309
  open_like.pre_open_guard.access: count=1 total_ns=62506 avg_ns=62506 max_ns=62506
  open_like.pre_open_guard.opendir: count=26 total_ns=289783 avg_ns=11145 max_ns=24265
  open_like.post_open_revalidation.access: count=1 total_ns=6188 avg_ns=6188 max_ns=6188
  open_like.post_open_revalidation.opendir: count=26 total_ns=99064 avg_ns=3810 max_ns=8433
  stat_child_no_follow: count=265989 total_ns=278357013 avg_ns=1046 max_ns=156795
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3626799 avg_ns=13 max_ns=875
  stat_child_no_follow.host_fstat: count=265989 total_ns=43383158 avg_ns=163 max_ns=62988
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=278357013 avg_ns=1046 max_ns=156795
  source_root_path: count=260372 total_ns=334643901 avg_ns=1285 max_ns=326211
  resolved_virtual_path: count=260399 total_ns=705569671 avg_ns=2709 max_ns=238229
  resolved_virtual_path_from_path: count=260161 total_ns=705053807 avg_ns=2710 max_ns=238229
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=606497107 avg_ns=2331 max_ns=233286
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=499130762 avg_ns=852 max_ns=231684
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=54368414 avg_ns=92 max_ns=78117
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=85188725 avg_ns=327 max_ns=68280
  resolved_virtual_path_from_open_fd: count=238 total_ns=515864 avg_ns=2167 max_ns=6866
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1016980872 avg_ns=5649893 max_ns=14700684
  readdir_scan.name_child_path_materialization: count=180 total_ns=88260456 avg_ns=490335 max_ns=1214948
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=756113397 avg_ns=4200629 max_ns=10831852
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=50998304 avg_ns=283323 max_ns=1134516
  readdir_page_commit: count=180 total_ns=114972985 avg_ns=638738 max_ns=2310026
  readdirplus_directory_scan: count=31 total_ns=64861633 avg_ns=2092310 max_ns=3272712
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=25946660 avg_ns=836989 max_ns=1123900
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=5764485 avg_ns=989 max_ns=7755
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=10705220 avg_ns=1836 max_ns=44465
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1852414 avg_ns=59755 max_ns=92123
  readdirplus_attr_generation_scan: count=5859 total_ns=5764485 avg_ns=983 max_ns=7755
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=15210021 avg_ns=490645 max_ns=730479
  readdirplus_page_commit: count=31 total_ns=4214169 avg_ns=135940 max_ns=245044
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
