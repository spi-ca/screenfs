# ScreenFS benchmark result

- timestamp: `2026-06-15T09:28:56.501111+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --iterations 10 --warmups 3 --perf-counters --small-files 1024 --dir-entries 2048 --metadata-ops 1024 --sync-ops 64 --hidden-misses 512 --matcher-misses 512 --symlink-parent-mutations 64 --policy-preset fast-path-cache-eligible --workload-set metadata-open-path --output-json docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+84 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `1b9b5de4a9f56ef6ff65a27c0a01f690a922ab5d9e75de913edf39db4ee4f94d`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+84 more)`
- screenfs_source_git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `metadata-open-path`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.007171 | 0.064550 | 9.002 | 0.065451 | 0.065487 | 0.065516 |
| metadata_getattr | 0.008821 | 0.083709 | 9.490 | 0.087056 | 0.087566 | 0.087973 |
| metadata_open | 0.007297 | 0.092995 | 12.745 | 0.094316 | 0.094704 | 0.095015 |
| metadata_readlink | 0.000671 | 0.076775 | 114.416 | 0.078417 | 0.078671 | 0.078873 |
| metadata_access | 0.006774 | 0.084126 | 12.419 | 0.086638 | 0.087765 | 0.088667 |
| metadata_statfs | 0.000803 | 0.005425 | 6.758 | 0.005481 | 0.005492 | 0.005500 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=183966984 avg_ns=13818 max_ns=97721
  fuse_op.flush: count=13312 total_ns=68447623 avg_ns=5141 max_ns=295361
  fuse_op.getattr: count=13313 total_ns=178819899 avg_ns=13431 max_ns=50642
  fuse_op.lookup: count=199685 total_ns=2756124207 avg_ns=13802 max_ns=18148213
  fuse_op.open: count=13312 total_ns=192707676 avg_ns=14476 max_ns=69662
  fuse_op.readlink: count=13312 total_ns=206546909 avg_ns=15515 max_ns=317804
  fuse_op.release: count=13312 total_ns=13661957 avg_ns=1026 max_ns=6932
  fuse_op.statfs: count=13314 total_ns=3413601 avg_ns=256 max_ns=5164
  policy_decision: count=772116 total_ns=231891141 avg_ns=300 max_ns=301158
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=772116 total_ns=87382950 avg_ns=113 max_ns=19339
  matcher_candidate_order.path: count=2316348 total_ns=269982093 avg_ns=116 max_ns=15191
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8040612
  matcher_candidate_order_ancestor_steps.descendant: count=2010153
  matcher_candidate_order_ancestor_steps.path: count=6030459
  state_read_lock_wait: count=266247 total_ns=6477754 avg_ns=24 max_ns=2112
  state_read_lock_hold: count=266247 total_ns=20416274 avg_ns=76 max_ns=9374
  state_write_lock_wait: count=212995 total_ns=5069169 avg_ns=23 max_ns=5758
  state_write_lock_hold: count=212995 total_ns=105006692 avg_ns=493 max_ns=69295
  open_confined_openat2: count=505870 total_ns=342631706 avg_ns=677 max_ns=18098339
  source_root_path: count=505869 total_ns=822859305 avg_ns=1626 max_ns=49197
  resolved_virtual_path: count=505869 total_ns=550300383 avg_ns=1087 max_ns=50325
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=505869 total_ns=550300383 avg_ns=1087 max_ns=50325
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=62162434 avg_ns=4669 max_ns=291728
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
