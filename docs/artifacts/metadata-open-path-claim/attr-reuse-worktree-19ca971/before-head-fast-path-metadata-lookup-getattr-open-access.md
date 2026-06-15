# ScreenFS benchmark result

- timestamp: `2026-06-15T19:39:53.420561+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-attr-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-attr-171403 --perf-counters --policy-preset fast-path-cache-eligible --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_access --iterations 10 --warmups 3 --metadata-ops 1024 --output-json /tmp/screenfs-attr-reuse-bench-20260616-043944/before-head-fast-path-metadata-lookup-getattr-open-access.json --output-md /tmp/screenfs-attr-reuse-bench-20260616-043944/before-head-fast-path-metadata-lookup-getattr-open-access.md --output-svg /tmp/screenfs-attr-reuse-bench-20260616-043944/before-head-fast-path-metadata-lookup-getattr-open-access.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-attr-target/release/screenfs`
- screenfs_bin_sha256: `68d028b8be904d44afc22679ba331a783ae6e05fc5af6e941fb4cb1e7964ed3a`
- screenfs_source_root: `/tmp/screenfs-before-attr-171403`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `True`
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
| metadata_lookup | 0.005898 | 0.095657 | 16.218 | 0.100560 | 0.100947 | 0.101257 |
| metadata_getattr | 0.006652 | 0.124985 | 18.789 | 0.129408 | 0.130707 | 0.131746 |
| metadata_open | 0.006331 | 0.127302 | 20.106 | 0.130820 | 0.131488 | 0.132022 |
| metadata_access | 0.005904 | 0.124114 | 21.023 | 0.129550 | 0.130635 | 0.131503 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=269475219 avg_ns=20241 max_ns=64252
  fuse_op.flush: count=13312 total_ns=82790884 avg_ns=6219 max_ns=190130
  fuse_op.getattr: count=13313 total_ns=263689296 avg_ns=19806 max_ns=52109
  fuse_op.lookup: count=159749 total_ns=3233967158 avg_ns=20244 max_ns=28736907
  fuse_op.open: count=13312 total_ns=268763075 avg_ns=20189 max_ns=204918
  fuse_op.release: count=13312 total_ns=11844332 avg_ns=889 max_ns=7324
  fuse_op.statfs: count=2 total_ns=3808 avg_ns=1904 max_ns=2195
  policy_decision: count=599060 total_ns=180088739 avg_ns=300 max_ns=261776
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=599060 total_ns=67075802 avg_ns=111 max_ns=17303
  matcher_candidate_order.path: count=1797180 total_ns=206642465 avg_ns=114 max_ns=9949
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=6176932
  matcher_candidate_order_ancestor_steps.descendant: count=1544233
  matcher_candidate_order_ancestor_steps.path: count=4632699
  state_read_lock_wait: count=212999 total_ns=5105973 avg_ns=23 max_ns=5511
  state_read_lock_hold: count=212999 total_ns=22174820 avg_ns=104 max_ns=8466
  state_write_lock_wait: count=173059 total_ns=4211799 avg_ns=24 max_ns=7875
  state_write_lock_hold: count=173059 total_ns=85971182 avg_ns=496 max_ns=43142
  open_confined_openat2: count=399374 total_ns=457918909 avg_ns=1146 max_ns=28703984
  source_root_path: count=399373 total_ns=1338970497 avg_ns=3352 max_ns=207964
  resolved_virtual_path: count=399373 total_ns=723424663 avg_ns=1811 max_ns=176555
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=399373 total_ns=723424663 avg_ns=1811 max_ns=176555
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=78923874 avg_ns=5928 max_ns=189819
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
