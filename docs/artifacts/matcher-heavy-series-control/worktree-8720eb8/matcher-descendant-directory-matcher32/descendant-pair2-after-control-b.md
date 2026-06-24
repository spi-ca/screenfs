# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:28.848023+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-after-control-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-series-after-control`
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
| matcher_descendant_readdir | 0.000044 | 0.000594 | 13.397 | 0.000634 | 0.000646 | 0.000655 |
| matcher_descendant_readdirplus | 0.000061 | 0.002680 | 44.240 | 0.002910 | 0.003043 | 0.003149 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=63733 avg_ns=63733 max_ns=63733
  fuse_op.getattr: count=443 total_ns=5694308 avg_ns=12853 max_ns=46599
  fuse_op.lookup: count=1747 total_ns=16760236 avg_ns=9593 max_ns=109619
  fuse_op.opendir: count=26 total_ns=460048 avg_ns=17694 max_ns=44322
  fuse_op.readdir: count=26 total_ns=474744 avg_ns=18259 max_ns=36827
  fuse_op.readdirplus: count=26 total_ns=10434656 avg_ns=401332 max_ns=576724
  fuse_op.releasedir: count=26 total_ns=171378 avg_ns=6591 max_ns=15898
  fuse_op.statfs: count=2 total_ns=2249 avg_ns=1124 max_ns=1248
  policy_decision: count=4012 total_ns=6654792 avg_ns=1658 max_ns=12618
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1884993 avg_ns=469 max_ns=3232
  matcher_candidate_order.path: count=12036 total_ns=4776278 avg_ns=396 max_ns=78053
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1878743 avg_ns=468 max_ns=5944
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=565966 avg_ns=141 max_ns=3708
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1884993 avg_ns=469 max_ns=3232
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2331569 avg_ns=581 max_ns=78053
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=75449 avg_ns=33 max_ns=12622
  state_read_lock_hold: count=2269 total_ns=164627 avg_ns=72 max_ns=2356
  state_write_lock_wait: count=1875 total_ns=33726 avg_ns=17 max_ns=148
  state_write_lock_hold: count=1875 total_ns=1047751 avg_ns=558 max_ns=45817
  open_confined_openat2: count=3180 total_ns=1692846 avg_ns=532 max_ns=9732
  open_like.pre_open_guard.access: count=1 total_ns=52952 avg_ns=52952 max_ns=52952
  open_like.pre_open_guard.opendir: count=26 total_ns=294199 avg_ns=11315 max_ns=37451
  open_like.post_open_revalidation.access: count=1 total_ns=6637 avg_ns=6637 max_ns=6637
  open_like.post_open_revalidation.opendir: count=26 total_ns=127843 avg_ns=4917 max_ns=11841
  stat_child_no_follow: count=3101 total_ns=2912356 avg_ns=939 max_ns=14835
  stat_child_no_follow.attr_conversion: count=3099 total_ns=41725 avg_ns=13 max_ns=59
  stat_child_no_follow.host_fstat: count=3099 total_ns=425359 avg_ns=137 max_ns=1677
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=2912356 avg_ns=939 max_ns=14835
  source_root_path: count=2321 total_ns=3003261 avg_ns=1293 max_ns=22912
  resolved_virtual_path: count=2346 total_ns=8105578 avg_ns=3455 max_ns=44470
  resolved_virtual_path_from_path: count=2267 total_ns=7994579 avg_ns=3526 max_ns=44470
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=7073844 avg_ns=3120 max_ns=43971
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5878131 avg_ns=936 max_ns=43349
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=602345 avg_ns=95 max_ns=2007
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=803621 avg_ns=354 max_ns=3606
  resolved_virtual_path_from_open_fd: count=79 total_ns=110999 avg_ns=1405 max_ns=3745
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=161710 avg_ns=6219 max_ns=13568
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6387 avg_ns=245 max_ns=445
  readdirplus_directory_scan: count=26 total_ns=4475784 avg_ns=172145 max_ns=240375
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=288057 avg_ns=11079 max_ns=14344
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=757287 avg_ns=910 max_ns=2474
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3966161 avg_ns=4767 max_ns=81695
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=3924691 avg_ns=150949 max_ns=206492
  readdirplus_attr_generation_scan: count=858 total_ns=757287 avg_ns=882 max_ns=2474
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=100818 avg_ns=3877 max_ns=11106
  readdirplus_page_commit: count=26 total_ns=404598 avg_ns=15561 max_ns=45945
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
