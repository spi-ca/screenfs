# ScreenFS benchmark result

- timestamp: `2026-06-15T20:49:52.122995+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target/release/screenfs`
- screenfs_bin_sha256: `9c998fe7a1356552b25afbfdd6be82fd2bf07d2a37ccae50c731d32fa9a777dd`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `False`
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
| metadata_lookup | 0.006188 | 0.103619 | 16.745 | 0.104538 | 0.105697 | 0.106624 |
| metadata_getattr | 0.006803 | 0.138572 | 20.368 | 0.138853 | 0.138893 | 0.138925 |
| metadata_open | 0.006616 | 0.150698 | 22.778 | 0.157718 | 0.157988 | 0.158203 |
| metadata_readlink | 0.001084 | 0.136883 | 126.284 | 0.139774 | 0.142213 | 0.144165 |
| metadata_access | 0.006206 | 0.150494 | 24.249 | 0.152253 | 0.152499 | 0.152696 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=484779249 avg_ns=36413 max_ns=85652
  fuse_op.flush: count=13312 total_ns=88599481 avg_ns=6655 max_ns=101933
  fuse_op.getattr: count=13313 total_ns=365370421 avg_ns=27444 max_ns=58872
  fuse_op.lookup: count=199685 total_ns=4399152101 avg_ns=22030 max_ns=17551064
  fuse_op.open: count=13312 total_ns=485029901 avg_ns=36435 max_ns=80242
  fuse_op.readlink: count=13312 total_ns=442148771 avg_ns=33214 max_ns=69241
  fuse_op.release: count=13312 total_ns=15137601 avg_ns=1137 max_ns=9238
  fuse_op.statfs: count=2 total_ns=4859 avg_ns=2429 max_ns=2765
  policy_decision: count=572430 total_ns=257835323 avg_ns=450 max_ns=22462
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=572430 total_ns=74902388 avg_ns=130 max_ns=7574
  matcher_candidate_order.path: count=1717290 total_ns=289709445 avg_ns=168 max_ns=19547
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=572430
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=572430
  matcher_candidate_order_ancestor_steps: count=6549624
  matcher_candidate_order_ancestor_steps.descendant: count=1637406
  matcher_candidate_order_ancestor_steps.path: count=4912218
  state_read_lock_wait: count=266247 total_ns=6588826 avg_ns=24 max_ns=5761
  state_read_lock_hold: count=266247 total_ns=31045042 avg_ns=116 max_ns=9588
  state_write_lock_wait: count=212995 total_ns=5331468 avg_ns=25 max_ns=7715
  state_write_lock_hold: count=212995 total_ns=118145517 avg_ns=554 max_ns=44725
  open_confined_openat2: count=279560 total_ns=336599493 avg_ns=1204 max_ns=17530946
  stat_child_no_follow: count=252935 total_ns=3192402200 avg_ns=12621 max_ns=17544142
  source_root_path: count=279559 total_ns=1000492567 avg_ns=3578 max_ns=206651
  resolved_virtual_path: count=772114 total_ns=3081998415 avg_ns=3991 max_ns=211821
  resolved_virtual_path_from_path: count=492555 total_ns=2547666574 avg_ns=5172 max_ns=211821
  resolved_virtual_path_from_path_component_walk: count=492555 total_ns=2354251259 avg_ns=4779 max_ns=211129
  resolved_virtual_path_from_path_canonicalize: count=825358 total_ns=2135314831 avg_ns=2587 max_ns=210268
  resolved_virtual_path_from_path_source_root_confinement: count=825358 total_ns=118520620 avg_ns=143 max_ns=9777
  resolved_virtual_path_from_path_virtual_conversion: count=492555 total_ns=158300961 avg_ns=321 max_ns=12203
  resolved_virtual_path_from_open_fd: count=279559 total_ns=534331841 avg_ns=1911 max_ns=23554
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=83516539 avg_ns=6273 max_ns=100299
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
