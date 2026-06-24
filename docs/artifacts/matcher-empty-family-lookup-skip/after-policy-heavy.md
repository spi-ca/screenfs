# ScreenFS benchmark result

- timestamp: `2026-06-24T05:21:38.136136+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-path-only-after --workload-set policy-heavy-matrix --output-json /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy.json --output-md /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy.md --output-svg /tmp/screenfs-matcher-empty-family/formal/after-path-only-policy-heavy.svg`
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
- policy_label: `glob-matcher-heavy-path-only-after`
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
| metadata_lookup | 0.001903 | 0.008122 | 4.269 | 0.008893 | 0.008979 | 0.009048 |
| metadata_getattr | 0.001939 | 0.010705 | 5.520 | 0.011830 | 0.012031 | 0.012192 |
| metadata_access | 0.001283 | 0.012091 | 9.424 | 0.012961 | 0.013115 | 0.013237 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007278 | 0.007771 | 0.007918 | 0.008035 |
| matcher_readonly_access_wok | 0.016421 | 0.017014 | 0.017193 | 0.017335 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=89881466 avg_ns=17281 max_ns=288443
  fuse_op.getattr: count=3017 total_ns=30539159 avg_ns=10122 max_ns=285070
  fuse_op.lookup: count=43269 total_ns=343702420 avg_ns=7943 max_ns=280670
  fuse_op.statfs: count=2 total_ns=3578 avg_ns=1789 max_ns=2458
  policy_decision: count=59288 total_ns=60285130 avg_ns=1016 max_ns=262653
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
  matcher_candidate_order.descendant: count=56688 total_ns=22473146 avg_ns=396 max_ns=13988
  matcher_candidate_order.path: count=175264 total_ns=62706267 avg_ns=357 max_ns=51133
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=28653578 avg_ns=505 max_ns=27183
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=5976485 avg_ns=105 max_ns=9015
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2239229 avg_ns=861 max_ns=38284
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=22473146 avg_ns=396 max_ns=13988
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=25499548 avg_ns=449 max_ns=51133
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=337427 avg_ns=129 max_ns=1572
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=888376 avg_ns=17 max_ns=340
  state_read_lock_hold: count=51487 total_ns=2635634 avg_ns=51 max_ns=1237
  state_write_lock_wait: count=38067 total_ns=621100 avg_ns=16 max_ns=224
  state_write_lock_hold: count=38067 total_ns=10068612 avg_ns=264 max_ns=51805
  open_confined_openat2: count=54088 total_ns=26920589 avg_ns=497 max_ns=273630
  open_like.pre_open_guard.access: count=5201 total_ns=74240053 avg_ns=14274 max_ns=282777
  open_like.post_open_revalidation.access: count=2601 total_ns=12599780 avg_ns=4844 max_ns=17288
  stat_child_no_follow: count=51487 total_ns=45307729 avg_ns=879 max_ns=274025
  stat_child_no_follow.attr_conversion: count=48885 total_ns=634125 avg_ns=12 max_ns=3688
  stat_child_no_follow.host_fstat: count=48885 total_ns=6121915 avg_ns=125 max_ns=14941
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=45307729 avg_ns=879 max_ns=274025
  source_root_path: count=48887 total_ns=57935990 avg_ns=1185 max_ns=271994
  resolved_virtual_path: count=48886 total_ns=121027193 avg_ns=2475 max_ns=278160
  resolved_virtual_path_from_path: count=46285 total_ns=118882634 avg_ns=2568 max_ns=278160
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=104094550 avg_ns=2248 max_ns=277761
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=86500088 avg_ns=826 max_ns=272258
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=9471310 avg_ns=90 max_ns=274074
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=12620979 avg_ns=272 max_ns=29067
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2144559 avg_ns=824 max_ns=6444
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
