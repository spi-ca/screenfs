# ScreenFS benchmark result

- timestamp: `2026-06-24T05:22:36.230567+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-path-only-after-rerun --workload-set policy-heavy-matrix --output-json /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy-rerun.json --output-md /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy-rerun.md --output-svg /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy-rerun.svg`
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
| metadata_lookup | 0.001296 | 0.010999 | 8.488 | 0.011388 | 0.011404 | 0.011417 |
| metadata_getattr | 0.001357 | 0.014455 | 10.652 | 0.014947 | 0.014990 | 0.015024 |
| metadata_access | 0.001182 | 0.016778 | 14.192 | 0.017224 | 0.017370 | 0.017487 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009947 | 0.010302 | 0.010843 | 0.011277 |
| matcher_readonly_access_wok | 0.022916 | 0.023490 | 0.023591 | 0.023673 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=120183809 avg_ns=23107 max_ns=80030
  fuse_op.getattr: count=3017 total_ns=41133101 avg_ns=13633 max_ns=47440
  fuse_op.lookup: count=43269 total_ns=468817088 avg_ns=10834 max_ns=72918
  fuse_op.statfs: count=2 total_ns=6422 avg_ns=3211 max_ns=4214
  policy_decision: count=59288 total_ns=82076446 avg_ns=1384 max_ns=17825
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
  matcher_candidate_order.descendant: count=56688 total_ns=31954209 avg_ns=563 max_ns=35679
  matcher_candidate_order.path: count=175264 total_ns=87723805 avg_ns=500 max_ns=18683
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39829357 avg_ns=702 max_ns=18683
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9485504 avg_ns=167 max_ns=3428
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2833939 avg_ns=1089 max_ns=10971
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31954209 avg_ns=563 max_ns=35679
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35106832 avg_ns=619 max_ns=7023
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=468173 avg_ns=180 max_ns=765
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1220928 avg_ns=23 max_ns=5382
  state_read_lock_hold: count=51487 total_ns=3254670 avg_ns=63 max_ns=4145
  state_write_lock_wait: count=38067 total_ns=883422 avg_ns=23 max_ns=1178
  state_write_lock_hold: count=38067 total_ns=13265084 avg_ns=348 max_ns=41819
  open_confined_openat2: count=54088 total_ns=41633757 avg_ns=769 max_ns=31026
  open_like.pre_open_guard.access: count=5201 total_ns=99449285 avg_ns=19121 max_ns=66626
  open_like.post_open_revalidation.access: count=2601 total_ns=16646329 avg_ns=6399 max_ns=14788
  stat_child_no_follow: count=51487 total_ns=70258016 avg_ns=1364 max_ns=31114
  stat_child_no_follow.attr_conversion: count=48885 total_ns=857349 avg_ns=17 max_ns=4766
  stat_child_no_follow.host_fstat: count=48885 total_ns=11496641 avg_ns=235 max_ns=9466
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=70258016 avg_ns=1364 max_ns=31114
  source_root_path: count=48887 total_ns=88011890 avg_ns=1800 max_ns=54573
  resolved_virtual_path: count=48886 total_ns=160813364 avg_ns=3289 max_ns=22914
  resolved_virtual_path_from_path: count=46285 total_ns=157808631 avg_ns=3409 max_ns=22914
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=137852995 avg_ns=2978 max_ns=20905
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=114849556 avg_ns=1097 max_ns=19898
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11403987 avg_ns=108 max_ns=5076
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17076495 avg_ns=368 max_ns=18880
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3004733 avg_ns=1155 max_ns=9016
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
