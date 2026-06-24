# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:40.420388+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/before-fast-policy-heavy.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/before-fast-policy-heavy.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/before-fast-policy-heavy.svg --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
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
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
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
| metadata_lookup | 0.001862 | 0.005392 | 2.896 | 0.005780 | 0.005899 | 0.005993 |
| metadata_getattr | 0.001515 | 0.006838 | 4.514 | 0.007278 | 0.007301 | 0.007320 |
| metadata_access | 0.001151 | 0.007480 | 6.501 | 0.008028 | 0.008296 | 0.008510 |

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
  fuse_op.access: count=2601 total_ns=20310437 avg_ns=7808 max_ns=280987
  fuse_op.getattr: count=2601 total_ns=7446188 avg_ns=2862 max_ns=38630
  fuse_op.lookup: count=23405 total_ns=76004857 avg_ns=3247 max_ns=261437
  fuse_op.statfs: count=2 total_ns=1866 avg_ns=933 max_ns=1003
  policy_decision: count=31208 total_ns=10128911 avg_ns=324 max_ns=58499
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=31208 total_ns=4061143 avg_ns=130 max_ns=272997
  matcher_candidate_order.path: count=93624 total_ns=11514948 avg_ns=122 max_ns=16660
  matcher_candidate_order_by_source.hidden.path: count=31208 total_ns=3787177 avg_ns=121 max_ns=920
  matcher_candidate_order_by_source.internal_hidden.path: count=31208 total_ns=3975571 avg_ns=127 max_ns=16660
  matcher_candidate_order_by_source.visible.descendant: count=31208 total_ns=4061143 avg_ns=130 max_ns=272997
  matcher_candidate_order_by_source.visible.path: count=31208 total_ns=3752200 avg_ns=120 max_ns=4407
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=405676
  matcher_candidate_order_ancestor_steps.descendant: count=101419
  matcher_candidate_order_ancestor_steps.path: count=304257
  state_read_lock_wait: count=28607 total_ns=869418 avg_ns=30 max_ns=306
  state_read_lock_hold: count=28607 total_ns=1484959 avg_ns=51 max_ns=6417
  state_write_lock_wait: count=20803 total_ns=365148 avg_ns=17 max_ns=190
  state_write_lock_hold: count=20803 total_ns=5895027 avg_ns=283 max_ns=68577
  open_confined_openat2: count=31208 total_ns=15861335 avg_ns=508 max_ns=49591
  open_like.pre_open_guard.access: count=2601 total_ns=7241293 avg_ns=2784 max_ns=275543
  open_like.post_open_revalidation.access: count=2601 total_ns=10474784 avg_ns=4027 max_ns=42080
  stat_child_no_follow: count=28607 total_ns=26397822 avg_ns=922 max_ns=49764
  stat_child_no_follow.attr_conversion: count=26005 total_ns=357825 avg_ns=13 max_ns=316
  stat_child_no_follow.host_fstat: count=26005 total_ns=3590495 avg_ns=138 max_ns=15057
  stat_child_no_follow_context.path_guard_or_metadata: count=28607 total_ns=26397822 avg_ns=922 max_ns=49764
  source_root_path: count=2601 total_ns=3283792 avg_ns=1262 max_ns=31813
  resolved_virtual_path: count=2601 total_ns=2375402 avg_ns=913 max_ns=14498
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2375402 avg_ns=913 max_ns=14498
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
