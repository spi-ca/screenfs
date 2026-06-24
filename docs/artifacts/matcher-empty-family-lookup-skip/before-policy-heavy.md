# ScreenFS benchmark result

- timestamp: `2026-06-24T05:20:04.351205+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-before --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-empty-family-before --workload-set policy-heavy-matrix --output-json /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy.json --output-md /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy.md --output-svg /tmp/screenfs-matcher-empty-family/formal/before-policy-heavy.svg`
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
- policy_label: `glob-matcher-heavy-empty-family-before`
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
| metadata_lookup | 0.001540 | 0.011948 | 7.761 | 0.013317 | 0.013441 | 0.013541 |
| metadata_getattr | 0.001540 | 0.015085 | 9.796 | 0.015373 | 0.015538 | 0.015669 |
| metadata_access | 0.001313 | 0.016241 | 12.368 | 0.016916 | 0.017418 | 0.017819 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009938 | 0.010559 | 0.011098 | 0.011529 |
| matcher_readonly_access_wok | 0.018038 | 0.018654 | 0.018752 | 0.018831 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=106274823 avg_ns=20433 max_ns=420337
  fuse_op.getattr: count=3017 total_ns=40144507 avg_ns=13306 max_ns=74596
  fuse_op.lookup: count=43269 total_ns=440389510 avg_ns=10177 max_ns=301141
  fuse_op.statfs: count=2 total_ns=12635 avg_ns=6317 max_ns=11319
  policy_decision: count=59288 total_ns=74198016 avg_ns=1251 max_ns=275506
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
  matcher_candidate_order.descendant: count=56688 total_ns=29098006 avg_ns=513 max_ns=18176
  matcher_candidate_order.path: count=175264 total_ns=79993633 avg_ns=456 max_ns=46805
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=36558757 avg_ns=644 max_ns=30203
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=8318427 avg_ns=146 max_ns=9966
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2385440 avg_ns=917 max_ns=46805
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=29098006 avg_ns=513 max_ns=18176
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=32362456 avg_ns=570 max_ns=12697
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=368553 avg_ns=141 max_ns=4367
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1141736 avg_ns=22 max_ns=1345
  state_read_lock_hold: count=51487 total_ns=3276303 avg_ns=63 max_ns=9689
  state_write_lock_wait: count=38067 total_ns=803144 avg_ns=21 max_ns=396
  state_write_lock_hold: count=38067 total_ns=12822924 avg_ns=336 max_ns=58133
  open_confined_openat2: count=54088 total_ns=39645167 avg_ns=732 max_ns=281876
  open_like.pre_open_guard.access: count=5201 total_ns=85631965 avg_ns=16464 max_ns=420110
  open_like.post_open_revalidation.access: count=2601 total_ns=16593960 avg_ns=6379 max_ns=14576
  stat_child_no_follow: count=51487 total_ns=65540745 avg_ns=1272 max_ns=282982
  stat_child_no_follow.attr_conversion: count=48885 total_ns=793896 avg_ns=16 max_ns=7027
  stat_child_no_follow.host_fstat: count=48885 total_ns=10023599 avg_ns=205 max_ns=10568
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=65540745 avg_ns=1272 max_ns=282982
  source_root_path: count=48887 total_ns=81672130 avg_ns=1670 max_ns=66987
  resolved_virtual_path: count=48886 total_ns=146141554 avg_ns=2989 max_ns=403862
  resolved_virtual_path_from_path: count=46285 total_ns=143132430 avg_ns=3092 max_ns=403862
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=124265237 avg_ns=2684 max_ns=67361
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=103202521 avg_ns=986 max_ns=66732
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10651130 avg_ns=101 max_ns=2513
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=16171686 avg_ns=349 max_ns=399150
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3009124 avg_ns=1156 max_ns=6733
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
