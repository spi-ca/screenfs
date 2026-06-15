# ScreenFS benchmark result

- timestamp: `2026-06-15T19:40:02.024401+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-attr-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fast-path-cache-eligible --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_access --iterations 10 --warmups 3 --metadata-ops 1024 --output-json /tmp/screenfs-attr-reuse-bench-20260616-043944/after-worktree-fast-path-metadata-lookup-getattr-open-access.json --output-md /tmp/screenfs-attr-reuse-bench-20260616-043944/after-worktree-fast-path-metadata-lookup-getattr-open-access.md --output-svg /tmp/screenfs-attr-reuse-bench-20260616-043944/after-worktree-fast-path-metadata-lookup-getattr-open-access.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-attr-target/release/screenfs`
- screenfs_bin_sha256: `bf9302067d6b5396e155eb056f13eb69c150c95fb14e6408e94d7d17706e7877`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+8 more)`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.006173 | 0.098624 | 15.977 | 0.099064 | 0.100217 | 0.101139 |
| metadata_getattr | 0.007009 | 0.107563 | 15.346 | 0.108333 | 0.110236 | 0.111759 |
| metadata_open | 0.006652 | 0.116219 | 17.471 | 0.120689 | 0.121142 | 0.121505 |
| metadata_access | 0.006309 | 0.116529 | 18.471 | 0.117719 | 0.117902 | 0.118047 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=353774770 avg_ns=26573 max_ns=226740
  fuse_op.flush: count=13312 total_ns=85183535 avg_ns=6399 max_ns=73079
  fuse_op.getattr: count=13313 total_ns=229727001 avg_ns=17255 max_ns=32812
  fuse_op.lookup: count=159749 total_ns=2631002059 avg_ns=16469 max_ns=19196264
  fuse_op.open: count=13312 total_ns=346229785 avg_ns=26008 max_ns=65604
  fuse_op.release: count=13312 total_ns=11875627 avg_ns=892 max_ns=8196
  fuse_op.statfs: count=2 total_ns=4188 avg_ns=2094 max_ns=2635
  policy_decision: count=439312 total_ns=146692119 avg_ns=333 max_ns=21301
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=439312 total_ns=55334783 avg_ns=125 max_ns=18976
  matcher_candidate_order.path: count=1317936 total_ns=170146850 avg_ns=129 max_ns=17283
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=4898944
  matcher_candidate_order_ancestor_steps.descendant: count=1224736
  matcher_candidate_order_ancestor_steps.path: count=3674208
  state_read_lock_wait: count=212999 total_ns=5838045 avg_ns=27 max_ns=5901
  state_read_lock_hold: count=212999 total_ns=22330008 avg_ns=104 max_ns=8355
  state_write_lock_wait: count=173059 total_ns=4432010 avg_ns=25 max_ns=6602
  state_write_lock_hold: count=173059 total_ns=81042670 avg_ns=468 max_ns=40727
  open_confined_openat2: count=239626 total_ns=313025639 avg_ns=1306 max_ns=19180594
  stat_child_no_follow: count=213001 total_ns=2856695866 avg_ns=13411 max_ns=19193730
  source_root_path: count=239625 total_ns=836387662 avg_ns=3490 max_ns=202954
  resolved_virtual_path: count=452625 total_ns=1243882818 avg_ns=2748 max_ns=22813
  resolved_virtual_path_from_path: count=213000 total_ns=796006958 avg_ns=3737 max_ns=22643
  resolved_virtual_path_from_path_component_walk: count=213000 total_ns=716680561 avg_ns=3364 max_ns=22092
  resolved_virtual_path_from_path_canonicalize: count=266245 total_ns=648154588 avg_ns=2434 max_ns=21561
  resolved_virtual_path_from_path_source_root_confinement: count=266245 total_ns=35061829 avg_ns=131 max_ns=8637
  resolved_virtual_path_from_path_virtual_conversion: count=213000 total_ns=63211831 avg_ns=296 max_ns=15419
  resolved_virtual_path_from_open_fd: count=239625 total_ns=447875860 avg_ns=1869 max_ns=22813
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=80847159 avg_ns=6073 max_ns=72257
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
