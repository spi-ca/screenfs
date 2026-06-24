# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:33.997398+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-before-control-a.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-series-before-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-series-control`
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
| matcher_descendant_readdir | 0.000020 | 0.000538 | 26.527 | 0.000591 | 0.000609 | 0.000623 |
| matcher_descendant_readdirplus | 0.000047 | 0.003227 | 68.129 | 0.003630 | 0.004470 | 0.005142 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=90787 avg_ns=90787 max_ns=90787
  fuse_op.getattr: count=443 total_ns=6167820 avg_ns=13922 max_ns=115704
  fuse_op.lookup: count=1747 total_ns=18410666 avg_ns=10538 max_ns=140349
  fuse_op.opendir: count=26 total_ns=447437 avg_ns=17209 max_ns=47729
  fuse_op.readdir: count=26 total_ns=485066 avg_ns=18656 max_ns=44033
  fuse_op.readdirplus: count=26 total_ns=10692740 avg_ns=411259 max_ns=557401
  fuse_op.releasedir: count=26 total_ns=127062 avg_ns=4887 max_ns=15467
  fuse_op.statfs: count=2 total_ns=2615 avg_ns=1307 max_ns=1514
  policy_decision: count=4012 total_ns=6876115 avg_ns=1713 max_ns=71292
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1998358 avg_ns=498 max_ns=60535
  matcher_candidate_order.path: count=12036 total_ns=4942242 avg_ns=410 max_ns=59295
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1985438 avg_ns=494 max_ns=22135
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=663006 avg_ns=165 max_ns=59295
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1998358 avg_ns=498 max_ns=60535
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2293798 avg_ns=571 max_ns=5276
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=62394 avg_ns=27 max_ns=207
  state_read_lock_hold: count=2269 total_ns=164826 avg_ns=72 max_ns=2194
  state_write_lock_wait: count=1875 total_ns=34688 avg_ns=18 max_ns=152
  state_write_lock_hold: count=1875 total_ns=1045601 avg_ns=557 max_ns=35702
  open_confined_openat2: count=3180 total_ns=1936916 avg_ns=609 max_ns=12392
  open_like.pre_open_guard.access: count=1 total_ns=71227 avg_ns=71227 max_ns=71227
  open_like.pre_open_guard.opendir: count=26 total_ns=276178 avg_ns=10622 max_ns=29261
  open_like.post_open_revalidation.access: count=1 total_ns=11744 avg_ns=11744 max_ns=11744
  open_like.post_open_revalidation.opendir: count=26 total_ns=134428 avg_ns=5170 max_ns=14441
  stat_child_no_follow: count=3101 total_ns=3342730 avg_ns=1077 max_ns=47128
  stat_child_no_follow.attr_conversion: count=3099 total_ns=42879 avg_ns=13 max_ns=62
  stat_child_no_follow.host_fstat: count=3099 total_ns=458742 avg_ns=148 max_ns=2296
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3342730 avg_ns=1077 max_ns=47128
  source_root_path: count=2321 total_ns=3476978 avg_ns=1498 max_ns=50564
  resolved_virtual_path: count=2346 total_ns=8760835 avg_ns=3734 max_ns=79759
  resolved_virtual_path_from_path: count=2267 total_ns=8640803 avg_ns=3811 max_ns=79759
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=7639400 avg_ns=3369 max_ns=79198
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=6329993 avg_ns=1008 max_ns=78246
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=620848 avg_ns=98 max_ns=2390
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=880382 avg_ns=388 max_ns=17730
  resolved_virtual_path_from_open_fd: count=79 total_ns=120032 avg_ns=1519 max_ns=6121
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=186913 avg_ns=7188 max_ns=18586
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6365 avg_ns=244 max_ns=459
  readdirplus_directory_scan: count=26 total_ns=4599196 avg_ns=176892 max_ns=245099
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=287113 avg_ns=11042 max_ns=14105
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=853351 avg_ns=1025 max_ns=47186
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3953146 avg_ns=4751 max_ns=28346
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4003929 avg_ns=153997 max_ns=223843
  readdirplus_attr_generation_scan: count=858 total_ns=853351 avg_ns=994 max_ns=47186
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=120947 avg_ns=4651 max_ns=17046
  readdirplus_page_commit: count=26 total_ns=424543 avg_ns=16328 max_ns=35805
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
