# ScreenFS benchmark result

- timestamp: `2026-06-15T20:19:50.171737+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path.svg`
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
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.005880 | 0.131265 | 22.325 | 0.143272 | 0.146518 | 0.149114 |
| metadata_getattr | 0.006562 | 0.172312 | 26.261 | 0.184042 | 0.187556 | 0.190368 |
| metadata_open | 0.006362 | 0.177651 | 27.922 | 0.179624 | 0.180133 | 0.180540 |
| metadata_readlink | 0.001021 | 0.183640 | 179.855 | 0.193741 | 0.194815 | 0.195674 |
| metadata_access | 0.005987 | 0.180176 | 30.094 | 0.185294 | 0.189207 | 0.192338 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=504793140 avg_ns=37917 max_ns=81645
  fuse_op.flush: count=13312 total_ns=82089401 avg_ns=6166 max_ns=87295
  fuse_op.getattr: count=13313 total_ns=474469017 avg_ns=35639 max_ns=67308
  fuse_op.lookup: count=199685 total_ns=6366422385 avg_ns=31882 max_ns=19538736
  fuse_op.open: count=13312 total_ns=488073832 avg_ns=36664 max_ns=79952
  fuse_op.readlink: count=13312 total_ns=597117520 avg_ns=44855 max_ns=111170
  fuse_op.release: count=13312 total_ns=11807735 avg_ns=886 max_ns=8096
  fuse_op.statfs: count=2 total_ns=4619 avg_ns=2309 max_ns=2665
  policy_decision: count=798740 total_ns=564613901 avg_ns=706 max_ns=19527
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=798740 total_ns=92523906 avg_ns=115 max_ns=9398
  matcher_candidate_order.path: count=2396220 total_ns=641823933 avg_ns=267 max_ns=198777
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=26358420
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=26358420
  matcher_candidate_order_ancestor_steps: count=8466596
  matcher_candidate_order_ancestor_steps.descendant: count=2116649
  matcher_candidate_order_ancestor_steps.path: count=6349947
  state_read_lock_wait: count=266247 total_ns=6524717 avg_ns=24 max_ns=6993
  state_read_lock_hold: count=266247 total_ns=27372171 avg_ns=102 max_ns=8376
  state_write_lock_wait: count=212995 total_ns=5269906 avg_ns=24 max_ns=7134
  state_write_lock_hold: count=212995 total_ns=116225788 avg_ns=545 max_ns=41018
  open_confined_openat2: count=505870 total_ns=561686879 avg_ns=1110 max_ns=19494231
  source_root_path: count=758804 total_ns=2516352012 avg_ns=3316 max_ns=188627
  resolved_virtual_path: count=745490 total_ns=2536148416 avg_ns=3401 max_ns=185642
  resolved_virtual_path_from_path: count=239621 total_ns=1606013548 avg_ns=6702 max_ns=47460
  resolved_virtual_path_from_path_component_walk: count=239621 total_ns=1496761692 avg_ns=6246 max_ns=46739
  resolved_virtual_path_from_path_canonicalize: count=519177 total_ns=1362217526 avg_ns=2623 max_ns=46027
  resolved_virtual_path_from_path_source_root_confinement: count=519177 total_ns=67128381 avg_ns=129 max_ns=18144
  resolved_virtual_path_from_path_virtual_conversion: count=239621 total_ns=90136600 avg_ns=376 max_ns=19998
  resolved_virtual_path_from_open_fd: count=505869 total_ns=930134868 avg_ns=1838 max_ns=185642
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=78000504 avg_ns=5859 max_ns=86364
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
