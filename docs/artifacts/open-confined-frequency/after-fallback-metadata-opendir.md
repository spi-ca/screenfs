# ScreenFS benchmark result

- timestamp: `2026-06-20T13:16:09.344074+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.json --output-md docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.md --output-svg docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `4108fc6104e76aa03b2eb6c4c7b2c9fa63c0e4c5be3d0b8501bbd28f49771ccb`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+11 more)`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_opendir | 0.000722 | 0.025559 | 35.414 | 0.026735 | 0.027040 | 0.027284 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=40984 avg_ns=40984 max_ns=40984
  fuse_op.getattr: count=6657 total_ns=59653881 avg_ns=8961 max_ns=101317
  fuse_op.lookup: count=13317 total_ns=105532975 avg_ns=7924 max_ns=79909
  fuse_op.opendir: count=6656 total_ns=53814478 avg_ns=8085 max_ns=288598
  fuse_op.releasedir: count=6656 total_ns=1281548 avg_ns=192 max_ns=7568
  fuse_op.statfs: count=2 total_ns=1766 avg_ns=883 max_ns=1235
  policy_decision: count=53262 total_ns=13203265 avg_ns=247 max_ns=25311
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=53262 total_ns=3856702 avg_ns=72 max_ns=15639
  matcher_candidate_order.path: count=159786 total_ns=15277077 avg_ns=95 max_ns=7781
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=53262
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=53262
  matcher_candidate_order_ancestor_steps: count=505976
  matcher_candidate_order_ancestor_steps.descendant: count=126494
  matcher_candidate_order_ancestor_steps.path: count=379482
  state_read_lock_wait: count=26631 total_ns=788126 avg_ns=29 max_ns=312
  state_read_lock_hold: count=26631 total_ns=1202034 avg_ns=45 max_ns=16253
  state_write_lock_wait: count=26627 total_ns=447961 avg_ns=16 max_ns=274
  state_write_lock_hold: count=26627 total_ns=2723964 avg_ns=102 max_ns=25988
  open_confined_openat2: count=26631 total_ns=9533021 avg_ns=357 max_ns=13695
  stat_child_no_follow: count=19974 total_ns=92706846 avg_ns=4641 max_ns=80016
  source_root_path: count=26631 total_ns=30979325 avg_ns=1163 max_ns=75967
  resolved_virtual_path: count=73234 total_ns=87746980 avg_ns=1198 max_ns=282415
  resolved_virtual_path_from_path: count=46603 total_ns=68269988 avg_ns=1464 max_ns=282415
  resolved_virtual_path_from_path_component_walk: count=46603 total_ns=55254472 avg_ns=1185 max_ns=65498
  resolved_virtual_path_from_path_canonicalize: count=59918 total_ns=43739504 avg_ns=729 max_ns=48289
  resolved_virtual_path_from_path_source_root_confinement: count=59918 total_ns=6288413 avg_ns=104 max_ns=5420
  resolved_virtual_path_from_path_virtual_conversion: count=46603 total_ns=10578750 avg_ns=226 max_ns=5571
  resolved_virtual_path_from_open_fd: count=26631 total_ns=19476992 avg_ns=731 max_ns=64342
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
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
