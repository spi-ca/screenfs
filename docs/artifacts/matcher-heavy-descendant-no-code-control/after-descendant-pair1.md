# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:35.069836+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-after-pair1 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair1.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair1.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair1.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+19 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+19 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-descendant-same-binary-control-after-pair1`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
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
| matcher_descendant_readdir | 0.000021 | 0.000528 | 25.562 | 0.000583 | 0.000651 | 0.000706 |
| matcher_descendant_readdirplus | 0.000046 | 0.002990 | 65.662 | 0.003225 | 0.003233 | 0.003240 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=99782 avg_ns=99782 max_ns=99782
  fuse_op.getattr: count=443 total_ns=5690232 avg_ns=12844 max_ns=77450
  fuse_op.lookup: count=1747 total_ns=16662447 avg_ns=9537 max_ns=93962
  fuse_op.opendir: count=26 total_ns=395525 avg_ns=15212 max_ns=22465
  fuse_op.readdir: count=26 total_ns=337375 avg_ns=12975 max_ns=22766
  fuse_op.readdirplus: count=26 total_ns=9509223 avg_ns=365739 max_ns=419349
  fuse_op.releasedir: count=26 total_ns=159588 avg_ns=6138 max_ns=15883
  fuse_op.statfs: count=2 total_ns=2294 avg_ns=1147 max_ns=1383
  policy_decision: count=4012 total_ns=6296099 avg_ns=1569 max_ns=53671
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1748247 avg_ns=435 max_ns=3261
  matcher_candidate_order.path: count=12036 total_ns=4454686 avg_ns=370 max_ns=5700
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1802342 avg_ns=449 max_ns=5700
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=543842 avg_ns=135 max_ns=5196
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1748247 avg_ns=435 max_ns=3261
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2108502 avg_ns=525 max_ns=3555
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=65309 avg_ns=28 max_ns=281
  state_read_lock_hold: count=2269 total_ns=161984 avg_ns=71 max_ns=2394
  state_write_lock_wait: count=1875 total_ns=32774 avg_ns=17 max_ns=267
  state_write_lock_hold: count=1875 total_ns=985950 avg_ns=525 max_ns=29950
  open_confined_openat2: count=3180 total_ns=1690001 avg_ns=531 max_ns=14526
  open_like.pre_open_guard.access: count=1 total_ns=80670 avg_ns=80670 max_ns=80670
  open_like.pre_open_guard.opendir: count=26 total_ns=251258 avg_ns=9663 max_ns=14070
  open_like.post_open_revalidation.access: count=1 total_ns=11640 avg_ns=11640 max_ns=11640
  open_like.post_open_revalidation.opendir: count=26 total_ns=113101 avg_ns=4350 max_ns=6263
  stat_child_no_follow: count=3101 total_ns=2877913 avg_ns=928 max_ns=18745
  stat_child_no_follow.attr_conversion: count=3099 total_ns=40343 avg_ns=13 max_ns=52
  stat_child_no_follow.host_fstat: count=3099 total_ns=396460 avg_ns=127 max_ns=4031
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=2877913 avg_ns=928 max_ns=18745
  source_root_path: count=2321 total_ns=3056978 avg_ns=1317 max_ns=31745
  resolved_virtual_path: count=2346 total_ns=7928774 avg_ns=3379 max_ns=30609
  resolved_virtual_path_from_path: count=2267 total_ns=7832887 avg_ns=3455 max_ns=30609
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=6922932 avg_ns=3053 max_ns=26855
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5747895 avg_ns=915 max_ns=23057
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=582125 avg_ns=92 max_ns=2063
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=799863 avg_ns=352 max_ns=3557
  resolved_virtual_path_from_open_fd: count=79 total_ns=95887 avg_ns=1213 max_ns=6348
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=106202 avg_ns=4084 max_ns=7577
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=4491 avg_ns=172 max_ns=397
  readdirplus_directory_scan: count=26 total_ns=4120312 avg_ns=158473 max_ns=187084
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=274275 avg_ns=10549 max_ns=13617
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=666543 avg_ns=801 max_ns=1665
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3585064 avg_ns=4308 max_ns=8165
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=3572064 avg_ns=137387 max_ns=160014
  readdirplus_attr_generation_scan: count=858 total_ns=666543 avg_ns=776 max_ns=1665
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=99062 avg_ns=3810 max_ns=9918
  readdirplus_page_commit: count=26 total_ns=379327 avg_ns=14589 max_ns=30044
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
