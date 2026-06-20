# ScreenFS benchmark result

- timestamp: `2026-06-20T07:34:51.861916+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-symlink-surface --iterations 3 --warmups 1 --dir-entries 200 --output-json docs/artifacts/current-directory-symlink-surface-smoke.json --output-md docs/artifacts/current-directory-symlink-surface-smoke.md --output-svg docs/artifacts/current-directory-symlink-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+17 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-symlink-surface`
- comparable_workloads: `readdir_symlink_visibility, readdirplus_symlink_visibility`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_symlink_visibility | 0.000084 | 0.002280 | 27.095 | 0.002376 | 0.002388 | 0.002398 |
| readdirplus_symlink_visibility | 0.000273 | 0.014358 | 52.526 | 0.014992 | 0.015071 | 0.015135 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=67951 avg_ns=67951 max_ns=67951
  fuse_op.getattr: count=817 total_ns=11644993 avg_ns=14253 max_ns=45085
  fuse_op.lookup: count=2445 total_ns=25961861 avg_ns=10618 max_ns=88460
  fuse_op.opendir: count=8 total_ns=130547 avg_ns=16318 max_ns=31646
  fuse_op.readdir: count=15 total_ns=1203204 avg_ns=80213 max_ns=237761
  fuse_op.readdirplus: count=9 total_ns=16111135 avg_ns=1790126 max_ns=2952895
  fuse_op.releasedir: count=8 total_ns=102840 avg_ns=12855 max_ns=17747
  fuse_op.statfs: count=2 total_ns=6644 avg_ns=3322 max_ns=5653
  policy_decision: count=11574 total_ns=4338780 avg_ns=374 max_ns=17218
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=11574 total_ns=1299641 avg_ns=112 max_ns=4371
  matcher_candidate_order.path: count=34722 total_ns=4869415 avg_ns=140 max_ns=4895
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=11574
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=11574
  matcher_candidate_order_ancestor_steps: count=151928
  matcher_candidate_order_ancestor_steps.descendant: count=37982
  matcher_candidate_order_ancestor_steps.path: count=113946
  state_read_lock_wait: count=3295 total_ns=57876 avg_ns=17 max_ns=240
  state_read_lock_hold: count=3295 total_ns=177699 avg_ns=53 max_ns=3174
  state_write_lock_wait: count=2491 total_ns=41738 avg_ns=16 max_ns=262
  state_write_lock_hold: count=2491 total_ns=1862622 avg_ns=747 max_ns=325413
  open_confined_openat2: count=3320 total_ns=1315636 avg_ns=396 max_ns=9593
  stat_child_no_follow: count=3287 total_ns=18875620 avg_ns=5742 max_ns=85223
  source_root_path: count=4991 total_ns=5863204 avg_ns=1174 max_ns=81347
  resolved_virtual_path: count=11562 total_ns=22801375 avg_ns=1972 max_ns=50313
  resolved_virtual_path_from_path: count=8243 total_ns=20232960 avg_ns=2454 max_ns=50313
  resolved_virtual_path_from_path_component_walk: count=8243 total_ns=17594505 avg_ns=2134 max_ns=49899
  resolved_virtual_path_from_path_canonicalize: count=16454 total_ns=14219180 avg_ns=864 max_ns=49185
  resolved_virtual_path_from_path_source_root_confinement: count=16454 total_ns=1986017 avg_ns=120 max_ns=822
  resolved_virtual_path_from_path_virtual_conversion: count=8243 total_ns=2247832 avg_ns=272 max_ns=1166
  resolved_virtual_path_from_open_fd: count=3319 total_ns=2568415 avg_ns=773 max_ns=8549
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=15 total_ns=958496 avg_ns=63899 max_ns=178112
  readdir_attr_generation_scan: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=15 total_ns=472446 avg_ns=31496 max_ns=82543
  readdir_candidate_selection: count=15 total_ns=10795 avg_ns=719 max_ns=3250
  readdir_page_commit: count=15 total_ns=96222 avg_ns=6414 max_ns=32358
  readdirplus_directory_scan: count=9 total_ns=14661852 avg_ns=1629094 max_ns=2541246
  readdirplus_attr_generation_scan: count=9 total_ns=754484 avg_ns=83831 max_ns=117585
  readdirplus_attr_generation_entries: count=1616
  readdirplus_symlink_visibility: count=9 total_ns=10679299 avg_ns=1186588 max_ns=1866017
  readdirplus_candidate_selection: count=9 total_ns=202415 avg_ns=22490 max_ns=42979
  readdirplus_page_commit: count=9 total_ns=998782 avg_ns=110975 max_ns=325563
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
