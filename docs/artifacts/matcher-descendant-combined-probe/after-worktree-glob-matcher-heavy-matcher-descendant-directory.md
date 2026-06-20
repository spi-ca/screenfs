# ScreenFS benchmark result

- timestamp: `2026-06-20T16:07:12.422577+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set matcher-descendant-directory --matcher-extra-rules 32 --policy-label glob-matcher-heavy --iterations 10 --warmups 3 --dir-entries 200 --output-json docs/artifacts/matcher-descendant-combined-probe/after-worktree-glob-matcher-heavy-matcher-descendant-directory.json --output-md docs/artifacts/matcher-descendant-combined-probe/after-worktree-glob-matcher-heavy-matcher-descendant-directory.md --output-svg docs/artifacts/matcher-descendant-combined-probe/after-worktree-glob-matcher-heavy-matcher-descendant-directory.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+10 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `54df5851f98953e2f644f3b7a5cd6d78c496bacaec5d83569e530717218b7c13`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+10 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| matcher_descendant_readdir | 0.000022 | 0.000879 | 39.999 | 0.000927 | 0.000992 | 0.001044 |
| matcher_descendant_readdirplus | 0.000046 | 0.004185 | 91.456 | 0.004340 | 0.004367 | 0.004388 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=42090 avg_ns=42090 max_ns=42090
  fuse_op.getattr: count=443 total_ns=9208667 avg_ns=20787 max_ns=37433
  fuse_op.lookup: count=1747 total_ns=25257791 avg_ns=14457 max_ns=87821
  fuse_op.opendir: count=26 total_ns=586950 avg_ns=22575 max_ns=35718
  fuse_op.readdir: count=26 total_ns=422336 avg_ns=16243 max_ns=44581
  fuse_op.readdirplus: count=26 total_ns=17974386 avg_ns=691322 max_ns=815806
  fuse_op.releasedir: count=26 total_ns=175655 avg_ns=6755 max_ns=15539
  fuse_op.statfs: count=2 total_ns=2072 avg_ns=1036 max_ns=1107
  policy_decision: count=7112 total_ns=9726067 avg_ns=1367 max_ns=50372
  matcher_candidates: count=154742
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=154742
  matcher_candidate_order.descendant: count=7112 total_ns=2732870 avg_ns=384 max_ns=7183
  matcher_candidate_order.path: count=21336 total_ns=7136396 avg_ns=334 max_ns=33658
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=696976
  matcher_candidate_order_seen_slots.descendant: count=227584
  matcher_candidate_order_seen_slots.path: count=469392
  matcher_candidate_order_ancestor_steps: count=107136
  matcher_candidate_order_ancestor_steps.descendant: count=26784
  matcher_candidate_order_ancestor_steps.path: count=80352
  state_read_lock_wait: count=2269 total_ns=41513 avg_ns=18 max_ns=135
  state_read_lock_hold: count=2269 total_ns=177429 avg_ns=78 max_ns=14490
  state_write_lock_wait: count=1875 total_ns=33381 avg_ns=17 max_ns=272
  state_write_lock_hold: count=1875 total_ns=1018315 avg_ns=543 max_ns=30350
  open_confined_openat2: count=3180 total_ns=1479819 avg_ns=465 max_ns=5387
  stat_child_no_follow: count=3101 total_ns=26611247 avg_ns=8581 max_ns=59794
  source_root_path: count=3153 total_ns=3752637 avg_ns=1190 max_ns=19480
  resolved_virtual_path: count=8546 total_ns=18803263 avg_ns=2200 max_ns=77758
  resolved_virtual_path_from_path: count=5367 total_ns=15969290 avg_ns=2975 max_ns=77758
  resolved_virtual_path_from_path_component_walk: count=5367 total_ns=14084420 avg_ns=2624 max_ns=77375
  resolved_virtual_path_from_path_canonicalize: count=12780 total_ns=11510612 avg_ns=900 max_ns=76869
  resolved_virtual_path_from_path_source_root_confinement: count=12780 total_ns=1402645 avg_ns=109 max_ns=14806
  resolved_virtual_path_from_path_virtual_conversion: count=5367 total_ns=1623030 avg_ns=302 max_ns=1519
  resolved_virtual_path_from_open_fd: count=3179 total_ns=2833973 avg_ns=891 max_ns=8738
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=153975 avg_ns=5922 max_ns=18001
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6990 avg_ns=268 max_ns=675
  readdirplus_directory_scan: count=26 total_ns=4379940 avg_ns=168459 max_ns=230664
  readdirplus_attr_generation_scan: count=858 total_ns=8385007 avg_ns=9772 max_ns=59855
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=87969 avg_ns=3383 max_ns=8151
  readdirplus_page_commit: count=26 total_ns=401737 avg_ns=15451 max_ns=30444
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
