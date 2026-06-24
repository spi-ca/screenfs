# ScreenFS benchmark result

- timestamp: `2026-06-24T07:48:58.256727+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 1 --warmups 1 --dir-entries 10 --metadata-ops 5 --matcher-extra-rules 1 --matcher-misses 5 --policy-preset fallback-unsafe-policy --policy-label series-smoke --workload-set policy-heavy-matrix --output-json /tmp/screenfs-series-smoke/smoke-pair2-before-control-a.json --output-md /tmp/screenfs-series-smoke/smoke-pair2-before-control-a.md --output-svg /tmp/screenfs-series-smoke/smoke-pair2-before-control-a.svg`
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
| metadata_lookup | 0.000070 | 0.000522 | 7.465 | 0.000522 | 0.000522 | 0.000522 |
| metadata_getattr | 0.000056 | 0.000411 | 7.303 | 0.000411 | 0.000411 | 0.000411 |
| metadata_access | 0.000048 | 0.000348 | 7.183 | 0.000348 | 0.000348 | 0.000348 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.000353 | 0.000353 | 0.000353 | 0.000353 |
| matcher_readonly_access_wok | 0.000579 | 0.000579 | 0.000579 | 0.000579 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=21 total_ns=468974 avg_ns=22332 max_ns=105331
  fuse_op.getattr: count=13 total_ns=211185 avg_ns=16245 max_ns=42462
  fuse_op.lookup: count=173 total_ns=2545700 avg_ns=14715 max_ns=302905
  fuse_op.statfs: count=2 total_ns=4348 avg_ns=2174 max_ns=2181
  policy_decision: count=238 total_ns=200806 avg_ns=843 max_ns=3210
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
  matcher_candidate_order.descendant: count=228 total_ns=94077 avg_ns=412 max_ns=2229
  matcher_candidate_order.path: count=704 total_ns=249231 avg_ns=354 max_ns=8041
  matcher_candidate_order_by_source.hidden.path: count=228 total_ns=105878 avg_ns=464 max_ns=2230
  matcher_candidate_order_by_source.internal_hidden.path: count=228 total_ns=70914 avg_ns=311 max_ns=8041
  matcher_candidate_order_by_source.readonly.path: count=10 total_ns=5024 avg_ns=502 max_ns=854
  matcher_candidate_order_by_source.visible.descendant: count=228 total_ns=94077 avg_ns=412 max_ns=2229
  matcher_candidate_order_by_source.visible.path: count=228 total_ns=65620 avg_ns=287 max_ns=1159
  matcher_candidate_order_by_source.writable.path: count=10 total_ns=1795 avg_ns=179 max_ns=198
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1160
  matcher_candidate_order_seen_slots.descendant: count=228
  matcher_candidate_order_seen_slots.path: count=932
  matcher_candidate_order_ancestor_steps: count=3208
  matcher_candidate_order_ancestor_steps.descendant: count=777
  matcher_candidate_order_ancestor_steps.path: count=2431
  state_read_lock_wait: count=207 total_ns=5717 avg_ns=27 max_ns=173
  state_read_lock_hold: count=207 total_ns=21414 avg_ns=103 max_ns=1415
  state_write_lock_wait: count=151 total_ns=3989 avg_ns=26 max_ns=143
  state_write_lock_hold: count=151 total_ns=75030 avg_ns=496 max_ns=19327
  open_confined_openat2: count=218 total_ns=263157 avg_ns=1207 max_ns=15770
  open_like.pre_open_guard.access: count=21 total_ns=386938 avg_ns=18425 max_ns=83026
  open_like.post_open_revalidation.access: count=11 total_ns=54320 avg_ns=4938 max_ns=11351
  stat_child_no_follow: count=207 total_ns=416122 avg_ns=2010 max_ns=21407
  stat_child_no_follow.attr_conversion: count=195 total_ns=3731 avg_ns=19 max_ns=53
  stat_child_no_follow.host_fstat: count=195 total_ns=56665 avg_ns=290 max_ns=2206
  stat_child_no_follow_context.path_guard_or_metadata: count=207 total_ns=416122 avg_ns=2010 max_ns=21407
  source_root_path: count=197 total_ns=846115 avg_ns=4295 max_ns=263597
  resolved_virtual_path: count=196 total_ns=759617 avg_ns=3875 max_ns=15312
  resolved_virtual_path_from_path: count=185 total_ns=738811 avg_ns=3993 max_ns=15312
  resolved_virtual_path_from_path_component_walk: count=185 total_ns=618601 avg_ns=3343 max_ns=12278
  resolved_virtual_path_from_path_canonicalize: count=417 total_ns=500178 avg_ns=1199 max_ns=9271
  resolved_virtual_path_from_path_source_root_confinement: count=417 total_ns=57262 avg_ns=137 max_ns=1595
  resolved_virtual_path_from_path_virtual_conversion: count=185 total_ns=106909 avg_ns=577 max_ns=2897
  resolved_virtual_path_from_open_fd: count=11 total_ns=20806 avg_ns=1891 max_ns=6634
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
