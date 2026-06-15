# ScreenFS benchmark result

- timestamp: `2026-06-15T22:11:13.189417+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-postmeta-target-perf/release/screenfs --screenfs-source-root /tmp/screenfs-before-postmeta-3cb95ba --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set read-only-close-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface-perf.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-env.json;  M docs/artifacts/managed-fio-attribution-native.json; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-postmeta-target-perf/release/screenfs`
- screenfs_bin_sha256: `9c998fe7a1356552b25afbfdd6be82fd2bf07d2a37ccae50c731d32fa9a777dd`
- screenfs_source_root: `/tmp/screenfs-before-postmeta-3cb95ba`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `True`
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
| read_only_open_close | 0.027685 | 0.570556 | 20.609 | 0.588338 | 0.589228 | 0.589940 |
| read_only_open_read_close | 0.031253 | 0.740094 | 23.681 | 0.757048 | 0.757962 | 0.758694 |
| write_open_write_close | 0.000605 | 0.028358 | 46.881 | 0.030141 | 0.030203 | 0.030252 |
| write_open_fsync_close | 0.000651 | 0.032066 | 49.246 | 0.033261 | 0.033311 | 0.033352 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=75392 avg_ns=75392 max_ns=75392
  fuse_op.create: count=26 total_ns=3179745 avg_ns=122297 max_ns=154172
  fuse_op.flush: count=109824 total_ns=857193108 avg_ns=7805 max_ns=113625
  fuse_op.fsync: count=1664 total_ns=12621289 avg_ns=7584 max_ns=16492
  fuse_op.getattr: count=53249 total_ns=964135780 avg_ns=18106 max_ns=292032
  fuse_op.getxattr: count=6630 total_ns=185989971 avg_ns=28052 max_ns=57619
  fuse_op.lookup: count=329555 total_ns=5114639794 avg_ns=15519 max_ns=386290
  fuse_op.open: count=109798 total_ns=2960881366 avg_ns=26966 max_ns=210318
  fuse_op.read: count=53248 total_ns=92345520 avg_ns=1734 max_ns=16932
  fuse_op.release: count=109824 total_ns=112532551 avg_ns=1024 max_ns=20639
  fuse_op.statfs: count=2 total_ns=2605 avg_ns=1302 max_ns=1322
  fuse_op.unlink: count=26 total_ns=9233082 avg_ns=355118 max_ns=463045
  fuse_op.write: count=3328 total_ns=8338678 avg_ns=2505 max_ns=9197
  policy_decision: count=1128934 total_ns=411086689 avg_ns=364 max_ns=274289
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1122148 total_ns=152640313 avg_ns=136 max_ns=11812
  matcher_candidate_order.path: count=3380016 total_ns=451185361 avg_ns=133 max_ns=177726
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=13373168
  matcher_candidate_order_ancestor_steps.descendant: count=3329772
  matcher_candidate_order_ancestor_steps.path: count=10043396
  state_read_lock_wait: count=667349 total_ns=17544120 avg_ns=26 max_ns=7354
  state_read_lock_hold: count=667349 total_ns=91430378 avg_ns=137 max_ns=11712
  state_write_lock_wait: count=549201 total_ns=14278691 avg_ns=25 max_ns=13796
  state_write_lock_hold: count=549201 total_ns=296825750 avg_ns=540 max_ns=363627
  open_confined_openat2: count=615922 total_ns=702585251 avg_ns=1140 max_ns=21160
  stat_child_no_follow: count=499441 total_ns=6903317928 avg_ns=13822 max_ns=383896
  source_root_path: count=615947 total_ns=2235076639 avg_ns=3628 max_ns=192574
  resolved_virtual_path: count=1115439 total_ns=3195464203 avg_ns=2864 max_ns=375601
  resolved_virtual_path_from_path: count=499492 total_ns=2018476412 avg_ns=4041 max_ns=203835
  resolved_virtual_path_from_path_component_walk: count=499492 total_ns=1825959002 avg_ns=3655 max_ns=203365
  resolved_virtual_path_from_path_canonicalize: count=669375 total_ns=1647236413 avg_ns=2460 max_ns=202763
  resolved_virtual_path_from_path_source_root_confinement: count=669375 total_ns=89463653 avg_ns=133 max_ns=21691
  resolved_virtual_path_from_path_virtual_conversion: count=499492 total_ns=156982590 avg_ns=314 max_ns=18785
  resolved_virtual_path_from_open_fd: count=615947 total_ns=1176987791 avg_ns=1910 max_ns=375601
  read_handle_snapshot: count=53248 total_ns=16565389 avg_ns=311 max_ns=11852
  read_guard_path: count=53248 total_ns=2186176 avg_ns=41 max_ns=631
  read_io: count=53248 total_ns=62676999 avg_ns=1177 max_ns=16351
  write_handle_snapshot: count=3328 total_ns=838521 avg_ns=251 max_ns=571
  write_guard_mutation: count=3328 total_ns=81676 avg_ns=24 max_ns=101
  write_io: count=3328 total_ns=6723872 avg_ns=2020 max_ns=8647
  file_sync.flush: count=109824 total_ns=816848267 avg_ns=7437 max_ns=112683
  file_sync.fsync: count=1664 total_ns=12087175 avg_ns=7263 max_ns=16180
  read_size_bucket.0_4k: count=53248 total_ns=62676999 avg_ns=1177 max_ns=16351
  write_size_bucket.0_4k: count=3328 total_ns=6723872 avg_ns=2020 max_ns=8647
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
