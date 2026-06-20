# ScreenFS benchmark result

- timestamp: `2026-06-20T07:37:11.735975+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-symlink-surface --iterations 3 --warmups 1 --dir-entries 200 --output-json docs/artifacts/current-directory-symlink-surface-smoke.json --output-md docs/artifacts/current-directory-symlink-surface-smoke.md --output-svg docs/artifacts/current-directory-symlink-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `459757d51c2497af9800fd465a5579433f31f4334d19dce3152a804e8a63dde4`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+20 more)`
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
| readdir_symlink_visibility | 0.000104 | 0.002415 | 23.327 | 0.002469 | 0.002476 | 0.002481 |
| readdirplus_symlink_visibility | 0.000323 | 0.015621 | 48.294 | 0.015631 | 0.015632 | 0.015633 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=68075 avg_ns=68075 max_ns=68075
  fuse_op.getattr: count=817 total_ns=12416063 avg_ns=15197 max_ns=65451
  fuse_op.lookup: count=2445 total_ns=27090677 avg_ns=11080 max_ns=77092
  fuse_op.opendir: count=8 total_ns=164564 avg_ns=20570 max_ns=44342
  fuse_op.readdir: count=15 total_ns=1343976 avg_ns=89598 max_ns=246577
  fuse_op.readdirplus: count=9 total_ns=15061294 avg_ns=1673477 max_ns=2627896
  fuse_op.releasedir: count=8 total_ns=170457 avg_ns=21307 max_ns=34418
  fuse_op.statfs: count=2 total_ns=1767 avg_ns=883 max_ns=990
  policy_decision: count=11574 total_ns=4498357 avg_ns=388 max_ns=26651
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=11574 total_ns=1345537 avg_ns=116 max_ns=493
  matcher_candidate_order.path: count=34722 total_ns=5094924 avg_ns=146 max_ns=16251
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=11574
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=11574
  matcher_candidate_order_ancestor_steps: count=151928
  matcher_candidate_order_ancestor_steps.descendant: count=37982
  matcher_candidate_order_ancestor_steps.path: count=113946
  state_read_lock_wait: count=3295 total_ns=60399 avg_ns=18 max_ns=330
  state_read_lock_hold: count=3295 total_ns=195597 avg_ns=59 max_ns=2676
  state_write_lock_wait: count=2491 total_ns=43279 avg_ns=17 max_ns=174
  state_write_lock_hold: count=2491 total_ns=1938674 avg_ns=778 max_ns=329061
  open_confined_openat2: count=3320 total_ns=1446239 avg_ns=435 max_ns=11183
  stat_child_no_follow: count=3287 total_ns=19950975 avg_ns=6069 max_ns=58124
  source_root_path: count=3335 total_ns=4316648 avg_ns=1294 max_ns=23259
  resolved_virtual_path: count=11562 total_ns=24120611 avg_ns=2086 max_ns=52130
  resolved_virtual_path_from_path: count=8243 total_ns=21388426 avg_ns=2594 max_ns=52130
  resolved_virtual_path_from_path_component_walk: count=8243 total_ns=18607912 avg_ns=2257 max_ns=51745
  resolved_virtual_path_from_path_canonicalize: count=16454 total_ns=14999671 avg_ns=911 max_ns=50985
  resolved_virtual_path_from_path_source_root_confinement: count=16454 total_ns=2072285 avg_ns=125 max_ns=851
  resolved_virtual_path_from_path_virtual_conversion: count=8243 total_ns=2388251 avg_ns=289 max_ns=28909
  resolved_virtual_path_from_open_fd: count=3319 total_ns=2732185 avg_ns=823 max_ns=7353
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=15 total_ns=1044406 avg_ns=69627 max_ns=169581
  readdir_attr_generation_scan: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=15 total_ns=466310 avg_ns=31087 max_ns=94936
  readdir_candidate_selection: count=15 total_ns=10474 avg_ns=698 max_ns=3621
  readdir_page_commit: count=15 total_ns=89874 avg_ns=5991 max_ns=28751
  readdirplus_directory_scan: count=9 total_ns=13601101 avg_ns=1511233 max_ns=2214459
  readdirplus_attr_generation_scan: count=9 total_ns=791224 avg_ns=87913 max_ns=113976
  readdirplus_attr_generation_entries: count=1616
  readdirplus_symlink_visibility: count=9 total_ns=9458682 avg_ns=1050964 max_ns=1548287
  readdirplus_candidate_selection: count=9 total_ns=235376 avg_ns=26152 max_ns=49054
  readdirplus_page_commit: count=9 total_ns=974775 avg_ns=108308 max_ns=329200
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
