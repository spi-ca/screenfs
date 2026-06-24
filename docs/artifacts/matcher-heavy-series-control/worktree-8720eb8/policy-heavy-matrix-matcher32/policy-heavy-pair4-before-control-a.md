# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:25.671348+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-before-control-a.svg`
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
| metadata_lookup | 0.001798 | 0.011096 | 6.171 | 0.013655 | 0.014725 | 0.015581 |
| metadata_getattr | 0.001575 | 0.017039 | 10.819 | 0.021466 | 0.021737 | 0.021954 |
| metadata_access | 0.001367 | 0.018569 | 13.588 | 0.021432 | 0.021972 | 0.022404 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.008362 | 0.012599 | 0.012975 | 0.013275 |
| matcher_readonly_access_wok | 0.017916 | 0.018710 | 0.018983 | 0.019201 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=108589305 avg_ns=20878 max_ns=102015
  fuse_op.getattr: count=3017 total_ns=42191249 avg_ns=13984 max_ns=89552
  fuse_op.lookup: count=43269 total_ns=436395650 avg_ns=10085 max_ns=278588
  fuse_op.statfs: count=2 total_ns=10975 avg_ns=5487 max_ns=9291
  policy_decision: count=59288 total_ns=73234016 avg_ns=1235 max_ns=44912
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
  matcher_candidate_order.descendant: count=56688 total_ns=28634935 avg_ns=505 max_ns=33152
  matcher_candidate_order.path: count=175264 total_ns=79123046 avg_ns=451 max_ns=63518
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=35986543 avg_ns=634 max_ns=42257
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=8143304 avg_ns=143 max_ns=27898
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2343603 avg_ns=901 max_ns=6670
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=28634935 avg_ns=505 max_ns=33152
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=32277822 avg_ns=569 max_ns=63518
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=371774 avg_ns=142 max_ns=388
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1413455 avg_ns=27 max_ns=1085
  state_read_lock_hold: count=51487 total_ns=3290232 avg_ns=63 max_ns=27204
  state_write_lock_wait: count=38067 total_ns=784405 avg_ns=20 max_ns=378
  state_write_lock_hold: count=38067 total_ns=13371279 avg_ns=351 max_ns=43805
  open_confined_openat2: count=54088 total_ns=40625343 avg_ns=751 max_ns=55791
  open_like.pre_open_guard.access: count=5201 total_ns=86765095 avg_ns=16682 max_ns=90012
  open_like.post_open_revalidation.access: count=2601 total_ns=17250671 avg_ns=6632 max_ns=86659
  stat_child_no_follow: count=51487 total_ns=66623297 avg_ns=1293 max_ns=106084
  stat_child_no_follow.attr_conversion: count=48885 total_ns=763026 avg_ns=15 max_ns=217
  stat_child_no_follow.host_fstat: count=48885 total_ns=9920076 avg_ns=202 max_ns=26179
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=66623297 avg_ns=1293 max_ns=106084
  source_root_path: count=48887 total_ns=83387694 avg_ns=1705 max_ns=71982
  resolved_virtual_path: count=48886 total_ns=148380246 avg_ns=3035 max_ns=267888
  resolved_virtual_path_from_path: count=46285 total_ns=145079588 avg_ns=3134 max_ns=267888
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=126058948 avg_ns=2723 max_ns=171122
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=104579356 avg_ns=999 max_ns=169910
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10941877 avg_ns=104 max_ns=78535
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=16320891 avg_ns=352 max_ns=263786
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3300658 avg_ns=1268 max_ns=80104
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
