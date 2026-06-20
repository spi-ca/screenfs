# ScreenFS benchmark result

- timestamp: `2026-06-20T07:21:47.613151+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set mutation-invalidation --iterations 1 --warmups 1 --symlink-parent-mutations 20 --small-files 200 --output-json docs/artifacts/current-mutation-invalidation-set-smoke.json --output-md docs/artifacts/current-mutation-invalidation-set-smoke.md --output-svg docs/artifacts/current-mutation-invalidation-set-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir, subtree_rename_cached_unrelated`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.016918 | 0.016918 | 0.016918 | 0.016918 |
| pinned_symlink_parent_mkdir_rmdir | 0.022261 | 0.022261 | 0.022261 | 0.022261 |
| subtree_rename_cached_unrelated | 0.023128 | 0.023128 | 0.023128 | 0.023128 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=71273 avg_ns=71273 max_ns=71273
  fuse_op.getattr: count=581 total_ns=10102947 avg_ns=17388 max_ns=53997
  fuse_op.lookup: count=4280 total_ns=56622507 avg_ns=13229 max_ns=100589
  fuse_op.mkdir: count=80 total_ns=7465989 avg_ns=93324 max_ns=136729
  fuse_op.opendir: count=42 total_ns=759707 avg_ns=18088 max_ns=29958
  fuse_op.readdirplus: count=42 total_ns=1673818 avg_ns=39852 max_ns=74475
  fuse_op.readlink: count=368 total_ns=6719737 avg_ns=18260 max_ns=155229
  fuse_op.releasedir: count=42 total_ns=25852 avg_ns=615 max_ns=1058
  fuse_op.rename: count=4 total_ns=652338 avg_ns=163084 max_ns=172911
  fuse_op.rmdir: count=80 total_ns=5943151 avg_ns=74289 max_ns=168674
  fuse_op.statfs: count=2 total_ns=2872 avg_ns=1436 max_ns=1896
  policy_decision: count=14494 total_ns=6595274 avg_ns=455 max_ns=69247
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=13990 total_ns=1814263 avg_ns=129 max_ns=8065
  matcher_candidate_order.path: count=42978 total_ns=7204617 avg_ns=167 max_ns=58498
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=14494
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=14494
  matcher_candidate_order_ancestor_steps: count=195804
  matcher_candidate_order_ancestor_steps.descendant: count=47871
  matcher_candidate_order_ancestor_steps.path: count=147933
  state_read_lock_wait: count=5482 total_ns=101254 avg_ns=18 max_ns=187
  state_read_lock_hold: count=5482 total_ns=374451 avg_ns=68 max_ns=1295
  state_write_lock_wait: count=4362 total_ns=77690 avg_ns=17 max_ns=154
  state_write_lock_hold: count=4362 total_ns=2011330 avg_ns=461 max_ns=49797
  open_confined_openat2: count=6361 total_ns=3106091 avg_ns=488 max_ns=77316
  stat_child_no_follow: count=6108 total_ns=42784863 avg_ns=7004 max_ns=92779
  source_root_path: count=6524 total_ns=8425017 avg_ns=1291 max_ns=35796
  resolved_virtual_path: count=18077 total_ns=40331413 avg_ns=2231 max_ns=136446
  resolved_virtual_path_from_path: count=11717 total_ns=34508276 avg_ns=2945 max_ns=73648
  resolved_virtual_path_from_path_component_walk: count=11717 total_ns=30271094 avg_ns=2583 max_ns=73109
  resolved_virtual_path_from_path_canonicalize: count=26195 total_ns=24262435 avg_ns=926 max_ns=62854
  resolved_virtual_path_from_path_source_root_confinement: count=26195 total_ns=3445978 avg_ns=131 max_ns=4164
  resolved_virtual_path_from_path_virtual_conversion: count=11717 total_ns=3645699 avg_ns=311 max_ns=3807
  resolved_virtual_path_from_open_fd: count=6360 total_ns=5823137 avg_ns=915 max_ns=136446
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=42 total_ns=417468 avg_ns=9939 max_ns=43494
  readdirplus_attr_generation_scan: count=42 total_ns=53257 avg_ns=1268 max_ns=1799
  readdirplus_attr_generation_entries: count=82
  readdirplus_symlink_visibility: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=42 total_ns=24517 avg_ns=583 max_ns=7266
  readdirplus_page_commit: count=42 total_ns=60323 avg_ns=1436 max_ns=2431
  invalidations: count=164 invalidated_entries=88 evicted_entries=0 scanned_entries=2368
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
