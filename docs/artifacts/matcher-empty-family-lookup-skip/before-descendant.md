# ScreenFS benchmark result

- timestamp: `2026-06-24T05:20:24.639157+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-before --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-empty-family-before --workload-set matcher-descendant-directory --output-json /tmp/screenfs-matcher-empty-family/formal/before-descendant.json --output-md /tmp/screenfs-matcher-empty-family/formal/before-descendant.md --output-svg /tmp/screenfs-matcher-empty-family/formal/before-descendant.svg`
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
- policy_label: `glob-matcher-heavy-empty-family-before`
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
| matcher_descendant_readdir | 0.000042 | 0.000551 | 13.199 | 0.000799 | 0.001239 | 0.001592 |
| matcher_descendant_readdirplus | 0.000096 | 0.002824 | 29.347 | 0.003366 | 0.003406 | 0.003438 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=88546 avg_ns=88546 max_ns=88546
  fuse_op.getattr: count=443 total_ns=5885986 avg_ns=13286 max_ns=101663
  fuse_op.lookup: count=1747 total_ns=16544442 avg_ns=9470 max_ns=109685
  fuse_op.opendir: count=26 total_ns=497058 avg_ns=19117 max_ns=59006
  fuse_op.readdir: count=26 total_ns=434585 avg_ns=16714 max_ns=48401
  fuse_op.readdirplus: count=26 total_ns=10682450 avg_ns=410863 max_ns=1262889
  fuse_op.releasedir: count=26 total_ns=223753 avg_ns=8605 max_ns=16849
  fuse_op.statfs: count=2 total_ns=5796 avg_ns=2898 max_ns=3728
  policy_decision: count=4012 total_ns=6624483 avg_ns=1651 max_ns=11594
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1885637 avg_ns=469 max_ns=13618
  matcher_candidate_order.path: count=12036 total_ns=4671884 avg_ns=388 max_ns=10816
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1845695 avg_ns=460 max_ns=6499
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=588787 avg_ns=146 max_ns=10816
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1885637 avg_ns=469 max_ns=13618
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2237402 avg_ns=557 max_ns=4515
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=41374 avg_ns=18 max_ns=216
  state_read_lock_hold: count=2269 total_ns=169655 avg_ns=74 max_ns=3143
  state_write_lock_wait: count=1875 total_ns=32025 avg_ns=17 max_ns=182
  state_write_lock_hold: count=1875 total_ns=1084464 avg_ns=578 max_ns=47093
  open_confined_openat2: count=3180 total_ns=1823418 avg_ns=573 max_ns=37809
  open_like.pre_open_guard.access: count=1 total_ns=73130 avg_ns=73130 max_ns=73130
  open_like.pre_open_guard.opendir: count=26 total_ns=320301 avg_ns=12319 max_ns=35974
  open_like.post_open_revalidation.access: count=1 total_ns=8863 avg_ns=8863 max_ns=8863
  open_like.post_open_revalidation.opendir: count=26 total_ns=137522 avg_ns=5289 max_ns=17858
  stat_child_no_follow: count=3101 total_ns=3050820 avg_ns=983 max_ns=38691
  stat_child_no_follow.attr_conversion: count=3099 total_ns=41380 avg_ns=13 max_ns=78
  stat_child_no_follow.host_fstat: count=3099 total_ns=396534 avg_ns=127 max_ns=2772
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3050820 avg_ns=983 max_ns=38691
  source_root_path: count=2321 total_ns=3094998 avg_ns=1333 max_ns=52680
  resolved_virtual_path: count=2346 total_ns=8034109 avg_ns=3424 max_ns=37484
  resolved_virtual_path_from_path: count=2267 total_ns=7914584 avg_ns=3491 max_ns=37484
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=6982726 avg_ns=3080 max_ns=21084
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5806621 avg_ns=925 max_ns=16741
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=575272 avg_ns=91 max_ns=2378
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=817410 avg_ns=360 max_ns=16138
  resolved_virtual_path_from_open_fd: count=79 total_ns=119525 avg_ns=1512 max_ns=5422
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=147824 avg_ns=5685 max_ns=21745
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5384 avg_ns=207 max_ns=651
  readdirplus_directory_scan: count=26 total_ns=4582767 avg_ns=176260 max_ns=533309
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=283296 avg_ns=10896 max_ns=26224
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=779841 avg_ns=937 max_ns=38760
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3971397 avg_ns=4773 max_ns=15735
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=3999958 avg_ns=153844 max_ns=457603
  readdirplus_attr_generation_scan: count=858 total_ns=779841 avg_ns=908 max_ns=38760
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=124981 avg_ns=4806 max_ns=22141
  readdirplus_page_commit: count=26 total_ns=414745 avg_ns=15951 max_ns=47492
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
