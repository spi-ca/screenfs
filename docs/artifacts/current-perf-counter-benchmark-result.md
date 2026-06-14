# ScreenFS benchmark result

- timestamp: `2026-06-14T21:59:16.565239+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- git: `4a826e352c96967da1f8cd589f4b0f341be57617`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `2a3335c58ca1419a2e38ee5ca257f1578c98143c901ff6609819da70bf12eb5d`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.002027 | 0.002566 | 1.266 | 0.002748 | 0.002771 | 0.002789 |
| seq_write | 0.000661 | 0.002207 | 3.341 | 0.002617 | 0.002668 | 0.002709 |
| small_read | 0.000033 | 0.001950 | 58.984 | 0.002011 | 0.002019 | 0.002025 |
| small_write | 0.000064 | 0.003296 | 51.496 | 0.003515 | 0.003542 | 0.003564 |
| write_fsync_close | 0.000065 | 0.005728 | 88.443 | 0.006001 | 0.006035 | 0.006063 |
| small_stat_open_read | 0.000681 | 0.046270 | 67.929 | 0.048041 | 0.048262 | 0.048439 |
| readdir_lstat | 0.000318 | 0.036113 | 113.457 | 0.036804 | 0.036890 | 0.036960 |
| symlink_open_read | 0.000005 | 0.000215 | 40.109 | 0.000229 | 0.000230 | 0.000232 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.006297 | 0.006520 | 0.006548 | 0.006570 |
| matcher_hidden_stat_miss | 0.016245 | 0.017138 | 0.017250 | 0.017339 |
| symlink_parent_mkdir_rmdir | 0.194784 | 0.240109 | 0.245774 | 0.250307 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65507 avg_ns=65507 max_ns=65507
  fuse_op.create: count=72 total_ns=6417851 avg_ns=89136 max_ns=149448
  fuse_op.flush: count=884 total_ns=3024477 avg_ns=3421 max_ns=143357
  fuse_op.fsync: count=64 total_ns=239863 avg_ns=3747 max_ns=9052
  fuse_op.getattr: count=5509 total_ns=95704419 avg_ns=17372 max_ns=135019
  fuse_op.getxattr: count=336 total_ns=6546498 avg_ns=19483 max_ns=45227
  fuse_op.lookup: count=36933 total_ns=636709440 avg_ns=17239 max_ns=669588
  fuse_op.mkdir: count=800 total_ns=85941115 avg_ns=107426 max_ns=377701
  fuse_op.open: count=812 total_ns=13989050 avg_ns=17227 max_ns=70246
  fuse_op.opendir: count=8 total_ns=112918 avg_ns=14114 max_ns=16504
  fuse_op.read: count=956 total_ns=13991383 avg_ns=14635 max_ns=51434
  fuse_op.readdir: count=15 total_ns=4921473 avg_ns=328098 max_ns=1242700
  fuse_op.readdirplus: count=9 total_ns=9525771 avg_ns=1058419 max_ns=1821595
  fuse_op.readlink: count=2408 total_ns=65120248 avg_ns=27043 max_ns=313629
  fuse_op.release: count=884 total_ns=557252 avg_ns=630 max_ns=3902
  fuse_op.releasedir: count=8 total_ns=160511 avg_ns=20063 max_ns=37422
  fuse_op.rmdir: count=800 total_ns=128340306 avg_ns=160425 max_ns=407800
  fuse_op.statfs: count=2 total_ns=1864 avg_ns=932 max_ns=1079
  fuse_op.unlink: count=72 total_ns=4427082 avg_ns=61487 max_ns=182040
  fuse_op.write: count=336 total_ns=8350643 avg_ns=24853 max_ns=167332
  policy_decision: count=178348 total_ns=117867213 avg_ns=660 max_ns=46418
  matcher_candidates: count=1600
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1600
  matcher_candidate_order.descendant: count=172708 total_ns=20261757 avg_ns=117 max_ns=8615
  matcher_candidate_order.path: count=529404 total_ns=126975339 avg_ns=239 max_ns=88317
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3745308
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3745308
  matcher_candidate_order_ancestor_steps: count=2162084
  matcher_candidate_order_ancestor_steps.descendant: count=528585
  matcher_candidate_order_ancestor_steps.path: count=1633499
  state_read_lock_wait: count=50015 total_ns=1038654 avg_ns=20 max_ns=1287
  state_read_lock_hold: count=50015 total_ns=3187997 avg_ns=63 max_ns=4508
  state_write_lock_wait: count=36343 total_ns=737686 avg_ns=20 max_ns=552
  state_write_lock_hold: count=36343 total_ns=89270074 avg_ns=2456 max_ns=436766
  open_confined_openat2: count=102298 total_ns=45894031 avg_ns=448 max_ns=320143
  source_root_path: count=150700 total_ns=185622634 avg_ns=1231 max_ns=58655
  resolved_virtual_path: count=150910 total_ns=240477922 avg_ns=1593 max_ns=269722
  resolved_virtual_path_from_path: count=48541 total_ns=152110289 avg_ns=3133 max_ns=264953
  resolved_virtual_path_from_path_component_walk: count=48541 total_ns=134220450 avg_ns=2765 max_ns=264501
  resolved_virtual_path_from_path_canonicalize: count=118617 total_ns=110657273 avg_ns=932 max_ns=263889
  resolved_virtual_path_from_path_source_root_confinement: count=118617 total_ns=10844211 avg_ns=91 max_ns=5103
  resolved_virtual_path_from_path_virtual_conversion: count=48541 total_ns=15222136 avg_ns=313 max_ns=6366
  resolved_virtual_path_from_open_fd: count=102369 total_ns=88367633 avg_ns=863 max_ns=269722
  read_size_bucket.0_4k: count=804 total_ns=395503 avg_ns=491 max_ns=1855
  read_size_bucket.4k_64k: count=12 total_ns=45137 avg_ns=3761 max_ns=8324
  read_size_bucket.64k_1m: count=140 total_ns=1193245 avg_ns=8523 max_ns=31735
  write_size_bucket.0_4k: count=320 total_ns=326418 avg_ns=1020 max_ns=4104
  write_size_bucket.64k_1m: count=16 total_ns=1943010 avg_ns=121438 max_ns=141742
  readdir_directory_scan: count=15 total_ns=3594296 avg_ns=239619 max_ns=875703
  readdir_attr_generation_scan: count=15 total_ns=586126 avg_ns=39075 max_ns=150719
  readdir_attr_generation_entries: count=1256
  readdir_symlink_visibility: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=15 total_ns=157313 avg_ns=10487 max_ns=46387
  readdir_page_commit: count=15 total_ns=1063096 avg_ns=70873 max_ns=436883
  readdirplus_directory_scan: count=9 total_ns=7888772 avg_ns=876530 max_ns=1535183
  readdirplus_attr_generation_scan: count=9 total_ns=1452476 avg_ns=161386 max_ns=394251
  readdirplus_attr_generation_entries: count=2800
  readdirplus_symlink_visibility: count=9 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=9 total_ns=403665 avg_ns=44851 max_ns=79491
  readdirplus_page_commit: count=9 total_ns=1134901 avg_ns=126100 max_ns=281872
  invalidations: count=1744 invalidated_entries=872 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
