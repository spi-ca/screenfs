# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:15.313093+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-after-control-b.svg`
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
| metadata_lookup | 0.001872 | 0.008812 | 4.708 | 0.009404 | 0.009461 | 0.009506 |
| metadata_getattr | 0.001311 | 0.010616 | 8.097 | 0.011372 | 0.011622 | 0.011822 |
| metadata_access | 0.001107 | 0.017886 | 16.164 | 0.020090 | 0.020482 | 0.020795 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.010513 | 0.011954 | 0.012532 | 0.012996 |
| matcher_readonly_access_wok | 0.018208 | 0.020229 | 0.021136 | 0.021862 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=109878974 avg_ns=21126 max_ns=204070
  fuse_op.getattr: count=3017 total_ns=31103678 avg_ns=10309 max_ns=95452
  fuse_op.lookup: count=43269 total_ns=406540694 avg_ns=9395 max_ns=145755
  fuse_op.statfs: count=2 total_ns=5714 avg_ns=2857 max_ns=4326
  policy_decision: count=59288 total_ns=69800329 avg_ns=1177 max_ns=39419
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
  matcher_candidate_order.descendant: count=56688 total_ns=26699498 avg_ns=470 max_ns=45609
  matcher_candidate_order.path: count=175264 total_ns=74900391 avg_ns=427 max_ns=82074
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=34145831 avg_ns=602 max_ns=82074
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7596691 avg_ns=134 max_ns=29992
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2493439 avg_ns=959 max_ns=6975
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=26699498 avg_ns=470 max_ns=45609
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=30273336 avg_ns=534 max_ns=46711
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=391094 avg_ns=150 max_ns=472
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1360930 avg_ns=26 max_ns=525
  state_read_lock_hold: count=51487 total_ns=3164418 avg_ns=61 max_ns=5398
  state_write_lock_wait: count=38067 total_ns=737311 avg_ns=19 max_ns=382
  state_write_lock_hold: count=38067 total_ns=12545037 avg_ns=329 max_ns=18304
  open_confined_openat2: count=54088 total_ns=35587287 avg_ns=657 max_ns=104642
  open_like.pre_open_guard.access: count=5201 total_ns=89097314 avg_ns=17130 max_ns=167098
  open_like.post_open_revalidation.access: count=2601 total_ns=16409036 avg_ns=6308 max_ns=97312
  stat_child_no_follow: count=51487 total_ns=59264305 avg_ns=1151 max_ns=106025
  stat_child_no_follow.attr_conversion: count=48885 total_ns=730814 avg_ns=14 max_ns=2892
  stat_child_no_follow.host_fstat: count=48885 total_ns=8740821 avg_ns=178 max_ns=29066
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=59264305 avg_ns=1151 max_ns=106025
  source_root_path: count=48887 total_ns=74798195 avg_ns=1530 max_ns=77604
  resolved_virtual_path: count=48886 total_ns=139534924 avg_ns=2854 max_ns=156243
  resolved_virtual_path_from_path: count=46285 total_ns=136355161 avg_ns=2945 max_ns=156243
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=118843962 avg_ns=2567 max_ns=155339
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=98605041 avg_ns=942 max_ns=152851
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10303180 avg_ns=98 max_ns=17849
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=15019191 avg_ns=324 max_ns=30617
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3179763 avg_ns=1222 max_ns=31418
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
