# ScreenFS benchmark result

- timestamp: `2026-06-20T16:20:51.523539+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin /tmp/screenfs-matcher-before-920c00e/target/release/screenfs --screenfs-source-root /tmp/screenfs-matcher-before-920c00e --policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --iterations 10 --warmups 3 --symlink-parent-mutations 20 --small-files 200 --output-json docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-mutation-invalidation.json --output-md docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-mutation-invalidation.md --output-svg docs/artifacts/mutation-invalidation-breadth-index/before-920c00e-mutation-invalidation.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+7 more)`
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
| symlink_parent_mkdir_rmdir | 0.014570 | 0.014894 | 0.015156 | 0.015365 |
| pinned_symlink_parent_mkdir_rmdir | 0.021862 | 0.022464 | 0.022673 | 0.022840 |
| subtree_rename_cached_unrelated | 0.018768 | 0.019719 | 0.019990 | 0.020207 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=33741 avg_ns=33741 max_ns=33741
  fuse_op.getattr: count=3771 total_ns=61728348 avg_ns=16369 max_ns=100879
  fuse_op.lookup: count=27798 total_ns=340754356 avg_ns=12258 max_ns=105190
  fuse_op.mkdir: count=520 total_ns=44094741 avg_ns=84797 max_ns=135596
  fuse_op.opendir: count=273 total_ns=4428542 avg_ns=16221 max_ns=20856
  fuse_op.readdirplus: count=273 total_ns=15557999 avg_ns=56989 max_ns=84094
  fuse_op.readlink: count=2392 total_ns=41274353 avg_ns=17255 max_ns=82030
  fuse_op.releasedir: count=273 total_ns=145304 avg_ns=532 max_ns=2305
  fuse_op.rename: count=26 total_ns=3721995 avg_ns=143153 max_ns=163395
  fuse_op.rmdir: count=520 total_ns=34754828 avg_ns=66836 max_ns=154725
  fuse_op.statfs: count=2 total_ns=4052 avg_ns=2026 max_ns=3049
  policy_decision: count=95211 total_ns=40651773 avg_ns=426 max_ns=26412
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91935 total_ns=11310811 avg_ns=123 max_ns=7269
  matcher_candidate_order.path: count=282357 total_ns=45266976 avg_ns=160 max_ns=32257
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=95211
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=95211
  matcher_candidate_order_ancestor_steps: count=1291408
  matcher_candidate_order_ancestor_steps.descendant: count=315832
  matcher_candidate_order_ancestor_steps.path: count=975576
  state_read_lock_wait: count=35600 total_ns=1088884 avg_ns=30 max_ns=14537
  state_read_lock_hold: count=35600 total_ns=2229477 avg_ns=62 max_ns=48229
  state_write_lock_wait: count=28342 total_ns=488603 avg_ns=17 max_ns=1878
  state_write_lock_hold: count=28342 total_ns=10191961 avg_ns=359 max_ns=49466
  open_confined_openat2: count=41841 total_ns=18163332 avg_ns=434 max_ns=85646
  stat_child_no_follow: count=40202 total_ns=261991807 avg_ns=6516 max_ns=94568
  source_root_path: count=40449 total_ns=48629231 avg_ns=1202 max_ns=63325
  resolved_virtual_path: count=118484 total_ns=248949798 avg_ns=2101 max_ns=81696
  resolved_virtual_path_from_path: count=76644 total_ns=214093830 avg_ns=2793 max_ns=81696
  resolved_virtual_path_from_path_component_walk: count=76644 total_ns=187928971 avg_ns=2451 max_ns=81141
  resolved_virtual_path_from_path_canonicalize: count=171817 total_ns=154055164 avg_ns=896 max_ns=80530
  resolved_virtual_path_from_path_source_root_confinement: count=171817 total_ns=18715271 avg_ns=108 max_ns=14443
  resolved_virtual_path_from_path_virtual_conversion: count=76644 total_ns=22567794 avg_ns=294 max_ns=25039
  resolved_virtual_path_from_open_fd: count=41840 total_ns=34855968 avg_ns=833 max_ns=33940
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
  readdirplus_directory_scan: count=273 total_ns=1871665 avg_ns=6855 max_ns=18616
  readdirplus_attr_generation_scan: count=806 total_ns=4751932 avg_ns=5895 max_ns=11653
  readdirplus_attr_generation_entries: count=533
  readdirplus_symlink_visibility: count=273 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=273 total_ns=88328 avg_ns=323 max_ns=11536
  readdirplus_page_commit: count=273 total_ns=312001 avg_ns=1142 max_ns=2179
  invalidations: count=1066 invalidated_entries=572 evicted_entries=0 scanned_entries=15392
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
