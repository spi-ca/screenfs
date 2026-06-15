# ScreenFS benchmark result

- timestamp: `2026-06-15T20:19:00.299632+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+10 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-guard-context-target/release/screenfs`
- screenfs_bin_sha256: `68d028b8be904d44afc22679ba331a783ae6e05fc5af6e941fb4cb1e7964ed3a`
- screenfs_source_root: `/tmp/screenfs-before-guard-context-19ca971`
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
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.005960 | 0.097431 | 16.348 | 0.100838 | 0.105459 | 0.109156 |
| metadata_getattr | 0.006541 | 0.127716 | 19.527 | 0.135809 | 0.137885 | 0.139545 |
| metadata_open | 0.006350 | 0.128069 | 20.167 | 0.134104 | 0.135082 | 0.135864 |
| metadata_readlink | 0.001047 | 0.111464 | 106.477 | 0.113954 | 0.114019 | 0.114072 |
| metadata_access | 0.005935 | 0.123946 | 20.884 | 0.128308 | 0.128982 | 0.129520 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=265548857 avg_ns=19946 max_ns=95360
  fuse_op.flush: count=13312 total_ns=85547991 avg_ns=6426 max_ns=94259
  fuse_op.getattr: count=13313 total_ns=268357670 avg_ns=20157 max_ns=222772
  fuse_op.lookup: count=199685 total_ns=4007582171 avg_ns=20069 max_ns=21227592
  fuse_op.open: count=13312 total_ns=267630054 avg_ns=20104 max_ns=60514
  fuse_op.readlink: count=13312 total_ns=288535907 avg_ns=21674 max_ns=53351
  fuse_op.release: count=13312 total_ns=13062210 avg_ns=981 max_ns=5060
  fuse_op.statfs: count=2 total_ns=4088 avg_ns=2044 max_ns=2565
  policy_decision: count=772116 total_ns=238214004 avg_ns=308 max_ns=204216
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=772116 total_ns=86991988 avg_ns=112 max_ns=11071
  matcher_candidate_order.path: count=2316348 total_ns=268701037 avg_ns=116 max_ns=12403
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8040612
  matcher_candidate_order_ancestor_steps.descendant: count=2010153
  matcher_candidate_order_ancestor_steps.path: count=6030459
  state_read_lock_wait: count=266247 total_ns=6475928 avg_ns=24 max_ns=69281
  state_read_lock_hold: count=266247 total_ns=30406820 avg_ns=114 max_ns=6753
  state_write_lock_wait: count=212995 total_ns=5186737 avg_ns=24 max_ns=2996
  state_write_lock_hold: count=212995 total_ns=121644204 avg_ns=571 max_ns=185120
  open_confined_openat2: count=505870 total_ns=552775116 avg_ns=1092 max_ns=21203166
  source_root_path: count=505869 total_ns=1671309376 avg_ns=3303 max_ns=202133
  resolved_virtual_path: count=505869 total_ns=912156041 avg_ns=1803 max_ns=180432
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=505869 total_ns=912156041 avg_ns=1803 max_ns=180432
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=81226823 avg_ns=6101 max_ns=93817
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
