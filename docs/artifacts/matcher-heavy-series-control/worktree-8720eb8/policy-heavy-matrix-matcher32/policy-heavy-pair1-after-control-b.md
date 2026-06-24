# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:13.258117+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-after-control-b.svg`
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
| metadata_lookup | 0.001901 | 0.008698 | 4.576 | 0.009653 | 0.009906 | 0.010109 |
| metadata_getattr | 0.001709 | 0.011423 | 6.684 | 0.012337 | 0.012615 | 0.012838 |
| metadata_access | 0.000979 | 0.018066 | 18.453 | 0.018555 | 0.018890 | 0.019158 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.011308 | 0.011459 | 0.011466 | 0.011472 |
| matcher_readonly_access_wok | 0.018119 | 0.018750 | 0.019245 | 0.019641 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=105201205 avg_ns=20227 max_ns=204400
  fuse_op.getattr: count=3017 total_ns=32058081 avg_ns=10625 max_ns=167351
  fuse_op.lookup: count=43269 total_ns=402881777 avg_ns=9311 max_ns=275668
  fuse_op.statfs: count=2 total_ns=1950 avg_ns=975 max_ns=1211
  policy_decision: count=59288 total_ns=69528652 avg_ns=1172 max_ns=111244
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
  matcher_candidate_order.descendant: count=56688 total_ns=26577188 avg_ns=468 max_ns=17220
  matcher_candidate_order.path: count=175264 total_ns=74131753 avg_ns=422 max_ns=263690
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=33840622 avg_ns=596 max_ns=263690
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7524553 avg_ns=132 max_ns=15152
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2370330 avg_ns=911 max_ns=9774
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=26577188 avg_ns=468 max_ns=17220
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=30018413 avg_ns=529 max_ns=20339
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=377835 avg_ns=145 max_ns=1708
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1392893 avg_ns=27 max_ns=7718
  state_read_lock_hold: count=51487 total_ns=3052930 avg_ns=59 max_ns=4858
  state_write_lock_wait: count=38067 total_ns=728393 avg_ns=19 max_ns=2595
  state_write_lock_hold: count=38067 total_ns=12388218 avg_ns=325 max_ns=53218
  open_confined_openat2: count=54088 total_ns=34992492 avg_ns=646 max_ns=29850
  open_like.pre_open_guard.access: count=5201 total_ns=84669138 avg_ns=16279 max_ns=152938
  open_like.post_open_revalidation.access: count=2601 total_ns=16321105 avg_ns=6274 max_ns=43679
  stat_child_no_follow: count=51487 total_ns=58258981 avg_ns=1131 max_ns=44373
  stat_child_no_follow.attr_conversion: count=48885 total_ns=719160 avg_ns=14 max_ns=311
  stat_child_no_follow.host_fstat: count=48885 total_ns=8621752 avg_ns=176 max_ns=33548
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=58258981 avg_ns=1131 max_ns=44373
  source_root_path: count=48887 total_ns=71539402 avg_ns=1463 max_ns=77440
  resolved_virtual_path: count=48886 total_ns=137483410 avg_ns=2812 max_ns=61115
  resolved_virtual_path_from_path: count=46285 total_ns=134378423 avg_ns=2903 max_ns=61115
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=116563412 avg_ns=2518 max_ns=60283
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=96265171 avg_ns=920 max_ns=59629
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10554587 avg_ns=100 max_ns=8179
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=15303478 avg_ns=330 max_ns=8462
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3104987 avg_ns=1193 max_ns=33192
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
