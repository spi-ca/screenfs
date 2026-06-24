# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:23.633322+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-after-control-b.svg`
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
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss, matcher_readonly_access_wok`
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
| metadata_lookup | 0.001342 | 0.012075 | 8.998 | 0.013051 | 0.013146 | 0.013222 |
| metadata_getattr | 0.001376 | 0.016214 | 11.788 | 0.017025 | 0.017758 | 0.018344 |
| metadata_access | 0.001325 | 0.017019 | 12.849 | 0.017873 | 0.017904 | 0.017929 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009507 | 0.010883 | 0.011000 | 0.011094 |
| matcher_readonly_access_wok | 0.022311 | 0.022816 | 0.022989 | 0.023128 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=119674187 avg_ns=23009 max_ns=81204
  fuse_op.getattr: count=3017 total_ns=42100813 avg_ns=13954 max_ns=41169
  fuse_op.lookup: count=43269 total_ns=479117468 avg_ns=11072 max_ns=87555
  fuse_op.statfs: count=2 total_ns=5246 avg_ns=2623 max_ns=3177
  policy_decision: count=59288 total_ns=81023305 avg_ns=1366 max_ns=15040
  matcher_candidates: count=614320
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=2600
  matcher_candidates_by_source.visible.descendant: count=609120
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=614320
  matcher_candidate_order.descendant: count=56688 total_ns=32418198 avg_ns=571 max_ns=19091
  matcher_candidate_order.path: count=175264 total_ns=88054751 avg_ns=502 max_ns=50585
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39832805 avg_ns=702 max_ns=36252
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9629743 avg_ns=169 max_ns=50585
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2628006 avg_ns=1010 max_ns=4899
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=32418198 avg_ns=571 max_ns=19091
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35498762 avg_ns=626 max_ns=9038
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=465435 avg_ns=179 max_ns=463
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1288127 avg_ns=25 max_ns=334
  state_read_lock_hold: count=51487 total_ns=3269646 avg_ns=63 max_ns=18639
  state_write_lock_wait: count=38067 total_ns=886217 avg_ns=23 max_ns=364
  state_write_lock_hold: count=38067 total_ns=13992818 avg_ns=367 max_ns=19945
  open_confined_openat2: count=54088 total_ns=45229616 avg_ns=836 max_ns=43471
  open_like.pre_open_guard.access: count=5201 total_ns=98725006 avg_ns=18981 max_ns=66418
  open_like.post_open_revalidation.access: count=2601 total_ns=16772827 avg_ns=6448 max_ns=17237
  stat_child_no_follow: count=51487 total_ns=75210501 avg_ns=1460 max_ns=44166
  stat_child_no_follow.attr_conversion: count=48885 total_ns=850775 avg_ns=17 max_ns=219
  stat_child_no_follow.host_fstat: count=48885 total_ns=11988428 avg_ns=245 max_ns=13311
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=75210501 avg_ns=1460 max_ns=44166
  source_root_path: count=48887 total_ns=93532625 avg_ns=1913 max_ns=71890
  resolved_virtual_path: count=48886 total_ns=162852390 avg_ns=3331 max_ns=47780
  resolved_virtual_path_from_path: count=46285 total_ns=159693433 avg_ns=3450 max_ns=47780
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=139251771 avg_ns=3008 max_ns=47138
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=115772686 avg_ns=1106 max_ns=46303
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11423630 avg_ns=109 max_ns=9033
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17534579 avg_ns=378 max_ns=10558
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3158957 avg_ns=1214 max_ns=9175
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_entries: count=0
  readdirplus_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
