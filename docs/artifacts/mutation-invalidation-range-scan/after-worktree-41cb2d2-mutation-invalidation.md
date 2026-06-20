# ScreenFS benchmark result

- timestamp: `2026-06-20T07:03:02.658012+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set mutation-invalidation --iterations 10 --warmups 3 --symlink-parent-mutations 200 --output-json docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-mutation-invalidation.json --output-md docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-mutation-invalidation.md --output-svg docs/artifacts/mutation-invalidation-range-scan/after-worktree-41cb2d2-mutation-invalidation.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py;  M src/fs/state.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `771ab46bb690a04abbaa48d56040ddc15512426f691ec3a816a11eb3d5568b7d`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py;  M src/fs/state.rs; ... (+11 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.145604 | 0.183083 | 0.192482 | 0.200001 |
| pinned_symlink_parent_mkdir_rmdir | 0.230316 | 0.294370 | 0.296427 | 0.298073 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=68550 avg_ns=68550 max_ns=68550
  fuse_op.create: count=13 total_ns=1654335 avg_ns=127256 max_ns=158890
  fuse_op.flush: count=13 total_ns=709720 avg_ns=54593 max_ns=264116
  fuse_op.getattr: count=10466 total_ns=189623413 avg_ns=18118 max_ns=403989
  fuse_op.getxattr: count=13 total_ns=430416 avg_ns=33108 max_ns=46708
  fuse_op.lookup: count=166756 total_ns=2349642740 avg_ns=14090 max_ns=538990
  fuse_op.mkdir: count=5200 total_ns=514259749 avg_ns=98896 max_ns=430381
  fuse_op.opendir: count=2613 total_ns=54380759 avg_ns=20811 max_ns=113660
  fuse_op.readdirplus: count=2613 total_ns=115942217 avg_ns=44371 max_ns=222875
  fuse_op.readlink: count=23478 total_ns=455748515 avg_ns=19411 max_ns=1069214
  fuse_op.release: count=13 total_ns=12185 avg_ns=937 max_ns=1746
  fuse_op.releasedir: count=2613 total_ns=1941805 avg_ns=743 max_ns=3583
  fuse_op.rmdir: count=5200 total_ns=396159247 avg_ns=76184 max_ns=377800
  fuse_op.statfs: count=2 total_ns=2422 avg_ns=1211 max_ns=1560
  fuse_op.unlink: count=13 total_ns=1145280 avg_ns=88098 max_ns=104840
  fuse_op.write: count=13 total_ns=520635 avg_ns=40048 max_ns=56467
  policy_decision: count=654525 total_ns=311966864 avg_ns=476 max_ns=80287
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=623208 total_ns=86217226 avg_ns=138 max_ns=307179
  matcher_candidate_order.path: count=1932258 total_ns=345533182 avg_ns=178 max_ns=248148
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=654525
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=654525
  matcher_candidate_order_ancestor_steps: count=8971030
  matcher_candidate_order_ancestor_steps.descendant: count=2174891
  matcher_candidate_order_ancestor_steps.path: count=6796139
  state_read_lock_wait: count=216392 total_ns=4536154 avg_ns=20 max_ns=66966
  state_read_lock_hold: count=216392 total_ns=15714940 avg_ns=72 max_ns=59017
  state_write_lock_wait: count=172032 total_ns=3520697 avg_ns=20 max_ns=8878
  state_write_lock_hold: count=172032 total_ns=62040493 avg_ns=360 max_ns=212578
  open_confined_openat2: count=271162 total_ns=145894294 avg_ns=538 max_ns=205483
  stat_child_no_follow: count=255496 total_ns=1870814610 avg_ns=7322 max_ns=1047634
  source_root_path: count=281639 total_ns=418738871 avg_ns=1486 max_ns=517539
  resolved_virtual_path: count=750924 total_ns=1733429054 avg_ns=2308 max_ns=393823
  resolved_virtual_path_from_path: count=479737 total_ns=1467228534 avg_ns=3058 max_ns=393823
  resolved_virtual_path_from_path_component_walk: count=479737 total_ns=1281039668 avg_ns=2670 max_ns=393363
  resolved_virtual_path_from_path_canonicalize: count=1063895 total_ns=1033660531 avg_ns=971 max_ns=392430
  resolved_virtual_path_from_path_source_root_confinement: count=1063895 total_ns=138226186 avg_ns=129 max_ns=85025
  resolved_virtual_path_from_path_virtual_conversion: count=479737 total_ns=160385507 avg_ns=334 max_ns=54593
  resolved_virtual_path_from_open_fd: count=271187 total_ns=266200520 avg_ns=981 max_ns=324645
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=13 total_ns=9879 avg_ns=759 max_ns=1483
  write_guard_mutation: count=13 total_ns=431678 avg_ns=33206 max_ns=39630
  write_io: count=13 total_ns=73453 avg_ns=5650 max_ns=14548
  file_sync.flush: count=13 total_ns=697578 avg_ns=53659 max_ns=262119
  write_size_bucket.0_4k: count=13 total_ns=73453 avg_ns=5650 max_ns=14548
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=2613 total_ns=24543325 avg_ns=9392 max_ns=186231
  readdirplus_attr_generation_scan: count=2613 total_ns=4090914 avg_ns=1565 max_ns=176117
  readdirplus_attr_generation_entries: count=5213
  readdirplus_symlink_visibility: count=2613 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=2613 total_ns=1094766 avg_ns=418 max_ns=43776
  readdirplus_page_commit: count=2613 total_ns=4421037 avg_ns=1691 max_ns=54779
  invalidations: count=10426 invalidated_entries=5213 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
