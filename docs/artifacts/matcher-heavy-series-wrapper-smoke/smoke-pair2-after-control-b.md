# ScreenFS benchmark result

- timestamp: `2026-06-24T07:48:57.329132+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 1 --warmups 1 --dir-entries 10 --metadata-ops 5 --matcher-extra-rules 1 --matcher-misses 5 --policy-preset fallback-unsafe-policy --policy-label series-smoke --workload-set policy-heavy-matrix --output-json /tmp/screenfs-series-smoke/smoke-pair2-after-control-b.json --output-md /tmp/screenfs-series-smoke/smoke-pair2-after-control-b.md --output-svg /tmp/screenfs-series-smoke/smoke-pair2-after-control-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+25 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+25 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `series-smoke`
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
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.000046 | 0.000282 | 6.084 | 0.000282 | 0.000282 | 0.000282 |
| metadata_getattr | 0.000046 | 0.000325 | 7.023 | 0.000325 | 0.000325 | 0.000325 |
| metadata_access | 0.000041 | 0.000342 | 8.371 | 0.000342 | 0.000342 | 0.000342 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.000311 | 0.000311 | 0.000311 | 0.000311 |
| matcher_readonly_access_wok | 0.000507 | 0.000507 | 0.000507 | 0.000507 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=21 total_ns=428178 avg_ns=20389 max_ns=73839
  fuse_op.getattr: count=13 total_ns=172625 avg_ns=13278 max_ns=26780
  fuse_op.lookup: count=173 total_ns=1946778 avg_ns=11253 max_ns=68257
  fuse_op.statfs: count=2 total_ns=4973 avg_ns=2486 max_ns=3099
  policy_decision: count=238 total_ns=181502 avg_ns=762 max_ns=2717
  matcher_candidates: count=97
  matcher_candidates_by_source.hidden.path: count=10
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=10
  matcher_candidates_by_source.visible.descendant: count=77
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=97
  matcher_candidate_order.descendant: count=228 total_ns=82123 avg_ns=360 max_ns=1475
  matcher_candidate_order.path: count=704 total_ns=219368 avg_ns=311 max_ns=6774
  matcher_candidate_order_by_source.hidden.path: count=228 total_ns=94480 avg_ns=414 max_ns=1539
  matcher_candidate_order_by_source.internal_hidden.path: count=228 total_ns=57498 avg_ns=252 max_ns=6774
  matcher_candidate_order_by_source.readonly.path: count=10 total_ns=4763 avg_ns=476 max_ns=996
  matcher_candidate_order_by_source.visible.descendant: count=228 total_ns=82123 avg_ns=360 max_ns=1475
  matcher_candidate_order_by_source.visible.path: count=228 total_ns=60771 avg_ns=266 max_ns=875
  matcher_candidate_order_by_source.writable.path: count=10 total_ns=1856 avg_ns=185 max_ns=213
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1160
  matcher_candidate_order_seen_slots.descendant: count=228
  matcher_candidate_order_seen_slots.path: count=932
  matcher_candidate_order_ancestor_steps: count=3208
  matcher_candidate_order_ancestor_steps.descendant: count=777
  matcher_candidate_order_ancestor_steps.path: count=2431
  state_read_lock_wait: count=207 total_ns=5302 avg_ns=25 max_ns=238
  state_read_lock_hold: count=207 total_ns=16952 avg_ns=81 max_ns=1190
  state_write_lock_wait: count=151 total_ns=3739 avg_ns=24 max_ns=158
  state_write_lock_hold: count=151 total_ns=47861 avg_ns=316 max_ns=2263
  open_confined_openat2: count=218 total_ns=226989 avg_ns=1041 max_ns=11812
  open_like.pre_open_guard.access: count=21 total_ns=350328 avg_ns=16682 max_ns=54555
  open_like.post_open_revalidation.access: count=11 total_ns=49515 avg_ns=4501 max_ns=7850
  stat_child_no_follow: count=207 total_ns=360589 avg_ns=1741 max_ns=18898
  stat_child_no_follow.attr_conversion: count=195 total_ns=3499 avg_ns=17 max_ns=32
  stat_child_no_follow.host_fstat: count=195 total_ns=50284 avg_ns=257 max_ns=2402
  stat_child_no_follow_context.path_guard_or_metadata: count=207 total_ns=360589 avg_ns=1741 max_ns=18898
  source_root_path: count=197 total_ns=499738 avg_ns=2536 max_ns=26501
  resolved_virtual_path: count=196 total_ns=687899 avg_ns=3509 max_ns=10133
  resolved_virtual_path_from_path: count=185 total_ns=670533 avg_ns=3624 max_ns=10133
  resolved_virtual_path_from_path_component_walk: count=185 total_ns=575304 avg_ns=3109 max_ns=8828
  resolved_virtual_path_from_path_canonicalize: count=417 total_ns=473212 avg_ns=1134 max_ns=7230
  resolved_virtual_path_from_path_source_root_confinement: count=417 total_ns=50041 avg_ns=120 max_ns=957
  resolved_virtual_path_from_path_virtual_conversion: count=185 total_ns=83043 avg_ns=448 max_ns=1482
  resolved_virtual_path_from_open_fd: count=11 total_ns=17366 avg_ns=1578 max_ns=4085
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
