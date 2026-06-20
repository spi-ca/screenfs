# ScreenFS benchmark result

- timestamp: `2026-06-20T16:31:36.532366+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload subtree_rename_cached_unrelated --iterations 10 --warmups 3 --small-files 200 --output-json docs/artifacts/mutation-invalidation-breadth-index/after-worktree-subtree-rename-cached-unrelated.json --output-md docs/artifacts/mutation-invalidation-breadth-index/after-worktree-subtree-rename-cached-unrelated.md --output-svg docs/artifacts/mutation-invalidation-breadth-index/after-worktree-subtree-rename-cached-unrelated.svg`
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
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `subtree_rename_cached_unrelated`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| subtree_rename_cached_unrelated | 0.025176 | 0.025914 | 0.026023 | 0.026110 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=53338 avg_ns=53338 max_ns=53338
  fuse_op.getattr: count=2679 total_ns=56414586 avg_ns=21058 max_ns=71681
  fuse_op.lookup: count=10937 total_ns=173031000 avg_ns=15820 max_ns=61593
  fuse_op.rename: count=26 total_ns=3884450 avg_ns=149401 max_ns=216718
  fuse_op.statfs: count=2 total_ns=5092 avg_ns=2546 max_ns=3544
  policy_decision: count=27962 total_ns=14189990 avg_ns=507 max_ns=6703
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=27806 total_ns=4154985 avg_ns=149 max_ns=1597
  matcher_candidate_order.path: count=83730 total_ns=16283343 avg_ns=194 max_ns=11332
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=27962
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=27962
  matcher_candidate_order_ancestor_steps: count=365600
  matcher_candidate_order_ancestor_steps.descendant: count=91140
  matcher_candidate_order_ancestor_steps.path: count=274460
  state_read_lock_wait: count=13669 total_ns=324142 avg_ns=23 max_ns=294
  state_read_lock_hold: count=13669 total_ns=1074094 avg_ns=78 max_ns=3719
  state_write_lock_wait: count=10909 total_ns=258549 avg_ns=23 max_ns=447
  state_write_lock_hold: count=10909 total_ns=6095953 avg_ns=558 max_ns=37075
  open_confined_openat2: count=13878 total_ns=9765843 avg_ns=703 max_ns=33006
  stat_child_no_follow: count=13825 total_ns=129223201 avg_ns=9347 max_ns=61240
  source_root_path: count=13799 total_ns=26250064 avg_ns=1902 max_ns=44036
  resolved_virtual_path: count=41420 total_ns=112517122 avg_ns=2716 max_ns=29992
  resolved_virtual_path_from_path: count=27543 total_ns=96840519 avg_ns=3515 max_ns=23390
  resolved_virtual_path_from_path_component_walk: count=27543 total_ns=85858818 avg_ns=3117 max_ns=22899
  resolved_virtual_path_from_path_canonicalize: count=62604 total_ns=70095329 avg_ns=1119 max_ns=17385
  resolved_virtual_path_from_path_source_root_confinement: count=62604 total_ns=8631018 avg_ns=137 max_ns=6954
  resolved_virtual_path_from_path_virtual_conversion: count=27543 total_ns=9297778 avg_ns=337 max_ns=8799
  resolved_virtual_path_from_open_fd: count=13877 total_ns=15676603 avg_ns=1129 max_ns=29992
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
  invalidations: count=26 invalidated_entries=52 evicted_entries=0 scanned_entries=52
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
