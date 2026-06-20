# ScreenFS benchmark result

- timestamp: `2026-06-20T17:21:33.232537+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.json --output-md docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.md --output-svg docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.012330 | 0.048379 | 3.924 | 0.049748 | 0.050330 | 0.050796 |
| seq_write | 0.011281 | 0.039898 | 3.537 | 0.040523 | 0.041256 | 0.041843 |
| small_read | 0.000725 | 0.025804 | 35.597 | 0.026740 | 0.026853 | 0.026944 |
| small_write | 0.000965 | 0.070246 | 72.764 | 0.070960 | 0.071440 | 0.071825 |
| rand_read_4k | 0.021696 | 0.641776 | 29.580 | 0.680388 | 0.700807 | 0.717142 |
| rand_write_4k | 0.020759 | 1.080760 | 52.063 | 1.125872 | 1.130156 | 1.133583 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=49268 avg_ns=49268 max_ns=49268
  fuse_op.create: count=39 total_ns=4739145 avg_ns=121516 max_ns=145893
  fuse_op.flush: count=39 total_ns=1760807 avg_ns=45148 max_ns=178081
  fuse_op.getattr: count=227202 total_ns=3480284363 avg_ns=15318 max_ns=1213101
  fuse_op.getxattr: count=227149 total_ns=5120710186 avg_ns=22543 max_ns=981272
  fuse_op.lookup: count=434 total_ns=7642851 avg_ns=17610 max_ns=93994
  fuse_op.open: count=39 total_ns=893545 avg_ns=22911 max_ns=26676
  fuse_op.read: count=148343 total_ns=3126742408 avg_ns=21077 max_ns=2685014
  fuse_op.release: count=78 total_ns=192156 avg_ns=2463 max_ns=5476
  fuse_op.setattr: count=13 total_ns=572560 avg_ns=44043 max_ns=51257
  fuse_op.statfs: count=2 total_ns=4773 avg_ns=2386 max_ns=3845
  fuse_op.unlink: count=39 total_ns=72380169 avg_ns=1855901 max_ns=4047346
  fuse_op.write: count=227136 total_ns=6204926674 avg_ns=27318 max_ns=1355916
  policy_decision: count=3173262 total_ns=1585689051 avg_ns=499 max_ns=328639
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=2718691 total_ns=405227744 avg_ns=149 max_ns=345048
  matcher_candidate_order.path: count=9065215 total_ns=1727717217 avg_ns=190 max_ns=902727
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3173262
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3173262
  matcher_candidate_order_ancestor_steps: count=43807364
  matcher_candidate_order_ancestor_steps.descendant: count=10042777
  matcher_candidate_order_ancestor_steps.path: count=33764587
  state_read_lock_wait: count=830434 total_ns=19087377 avg_ns=22 max_ns=14263
  state_read_lock_hold: count=830434 total_ns=65770018 avg_ns=79 max_ns=64766
  state_write_lock_wait: count=588 total_ns=23781 avg_ns=40 max_ns=630
  state_write_lock_hold: count=588 total_ns=700000 avg_ns=1190 max_ns=11636
  open_confined_openat2: count=1058000 total_ns=692823535 avg_ns=654 max_ns=266966
  open_like.pre_open_guard.access: count=1 total_ns=40647 avg_ns=40647 max_ns=40647
  open_like.pre_open_guard.open: count=39 total_ns=616302 avg_ns=15802 max_ns=19691
  open_like.post_open_revalidation.access: count=1 total_ns=4481 avg_ns=4481 max_ns=4481
  open_like.post_open_revalidation.open: count=39 total_ns=129850 avg_ns=3329 max_ns=3738
  stat_child_no_follow: count=830720 total_ns=7390270821 avg_ns=8896 max_ns=2628777
  source_root_path: count=1285070 total_ns=2027825084 avg_ns=1577 max_ns=487239
  resolved_virtual_path: count=3094798 total_ns=7332828441 avg_ns=2369 max_ns=2576593
  resolved_virtual_path_from_path: count=1661281 total_ns=5772852513 avg_ns=3474 max_ns=2576593
  resolved_virtual_path_from_path_component_walk: count=1661281 total_ns=5108550928 avg_ns=3075 max_ns=2573104
  resolved_virtual_path_from_path_canonicalize: count=4151941 total_ns=4176018677 avg_ns=1005 max_ns=1163322
  resolved_virtual_path_from_path_source_root_confinement: count=4151941 total_ns=514178474 avg_ns=123 max_ns=2560692
  resolved_virtual_path_from_path_virtual_conversion: count=1661281 total_ns=569448947 avg_ns=342 max_ns=70749
  resolved_virtual_path_from_open_fd: count=1433517 total_ns=1559975928 avg_ns=1088 max_ns=318099
  read_handle_snapshot: count=148343 total_ns=30422041 avg_ns=205 max_ns=40987
  read_guard_path: count=148343 total_ns=2685817141 avg_ns=18105 max_ns=2670395
  read_io: count=148343 total_ns=382161798 avg_ns=2576 max_ns=211613
  write_handle_snapshot: count=227136 total_ns=48296349 avg_ns=212 max_ns=47757
  write_guard_mutation: count=227136 total_ns=5561870780 avg_ns=24486 max_ns=1343082
  write_io: count=227136 total_ns=552825259 avg_ns=2433 max_ns=772866
  file_sync.flush: count=39 total_ns=1707583 avg_ns=43784 max_ns=175864
  read_size_bucket.0_4k: count=126893 total_ns=169026996 avg_ns=1332 max_ns=211613
  read_size_bucket.4k_64k: count=14365 total_ns=39248050 avg_ns=2732 max_ns=51558
  read_size_bucket.64k_1m: count=7085 total_ns=173886752 avg_ns=24542 max_ns=123224
  write_size_bucket.0_4k: count=226304 total_ns=406735517 avg_ns=1797 max_ns=772866
  write_size_bucket.64k_1m: count=832 total_ns=146089742 avg_ns=175588 max_ns=384916
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
