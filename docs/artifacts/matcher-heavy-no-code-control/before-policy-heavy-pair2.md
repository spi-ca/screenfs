# ScreenFS benchmark result

- timestamp: `2026-06-24T05:33:54.492182+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-before-pair2 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair2.json --output-md docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair2.md --output-svg docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair2.svg`
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
- policy_label: `matcher-heavy-no-code-control-before-pair2`
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
| metadata_lookup | 0.001585 | 0.011067 | 6.984 | 0.011249 | 0.011404 | 0.011529 |
| metadata_getattr | 0.001508 | 0.015336 | 10.170 | 0.016114 | 0.016138 | 0.016158 |
| metadata_access | 0.001325 | 0.015844 | 11.954 | 0.016094 | 0.016095 | 0.016096 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009403 | 0.009802 | 0.009820 | 0.009833 |
| matcher_readonly_access_wok | 0.022420 | 0.024959 | 0.024999 | 0.025031 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=118031959 avg_ns=22694 max_ns=83271
  fuse_op.getattr: count=3017 total_ns=41016763 avg_ns=13595 max_ns=29289
  fuse_op.lookup: count=43269 total_ns=462659161 avg_ns=10692 max_ns=120084
  fuse_op.statfs: count=2 total_ns=23738 avg_ns=11869 max_ns=21383
  policy_decision: count=59288 total_ns=80474667 avg_ns=1357 max_ns=30828
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
  matcher_candidate_order.descendant: count=56688 total_ns=31258109 avg_ns=551 max_ns=21390
  matcher_candidate_order.path: count=175264 total_ns=86915891 avg_ns=495 max_ns=11199
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39011668 avg_ns=688 max_ns=9185
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9375534 avg_ns=165 max_ns=9560
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2719483 avg_ns=1045 max_ns=6615
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31258109 avg_ns=551 max_ns=21390
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35343194 avg_ns=623 max_ns=11199
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=466012 avg_ns=179 max_ns=750
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1351421 avg_ns=26 max_ns=589
  state_read_lock_hold: count=51487 total_ns=3211557 avg_ns=62 max_ns=22636
  state_write_lock_wait: count=38067 total_ns=888103 avg_ns=23 max_ns=404
  state_write_lock_hold: count=38067 total_ns=13539085 avg_ns=355 max_ns=38346
  open_confined_openat2: count=54088 total_ns=40953333 avg_ns=757 max_ns=31354
  open_like.pre_open_guard.access: count=5201 total_ns=97763885 avg_ns=18797 max_ns=67680
  open_like.post_open_revalidation.access: count=2601 total_ns=16323643 avg_ns=6275 max_ns=14784
  stat_child_no_follow: count=51487 total_ns=69283693 avg_ns=1345 max_ns=34739
  stat_child_no_follow.attr_conversion: count=48885 total_ns=855077 avg_ns=17 max_ns=58
  stat_child_no_follow.host_fstat: count=48885 total_ns=11124435 avg_ns=227 max_ns=10553
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=69283693 avg_ns=1345 max_ns=34739
  source_root_path: count=48887 total_ns=86899185 avg_ns=1777 max_ns=41786
  resolved_virtual_path: count=48886 total_ns=159927474 avg_ns=3271 max_ns=18259
  resolved_virtual_path_from_path: count=46285 total_ns=156983958 avg_ns=3391 max_ns=18259
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=137411851 avg_ns=2968 max_ns=17270
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=113724711 avg_ns=1086 max_ns=15999
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11641709 avg_ns=111 max_ns=2883
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=16704457 avg_ns=360 max_ns=9486
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2943516 avg_ns=1131 max_ns=6761
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
