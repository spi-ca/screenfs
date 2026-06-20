# ScreenFS benchmark result

- timestamp: `2026-06-20T12:41:06.492122+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+9 more)`
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
| metadata_open | 0.004887 | 0.030705 | 6.283 | 0.031455 | 0.031973 | 0.032387 |
| metadata_opendir | 0.000912 | 0.025422 | 27.868 | 0.026268 | 0.027018 | 0.027618 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=82881 avg_ns=82881 max_ns=82881
  fuse_op.getattr: count=6657 total_ns=46725254 avg_ns=7018 max_ns=317426
  fuse_op.lookup: count=33285 total_ns=290230725 avg_ns=8719 max_ns=11870046
  fuse_op.open: count=6656 total_ns=82259994 avg_ns=12358 max_ns=287306
  fuse_op.opendir: count=6656 total_ns=65741690 avg_ns=9877 max_ns=355643
  fuse_op.release: count=6656 total_ns=3435837 avg_ns=516 max_ns=3162
  fuse_op.releasedir: count=6656 total_ns=1534650 avg_ns=230 max_ns=1891
  fuse_op.statfs: count=2 total_ns=2158 avg_ns=1079 max_ns=1202
  policy_decision: count=119822 total_ns=28798624 avg_ns=240 max_ns=299186
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=10526572 avg_ns=87 max_ns=27648
  matcher_candidate_order.path: count=359466 total_ns=34031183 avg_ns=94 max_ns=27468
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=969943 avg_ns=18 max_ns=941
  state_read_lock_hold: count=53255 total_ns=3251270 avg_ns=61 max_ns=18537
  state_write_lock_wait: count=59907 total_ns=1058909 avg_ns=17 max_ns=252
  state_write_lock_hold: count=59907 total_ns=14432914 avg_ns=240 max_ns=281490
  open_confined_openat2: count=66568 total_ns=68255106 avg_ns=1025 max_ns=11842081
  stat_child_no_follow: count=53255 total_ns=340768091 avg_ns=6398 max_ns=11866238
  source_root_path: count=53255 total_ns=67038812 avg_ns=1258 max_ns=64272
  resolved_virtual_path: count=119821 total_ns=125122771 avg_ns=1044 max_ns=272924
  resolved_virtual_path_from_path: count=53254 total_ns=67607323 avg_ns=1269 max_ns=72848
  resolved_virtual_path_from_path_component_walk: count=53254 total_ns=53026486 avg_ns=995 max_ns=72092
  resolved_virtual_path_from_path_canonicalize: count=53253 total_ns=41709799 avg_ns=783 max_ns=71250
  resolved_virtual_path_from_path_source_root_confinement: count=53253 total_ns=6113112 avg_ns=114 max_ns=17178
  resolved_virtual_path_from_path_virtual_conversion: count=53254 total_ns=11935541 avg_ns=224 max_ns=4350
  resolved_virtual_path_from_open_fd: count=66567 total_ns=57515448 avg_ns=864 max_ns=272924
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
