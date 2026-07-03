# ScreenFS benchmark result

- timestamp: `2026-07-03T02:06:55.334014+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher20 --cache-control warm --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-bytes 4096 --small-io-ops 64 --sync-bytes 4096 --sync-ops 16 --small-files 200 --dir-entries 500 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `29f0c974548bb323f63687ee18e62ec2627412d7`
- git_dirty_status: `M README.md;  M docs/README.md;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md; ... (+15 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `17ba7b5411da0700f20d6c2cf368c45277422525f3afee0df781aa04a110feb8`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `29f0c974548bb323f63687ee18e62ec2627412d7`
- screenfs_source_git_dirty_status: `M README.md;  M docs/README.md;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md; ... (+15 more)`
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
| seq_read | 0.002047 | 0.002318 | 1.132 | 0.002577 | 0.002610 | 0.002636 |
| seq_write | 0.000515 | 0.001455 | 2.827 | 0.001476 | 0.001479 | 0.001481 |
| small_read | 0.000098 | 0.001123 | 11.429 | 0.001142 | 0.001145 | 0.001147 |
| small_write | 0.000134 | 0.003511 | 26.211 | 0.003516 | 0.003517 | 0.003517 |
| write_fsync_close | 0.000150 | 0.003965 | 26.497 | 0.004233 | 0.004266 | 0.004293 |
| small_stat_open_read | 0.001916 | 0.029566 | 15.428 | 0.030114 | 0.030183 | 0.030238 |
| readdir_lstat | 0.001024 | 0.021925 | 21.406 | 0.023864 | 0.024106 | 0.024300 |
| symlink_open_read | 0.000016 | 0.000191 | 11.913 | 0.000194 | 0.000194 | 0.000195 |
| rand_read_4k | 0.016311 | 0.235914 | 14.463 | 0.238462 | 0.238781 | 0.239036 |
| rand_write_4k | 0.009659 | 0.793668 | 82.173 | 0.802538 | 0.803646 | 0.804533 |
| sync_write_4k | 0.000030 | 0.001155 | 38.010 | 0.001245 | 0.001257 | 0.001266 |
| small_open_read_close | 0.023624 | 0.421794 | 17.855 | 0.429161 | 0.430081 | 0.430818 |
| concurrent_rand_read_write_4k | 0.289312 | 1.431944 | 4.949 | 1.441162 | 1.442314 | 1.443236 |
| metadata_lookup | 0.002579 | 0.018855 | 7.311 | 0.019029 | 0.019050 | 0.019068 |
| metadata_getattr | 0.002904 | 0.026044 | 8.967 | 0.027677 | 0.027882 | 0.028045 |
| metadata_open | 0.002729 | 0.028795 | 10.552 | 0.029457 | 0.029540 | 0.029607 |
| metadata_readlink | 0.000298 | 0.031753 | 106.495 | 0.031848 | 0.031860 | 0.031870 |
| metadata_access | 0.002543 | 0.028349 | 11.146 | 0.028908 | 0.028978 | 0.029034 |
| metadata_statfs | 0.000338 | 0.002010 | 5.951 | 0.002038 | 0.002042 | 0.002045 |
| metadata_opendir | 0.001014 | 0.022008 | 21.697 | 0.022333 | 0.022374 | 0.022406 |
| sync_flush_only | 0.000063 | 0.002853 | 45.154 | 0.002933 | 0.002943 | 0.002951 |
| sync_fsync_only | 0.000032 | 0.001112 | 34.427 | 0.001159 | 0.001165 | 0.001170 |
| sync_release_flush | 0.000048 | 0.002570 | 53.916 | 0.002721 | 0.002740 | 0.002755 |
| read_only_open_close | 0.022515 | 0.238648 | 10.599 | 0.239546 | 0.239658 | 0.239748 |
| read_only_open_read_close | 0.023979 | 0.387826 | 16.174 | 0.391281 | 0.391713 | 0.392058 |
| write_open_write_close | 0.000049 | 0.002750 | 55.588 | 0.002825 | 0.002835 | 0.002842 |
| write_open_fsync_close | 0.000054 | 0.002787 | 52.024 | 0.003003 | 0.003030 | 0.003052 |
| readdir_basic | 0.000122 | 0.001154 | 9.436 | 0.001279 | 0.001294 | 0.001307 |
| readdirplus_basic | 0.000560 | 0.022305 | 39.843 | 0.022530 | 0.022558 | 0.022580 |
| readdir_symlink_visibility | 0.000261 | 0.023938 | 91.742 | 0.024558 | 0.024636 | 0.024698 |
| readdirplus_symlink_visibility | 0.000745 | 0.036320 | 48.736 | 0.037549 | 0.037703 | 0.037825 |
| matcher_descendant_readdir | 0.000019 | 0.000353 | 18.195 | 0.000402 | 0.000408 | 0.000413 |
| matcher_descendant_readdirplus | 0.000038 | 0.001561 | 41.385 | 0.001600 | 0.001605 | 0.001609 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.005294 | 0.005356 | 0.005364 | 0.005370 |
| matcher_hidden_stat_miss | 0.006670 | 0.007348 | 0.007433 | 0.007500 |
| symlink_parent_mkdir_rmdir | 0.141315 | 0.141845 | 0.141912 | 0.141965 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=29195288 avg_ns=14248 max_ns=103747
  fuse_op.create: count=116 total_ns=8778301 avg_ns=75675 max_ns=220806
  fuse_op.flush: count=356 total_ns=1905000 avg_ns=5351 max_ns=146092
  fuse_op.fsync: count=196 total_ns=556792 avg_ns=2840 max_ns=9252
  fuse_op.getattr: count=390617 total_ns=4225600787 avg_ns=10817 max_ns=747126
  fuse_op.getxattr: count=328660 total_ns=6957314729 avg_ns=21168 max_ns=1011386
  fuse_op.lookup: count=232661 total_ns=1910139745 avg_ns=8209 max_ns=15715183
  fuse_op.mkdir: count=800 total_ns=54106536 avg_ns=67633 max_ns=182697
  fuse_op.open: count=52272 total_ns=767766982 avg_ns=14687 max_ns=1093925
  fuse_op.opendir: count=2080 total_ns=17013714 avg_ns=8179 max_ns=29737
  fuse_op.read: count=50617 total_ns=788549867 avg_ns=15578 max_ns=1016580
  fuse_op.readdir: count=54 total_ns=47273859 avg_ns=875441 max_ns=8494126
  fuse_op.readdirplus: count=34 total_ns=88834953 avg_ns=2612792 max_ns=15942653
  fuse_op.readlink: count=4456 total_ns=100569365 avg_ns=22569 max_ns=98053
  fuse_op.release: count=52388 total_ns=24398935 avg_ns=465 max_ns=10574
  fuse_op.releasedir: count=2080 total_ns=1496434 avg_ns=719 max_ns=121578
  fuse_op.rmdir: count=800 total_ns=124075027 avg_ns=155093 max_ns=470413
  fuse_op.setattr: count=20 total_ns=1133783 avg_ns=56689 max_ns=122114
  fuse_op.statfs: count=2050 total_ns=344238 avg_ns=167 max_ns=1672
  fuse_op.unlink: count=116 total_ns=9429777 avg_ns=81291 max_ns=345762
  fuse_op.write: count=328400 total_ns=11837591754 avg_ns=36046 max_ns=2054125
  policy_decision: count=3518334 total_ns=4269397384 avg_ns=1213 max_ns=990013
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
  matcher_candidate_order.descendant: count=2855402 total_ns=1212331932 avg_ns=424 max_ns=980342
  matcher_candidate_order.path: count=9892070 total_ns=4325486181 avg_ns=437 max_ns=2011929
  matcher_candidate_order_by_source.hidden.path: count=2855402 total_ns=1682875115 avg_ns=589 max_ns=435909
  matcher_candidate_order_by_source.internal_hidden.path: count=2855402 total_ns=474032996 avg_ns=166 max_ns=2011929
  matcher_candidate_order_by_source.readonly.path: count=662932 total_ns=434860665 avg_ns=655 max_ns=93065
  matcher_candidate_order_by_source.visible.descendant: count=2855402 total_ns=1212331932 avg_ns=424 max_ns=980342
  matcher_candidate_order_by_source.visible.path: count=2855402 total_ns=1619410254 avg_ns=567 max_ns=320371
  matcher_candidate_order_by_source.writable.path: count=662932 total_ns=114307151 avg_ns=172 max_ns=18633
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=190956496
  matcher_candidate_order_seen_slots.descendant: count=57108040
  matcher_candidate_order_seen_slots.path: count=133848456
  matcher_candidate_order_ancestor_steps: count=50087436
  matcher_candidate_order_ancestor_steps.descendant: count=11195427
  matcher_candidate_order_ancestor_steps.path: count=38892009
  state_read_lock_wait: count=1394304 total_ns=53266601 avg_ns=38 max_ns=36050
  state_read_lock_hold: count=1394304 total_ns=96643024 avg_ns=69 max_ns=50665
  state_write_lock_wait: count=337263 total_ns=5509137 avg_ns=16 max_ns=8614
  state_write_lock_hold: count=337263 total_ns=199007925 avg_ns=590 max_ns=1342189
  open_confined_openat2: count=1792093 total_ns=1072214917 avg_ns=598 max_ns=15698833
  open_like.pre_open_guard.access: count=2049 total_ns=18399959 avg_ns=8979 max_ns=83573
  open_like.pre_open_guard.open: count=52272 total_ns=487132727 avg_ns=9319 max_ns=762401
  open_like.pre_open_guard.opendir: count=2080 total_ns=13209422 avg_ns=6350 max_ns=20145
  open_like.post_open_revalidation.access: count=2049 total_ns=8942245 avg_ns=4364 max_ns=12929
  open_like.post_open_revalidation.open: count=52272 total_ns=233182296 avg_ns=4460 max_ns=330248
  open_like.post_open_revalidation.opendir: count=2080 total_ns=1843173 avg_ns=886 max_ns=11527
  stat_child_no_follow: count=1405092 total_ns=1556100167 avg_ns=1107 max_ns=15699992
  stat_child_no_follow.attr_conversion: count=1396858 total_ns=19189878 avg_ns=13 max_ns=5595
  stat_child_no_follow.directory_revalidation: count=4456 total_ns=30522615 avg_ns=6849 max_ns=45693
  stat_child_no_follow.host_fstat: count=1392402 total_ns=193431971 avg_ns=138 max_ns=17784
  stat_child_no_follow.host_fstatat: count=4456 total_ns=888725 avg_ns=199 max_ns=1647
  stat_child_no_follow.parent_open: count=4456 total_ns=3013848 avg_ns=676 max_ns=6353
  stat_child_no_follow_context.path_guard_or_metadata: count=1400636 total_ns=1520732850 avg_ns=1085 max_ns=15699992
  stat_child_no_follow_context.readlink_pre_open: count=4456 total_ns=35367317 avg_ns=7937 max_ns=54594
  source_root_path: count=1731804 total_ns=2241269684 avg_ns=1294 max_ns=342127
  resolved_virtual_path: count=2174428 total_ns=5775752758 avg_ns=2656 max_ns=395987
  resolved_virtual_path_from_path: count=1403838 total_ns=5024996660 avg_ns=3579 max_ns=395987
  resolved_virtual_path_from_path_component_walk: count=1403838 total_ns=4448627521 avg_ns=3168 max_ns=395547
  resolved_virtual_path_from_path_canonicalize: count=3978052 total_ns=3663455179 avg_ns=920 max_ns=394798
  resolved_virtual_path_from_path_source_root_confinement: count=3978052 total_ns=441395513 avg_ns=110 max_ns=276073
  resolved_virtual_path_from_path_virtual_conversion: count=1403838 total_ns=504531301 avg_ns=359 max_ns=328024
  resolved_virtual_path_from_open_fd: count=770590 total_ns=750756098 avg_ns=974 max_ns=277851
  read_handle_snapshot: count=50617 total_ns=12260106 avg_ns=242 max_ns=50801
  read_guard_path: count=50617 total_ns=744594951 avg_ns=14710 max_ns=690507
  read_io: count=50617 total_ns=22321021 avg_ns=440 max_ns=325449
  write_handle_snapshot: count=328400 total_ns=89263268 avg_ns=271 max_ns=12878
  write_guard_mutation: count=328400 total_ns=11403281779 avg_ns=34723 max_ns=2052083
  write_io: count=328400 total_ns=264242620 avg_ns=804 max_ns=349661
  file_sync.flush: count=356 total_ns=1799957 avg_ns=5056 max_ns=145099
  file_sync.fsync: count=196 total_ns=508793 avg_ns=2595 max_ns=8615
  read_size_bucket.0_4k: count=48407 total_ns=18482984 avg_ns=381 max_ns=325449
  read_size_bucket.4k_64k: count=2070 total_ns=2797191 avg_ns=1351 max_ns=12955
  read_size_bucket.64k_1m: count=140 total_ns=1040846 avg_ns=7434 max_ns=31088
  write_size_bucket.0_4k: count=328384 total_ns=263549029 avg_ns=802 max_ns=349661
  write_size_bucket.64k_1m: count=16 total_ns=693591 avg_ns=43349 max_ns=57201
  readdir_directory_scan: count=54 total_ns=39801274 avg_ns=737060 max_ns=7107464
  readdir_scan.name_child_path_materialization: count=54 total_ns=1892581 avg_ns=35047 max_ns=209556
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=98875 avg_ns=1831 max_ns=10349
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=34788527 avg_ns=644231 max_ns=6673311
  readdir_candidate_selection: count=54 total_ns=651391 avg_ns=12062 max_ns=72827
  readdir_page_commit: count=54 total_ns=6090689 avg_ns=112790 max_ns=1342464
  readdirplus_directory_scan: count=34 total_ns=58112112 avg_ns=1709179 max_ns=11450224
  readdirplus_scan.name_child_path_materialization: count=34 total_ns=2756059 avg_ns=81060 max_ns=287965
  readdirplus_scan.returned_attr_hydration: count=4816 total_ns=4666900 avg_ns=969 max_ns=9752
  readdirplus_scan.returned_policy_recheck: count=4816 total_ns=755134 avg_ns=156 max_ns=5724
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=20025238 avg_ns=12902 max_ns=31833
  readdirplus_scan.scan_fallback_attr: count=34 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=34 total_ns=847745 avg_ns=24933 max_ns=98216
  readdirplus_attr_generation_scan: count=4850 total_ns=4666900 avg_ns=962 max_ns=9752
  readdirplus_attr_generation_entries: count=4816
  readdirplus_symlink_visibility: count=34 total_ns=50902201 avg_ns=1497123 max_ns=10802184
  readdirplus_candidate_selection: count=34 total_ns=1456833 avg_ns=42848 max_ns=160010
  readdirplus_page_commit: count=34 total_ns=3498805 avg_ns=102906 max_ns=285780
  invalidations: count=1832 invalidated_entries=916 evicted_entries=0 scanned_entries=1024720
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
