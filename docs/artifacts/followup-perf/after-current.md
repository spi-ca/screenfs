# ScreenFS benchmark result

- timestamp: `2026-06-14T21:59:14.503671+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --iterations 10 --warmups 3 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/followup-perf/after-current.json --output-md docs/artifacts/followup-perf/after-current.md --output-svg docs/artifacts/followup-perf/after-current.svg`
- git: `4a826e352c96967da1f8cd589f4b0f341be57617`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `2a3335c58ca1419a2e38ee5ca257f1578c98143c901ff6609819da70bf12eb5d`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.000765 | 0.002657 | 3.472 | 0.002968 | 0.003025 | 0.003070 |
| seq_write | 0.000403 | 0.002069 | 5.138 | 0.002518 | 0.002519 | 0.002520 |
| small_read | 0.000024 | 0.002004 | 85.293 | 0.002162 | 0.002283 | 0.002380 |
| small_write | 0.000046 | 0.003392 | 74.476 | 0.003667 | 0.003796 | 0.003899 |
| write_fsync_close | 0.000054 | 0.005722 | 106.279 | 0.005946 | 0.006066 | 0.006161 |
| small_stat_open_read | 0.000627 | 0.045797 | 72.989 | 0.048239 | 0.048610 | 0.048906 |
| readdir_lstat | 0.000357 | 0.039660 | 111.145 | 0.040755 | 0.041059 | 0.041302 |
| symlink_open_read | 0.000004 | 0.000198 | 44.089 | 0.000217 | 0.000224 | 0.000229 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.006370 | 0.006973 | 0.007044 | 0.007100 |
| matcher_hidden_stat_miss | 0.009771 | 0.010409 | 0.010564 | 0.010688 |
| symlink_parent_mkdir_rmdir | 0.181465 | 0.186076 | 0.186168 | 0.186242 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=63567 avg_ns=63567 max_ns=63567
  fuse_op.create: count=234 total_ns=20970004 avg_ns=89615 max_ns=348378
  fuse_op.flush: count=2873 total_ns=9570934 avg_ns=3331 max_ns=279057
  fuse_op.fsync: count=208 total_ns=787578 avg_ns=3786 max_ns=20939
  fuse_op.getattr: count=17902 total_ns=308767401 avg_ns=17247 max_ns=295348
  fuse_op.getxattr: count=1092 total_ns=21842810 avg_ns=20002 max_ns=273951
  fuse_op.lookup: count=120021 total_ns=1819415974 avg_ns=15159 max_ns=642215
  fuse_op.mkdir: count=2600 total_ns=229352490 avg_ns=88212 max_ns=341041
  fuse_op.open: count=2639 total_ns=45727620 avg_ns=17327 max_ns=290487
  fuse_op.opendir: count=26 total_ns=404278 avg_ns=15549 max_ns=29281
  fuse_op.read: count=3107 total_ns=45357323 avg_ns=14598 max_ns=161413
  fuse_op.readdir: count=51 total_ns=16063623 avg_ns=314973 max_ns=1516298
  fuse_op.readdirplus: count=27 total_ns=30472207 avg_ns=1128600 max_ns=2108468
  fuse_op.readlink: count=7826 total_ns=177491857 avg_ns=22679 max_ns=292745
  fuse_op.release: count=2873 total_ns=1827398 avg_ns=636 max_ns=15787
  fuse_op.releasedir: count=26 total_ns=693326 avg_ns=26666 max_ns=54035
  fuse_op.rmdir: count=2600 total_ns=356798709 avg_ns=137230 max_ns=412340
  fuse_op.statfs: count=2 total_ns=1470 avg_ns=735 max_ns=901
  fuse_op.unlink: count=234 total_ns=14091872 avg_ns=60221 max_ns=175141
  fuse_op.write: count=1092 total_ns=27883227 avg_ns=25534 max_ns=239952
  policy_decision: count=579586 total_ns=347165276 avg_ns=598 max_ns=276183
  matcher_candidates: count=5200
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=5200
  matcher_candidate_order.descendant: count=561256 total_ns=60179750 avg_ns=107 max_ns=253466
  matcher_candidate_order.path: count=1720428 total_ns=366942460 avg_ns=213 max_ns=276108
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=12171306
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=12171306
  matcher_candidate_order_ancestor_steps: count=7026404
  matcher_candidate_order_ancestor_steps.descendant: count=1717809
  matcher_candidate_order_ancestor_steps.path: count=5308595
  state_read_lock_wait: count=162533 total_ns=3077107 avg_ns=18 max_ns=3836
  state_read_lock_hold: count=162533 total_ns=9730950 avg_ns=59 max_ns=50232
  state_write_lock_wait: count=118108 total_ns=2183619 avg_ns=18 max_ns=1382
  state_write_lock_hold: count=118108 total_ns=251137105 avg_ns=2126 max_ns=499975
  open_confined_openat2: count=332437 total_ns=123569165 avg_ns=371 max_ns=258584
  source_root_path: count=489730 total_ns=517142784 avg_ns=1055 max_ns=318592
  resolved_virtual_path: count=490417 total_ns=710165797 avg_ns=1448 max_ns=310053
  resolved_virtual_path_from_path: count=157747 total_ns=456866538 avg_ns=2896 max_ns=310053
  resolved_virtual_path_from_path_component_walk: count=157747 total_ns=403697885 avg_ns=2559 max_ns=309587
  resolved_virtual_path_from_path_canonicalize: count=385485 total_ns=333543073 avg_ns=865 max_ns=309203
  resolved_virtual_path_from_path_source_root_confinement: count=385485 total_ns=32628504 avg_ns=84 max_ns=13408
  resolved_virtual_path_from_path_virtual_conversion: count=157747 total_ns=45724662 avg_ns=289 max_ns=20357
  resolved_virtual_path_from_open_fd: count=332670 total_ns=253299259 avg_ns=761 max_ns=259353
  read_size_bucket.0_4k: count=2613 total_ns=1067526 avg_ns=408 max_ns=8305
  read_size_bucket.4k_64k: count=39 total_ns=103024 avg_ns=2641 max_ns=13121
  read_size_bucket.64k_1m: count=455 total_ns=3409998 avg_ns=7494 max_ns=26461
  write_size_bucket.0_4k: count=1040 total_ns=1081678 avg_ns=1040 max_ns=7285
  write_size_bucket.64k_1m: count=52 total_ns=6681194 avg_ns=128484 max_ns=217968
  readdir_directory_scan: count=51 total_ns=12099633 avg_ns=237247 max_ns=947032
  readdir_attr_generation_scan: count=51 total_ns=1980695 avg_ns=38837 max_ns=194473
  readdir_attr_generation_entries: count=4082
  readdir_symlink_visibility: count=51 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=51 total_ns=499969 avg_ns=9803 max_ns=64766
  readdir_page_commit: count=51 total_ns=3042096 avg_ns=59648 max_ns=500093
  readdirplus_directory_scan: count=27 total_ns=25805254 avg_ns=955750 max_ns=1791339
  readdirplus_attr_generation_scan: count=27 total_ns=4790289 avg_ns=177418 max_ns=428681
  readdirplus_attr_generation_entries: count=9100
  readdirplus_symlink_visibility: count=27 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=27 total_ns=1262479 avg_ns=46758 max_ns=79474
  readdirplus_page_commit: count=27 total_ns=3085294 avg_ns=114270 max_ns=272912
  invalidations: count=5668 invalidated_entries=2834 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
