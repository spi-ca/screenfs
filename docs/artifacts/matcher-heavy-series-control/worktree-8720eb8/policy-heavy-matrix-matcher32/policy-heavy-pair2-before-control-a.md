# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:17.232398+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-before-control-a.svg`
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
| metadata_lookup | 0.001961 | 0.008655 | 4.413 | 0.009072 | 0.009593 | 0.010009 |
| metadata_getattr | 0.002217 | 0.011151 | 5.029 | 0.011753 | 0.011756 | 0.011758 |
| metadata_access | 0.001492 | 0.012469 | 8.360 | 0.013146 | 0.013914 | 0.014529 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007714 | 0.008069 | 0.008387 | 0.008641 |
| matcher_readonly_access_wok | 0.018081 | 0.023486 | 0.023607 | 0.023703 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=95949600 avg_ns=18448 max_ns=124927
  fuse_op.getattr: count=3017 total_ns=31466213 avg_ns=10429 max_ns=50286
  fuse_op.lookup: count=43269 total_ns=365852790 avg_ns=8455 max_ns=672252
  fuse_op.statfs: count=2 total_ns=4043 avg_ns=2021 max_ns=2413
  policy_decision: count=59288 total_ns=64255007 avg_ns=1083 max_ns=35776
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
  matcher_candidate_order.descendant: count=56688 total_ns=23989109 avg_ns=423 max_ns=13503
  matcher_candidate_order.path: count=175264 total_ns=68852255 avg_ns=392 max_ns=259326
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=30830120 avg_ns=543 max_ns=70055
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7107125 avg_ns=125 max_ns=259326
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2526569 avg_ns=971 max_ns=33635
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=23989109 avg_ns=423 max_ns=13503
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=27998528 avg_ns=493 max_ns=70656
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=389913 avg_ns=149 max_ns=512
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1371726 avg_ns=26 max_ns=2491
  state_read_lock_hold: count=51487 total_ns=2885199 avg_ns=56 max_ns=4023
  state_write_lock_wait: count=38067 total_ns=664474 avg_ns=17 max_ns=650
  state_write_lock_hold: count=38067 total_ns=10943866 avg_ns=287 max_ns=10115
  open_confined_openat2: count=54088 total_ns=29037952 avg_ns=536 max_ns=39355
  open_like.pre_open_guard.access: count=5201 total_ns=79706352 avg_ns=15325 max_ns=97863
  open_like.post_open_revalidation.access: count=2601 total_ns=13008863 avg_ns=5001 max_ns=15397
  stat_child_no_follow: count=51487 total_ns=49313082 avg_ns=957 max_ns=340785
  stat_child_no_follow.attr_conversion: count=48885 total_ns=666286 avg_ns=13 max_ns=665
  stat_child_no_follow.host_fstat: count=48885 total_ns=7238061 avg_ns=148 max_ns=339512
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=49313082 avg_ns=957 max_ns=340785
  source_root_path: count=48887 total_ns=61911785 avg_ns=1266 max_ns=35443
  resolved_virtual_path: count=48886 total_ns=128556264 avg_ns=2629 max_ns=324154
  resolved_virtual_path_from_path: count=46285 total_ns=126321518 avg_ns=2729 max_ns=324154
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=109979265 avg_ns=2376 max_ns=323545
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=91173321 avg_ns=871 max_ns=322955
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=9897070 avg_ns=94 max_ns=4437
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=14019898 avg_ns=302 max_ns=101533
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2234746 avg_ns=859 max_ns=7029
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
