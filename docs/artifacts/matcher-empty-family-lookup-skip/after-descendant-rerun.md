# ScreenFS benchmark result

- timestamp: `2026-06-24T05:22:14.503280+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-path-only-after-rerun --workload-set matcher-descendant-directory --output-json /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant-rerun.json --output-md /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant-rerun.md --output-svg /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only`
- screenfs_bin_sha256: `5d1d2cee33f097a723efd3196a68074f2af19b05c5026698fbecfc1789bf4ae7`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-path-only-after-rerun`
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
| matcher_descendant_readdir | 0.000042 | 0.000495 | 11.729 | 0.000613 | 0.000626 | 0.000637 |
| matcher_descendant_readdirplus | 0.000095 | 0.002718 | 28.673 | 0.003039 | 0.003065 | 0.003086 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=94065 avg_ns=94065 max_ns=94065
  fuse_op.getattr: count=443 total_ns=5528802 avg_ns=12480 max_ns=56499
  fuse_op.lookup: count=1747 total_ns=16305172 avg_ns=9333 max_ns=127389
  fuse_op.opendir: count=26 total_ns=415896 avg_ns=15996 max_ns=47118
  fuse_op.readdir: count=26 total_ns=366263 avg_ns=14087 max_ns=56602
  fuse_op.readdirplus: count=26 total_ns=9890205 avg_ns=380392 max_ns=578782
  fuse_op.releasedir: count=26 total_ns=106242 avg_ns=4086 max_ns=15438
  fuse_op.statfs: count=2 total_ns=4613 avg_ns=2306 max_ns=2926
  policy_decision: count=4012 total_ns=6299688 avg_ns=1570 max_ns=26991
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1781738 avg_ns=444 max_ns=13711
  matcher_candidate_order.path: count=12036 total_ns=4423803 avg_ns=367 max_ns=17767
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1770037 avg_ns=441 max_ns=17767
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=545368 avg_ns=135 max_ns=10052
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1781738 avg_ns=444 max_ns=13711
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2108398 avg_ns=525 max_ns=8026
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=40689 avg_ns=17 max_ns=262
  state_read_lock_hold: count=2269 total_ns=150539 avg_ns=66 max_ns=3012
  state_write_lock_wait: count=1875 total_ns=31884 avg_ns=17 max_ns=198
  state_write_lock_hold: count=1875 total_ns=933703 avg_ns=497 max_ns=45720
  open_confined_openat2: count=3180 total_ns=1688377 avg_ns=530 max_ns=23231
  open_like.pre_open_guard.access: count=1 total_ns=76368 avg_ns=76368 max_ns=76368
  open_like.pre_open_guard.opendir: count=26 total_ns=260361 avg_ns=10013 max_ns=27935
  open_like.post_open_revalidation.access: count=1 total_ns=10746 avg_ns=10746 max_ns=10746
  open_like.post_open_revalidation.opendir: count=26 total_ns=121610 avg_ns=4677 max_ns=15038
  stat_child_no_follow: count=3101 total_ns=2952851 avg_ns=952 max_ns=61575
  stat_child_no_follow.attr_conversion: count=3099 total_ns=40944 avg_ns=13 max_ns=192
  stat_child_no_follow.host_fstat: count=3099 total_ns=383545 avg_ns=123 max_ns=2601
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=2952851 avg_ns=952 max_ns=61575
  source_root_path: count=2321 total_ns=2982439 avg_ns=1284 max_ns=36874
  resolved_virtual_path: count=2346 total_ns=7810231 avg_ns=3329 max_ns=40820
  resolved_virtual_path_from_path: count=2267 total_ns=7712115 avg_ns=3401 max_ns=40820
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=6816491 avg_ns=3006 max_ns=32819
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5674245 avg_ns=904 max_ns=26079
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=572586 avg_ns=91 max_ns=3562
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=785498 avg_ns=346 max_ns=14585
  resolved_virtual_path_from_open_fd: count=79 total_ns=98116 avg_ns=1241 max_ns=5975
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=109094 avg_ns=4195 max_ns=12835
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5207 avg_ns=200 max_ns=442
  readdirplus_directory_scan: count=26 total_ns=4240549 avg_ns=163098 max_ns=242256
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=313266 avg_ns=12048 max_ns=47654
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=804310 avg_ns=966 max_ns=61644
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3682304 avg_ns=4425 max_ns=8188
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=3696917 avg_ns=142189 max_ns=208182
  readdirplus_attr_generation_scan: count=858 total_ns=804310 avg_ns=937 max_ns=61644
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=93022 avg_ns=3577 max_ns=9140
  readdirplus_page_commit: count=26 total_ns=392923 avg_ns=15112 max_ns=45883
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
