# ScreenFS benchmark result

- timestamp: `2026-06-20T13:16:06.661915+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.svg`
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
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_open | 0.001813 | 0.046021 | 25.388 | 0.053258 | 0.054360 | 0.055241 |
| metadata_opendir | 0.000570 | 0.034090 | 59.822 | 0.034595 | 0.034654 | 0.034701 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=43077 avg_ns=43077 max_ns=43077
  fuse_op.getattr: count=6657 total_ns=80285501 avg_ns=12060 max_ns=40598
  fuse_op.lookup: count=33285 total_ns=463085041 avg_ns=13912 max_ns=19658608
  fuse_op.open: count=6656 total_ns=82603985 avg_ns=12410 max_ns=90511
  fuse_op.opendir: count=6656 total_ns=72574601 avg_ns=10903 max_ns=64256
  fuse_op.release: count=6656 total_ns=5816262 avg_ns=873 max_ns=10524
  fuse_op.releasedir: count=6656 total_ns=2119327 avg_ns=318 max_ns=5702
  fuse_op.statfs: count=2 total_ns=4343 avg_ns=2171 max_ns=2218
  policy_decision: count=106510 total_ns=38167221 avg_ns=358 max_ns=17097
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=106510 total_ns=11333276 avg_ns=106 max_ns=5323
  matcher_candidate_order.path: count=319530 total_ns=44976766 avg_ns=140 max_ns=46985
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=106510
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=106510
  matcher_candidate_order_ancestor_steps: count=1118328
  matcher_candidate_order_ancestor_steps.descendant: count=279582
  matcher_candidate_order_ancestor_steps.path: count=838746
  state_read_lock_wait: count=53255 total_ns=1369680 avg_ns=25 max_ns=433
  state_read_lock_hold: count=53255 total_ns=3524049 avg_ns=66 max_ns=11325
  state_write_lock_wait: count=59907 total_ns=1305071 avg_ns=21 max_ns=535
  state_write_lock_hold: count=59907 total_ns=17781260 avg_ns=296 max_ns=52854
  open_confined_openat2: count=53255 total_ns=95550731 avg_ns=1794 max_ns=19625809
  stat_child_no_follow: count=39942 total_ns=341035814 avg_ns=8538 max_ns=19650442
  source_root_path: count=53255 total_ns=94203449 avg_ns=1768 max_ns=73797
  resolved_virtual_path: count=146450 total_ns=257664830 avg_ns=1759 max_ns=270759
  resolved_virtual_path_from_path: count=93195 total_ns=199738463 avg_ns=2143 max_ns=270759
  resolved_virtual_path_from_path_component_walk: count=93195 total_ns=168068539 avg_ns=1803 max_ns=270265
  resolved_virtual_path_from_path_canonicalize: count=139790 total_ns=134993798 avg_ns=965 max_ns=185215
  resolved_virtual_path_from_path_source_root_confinement: count=139790 total_ns=17008372 avg_ns=121 max_ns=268438
  resolved_virtual_path_from_path_virtual_conversion: count=93195 total_ns=26243871 avg_ns=281 max_ns=11600
  resolved_virtual_path_from_open_fd: count=53255 total_ns=57926367 avg_ns=1087 max_ns=28303
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
