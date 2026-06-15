# ScreenFS benchmark result

- timestamp: `2026-06-15T20:19:23.583951+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-guard-context-target/release/screenfs`
- screenfs_bin_sha256: `68d028b8be904d44afc22679ba331a783ae6e05fc5af6e941fb4cb1e7964ed3a`
- screenfs_source_root: `/tmp/screenfs-before-guard-context-19ca971`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.006210 | 0.129422 | 20.840 | 0.130092 | 0.131345 | 0.132347 |
| metadata_getattr | 0.006888 | 0.175433 | 25.469 | 0.180957 | 0.181768 | 0.182416 |
| metadata_open | 0.006648 | 0.180162 | 27.099 | 0.191256 | 0.199107 | 0.205388 |
| metadata_readlink | 0.001062 | 0.165495 | 155.840 | 0.167764 | 0.171048 | 0.173675 |
| metadata_access | 0.006211 | 0.167530 | 26.975 | 0.173702 | 0.173991 | 0.174222 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=451065937 avg_ns=33881 max_ns=69702
  fuse_op.flush: count=13312 total_ns=90991656 avg_ns=6835 max_ns=109527
  fuse_op.getattr: count=13313 total_ns=452998533 avg_ns=34026 max_ns=89609
  fuse_op.lookup: count=199685 total_ns=6077386231 avg_ns=30434 max_ns=9102661
  fuse_op.open: count=13312 total_ns=467993731 avg_ns=35155 max_ns=93286
  fuse_op.readlink: count=13312 total_ns=509639439 avg_ns=38284 max_ns=77537
  fuse_op.release: count=13312 total_ns=15500132 avg_ns=1164 max_ns=20709
  fuse_op.statfs: count=2 total_ns=3638 avg_ns=1819 max_ns=1874
  policy_decision: count=798740 total_ns=333409460 avg_ns=417 max_ns=25358
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=798740 total_ns=93054852 avg_ns=116 max_ns=18565
  matcher_candidate_order.path: count=2396220 total_ns=387755740 avg_ns=161 max_ns=28905
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=798740
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=798740
  matcher_candidate_order_ancestor_steps: count=8466596
  matcher_candidate_order_ancestor_steps.descendant: count=2116649
  matcher_candidate_order_ancestor_steps.path: count=6349947
  state_read_lock_wait: count=266247 total_ns=6585608 avg_ns=24 max_ns=7394
  state_read_lock_hold: count=266247 total_ns=30850465 avg_ns=115 max_ns=14708
  state_write_lock_wait: count=212995 total_ns=5312146 avg_ns=24 max_ns=7424
  state_write_lock_hold: count=212995 total_ns=120530583 avg_ns=565 max_ns=41869
  open_confined_openat2: count=505870 total_ns=531524363 avg_ns=1050 max_ns=9065320
  source_root_path: count=758804 total_ns=2513095605 avg_ns=3311 max_ns=200580
  resolved_virtual_path: count=745490 total_ns=2569430594 avg_ns=3446 max_ns=36318
  resolved_virtual_path_from_path: count=239621 total_ns=1628057705 avg_ns=6794 max_ns=36318
  resolved_virtual_path_from_path_component_walk: count=239621 total_ns=1517471454 avg_ns=6332 max_ns=35527
  resolved_virtual_path_from_path_canonicalize: count=519177 total_ns=1385452522 avg_ns=2668 max_ns=34595
  resolved_virtual_path_from_path_source_root_confinement: count=519177 total_ns=67435328 avg_ns=129 max_ns=10100
  resolved_virtual_path_from_path_virtual_conversion: count=239621 total_ns=92082938 avg_ns=384 max_ns=12724
  resolved_virtual_path_from_open_fd: count=505869 total_ns=941372889 avg_ns=1860 max_ns=32953
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=85863332 avg_ns=6450 max_ns=108796
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
