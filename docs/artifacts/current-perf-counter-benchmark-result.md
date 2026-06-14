# ScreenFS benchmark result

- timestamp: `2026-06-14T10:18:33.424241+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- git: `c37b9d7aa4a13961fefe5b8121ef286ba0b834b9`
- git_dirty_status: `M .gitignore;  M Cargo.toml;  M README.md;  M docs/README.md;  M docs/artifacts/current-toctou-hardening-evidence.md; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `5767cd8621679d5574438926615e6780fe179754e546acd270a13a2f903cd3f6`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.001001 | 0.003589 | 3.586 | 0.003836 | 0.003867 | 0.003892 |
| seq_write | 0.000430 | 0.002350 | 5.459 | 0.002601 | 0.002632 | 0.002657 |
| small_read | 0.000025 | 0.001752 | 68.819 | 0.001876 | 0.001891 | 0.001904 |
| small_write | 0.000048 | 0.003294 | 68.490 | 0.003654 | 0.003699 | 0.003735 |
| write_fsync_close | 0.000052 | 0.005631 | 108.202 | 0.005702 | 0.005711 | 0.005718 |
| small_stat_open_read | 0.001039 | 0.044042 | 42.385 | 0.045400 | 0.045570 | 0.045706 |
| readdir_lstat | 0.000331 | 0.036425 | 110.040 | 0.036471 | 0.036477 | 0.036481 |
| symlink_open_read | 0.000006 | 0.000236 | 40.044 | 0.000240 | 0.000241 | 0.000241 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.005357 | 0.005970 | 0.006046 | 0.006108 |
| symlink_parent_mkdir_rmdir | 0.197133 | 0.207075 | 0.208318 | 0.209312 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=41878 avg_ns=41878 max_ns=41878
  fuse_op.create: count=72 total_ns=6445861 avg_ns=89525 max_ns=130518
  fuse_op.flush: count=884 total_ns=2752229 avg_ns=3113 max_ns=99592
  fuse_op.fsync: count=64 total_ns=224381 avg_ns=3505 max_ns=6847
  fuse_op.getattr: count=5509 total_ns=95495796 avg_ns=17334 max_ns=70639
  fuse_op.getxattr: count=336 total_ns=6507462 avg_ns=19367 max_ns=49476
  fuse_op.lookup: count=34533 total_ns=527626507 avg_ns=15278 max_ns=659308
  fuse_op.mkdir: count=800 total_ns=82387071 avg_ns=102983 max_ns=366717
  fuse_op.open: count=812 total_ns=14258952 avg_ns=17560 max_ns=40176
  fuse_op.opendir: count=8 total_ns=126141 avg_ns=15767 max_ns=28947
  fuse_op.read: count=956 total_ns=14605885 avg_ns=15278 max_ns=122357
  fuse_op.readdir: count=15 total_ns=7637386 avg_ns=509159 max_ns=1927936
  fuse_op.readdirplus: count=9 total_ns=12968685 avg_ns=1440965 max_ns=2676232
  fuse_op.readlink: count=2408 total_ns=59947727 avg_ns=24895 max_ns=84777
  fuse_op.release: count=884 total_ns=537322 avg_ns=607 max_ns=2741
  fuse_op.releasedir: count=8 total_ns=217736 avg_ns=27217 max_ns=54379
  fuse_op.rmdir: count=800 total_ns=122253519 avg_ns=152816 max_ns=445837
  fuse_op.statfs: count=2 total_ns=1194 avg_ns=597 max_ns=684
  fuse_op.unlink: count=72 total_ns=4709365 avg_ns=65407 max_ns=368569
  fuse_op.write: count=336 total_ns=8690591 avg_ns=25864 max_ns=200626
  policy_decision: count=171948 total_ns=167560000 avg_ns=974 max_ns=56672
  matcher_candidates: count=800
  state_read_lock_wait: count=47615 total_ns=1462069 avg_ns=30 max_ns=2069
  state_read_lock_hold: count=47615 total_ns=2909555 avg_ns=61 max_ns=2185
  state_write_lock_wait: count=34743 total_ns=662085 avg_ns=19 max_ns=3781
  state_write_lock_hold: count=34743 total_ns=81733519 avg_ns=2352 max_ns=483428
  open_confined_openat2: count=98298 total_ns=38490969 avg_ns=391 max_ns=52819
  resolved_virtual_path: count=145310 total_ns=323058296 avg_ns=2223 max_ns=321373
  read_size_bucket.0_4k: count=804 total_ns=226476 avg_ns=281 max_ns=1812
  read_size_bucket.4k_64k: count=12 total_ns=31312 avg_ns=2609 max_ns=6169
  read_size_bucket.64k_1m: count=140 total_ns=1523113 avg_ns=10879 max_ns=84610
  write_size_bucket.0_4k: count=320 total_ns=403364 avg_ns=1260 max_ns=61848
  write_size_bucket.64k_1m: count=16 total_ns=2017888 avg_ns=126118 max_ns=179464
  readdir_attr_generation_scan: count=15 total_ns=6307588 avg_ns=420505 max_ns=1390239
  readdir_attr_generation_entries: count=5100
  readdirplus_attr_generation_scan: count=9 total_ns=11318832 avg_ns=1257648 max_ns=2363590
  readdirplus_attr_generation_entries: count=3300
  invalidations: count=1744 invalidated_entries=872 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
