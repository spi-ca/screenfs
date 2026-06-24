# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:46.195487+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fast-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fast-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fast-open-confined-surface.svg --workload-set open-confined-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
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
| metadata_open | 0.004634 | 0.022074 | 4.763 | 0.026986 | 0.027564 | 0.028026 |
| metadata_opendir | 0.000948 | 0.017889 | 18.870 | 0.018830 | 0.019085 | 0.019290 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=109024 avg_ns=109024 max_ns=109024
  fuse_op.getattr: count=6657 total_ns=16846332 avg_ns=2530 max_ns=127385
  fuse_op.lookup: count=33285 total_ns=164627414 avg_ns=4945 max_ns=16070342
  fuse_op.open: count=6656 total_ns=60923037 avg_ns=9153 max_ns=108650
  fuse_op.opendir: count=6656 total_ns=50011294 avg_ns=7513 max_ns=116308
  fuse_op.release: count=6656 total_ns=5407582 avg_ns=812 max_ns=390633
  fuse_op.releasedir: count=6656 total_ns=1507832 avg_ns=226 max_ns=1436
  fuse_op.statfs: count=2 total_ns=3471 avg_ns=1735 max_ns=2298
  policy_decision: count=66568 total_ns=21073447 avg_ns=316 max_ns=16766
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=66568 total_ns=7745948 avg_ns=116 max_ns=8360
  matcher_candidate_order.path: count=199704 total_ns=25004572 avg_ns=125 max_ns=22709
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=825420
  matcher_candidate_order_ancestor_steps.descendant: count=206355
  matcher_candidate_order_ancestor_steps.path: count=619065
  state_read_lock_wait: count=53255 total_ns=1050757 avg_ns=19 max_ns=5507
  state_read_lock_hold: count=53255 total_ns=3294345 avg_ns=61 max_ns=15990
  state_write_lock_wait: count=59907 total_ns=1190087 avg_ns=19 max_ns=15049
  state_write_lock_hold: count=59907 total_ns=15227964 avg_ns=254 max_ns=151791
  open_confined_openat2: count=66568 total_ns=86808066 avg_ns=1304 max_ns=16050163
  open_like.pre_open_guard.access: count=1 total_ns=47948 avg_ns=47948 max_ns=47948
  open_like.pre_open_guard.open: count=6656 total_ns=19424221 avg_ns=2918 max_ns=31470
  open_like.pre_open_guard.opendir: count=6656 total_ns=17189248 avg_ns=2582 max_ns=111818
  open_like.post_open_revalidation.access: count=1 total_ns=45036 avg_ns=45036 max_ns=45036
  open_like.post_open_revalidation.open: count=6656 total_ns=34150154 avg_ns=5130 max_ns=105436
  open_like.post_open_revalidation.opendir: count=6656 total_ns=25575462 avg_ns=3842 max_ns=69439
  stat_child_no_follow: count=53255 total_ns=105629095 avg_ns=1983 max_ns=16058171
  stat_child_no_follow.attr_conversion: count=53253 total_ns=788903 avg_ns=14 max_ns=11379
  stat_child_no_follow.host_fstat: count=53253 total_ns=8225610 avg_ns=154 max_ns=3879
  stat_child_no_follow_context.path_guard_or_metadata: count=53255 total_ns=105629095 avg_ns=1983 max_ns=16058171
  source_root_path: count=13313 total_ns=20044301 avg_ns=1505 max_ns=41508
  resolved_virtual_path: count=13313 total_ns=16855098 avg_ns=1266 max_ns=102120
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=13313 total_ns=16855098 avg_ns=1266 max_ns=102120
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
  readdirplus_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_entries: count=0
  readdirplus_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
