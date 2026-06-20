# ScreenFS benchmark result

- timestamp: `2026-06-20T07:02:56.286349+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-before-inval-oy7Yiu/target/release/screenfs --screenfs-source-root /tmp/screenfs-before-inval-oy7Yiu --perf-counters --workload-set mutation-invalidation --iterations 10 --warmups 3 --symlink-parent-mutations 200 --output-json docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-mutation-invalidation.json --output-md docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-mutation-invalidation.md --output-svg docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-mutation-invalidation.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py;  M src/fs/state.rs; ... (+10 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-before-inval-oy7Yiu/target/release/screenfs`
- screenfs_bin_sha256: `a2d35ad0912b5f2a94d8b53c9f20ce38a984dc230c9f5510eadc14fcf928f0ec`
- screenfs_source_root: `/tmp/screenfs-before-inval-oy7Yiu`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `True`
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
| symlink_parent_mkdir_rmdir | 0.141383 | 0.143631 | 0.144126 | 0.144522 |
| pinned_symlink_parent_mkdir_rmdir | 0.207482 | 0.210783 | 0.213492 | 0.215659 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=91624 avg_ns=91624 max_ns=91624
  fuse_op.create: count=13 total_ns=1407259 avg_ns=108250 max_ns=126958
  fuse_op.flush: count=13 total_ns=571230 avg_ns=43940 max_ns=126373
  fuse_op.getattr: count=10466 total_ns=161710235 avg_ns=15451 max_ns=275299
  fuse_op.getxattr: count=13 total_ns=359815 avg_ns=27678 max_ns=38935
  fuse_op.lookup: count=166756 total_ns=2029163191 avg_ns=12168 max_ns=645500
  fuse_op.mkdir: count=5200 total_ns=443612477 avg_ns=85310 max_ns=247807
  fuse_op.opendir: count=2613 total_ns=44393225 avg_ns=16989 max_ns=89575
  fuse_op.readdirplus: count=2613 total_ns=92618560 avg_ns=35445 max_ns=99563
  fuse_op.readlink: count=23478 total_ns=391902332 avg_ns=16692 max_ns=293016
  fuse_op.release: count=13 total_ns=9544 avg_ns=734 max_ns=1178
  fuse_op.releasedir: count=2613 total_ns=1552080 avg_ns=593 max_ns=3263
  fuse_op.rmdir: count=5200 total_ns=349728458 avg_ns=67255 max_ns=166376
  fuse_op.statfs: count=2 total_ns=4383 avg_ns=2191 max_ns=2985
  fuse_op.unlink: count=13 total_ns=991374 avg_ns=76259 max_ns=92490
  fuse_op.write: count=13 total_ns=450617 avg_ns=34662 max_ns=39809
  policy_decision: count=654525 total_ns=277433745 avg_ns=423 max_ns=273586
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=623208 total_ns=76935143 avg_ns=123 max_ns=56114
  matcher_candidate_order.path: count=1932258 total_ns=304312446 avg_ns=157 max_ns=71691
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=654525
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=654525
  matcher_candidate_order_ancestor_steps: count=8971030
  matcher_candidate_order_ancestor_steps.descendant: count=2174891
  matcher_candidate_order_ancestor_steps.path: count=6796139
  state_read_lock_wait: count=216392 total_ns=3784514 avg_ns=17 max_ns=4953
  state_read_lock_hold: count=216392 total_ns=12579800 avg_ns=58 max_ns=4364
  state_write_lock_wait: count=172032 total_ns=2942527 avg_ns=17 max_ns=3485
  state_write_lock_hold: count=172032 total_ns=50821648 avg_ns=295 max_ns=153875
  open_confined_openat2: count=271162 total_ns=114189416 avg_ns=421 max_ns=78838
  stat_child_no_follow: count=255496 total_ns=1581487072 avg_ns=6189 max_ns=353187
  source_root_path: count=281639 total_ns=325910917 avg_ns=1157 max_ns=82252
  resolved_virtual_path: count=750924 total_ns=1517888669 avg_ns=2021 max_ns=348495
  resolved_virtual_path_from_path: count=479737 total_ns=1292505975 avg_ns=2694 max_ns=348495
  resolved_virtual_path_from_path_component_walk: count=479737 total_ns=1129887587 avg_ns=2355 max_ns=348183
  resolved_virtual_path_from_path_canonicalize: count=1063895 total_ns=905008288 avg_ns=850 max_ns=347792
  resolved_virtual_path_from_path_source_root_confinement: count=1063895 total_ns=131109323 avg_ns=123 max_ns=82990
  resolved_virtual_path_from_path_virtual_conversion: count=479737 total_ns=139976927 avg_ns=291 max_ns=43580
  resolved_virtual_path_from_open_fd: count=271187 total_ns=225382694 avg_ns=831 max_ns=298292
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=13 total_ns=7636 avg_ns=587 max_ns=1078
  write_guard_mutation: count=13 total_ns=373873 avg_ns=28759 max_ns=31089
  write_io: count=13 total_ns=65402 avg_ns=5030 max_ns=9060
  file_sync.flush: count=13 total_ns=560615 avg_ns=43124 max_ns=125646
  write_size_bucket.0_4k: count=13 total_ns=65402 avg_ns=5030 max_ns=9060
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=2613 total_ns=19030749 avg_ns=7283 max_ns=67818
  readdirplus_attr_generation_scan: count=2613 total_ns=3096287 avg_ns=1184 max_ns=60297
  readdirplus_attr_generation_entries: count=5213
  readdirplus_symlink_visibility: count=2613 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=2613 total_ns=579887 avg_ns=221 max_ns=7219
  readdirplus_page_commit: count=2613 total_ns=2955611 avg_ns=1131 max_ns=2761
  invalidations: count=10426 invalidated_entries=5213 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
