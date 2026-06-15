# ScreenFS benchmark result

- timestamp: `2026-06-15T07:58:52.334083+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --iterations 10 --warmups 3 --policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum --perf-counters --read-mib 16 --write-mib 16 --rand-io-ops 4096 --sync-ops 128 --open-read-close-ops 1024 --screenfs-bin target/release/screenfs --screenfs-source-root . --output-json docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json --output-md docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.md --output-svg docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+80 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `6cbc30137108ace1e7df09c285534465afa74d61b7b2a9bc88e54cf63198aff6`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+80 more)`
- screenfs_source_git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `per-open-cache-minimum`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| rand_read_4k | 0.005425 | 0.111830 | 20.612 | 0.115624 | 0.115627 | 0.115630 |
| rand_write_4k | 0.005533 | 0.122293 | 22.101 | 0.123902 | 0.124024 | 0.124120 |
| sync_write_4k | 0.000156 | 0.004010 | 25.646 | 0.004269 | 0.004333 | 0.004385 |
| small_open_read_close | 0.008473 | 0.158113 | 18.661 | 0.160111 | 0.160323 | 0.160492 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=91358 avg_ns=91358 max_ns=91358
  fuse_op.create: count=26 total_ns=2687011 avg_ns=103346 max_ns=151120
  fuse_op.flush: count=13351 total_ns=71927035 avg_ns=5387 max_ns=338070
  fuse_op.fsync: count=52 total_ns=426159 avg_ns=8195 max_ns=30646
  fuse_op.getattr: count=79886 total_ns=1129913374 avg_ns=14144 max_ns=139834
  fuse_op.getxattr: count=54925 total_ns=808997416 avg_ns=14729 max_ns=72867
  fuse_op.lookup: count=40175 total_ns=570197593 avg_ns=14192 max_ns=92934
  fuse_op.open: count=13325 total_ns=190623676 avg_ns=14305 max_ns=61441
  fuse_op.read: count=48763 total_ns=109779440 avg_ns=2251 max_ns=87037
  fuse_op.release: count=13351 total_ns=8410272 avg_ns=629 max_ns=9563
  fuse_op.setattr: count=13 total_ns=518034 avg_ns=39848 max_ns=52484
  fuse_op.statfs: count=2 total_ns=5762 avg_ns=2881 max_ns=3180
  fuse_op.unlink: count=26 total_ns=9760988 avg_ns=375422 max_ns=855408
  fuse_op.write: count=54912 total_ns=129838184 avg_ns=2364 max_ns=116547
  policy_decision: count=565923 total_ns=224146351 avg_ns=396 max_ns=19753
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=565715 total_ns=81522710 avg_ns=144 max_ns=26536
  matcher_candidate_order.path: count=1697561 total_ns=255185920 avg_ns=150 max_ns=19404
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=7335128
  matcher_candidate_order_ancestor_steps.descendant: count=1833418
  matcher_candidate_order_ancestor_steps.path: count=5501710
  state_read_lock_wait: count=305455 total_ns=7550103 avg_ns=24 max_ns=5296
  state_read_lock_hold: count=305455 total_ns=23579256 avg_ns=77 max_ns=10285
  state_write_lock_wait: count=66875 total_ns=1631793 avg_ns=24 max_ns=834
  state_write_lock_hold: count=66875 total_ns=29648638 avg_ns=443 max_ns=55152
  open_confined_openat2: count=376962 total_ns=242564239 avg_ns=643 max_ns=89657
  source_root_path: count=376987 total_ns=659193863 avg_ns=1748 max_ns=63670
  resolved_virtual_path: count=377039 total_ns=429528492 avg_ns=1139 max_ns=41115
  resolved_virtual_path_from_path: count=52 total_ns=218331 avg_ns=4198 max_ns=8373
  resolved_virtual_path_from_path_component_walk: count=52 total_ns=185810 avg_ns=3573 max_ns=7436
  resolved_virtual_path_from_path_canonicalize: count=104 total_ns=127405 avg_ns=1225 max_ns=3831
  resolved_virtual_path_from_path_source_root_confinement: count=104 total_ns=28394 avg_ns=273 max_ns=1320
  resolved_virtual_path_from_path_virtual_conversion: count=52 total_ns=21683 avg_ns=416 max_ns=830
  resolved_virtual_path_from_open_fd: count=376987 total_ns=429310161 avg_ns=1138 max_ns=41115
  read_handle_snapshot: count=48763 total_ns=10011140 avg_ns=205 max_ns=7827
  read_guard_path: count=48763 total_ns=998250 avg_ns=20 max_ns=4891
  read_io: count=48763 total_ns=90214668 avg_ns=1850 max_ns=86642
  write_handle_snapshot: count=54912 total_ns=11247865 avg_ns=204 max_ns=5694
  write_guard_mutation: count=54912 total_ns=1089242 avg_ns=19 max_ns=1599
  write_io: count=54912 total_ns=107556676 avg_ns=1958 max_ns=114778
  read_size_bucket.0_4k: count=44889 total_ns=74752162 avg_ns=1665 max_ns=34951
  read_size_bucket.4k_64k: count=3874 total_ns=15462506 avg_ns=3991 max_ns=86642
  write_size_bucket.0_4k: count=54912 total_ns=107556676 avg_ns=1958 max_ns=114778
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
