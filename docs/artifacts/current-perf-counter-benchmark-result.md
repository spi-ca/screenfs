# ScreenFS benchmark result

- timestamp: `2026-06-15T22:57:26.286115+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+22 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `93aa3942c0f6ba6dcf017f3be6197edbcebaff2683ec0da4eef1cf61341d1a6e`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+22 more)`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher20`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `all`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, write_fsync_close, small_stat_open_read, readdir_lstat, symlink_open_read, rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close, metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs, sync_flush_only, sync_fsync_only, sync_release_flush, read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close, readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `hidden_stat_miss, matcher_hidden_stat_miss, symlink_parent_mkdir_rmdir`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.000810 | 0.004312 | 5.321 | 0.004387 | 0.004396 | 0.004404 |
| seq_write | 0.000976 | 0.003406 | 3.492 | 0.003750 | 0.003793 | 0.003828 |
| small_read | 0.000069 | 0.003521 | 50.836 | 0.003553 | 0.003556 | 0.003560 |
| small_write | 0.000134 | 0.008252 | 61.441 | 0.008263 | 0.008265 | 0.008266 |
| write_fsync_close | 0.000156 | 0.012358 | 79.146 | 0.012658 | 0.012695 | 0.012725 |
| small_stat_open_read | 0.001544 | 0.101617 | 65.815 | 0.106081 | 0.106639 | 0.107086 |
| readdir_lstat | 0.000822 | 0.071707 | 87.182 | 0.072241 | 0.072308 | 0.072361 |
| symlink_open_read | 0.000012 | 0.000478 | 39.897 | 0.000492 | 0.000493 | 0.000495 |
| rand_read_4k | 0.016974 | 0.769309 | 45.324 | 0.808161 | 0.813018 | 0.816903 |
| rand_write_4k | 0.017194 | 1.925175 | 111.970 | 2.018483 | 2.030146 | 2.039477 |
| sync_write_4k | 0.000054 | 0.002602 | 48.509 | 0.002611 | 0.002613 | 0.002614 |
| small_open_read_close | 0.034103 | 1.224557 | 35.907 | 1.262428 | 1.267162 | 1.270949 |
| metadata_lookup | 0.003161 | 0.055590 | 17.585 | 0.055657 | 0.055666 | 0.055673 |
| metadata_getattr | 0.003460 | 0.073616 | 21.275 | 0.073829 | 0.073856 | 0.073877 |
| metadata_open | 0.003351 | 0.086414 | 25.791 | 0.087402 | 0.087525 | 0.087624 |
| metadata_readlink | 0.000544 | 0.071873 | 132.141 | 0.074339 | 0.074647 | 0.074894 |
| metadata_access | 0.003147 | 0.082341 | 26.163 | 0.083104 | 0.083200 | 0.083276 |
| metadata_statfs | 0.000645 | 0.004186 | 6.491 | 0.004197 | 0.004199 | 0.004200 |
| sync_flush_only | 0.000122 | 0.006975 | 57.201 | 0.007009 | 0.007014 | 0.007017 |
| sync_fsync_only | 0.000057 | 0.002864 | 50.156 | 0.002881 | 0.002883 | 0.002885 |
| sync_release_flush | 0.000091 | 0.006103 | 66.773 | 0.006344 | 0.006374 | 0.006399 |
| read_only_open_close | 0.029066 | 0.672413 | 23.134 | 0.678113 | 0.678826 | 0.679396 |
| read_only_open_read_close | 0.032154 | 1.046400 | 32.544 | 1.046666 | 1.046700 | 1.046726 |
| write_open_write_close | 0.000099 | 0.006415 | 64.964 | 0.006517 | 0.006530 | 0.006540 |
| write_open_fsync_close | 0.000102 | 0.006861 | 67.189 | 0.007101 | 0.007131 | 0.007155 |
| readdir_basic | 0.000118 | 0.004032 | 34.041 | 0.004086 | 0.004092 | 0.004098 |
| readdirplus_basic | 0.000829 | 0.067848 | 81.797 | 0.067910 | 0.067918 | 0.067924 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.012929 | 0.012970 | 0.012975 | 0.012979 |
| matcher_hidden_stat_miss | 0.019828 | 0.019951 | 0.019967 | 0.019979 |
| symlink_parent_mkdir_rmdir | 0.341112 | 0.358290 | 0.360437 | 0.362155 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=83858824 avg_ns=40926 max_ns=73058
  fuse_op.create: count=100 total_ns=19446358 avg_ns=194463 max_ns=241377
  fuse_op.flush: count=340 total_ns=3657782 avg_ns=10758 max_ns=119947
  fuse_op.fsync: count=196 total_ns=1731538 avg_ns=8834 max_ns=91403
  fuse_op.getattr: count=124321 total_ns=3927132915 avg_ns=31588 max_ns=85332
  fuse_op.getxattr: count=66500 total_ns=3059317329 avg_ns=46004 max_ns=207042
  fuse_op.lookup: count=222037 total_ns=5408530334 avg_ns=24358 max_ns=20230358
  fuse_op.mkdir: count=800 total_ns=166684878 avg_ns=208356 max_ns=393524
  fuse_op.open: count=52256 total_ns=2103674068 avg_ns=40257 max_ns=94068
  fuse_op.opendir: count=16 total_ns=517268 avg_ns=32329 max_ns=40767
  fuse_op.read: count=37148 total_ns=1603696006 avg_ns=43170 max_ns=100330
  fuse_op.readdir: count=31 total_ns=16410650 avg_ns=529375 max_ns=1549111
  fuse_op.readdirplus: count=17 total_ns=29425205 avg_ns=1730894 max_ns=2445336
  fuse_op.readlink: count=4456 total_ns=173287068 avg_ns=38888 max_ns=84671
  fuse_op.release: count=52356 total_ns=52174188 avg_ns=996 max_ns=16922
  fuse_op.releasedir: count=16 total_ns=406550 avg_ns=25409 max_ns=45576
  fuse_op.rmdir: count=800 total_ns=202748616 avg_ns=253435 max_ns=543128
  fuse_op.setattr: count=4 total_ns=391793 avg_ns=97948 max_ns=131048
  fuse_op.statfs: count=2050 total_ns=1067427 avg_ns=520 max_ns=2044
  fuse_op.unlink: count=100 total_ns=17765528 avg_ns=177655 max_ns=535272
  fuse_op.write: count=66256 total_ns=3687530914 avg_ns=55655 max_ns=351224
  policy_decision: count=1691714 total_ns=1661374508 avg_ns=982 max_ns=19166
  matcher_candidates: count=1600
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1600
  matcher_candidate_order.descendant: count=1553214 total_ns=230740665 avg_ns=148 max_ns=30838
  matcher_candidate_order.path: count=4936642 total_ns=1833946560 avg_ns=371 max_ns=29345
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=35525994
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=35525994
  matcher_candidate_order_ancestor_steps: count=21988216
  matcher_candidate_order_ancestor_steps.descendant: count=5219454
  matcher_candidate_order_ancestor_steps.path: count=16768762
  state_read_lock_wait: count=577427 total_ns=14390530 avg_ns=24 max_ns=9588
  state_read_lock_hold: count=577427 total_ns=76655082 avg_ns=132 max_ns=9268
  state_write_lock_wait: count=322391 total_ns=8068100 avg_ns=25 max_ns=7515
  state_write_lock_hold: count=322391 total_ns=277504274 avg_ns=860 max_ns=607700
  open_confined_openat2: count=705952 total_ns=865188967 avg_ns=1225 max_ns=20208026
  stat_child_no_follow: count=583279 total_ns=9346851053 avg_ns=16024 max_ns=20222554
  source_root_path: count=981163 total_ns=3532778335 avg_ns=3600 max_ns=130667
  resolved_virtual_path: count=1967158 total_ns=9576138345 avg_ns=4868 max_ns=265172
  resolved_virtual_path_from_path: count=1157703 total_ns=7937761825 avg_ns=6856 max_ns=32442
  resolved_virtual_path_from_path_component_walk: count=1157703 total_ns=7385166188 avg_ns=6379 max_ns=31830
  resolved_virtual_path_from_path_canonicalize: count=2472778 total_ns=6730549159 avg_ns=2721 max_ns=30849
  resolved_virtual_path_from_path_source_root_confinement: count=2472778 total_ns=352293897 avg_ns=142 max_ns=14528
  resolved_virtual_path_from_path_virtual_conversion: count=1157703 total_ns=469006353 avg_ns=405 max_ns=17593
  resolved_virtual_path_from_open_fd: count=809455 total_ns=1638376520 avg_ns=2024 max_ns=265172
  read_handle_snapshot: count=37148 total_ns=10625927 avg_ns=286 max_ns=9868
  read_guard_path: count=37148 total_ns=1548018365 avg_ns=41671 max_ns=97194
  read_io: count=37148 total_ns=36833163 avg_ns=991 max_ns=22924
  write_handle_snapshot: count=66256 total_ns=17001747 avg_ns=256 max_ns=7263
  write_guard_mutation: count=66256 total_ns=3570985368 avg_ns=53896 max_ns=348460
  write_io: count=66256 total_ns=87947262 avg_ns=1327 max_ns=281573
  file_sync.flush: count=340 total_ns=3524213 avg_ns=10365 max_ns=119376
  file_sync.fsync: count=196 total_ns=1657632 avg_ns=8457 max_ns=90241
  read_size_bucket.0_4k: count=36608 total_ns=34226333 avg_ns=934 max_ns=9538
  read_size_bucket.4k_64k: count=400 total_ns=762080 avg_ns=1905 max_ns=7124
  read_size_bucket.64k_1m: count=140 total_ns=1844750 avg_ns=13176 max_ns=22924
  write_size_bucket.0_4k: count=66240 total_ns=84193649 avg_ns=1271 max_ns=38924
  write_size_bucket.64k_1m: count=16 total_ns=3753613 avg_ns=234600 max_ns=281573
  readdir_directory_scan: count=31 total_ns=12005435 avg_ns=387272 max_ns=1085574
  readdir_attr_generation_scan: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=31 total_ns=527782 avg_ns=17025 max_ns=93235
  readdir_page_commit: count=31 total_ns=3523138 avg_ns=113649 max_ns=607820
  readdirplus_directory_scan: count=17 total_ns=25719944 avg_ns=1512937 max_ns=2189012
  readdirplus_attr_generation_scan: count=17 total_ns=7120946 avg_ns=418879 max_ns=634751
  readdirplus_attr_generation_entries: count=6800
  readdirplus_symlink_visibility: count=17 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=17 total_ns=1259134 avg_ns=74066 max_ns=126532
  readdirplus_page_commit: count=17 total_ns=2491862 avg_ns=146580 max_ns=309365
  invalidations: count=1800 invalidated_entries=900 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
