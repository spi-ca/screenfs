# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:34.375949+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/after-dirty-worktree-70a3105-fallback-open-confined-surface.svg`
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
| metadata_open | 0.001639 | 0.053663 | 32.745 | 0.061205 | 0.061351 | 0.061468 |
| metadata_opendir | 0.000572 | 0.038556 | 67.358 | 0.040643 | 0.041441 | 0.042079 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=71329 avg_ns=71329 max_ns=71329
  fuse_op.getattr: count=6657 total_ns=80439118 avg_ns=12083 max_ns=48117
  fuse_op.lookup: count=33285 total_ns=462906645 avg_ns=13907 max_ns=14755109
  fuse_op.open: count=6656 total_ns=130719869 avg_ns=19639 max_ns=240269
  fuse_op.opendir: count=6656 total_ns=106577053 avg_ns=16012 max_ns=61264
  fuse_op.release: count=6656 total_ns=5320827 avg_ns=799 max_ns=13907
  fuse_op.releasedir: count=6656 total_ns=2219059 avg_ns=333 max_ns=3900
  fuse_op.statfs: count=2 total_ns=4855 avg_ns=2427 max_ns=2988
  policy_decision: count=119822 total_ns=43128324 avg_ns=359 max_ns=13769
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=12459823 avg_ns=103 max_ns=13642
  matcher_candidate_order.path: count=359466 total_ns=49484450 avg_ns=137 max_ns=14604
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=119822
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=119822
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=1202016 avg_ns=22 max_ns=2902
  state_read_lock_hold: count=53255 total_ns=3656130 avg_ns=68 max_ns=4854
  state_write_lock_wait: count=59907 total_ns=1314397 avg_ns=21 max_ns=633
  state_write_lock_hold: count=59907 total_ns=17948609 avg_ns=299 max_ns=45168
  open_confined_openat2: count=66568 total_ns=94622674 avg_ns=1421 max_ns=14739934
  open_like.pre_open_guard.access: count=1 total_ns=62181 avg_ns=62181 max_ns=62181
  open_like.pre_open_guard.open: count=6656 total_ns=100461714 avg_ns=15093 max_ns=116355
  open_like.pre_open_guard.opendir: count=6656 total_ns=79160105 avg_ns=11893 max_ns=38071
  open_like.post_open_revalidation.access: count=1 total_ns=3897 avg_ns=3897 max_ns=3897
  open_like.post_open_revalidation.open: count=6656 total_ns=21537756 avg_ns=3235 max_ns=221839
  open_like.post_open_revalidation.opendir: count=6656 total_ns=18994944 avg_ns=2853 max_ns=8368
  stat_child_no_follow: count=53255 total_ns=444628901 avg_ns=8349 max_ns=14750445
  source_root_path: count=53255 total_ns=94991995 avg_ns=1783 max_ns=42924
  resolved_virtual_path: count=173074 total_ns=302946778 avg_ns=1750 max_ns=216601
  resolved_virtual_path_from_path: count=106507 total_ns=230906871 avg_ns=2167 max_ns=16593
  resolved_virtual_path_from_path_component_walk: count=106507 total_ns=193919890 avg_ns=1820 max_ns=15931
  resolved_virtual_path_from_path_canonicalize: count=159758 total_ns=153096856 avg_ns=958 max_ns=14610
  resolved_virtual_path_from_path_source_root_confinement: count=159758 total_ns=21567880 avg_ns=135 max_ns=6843
  resolved_virtual_path_from_path_virtual_conversion: count=106507 total_ns=30833835 avg_ns=289 max_ns=7265
  resolved_virtual_path_from_open_fd: count=66567 total_ns=72039907 avg_ns=1082 max_ns=216601
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
