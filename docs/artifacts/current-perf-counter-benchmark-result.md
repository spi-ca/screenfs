# ScreenFS benchmark result

- timestamp: `2026-06-20T14:59:22.947818+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `868f0dcba4e894e1fb1bcddd16712aa4e4098f6b`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `868f0dcba4e894e1fb1bcddd16712aa4e4098f6b`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher20`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `all`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, write_fsync_close, small_stat_open_read, readdir_lstat, symlink_open_read, rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close, metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs, metadata_opendir, sync_flush_only, sync_fsync_only, sync_release_flush, read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close, readdir_basic, readdirplus_basic, readdir_symlink_visibility, readdirplus_symlink_visibility`
- screenfs_only_workloads: `hidden_stat_miss, matcher_hidden_stat_miss, symlink_parent_mkdir_rmdir`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.001369 | 0.003926 | 2.869 | 0.004044 | 0.004059 | 0.004071 |
| seq_write | 0.000648 | 0.003143 | 4.847 | 0.003327 | 0.003351 | 0.003369 |
| small_read | 0.000041 | 0.002021 | 49.506 | 0.002174 | 0.002193 | 0.002208 |
| small_write | 0.000059 | 0.005054 | 86.268 | 0.005079 | 0.005082 | 0.005084 |
| write_fsync_close | 0.000088 | 0.007329 | 83.380 | 0.008278 | 0.008396 | 0.008491 |
| small_stat_open_read | 0.001081 | 0.058767 | 54.362 | 0.059329 | 0.059400 | 0.059456 |
| readdir_lstat | 0.000486 | 0.046273 | 95.222 | 0.047356 | 0.047491 | 0.047599 |
| symlink_open_read | 0.000011 | 0.000259 | 23.935 | 0.000273 | 0.000275 | 0.000276 |
| rand_read_4k | 0.013328 | 0.320090 | 24.016 | 0.343733 | 0.346689 | 0.349053 |
| rand_write_4k | 0.013888 | 1.216761 | 87.610 | 1.220617 | 1.221100 | 1.221485 |
| sync_write_4k | 0.000027 | 0.001653 | 60.150 | 0.001743 | 0.001754 | 0.001763 |
| small_open_read_close | 0.029560 | 0.773664 | 26.173 | 0.783903 | 0.785183 | 0.786206 |
| metadata_lookup | 0.003189 | 0.035356 | 11.087 | 0.035403 | 0.035409 | 0.035414 |
| metadata_getattr | 0.003260 | 0.046879 | 14.382 | 0.046912 | 0.046916 | 0.046919 |
| metadata_open | 0.003017 | 0.054121 | 17.942 | 0.054481 | 0.054526 | 0.054562 |
| metadata_readlink | 0.000276 | 0.046702 | 169.186 | 0.047679 | 0.047801 | 0.047899 |
| metadata_access | 0.002935 | 0.048412 | 16.495 | 0.049928 | 0.050118 | 0.050269 |
| metadata_statfs | 0.000336 | 0.002623 | 7.798 | 0.002626 | 0.002626 | 0.002626 |
| metadata_opendir | 0.000969 | 0.039486 | 40.767 | 0.040100 | 0.040177 | 0.040238 |
| sync_flush_only | 0.000069 | 0.004272 | 61.923 | 0.004449 | 0.004471 | 0.004488 |
| sync_fsync_only | 0.000032 | 0.001770 | 55.160 | 0.001918 | 0.001936 | 0.001951 |
| sync_release_flush | 0.000050 | 0.004053 | 81.311 | 0.004399 | 0.004442 | 0.004477 |
| read_only_open_close | 0.024918 | 0.389911 | 15.648 | 0.414433 | 0.417499 | 0.419951 |
| read_only_open_read_close | 0.027731 | 0.631174 | 22.760 | 0.635899 | 0.636489 | 0.636962 |
| write_open_write_close | 0.000054 | 0.003886 | 72.579 | 0.003908 | 0.003910 | 0.003912 |
| write_open_fsync_close | 0.000059 | 0.003988 | 67.688 | 0.004327 | 0.004370 | 0.004404 |
| readdir_basic | 0.000100 | 0.005167 | 51.887 | 0.005358 | 0.005382 | 0.005401 |
| readdirplus_basic | 0.000499 | 0.044932 | 89.958 | 0.045789 | 0.045896 | 0.045982 |
| readdir_symlink_visibility | 0.000196 | 0.015634 | 79.614 | 0.015822 | 0.015846 | 0.015865 |
| readdirplus_symlink_visibility | 0.000676 | 0.059818 | 88.463 | 0.060146 | 0.060186 | 0.060219 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.007938 | 0.008055 | 0.008070 | 0.008081 |
| matcher_hidden_stat_miss | 0.013196 | 0.013957 | 0.014052 | 0.014128 |
| symlink_parent_mkdir_rmdir | 0.227349 | 0.227771 | 0.227824 | 0.227866 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=47798597 avg_ns=23327 max_ns=84489
  fuse_op.create: count=100 total_ns=12503545 avg_ns=125035 max_ns=181659
  fuse_op.flush: count=340 total_ns=2780186 avg_ns=8177 max_ns=215730
  fuse_op.fsync: count=196 total_ns=1241546 avg_ns=6334 max_ns=107519
  fuse_op.getattr: count=128385 total_ns=2085557169 avg_ns=16244 max_ns=287298
  fuse_op.getxattr: count=66500 total_ns=1754693031 avg_ns=26386 max_ns=287949
  fuse_op.lookup: count=232173 total_ns=3511091917 avg_ns=15122 max_ns=19897694
  fuse_op.mkdir: count=800 total_ns=102655342 avg_ns=128319 max_ns=270953
  fuse_op.open: count=52256 total_ns=1232988819 avg_ns=23595 max_ns=320982
  fuse_op.opendir: count=2072 total_ns=34879247 avg_ns=16833 max_ns=69787
  fuse_op.read: count=37148 total_ns=869440899 avg_ns=23404 max_ns=175366
  fuse_op.readdir: count=46 total_ns=53083237 avg_ns=1153983 max_ns=4760722
  fuse_op.readdirplus: count=26 total_ns=147916439 avg_ns=5689093 max_ns=11455090
  fuse_op.readlink: count=4456 total_ns=110150111 avg_ns=24719 max_ns=96160
  fuse_op.release: count=52356 total_ns=30931254 avg_ns=590 max_ns=76405
  fuse_op.releasedir: count=2072 total_ns=1948650 avg_ns=940 max_ns=138755
  fuse_op.rmdir: count=800 total_ns=180743691 avg_ns=225929 max_ns=541175
  fuse_op.setattr: count=4 total_ns=226177 avg_ns=56544 max_ns=59574
  fuse_op.statfs: count=2050 total_ns=520693 avg_ns=253 max_ns=2823
  fuse_op.unlink: count=100 total_ns=12137311 avg_ns=121373 max_ns=560124
  fuse_op.write: count=66256 total_ns=2269770874 avg_ns=34257 max_ns=277565
  policy_decision: count=1754170 total_ns=1468113578 avg_ns=836 max_ns=146639
  matcher_candidates: count=1600
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1600
  matcher_candidate_order.descendant: count=1615670 total_ns=230977748 avg_ns=142 max_ns=111748
  matcher_candidate_order.path: count=5124010 total_ns=1597550027 avg_ns=311 max_ns=273732
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=36837570
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=36837570
  matcher_candidate_order_ancestor_steps: count=22764856
  matcher_candidate_order_ancestor_steps.descendant: count=5413614
  matcher_candidate_order_ancestor_steps.path: count=17351242
  state_read_lock_wait: count=593707 total_ns=14650333 avg_ns=24 max_ns=7993
  state_read_lock_hold: count=593707 total_ns=44411943 avg_ns=74 max_ns=68705
  state_write_lock_wait: count=336671 total_ns=8374388 avg_ns=24 max_ns=7090
  state_write_lock_hold: count=336671 total_ns=275557988 avg_ns=818 max_ns=660092
  open_confined_openat2: count=728960 total_ns=553801651 avg_ns=759 max_ns=19832221
  stat_child_no_follow: count=604207 total_ns=5546867969 avg_ns=9180 max_ns=19884657
  source_root_path: count=745407 total_ns=1325715047 avg_ns=1778 max_ns=261675
  resolved_virtual_path: count=2035390 total_ns=4670472152 avg_ns=2294 max_ns=270588
  resolved_virtual_path_from_path: count=1202927 total_ns=3731761259 avg_ns=3102 max_ns=270588
  resolved_virtual_path_from_path_component_walk: count=1202927 total_ns=3266165254 avg_ns=2715 max_ns=270190
  resolved_virtual_path_from_path_canonicalize: count=2554866 total_ns=2666527198 avg_ns=1043 max_ns=269650
  resolved_virtual_path_from_path_source_root_confinement: count=2554866 total_ns=314488302 avg_ns=123 max_ns=87952
  resolved_virtual_path_from_path_virtual_conversion: count=1202927 total_ns=395350390 avg_ns=328 max_ns=101617
  resolved_virtual_path_from_open_fd: count=832463 total_ns=938710893 avg_ns=1127 max_ns=94325
  read_handle_snapshot: count=37148 total_ns=8803970 avg_ns=236 max_ns=6259
  read_guard_path: count=37148 total_ns=813315246 avg_ns=21893 max_ns=173908
  read_io: count=37148 total_ns=40484362 avg_ns=1089 max_ns=87827
  write_handle_snapshot: count=66256 total_ns=14678589 avg_ns=221 max_ns=32999
  write_guard_mutation: count=66256 total_ns=2147548055 avg_ns=32412 max_ns=180229
  write_io: count=66256 total_ns=96081172 avg_ns=1450 max_ns=209013
  file_sync.flush: count=340 total_ns=2650815 avg_ns=7796 max_ns=214792
  file_sync.fsync: count=196 total_ns=1170801 avg_ns=5973 max_ns=106208
  read_size_bucket.0_4k: count=36608 total_ns=35293834 avg_ns=964 max_ns=87827
  read_size_bucket.4k_64k: count=400 total_ns=1318176 avg_ns=3295 max_ns=19413
  read_size_bucket.64k_1m: count=140 total_ns=3872352 avg_ns=27659 max_ns=70396
  write_size_bucket.0_4k: count=66240 total_ns=93249007 avg_ns=1407 max_ns=153435
  write_size_bucket.64k_1m: count=16 total_ns=2832165 avg_ns=177010 max_ns=209013
  readdir_directory_scan: count=46 total_ns=45334479 avg_ns=985532 max_ns=4127778
  readdir_attr_generation_scan: count=46 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=46 total_ns=23227461 avg_ns=504944 max_ns=2963548
  readdir_candidate_selection: count=46 total_ns=940701 avg_ns=20450 max_ns=104583
  readdir_page_commit: count=46 total_ns=6132211 avg_ns=133308 max_ns=660384
  readdirplus_directory_scan: count=26 total_ns=71686116 avg_ns=2757158 max_ns=6637835
  readdirplus_attr_generation_scan: count=4682 total_ns=43533027 avg_ns=9297 max_ns=98352
  readdirplus_attr_generation_entries: count=4656
  readdirplus_symlink_visibility: count=26 total_ns=37895845 avg_ns=1457532 max_ns=4857183
  readdirplus_candidate_selection: count=26 total_ns=1771280 avg_ns=68126 max_ns=99008
  readdirplus_page_commit: count=26 total_ns=4646514 avg_ns=178712 max_ns=493786
  invalidations: count=1800 invalidated_entries=900 evicted_entries=0 scanned_entries=992340
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
