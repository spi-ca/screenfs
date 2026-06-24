# ScreenFS benchmark result

- timestamp: `2026-06-24T05:22:13.293679+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-before --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-empty-family-before-rerun --workload-set matcher-descendant-directory --output-json /tmp/screenfs-matcher-empty-family/formal/before-descendant-rerun.json --output-md /tmp/screenfs-matcher-empty-family/formal/before-descendant-rerun.md --output-svg /tmp/screenfs-matcher-empty-family/formal/before-descendant-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-empty-family/formal/screenfs-before`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-empty-family-before-rerun`
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
| matcher_descendant_readdir | 0.000029 | 0.000612 | 21.088 | 0.000648 | 0.000712 | 0.000764 |
| matcher_descendant_readdirplus | 0.000067 | 0.003391 | 50.698 | 0.003491 | 0.003774 | 0.004000 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=96319 avg_ns=96319 max_ns=96319
  fuse_op.getattr: count=443 total_ns=7016181 avg_ns=15837 max_ns=44922
  fuse_op.lookup: count=1747 total_ns=20940387 avg_ns=11986 max_ns=78071
  fuse_op.opendir: count=26 total_ns=516686 avg_ns=19872 max_ns=28537
  fuse_op.readdir: count=26 total_ns=399514 avg_ns=15365 max_ns=29370
  fuse_op.readdirplus: count=26 total_ns=12058030 avg_ns=463770 max_ns=580455
  fuse_op.releasedir: count=26 total_ns=61220 avg_ns=2354 max_ns=5244
  fuse_op.statfs: count=2 total_ns=7976 avg_ns=3988 max_ns=4170
  policy_decision: count=4012 total_ns=7861826 avg_ns=1959 max_ns=129838
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2129541 avg_ns=530 max_ns=8407
  matcher_candidate_order.path: count=12036 total_ns=5598184 avg_ns=465 max_ns=17587
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2318052 avg_ns=577 max_ns=17587
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=707129 avg_ns=176 max_ns=4128
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2129541 avg_ns=530 max_ns=8407
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2573003 avg_ns=641 max_ns=2246
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=61274 avg_ns=27 max_ns=8846
  state_read_lock_hold: count=2269 total_ns=155591 avg_ns=68 max_ns=1084
  state_write_lock_wait: count=1875 total_ns=43513 avg_ns=23 max_ns=205
  state_write_lock_hold: count=1875 total_ns=990284 avg_ns=528 max_ns=44170
  open_confined_openat2: count=3180 total_ns=2301517 avg_ns=723 max_ns=11382
  open_like.pre_open_guard.access: count=1 total_ns=74951 avg_ns=74951 max_ns=74951
  open_like.pre_open_guard.opendir: count=26 total_ns=331105 avg_ns=12734 max_ns=18765
  open_like.post_open_revalidation.access: count=1 total_ns=7201 avg_ns=7201 max_ns=7201
  open_like.post_open_revalidation.opendir: count=26 total_ns=144990 avg_ns=5576 max_ns=7556
  stat_child_no_follow: count=3101 total_ns=3975281 avg_ns=1281 max_ns=18117
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53470 avg_ns=17 max_ns=26
  stat_child_no_follow.host_fstat: count=3099 total_ns=656116 avg_ns=211 max_ns=2167
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3975281 avg_ns=1281 max_ns=18117
  source_root_path: count=2321 total_ns=4467623 avg_ns=1924 max_ns=30283
  resolved_virtual_path: count=2346 total_ns=10096833 avg_ns=4303 max_ns=17675
  resolved_virtual_path_from_path: count=2267 total_ns=9986519 avg_ns=4405 max_ns=17675
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8891264 avg_ns=3922 max_ns=16857
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7438646 avg_ns=1185 max_ns=15897
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=686540 avg_ns=109 max_ns=977
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=953612 avg_ns=420 max_ns=1724
  resolved_virtual_path_from_open_fd: count=79 total_ns=110314 avg_ns=1396 max_ns=4458
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=130157 avg_ns=5006 max_ns=13140
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5434 avg_ns=209 max_ns=653
  readdirplus_directory_scan: count=26 total_ns=5216722 avg_ns=200643 max_ns=319865
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=312224 avg_ns=12008 max_ns=18831
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=967883 avg_ns=1163 max_ns=5166
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4457065 avg_ns=5357 max_ns=22839
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4576281 avg_ns=176010 max_ns=298180
  readdirplus_attr_generation_scan: count=858 total_ns=967883 avg_ns=1128 max_ns=5166
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=142963 avg_ns=5498 max_ns=19201
  readdirplus_page_commit: count=26 total_ns=442041 avg_ns=17001 max_ns=44804
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
