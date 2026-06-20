# ScreenFS benchmark result

- timestamp: `2026-06-20T16:06:20.612611+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin /tmp/screenfs-matcher-before-920c00e/target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set matcher-descendant-directory --matcher-extra-rules 32 --policy-label glob-matcher-heavy --iterations 10 --warmups 3 --dir-entries 200 --output-json docs/artifacts/matcher-descendant-combined-probe/before-920c00e-glob-matcher-heavy-matcher-descendant-directory.json --output-md docs/artifacts/matcher-descendant-combined-probe/before-920c00e-glob-matcher-heavy-matcher-descendant-directory.md --output-svg docs/artifacts/matcher-descendant-combined-probe/before-920c00e-glob-matcher-heavy-matcher-descendant-directory.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+10 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-before-920c00e/target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/tmp/screenfs-matcher-before-920c00e`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `True`
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
| matcher_descendant_readdir | 0.000016 | 0.000918 | 58.086 | 0.000990 | 0.001009 | 0.001024 |
| matcher_descendant_readdirplus | 0.000051 | 0.004087 | 80.931 | 0.004297 | 0.004301 | 0.004305 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=69514 avg_ns=69514 max_ns=69514
  fuse_op.getattr: count=443 total_ns=9535871 avg_ns=21525 max_ns=53469
  fuse_op.lookup: count=1747 total_ns=25313239 avg_ns=14489 max_ns=67408
  fuse_op.opendir: count=26 total_ns=550186 avg_ns=21161 max_ns=32244
  fuse_op.readdir: count=26 total_ns=451016 avg_ns=17346 max_ns=62850
  fuse_op.readdirplus: count=26 total_ns=18436663 avg_ns=709102 max_ns=896564
  fuse_op.releasedir: count=26 total_ns=181680 avg_ns=6987 max_ns=16014
  fuse_op.statfs: count=2 total_ns=7603 avg_ns=3801 max_ns=6249
  policy_decision: count=7112 total_ns=9922106 avg_ns=1395 max_ns=31563
  matcher_candidates: count=154742
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=154742
  matcher_candidate_order.descendant: count=7112 total_ns=2760257 avg_ns=388 max_ns=5333
  matcher_candidate_order.path: count=21336 total_ns=7120776 avg_ns=333 max_ns=6820
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=696976
  matcher_candidate_order_seen_slots.descendant: count=227584
  matcher_candidate_order_seen_slots.path: count=469392
  matcher_candidate_order_ancestor_steps: count=107136
  matcher_candidate_order_ancestor_steps.descendant: count=26784
  matcher_candidate_order_ancestor_steps.path: count=80352
  state_read_lock_wait: count=2269 total_ns=41789 avg_ns=18 max_ns=152
  state_read_lock_hold: count=2269 total_ns=178869 avg_ns=78 max_ns=2287
  state_write_lock_wait: count=1875 total_ns=32385 avg_ns=17 max_ns=99
  state_write_lock_hold: count=1875 total_ns=1015673 avg_ns=541 max_ns=29482
  open_confined_openat2: count=3180 total_ns=1503473 avg_ns=472 max_ns=8725
  stat_child_no_follow: count=3101 total_ns=26754457 avg_ns=8627 max_ns=49286
  source_root_path: count=3153 total_ns=3884240 avg_ns=1231 max_ns=27653
  resolved_virtual_path: count=8546 total_ns=18964968 avg_ns=2219 max_ns=32150
  resolved_virtual_path_from_path: count=5367 total_ns=16104790 avg_ns=3000 max_ns=32150
  resolved_virtual_path_from_path_component_walk: count=5367 total_ns=14213906 avg_ns=2648 max_ns=31711
  resolved_virtual_path_from_path_canonicalize: count=12780 total_ns=11598493 avg_ns=907 max_ns=30953
  resolved_virtual_path_from_path_source_root_confinement: count=12780 total_ns=1420722 avg_ns=111 max_ns=953
  resolved_virtual_path_from_path_virtual_conversion: count=5367 total_ns=1634407 avg_ns=304 max_ns=3221
  resolved_virtual_path_from_open_fd: count=3179 total_ns=2860178 avg_ns=899 max_ns=15277
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=156425 avg_ns=6016 max_ns=20477
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6684 avg_ns=257 max_ns=628
  readdirplus_directory_scan: count=26 total_ns=4535830 avg_ns=174455 max_ns=261903
  readdirplus_attr_generation_scan: count=858 total_ns=8464778 avg_ns=9865 max_ns=28172
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=114451 avg_ns=4401 max_ns=9682
  readdirplus_page_commit: count=26 total_ns=394575 avg_ns=15175 max_ns=29577
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
