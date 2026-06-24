# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:21.458534+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair3-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair3-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair3-after-control-b.svg`
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
| metadata_lookup | 0.001289 | 0.012201 | 9.467 | 0.012591 | 0.013090 | 0.013489 |
| metadata_getattr | 0.001376 | 0.016252 | 11.811 | 0.017925 | 0.018200 | 0.018420 |
| metadata_access | 0.001273 | 0.017216 | 13.527 | 0.018128 | 0.018322 | 0.018477 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.010397 | 0.012057 | 0.012217 | 0.012345 |
| matcher_readonly_access_wok | 0.024851 | 0.026994 | 0.027978 | 0.028766 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=122154396 avg_ns=23486 max_ns=105849
  fuse_op.getattr: count=3017 total_ns=42666486 avg_ns=14142 max_ns=102369
  fuse_op.lookup: count=43269 total_ns=490332735 avg_ns=11332 max_ns=184469
  fuse_op.statfs: count=2 total_ns=2265 avg_ns=1132 max_ns=1217
  policy_decision: count=59288 total_ns=82001557 avg_ns=1383 max_ns=76918
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
  matcher_candidate_order.descendant: count=56688 total_ns=32474298 avg_ns=572 max_ns=28301
  matcher_candidate_order.path: count=175264 total_ns=88246629 avg_ns=503 max_ns=30843
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39825862 avg_ns=702 max_ns=30843
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9219888 avg_ns=162 max_ns=27448
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2857472 avg_ns=1099 max_ns=4489
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=32474298 avg_ns=572 max_ns=28301
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35873487 avg_ns=632 max_ns=26231
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=469920 avg_ns=180 max_ns=1158
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1364984 avg_ns=26 max_ns=1141
  state_read_lock_hold: count=51487 total_ns=3582391 avg_ns=69 max_ns=21799
  state_write_lock_wait: count=38067 total_ns=889560 avg_ns=23 max_ns=784
  state_write_lock_hold: count=38067 total_ns=14848118 avg_ns=390 max_ns=20316
  open_confined_openat2: count=54088 total_ns=48067508 avg_ns=888 max_ns=70082
  open_like.pre_open_guard.access: count=5201 total_ns=101176280 avg_ns=19453 max_ns=105349
  open_like.post_open_revalidation.access: count=2601 total_ns=16659294 avg_ns=6404 max_ns=31708
  stat_child_no_follow: count=51487 total_ns=78523744 avg_ns=1525 max_ns=168584
  stat_child_no_follow.attr_conversion: count=48885 total_ns=868097 avg_ns=17 max_ns=19535
  stat_child_no_follow.host_fstat: count=48885 total_ns=12143632 avg_ns=248 max_ns=166773
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=78523744 avg_ns=1525 max_ns=168584
  source_root_path: count=48887 total_ns=95702688 avg_ns=1957 max_ns=78588
  resolved_virtual_path: count=48886 total_ns=166254295 avg_ns=3400 max_ns=151667
  resolved_virtual_path_from_path: count=46285 total_ns=163098984 avg_ns=3523 max_ns=151667
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=142466580 avg_ns=3078 max_ns=150188
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=117644982 avg_ns=1124 max_ns=95425
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11667066 avg_ns=111 max_ns=14256
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17670228 avg_ns=381 max_ns=37247
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3155311 avg_ns=1213 max_ns=10532
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
