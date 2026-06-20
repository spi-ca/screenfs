# ScreenFS benchmark result

- timestamp: `2026-06-20T15:43:08.504833+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set policy-heavy-matrix --matcher-extra-rules 32 --iterations 3 --warmups 1 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/current-policy-heavy-matrix-smoke.json --output-md docs/artifacts/current-policy-heavy-matrix-smoke.md --output-svg docs/artifacts/current-policy-heavy-matrix-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.001218 | 0.013979 | 11.480 | 0.014338 | 0.014383 | 0.014418 |
| metadata_getattr | 0.001268 | 0.019368 | 15.275 | 0.019543 | 0.019565 | 0.019582 |
| metadata_access | 0.001082 | 0.022486 | 20.782 | 0.022918 | 0.022972 | 0.023015 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.012973 | 0.012985 | 0.012987 | 0.012988 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=23437994 avg_ns=29260 max_ns=53250
  fuse_op.getattr: count=801 total_ns=17121277 avg_ns=21374 max_ns=40510
  fuse_op.lookup: count=9605 total_ns=149004507 avg_ns=15513 max_ns=157502
  fuse_op.statfs: count=2 total_ns=4517 avg_ns=2258 max_ns=2377
  policy_decision: count=23214 total_ns=23221422 avg_ns=1000 max_ns=11953
  matcher_candidates: count=359424
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=359424
  matcher_candidate_order.descendant: count=23214 total_ns=10331956 avg_ns=445 max_ns=5758
  matcher_candidate_order.path: count=69642 total_ns=25181131 avg_ns=361 max_ns=8650
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2274972
  matcher_candidate_order_seen_slots.descendant: count=742848
  matcher_candidate_order_seen_slots.path: count=1532124
  matcher_candidate_order_ancestor_steps: count=249720
  matcher_candidate_order_ancestor_steps.descendant: count=62430
  matcher_candidate_order_ancestor_steps.path: count=187290
  state_read_lock_wait: count=11207 total_ns=268237 avg_ns=23 max_ns=482
  state_read_lock_hold: count=11207 total_ns=742893 avg_ns=66 max_ns=10101
  state_write_lock_wait: count=8003 total_ns=184679 avg_ns=23 max_ns=264
  state_write_lock_hold: count=8003 total_ns=2681951 avg_ns=335 max_ns=7374
  open_confined_openat2: count=12008 total_ns=7511615 avg_ns=625 max_ns=8453
  stat_child_no_follow: count=11207 total_ns=99805342 avg_ns=8905 max_ns=52447
  source_root_path: count=11207 total_ns=20861634 avg_ns=1861 max_ns=31444
  resolved_virtual_path: count=32818 total_ns=61723333 avg_ns=1880 max_ns=18466
  resolved_virtual_path_from_path: count=20811 total_ns=48317757 avg_ns=2321 max_ns=11737
  resolved_virtual_path_from_path_component_walk: count=20811 total_ns=41176372 avg_ns=1978 max_ns=11253
  resolved_virtual_path_from_path_canonicalize: count=32014 total_ns=33246847 avg_ns=1038 max_ns=10473
  resolved_virtual_path_from_path_source_root_confinement: count=32014 total_ns=3899875 avg_ns=121 max_ns=4636
  resolved_virtual_path_from_path_virtual_conversion: count=20811 total_ns=5897069 avg_ns=283 max_ns=3709
  resolved_virtual_path_from_open_fd: count=12007 total_ns=13405576 avg_ns=1116 max_ns=18466
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
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
