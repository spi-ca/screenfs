# ScreenFS benchmark result

- timestamp: `2026-06-20T16:31:35.397774+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --iterations 10 --warmups 3 --symlink-parent-mutations 20 --small-files 200 --output-json docs/artifacts/mutation-invalidation-breadth-index/after-worktree-mutation-invalidation.json --output-md docs/artifacts/mutation-invalidation-breadth-index/after-worktree-mutation-invalidation.md --output-svg docs/artifacts/mutation-invalidation-breadth-index/after-worktree-mutation-invalidation.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a9f24a5bfc60789f058666dc58010a4d2525ad12d3e7253b557b548fd2652227`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+11 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir, subtree_rename_cached_unrelated`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.018056 | 0.018405 | 0.018681 | 0.018903 |
| pinned_symlink_parent_mkdir_rmdir | 0.028319 | 0.030077 | 0.030248 | 0.030386 |
| subtree_rename_cached_unrelated | 0.027016 | 0.028013 | 0.028015 | 0.028017 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=45671 avg_ns=45671 max_ns=45671
  fuse_op.getattr: count=3771 total_ns=79658270 avg_ns=21123 max_ns=63961
  fuse_op.lookup: count=27798 total_ns=447586164 avg_ns=16101 max_ns=69690
  fuse_op.mkdir: count=520 total_ns=56598745 avg_ns=108843 max_ns=165887
  fuse_op.opendir: count=273 total_ns=5758607 avg_ns=21093 max_ns=29493
  fuse_op.readdirplus: count=273 total_ns=20156697 avg_ns=73834 max_ns=112873
  fuse_op.readlink: count=2392 total_ns=51461559 avg_ns=21514 max_ns=49678
  fuse_op.releasedir: count=273 total_ns=189971 avg_ns=695 max_ns=2764
  fuse_op.rename: count=26 total_ns=3921444 avg_ns=150824 max_ns=182578
  fuse_op.rmdir: count=520 total_ns=43763212 avg_ns=84160 max_ns=377369
  fuse_op.statfs: count=2 total_ns=3220 avg_ns=1610 max_ns=1663
  policy_decision: count=95211 total_ns=52230457 avg_ns=548 max_ns=271524
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91935 total_ns=13967205 avg_ns=151 max_ns=3810
  matcher_candidate_order.path: count=282357 total_ns=56360785 avg_ns=199 max_ns=11369
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=95211
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=95211
  matcher_candidate_order_ancestor_steps: count=1291408
  matcher_candidate_order_ancestor_steps.descendant: count=315832
  matcher_candidate_order_ancestor_steps.path: count=975576
  state_read_lock_wait: count=35600 total_ns=851372 avg_ns=23 max_ns=385
  state_read_lock_hold: count=35600 total_ns=2791759 avg_ns=78 max_ns=14595
  state_write_lock_wait: count=28342 total_ns=690099 avg_ns=24 max_ns=688
  state_write_lock_hold: count=28342 total_ns=14850817 avg_ns=523 max_ns=49771
  open_confined_openat2: count=41841 total_ns=28803651 avg_ns=688 max_ns=27342
  stat_child_no_follow: count=40202 total_ns=356577818 avg_ns=8869 max_ns=56195
  source_root_path: count=40449 total_ns=78130722 avg_ns=1931 max_ns=24210
  resolved_virtual_path: count=118484 total_ns=313364043 avg_ns=2644 max_ns=46905
  resolved_virtual_path_from_path: count=76644 total_ns=265652501 avg_ns=3466 max_ns=46905
  resolved_virtual_path_from_path_component_walk: count=76644 total_ns=235222001 avg_ns=3069 max_ns=45964
  resolved_virtual_path_from_path_canonicalize: count=171817 total_ns=192772259 avg_ns=1121 max_ns=44749
  resolved_virtual_path_from_path_source_root_confinement: count=171817 total_ns=23045955 avg_ns=134 max_ns=10302
  resolved_virtual_path_from_path_virtual_conversion: count=76644 total_ns=25722833 avg_ns=335 max_ns=8791
  resolved_virtual_path_from_open_fd: count=41840 total_ns=47711542 avg_ns=1140 max_ns=22414
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
  readdirplus_directory_scan: count=273 total_ns=2435093 avg_ns=8919 max_ns=26969
  readdirplus_attr_generation_scan: count=806 total_ns=6096950 avg_ns=7564 max_ns=18586
  readdirplus_attr_generation_entries: count=533
  readdirplus_symlink_visibility: count=273 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=273 total_ns=138606 avg_ns=507 max_ns=17812
  readdirplus_page_commit: count=273 total_ns=460924 avg_ns=1688 max_ns=6759
  invalidations: count=1066 invalidated_entries=572 evicted_entries=0 scanned_entries=572
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
