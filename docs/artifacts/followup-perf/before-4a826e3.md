# ScreenFS benchmark result

- timestamp: `2026-06-14T21:59:09.938916+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-before-4a826e3/target/release/screenfs --perf-counters --iterations 10 --warmups 3 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/followup-perf/before-4a826e3.json --output-md docs/artifacts/followup-perf/before-4a826e3.md --output-svg docs/artifacts/followup-perf/before-4a826e3.svg`
- git: `4a826e352c96967da1f8cd589f4b0f341be57617`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-before-4a826e3/target/release/screenfs`
- screenfs_bin_sha256: `75e858dfeb104a1064e2a7ab5cfa4adeeee1dea2011b7d8c0b221d2eb0ac9b2b`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.000780 | 0.002905 | 3.725 | 0.002996 | 0.003043 | 0.003081 |
| seq_write | 0.000380 | 0.002162 | 5.682 | 0.002504 | 0.002571 | 0.002625 |
| small_read | 0.000024 | 0.001764 | 72.722 | 0.001884 | 0.002124 | 0.002317 |
| small_write | 0.000047 | 0.003888 | 83.478 | 0.004243 | 0.004814 | 0.005271 |
| write_fsync_close | 0.000060 | 0.008085 | 134.042 | 0.009577 | 0.009690 | 0.009781 |
| small_stat_open_read | 0.000650 | 0.069462 | 106.833 | 0.070716 | 0.072065 | 0.073143 |
| readdir_lstat | 0.000320 | 0.058503 | 182.588 | 0.059671 | 0.060084 | 0.060415 |
| symlink_open_read | 0.000005 | 0.000347 | 71.880 | 0.000381 | 0.000383 | 0.000384 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.009080 | 0.009174 | 0.009175 | 0.009175 |
| matcher_hidden_stat_miss | 0.015278 | 0.015523 | 0.015595 | 0.015653 |
| symlink_parent_mkdir_rmdir | 0.276760 | 0.278128 | 0.278536 | 0.278863 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62245 avg_ns=62245 max_ns=62245
  fuse_op.create: count=234 total_ns=29862137 avg_ns=127615 max_ns=217529
  fuse_op.flush: count=2873 total_ns=16618928 avg_ns=5784 max_ns=280598
  fuse_op.fsync: count=208 total_ns=1688672 avg_ns=8118 max_ns=223210
  fuse_op.getattr: count=17902 total_ns=468723134 avg_ns=26182 max_ns=110067
  fuse_op.getxattr: count=1092 total_ns=25424218 avg_ns=23282 max_ns=74916
  fuse_op.lookup: count=120021 total_ns=2612233864 avg_ns=21764 max_ns=335825
  fuse_op.mkdir: count=2600 total_ns=374178521 avg_ns=143914 max_ns=354174
  fuse_op.open: count=2639 total_ns=74651190 avg_ns=28287 max_ns=53656
  fuse_op.opendir: count=26 total_ns=621417 avg_ns=23900 max_ns=35535
  fuse_op.read: count=3107 total_ns=67141204 avg_ns=21609 max_ns=344800
  fuse_op.readdir: count=51 total_ns=39649471 avg_ns=777440 max_ns=2614641
  fuse_op.readdirplus: count=27 total_ns=59031940 avg_ns=2186368 max_ns=3523868
  fuse_op.readlink: count=7826 total_ns=269505478 avg_ns=34437 max_ns=103440
  fuse_op.release: count=2873 total_ns=1941253 avg_ns=675 max_ns=16713
  fuse_op.releasedir: count=26 total_ns=1192212 avg_ns=45854 max_ns=104372
  fuse_op.rmdir: count=2600 total_ns=495855137 avg_ns=190713 max_ns=3108460
  fuse_op.statfs: count=2 total_ns=6977 avg_ns=3488 max_ns=3771
  fuse_op.unlink: count=234 total_ns=19165946 avg_ns=81905 max_ns=234508
  fuse_op.write: count=1092 total_ns=32178410 avg_ns=29467 max_ns=227074
  policy_decision: count=579586 total_ns=832021022 avg_ns=1435 max_ns=326515
  matcher_candidates: count=5200
  state_read_lock_wait: count=162533 total_ns=4515376 avg_ns=27 max_ns=5312
  state_read_lock_hold: count=162533 total_ns=11008565 avg_ns=67 max_ns=15858
  state_write_lock_wait: count=118108 total_ns=2858460 avg_ns=24 max_ns=7812
  state_write_lock_hold: count=118108 total_ns=299238701 avg_ns=2533 max_ns=672214
  open_confined_openat2: count=332437 total_ns=189051423 avg_ns=568 max_ns=35921
  source_root_path: count=489730 total_ns=742612191 avg_ns=1516 max_ns=327837
  resolved_virtual_path: count=490417 total_ns=867635808 avg_ns=1769 max_ns=332619
  resolved_virtual_path_from_path: count=157747 total_ns=522576764 avg_ns=3312 max_ns=332619
  resolved_virtual_path_from_open_fd: count=332670 total_ns=345059044 avg_ns=1037 max_ns=308158
  read_size_bucket.0_4k: count=2613 total_ns=3279847 avg_ns=1255 max_ns=6966
  read_size_bucket.4k_64k: count=39 total_ns=76529 avg_ns=1962 max_ns=9450
  read_size_bucket.64k_1m: count=455 total_ns=3897971 avg_ns=8566 max_ns=27592
  write_size_bucket.0_4k: count=1040 total_ns=1246812 avg_ns=1198 max_ns=53877
  write_size_bucket.64k_1m: count=52 total_ns=6690600 avg_ns=128665 max_ns=203744
  readdir_attr_generation_scan: count=51 total_ns=34204540 avg_ns=670677 max_ns=2168088
  readdir_attr_generation_entries: count=17700
  readdirplus_attr_generation_scan: count=27 total_ns=52531393 avg_ns=1945607 max_ns=3118478
  readdirplus_attr_generation_entries: count=9600
  invalidations: count=5668 invalidated_entries=2834 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
