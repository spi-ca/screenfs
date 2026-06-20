# ScreenFS benchmark result

- timestamp: `2026-06-20T13:12:31.639507+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.json --output-md docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.md --output-svg docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a404783a0163e8892396f793ab428ddeb12bc92aa38b4b444459b8ed1038d385`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+11 more)`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_open, metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_open | 0.001798 | 0.035286 | 19.629 | 0.036092 | 0.036512 | 0.036849 |
| metadata_opendir | 0.000625 | 0.028406 | 45.431 | 0.029162 | 0.029298 | 0.029406 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=68828 avg_ns=68828 max_ns=68828
  fuse_op.getattr: count=6657 total_ns=58044312 avg_ns=8719 max_ns=24055
  fuse_op.lookup: count=33285 total_ns=356025839 avg_ns=10696 max_ns=12569753
  fuse_op.open: count=6656 total_ns=51355935 avg_ns=7715 max_ns=87979
  fuse_op.opendir: count=6656 total_ns=49302488 avg_ns=7407 max_ns=44459
  fuse_op.release: count=6656 total_ns=7131420 avg_ns=1071 max_ns=41962
  fuse_op.releasedir: count=6656 total_ns=1929165 avg_ns=289 max_ns=9130
  fuse_op.statfs: count=2 total_ns=7808 avg_ns=3904 max_ns=6501
  policy_decision: count=106510 total_ns=30453291 avg_ns=285 max_ns=8288
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=106510 total_ns=11203782 avg_ns=105 max_ns=6492
  matcher_candidate_order.path: count=319530 total_ns=35572598 avg_ns=111 max_ns=8387
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1118328
  matcher_candidate_order_ancestor_steps.descendant: count=279582
  matcher_candidate_order_ancestor_steps.path: count=838746
  state_read_lock_wait: count=53255 total_ns=1213861 avg_ns=22 max_ns=5449
  state_read_lock_hold: count=53255 total_ns=3449753 avg_ns=64 max_ns=1285
  state_write_lock_wait: count=59907 total_ns=1331489 avg_ns=22 max_ns=670
  state_write_lock_hold: count=59907 total_ns=17379017 avg_ns=290 max_ns=52456
  open_confined_openat2: count=53255 total_ns=83374320 avg_ns=1565 max_ns=12547243
  stat_child_no_follow: count=39942 total_ns=327039591 avg_ns=8187 max_ns=12565287
  source_root_path: count=53255 total_ns=94768338 avg_ns=1779 max_ns=62223
  resolved_virtual_path: count=93197 total_ns=111539637 avg_ns=1196 max_ns=31579
  resolved_virtual_path_from_path: count=39942 total_ns=51349449 avg_ns=1285 max_ns=31579
  resolved_virtual_path_from_path_component_walk: count=39942 total_ns=40170950 avg_ns=1005 max_ns=31196
  resolved_virtual_path_from_path_canonicalize: count=33285 total_ns=31056000 avg_ns=933 max_ns=30839
  resolved_virtual_path_from_path_source_root_confinement: count=33285 total_ns=4509117 avg_ns=135 max_ns=7171
  resolved_virtual_path_from_path_virtual_conversion: count=39942 total_ns=8795808 avg_ns=220 max_ns=3350
  resolved_virtual_path_from_open_fd: count=53255 total_ns=60190188 avg_ns=1130 max_ns=24782
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
