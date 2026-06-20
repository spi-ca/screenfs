# ScreenFS benchmark result

- timestamp: `2026-06-20T17:21:05.524614+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.json --output-md docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.md --output-svg docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+19 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+19 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.013949 | 0.037499 | 2.688 | 0.038685 | 0.038800 | 0.038893 |
| seq_write | 0.011347 | 0.035348 | 3.115 | 0.035762 | 0.036570 | 0.037217 |
| small_read | 0.000751 | 0.019275 | 25.678 | 0.019322 | 0.019361 | 0.019391 |
| small_write | 0.001011 | 0.025340 | 25.068 | 0.031942 | 0.032616 | 0.033156 |
| rand_read_4k | 0.021954 | 0.362439 | 16.509 | 0.385754 | 0.387852 | 0.389531 |
| rand_write_4k | 0.020759 | 0.506198 | 24.385 | 0.517624 | 0.517832 | 0.518000 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=51246 avg_ns=51246 max_ns=51246
  fuse_op.create: count=39 total_ns=3514297 avg_ns=90110 max_ns=123112
  fuse_op.flush: count=39 total_ns=1782128 avg_ns=45695 max_ns=201157
  fuse_op.getattr: count=227202 total_ns=2274026581 avg_ns=10008 max_ns=656960
  fuse_op.getxattr: count=227149 total_ns=3707191166 avg_ns=16320 max_ns=641526
  fuse_op.lookup: count=434 total_ns=5667775 avg_ns=13059 max_ns=70218
  fuse_op.open: count=39 total_ns=722295 avg_ns=18520 max_ns=52549
  fuse_op.read: count=148343 total_ns=421643021 avg_ns=2842 max_ns=79825
  fuse_op.release: count=78 total_ns=161544 avg_ns=2071 max_ns=4192
  fuse_op.setattr: count=13 total_ns=472504 avg_ns=36346 max_ns=40651
  fuse_op.statfs: count=2 total_ns=4730 avg_ns=2365 max_ns=3161
  fuse_op.unlink: count=39 total_ns=62157981 avg_ns=1593794 max_ns=2710771
  fuse_op.write: count=227136 total_ns=608258862 avg_ns=2677 max_ns=740661
  policy_decision: count=1138203 total_ns=455527650 avg_ns=400 max_ns=320304
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1137904 total_ns=162563739 avg_ns=142 max_ns=47574
  matcher_candidate_order.path: count=3414310 total_ns=497752479 avg_ns=145 max_ns=92112
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=16382824
  matcher_candidate_order_ancestor_steps.descendant: count=4095186
  matcher_candidate_order_ancestor_steps.path: count=12287638
  state_read_lock_wait: count=830434 total_ns=17700838 avg_ns=21 max_ns=6484
  state_read_lock_hold: count=830434 total_ns=57974636 avg_ns=69 max_ns=27053
  state_write_lock_wait: count=588 total_ns=19737 avg_ns=33 max_ns=370
  state_write_lock_hold: count=588 total_ns=624744 avg_ns=1062 max_ns=12032
  open_confined_openat2: count=682443 total_ns=445465640 avg_ns=652 max_ns=91186
  open_like.pre_open_guard.access: count=1 total_ns=16979 avg_ns=16979 max_ns=16979
  open_like.pre_open_guard.open: count=39 total_ns=455428 avg_ns=11677 max_ns=41940
  open_like.post_open_revalidation.access: count=1 total_ns=29584 avg_ns=29584 max_ns=29584
  open_like.post_open_revalidation.open: count=39 total_ns=125860 avg_ns=3227 max_ns=3872
  stat_child_no_follow: count=455163 total_ns=3906172457 avg_ns=8581 max_ns=654971
  source_root_path: count=682364 total_ns=1098626917 avg_ns=1610 max_ns=323127
  resolved_virtual_path: count=1137721 total_ns=1936552085 avg_ns=1702 max_ns=85963
  resolved_virtual_path_from_path: count=455240 total_ns=1229533601 avg_ns=2700 max_ns=85963
  resolved_virtual_path_from_path_component_walk: count=455240 total_ns=1063555919 avg_ns=2336 max_ns=85637
  resolved_virtual_path_from_path_canonicalize: count=909966 total_ns=849418526 avg_ns=933 max_ns=85185
  resolved_virtual_path_from_path_source_root_confinement: count=909966 total_ns=121146138 avg_ns=133 max_ns=33856
  resolved_virtual_path_from_path_virtual_conversion: count=455240 total_ns=139687435 avg_ns=306 max_ns=46468
  resolved_virtual_path_from_open_fd: count=682481 total_ns=707018484 avg_ns=1035 max_ns=67455
  read_handle_snapshot: count=148343 total_ns=28554076 avg_ns=192 max_ns=76196
  read_guard_path: count=148343 total_ns=2401224 avg_ns=16 max_ns=1635
  read_io: count=148343 total_ns=366487027 avg_ns=2470 max_ns=74276
  write_handle_snapshot: count=227136 total_ns=46839947 avg_ns=206 max_ns=19261
  write_guard_mutation: count=227136 total_ns=3852272 avg_ns=16 max_ns=1615
  write_io: count=227136 total_ns=519005076 avg_ns=2284 max_ns=739875
  file_sync.flush: count=39 total_ns=1735345 avg_ns=44496 max_ns=199117
  read_size_bucket.0_4k: count=126893 total_ns=149149057 avg_ns=1175 max_ns=62268
  read_size_bucket.4k_64k: count=14365 total_ns=36496380 avg_ns=2540 max_ns=29842
  read_size_bucket.64k_1m: count=7085 total_ns=180841590 avg_ns=25524 max_ns=74276
  write_size_bucket.0_4k: count=226304 total_ns=378348370 avg_ns=1671 max_ns=739875
  write_size_bucket.64k_1m: count=832 total_ns=140656706 avg_ns=169058 max_ns=358776
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
  invalidations: count=78 invalidated_entries=39 evicted_entries=0 scanned_entries=234
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
