# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:39.712501+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-after-pair3 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair3.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair3.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/after-descendant-pair3.svg`
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
- policy_label: `matcher-descendant-same-binary-control-after-pair3`
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
| matcher_descendant_readdir | 0.000029 | 0.000589 | 20.173 | 0.000597 | 0.000600 | 0.000603 |
| matcher_descendant_readdirplus | 0.000069 | 0.003334 | 48.487 | 0.003366 | 0.003376 | 0.003385 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=88781 avg_ns=88781 max_ns=88781
  fuse_op.getattr: count=443 total_ns=6899027 avg_ns=15573 max_ns=29510
  fuse_op.lookup: count=1747 total_ns=20463744 avg_ns=11713 max_ns=81621
  fuse_op.opendir: count=26 total_ns=512842 avg_ns=19724 max_ns=34179
  fuse_op.readdir: count=26 total_ns=354969 avg_ns=13652 max_ns=32735
  fuse_op.readdirplus: count=26 total_ns=11805229 avg_ns=454047 max_ns=567853
  fuse_op.releasedir: count=26 total_ns=55209 avg_ns=2123 max_ns=3772
  fuse_op.statfs: count=2 total_ns=6462 avg_ns=3231 max_ns=3544
  policy_decision: count=4012 total_ns=7765993 avg_ns=1935 max_ns=10157
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2040428 avg_ns=508 max_ns=9025
  matcher_candidate_order.path: count=12036 total_ns=5647156 avg_ns=469 max_ns=11572
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2342611 avg_ns=583 max_ns=8853
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=734550 avg_ns=183 max_ns=11572
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2040428 avg_ns=508 max_ns=9025
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2569995 avg_ns=640 max_ns=3367
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=56921 avg_ns=25 max_ns=334
  state_read_lock_hold: count=2269 total_ns=145548 avg_ns=64 max_ns=1311
  state_write_lock_wait: count=1875 total_ns=43421 avg_ns=23 max_ns=137
  state_write_lock_hold: count=1875 total_ns=966270 avg_ns=515 max_ns=43918
  open_confined_openat2: count=3180 total_ns=2280518 avg_ns=717 max_ns=10875
  open_like.pre_open_guard.access: count=1 total_ns=70587 avg_ns=70587 max_ns=70587
  open_like.pre_open_guard.opendir: count=26 total_ns=324392 avg_ns=12476 max_ns=19763
  open_like.post_open_revalidation.access: count=1 total_ns=11043 avg_ns=11043 max_ns=11043
  open_like.post_open_revalidation.opendir: count=26 total_ns=148854 avg_ns=5725 max_ns=10220
  stat_child_no_follow: count=3101 total_ns=3940284 avg_ns=1270 max_ns=19021
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53626 avg_ns=17 max_ns=46
  stat_child_no_follow.host_fstat: count=3099 total_ns=629719 avg_ns=203 max_ns=2663
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3940284 avg_ns=1270 max_ns=19021
  source_root_path: count=2321 total_ns=4190455 avg_ns=1805 max_ns=28440
  resolved_virtual_path: count=2346 total_ns=9977371 avg_ns=4252 max_ns=9688
  resolved_virtual_path_from_path: count=2267 total_ns=9863845 avg_ns=4351 max_ns=9688
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8791395 avg_ns=3877 max_ns=9027
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7383771 avg_ns=1176 max_ns=8202
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=653294 avg_ns=104 max_ns=1095
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=928762 avg_ns=409 max_ns=1401
  resolved_virtual_path_from_open_fd: count=79 total_ns=113526 avg_ns=1437 max_ns=5661
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=103204 avg_ns=3969 max_ns=13409
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5081 avg_ns=195 max_ns=406
  readdirplus_directory_scan: count=26 total_ns=5012300 avg_ns=192780 max_ns=237230
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=312521 avg_ns=12020 max_ns=26031
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=969186 avg_ns=1164 max_ns=2471
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4431900 avg_ns=5326 max_ns=7126
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4419055 avg_ns=169963 max_ns=200855
  readdirplus_attr_generation_scan: count=858 total_ns=969186 avg_ns=1129 max_ns=2471
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=110196 avg_ns=4238 max_ns=18794
  readdirplus_page_commit: count=26 total_ns=427582 avg_ns=16445 max_ns=44043
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
