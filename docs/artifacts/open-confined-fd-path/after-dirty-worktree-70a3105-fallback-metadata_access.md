# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:38.249378+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_access --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_access.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_access.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-metadata_access.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `77d2fc749459704dab16352ddbe5567c35d8ce313519dbc39fdf43a471081280`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/guards.rs; ?? docs/artifacts/open-confined-fd-path/; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_access`
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
| metadata_access | 0.001521 | 0.032825 | 21.585 | 0.033461 | 0.033513 | 0.033555 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=102749028 avg_ns=15434 max_ns=88169
  fuse_op.getattr: count=1 total_ns=13252 avg_ns=13252 max_ns=13252
  fuse_op.lookup: count=19973 total_ns=199564437 avg_ns=9991 max_ns=258352
  fuse_op.statfs: count=2 total_ns=4494 avg_ns=2247 max_ns=3035
  policy_decision: count=59918 total_ns=19065153 avg_ns=318 max_ns=98961
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=5638534 avg_ns=94 max_ns=8969
  matcher_candidate_order.path: count=179754 total_ns=21964496 avg_ns=122 max_ns=26104
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=692344
  matcher_candidate_order_ancestor_steps.descendant: count=173086
  matcher_candidate_order_ancestor_steps.path: count=519258
  state_read_lock_wait: count=26631 total_ns=449489 avg_ns=16 max_ns=104
  state_read_lock_hold: count=26631 total_ns=1417427 avg_ns=53 max_ns=2746
  state_write_lock_wait: count=19971 total_ns=323432 avg_ns=16 max_ns=815
  state_write_lock_hold: count=19971 total_ns=7301363 avg_ns=365 max_ns=18661
  open_confined_openat2: count=33288 total_ns=12603699 avg_ns=378 max_ns=12872
  open_like.pre_open_guard.access: count=6657 total_ns=79552781 avg_ns=11950 max_ns=78172
  open_like.post_open_revalidation.access: count=6657 total_ns=17084241 avg_ns=2566 max_ns=13755
  stat_child_no_follow: count=26631 total_ns=150838017 avg_ns=5664 max_ns=51709
  source_root_path: count=26631 total_ns=30648278 avg_ns=1150 max_ns=40033
  resolved_virtual_path: count=86546 total_ns=133055185 avg_ns=1537 max_ns=45187
  resolved_virtual_path_from_path: count=53259 total_ns=108000552 avg_ns=2027 max_ns=45187
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=91878313 avg_ns=1725 max_ns=44695
  resolved_virtual_path_from_path_canonicalize: count=93198 total_ns=72685373 avg_ns=779 max_ns=43889
  resolved_virtual_path_from_path_source_root_confinement: count=93198 total_ns=11475952 avg_ns=123 max_ns=4616
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=13621295 avg_ns=255 max_ns=15137
  resolved_virtual_path_from_open_fd: count=33287 total_ns=25054633 avg_ns=752 max_ns=12166
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
