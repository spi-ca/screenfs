# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:37.469348+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-after-pair2 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair2.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair2.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair2.svg`
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
- policy_label: `matcher-descendant-same-binary-control-after-pair2`
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
| matcher_descendant_readdir | 0.000027 | 0.000592 | 21.674 | 0.000601 | 0.000604 | 0.000606 |
| matcher_descendant_readdirplus | 0.000063 | 0.003460 | 54.526 | 0.003660 | 0.003674 | 0.003685 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=98795 avg_ns=98795 max_ns=98795
  fuse_op.getattr: count=443 total_ns=7037388 avg_ns=15885 max_ns=52362
  fuse_op.lookup: count=1747 total_ns=20828730 avg_ns=11922 max_ns=83661
  fuse_op.opendir: count=26 total_ns=514858 avg_ns=19802 max_ns=30644
  fuse_op.readdir: count=26 total_ns=440222 avg_ns=16931 max_ns=38397
  fuse_op.readdirplus: count=26 total_ns=11967795 avg_ns=460299 max_ns=561947
  fuse_op.releasedir: count=26 total_ns=72798 avg_ns=2799 max_ns=7043
  fuse_op.statfs: count=2 total_ns=9036 avg_ns=4518 max_ns=5253
  policy_decision: count=4012 total_ns=7842777 avg_ns=1954 max_ns=11749
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2052719 avg_ns=511 max_ns=8526
  matcher_candidate_order.path: count=12036 total_ns=5684892 avg_ns=472 max_ns=10349
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2351364 avg_ns=586 max_ns=2573
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=739888 avg_ns=184 max_ns=10349
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2052719 avg_ns=511 max_ns=8526
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2593640 avg_ns=646 max_ns=2338
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=58192 avg_ns=25 max_ns=320
  state_read_lock_hold: count=2269 total_ns=155851 avg_ns=68 max_ns=1576
  state_write_lock_wait: count=1875 total_ns=44291 avg_ns=23 max_ns=200
  state_write_lock_hold: count=1875 total_ns=1034515 avg_ns=551 max_ns=43828
  open_confined_openat2: count=3180 total_ns=2361191 avg_ns=742 max_ns=11017
  open_like.pre_open_guard.access: count=1 total_ns=66249 avg_ns=66249 max_ns=66249
  open_like.pre_open_guard.opendir: count=26 total_ns=324556 avg_ns=12482 max_ns=18087
  open_like.post_open_revalidation.access: count=1 total_ns=11352 avg_ns=11352 max_ns=11352
  open_like.post_open_revalidation.opendir: count=26 total_ns=142698 avg_ns=5488 max_ns=8796
  stat_child_no_follow: count=3101 total_ns=4039193 avg_ns=1302 max_ns=16609
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53895 avg_ns=17 max_ns=45
  stat_child_no_follow.host_fstat: count=3099 total_ns=667418 avg_ns=215 max_ns=9450
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=4039193 avg_ns=1302 max_ns=16609
  source_root_path: count=2321 total_ns=4375141 avg_ns=1885 max_ns=31646
  resolved_virtual_path: count=2346 total_ns=10133598 avg_ns=4319 max_ns=14126
  resolved_virtual_path_from_path: count=2267 total_ns=10017111 avg_ns=4418 max_ns=14126
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8923449 avg_ns=3936 max_ns=10687
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7456664 avg_ns=1188 max_ns=9708
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=696939 avg_ns=111 max_ns=7913
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=947599 avg_ns=417 max_ns=5798
  resolved_virtual_path_from_open_fd: count=79 total_ns=116487 avg_ns=1474 max_ns=7137
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=151032 avg_ns=5808 max_ns=18864
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=9252 avg_ns=355 max_ns=1265
  readdirplus_directory_scan: count=26 total_ns=5096011 avg_ns=196000 max_ns=236986
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=335892 avg_ns=12918 max_ns=32290
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1004436 avg_ns=1207 max_ns=11092
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4426387 avg_ns=5320 max_ns=14634
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4419485 avg_ns=169980 max_ns=203106
  readdirplus_attr_generation_scan: count=858 total_ns=1004436 avg_ns=1170 max_ns=11092
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=162316 avg_ns=6242 max_ns=37618
  readdirplus_page_commit: count=26 total_ns=452507 avg_ns=17404 max_ns=43983
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
