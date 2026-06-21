# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:35.771985+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fast-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fast-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fast-open-confined-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `77d2fc749459704dab16352ddbe5567c35d8ce313519dbc39fdf43a471081280`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_open | 0.001797 | 0.028680 | 15.961 | 0.030925 | 0.031854 | 0.032597 |
| metadata_opendir | 0.000594 | 0.022288 | 37.525 | 0.025306 | 0.025326 | 0.025342 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65560 avg_ns=65560 max_ns=65560
  fuse_op.getattr: count=6657 total_ns=41967028 avg_ns=6304 max_ns=31707
  fuse_op.lookup: count=33285 total_ns=288499412 avg_ns=8667 max_ns=15572715
  fuse_op.open: count=6656 total_ns=80599934 avg_ns=12109 max_ns=639591
  fuse_op.opendir: count=6656 total_ns=61473849 avg_ns=9235 max_ns=46163
  fuse_op.release: count=6656 total_ns=3596628 avg_ns=540 max_ns=3714
  fuse_op.releasedir: count=6656 total_ns=1458003 avg_ns=219 max_ns=2314
  fuse_op.statfs: count=2 total_ns=2775 avg_ns=1387 max_ns=1602
  policy_decision: count=119822 total_ns=26820785 avg_ns=223 max_ns=46196
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=10013460 avg_ns=83 max_ns=4157
  matcher_candidate_order.path: count=359466 total_ns=32337040 avg_ns=89 max_ns=308249
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=920144 avg_ns=17 max_ns=2586
  state_read_lock_hold: count=53255 total_ns=3003875 avg_ns=56 max_ns=3922
  state_write_lock_wait: count=59907 total_ns=1019045 avg_ns=17 max_ns=336
  state_write_lock_hold: count=59907 total_ns=12463102 avg_ns=208 max_ns=37725
  open_confined_openat2: count=66568 total_ns=80379856 avg_ns=1207 max_ns=15535682
  open_like.pre_open_guard.access: count=1 total_ns=23732 avg_ns=23732 max_ns=23732
  open_like.pre_open_guard.open: count=6656 total_ns=56945745 avg_ns=8555 max_ns=327765
  open_like.pre_open_guard.opendir: count=6656 total_ns=41786715 avg_ns=6278 max_ns=29704
  open_like.post_open_revalidation.access: count=1 total_ns=36199 avg_ns=36199 max_ns=36199
  open_like.post_open_revalidation.open: count=6656 total_ns=17185749 avg_ns=2581 max_ns=310580
  open_like.post_open_revalidation.opendir: count=6656 total_ns=13502681 avg_ns=2028 max_ns=6158
  stat_child_no_follow: count=53255 total_ns=337567285 avg_ns=6338 max_ns=15568338
  source_root_path: count=53255 total_ns=63085308 avg_ns=1184 max_ns=27353
  resolved_virtual_path: count=119821 total_ns=118359812 avg_ns=987 max_ns=318142
  resolved_virtual_path_from_path: count=53254 total_ns=64537413 avg_ns=1211 max_ns=8837
  resolved_virtual_path_from_path_component_walk: count=53254 total_ns=50243470 avg_ns=943 max_ns=7710
  resolved_virtual_path_from_path_canonicalize: count=53253 total_ns=39427667 avg_ns=740 max_ns=7257
  resolved_virtual_path_from_path_source_root_confinement: count=53253 total_ns=5813325 avg_ns=109 max_ns=874
  resolved_virtual_path_from_path_virtual_conversion: count=53254 total_ns=11697961 avg_ns=219 max_ns=7528
  resolved_virtual_path_from_open_fd: count=66567 total_ns=53822399 avg_ns=808 max_ns=318142
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
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
