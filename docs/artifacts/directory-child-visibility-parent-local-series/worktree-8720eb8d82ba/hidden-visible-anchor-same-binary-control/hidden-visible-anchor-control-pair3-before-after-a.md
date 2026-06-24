# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:43.099272+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-before-after-a.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-before-after-a.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-before-after-a.svg`
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
| readdir_basic | 0.000888 | 0.046764 | 52.647 | 0.049516 | 0.050945 | 0.052088 |
| readdirplus_basic | 0.003596 | 0.338002 | 94.004 | 0.392479 | 0.394974 | 0.396971 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=48374 avg_ns=48374 max_ns=48374
  fuse_op.getattr: count=65027 total_ns=630307441 avg_ns=9693 max_ns=366064
  fuse_op.lookup: count=195055 total_ns=1717039585 avg_ns=8802 max_ns=360466
  fuse_op.opendir: count=26 total_ns=470547 avg_ns=18097 max_ns=26340
  fuse_op.readdir: count=180 total_ns=1238354970 avg_ns=6879749 max_ns=15163691
  fuse_op.readdirplus: count=31 total_ns=99697498 avg_ns=3216048 max_ns=4466495
  fuse_op.releasedir: count=26 total_ns=11193040 avg_ns=430501 max_ns=894594
  fuse_op.statfs: count=2 total_ns=2991 avg_ns=1495 max_ns=1638
  policy_decision: count=694047 total_ns=370678338 avg_ns=534 max_ns=263388
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=156821903 avg_ns=225 max_ns=105034
  matcher_candidate_order.path: count=2082141 total_ns=322421971 avg_ns=154 max_ns=145754
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=75626847 avg_ns=108 max_ns=89982
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=83393795 avg_ns=120 max_ns=145754
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=156821903 avg_ns=225 max_ns=105034
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=163401329 avg_ns=235 max_ns=118017
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5551050 avg_ns=21 max_ns=17870
  state_read_lock_hold: count=260320 total_ns=17335843 avg_ns=66 max_ns=31032
  state_write_lock_wait: count=195344 total_ns=3941637 avg_ns=20 max_ns=19430
  state_write_lock_hold: count=195344 total_ns=244180594 avg_ns=1250 max_ns=2263710
  open_confined_openat2: count=266227 total_ns=194991832 avg_ns=732 max_ns=341443
  open_like.pre_open_guard.access: count=1 total_ns=41436 avg_ns=41436 max_ns=41436
  open_like.pre_open_guard.opendir: count=26 total_ns=317434 avg_ns=12209 max_ns=20949
  open_like.post_open_revalidation.access: count=1 total_ns=4134 avg_ns=4134 max_ns=4134
  open_like.post_open_revalidation.opendir: count=26 total_ns=95685 avg_ns=3680 max_ns=6081
  stat_child_no_follow: count=265989 total_ns=338599722 avg_ns=1272 max_ns=342038
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4109865 avg_ns=15 max_ns=34981
  stat_child_no_follow.host_fstat: count=265989 total_ns=55497537 avg_ns=208 max_ns=141194
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=338599722 avg_ns=1272 max_ns=342038
  source_root_path: count=260372 total_ns=411462638 avg_ns=1580 max_ns=352359
  resolved_virtual_path: count=260399 total_ns=804421322 avg_ns=3089 max_ns=360803
  resolved_virtual_path_from_path: count=260161 total_ns=803868752 avg_ns=3089 max_ns=360803
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=691777785 avg_ns=2659 max_ns=360334
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=570044266 avg_ns=973 max_ns=359741
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=57658690 avg_ns=98 max_ns=80996
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=97380553 avg_ns=374 max_ns=336654
  resolved_virtual_path_from_open_fd: count=238 total_ns=552570 avg_ns=2321 max_ns=7865
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1096886085 avg_ns=6093811 max_ns=14251210
  readdir_scan.name_child_path_materialization: count=180 total_ns=91680088 avg_ns=509333 max_ns=1178894
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=795161322 avg_ns=4417562 max_ns=10592617
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=55930906 avg_ns=310727 max_ns=926147
  readdir_page_commit: count=180 total_ns=119199761 avg_ns=662220 max_ns=2263895
  readdirplus_directory_scan: count=31 total_ns=74349479 avg_ns=2398370 max_ns=3432537
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27290961 avg_ns=880353 max_ns=1216536
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6955709 avg_ns=1193 max_ns=12405
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=11595586 avg_ns=1989 max_ns=14384
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2019152 avg_ns=65133 max_ns=91624
  readdirplus_attr_generation_scan: count=5859 total_ns=6955709 avg_ns=1187 max_ns=12405
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16322017 avg_ns=526516 max_ns=715515
  readdirplus_page_commit: count=31 total_ns=4682051 avg_ns=151033 max_ns=276518
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
