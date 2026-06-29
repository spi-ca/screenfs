# ScreenFS benchmark result

- timestamp: `2026-06-29T21:12:23.045257+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `db3c89f155745bcca401c709857b205c78459876`
- git_dirty_status: `M .pi/skills/run-screenfs-benchmarks/SKILL.md;  M docs/architecture.md;  M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-goal-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md; ... (+28 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `db3c89f155745bcca401c709857b205c78459876`
- screenfs_source_git_dirty_status: `M .pi/skills/run-screenfs-benchmarks/SKILL.md;  M docs/architecture.md;  M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-goal-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md; ... (+28 more)`
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
| seq_read | 0.001125 | 0.003292 | 2.926 | 0.003324 | 0.003328 | 0.003331 |
| seq_write | 0.000322 | 0.002288 | 7.102 | 0.002535 | 0.002566 | 0.002590 |
| small_read | 0.000026 | 0.001351 | 52.673 | 0.001610 | 0.001643 | 0.001669 |
| small_write | 0.000039 | 0.004144 | 105.012 | 0.004867 | 0.004958 | 0.005030 |
| write_fsync_close | 0.000054 | 0.005928 | 109.719 | 0.006106 | 0.006129 | 0.006147 |
| small_stat_open_read | 0.000701 | 0.034939 | 49.837 | 0.035515 | 0.035587 | 0.035645 |
| readdir_lstat | 0.000315 | 0.025564 | 81.173 | 0.026050 | 0.026111 | 0.026160 |
| symlink_open_read | 0.000005 | 0.000215 | 42.515 | 0.000220 | 0.000221 | 0.000221 |
| rand_read_4k | 0.006987 | 0.267732 | 38.321 | 0.268095 | 0.268140 | 0.268176 |
| rand_write_4k | 0.006513 | 0.910929 | 139.873 | 1.060461 | 1.079152 | 1.094106 |
| sync_write_4k | 0.000029 | 0.001319 | 45.377 | 0.001407 | 0.001418 | 0.001427 |
| small_open_read_close | 0.015581 | 0.467255 | 29.990 | 0.495008 | 0.498477 | 0.501253 |
| concurrent_rand_read_write_4k | 0.234782 | 2.254841 | 9.604 | 2.258468 | 2.258921 | 2.259284 |
| metadata_lookup | 0.001662 | 0.029042 | 17.473 | 0.029341 | 0.029378 | 0.029408 |
| metadata_getattr | 0.001895 | 0.037284 | 19.679 | 0.037581 | 0.037618 | 0.037648 |
| metadata_open | 0.002716 | 0.046336 | 17.061 | 0.046838 | 0.046901 | 0.046951 |
| metadata_readlink | 0.000267 | 0.045128 | 169.107 | 0.045484 | 0.045529 | 0.045564 |
| metadata_access | 0.001667 | 0.041710 | 25.016 | 0.042264 | 0.042333 | 0.042388 |
| metadata_statfs | 0.000213 | 0.002556 | 11.979 | 0.002578 | 0.002581 | 0.002583 |
| metadata_opendir | 0.000690 | 0.032284 | 46.778 | 0.032732 | 0.032788 | 0.032833 |
| sync_flush_only | 0.000046 | 0.003816 | 82.765 | 0.003939 | 0.003955 | 0.003967 |
| sync_fsync_only | 0.000024 | 0.001656 | 70.157 | 0.001722 | 0.001730 | 0.001736 |
| sync_release_flush | 0.000036 | 0.003475 | 97.878 | 0.004121 | 0.004202 | 0.004267 |
| read_only_open_close | 0.014779 | 0.355244 | 24.037 | 0.364387 | 0.365530 | 0.366445 |
| read_only_open_read_close | 0.015880 | 0.406542 | 25.601 | 0.492743 | 0.503518 | 0.512138 |
| write_open_write_close | 0.000033 | 0.002671 | 81.883 | 0.002918 | 0.002948 | 0.002973 |
| write_open_fsync_close | 0.000033 | 0.002780 | 83.743 | 0.002787 | 0.002788 | 0.002789 |
| readdir_basic | 0.000097 | 0.001247 | 12.822 | 0.001293 | 0.001298 | 0.001303 |
| readdirplus_basic | 0.000388 | 0.024294 | 62.656 | 0.024696 | 0.024747 | 0.024787 |
| readdir_symlink_visibility | 0.000169 | 0.009637 | 56.903 | 0.009700 | 0.009708 | 0.009714 |
| readdirplus_symlink_visibility | 0.000481 | 0.038399 | 79.872 | 0.041219 | 0.041572 | 0.041854 |
| matcher_descendant_readdir | 0.000011 | 0.000429 | 38.664 | 0.000440 | 0.000442 | 0.000443 |
| matcher_descendant_readdirplus | 0.000024 | 0.001985 | 81.721 | 0.002012 | 0.002016 | 0.002018 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.004753 | 0.005068 | 0.005108 | 0.005140 |
| matcher_hidden_stat_miss | 0.007533 | 0.007809 | 0.007844 | 0.007871 |
| symlink_parent_mkdir_rmdir | 0.160793 | 0.170726 | 0.171968 | 0.172961 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=41416980 avg_ns=20213 max_ns=103456
  fuse_op.create: count=116 total_ns=10519024 avg_ns=90681 max_ns=266771
  fuse_op.flush: count=356 total_ns=2886977 avg_ns=8109 max_ns=299292
  fuse_op.fsync: count=196 total_ns=833225 avg_ns=4251 max_ns=17317
  fuse_op.getattr: count=390617 total_ns=5572370572 avg_ns=14265 max_ns=2442559
  fuse_op.getxattr: count=328660 total_ns=9352171092 avg_ns=28455 max_ns=2634348
  fuse_op.lookup: count=232661 total_ns=2385119982 avg_ns=10251 max_ns=14015109
  fuse_op.mkdir: count=800 total_ns=60486751 avg_ns=75608 max_ns=202174
  fuse_op.open: count=52272 total_ns=953305807 avg_ns=18237 max_ns=272576
  fuse_op.opendir: count=2080 total_ns=24483990 avg_ns=11771 max_ns=73905
  fuse_op.read: count=50619 total_ns=997887475 avg_ns=19713 max_ns=673327
  fuse_op.readdir: count=54 total_ns=30743520 avg_ns=569324 max_ns=3442020
  fuse_op.readdirplus: count=34 total_ns=60723488 avg_ns=1785984 max_ns=6541678
  fuse_op.readlink: count=4456 total_ns=125142574 avg_ns=28084 max_ns=94127
  fuse_op.release: count=52388 total_ns=29560550 avg_ns=564 max_ns=10163
  fuse_op.releasedir: count=2080 total_ns=1715464 avg_ns=824 max_ns=97761
  fuse_op.rmdir: count=800 total_ns=133810267 avg_ns=167262 max_ns=296438
  fuse_op.setattr: count=20 total_ns=1145267 avg_ns=57263 max_ns=88256
  fuse_op.statfs: count=2050 total_ns=590123 avg_ns=287 max_ns=4680
  fuse_op.unlink: count=116 total_ns=14070431 avg_ns=121296 max_ns=455782
  fuse_op.write: count=328400 total_ns=15872531827 avg_ns=48332 max_ns=2091176
  policy_decision: count=3518338 total_ns=4958369040 avg_ns=1409 max_ns=1773624
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
  matcher_candidate_order.descendant: count=2855406 total_ns=1407458960 avg_ns=492 max_ns=2019996
  matcher_candidate_order.path: count=9892082 total_ns=5119774509 avg_ns=517 max_ns=2014152
  matcher_candidate_order_by_source.hidden.path: count=2855406 total_ns=1982134177 avg_ns=694 max_ns=449405
  matcher_candidate_order_by_source.internal_hidden.path: count=2855406 total_ns=556859299 avg_ns=195 max_ns=2014152
  matcher_candidate_order_by_source.readonly.path: count=662932 total_ns=535732424 avg_ns=808 max_ns=583993
  matcher_candidate_order_by_source.visible.descendant: count=2855406 total_ns=1407458960 avg_ns=492 max_ns=2019996
  matcher_candidate_order_by_source.visible.path: count=2855406 total_ns=1913584776 avg_ns=670 max_ns=707376
  matcher_candidate_order_by_source.writable.path: count=662932 total_ns=131463833 avg_ns=198 max_ns=309568
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=190956744
  matcher_candidate_order_seen_slots.descendant: count=57108120
  matcher_candidate_order_seen_slots.path: count=133848624
  matcher_candidate_order_ancestor_steps: count=50087500
  matcher_candidate_order_ancestor_steps.descendant: count=11195443
  matcher_candidate_order_ancestor_steps.path: count=38892057
  state_read_lock_wait: count=1394306 total_ns=91093399 avg_ns=65 max_ns=116891
  state_read_lock_hold: count=1394306 total_ns=143150356 avg_ns=102 max_ns=323280
  state_write_lock_wait: count=337263 total_ns=6885331 avg_ns=20 max_ns=7288
  state_write_lock_hold: count=337263 total_ns=236684860 avg_ns=701 max_ns=568587
  open_confined_openat2: count=1792095 total_ns=1568911357 avg_ns=875 max_ns=13957348
  open_like.pre_open_guard.access: count=2049 total_ns=26210839 avg_ns=12792 max_ns=81423
  open_like.pre_open_guard.open: count=52272 total_ns=602682977 avg_ns=11529 max_ns=266483
  open_like.pre_open_guard.opendir: count=2080 total_ns=18861343 avg_ns=9067 max_ns=43267
  open_like.post_open_revalidation.access: count=2049 total_ns=12378561 avg_ns=6041 max_ns=13074
  open_like.post_open_revalidation.open: count=52272 total_ns=287691945 avg_ns=5503 max_ns=99371
  open_like.post_open_revalidation.opendir: count=2080 total_ns=2827488 avg_ns=1359 max_ns=6815
  stat_child_no_follow: count=1405094 total_ns=2207317771 avg_ns=1570 max_ns=13964055
  stat_child_no_follow.attr_conversion: count=1396860 total_ns=22306828 avg_ns=15 max_ns=41543
  stat_child_no_follow.directory_revalidation: count=4456 total_ns=38460425 avg_ns=8631 max_ns=60333
  stat_child_no_follow.host_fstat: count=1392404 total_ns=265572643 avg_ns=190 max_ns=560296
  stat_child_no_follow.host_fstatat: count=4456 total_ns=1365635 avg_ns=306 max_ns=1777
  stat_child_no_follow.parent_open: count=4456 total_ns=3837440 avg_ns=861 max_ns=9977
  stat_child_no_follow_context.path_guard_or_metadata: count=1400638 total_ns=2162653172 avg_ns=1544 max_ns=13964055
  stat_child_no_follow_context.readlink_pre_open: count=4456 total_ns=44664599 avg_ns=10023 max_ns=61359
  source_root_path: count=1731806 total_ns=3308859924 avg_ns=1910 max_ns=1813453
  resolved_virtual_path: count=2174432 total_ns=6915561342 avg_ns=3180 max_ns=2433491
  resolved_virtual_path_from_path: count=1403840 total_ns=5915626682 avg_ns=4213 max_ns=2433491
  resolved_virtual_path_from_path_component_walk: count=1403840 total_ns=5242837915 avg_ns=3734 max_ns=2430151
  resolved_virtual_path_from_path_canonicalize: count=3978058 total_ns=4346640160 avg_ns=1092 max_ns=2428083
  resolved_virtual_path_from_path_source_root_confinement: count=3978058 total_ns=473908122 avg_ns=119 max_ns=371743
  resolved_virtual_path_from_path_virtual_conversion: count=1403840 total_ns=586951958 avg_ns=418 max_ns=291048
  resolved_virtual_path_from_open_fd: count=770592 total_ns=999934660 avg_ns=1297 max_ns=1215036
  read_handle_snapshot: count=50619 total_ns=16173990 avg_ns=319 max_ns=14938
  read_guard_path: count=50619 total_ns=930053808 avg_ns=18373 max_ns=665921
  read_io: count=50619 total_ns=39741892 avg_ns=785 max_ns=90448
  write_handle_snapshot: count=328400 total_ns=124742504 avg_ns=379 max_ns=93597
  write_guard_mutation: count=328400 total_ns=15265608660 avg_ns=46484 max_ns=2088073
  write_io: count=328400 total_ns=377565477 avg_ns=1149 max_ns=1722960
  file_sync.flush: count=356 total_ns=2770335 avg_ns=7781 max_ns=296669
  file_sync.fsync: count=196 total_ns=777641 avg_ns=3967 max_ns=16871
  read_size_bucket.0_4k: count=48408 total_ns=32909079 avg_ns=679 max_ns=90448
  read_size_bucket.4k_64k: count=2071 total_ns=4525842 avg_ns=2185 max_ns=50277
  read_size_bucket.64k_1m: count=140 total_ns=2306971 avg_ns=16478 max_ns=63054
  write_size_bucket.0_4k: count=328384 total_ns=374977010 avg_ns=1141 max_ns=1722960
  write_size_bucket.64k_1m: count=16 total_ns=2588467 avg_ns=161779 max_ns=205998
  readdir_directory_scan: count=54 total_ns=24732526 avg_ns=458009 max_ns=2832100
  readdir_scan.name_child_path_materialization: count=54 total_ns=1496702 avg_ns=27716 max_ns=91509
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=79840 avg_ns=1478 max_ns=5048
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=20022858 avg_ns=370793 max_ns=2607173
  readdir_candidate_selection: count=54 total_ns=576115 avg_ns=10668 max_ns=58825
  readdir_page_commit: count=54 total_ns=4717057 avg_ns=87352 max_ns=568690
  readdirplus_directory_scan: count=34 total_ns=38675225 avg_ns=1137506 max_ns=4541469
  readdirplus_scan.name_child_path_materialization: count=34 total_ns=2291208 avg_ns=67388 max_ns=120074
  readdirplus_scan.returned_attr_hydration: count=4816 total_ns=4079695 avg_ns=847 max_ns=5986
  readdirplus_scan.returned_policy_recheck: count=4816 total_ns=847566 avg_ns=175 max_ns=5969
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=12301861 avg_ns=7926 max_ns=12727
  readdirplus_scan.scan_fallback_attr: count=34 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=34 total_ns=941040 avg_ns=27677 max_ns=106938
  readdirplus_attr_generation_scan: count=4850 total_ns=4079695 avg_ns=841 max_ns=5986
  readdirplus_attr_generation_entries: count=4816
  readdirplus_symlink_visibility: count=34 total_ns=32201625 avg_ns=947106 max_ns=4268150
  readdirplus_candidate_selection: count=34 total_ns=1181548 avg_ns=34751 max_ns=75613
  readdirplus_page_commit: count=34 total_ns=3253980 avg_ns=95705 max_ns=315860
  invalidations: count=1832 invalidated_entries=916 evicted_entries=0 scanned_entries=1024720
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
