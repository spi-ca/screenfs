# ScreenFS benchmark result

- timestamp: `2026-06-24T05:22:33.964443+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-before --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-empty-family-before-rerun --workload-set policy-heavy-matrix --output-json /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy-rerun.json --output-md /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy-rerun.md --output-svg /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy-rerun.svg`
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
- policy_label: `glob-matcher-heavy-empty-family-before-rerun`
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
| metadata_lookup | 0.001249 | 0.011752 | 9.409 | 0.012778 | 0.012797 | 0.012812 |
| metadata_getattr | 0.001321 | 0.016030 | 12.135 | 0.016688 | 0.016693 | 0.016697 |
| metadata_access | 0.001155 | 0.012325 | 10.673 | 0.017411 | 0.017573 | 0.017703 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007059 | 0.007452 | 0.007475 | 0.007493 |
| matcher_readonly_access_wok | 0.016843 | 0.018315 | 0.019085 | 0.019701 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=97508282 avg_ns=18747 max_ns=299454
  fuse_op.getattr: count=3017 total_ns=40053866 avg_ns=13276 max_ns=62561
  fuse_op.lookup: count=43269 total_ns=407303103 avg_ns=9413 max_ns=353710
  fuse_op.statfs: count=2 total_ns=5220 avg_ns=2610 max_ns=3965
  policy_decision: count=59288 total_ns=68194748 avg_ns=1150 max_ns=19753
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
  matcher_candidate_order.descendant: count=56688 total_ns=27127258 avg_ns=478 max_ns=19364
  matcher_candidate_order.path: count=175264 total_ns=74097015 avg_ns=422 max_ns=277624
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=33880583 avg_ns=597 max_ns=19162
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7582855 avg_ns=133 max_ns=277624
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2340626 avg_ns=900 max_ns=43327
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=27127258 avg_ns=478 max_ns=19364
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=29936662 avg_ns=528 max_ns=7356
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=356289 avg_ns=137 max_ns=422
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1030290 avg_ns=20 max_ns=887
  state_read_lock_hold: count=51487 total_ns=2897259 avg_ns=56 max_ns=2178
  state_write_lock_wait: count=38067 total_ns=736150 avg_ns=19 max_ns=367
  state_write_lock_hold: count=38067 total_ns=11791972 avg_ns=309 max_ns=44871
  open_confined_openat2: count=54088 total_ns=34709222 avg_ns=641 max_ns=53349
  open_like.pre_open_guard.access: count=5201 total_ns=79578432 avg_ns=15300 max_ns=299169
  open_like.post_open_revalidation.access: count=2601 total_ns=14323890 avg_ns=5507 max_ns=37469
  stat_child_no_follow: count=51487 total_ns=57874952 avg_ns=1124 max_ns=53690
  stat_child_no_follow.attr_conversion: count=48885 total_ns=721499 avg_ns=14 max_ns=171
  stat_child_no_follow.host_fstat: count=48885 total_ns=8497365 avg_ns=173 max_ns=11033
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=57874952 avg_ns=1124 max_ns=53690
  source_root_path: count=48887 total_ns=73961547 avg_ns=1512 max_ns=44422
  resolved_virtual_path: count=48886 total_ns=138331313 avg_ns=2829 max_ns=313876
  resolved_virtual_path_from_path: count=46285 total_ns=135765125 avg_ns=2933 max_ns=313876
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=117848086 avg_ns=2546 max_ns=270244
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=97813446 avg_ns=934 max_ns=269826
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10301810 avg_ns=98 max_ns=8447
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=15409286 avg_ns=332 max_ns=312713
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2566188 avg_ns=986 max_ns=20718
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
