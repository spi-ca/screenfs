# ScreenFS benchmark result

- timestamp: `2026-06-29T16:30:32.572819+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `5c5658883a894a302a12b92be54cf2d0fa009cb5`
- git_dirty_status: `M .pi/skills/run-screenfs-benchmarks/SKILL.md;  M docs/architecture.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-perf-counter-baseline-summary.md;  M docs/artifacts/current-perf-counter-benchmark-result.json; ... (+30 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `bd1820eb37a87ad34655c2b79dfe4bbe7edda2e9f73a44e5944df4ae7be03eae`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `5c5658883a894a302a12b92be54cf2d0fa009cb5`
- screenfs_source_git_dirty_status: `M .pi/skills/run-screenfs-benchmarks/SKILL.md;  M docs/architecture.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-perf-counter-baseline-summary.md;  M docs/artifacts/current-perf-counter-benchmark-result.json; ... (+30 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher20`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `all`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, write_fsync_close, small_stat_open_read, readdir_lstat, symlink_open_read, rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close, concurrent_rand_read_write_4k, metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs, metadata_opendir, sync_flush_only, sync_fsync_only, sync_release_flush, read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close, readdir_basic, readdirplus_basic, readdir_symlink_visibility, readdirplus_symlink_visibility, matcher_descendant_readdir, matcher_descendant_readdirplus`
- screenfs_only_workloads: `hidden_stat_miss, matcher_hidden_stat_miss, symlink_parent_mkdir_rmdir`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.001628 | 0.004033 | 2.477 | 0.004304 | 0.004338 | 0.004365 |
| seq_write | 0.000987 | 0.003218 | 3.261 | 0.003279 | 0.003287 | 0.003293 |
| small_read | 0.000052 | 0.002106 | 40.548 | 0.002135 | 0.002139 | 0.002141 |
| small_write | 0.000075 | 0.004611 | 61.426 | 0.004789 | 0.004811 | 0.004828 |
| write_fsync_close | 0.000101 | 0.005736 | 56.868 | 0.005752 | 0.005754 | 0.005756 |
| small_stat_open_read | 0.001074 | 0.035009 | 32.596 | 0.040647 | 0.041351 | 0.041915 |
| readdir_lstat | 0.000491 | 0.028523 | 58.069 | 0.032419 | 0.032906 | 0.033296 |
| symlink_open_read | 0.000009 | 0.000178 | 19.531 | 0.000217 | 0.000222 | 0.000226 |
| rand_read_4k | 0.014738 | 0.361181 | 24.507 | 0.362225 | 0.362356 | 0.362461 |
| rand_write_4k | 0.012911 | 1.128439 | 87.403 | 1.133703 | 1.134361 | 1.134887 |
| sync_write_4k | 0.000031 | 0.001506 | 48.135 | 0.001525 | 0.001528 | 0.001529 |
| small_open_read_close | 0.028949 | 0.642724 | 22.202 | 0.651393 | 0.652476 | 0.653343 |
| concurrent_rand_read_write_4k | 0.211617 | 2.101984 | 9.933 | 2.201107 | 2.213498 | 2.223410 |
| metadata_lookup | 0.003261 | 0.019926 | 6.110 | 0.020083 | 0.020103 | 0.020119 |
| metadata_getattr | 0.003447 | 0.025641 | 7.439 | 0.026617 | 0.026738 | 0.026836 |
| metadata_open | 0.003070 | 0.029593 | 9.639 | 0.029927 | 0.029969 | 0.030002 |
| metadata_readlink | 0.000313 | 0.032977 | 105.306 | 0.035081 | 0.035344 | 0.035555 |
| metadata_access | 0.002770 | 0.029037 | 10.481 | 0.029498 | 0.029556 | 0.029602 |
| metadata_statfs | 0.000331 | 0.002107 | 6.371 | 0.002109 | 0.002109 | 0.002109 |
| metadata_opendir | 0.001100 | 0.022557 | 20.510 | 0.022623 | 0.022631 | 0.022637 |
| sync_flush_only | 0.000076 | 0.002656 | 35.117 | 0.002860 | 0.002886 | 0.002907 |
| sync_fsync_only | 0.000034 | 0.001137 | 33.653 | 0.001185 | 0.001192 | 0.001196 |
| sync_release_flush | 0.000053 | 0.002421 | 45.718 | 0.002478 | 0.002486 | 0.002491 |
| read_only_open_close | 0.025302 | 0.249228 | 9.850 | 0.320956 | 0.329922 | 0.337095 |
| read_only_open_read_close | 0.028947 | 0.481087 | 16.620 | 0.504395 | 0.507308 | 0.509639 |
| write_open_write_close | 0.000053 | 0.003846 | 72.951 | 0.004476 | 0.004554 | 0.004617 |
| write_open_fsync_close | 0.000054 | 0.003662 | 68.227 | 0.004663 | 0.004788 | 0.004889 |
| readdir_basic | 0.000106 | 0.004917 | 46.237 | 0.005061 | 0.005079 | 0.005094 |
| readdirplus_basic | 0.000586 | 0.037281 | 63.668 | 0.037825 | 0.037893 | 0.037947 |
| readdir_symlink_visibility | 0.000207 | 0.019723 | 95.160 | 0.020134 | 0.020186 | 0.020227 |
| readdirplus_symlink_visibility | 0.000659 | 0.058091 | 88.150 | 0.058999 | 0.059113 | 0.059204 |
| matcher_descendant_readdir | 0.000019 | 0.000466 | 24.797 | 0.000476 | 0.000477 | 0.000478 |
| matcher_descendant_readdirplus | 0.000036 | 0.002465 | 69.193 | 0.002515 | 0.002521 | 0.002526 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.006403 | 0.006959 | 0.007028 | 0.007084 |
| matcher_hidden_stat_miss | 0.009765 | 0.010953 | 0.011101 | 0.011220 |
| symlink_parent_mkdir_rmdir | 0.199978 | 0.200797 | 0.200899 | 0.200981 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=29211491 avg_ns=14256 max_ns=85730
  fuse_op.create: count=116 total_ns=11317151 avg_ns=97561 max_ns=166091
  fuse_op.flush: count=356 total_ns=3136801 avg_ns=8811 max_ns=258521
  fuse_op.fsync: count=196 total_ns=1045905 avg_ns=5336 max_ns=150130
  fuse_op.getattr: count=390617 total_ns=5668193929 avg_ns=14510 max_ns=2060664
  fuse_op.getxattr: count=328660 total_ns=9337004476 avg_ns=28409 max_ns=2132197
  fuse_op.lookup: count=232661 total_ns=2411945036 avg_ns=10366 max_ns=13997133
  fuse_op.mkdir: count=800 total_ns=78284942 avg_ns=97856 max_ns=218693
  fuse_op.open: count=52272 total_ns=943485109 avg_ns=18049 max_ns=278241
  fuse_op.opendir: count=2080 total_ns=17251584 avg_ns=8294 max_ns=32017
  fuse_op.read: count=50623 total_ns=1055940092 avg_ns=20858 max_ns=1274289
  fuse_op.readdir: count=54 total_ns=67100013 avg_ns=1242592 max_ns=6082336
  fuse_op.readdirplus: count=34 total_ns=147142597 avg_ns=4327723 max_ns=13817199
  fuse_op.readlink: count=4456 total_ns=124776839 avg_ns=28001 max_ns=75710
  fuse_op.release: count=52388 total_ns=29336477 avg_ns=559 max_ns=17076
  fuse_op.releasedir: count=2080 total_ns=1848515 avg_ns=888 max_ns=155476
  fuse_op.rmdir: count=800 total_ns=164930311 avg_ns=206162 max_ns=358913
  fuse_op.setattr: count=20 total_ns=1183843 avg_ns=59192 max_ns=92333
  fuse_op.statfs: count=2050 total_ns=362567 avg_ns=176 max_ns=5229
  fuse_op.unlink: count=116 total_ns=15793423 avg_ns=136150 max_ns=676301
  fuse_op.write: count=328400 total_ns=15917254655 avg_ns=48469 max_ns=3905590
  policy_decision: count=3540002 total_ns=5106108978 avg_ns=1442 max_ns=997923
  matcher_candidates: count=1544972
  matcher_candidates_by_source.hidden.path: count=2192
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=1542780
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1544972
  matcher_candidate_order.descendant: count=2877070 total_ns=1466017880 avg_ns=509 max_ns=2024753
  matcher_candidate_order.path: count=9957074 total_ns=5281732377 avg_ns=530 max_ns=441579
  matcher_candidate_order_by_source.hidden.path: count=2877070 total_ns=2048035095 avg_ns=711 max_ns=441579
  matcher_candidate_order_by_source.internal_hidden.path: count=2877070 total_ns=560154190 avg_ns=194 max_ns=382270
  matcher_candidate_order_by_source.readonly.path: count=662932 total_ns=552785082 avg_ns=833 max_ns=269097
  matcher_candidate_order_by_source.visible.descendant: count=2877070 total_ns=1466017880 avg_ns=509 max_ns=2024753
  matcher_candidate_order_by_source.visible.path: count=2877070 total_ns=1986783726 avg_ns=690 max_ns=391101
  matcher_candidate_order_by_source.writable.path: count=662932 total_ns=133974284 avg_ns=202 max_ns=253012
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=192299912
  matcher_candidate_order_seen_slots.descendant: count=57541400
  matcher_candidate_order_seen_slots.path: count=134758512
  matcher_candidate_order_ancestor_steps: count=50434124
  matcher_candidate_order_ancestor_steps.descendant: count=11282099
  matcher_candidate_order_ancestor_steps.path: count=39152025
  state_read_lock_wait: count=1394310 total_ns=86971976 avg_ns=62 max_ns=101928
  state_read_lock_hold: count=1394310 total_ns=137842332 avg_ns=98 max_ns=124872
  state_write_lock_wait: count=337263 total_ns=6867475 avg_ns=20 max_ns=5820
  state_write_lock_hold: count=337263 total_ns=255639826 avg_ns=757 max_ns=653754
  open_confined_openat2: count=1792099 total_ns=1609890562 avg_ns=898 max_ns=13980688
  open_like.pre_open_guard.access: count=2049 total_ns=18337123 avg_ns=8949 max_ns=50321
  open_like.pre_open_guard.open: count=52272 total_ns=596444287 avg_ns=11410 max_ns=115767
  open_like.pre_open_guard.opendir: count=2080 total_ns=13435302 avg_ns=6459 max_ns=22250
  open_like.post_open_revalidation.access: count=2049 total_ns=9120549 avg_ns=4451 max_ns=47993
  open_like.post_open_revalidation.open: count=52272 total_ns=285680437 avg_ns=5465 max_ns=267468
  open_like.post_open_revalidation.opendir: count=2080 total_ns=1908795 avg_ns=917 max_ns=3093
  stat_child_no_follow: count=1405098 total_ns=2257931483 avg_ns=1606 max_ns=13982417
  stat_child_no_follow.attr_conversion: count=1396864 total_ns=22972878 avg_ns=16 max_ns=33385
  stat_child_no_follow.directory_revalidation: count=4456 total_ns=38170679 avg_ns=8566 max_ns=33659
  stat_child_no_follow.host_fstat: count=1392408 total_ns=283766335 avg_ns=203 max_ns=1158070
  stat_child_no_follow.host_fstatat: count=4456 total_ns=1485813 avg_ns=333 max_ns=2441
  stat_child_no_follow.parent_open: count=4456 total_ns=3752617 avg_ns=842 max_ns=14037
  stat_child_no_follow_context.path_guard_or_metadata: count=1400642 total_ns=2213512059 avg_ns=1580 max_ns=13982417
  stat_child_no_follow_context.readlink_pre_open: count=4456 total_ns=44419424 avg_ns=9968 max_ns=47165
  source_root_path: count=1731810 total_ns=3369887027 avg_ns=1945 max_ns=948767
  resolved_virtual_path: count=2174440 total_ns=7038092033 avg_ns=3236 max_ns=2011897
  resolved_virtual_path_from_path: count=1403844 total_ns=6040302040 avg_ns=4302 max_ns=2011897
  resolved_virtual_path_from_path_component_walk: count=1403844 total_ns=5369231079 avg_ns=3824 max_ns=2009973
  resolved_virtual_path_from_path_canonicalize: count=3978070 total_ns=4443704219 avg_ns=1117 max_ns=2008879
  resolved_virtual_path_from_path_source_root_confinement: count=3978070 total_ns=490572753 avg_ns=123 max_ns=267505
  resolved_virtual_path_from_path_virtual_conversion: count=1403844 total_ns=584958191 avg_ns=416 max_ns=330873
  resolved_virtual_path_from_open_fd: count=770596 total_ns=997789993 avg_ns=1294 max_ns=625711
  read_handle_snapshot: count=50623 total_ns=15137145 avg_ns=299 max_ns=260069
  read_guard_path: count=50623 total_ns=974340531 avg_ns=19246 max_ns=1268182
  read_io: count=50623 total_ns=55267557 avg_ns=1091 max_ns=111951
  write_handle_snapshot: count=328400 total_ns=118423108 avg_ns=360 max_ns=102335
  write_guard_mutation: count=328400 total_ns=15268482649 avg_ns=46493 max_ns=3900305
  write_io: count=328400 total_ns=426036611 avg_ns=1297 max_ns=457670
  file_sync.flush: count=356 total_ns=3011891 avg_ns=8460 max_ns=258095
  file_sync.fsync: count=196 total_ns=990083 avg_ns=5051 max_ns=149096
  read_size_bucket.0_4k: count=48412 total_ns=45388549 avg_ns=937 max_ns=111951
  read_size_bucket.4k_64k: count=2071 total_ns=5966972 avg_ns=2881 max_ns=19519
  read_size_bucket.64k_1m: count=140 total_ns=3912036 avg_ns=27943 max_ns=49182
  write_size_bucket.0_4k: count=328384 total_ns=422565090 avg_ns=1286 max_ns=457670
  write_size_bucket.64k_1m: count=16 total_ns=3471521 avg_ns=216970 max_ns=298915
  readdir_directory_scan: count=54 total_ns=59100978 avg_ns=1094462 max_ns=5580598
  readdir_scan.name_child_path_materialization: count=54 total_ns=1855573 avg_ns=34362 max_ns=109928
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=21723291 avg_ns=402283 max_ns=1407481
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=28951461 avg_ns=536138 max_ns=3791991
  readdir_candidate_selection: count=54 total_ns=995491 avg_ns=18435 max_ns=79827
  readdir_page_commit: count=54 total_ns=6099962 avg_ns=112962 max_ns=653878
  readdirplus_directory_scan: count=34 total_ns=97222488 avg_ns=2859484 max_ns=9864624
  readdirplus_scan.name_child_path_materialization: count=34 total_ns=2855123 avg_ns=83974 max_ns=157975
  readdirplus_scan.returned_attr_hydration: count=4816 total_ns=7183284 avg_ns=1491 max_ns=51740
  readdirplus_scan.returned_policy_recheck: count=4816 total_ns=18127457 avg_ns=3764 max_ns=81330
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=17998060 avg_ns=11596 max_ns=73227
  readdirplus_scan.scan_fallback_attr: count=34 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=34 total_ns=40086255 avg_ns=1179007 max_ns=2625469
  readdirplus_attr_generation_scan: count=4850 total_ns=7183284 avg_ns=1481 max_ns=51740
  readdirplus_attr_generation_entries: count=4816
  readdirplus_symlink_visibility: count=34 total_ns=48450693 avg_ns=1425020 max_ns=6712243
  readdirplus_candidate_selection: count=34 total_ns=1882775 avg_ns=55375 max_ns=135507
  readdirplus_page_commit: count=34 total_ns=4268558 avg_ns=125545 max_ns=446433
  invalidations: count=1832 invalidated_entries=916 evicted_entries=0 scanned_entries=1024720
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
