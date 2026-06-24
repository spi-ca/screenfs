# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:11.215787+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-before-control-a.svg`
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
| metadata_lookup | 0.001601 | 0.011337 | 7.079 | 0.013126 | 0.013412 | 0.013640 |
| metadata_getattr | 0.001597 | 0.015688 | 9.823 | 0.016804 | 0.016982 | 0.017124 |
| metadata_access | 0.001365 | 0.016711 | 12.239 | 0.018195 | 0.018273 | 0.018335 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009517 | 0.009889 | 0.010049 | 0.010176 |
| matcher_readonly_access_wok | 0.022342 | 0.023519 | 0.023971 | 0.024332 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=120652883 avg_ns=23198 max_ns=133260
  fuse_op.getattr: count=3017 total_ns=41872917 avg_ns=13878 max_ns=111149
  fuse_op.lookup: count=43269 total_ns=474790258 avg_ns=10972 max_ns=120184
  fuse_op.statfs: count=2 total_ns=5474 avg_ns=2737 max_ns=3934
  policy_decision: count=59288 total_ns=81277243 avg_ns=1370 max_ns=36704
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
  matcher_candidate_order.descendant: count=56688 total_ns=31459541 avg_ns=554 max_ns=28292
  matcher_candidate_order.path: count=175264 total_ns=87671943 avg_ns=500 max_ns=108187
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39990942 avg_ns=705 max_ns=108187
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=8972440 avg_ns=158 max_ns=6184
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2764557 avg_ns=1063 max_ns=2370
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31459541 avg_ns=554 max_ns=28292
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35466724 avg_ns=625 max_ns=17068
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=477280 avg_ns=183 max_ns=527
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1350468 avg_ns=26 max_ns=1522
  state_read_lock_hold: count=51487 total_ns=3491256 avg_ns=67 max_ns=9070
  state_write_lock_wait: count=38067 total_ns=884411 avg_ns=23 max_ns=350
  state_write_lock_hold: count=38067 total_ns=13855310 avg_ns=363 max_ns=12495
  open_confined_openat2: count=54088 total_ns=43711775 avg_ns=808 max_ns=93912
  open_like.pre_open_guard.access: count=5201 total_ns=99713773 avg_ns=19172 max_ns=133011
  open_like.post_open_revalidation.access: count=2601 total_ns=16702273 avg_ns=6421 max_ns=13223
  stat_child_no_follow: count=51487 total_ns=72236942 avg_ns=1403 max_ns=95277
  stat_child_no_follow.attr_conversion: count=48885 total_ns=864359 avg_ns=17 max_ns=9679
  stat_child_no_follow.host_fstat: count=48885 total_ns=11192964 avg_ns=228 max_ns=6103
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=72236942 avg_ns=1403 max_ns=95277
  source_root_path: count=48887 total_ns=93006500 avg_ns=1902 max_ns=52357
  resolved_virtual_path: count=48886 total_ns=162114269 avg_ns=3316 max_ns=113796
  resolved_virtual_path_from_path: count=46285 total_ns=158965730 avg_ns=3434 max_ns=113796
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=138849351 avg_ns=2999 max_ns=113200
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=115596190 avg_ns=1104 max_ns=112866
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11483077 avg_ns=109 max_ns=38898
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17199165 avg_ns=371 max_ns=10670
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3148539 avg_ns=1210 max_ns=6452
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
