# ScreenFS benchmark result

- timestamp: `2026-06-14T14:18:56.649473+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- git: `66f3cbc18d14b5358e16683150017d3c69607b71`
- git_dirty_status: `M docs/artifacts/current-perf-counter-baseline-summary.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/benchmarks.md; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `1e338fcdff8006f09bfed7f654834432dc3fa6c0f72f09db72cbdc5d42cb1d11`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.002009 | 0.005719 | 2.846 | 0.005894 | 0.005916 | 0.005933 |
| seq_write | 0.003205 | 0.005043 | 1.573 | 0.005152 | 0.005166 | 0.005177 |
| small_read | 0.000084 | 0.004480 | 53.299 | 0.004525 | 0.004531 | 0.004535 |
| small_write | 0.000132 | 0.007545 | 57.047 | 0.008061 | 0.008126 | 0.008177 |
| write_fsync_close | 0.000159 | 0.014248 | 89.569 | 0.014901 | 0.014982 | 0.015048 |
| small_stat_open_read | 0.002592 | 0.124883 | 48.178 | 0.139303 | 0.141106 | 0.142548 |
| readdir_lstat | 0.001174 | 0.098953 | 84.296 | 0.122946 | 0.125946 | 0.128345 |
| symlink_open_read | 0.000018 | 0.000285 | 16.073 | 0.000296 | 0.000297 | 0.000298 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.008144 | 0.010975 | 0.011329 | 0.011612 |
| symlink_parent_mkdir_rmdir | 0.455281 | 0.494081 | 0.498931 | 0.502811 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=44365 avg_ns=44365 max_ns=44365
  fuse_op.create: count=72 total_ns=14900636 avg_ns=206953 max_ns=243668
  fuse_op.flush: count=884 total_ns=12932716 avg_ns=14629 max_ns=743876
  fuse_op.fsync: count=64 total_ns=1338613 avg_ns=20915 max_ns=31860
  fuse_op.getattr: count=5509 total_ns=192097209 avg_ns=34869 max_ns=2117098
  fuse_op.getxattr: count=336 total_ns=14987627 avg_ns=44606 max_ns=70122
  fuse_op.lookup: count=34533 total_ns=1138126780 avg_ns=32957 max_ns=2650710
  fuse_op.mkdir: count=800 total_ns=180909350 avg_ns=226136 max_ns=2303947
  fuse_op.open: count=812 total_ns=29704372 avg_ns=36581 max_ns=677271
  fuse_op.opendir: count=8 total_ns=212186 avg_ns=26523 max_ns=40708
  fuse_op.read: count=956 total_ns=30806920 avg_ns=32224 max_ns=1313467
  fuse_op.readdir: count=15 total_ns=16279091 avg_ns=1085272 max_ns=3976817
  fuse_op.readdirplus: count=9 total_ns=27883988 avg_ns=3098220 max_ns=5810738
  fuse_op.readlink: count=2408 total_ns=127323110 avg_ns=52875 max_ns=2128309
  fuse_op.release: count=884 total_ns=1083959 avg_ns=1226 max_ns=6473
  fuse_op.releasedir: count=8 total_ns=340554 avg_ns=42569 max_ns=68430
  fuse_op.rmdir: count=800 total_ns=254496863 avg_ns=318121 max_ns=4240134
  fuse_op.statfs: count=2 total_ns=7731 avg_ns=3865 max_ns=4577
  fuse_op.unlink: count=72 total_ns=10168038 avg_ns=141222 max_ns=362735
  fuse_op.write: count=336 total_ns=19096527 avg_ns=56834 max_ns=367652
  policy_decision: count=171948 total_ns=318570326 avg_ns=1852 max_ns=2300691
  matcher_candidates: count=800
  state_read_lock_wait: count=47615 total_ns=1821324 avg_ns=38 max_ns=1132
  state_read_lock_hold: count=47615 total_ns=5644886 avg_ns=118 max_ns=72088
  state_write_lock_wait: count=34743 total_ns=1532875 avg_ns=44 max_ns=114167
  state_write_lock_hold: count=34743 total_ns=172361746 avg_ns=4961 max_ns=4050814
  open_confined_openat2: count=98298 total_ns=94895121 avg_ns=965 max_ns=2023842
  source_root_path: count=145100 total_ns=349832834 avg_ns=2410 max_ns=2580653
  resolved_virtual_path: count=145310 total_ns=408003127 avg_ns=2807 max_ns=1774705
  resolved_virtual_path_from_path: count=46941 total_ns=248444757 avg_ns=5292 max_ns=776029
  resolved_virtual_path_from_open_fd: count=98369 total_ns=159558370 avg_ns=1622 max_ns=1774705
  read_size_bucket.0_4k: count=804 total_ns=1157257 avg_ns=1439 max_ns=4623
  read_size_bucket.4k_64k: count=12 total_ns=126288 avg_ns=10524 max_ns=17388
  read_size_bucket.64k_1m: count=140 total_ns=3657714 avg_ns=26126 max_ns=83832
  write_size_bucket.0_4k: count=320 total_ns=1089247 avg_ns=3403 max_ns=12599
  write_size_bucket.64k_1m: count=16 total_ns=4451528 avg_ns=278220 max_ns=296476
  readdir_attr_generation_scan: count=15 total_ns=12716710 avg_ns=847780 max_ns=3625123
  readdir_attr_generation_entries: count=5100
  readdirplus_attr_generation_scan: count=9 total_ns=24864835 avg_ns=2762759 max_ns=5534834
  readdirplus_attr_generation_entries: count=3300
  invalidations: count=1744 invalidated_entries=872 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
