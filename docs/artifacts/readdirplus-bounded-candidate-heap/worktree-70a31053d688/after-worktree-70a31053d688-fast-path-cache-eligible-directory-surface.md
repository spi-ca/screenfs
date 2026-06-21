# ScreenFS benchmark result

- timestamp: `2026-06-21T01:56:03.948396+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/tests/state_cache.rs; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `0a05d5096ce79b996b3e7ff85f7031cff909bae46f001f1c3ab093dad3458000`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/tests/state_cache.rs; ... (+3 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.000637 | 0.048839 | 76.612 | 0.050574 | 0.051081 | 0.051487 |
| readdirplus_basic | 0.003781 | 0.395960 | 104.729 | 0.420306 | 0.425590 | 0.429818 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62504 avg_ns=62504 max_ns=62504
  fuse_op.getattr: count=65027 total_ns=657956943 avg_ns=10118 max_ns=278844
  fuse_op.lookup: count=195057 total_ns=1749124421 avg_ns=8967 max_ns=312799
  fuse_op.opendir: count=26 total_ns=367746 avg_ns=14144 max_ns=26299
  fuse_op.readdir: count=180 total_ns=1080561725 avg_ns=6003120 max_ns=13889632
  fuse_op.readdirplus: count=31 total_ns=339790220 avg_ns=10960974 max_ns=15719379
  fuse_op.releasedir: count=26 total_ns=9337907 avg_ns=359150 max_ns=715204
  fuse_op.statfs: count=2 total_ns=5103 avg_ns=2551 max_ns=3339
  policy_decision: count=1099907 total_ns=378073544 avg_ns=343 max_ns=1076975
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=135094426 avg_ns=122 max_ns=259760
  matcher_candidate_order.path: count=3299721 total_ns=413300363 avg_ns=125 max_ns=307864
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5785369 avg_ns=22 max_ns=13364
  state_read_lock_hold: count=260322 total_ns=18722740 avg_ns=71 max_ns=26351
  state_write_lock_wait: count=195344 total_ns=3894935 avg_ns=19 max_ns=2787
  state_write_lock_hold: count=195344 total_ns=241156986 avg_ns=1234 max_ns=2144146
  open_confined_openat2: count=266229 total_ns=139313204 avg_ns=523 max_ns=101343
  open_like.pre_open_guard.access: count=1 total_ns=24592 avg_ns=24592 max_ns=24592
  open_like.pre_open_guard.opendir: count=26 total_ns=239973 avg_ns=9229 max_ns=20737
  open_like.post_open_revalidation.access: count=1 total_ns=33200 avg_ns=33200 max_ns=33200
  open_like.post_open_revalidation.opendir: count=26 total_ns=72501 avg_ns=2788 max_ns=3501
  stat_child_no_follow: count=265991 total_ns=1846514822 avg_ns=6942 max_ns=313952
  source_root_path: count=266202 total_ns=415315734 avg_ns=1560 max_ns=269842
  resolved_virtual_path: count=532218 total_ns=706278134 avg_ns=1327 max_ns=268979
  resolved_virtual_path_from_path: count=265990 total_ns=453307292 avg_ns=1704 max_ns=268979
  resolved_virtual_path_from_path_component_walk: count=265990 total_ns=374632566 avg_ns=1408 max_ns=268680
  resolved_virtual_path_from_path_canonicalize: count=336765 total_ns=297803569 avg_ns=884 max_ns=268464
  resolved_virtual_path_from_path_source_root_confinement: count=336765 total_ns=40322526 avg_ns=119 max_ns=44928
  resolved_virtual_path_from_path_virtual_conversion: count=265990 total_ns=64163058 avg_ns=241 max_ns=17741
  resolved_virtual_path_from_open_fd: count=266228 total_ns=252970842 avg_ns=950 max_ns=257577
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=924388974 avg_ns=5135494 max_ns=12782092
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=82746574 avg_ns=459703 max_ns=1893904
  readdir_page_commit: count=180 total_ns=114453791 avg_ns=635854 max_ns=2144268
  readdirplus_directory_scan: count=31 total_ns=283345787 avg_ns=9140186 max_ns=13165652
  readdirplus_attr_generation_scan: count=5859 total_ns=40713995 avg_ns=6948 max_ns=314011
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=29365577 avg_ns=947276 max_ns=1841893
  readdirplus_page_commit: count=31 total_ns=4591027 avg_ns=148097 max_ns=273585
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
