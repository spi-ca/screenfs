# ScreenFS benchmark result

- timestamp: `2026-06-20T12:41:04.521855+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+8 more)`
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
| metadata_open | 0.005087 | 0.034296 | 6.742 | 0.037764 | 0.037940 | 0.038081 |
| metadata_opendir | 0.001091 | 0.029568 | 27.109 | 0.030625 | 0.031184 | 0.031632 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=99073 avg_ns=99073 max_ns=99073
  fuse_op.getattr: count=6657 total_ns=63619313 avg_ns=9556 max_ns=295764
  fuse_op.lookup: count=33285 total_ns=373743731 avg_ns=11228 max_ns=20132177
  fuse_op.open: count=6656 total_ns=106399292 avg_ns=15985 max_ns=106458
  fuse_op.opendir: count=6656 total_ns=83439059 avg_ns=12535 max_ns=287695
  fuse_op.release: count=6656 total_ns=3463083 avg_ns=520 max_ns=2764
  fuse_op.releasedir: count=6656 total_ns=1477920 avg_ns=222 max_ns=7289
  fuse_op.statfs: count=2 total_ns=2880 avg_ns=1440 max_ns=2041
  policy_decision: count=119822 total_ns=35098649 avg_ns=292 max_ns=50882
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=10318504 avg_ns=86 max_ns=2765
  matcher_candidate_order.path: count=359466 total_ns=41090126 avg_ns=114 max_ns=75088
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=119822
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=119822
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=1600116 avg_ns=30 max_ns=475
  state_read_lock_hold: count=53255 total_ns=2799495 avg_ns=52 max_ns=1983
  state_write_lock_wait: count=59907 total_ns=1024111 avg_ns=17 max_ns=779
  state_write_lock_hold: count=59907 total_ns=13358548 avg_ns=222 max_ns=58782
  open_confined_openat2: count=66568 total_ns=78694556 avg_ns=1182 max_ns=20074636
  stat_child_no_follow: count=53255 total_ns=348056161 avg_ns=6535 max_ns=20117396
  source_root_path: count=53255 total_ns=65994179 avg_ns=1239 max_ns=273076
  resolved_virtual_path: count=173074 total_ns=244360717 avg_ns=1411 max_ns=275396
  resolved_virtual_path_from_path: count=106507 total_ns=188588355 avg_ns=1770 max_ns=71472
  resolved_virtual_path_from_path_component_walk: count=106507 total_ns=157645236 avg_ns=1480 max_ns=71161
  resolved_virtual_path_from_path_canonicalize: count=159758 total_ns=125293902 avg_ns=784 max_ns=70759
  resolved_virtual_path_from_path_source_root_confinement: count=159758 total_ns=17479899 avg_ns=109 max_ns=16404
  resolved_virtual_path_from_path_virtual_conversion: count=106507 total_ns=25898404 avg_ns=243 max_ns=19206
  resolved_virtual_path_from_open_fd: count=66567 total_ns=55772362 avg_ns=837 max_ns=275396
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
