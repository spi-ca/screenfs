# ScreenFS benchmark result

- timestamp: `2026-06-24T23:17:42.140773+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --perf-counters --workload-set all --iterations 3 --warmups 1 --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 --small-files 200 --dir-entries 500 --hidden-misses 200 --matcher-extra-rules 20 --matcher-misses 200 --symlink-parent-mutations 200 --output-json docs/artifacts/current-perf-counter-benchmark-result.json --output-md docs/artifacts/current-perf-counter-benchmark-result.md --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `60caec31b7323997abcd856b0bb395f2b97c07f1`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `bfa5b36cab85360f55cb559ff8549e6f7e6901a83e533ab372fbbbf1888e2821`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `60caec31b7323997abcd856b0bb395f2b97c07f1`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher20`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
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
| seq_read | 0.001264 | 0.004546 | 3.595 | 0.004606 | 0.004613 | 0.004619 |
| seq_write | 0.000652 | 0.003363 | 5.157 | 0.003645 | 0.003680 | 0.003708 |
| small_read | 0.000045 | 0.001705 | 37.910 | 0.001810 | 0.001823 | 0.001833 |
| small_write | 0.000064 | 0.004857 | 76.344 | 0.004874 | 0.004876 | 0.004878 |
| write_fsync_close | 0.000100 | 0.005724 | 57.146 | 0.006455 | 0.006546 | 0.006619 |
| small_stat_open_read | 0.001097 | 0.046909 | 42.747 | 0.047782 | 0.047891 | 0.047978 |
| readdir_lstat | 0.000549 | 0.038132 | 69.465 | 0.038892 | 0.038987 | 0.039063 |
| symlink_open_read | 0.000010 | 0.000266 | 27.829 | 0.000289 | 0.000291 | 0.000294 |
| rand_read_4k | 0.013199 | 0.349044 | 26.444 | 0.355287 | 0.356067 | 0.356691 |
| rand_write_4k | 0.011456 | 0.840451 | 73.366 | 0.858102 | 0.860309 | 0.862074 |
| sync_write_4k | 0.000029 | 0.001105 | 37.902 | 0.001196 | 0.001207 | 0.001216 |
| small_open_read_close | 0.027989 | 0.678680 | 24.248 | 0.685620 | 0.686487 | 0.687181 |
| concurrent_rand_read_write_4k | 0.218817 | 2.254701 | 10.304 | 2.284613 | 2.288352 | 2.291343 |
| metadata_lookup | 0.003023 | 0.022217 | 7.348 | 0.022442 | 0.022470 | 0.022493 |
| metadata_getattr | 0.003171 | 0.028046 | 8.845 | 0.029117 | 0.029250 | 0.029357 |
| metadata_open | 0.002915 | 0.030801 | 10.566 | 0.031676 | 0.031785 | 0.031873 |
| metadata_readlink | 0.000295 | 0.033416 | 113.276 | 0.034520 | 0.034658 | 0.034769 |
| metadata_access | 0.002802 | 0.029564 | 10.550 | 0.030342 | 0.030440 | 0.030517 |
| metadata_statfs | 0.000327 | 0.002169 | 6.644 | 0.002303 | 0.002320 | 0.002333 |
| metadata_opendir | 0.001104 | 0.023682 | 21.443 | 0.024005 | 0.024045 | 0.024077 |
| sync_flush_only | 0.000069 | 0.002671 | 38.659 | 0.002948 | 0.002983 | 0.003011 |
| sync_fsync_only | 0.000033 | 0.001140 | 34.024 | 0.001220 | 0.001230 | 0.001238 |
| sync_release_flush | 0.000049 | 0.002645 | 53.847 | 0.002708 | 0.002716 | 0.002722 |
| read_only_open_close | 0.025080 | 0.321224 | 12.808 | 0.340864 | 0.343320 | 0.345284 |
| read_only_open_read_close | 0.027041 | 0.581517 | 21.505 | 0.596514 | 0.598389 | 0.599889 |
| write_open_write_close | 0.000050 | 0.003528 | 70.536 | 0.003532 | 0.003532 | 0.003533 |
| write_open_fsync_close | 0.000053 | 0.003600 | 68.535 | 0.003645 | 0.003651 | 0.003656 |
| readdir_basic | 0.000103 | 0.004636 | 44.919 | 0.004713 | 0.004723 | 0.004730 |
| readdirplus_basic | 0.000517 | 0.036113 | 69.827 | 0.036301 | 0.036325 | 0.036343 |
| readdir_symlink_visibility | 0.000215 | 0.018282 | 85.104 | 0.018403 | 0.018418 | 0.018430 |
| readdirplus_symlink_visibility | 0.000684 | 0.060197 | 88.024 | 0.060696 | 0.060759 | 0.060809 |
| matcher_descendant_readdir | 0.000019 | 0.000491 | 26.183 | 0.000512 | 0.000515 | 0.000517 |
| matcher_descendant_readdirplus | 0.000035 | 0.002215 | 62.400 | 0.002227 | 0.002229 | 0.002230 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| hidden_stat_miss | 0.006305 | 0.006320 | 0.006321 | 0.006323 |
| matcher_hidden_stat_miss | 0.011216 | 0.011510 | 0.011547 | 0.011576 |
| symlink_parent_mkdir_rmdir | 0.176038 | 0.183563 | 0.184503 | 0.185256 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2049 total_ns=30121602 avg_ns=14700 max_ns=80032
  fuse_op.create: count=116 total_ns=10880974 avg_ns=93801 max_ns=188409
  fuse_op.flush: count=356 total_ns=2858955 avg_ns=8030 max_ns=159645
  fuse_op.fsync: count=196 total_ns=920322 avg_ns=4695 max_ns=54976
  fuse_op.getattr: count=390617 total_ns=5863241218 avg_ns=15010 max_ns=2052872
  fuse_op.getxattr: count=328660 total_ns=9287230736 avg_ns=28257 max_ns=2021686
  fuse_op.lookup: count=232661 total_ns=2517249724 avg_ns=10819 max_ns=17451247
  fuse_op.mkdir: count=800 total_ns=71023771 avg_ns=88779 max_ns=236694
  fuse_op.open: count=52272 total_ns=1010053546 avg_ns=19323 max_ns=280765
  fuse_op.opendir: count=2080 total_ns=21071803 avg_ns=10130 max_ns=38928
  fuse_op.read: count=50625 total_ns=1110729704 avg_ns=21940 max_ns=421946
  fuse_op.readdir: count=54 total_ns=67822061 avg_ns=1255964 max_ns=5983289
  fuse_op.readdirplus: count=34 total_ns=146936418 avg_ns=4321659 max_ns=12822953
  fuse_op.readlink: count=4456 total_ns=117016839 avg_ns=26260 max_ns=79274
  fuse_op.release: count=52388 total_ns=29869408 avg_ns=570 max_ns=98008
  fuse_op.releasedir: count=2080 total_ns=1738494 avg_ns=835 max_ns=167550
  fuse_op.rmdir: count=800 total_ns=154720152 avg_ns=193400 max_ns=337559
  fuse_op.setattr: count=20 total_ns=1084429 avg_ns=54221 max_ns=77824
  fuse_op.statfs: count=2050 total_ns=385500 avg_ns=188 max_ns=9121
  fuse_op.unlink: count=116 total_ns=14940508 avg_ns=128797 max_ns=583088
  fuse_op.write: count=328400 total_ns=15551015390 avg_ns=47353 max_ns=2074310
  policy_decision: count=3542174 total_ns=4795826189 avg_ns=1353 max_ns=1221161
  matcher_candidates: count=1545476
  matcher_candidates_by_source.hidden.path: count=2216
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=1543260
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1545476
  matcher_candidate_order.descendant: count=2879242 total_ns=1365695899 avg_ns=474 max_ns=685029
  matcher_candidate_order.path: count=9963590 total_ns=5127188417 avg_ns=514 max_ns=1237152
  matcher_candidate_order_by_source.hidden.path: count=2879242 total_ns=1982637496 avg_ns=688 max_ns=1237152
  matcher_candidate_order_by_source.internal_hidden.path: count=2879242 total_ns=574605903 avg_ns=199 max_ns=296005
  matcher_candidate_order_by_source.readonly.path: count=662932 total_ns=522973041 avg_ns=788 max_ns=319666
  matcher_candidate_order_by_source.visible.descendant: count=2879242 total_ns=1365695899 avg_ns=474 max_ns=685029
  matcher_candidate_order_by_source.visible.path: count=2879242 total_ns=1909071085 avg_ns=663 max_ns=905029
  matcher_candidate_order_by_source.writable.path: count=662932 total_ns=137900892 avg_ns=208 max_ns=254608
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=192434576
  matcher_candidate_order_seen_slots.descendant: count=57584840
  matcher_candidate_order_seen_slots.path: count=134849736
  matcher_candidate_order_ancestor_steps: count=50460300
  matcher_candidate_order_ancestor_steps.descendant: count=11288643
  matcher_candidate_order_ancestor_steps.path: count=39171657
  state_read_lock_wait: count=1394312 total_ns=85953410 avg_ns=61 max_ns=265068
  state_read_lock_hold: count=1394312 total_ns=118347028 avg_ns=84 max_ns=261731
  state_write_lock_wait: count=337263 total_ns=7697601 avg_ns=22 max_ns=6648
  state_write_lock_hold: count=337263 total_ns=260815000 avg_ns=773 max_ns=679077
  open_confined_openat2: count=1792101 total_ns=1666693827 avg_ns=930 max_ns=17411450
  open_like.pre_open_guard.access: count=2049 total_ns=18968919 avg_ns=9257 max_ns=67804
  open_like.pre_open_guard.open: count=52272 total_ns=642282984 avg_ns=12287 max_ns=273200
  open_like.pre_open_guard.opendir: count=2080 total_ns=13242214 avg_ns=6366 max_ns=24793
  open_like.post_open_revalidation.access: count=2049 total_ns=9229449 avg_ns=4504 max_ns=11396
  open_like.post_open_revalidation.open: count=52272 total_ns=297766724 avg_ns=5696 max_ns=185002
  open_like.post_open_revalidation.opendir: count=2080 total_ns=5914803 avg_ns=2843 max_ns=31116
  stat_child_no_follow: count=1405100 total_ns=2303317198 avg_ns=1639 max_ns=17415293
  stat_child_no_follow.attr_conversion: count=1396866 total_ns=21712442 avg_ns=15 max_ns=34728
  stat_child_no_follow.directory_revalidation: count=4456 total_ns=35796985 avg_ns=8033 max_ns=36182
  stat_child_no_follow.host_fstat: count=1392410 total_ns=275279952 avg_ns=197 max_ns=109761
  stat_child_no_follow.host_fstatat: count=4456 total_ns=1371573 avg_ns=307 max_ns=47361
  stat_child_no_follow.parent_open: count=4456 total_ns=3489565 avg_ns=783 max_ns=10728
  stat_child_no_follow_context.path_guard_or_metadata: count=1400644 total_ns=2261623521 avg_ns=1614 max_ns=17415293
  stat_child_no_follow_context.readlink_pre_open: count=4456 total_ns=41693677 avg_ns=9356 max_ns=55689
  source_root_path: count=1731812 total_ns=3499259466 avg_ns=2020 max_ns=1823376
  resolved_virtual_path: count=2174444 total_ns=7186467902 avg_ns=3304 max_ns=2011277
  resolved_virtual_path_from_path: count=1403846 total_ns=6173508490 avg_ns=4397 max_ns=1989286
  resolved_virtual_path_from_path_component_walk: count=1403846 total_ns=5481190249 avg_ns=3904 max_ns=1987315
  resolved_virtual_path_from_path_canonicalize: count=3978076 total_ns=4512921684 avg_ns=1134 max_ns=1986144
  resolved_virtual_path_from_path_source_root_confinement: count=3978076 total_ns=519788149 avg_ns=130 max_ns=283663
  resolved_virtual_path_from_path_virtual_conversion: count=1403846 total_ns=606781183 avg_ns=432 max_ns=320779
  resolved_virtual_path_from_open_fd: count=770598 total_ns=1012959412 avg_ns=1314 max_ns=2011277
  read_handle_snapshot: count=50625 total_ns=14921101 avg_ns=294 max_ns=15012
  read_guard_path: count=50625 total_ns=1025320598 avg_ns=20253 max_ns=421230
  read_io: count=50625 total_ns=58926610 avg_ns=1163 max_ns=376299
  write_handle_snapshot: count=328400 total_ns=110355522 avg_ns=336 max_ns=265349
  write_guard_mutation: count=328400 total_ns=14948534517 avg_ns=45519 max_ns=2071019
  write_io: count=328400 total_ns=384218039 avg_ns=1169 max_ns=689699
  file_sync.flush: count=356 total_ns=2741799 avg_ns=7701 max_ns=158422
  file_sync.fsync: count=196 total_ns=867122 avg_ns=4424 max_ns=54384
  read_size_bucket.0_4k: count=48418 total_ns=49423263 avg_ns=1020 max_ns=376299
  read_size_bucket.4k_64k: count=2067 total_ns=5432666 avg_ns=2628 max_ns=100729
  read_size_bucket.64k_1m: count=140 total_ns=4070681 avg_ns=29076 max_ns=55830
  write_size_bucket.0_4k: count=328384 total_ns=380210939 avg_ns=1157 max_ns=689699
  write_size_bucket.64k_1m: count=16 total_ns=4007100 avg_ns=250443 max_ns=315990
  readdir_directory_scan: count=54 total_ns=59744647 avg_ns=1106382 max_ns=5584212
  readdir_scan.name_child_path_materialization: count=54 total_ns=1765587 avg_ns=32696 max_ns=102863
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=22372008 avg_ns=414296 max_ns=1399246
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=29249023 avg_ns=541648 max_ns=3859934
  readdir_candidate_selection: count=54 total_ns=1050509 avg_ns=19453 max_ns=87996
  readdir_page_commit: count=54 total_ns=6167628 avg_ns=114215 max_ns=679217
  readdirplus_directory_scan: count=34 total_ns=96389583 avg_ns=2834987 max_ns=8892449
  readdirplus_scan.name_child_path_materialization: count=34 total_ns=2869277 avg_ns=84390 max_ns=140076
  readdirplus_scan.returned_attr_hydration: count=4816 total_ns=7626046 avg_ns=1583 max_ns=32335
  readdirplus_scan.returned_policy_recheck: count=4816 total_ns=18574262 avg_ns=3856 max_ns=24847
  readdirplus_scan.returned_symlink_visibility: count=1552 total_ns=17554864 avg_ns=11311 max_ns=31406
  readdirplus_scan.scan_fallback_attr: count=34 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=34 total_ns=40695419 avg_ns=1196924 max_ns=2282485
  readdirplus_attr_generation_scan: count=4850 total_ns=7626046 avg_ns=1572 max_ns=32335
  readdirplus_attr_generation_entries: count=4816
  readdirplus_symlink_visibility: count=34 total_ns=46973088 avg_ns=1381561 max_ns=6144990
  readdirplus_candidate_selection: count=34 total_ns=1982216 avg_ns=58300 max_ns=138169
  readdirplus_page_commit: count=34 total_ns=4458285 avg_ns=131126 max_ns=489455
  invalidations: count=1832 invalidated_entries=916 evicted_entries=0 scanned_entries=1024720
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
