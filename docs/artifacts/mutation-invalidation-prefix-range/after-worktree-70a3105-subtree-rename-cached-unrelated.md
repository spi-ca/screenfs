# ScreenFS benchmark result

- timestamp: `2026-06-21T02:31:50.860268+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload subtree_rename_cached_unrelated --small-files 200 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-subtree-rename-cached-unrelated.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-subtree-rename-cached-unrelated.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/after-worktree-70a3105-subtree-rename-cached-unrelated.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/state.rs;  M src/fs/tests/perf.rs; ... (+6 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `a31930e4cb63e4d8821e98a1383ddfd1b67dfee11d1a45dddd871aab69ae1c0d`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/state.rs;  M src/fs/tests/perf.rs; ... (+6 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `subtree_rename_cached_unrelated`
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

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| subtree_rename_cached_unrelated | 0.019719 | 0.021822 | 0.022516 | 0.023071 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65984 avg_ns=65984 max_ns=65984
  fuse_op.getattr: count=2679 total_ns=47616653 avg_ns=17774 max_ns=909440
  fuse_op.lookup: count=10937 total_ns=139526845 avg_ns=12757 max_ns=119160
  fuse_op.rename: count=26 total_ns=2893937 avg_ns=111305 max_ns=136618
  fuse_op.statfs: count=2 total_ns=3971 avg_ns=1985 max_ns=2268
  policy_decision: count=27962 total_ns=11459455 avg_ns=409 max_ns=1832
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=27806 total_ns=3402865 avg_ns=122 max_ns=17626
  matcher_candidate_order.path: count=83730 total_ns=13202427 avg_ns=157 max_ns=17294
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=27962
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=27962
  matcher_candidate_order_ancestor_steps: count=365600
  matcher_candidate_order_ancestor_steps.descendant: count=91140
  matcher_candidate_order_ancestor_steps.path: count=274460
  state_read_lock_wait: count=13669 total_ns=246037 avg_ns=17 max_ns=182
  state_read_lock_hold: count=13669 total_ns=836676 avg_ns=61 max_ns=970
  state_write_lock_wait: count=10909 total_ns=186773 avg_ns=17 max_ns=245
  state_write_lock_hold: count=10909 total_ns=4236114 avg_ns=388 max_ns=33570
  open_confined_openat2: count=13878 total_ns=6605202 avg_ns=475 max_ns=47775
  open_like.pre_open_guard.access: count=1 total_ns=54830 avg_ns=54830 max_ns=54830
  open_like.post_open_revalidation.access: count=1 total_ns=6161 avg_ns=6161 max_ns=6161
  stat_child_no_follow: count=13825 total_ns=100784579 avg_ns=7290 max_ns=243292
  source_root_path: count=13799 total_ns=18239848 avg_ns=1321 max_ns=30915
  resolved_virtual_path: count=41420 total_ns=95002519 avg_ns=2293 max_ns=884196
  resolved_virtual_path_from_path: count=27543 total_ns=83067541 avg_ns=3015 max_ns=884196
  resolved_virtual_path_from_path_component_walk: count=27543 total_ns=73612050 avg_ns=2672 max_ns=882823
  resolved_virtual_path_from_path_canonicalize: count=62604 total_ns=59604459 avg_ns=952 max_ns=881589
  resolved_virtual_path_from_path_source_root_confinement: count=62604 total_ns=8190742 avg_ns=130 max_ns=4176
  resolved_virtual_path_from_path_virtual_conversion: count=27543 total_ns=8131195 avg_ns=295 max_ns=15290
  resolved_virtual_path_from_open_fd: count=13877 total_ns=11934978 avg_ns=860 max_ns=233118
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
  invalidations: count=26 invalidated_entries=52 evicted_entries=0 scanned_entries=104
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
