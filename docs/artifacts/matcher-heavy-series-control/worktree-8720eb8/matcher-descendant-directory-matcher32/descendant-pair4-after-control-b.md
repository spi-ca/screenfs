# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:32.993604+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-after-control-b.svg`
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
| matcher_descendant_readdir | 0.000033 | 0.000549 | 16.464 | 0.000587 | 0.000627 | 0.000659 |
| matcher_descendant_readdirplus | 0.000081 | 0.002912 | 36.036 | 0.003203 | 0.003235 | 0.003260 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=87862 avg_ns=87862 max_ns=87862
  fuse_op.getattr: count=443 total_ns=5735031 avg_ns=12945 max_ns=36861
  fuse_op.lookup: count=1747 total_ns=17037538 avg_ns=9752 max_ns=77222
  fuse_op.opendir: count=26 total_ns=472622 avg_ns=18177 max_ns=58477
  fuse_op.readdir: count=26 total_ns=383281 avg_ns=14741 max_ns=26338
  fuse_op.readdirplus: count=26 total_ns=10991715 avg_ns=422758 max_ns=668147
  fuse_op.releasedir: count=26 total_ns=77444 avg_ns=2978 max_ns=11749
  fuse_op.statfs: count=2 total_ns=4543 avg_ns=2271 max_ns=3375
  policy_decision: count=4012 total_ns=6807278 avg_ns=1696 max_ns=8547
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2204254 avg_ns=549 max_ns=278919
  matcher_candidate_order.path: count=12036 total_ns=4805912 avg_ns=399 max_ns=7035
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1921693 avg_ns=478 max_ns=4837
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=590490 avg_ns=147 max_ns=7035
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2204254 avg_ns=549 max_ns=278919
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2293729 avg_ns=571 max_ns=3933
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=60361 avg_ns=26 max_ns=192
  state_read_lock_hold: count=2269 total_ns=148117 avg_ns=65 max_ns=1442
  state_write_lock_wait: count=1875 total_ns=34259 avg_ns=18 max_ns=281
  state_write_lock_hold: count=1875 total_ns=958633 avg_ns=511 max_ns=46500
  open_confined_openat2: count=3180 total_ns=1758568 avg_ns=553 max_ns=10108
  open_like.pre_open_guard.access: count=1 total_ns=71688 avg_ns=71688 max_ns=71688
  open_like.pre_open_guard.opendir: count=26 total_ns=300496 avg_ns=11557 max_ns=37952
  open_like.post_open_revalidation.access: count=1 total_ns=10122 avg_ns=10122 max_ns=10122
  open_like.post_open_revalidation.opendir: count=26 total_ns=135288 avg_ns=5203 max_ns=15603
  stat_child_no_follow: count=3101 total_ns=3027036 avg_ns=976 max_ns=13796
  stat_child_no_follow.attr_conversion: count=3099 total_ns=42793 avg_ns=13 max_ns=167
  stat_child_no_follow.host_fstat: count=3099 total_ns=447810 avg_ns=144 max_ns=7006
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3027036 avg_ns=976 max_ns=13796
  source_root_path: count=2321 total_ns=3133975 avg_ns=1350 max_ns=29876
  resolved_virtual_path: count=2346 total_ns=8136622 avg_ns=3468 max_ns=24249
  resolved_virtual_path_from_path: count=2267 total_ns=8031294 avg_ns=3542 max_ns=24249
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=7091868 avg_ns=3128 max_ns=21098
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5873597 avg_ns=936 max_ns=18069
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=606883 avg_ns=96 max_ns=1694
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=823380 avg_ns=363 max_ns=2920
  resolved_virtual_path_from_open_fd: count=79 total_ns=105328 avg_ns=1333 max_ns=6054
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=141609 avg_ns=5446 max_ns=15259
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5250 avg_ns=201 max_ns=408
  readdirplus_directory_scan: count=26 total_ns=4876556 avg_ns=187559 max_ns=440187
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=296328 avg_ns=11397 max_ns=15876
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=774602 avg_ns=931 max_ns=1940
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4036829 avg_ns=4851 max_ns=7345
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4284460 avg_ns=164786 max_ns=422614
  readdirplus_attr_generation_scan: count=858 total_ns=774602 avg_ns=902 max_ns=1940
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=123427 avg_ns=4747 max_ns=11621
  readdirplus_page_commit: count=26 total_ns=428933 avg_ns=16497 max_ns=46639
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
