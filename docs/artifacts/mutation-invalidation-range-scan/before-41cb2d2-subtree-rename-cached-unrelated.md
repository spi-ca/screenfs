# ScreenFS benchmark result

- timestamp: `2026-06-20T07:10:42.965130+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-before-subtree-kpgIBU/target/release/screenfs --screenfs-source-root /tmp/screenfs-before-subtree-kpgIBU --perf-counters --workload subtree_rename_cached_unrelated --iterations 5 --warmups 1 --small-files 500 --output-json docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-subtree-rename-cached-unrelated.json --output-md docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-subtree-rename-cached-unrelated.md --output-svg docs/artifacts/mutation-invalidation-range-scan/before-41cb2d2-subtree-rename-cached-unrelated.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+14 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-before-subtree-kpgIBU/target/release/screenfs`
- screenfs_bin_sha256: `a2d35ad0912b5f2a94d8b53c9f20ce38a984dc230c9f5510eadc14fcf928f0ec`
- screenfs_source_root: `/tmp/screenfs-before-subtree-kpgIBU`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `subtree_rename_cached_unrelated`
- iterations: `5`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| subtree_rename_cached_unrelated | 0.049884 | 0.050535 | 0.050548 | 0.050559 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61728 avg_ns=61728 max_ns=61728
  fuse_op.getattr: count=3037 total_ns=53103468 avg_ns=17485 max_ns=88852
  fuse_op.lookup: count=12250 total_ns=159013179 avg_ns=12980 max_ns=267623
  fuse_op.rename: count=12 total_ns=3065590 avg_ns=255465 max_ns=287692
  fuse_op.statfs: count=2 total_ns=4627 avg_ns=2313 max_ns=3321
  policy_decision: count=30912 total_ns=13227610 avg_ns=427 max_ns=15086
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=30840 total_ns=3845457 avg_ns=124 max_ns=534
  matcher_candidate_order.path: count=92664 total_ns=15004453 avg_ns=161 max_ns=33692
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=30912
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=30912
  matcher_candidate_order_ancestor_steps: count=406388
  matcher_candidate_order_ancestor_steps.descendant: count=101477
  matcher_candidate_order_ancestor_steps.path: count=304911
  state_read_lock_wait: count=15312 total_ns=285780 avg_ns=18 max_ns=277
  state_read_lock_hold: count=15312 total_ns=1028728 avg_ns=67 max_ns=20307
  state_write_lock_wait: count=12236 total_ns=216633 avg_ns=17 max_ns=307
  state_write_lock_hold: count=12236 total_ns=6974795 avg_ns=570 max_ns=151445
  open_confined_openat2: count=15409 total_ns=7492398 avg_ns=486 max_ns=46513
  stat_child_no_follow: count=15384 total_ns=112800040 avg_ns=7332 max_ns=79540
  source_root_path: count=15420 total_ns=19904118 avg_ns=1290 max_ns=30637
  resolved_virtual_path: count=46125 total_ns=106485179 avg_ns=2308 max_ns=260280
  resolved_virtual_path_from_path: count=30717 total_ns=93146333 avg_ns=3032 max_ns=260280
  resolved_virtual_path_from_path_component_walk: count=30717 total_ns=82118387 avg_ns=2673 max_ns=259890
  resolved_virtual_path_from_path_canonicalize: count=70299 total_ns=65905384 avg_ns=937 max_ns=259387
  resolved_virtual_path_from_path_source_root_confinement: count=70299 total_ns=9620147 avg_ns=136 max_ns=40054
  resolved_virtual_path_from_path_virtual_conversion: count=30717 total_ns=9492160 avg_ns=309 max_ns=63993
  resolved_virtual_path_from_open_fd: count=15408 total_ns=13338846 avg_ns=865 max_ns=69762
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
  invalidations: count=12 invalidated_entries=24 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
