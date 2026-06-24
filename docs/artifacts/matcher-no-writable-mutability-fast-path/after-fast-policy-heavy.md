# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:42.241230+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/after-fast-policy-heavy.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/after-fast-policy-heavy.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/after-fast-policy-heavy.svg --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
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
| metadata_lookup | 0.001924 | 0.006577 | 3.418 | 0.007285 | 0.007304 | 0.007320 |
| metadata_getattr | 0.002062 | 0.008702 | 4.220 | 0.009268 | 0.009448 | 0.009593 |
| metadata_access | 0.001828 | 0.010070 | 5.509 | 0.010543 | 0.010794 | 0.010995 |

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
  fuse_op.access: count=2601 total_ns=26034670 avg_ns=10009 max_ns=93901
  fuse_op.getattr: count=2601 total_ns=9044855 avg_ns=3477 max_ns=9411
  fuse_op.lookup: count=23405 total_ns=93308844 avg_ns=3986 max_ns=60300
  fuse_op.statfs: count=2 total_ns=1955 avg_ns=977 max_ns=1063
  policy_decision: count=31208 total_ns=11878549 avg_ns=380 max_ns=11050
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=31208 total_ns=4607197 avg_ns=147 max_ns=5700
  matcher_candidate_order.path: count=93624 total_ns=13516386 avg_ns=144 max_ns=11676
  matcher_candidate_order_by_source.hidden.path: count=31208 total_ns=4308592 avg_ns=138 max_ns=4365
  matcher_candidate_order_by_source.internal_hidden.path: count=31208 total_ns=4941624 avg_ns=158 max_ns=11676
  matcher_candidate_order_by_source.visible.descendant: count=31208 total_ns=4607197 avg_ns=147 max_ns=5700
  matcher_candidate_order_by_source.visible.path: count=31208 total_ns=4266170 avg_ns=136 max_ns=1268
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=405676
  matcher_candidate_order_ancestor_steps.descendant: count=101419
  matcher_candidate_order_ancestor_steps.path: count=304257
  state_read_lock_wait: count=28607 total_ns=742138 avg_ns=25 max_ns=1686
  state_read_lock_hold: count=28607 total_ns=1862233 avg_ns=65 max_ns=3229
  state_write_lock_wait: count=20803 total_ns=483284 avg_ns=23 max_ns=9690
  state_write_lock_hold: count=20803 total_ns=7773592 avg_ns=373 max_ns=23356
  open_confined_openat2: count=31208 total_ns=24961784 avg_ns=799 max_ns=34841
  open_like.pre_open_guard.access: count=2601 total_ns=8742888 avg_ns=3361 max_ns=45373
  open_like.post_open_revalidation.access: count=2601 total_ns=13781952 avg_ns=5298 max_ns=40948
  stat_child_no_follow: count=28607 total_ns=40005466 avg_ns=1398 max_ns=43137
  stat_child_no_follow.attr_conversion: count=26005 total_ns=451617 avg_ns=17 max_ns=273
  stat_child_no_follow.host_fstat: count=26005 total_ns=6375378 avg_ns=245 max_ns=27033
  stat_child_no_follow_context.path_guard_or_metadata: count=28607 total_ns=40005466 avg_ns=1398 max_ns=43137
  source_root_path: count=2601 total_ns=5339911 avg_ns=2053 max_ns=31286
  resolved_virtual_path: count=2601 total_ns=3158426 avg_ns=1214 max_ns=10160
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3158426 avg_ns=1214 max_ns=10160
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
