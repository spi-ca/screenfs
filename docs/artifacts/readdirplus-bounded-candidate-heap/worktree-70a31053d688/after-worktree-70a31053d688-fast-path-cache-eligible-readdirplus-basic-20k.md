# ScreenFS benchmark result

- timestamp: `2026-06-21T01:57:11.654164+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fast-path-cache-eligible --workload readdirplus_basic --dir-entries 20000 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.svg`
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
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
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
| readdirplus_basic | 0.016279 | 1.849429 | 113.607 | 2.172866 | 2.207019 | 2.234342 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=76427 avg_ns=76427 max_ns=76427
  fuse_op.getattr: count=260014 total_ns=2565781579 avg_ns=9867 max_ns=764459
  fuse_op.lookup: count=780031 total_ns=6887993798 avg_ns=8830 max_ns=1154802
  fuse_op.opendir: count=13 total_ns=325144 avg_ns=25011 max_ns=52257
  fuse_op.readdir: count=332 total_ns=7814776338 avg_ns=23538482 max_ns=55440862
  fuse_op.readdirplus: count=33 total_ns=989821376 avg_ns=29994587 max_ns=52622052
  fuse_op.releasedir: count=13 total_ns=19335617 avg_ns=1487355 max_ns=2324582
  fuse_op.statfs: count=2 total_ns=2307 avg_ns=1153 max_ns=1740
  policy_decision: count=5737598 total_ns=2088172016 avg_ns=363 max_ns=332980
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=5737598 total_ns=739770429 avg_ns=128 max_ns=262918
  matcher_candidate_order.path: count=17212794 total_ns=2260173998 avg_ns=131 max_ns=369075
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=81373824
  matcher_candidate_order_ancestor_steps.descendant: count=20343456
  matcher_candidate_order_ancestor_steps.path: count=61030368
  state_read_lock_wait: count=1040424 total_ns=21753955 avg_ns=20 max_ns=14077
  state_read_lock_hold: count=1040424 total_ns=73270293 avg_ns=70 max_ns=288025
  state_write_lock_wait: count=780433 total_ns=15060082 avg_ns=19 max_ns=8542
  state_write_lock_hold: count=780433 total_ns=813681145 avg_ns=1042 max_ns=2463443
  open_confined_openat2: count=1046710 total_ns=523811914 avg_ns=500 max_ns=106969
  open_like.pre_open_guard.access: count=1 total_ns=34623 avg_ns=34623 max_ns=34623
  open_like.pre_open_guard.opendir: count=13 total_ns=248226 avg_ns=19094 max_ns=44890
  open_like.post_open_revalidation.access: count=1 total_ns=35396 avg_ns=35396 max_ns=35396
  open_like.post_open_revalidation.opendir: count=13 total_ns=40629 avg_ns=3125 max_ns=4006
  stat_child_no_follow: count=1046331 total_ns=7076391373 avg_ns=6763 max_ns=1127453
  source_root_path: count=1046696 total_ns=1566855929 avg_ns=1496 max_ns=330300
  resolved_virtual_path: count=2093039 total_ns=2716230189 avg_ns=1297 max_ns=396513
  resolved_virtual_path_from_path: count=1046330 total_ns=1749288312 avg_ns=1671 max_ns=396513
  resolved_virtual_path_from_path_component_walk: count=1046330 total_ns=1429337482 avg_ns=1366 max_ns=394566
  resolved_virtual_path_from_path_canonicalize: count=1312549 total_ns=1128346113 avg_ns=859 max_ns=392946
  resolved_virtual_path_from_path_source_root_confinement: count=1312549 total_ns=155879573 avg_ns=118 max_ns=49057
  resolved_virtual_path_from_path_virtual_conversion: count=1046330 total_ns=263213399 avg_ns=251 max_ns=253965
  resolved_virtual_path_from_open_fd: count=1046709 total_ns=966941877 avg_ns=923 max_ns=348208
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=7423518745 avg_ns=22359996 max_ns=54391098
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=863291858 avg_ns=2600276 max_ns=9663847
  readdir_page_commit: count=332 total_ns=294550155 avg_ns=887199 max_ns=2463707
  readdirplus_directory_scan: count=33 total_ns=922468348 avg_ns=27953586 max_ns=50105908
  readdirplus_attr_generation_scan: count=6279 total_ns=42713077 avg_ns=6802 max_ns=260331
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=95995925 avg_ns=2908967 max_ns=6063289
  readdirplus_page_commit: count=33 total_ns=12892160 avg_ns=390671 max_ns=1024284
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
