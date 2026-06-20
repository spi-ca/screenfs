# ScreenFS benchmark result

- timestamp: `2026-06-20T07:57:48.720332+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set policy-heavy-matrix --matcher-extra-rules 32 --iterations 3 --warmups 1 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/current-policy-heavy-matrix-smoke.json --output-md docs/artifacts/current-policy-heavy-matrix-smoke.md --output-svg docs/artifacts/current-policy-heavy-matrix-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+27 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+27 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.000652 | 0.011200 | 17.167 | 0.011350 | 0.011369 | 0.011384 |
| metadata_getattr | 0.000694 | 0.013756 | 19.810 | 0.014105 | 0.014148 | 0.014183 |
| metadata_access | 0.000599 | 0.014577 | 24.343 | 0.015144 | 0.015215 | 0.015271 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.010294 | 0.010695 | 0.010745 | 0.010785 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=15385400 avg_ns=19207 max_ns=39920
  fuse_op.getattr: count=801 total_ns=11642560 avg_ns=14535 max_ns=61738
  fuse_op.lookup: count=9605 total_ns=103464341 avg_ns=10771 max_ns=275660
  fuse_op.statfs: count=2 total_ns=2014 avg_ns=1007 max_ns=1149
  policy_decision: count=23214 total_ns=12011594 avg_ns=517 max_ns=49538
  matcher_candidates: count=800
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=800
  matcher_candidate_order.descendant: count=23214 total_ns=2153424 avg_ns=92 max_ns=1057
  matcher_candidate_order.path: count=69642 total_ns=13359238 avg_ns=191 max_ns=5352
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=766062
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=766062
  matcher_candidate_order_ancestor_steps: count=249720
  matcher_candidate_order_ancestor_steps.descendant: count=62430
  matcher_candidate_order_ancestor_steps.path: count=187290
  state_read_lock_wait: count=11207 total_ns=196522 avg_ns=17 max_ns=184
  state_read_lock_hold: count=11207 total_ns=786589 avg_ns=70 max_ns=1069
  state_write_lock_wait: count=8003 total_ns=134251 avg_ns=16 max_ns=49
  state_write_lock_hold: count=8003 total_ns=2268376 avg_ns=283 max_ns=10635
  open_confined_openat2: count=12008 total_ns=4912910 avg_ns=409 max_ns=8302
  stat_child_no_follow: count=11207 total_ns=68262859 avg_ns=6091 max_ns=132201
  source_root_path: count=12007 total_ns=14718391 avg_ns=1225 max_ns=44482
  resolved_virtual_path: count=32818 total_ns=47984485 avg_ns=1462 max_ns=51374
  resolved_virtual_path_from_path: count=20811 total_ns=38591586 avg_ns=1854 max_ns=51374
  resolved_virtual_path_from_path_component_walk: count=20811 total_ns=32250839 avg_ns=1549 max_ns=50986
  resolved_virtual_path_from_path_canonicalize: count=32014 total_ns=25279047 avg_ns=789 max_ns=50254
  resolved_virtual_path_from_path_source_root_confinement: count=32014 total_ns=4171284 avg_ns=130 max_ns=35375
  resolved_virtual_path_from_path_virtual_conversion: count=20811 total_ns=5319406 avg_ns=255 max_ns=2173
  resolved_virtual_path_from_open_fd: count=12007 total_ns=9392899 avg_ns=782 max_ns=30819
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
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
