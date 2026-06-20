# ScreenFS benchmark result

- timestamp: `2026-06-20T16:20:52.642274+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin /tmp/screenfs-matcher-before-920c00e/target/release/screenfs --screenfs-source-root /tmp/screenfs-matcher-before-920c00e --policy-preset fallback-unsafe-policy --workload subtree_rename_cached_unrelated --iterations 10 --warmups 3 --small-files 200 --output-json docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-subtree-rename-cached-unrelated.json --output-md docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-subtree-rename-cached-unrelated.md --output-svg docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-subtree-rename-cached-unrelated.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+8 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-before-920c00e/target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/tmp/screenfs-matcher-before-920c00e`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `True`
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
| subtree_rename_cached_unrelated | 0.024493 | 0.025464 | 0.025893 | 0.026237 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=50789 avg_ns=50789 max_ns=50789
  fuse_op.getattr: count=2679 total_ns=56275000 avg_ns=21005 max_ns=42899
  fuse_op.lookup: count=10937 total_ns=171227965 avg_ns=15655 max_ns=72900
  fuse_op.rename: count=26 total_ns=4778679 avg_ns=183795 max_ns=214249
  fuse_op.statfs: count=2 total_ns=4354 avg_ns=2177 max_ns=2522
  policy_decision: count=27962 total_ns=14475827 avg_ns=517 max_ns=21166
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=27806 total_ns=4212046 avg_ns=151 max_ns=5713
  matcher_candidate_order.path: count=83730 total_ns=16498327 avg_ns=197 max_ns=2872
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=27962
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=27962
  matcher_candidate_order_ancestor_steps: count=365600
  matcher_candidate_order_ancestor_steps.descendant: count=91140
  matcher_candidate_order_ancestor_steps.path: count=274460
  state_read_lock_wait: count=13669 total_ns=331140 avg_ns=24 max_ns=245
  state_read_lock_hold: count=13669 total_ns=961751 avg_ns=70 max_ns=2181
  state_write_lock_wait: count=10909 total_ns=254241 avg_ns=23 max_ns=297
  state_write_lock_hold: count=10909 total_ns=6118990 avg_ns=560 max_ns=54877
  open_confined_openat2: count=13878 total_ns=9503173 avg_ns=684 max_ns=10453
  stat_child_no_follow: count=13825 total_ns=128537688 avg_ns=9297 max_ns=61731
  source_root_path: count=13799 total_ns=25981089 avg_ns=1882 max_ns=30426
  resolved_virtual_path: count=41420 total_ns=112049236 avg_ns=2705 max_ns=23578
  resolved_virtual_path_from_path: count=27543 total_ns=96296978 avg_ns=3496 max_ns=22193
  resolved_virtual_path_from_path_component_walk: count=27543 total_ns=85187503 avg_ns=3092 max_ns=21514
  resolved_virtual_path_from_path_canonicalize: count=62604 total_ns=70458888 avg_ns=1125 max_ns=16845
  resolved_virtual_path_from_path_source_root_confinement: count=62604 total_ns=7567175 avg_ns=120 max_ns=9302
  resolved_virtual_path_from_path_virtual_conversion: count=27543 total_ns=9460467 avg_ns=343 max_ns=6218
  resolved_virtual_path_from_open_fd: count=13877 total_ns=15752258 avg_ns=1135 max_ns=23578
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
  invalidations: count=26 invalidated_entries=52 evicted_entries=0 scanned_entries=10764
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
