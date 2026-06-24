# ScreenFS benchmark result

- timestamp: `2026-06-24T05:33:58.764599+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-before-pair3 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair3.json --output-md docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair3.md --output-svg docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair3.svg`
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
- policy_label: `matcher-heavy-no-code-control-before-pair3`
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
| metadata_lookup | 0.001421 | 0.010965 | 7.717 | 0.011251 | 0.011316 | 0.011367 |
| metadata_getattr | 0.001484 | 0.014531 | 9.789 | 0.015252 | 0.015260 | 0.015266 |
| metadata_access | 0.001317 | 0.016044 | 12.186 | 0.017436 | 0.017854 | 0.018189 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.010403 | 0.011563 | 0.011640 | 0.011702 |
| matcher_readonly_access_wok | 0.022578 | 0.024333 | 0.024407 | 0.024466 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=118851702 avg_ns=22851 max_ns=100700
  fuse_op.getattr: count=3017 total_ns=40819972 avg_ns=13529 max_ns=32788
  fuse_op.lookup: count=43269 total_ns=467475608 avg_ns=10803 max_ns=191121
  fuse_op.statfs: count=2 total_ns=8921 avg_ns=4460 max_ns=5427
  policy_decision: count=59288 total_ns=80626695 avg_ns=1359 max_ns=10711
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
  matcher_candidate_order.descendant: count=56688 total_ns=31242499 avg_ns=551 max_ns=36280
  matcher_candidate_order.path: count=175264 total_ns=86740624 avg_ns=494 max_ns=10579
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39448715 avg_ns=695 max_ns=10579
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=8869573 avg_ns=156 max_ns=10060
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2700977 avg_ns=1038 max_ns=2845
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31242499 avg_ns=551 max_ns=36280
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35257809 avg_ns=621 max_ns=8940
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=463550 avg_ns=178 max_ns=929
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1329127 avg_ns=25 max_ns=901
  state_read_lock_hold: count=51487 total_ns=3248819 avg_ns=63 max_ns=3672
  state_write_lock_wait: count=38067 total_ns=882793 avg_ns=23 max_ns=322
  state_write_lock_hold: count=38067 total_ns=13451022 avg_ns=353 max_ns=13107
  open_confined_openat2: count=54088 total_ns=41731349 avg_ns=771 max_ns=25689
  open_like.pre_open_guard.access: count=5201 total_ns=98226006 avg_ns=18885 max_ns=58873
  open_like.post_open_revalidation.access: count=2601 total_ns=16483199 avg_ns=6337 max_ns=15939
  stat_child_no_follow: count=51487 total_ns=70362685 avg_ns=1366 max_ns=44294
  stat_child_no_follow.attr_conversion: count=48885 total_ns=858168 avg_ns=17 max_ns=5481
  stat_child_no_follow.host_fstat: count=48885 total_ns=11498340 avg_ns=235 max_ns=36656
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=70362685 avg_ns=1366 max_ns=44294
  source_root_path: count=48887 total_ns=90178810 avg_ns=1844 max_ns=81682
  resolved_virtual_path: count=48886 total_ns=160546251 avg_ns=3284 max_ns=27519
  resolved_virtual_path_from_path: count=46285 total_ns=157513843 avg_ns=3403 max_ns=27519
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=137787644 avg_ns=2976 max_ns=26795
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=114139307 avg_ns=1090 max_ns=25813
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11833685 avg_ns=113 max_ns=2486
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=16779754 avg_ns=362 max_ns=14083
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3032408 avg_ns=1165 max_ns=10566
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
