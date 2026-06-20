# ScreenFS benchmark result

- timestamp: `2026-06-20T15:43:24.948846+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set matcher-descendant-directory --matcher-extra-rules 32 --iterations 3 --warmups 1 --dir-entries 200 --output-json docs/artifacts/current-matcher-descendant-directory-smoke.json --output-md docs/artifacts/current-matcher-descendant-directory-smoke.md --output-svg docs/artifacts/current-matcher-descendant-directory-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+3 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| matcher_descendant_readdir | 0.000024 | 0.001105 | 45.936 | 0.001117 | 0.001118 | 0.001119 |
| matcher_descendant_readdirplus | 0.000052 | 0.005177 | 100.298 | 0.005274 | 0.005286 | 0.005296 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=79460 avg_ns=79460 max_ns=79460
  fuse_op.getattr: count=137 total_ns=3602351 avg_ns=26294 max_ns=54651
  fuse_op.lookup: count=541 total_ns=9847939 avg_ns=18203 max_ns=80450
  fuse_op.opendir: count=8 total_ns=215416 avg_ns=26927 max_ns=30549
  fuse_op.readdir: count=8 total_ns=212543 avg_ns=26567 max_ns=43640
  fuse_op.readdirplus: count=8 total_ns=7142487 avg_ns=892810 max_ns=994695
  fuse_op.releasedir: count=8 total_ns=21285 avg_ns=2660 max_ns=3780
  fuse_op.statfs: count=2 total_ns=5335 avg_ns=2667 max_ns=3409
  policy_decision: count=2198 total_ns=3737656 avg_ns=1700 max_ns=4667
  matcher_candidates: count=47768
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=47768
  matcher_candidate_order.descendant: count=2198 total_ns=1104651 avg_ns=502 max_ns=62452
  matcher_candidate_order.path: count=6594 total_ns=2813751 avg_ns=426 max_ns=17533
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=215404
  matcher_candidate_order_seen_slots.descendant: count=70336
  matcher_candidate_order_seen_slots.path: count=145068
  matcher_candidate_order_ancestor_steps: count=33048
  matcher_candidate_order_ancestor_steps.descendant: count=8262
  matcher_candidate_order_ancestor_steps.path: count=24786
  state_read_lock_wait: count=703 total_ns=16937 avg_ns=24 max_ns=99
  state_read_lock_hold: count=703 total_ns=53554 avg_ns=76 max_ns=2320
  state_write_lock_wait: count=579 total_ns=14049 avg_ns=24 max_ns=209
  state_write_lock_hold: count=579 total_ns=339065 avg_ns=585 max_ns=38133
  open_confined_openat2: count=984 total_ns=692583 avg_ns=703 max_ns=17926
  stat_child_no_follow: count=959 total_ns=10772119 avg_ns=11232 max_ns=70928
  source_root_path: count=975 total_ns=1776097 avg_ns=1821 max_ns=27750
  resolved_virtual_path: count=2642 total_ns=7337727 avg_ns=2777 max_ns=39402
  resolved_virtual_path_from_path: count=1659 total_ns=6139410 avg_ns=3700 max_ns=39402
  resolved_virtual_path_from_path_component_walk: count=1659 total_ns=5448688 avg_ns=3284 max_ns=38951
  resolved_virtual_path_from_path_canonicalize: count=3942 total_ns=4453590 avg_ns=1129 max_ns=14799
  resolved_virtual_path_from_path_source_root_confinement: count=3942 total_ns=481279 avg_ns=122 max_ns=900
  resolved_virtual_path_from_path_virtual_conversion: count=1659 total_ns=591632 avg_ns=356 max_ns=6099
  resolved_virtual_path_from_open_fd: count=983 total_ns=1198317 avg_ns=1219 max_ns=8855
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=8 total_ns=95688 avg_ns=11961 max_ns=26904
  readdir_attr_generation_scan: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=8 total_ns=4374 avg_ns=546 max_ns=1317
  readdirplus_directory_scan: count=8 total_ns=1684818 avg_ns=210602 max_ns=239130
  readdirplus_attr_generation_scan: count=264 total_ns=3402711 avg_ns=12889 max_ns=19299
  readdirplus_attr_generation_entries: count=256
  readdirplus_symlink_visibility: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=8 total_ns=53691 avg_ns=6711 max_ns=13497
  readdirplus_page_commit: count=8 total_ns=159216 avg_ns=19902 max_ns=38276
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
