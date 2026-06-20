# ScreenFS benchmark result

- timestamp: `2026-06-20T13:16:08.209708+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.svg`
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
| metadata_open | 0.001652 | 0.029454 | 17.832 | 0.032547 | 0.037897 | 0.042177 |
| metadata_opendir | 0.000604 | 0.022218 | 36.771 | 0.023296 | 0.023384 | 0.023454 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=54031 avg_ns=54031 max_ns=54031
  fuse_op.getattr: count=6657 total_ns=42062546 avg_ns=6318 max_ns=53159
  fuse_op.lookup: count=33285 total_ns=268424729 avg_ns=8064 max_ns=12834213
  fuse_op.open: count=6656 total_ns=80904296 avg_ns=12155 max_ns=94106
  fuse_op.opendir: count=6656 total_ns=60370892 avg_ns=9070 max_ns=75430
  fuse_op.release: count=6656 total_ns=3840567 avg_ns=577 max_ns=55931
  fuse_op.releasedir: count=6656 total_ns=1403079 avg_ns=210 max_ns=3823
  fuse_op.statfs: count=2 total_ns=4879 avg_ns=2439 max_ns=3745
  policy_decision: count=119822 total_ns=27853410 avg_ns=232 max_ns=80995
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=10078830 avg_ns=84 max_ns=4603
  matcher_candidate_order.path: count=359466 total_ns=32564182 avg_ns=90 max_ns=15346
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=930575 avg_ns=17 max_ns=864
  state_read_lock_hold: count=53255 total_ns=2960723 avg_ns=55 max_ns=2647
  state_write_lock_wait: count=59907 total_ns=1056935 avg_ns=17 max_ns=7170
  state_write_lock_hold: count=59907 total_ns=13494361 avg_ns=225 max_ns=330623
  open_confined_openat2: count=66568 total_ns=55984755 avg_ns=841 max_ns=12800168
  stat_child_no_follow: count=53255 total_ns=315908832 avg_ns=5932 max_ns=12829940
  source_root_path: count=53255 total_ns=64057284 avg_ns=1202 max_ns=250941
  resolved_virtual_path: count=119821 total_ns=119466167 avg_ns=997 max_ns=46013
  resolved_virtual_path_from_path: count=53254 total_ns=65001449 avg_ns=1220 max_ns=46013
  resolved_virtual_path_from_path_component_walk: count=53254 total_ns=51011434 avg_ns=957 max_ns=45618
  resolved_virtual_path_from_path_canonicalize: count=53253 total_ns=39512528 avg_ns=741 max_ns=45183
  resolved_virtual_path_from_path_source_root_confinement: count=53253 total_ns=6599908 avg_ns=123 max_ns=3744
  resolved_virtual_path_from_path_virtual_conversion: count=53254 total_ns=11415496 avg_ns=214 max_ns=2800
  resolved_virtual_path_from_open_fd: count=66567 total_ns=54464718 avg_ns=818 max_ns=44850
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
