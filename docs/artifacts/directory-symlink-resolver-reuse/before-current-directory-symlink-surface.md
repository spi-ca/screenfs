# ScreenFS benchmark result

- timestamp: `2026-06-20T07:37:50.398748+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-symlink-surface --iterations 10 --warmups 3 --dir-entries 200 --output-json docs/artifacts/directory-symlink-resolver-reuse/before-current-directory-symlink-surface.json --output-md docs/artifacts/directory-symlink-resolver-reuse/before-current-directory-symlink-surface.md --output-svg docs/artifacts/directory-symlink-resolver-reuse/before-current-directory-symlink-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+18 more)`
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
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_symlink_visibility | 0.000060 | 0.002527 | 42.304 | 0.002710 | 0.002733 | 0.002751 |
| readdirplus_symlink_visibility | 0.000199 | 0.014520 | 73.008 | 0.015286 | 0.016361 | 0.017221 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62373 avg_ns=62373 max_ns=62373
  fuse_op.getattr: count=2653 total_ns=37658142 avg_ns=14194 max_ns=51022
  fuse_op.lookup: count=7935 total_ns=83288556 avg_ns=10496 max_ns=123420
  fuse_op.opendir: count=26 total_ns=430904 avg_ns=16573 max_ns=52732
  fuse_op.readdir: count=51 total_ns=3917161 avg_ns=76807 max_ns=198916
  fuse_op.readdirplus: count=27 total_ns=52345440 avg_ns=1938720 max_ns=2329993
  fuse_op.releasedir: count=26 total_ns=406964 avg_ns=15652 max_ns=32876
  fuse_op.statfs: count=2 total_ns=2969 avg_ns=1484 max_ns=1846
  policy_decision: count=37584 total_ns=14084396 avg_ns=374 max_ns=49819
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=37584 total_ns=4227355 avg_ns=112 max_ns=14660
  matcher_candidate_order.path: count=112752 total_ns=15867542 avg_ns=140 max_ns=20688
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=37584
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=37584
  matcher_candidate_order_ancestor_steps: count=493496
  matcher_candidate_order_ancestor_steps.descendant: count=123374
  matcher_candidate_order_ancestor_steps.path: count=370122
  state_read_lock_wait: count=10693 total_ns=183356 avg_ns=17 max_ns=273
  state_read_lock_hold: count=10693 total_ns=705318 avg_ns=65 max_ns=2783
  state_write_lock_wait: count=8089 total_ns=134226 avg_ns=16 max_ns=133
  state_write_lock_hold: count=8089 total_ns=5544516 avg_ns=685 max_ns=255664
  open_confined_openat2: count=10772 total_ns=4289343 avg_ns=398 max_ns=37770
  stat_child_no_follow: count=10667 total_ns=60388395 avg_ns=5661 max_ns=65119
  source_root_path: count=16205 total_ns=18616755 avg_ns=1148 max_ns=63444
  resolved_virtual_path: count=37536 total_ns=74115703 avg_ns=1974 max_ns=118324
  resolved_virtual_path_from_path: count=26765 total_ns=65839599 avg_ns=2459 max_ns=118324
  resolved_virtual_path_from_path_component_walk: count=26765 total_ns=57223635 avg_ns=2138 max_ns=77972
  resolved_virtual_path_from_path_canonicalize: count=53444 total_ns=46133681 avg_ns=863 max_ns=77293
  resolved_virtual_path_from_path_source_root_confinement: count=53444 total_ns=6500335 avg_ns=121 max_ns=26154
  resolved_virtual_path_from_path_virtual_conversion: count=26765 total_ns=7353376 avg_ns=274 max_ns=117345
  resolved_virtual_path_from_open_fd: count=10771 total_ns=8276104 avg_ns=768 max_ns=47526
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=51 total_ns=3168677 avg_ns=62130 max_ns=148923
  readdir_attr_generation_scan: count=51 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=51 total_ns=1570358 avg_ns=30791 max_ns=70192
  readdir_candidate_selection: count=51 total_ns=31589 avg_ns=619 max_ns=2558
  readdir_page_commit: count=51 total_ns=274750 avg_ns=5387 max_ns=25095
  readdirplus_directory_scan: count=27 total_ns=48482708 avg_ns=1795655 max_ns=2145393
  readdirplus_attr_generation_scan: count=27 total_ns=2684351 avg_ns=99420 max_ns=180373
  readdirplus_attr_generation_entries: count=5252
  readdirplus_symlink_visibility: count=27 total_ns=35467597 avg_ns=1313614 max_ns=1543412
  readdirplus_candidate_selection: count=27 total_ns=539567 avg_ns=19983 max_ns=42031
  readdirplus_page_commit: count=27 total_ns=2607848 avg_ns=96586 max_ns=255783
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
