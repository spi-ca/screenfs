# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:36.984888+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/before-fallback-no-matcher-policy-heavy.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/before-fallback-no-matcher-policy-heavy.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/before-fallback-no-matcher-policy-heavy.svg --policy-preset fallback-unsafe-policy --policy-label fallback-no-matcher --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `367b50287307eb569df4a3d08e42e6db25bf4e7cd1b9db1e3aa5b8fe2be4e673`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-no-matcher`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `(none)`
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
| metadata_lookup | 0.001016 | 0.007759 | 7.639 | 0.008194 | 0.008248 | 0.008292 |
| metadata_getattr | 0.001123 | 0.010126 | 9.013 | 0.010828 | 0.010840 | 0.010849 |
| metadata_access | 0.000960 | 0.011380 | 11.849 | 0.011483 | 0.011651 | 0.011785 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Skipped workloads

- `matcher_hidden_stat_miss`: matcher_hidden_stat_miss requires --matcher-extra-rules > 0
- `matcher_readonly_access_wok`: matcher_readonly_access_wok requires --matcher-extra-rules > 0

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=31550969 avg_ns=12130 max_ns=90350
  fuse_op.getattr: count=2601 total_ns=20897578 avg_ns=8034 max_ns=58487
  fuse_op.lookup: count=23405 total_ns=171441977 avg_ns=7325 max_ns=98148
  fuse_op.statfs: count=2 total_ns=4618 avg_ns=2309 max_ns=3471
  policy_decision: count=31208 total_ns=12461225 avg_ns=399 max_ns=56623
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=31208 total_ns=3802825 avg_ns=121 max_ns=39930
  matcher_candidate_order.path: count=93624 total_ns=14551557 avg_ns=155 max_ns=27541
  matcher_candidate_order_by_source.hidden.path: count=31208 total_ns=6486190 avg_ns=207 max_ns=27541
  matcher_candidate_order_by_source.internal_hidden.path: count=31208 total_ns=4237064 avg_ns=135 max_ns=4776
  matcher_candidate_order_by_source.visible.descendant: count=31208 total_ns=3802825 avg_ns=121 max_ns=39930
  matcher_candidate_order_by_source.visible.path: count=31208 total_ns=3828303 avg_ns=122 max_ns=2111
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=31208
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=31208
  matcher_candidate_order_ancestor_steps: count=405676
  matcher_candidate_order_ancestor_steps.descendant: count=101419
  matcher_candidate_order_ancestor_steps.path: count=304257
  state_read_lock_wait: count=28607 total_ns=863995 avg_ns=30 max_ns=1279
  state_read_lock_hold: count=28607 total_ns=1562898 avg_ns=54 max_ns=1218
  state_write_lock_wait: count=20803 total_ns=361331 avg_ns=17 max_ns=161
  state_write_lock_hold: count=20803 total_ns=5657149 avg_ns=271 max_ns=25433
  open_confined_openat2: count=31208 total_ns=16965283 avg_ns=543 max_ns=52897
  open_like.pre_open_guard.access: count=2601 total_ns=21127853 avg_ns=8122 max_ns=62215
  open_like.post_open_revalidation.access: count=2601 total_ns=7762686 avg_ns=2984 max_ns=28433
  stat_child_no_follow: count=28607 total_ns=27726467 avg_ns=969 max_ns=80210
  stat_child_no_follow.attr_conversion: count=26005 total_ns=355824 avg_ns=13 max_ns=239
  stat_child_no_follow.host_fstat: count=26005 total_ns=3723655 avg_ns=143 max_ns=13654
  stat_child_no_follow_context.path_guard_or_metadata: count=28607 total_ns=27726467 avg_ns=969 max_ns=80210
  source_root_path: count=28607 total_ns=36079240 avg_ns=1261 max_ns=90605
  resolved_virtual_path: count=28606 total_ns=71074633 avg_ns=2484 max_ns=70946
  resolved_virtual_path_from_path: count=26005 total_ns=68671969 avg_ns=2640 max_ns=70946
  resolved_virtual_path_from_path_component_walk: count=26005 total_ns=59447334 avg_ns=2285 max_ns=70506
  resolved_virtual_path_from_path_canonicalize: count=54609 total_ns=46912843 avg_ns=859 max_ns=57115
  resolved_virtual_path_from_path_source_root_confinement: count=54609 total_ns=8006554 avg_ns=146 max_ns=69717
  resolved_virtual_path_from_path_virtual_conversion: count=26005 total_ns=7893119 avg_ns=303 max_ns=5564
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2402664 avg_ns=923 max_ns=23997
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
