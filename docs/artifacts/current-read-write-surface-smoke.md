# ScreenFS benchmark result

- timestamp: `2026-06-20T08:20:50.514265+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set read-write-surface --iterations 1 --warmups 1 --read-mib 8 --write-mib 8 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/current-read-write-surface-smoke.json --output-md docs/artifacts/current-read-write-surface-smoke.md --output-svg docs/artifacts/current-read-write-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+34 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+34 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.001475 | 0.004929 | 3.341 | 0.004929 | 0.004929 | 0.004929 |
| seq_write | 0.001011 | 0.003346 | 3.311 | 0.003346 | 0.003346 | 0.003346 |
| small_read | 0.000224 | 0.009146 | 40.754 | 0.009146 | 0.009146 | 0.009146 |
| small_write | 0.000355 | 0.024414 | 68.783 | 0.024414 | 0.024414 | 0.024414 |
| rand_read_4k | 0.000437 | 0.018111 | 41.441 | 0.018111 | 0.018111 | 0.018111 |
| rand_write_4k | 0.000459 | 0.026503 | 57.779 | 0.026503 | 0.026503 | 0.026503 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=71170 avg_ns=71170 max_ns=71170
  fuse_op.create: count=6 total_ns=592466 avg_ns=98744 max_ns=108197
  fuse_op.flush: count=6 total_ns=324087 avg_ns=54014 max_ns=116111
  fuse_op.getattr: count=2075 total_ns=24824996 avg_ns=11963 max_ns=39386
  fuse_op.getxattr: count=2066 total_ns=36282627 avg_ns=17561 max_ns=57945
  fuse_op.lookup: count=71 total_ns=958764 avg_ns=13503 max_ns=55525
  fuse_op.open: count=6 total_ns=123391 avg_ns=20565 max_ns=30103
  fuse_op.read: count=1046 total_ns=19901324 avg_ns=19026 max_ns=55722
  fuse_op.release: count=12 total_ns=18513 avg_ns=1542 max_ns=2524
  fuse_op.setattr: count=2 total_ns=72644 avg_ns=36322 max_ns=39789
  fuse_op.statfs: count=2 total_ns=1228 avg_ns=614 max_ns=618
  fuse_op.unlink: count=6 total_ns=977098 avg_ns=162849 max_ns=277166
  fuse_op.write: count=2064 total_ns=50373832 avg_ns=24405 max_ns=201333
  policy_decision: count=28318 total_ns=11770809 avg_ns=415 max_ns=14441
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=24144 total_ns=3007792 avg_ns=124 max_ns=518
  matcher_candidate_order.path: count=80780 total_ns=12799541 avg_ns=158 max_ns=6363
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=28318
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=28318
  matcher_candidate_order_ancestor_steps: count=389232
  matcher_candidate_order_ancestor_steps.descendant: count=88972
  matcher_candidate_order_ancestor_steps.path: count=300260
  state_read_lock_wait: count=7349 total_ns=123850 avg_ns=16 max_ns=239
  state_read_lock_hold: count=7349 total_ns=509622 avg_ns=69 max_ns=1483
  state_write_lock_wait: count=93 total_ns=2059 avg_ns=22 max_ns=266
  state_write_lock_hold: count=93 total_ns=69559 avg_ns=747 max_ns=9808
  open_confined_openat2: count=9480 total_ns=4130934 avg_ns=435 max_ns=10250
  stat_child_no_follow: count=7393 total_ns=50184122 avg_ns=6788 max_ns=44909
  source_root_path: count=17785 total_ns=18793006 avg_ns=1056 max_ns=24088
  resolved_virtual_path: count=27354 total_ns=51531667 avg_ns=1883 max_ns=17572
  resolved_virtual_path_from_path: count=14759 total_ns=41074575 avg_ns=2783 max_ns=17572
  resolved_virtual_path_from_path_component_walk: count=14759 total_ns=35991144 avg_ns=2438 max_ns=17179
  resolved_virtual_path_from_path_canonicalize: count=36692 total_ns=29164774 avg_ns=794 max_ns=16542
  resolved_virtual_path_from_path_source_root_confinement: count=36692 total_ns=3937635 avg_ns=107 max_ns=1095
  resolved_virtual_path_from_path_virtual_conversion: count=14759 total_ns=4394007 avg_ns=297 max_ns=1993
  resolved_virtual_path_from_open_fd: count=12595 total_ns=10457092 avg_ns=830 max_ns=11346
  read_handle_snapshot: count=1046 total_ns=192624 avg_ns=184 max_ns=1015
  read_guard_path: count=1046 total_ns=17144665 avg_ns=16390 max_ns=34869
  read_io: count=1046 total_ns=2388251 avg_ns=2283 max_ns=35618
  write_handle_snapshot: count=2064 total_ns=390784 avg_ns=189 max_ns=1224
  write_guard_mutation: count=2064 total_ns=45739693 avg_ns=22160 max_ns=68438
  write_io: count=2064 total_ns=3906969 avg_ns=1892 max_ns=164072
  file_sync.flush: count=6 total_ns=319324 avg_ns=53220 max_ns=115515
  read_size_bucket.0_4k: count=882 total_ns=704762 avg_ns=799 max_ns=2603
  read_size_bucket.4k_64k: count=26 total_ns=85232 avg_ns=3278 max_ns=12516
  read_size_bucket.64k_1m: count=138 total_ns=1598257 avg_ns=11581 max_ns=35618
  write_size_bucket.0_4k: count=2048 total_ns=1802609 avg_ns=880 max_ns=8561
  write_size_bucket.64k_1m: count=16 total_ns=2104360 avg_ns=131522 max_ns=164072
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
  invalidations: count=12 invalidated_entries=6 evicted_entries=0 scanned_entries=36
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
