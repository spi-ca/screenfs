# ScreenFS benchmark result

- timestamp: `2026-06-24T05:34:00.977742+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-after-pair3 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair3.json --output-md docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair3.md --output-svg docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair3.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-heavy-no-code-control-after-pair3`
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
| metadata_lookup | 0.001416 | 0.011170 | 7.889 | 0.011349 | 0.011469 | 0.011564 |
| metadata_getattr | 0.001495 | 0.014479 | 9.685 | 0.014805 | 0.014921 | 0.015014 |
| metadata_access | 0.001295 | 0.015936 | 12.302 | 0.016419 | 0.016466 | 0.016503 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009327 | 0.009598 | 0.009598 | 0.009598 |
| matcher_readonly_access_wok | 0.023100 | 0.024177 | 0.024601 | 0.024940 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=119467194 avg_ns=22970 max_ns=305527
  fuse_op.getattr: count=3017 total_ns=40904798 avg_ns=13558 max_ns=28866
  fuse_op.lookup: count=43269 total_ns=466071436 avg_ns=10771 max_ns=293786
  fuse_op.statfs: count=2 total_ns=7887 avg_ns=3943 max_ns=4921
  policy_decision: count=59288 total_ns=80526421 avg_ns=1358 max_ns=276675
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
  matcher_candidate_order.descendant: count=56688 total_ns=31820667 avg_ns=561 max_ns=21496
  matcher_candidate_order.path: count=175264 total_ns=86454962 avg_ns=493 max_ns=11873
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39074673 avg_ns=689 max_ns=11873
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=8822990 avg_ns=155 max_ns=9039
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2738424 avg_ns=1053 max_ns=6266
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31820667 avg_ns=561 max_ns=21496
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35347626 avg_ns=623 max_ns=9356
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=471249 avg_ns=181 max_ns=562
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1302634 avg_ns=25 max_ns=826
  state_read_lock_hold: count=51487 total_ns=3191963 avg_ns=61 max_ns=1954
  state_write_lock_wait: count=38067 total_ns=882056 avg_ns=23 max_ns=2268
  state_write_lock_hold: count=38067 total_ns=13234593 avg_ns=347 max_ns=32838
  open_confined_openat2: count=54088 total_ns=41773725 avg_ns=772 max_ns=34085
  open_like.pre_open_guard.access: count=5201 total_ns=99073302 avg_ns=19048 max_ns=305095
  open_like.post_open_revalidation.access: count=2601 total_ns=16356571 avg_ns=6288 max_ns=16098
  stat_child_no_follow: count=51487 total_ns=69892137 avg_ns=1357 max_ns=34178
  stat_child_no_follow.attr_conversion: count=48885 total_ns=848230 avg_ns=17 max_ns=86
  stat_child_no_follow.host_fstat: count=48885 total_ns=11182427 avg_ns=228 max_ns=12319
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=69892137 avg_ns=1357 max_ns=34178
  source_root_path: count=48887 total_ns=89197328 avg_ns=1824 max_ns=40246
  resolved_virtual_path: count=48886 total_ns=161313062 avg_ns=3299 max_ns=32760
  resolved_virtual_path_from_path: count=46285 total_ns=158282348 avg_ns=3419 max_ns=32760
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=138868698 avg_ns=3000 max_ns=32235
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=115089507 avg_ns=1099 max_ns=31590
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11695133 avg_ns=111 max_ns=7729
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=16513203 avg_ns=356 max_ns=6744
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3030714 avg_ns=1165 max_ns=6879
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
