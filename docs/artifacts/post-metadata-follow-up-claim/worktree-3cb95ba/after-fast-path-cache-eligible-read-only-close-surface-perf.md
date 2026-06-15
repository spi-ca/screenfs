# ScreenFS benchmark result

- timestamp: `2026-06-15T22:45:03.059053+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-postmeta-target-perf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set read-only-close-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface-perf.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-postmeta-target-perf/release/screenfs`
- screenfs_bin_sha256: `93aa3942c0f6ba6dcf017f3be6197edbcebaff2683ec0da4eef1cf61341d1a6e`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-only-close-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| read_only_open_close | 0.028391 | 0.501824 | 17.675 | 0.511948 | 0.514734 | 0.516963 |
| read_only_open_read_close | 0.031972 | 0.668958 | 20.923 | 0.676724 | 0.681757 | 0.685784 |
| write_open_write_close | 0.000614 | 0.028763 | 46.883 | 0.031406 | 0.031793 | 0.032103 |
| write_open_fsync_close | 0.000660 | 0.030791 | 46.677 | 0.031071 | 0.031098 | 0.031120 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61907 avg_ns=61907 max_ns=61907
  fuse_op.create: count=26 total_ns=3205197 avg_ns=123276 max_ns=154082
  fuse_op.flush: count=3328 total_ns=26077825 avg_ns=7835 max_ns=96543
  fuse_op.fsync: count=1664 total_ns=12181184 avg_ns=7320 max_ns=17553
  fuse_op.getattr: count=53249 total_ns=971813229 avg_ns=18250 max_ns=231799
  fuse_op.getxattr: count=6630 total_ns=182695717 avg_ns=27555 max_ns=51197
  fuse_op.lookup: count=329555 total_ns=5145855462 avg_ns=15614 max_ns=232230
  fuse_op.open: count=109798 total_ns=2866541886 avg_ns=26107 max_ns=345735
  fuse_op.read: count=53248 total_ns=92800568 avg_ns=1742 max_ns=200369
  fuse_op.release: count=109824 total_ns=115055239 avg_ns=1047 max_ns=8395
  fuse_op.statfs: count=2 total_ns=2956 avg_ns=1478 max_ns=1593
  fuse_op.unlink: count=26 total_ns=9324762 avg_ns=358644 max_ns=462786
  fuse_op.write: count=3328 total_ns=8465405 avg_ns=2543 max_ns=10500
  policy_decision: count=1128934 total_ns=411323409 avg_ns=364 max_ns=81705
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1122148 total_ns=146017775 avg_ns=130 max_ns=10950
  matcher_candidate_order.path: count=3380016 total_ns=459657377 avg_ns=135 max_ns=30278
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=13373168
  matcher_candidate_order_ancestor_steps.descendant: count=3329772
  matcher_candidate_order_ancestor_steps.path: count=10043396
  state_read_lock_wait: count=560853 total_ns=15361470 avg_ns=27 max_ns=9117
  state_read_lock_hold: count=560853 total_ns=69878935 avg_ns=124 max_ns=7675
  state_write_lock_wait: count=549201 total_ns=13837295 avg_ns=25 max_ns=14528
  state_write_lock_hold: count=549201 total_ns=289321489 avg_ns=526 max_ns=364750
  open_confined_openat2: count=615922 total_ns=680800234 avg_ns=1105 max_ns=22773
  stat_child_no_follow: count=499441 total_ns=6886596833 avg_ns=13788 max_ns=229836
  source_root_path: count=615947 total_ns=2226038917 avg_ns=3614 max_ns=322440
  resolved_virtual_path: count=1115439 total_ns=3219692664 avg_ns=2886 max_ns=215407
  resolved_virtual_path_from_path: count=499492 total_ns=2038300327 avg_ns=4080 max_ns=208835
  resolved_virtual_path_from_path_component_walk: count=499492 total_ns=1848054623 avg_ns=3699 max_ns=208344
  resolved_virtual_path_from_path_canonicalize: count=669375 total_ns=1675282202 avg_ns=2502 max_ns=207894
  resolved_virtual_path_from_path_source_root_confinement: count=669375 total_ns=90103147 avg_ns=134 max_ns=19908
  resolved_virtual_path_from_path_virtual_conversion: count=499492 total_ns=154366361 avg_ns=309 max_ns=10029
  resolved_virtual_path_from_open_fd: count=615947 total_ns=1181392337 avg_ns=1918 max_ns=215407
  read_handle_snapshot: count=53248 total_ns=16625360 avg_ns=312 max_ns=7154
  read_guard_path: count=53248 total_ns=2065007 avg_ns=38 max_ns=411
  read_io: count=53248 total_ns=62122232 avg_ns=1166 max_ns=199718
  write_handle_snapshot: count=3328 total_ns=858392 avg_ns=257 max_ns=441
  write_guard_mutation: count=3328 total_ns=102567 avg_ns=30 max_ns=61
  write_io: count=3328 total_ns=6853672 avg_ns=2059 max_ns=10039
  file_sync.flush: count=3328 total_ns=24961818 avg_ns=7500 max_ns=95991
  file_sync.fsync: count=1664 total_ns=11671599 avg_ns=7014 max_ns=17253
  read_size_bucket.0_4k: count=53248 total_ns=62122232 avg_ns=1166 max_ns=199718
  write_size_bucket.0_4k: count=3328 total_ns=6853672 avg_ns=2059 max_ns=10039
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_entries: count=0
  readdirplus_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  invalidations: count=52 invalidated_entries=26 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
