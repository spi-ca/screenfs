# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:07.953286+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher20 --cache-control warm --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-bytes 4096 --small-io-ops 64 --sync-bytes 4096 --sync-ops 16 --small-files 200 --dir-entries 500 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+23 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+23 more)`
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
| seq_read | 0.001270 | 0.003135 | 2.467 | 0.003165 | 0.003169 | 0.003172 |
| seq_write | 0.000590 | 0.002922 | 4.950 | 0.003002 | 0.003012 | 0.003020 |
| small_read | 0.000041 | 0.001716 | 42.190 | 0.001723 | 0.001724 | 0.001725 |
| small_write | 0.000059 | 0.004774 | 81.576 | 0.005103 | 0.005144 | 0.005177 |
| write_fsync_close | 0.000088 | 0.005816 | 66.445 | 0.006344 | 0.006411 | 0.006463 |
| small_stat_open_read | 0.001053 | 0.044036 | 41.820 | 0.044328 | 0.044364 | 0.044393 |
| readdir_lstat | 0.000557 | 0.033777 | 60.673 | 0.034114 | 0.034156 | 0.034190 |
| symlink_open_read | 0.000011 | 0.000230 | 21.765 | 0.000284 | 0.000290 | 0.000296 |
| rand_read_4k | 0.011928 | 0.330741 | 27.728 | 0.331707 | 0.331828 | 0.331925 |
| rand_write_4k | 0.012413 | 0.914337 | 73.659 | 1.065968 | 1.084922 | 1.100085 |
| sync_write_4k | 0.000029 | 0.001250 | 43.337 | 0.001256 | 0.001256 | 0.001257 |
| small_open_read_close | 0.029010 | 0.461658 | 15.914 | 0.465085 | 0.465513 | 0.465856 |
| concurrent_rand_read_write_4k | 0.205883 | 1.494381 | 7.258 | 1.638893 | 1.656957 | 1.671409 |
| metadata_lookup | 0.003060 | 0.026677 | 8.718 | 0.028076 | 0.028250 | 0.028390 |
| metadata_getattr | 0.003294 | 0.036932 | 11.213 | 0.037203 | 0.037237 | 0.037264 |
| metadata_open | 0.002929 | 0.042446 | 14.493 | 0.042612 | 0.042632 | 0.042649 |
| metadata_readlink | 0.000275 | 0.045020 | 163.924 | 0.045078 | 0.045085 | 0.045091 |
| metadata_access | 0.002768 | 0.039730 | 14.353 | 0.042032 | 0.042320 | 0.042550 |
| metadata_statfs | 0.000335 | 0.002594 | 7.749 | 0.002601 | 0.002601 | 0.002602 |
| metadata_opendir | 0.000890 | 0.032804 | 36.874 | 0.033005 | 0.033030 | 0.033050 |
| sync_flush_only | 0.000066 | 0.003772 | 56.871 | 0.003881 | 0.003895 | 0.003906 |
| sync_fsync_only | 0.000030 | 0.001697 | 56.091 | 0.001813 | 0.001828 | 0.001839 |
| sync_release_flush | 0.000050 | 0.003451 | 69.052 | 0.003693 | 0.003724 | 0.003748 |
| read_only_open_close | 0.024899 | 0.258636 | 10.387 | 0.288863 | 0.292641 | 0.295664 |
| read_only_open_read_close | 0.026593 | 0.403372 | 15.168 | 0.414258 | 0.415619 | 0.416707 |
| write_open_write_close | 0.000052 | 0.002709 | 51.628 | 0.002808 | 0.002820 | 0.002830 |
| write_open_fsync_close | 0.000053 | 0.002863 | 53.981 | 0.002920 | 0.002927 | 0.002933 |
| readdir_basic | 0.000107 | 0.001246 | 11.626 | 0.001250 | 0.001250 | 0.001250 |
| readdirplus_basic | 0.000506 | 0.024499 | 48.402 | 0.024784 | 0.024819 | 0.024848 |
| readdir_symlink_visibility | 0.000209 | 0.009695 | 46.440 | 0.009726 | 0.009730 | 0.009733 |
| readdirplus_symlink_visibility | 0.000667 | 0.038418 | 57.578 | 0.038577 | 0.038597 | 0.038613 |
| matcher_descendant_readdir | 0.000018 | 0.000363 | 20.056 | 0.000408 | 0.000413 | 0.000418 |
| matcher_descendant_readdirplus | 0.000036 | 0.001796 | 50.426 | 0.001947 | 0.001966 | 0.001981 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.005108 | 0.005134 | 0.005137 | 0.005140 |
| matcher_hidden_stat_miss | 0.007415 | 0.007607 | 0.007631 | 0.007651 |
| symlink_parent_mkdir_rmdir | 0.148915 | 0.149529 | 0.149606 | 0.149667 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=40759050 avg_ns=19892 max_ns=76665
  fuse_op.create: count=116 total_ns=11016489 avg_ns=94969 max_ns=155988
  fuse_op.flush: count=356 total_ns=2932184 avg_ns=8236 max_ns=227288
  fuse_op.fsync: count=196 total_ns=1033212 avg_ns=5271 max_ns=67729
  fuse_op.getattr: count=390617 total_ns=4653538943 avg_ns=11913 max_ns=668896
  fuse_op.getxattr: count=328660 total_ns=7498679516 avg_ns=22815 max_ns=1599200
  fuse_op.lookup: count=232661 total_ns=2188650886 avg_ns=9407 max_ns=14616594
  fuse_op.mkdir: count=800 total_ns=57728712 avg_ns=72160 max_ns=204696
  fuse_op.open: count=52272 total_ns=861996557 avg_ns=16490 max_ns=536252
  fuse_op.opendir: count=2080 total_ns=24161317 avg_ns=11616 max_ns=89252
  fuse_op.read: count=50621 total_ns=880453825 avg_ns=17393 max_ns=570761
  fuse_op.readdir: count=54 total_ns=32805024 avg_ns=607500 max_ns=3297838
  fuse_op.readdirplus: count=34 total_ns=64640403 avg_ns=1901188 max_ns=6640065
  fuse_op.readlink: count=4456 total_ns=123067278 avg_ns=27618 max_ns=81042
  fuse_op.release: count=52388 total_ns=26756864 avg_ns=510 max_ns=59512
  fuse_op.releasedir: count=2080 total_ns=2121126 avg_ns=1019 max_ns=102229
  fuse_op.rmdir: count=800 total_ns=130655533 avg_ns=163319 max_ns=300637
  fuse_op.setattr: count=20 total_ns=1099865 avg_ns=54993 max_ns=83411
  fuse_op.statfs: count=2050 total_ns=541369 avg_ns=264 max_ns=2910
  fuse_op.unlink: count=116 total_ns=15475405 avg_ns=133408 max_ns=640168
  fuse_op.write: count=328400 total_ns=12767576115 avg_ns=38878 max_ns=2158704
  policy_decision: count=3518342 total_ns=4734343590 avg_ns=1345 max_ns=668315
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
  matcher_candidate_order.descendant: count=2855410 total_ns=1330503139 avg_ns=465 max_ns=648981
  matcher_candidate_order.path: count=9892094 total_ns=4840876208 avg_ns=489 max_ns=2036351
  matcher_candidate_order_by_source.hidden.path: count=2855410 total_ns=1883225191 avg_ns=659 max_ns=2036351
  matcher_candidate_order_by_source.internal_hidden.path: count=2855410 total_ns=513037807 avg_ns=179 max_ns=614865
  matcher_candidate_order_by_source.readonly.path: count=662932 total_ns=498227210 avg_ns=751 max_ns=285768
  matcher_candidate_order_by_source.visible.descendant: count=2855410 total_ns=1330503139 avg_ns=465 max_ns=648981
  matcher_candidate_order_by_source.visible.path: count=2855410 total_ns=1826142347 avg_ns=639 max_ns=326785
  matcher_candidate_order_by_source.writable.path: count=662932 total_ns=120243653 avg_ns=181 max_ns=264469
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=190956992
  matcher_candidate_order_seen_slots.descendant: count=57108200
  matcher_candidate_order_seen_slots.path: count=133848792
  matcher_candidate_order_ancestor_steps: count=50087564
  matcher_candidate_order_ancestor_steps.descendant: count=11195459
  matcher_candidate_order_ancestor_steps.path: count=38892105
  state_read_lock_wait: count=1394308 total_ns=56471933 avg_ns=40 max_ns=23350
  state_read_lock_hold: count=1394308 total_ns=102033267 avg_ns=73 max_ns=54355
  state_write_lock_wait: count=337263 total_ns=6420229 avg_ns=19 max_ns=16520
  state_write_lock_hold: count=337263 total_ns=220480492 avg_ns=653 max_ns=586109
  open_confined_openat2: count=1792097 total_ns=1183898145 avg_ns=660 max_ns=14590237
  open_like.pre_open_guard.access: count=2049 total_ns=25702968 avg_ns=12544 max_ns=62658
  open_like.pre_open_guard.open: count=52272 total_ns=545101327 avg_ns=10428 max_ns=526877
  open_like.pre_open_guard.opendir: count=2080 total_ns=18579069 avg_ns=8932 max_ns=34181
  open_like.post_open_revalidation.access: count=2049 total_ns=12290137 avg_ns=5998 max_ns=15076
  open_like.post_open_revalidation.open: count=52272 total_ns=263140337 avg_ns=5034 max_ns=71845
  open_like.post_open_revalidation.opendir: count=2080 total_ns=2752865 avg_ns=1323 max_ns=4926
  stat_child_no_follow: count=1405096 total_ns=1726190295 avg_ns=1228 max_ns=14596785
  stat_child_no_follow.attr_conversion: count=1396862 total_ns=21191144 avg_ns=15 max_ns=8427
  stat_child_no_follow.directory_revalidation: count=4456 total_ns=37565103 avg_ns=8430 max_ns=25177
  stat_child_no_follow.host_fstat: count=1392406 total_ns=226294346 avg_ns=162 max_ns=210787
  stat_child_no_follow.host_fstatat: count=4456 total_ns=1374304 avg_ns=308 max_ns=3227
  stat_child_no_follow.parent_open: count=4456 total_ns=3678021 avg_ns=825 max_ns=12671
  stat_child_no_follow_context.path_guard_or_metadata: count=1400640 total_ns=1682543582 avg_ns=1201 max_ns=14596785
  stat_child_no_follow_context.readlink_pre_open: count=4456 total_ns=43646713 avg_ns=9795 max_ns=30581
  source_root_path: count=1731808 total_ns=2563295312 avg_ns=1480 max_ns=672655
  resolved_virtual_path: count=2174436 total_ns=6281094491 avg_ns=2888 max_ns=669819
  resolved_virtual_path_from_path: count=1403842 total_ns=5454456927 avg_ns=3885 max_ns=669819
  resolved_virtual_path_from_path_component_walk: count=1403842 total_ns=4826238894 avg_ns=3437 max_ns=668893
  resolved_virtual_path_from_path_canonicalize: count=3978064 total_ns=3976502445 avg_ns=999 max_ns=668016
  resolved_virtual_path_from_path_source_root_confinement: count=3978064 total_ns=457978842 avg_ns=115 max_ns=632866
  resolved_virtual_path_from_path_virtual_conversion: count=1403842 total_ns=550486728 avg_ns=392 max_ns=488623
  resolved_virtual_path_from_open_fd: count=770594 total_ns=826637564 avg_ns=1072 max_ns=634892
  read_handle_snapshot: count=50621 total_ns=13771371 avg_ns=272 max_ns=19583
  read_guard_path: count=50621 total_ns=828295831 avg_ns=16362 max_ns=566715
  read_io: count=50621 total_ns=28077777 avg_ns=554 max_ns=50303
  write_handle_snapshot: count=328400 total_ns=88732352 avg_ns=270 max_ns=54457
  write_guard_mutation: count=328400 total_ns=12246970080 avg_ns=37292 max_ns=2151287
  write_io: count=328400 total_ns=355239026 avg_ns=1081 max_ns=242677
  file_sync.flush: count=356 total_ns=2813222 avg_ns=7902 max_ns=225711
  file_sync.fsync: count=196 total_ns=974489 avg_ns=4971 max_ns=67281
  read_size_bucket.0_4k: count=48413 total_ns=21840472 avg_ns=451 max_ns=50303
  read_size_bucket.4k_64k: count=2068 total_ns=3787523 avg_ns=1831 max_ns=15118
  read_size_bucket.64k_1m: count=140 total_ns=2449782 avg_ns=17498 max_ns=38812
  write_size_bucket.0_4k: count=328384 total_ns=352426810 avg_ns=1073 max_ns=173771
  write_size_bucket.64k_1m: count=16 total_ns=2812216 avg_ns=175763 max_ns=242677
  readdir_directory_scan: count=54 total_ns=26281659 avg_ns=486697 max_ns=2996625
  readdir_scan.name_child_path_materialization: count=54 total_ns=1613395 avg_ns=29877 max_ns=127919
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=89922 avg_ns=1665 max_ns=6417
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=20867451 avg_ns=386434 max_ns=2802570
  readdir_candidate_selection: count=54 total_ns=771731 avg_ns=14291 max_ns=82900
  readdir_page_commit: count=54 total_ns=5067851 avg_ns=93849 max_ns=586379
  readdirplus_directory_scan: count=34 total_ns=40612359 avg_ns=1194481 max_ns=4612809
  readdirplus_scan.name_child_path_materialization: count=34 total_ns=2643692 avg_ns=77755 max_ns=194975
  readdirplus_scan.returned_attr_hydration: count=4816 total_ns=5276424 avg_ns=1095 max_ns=10116
  readdirplus_scan.returned_policy_recheck: count=4816 total_ns=815137 avg_ns=169 max_ns=5626
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=12525782 avg_ns=8070 max_ns=17323
  readdirplus_scan.scan_fallback_attr: count=34 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=34 total_ns=911932 avg_ns=26821 max_ns=103956
  readdirplus_attr_generation_scan: count=4850 total_ns=5276424 avg_ns=1087 max_ns=10116
  readdirplus_attr_generation_entries: count=4816
  readdirplus_symlink_visibility: count=34 total_ns=32927176 avg_ns=968446 max_ns=4356554
  readdirplus_candidate_selection: count=34 total_ns=1474686 avg_ns=43373 max_ns=102154
  readdirplus_page_commit: count=34 total_ns=3622828 avg_ns=106553 max_ns=338170
  invalidations: count=1832 invalidated_entries=916 evicted_entries=0 scanned_entries=1024720
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
