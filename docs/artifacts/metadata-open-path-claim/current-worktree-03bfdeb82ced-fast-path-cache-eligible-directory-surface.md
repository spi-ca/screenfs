# ScreenFS benchmark result

- timestamp: `2026-06-15T09:18:50.855559+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --iterations 10 --warmups 3 --perf-counters --small-files 1024 --dir-entries 2048 --metadata-ops 1024 --sync-ops 64 --hidden-misses 512 --matcher-misses 512 --symlink-parent-mutations 64 --policy-preset fast-path-cache-eligible --workload-set directory-surface --output-json docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json --output-md docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.md --output-svg docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+83 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `1b9b5de4a9f56ef6ff65a27c0a01f690a922ab5d9e75de913edf39db4ee4f94d`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+83 more)`
- screenfs_source_git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.000470 | 0.022245 | 47.342 | 0.023409 | 0.023619 | 0.023787 |
| readdirplus_basic | 0.002921 | 0.174408 | 59.704 | 0.178899 | 0.179682 | 0.180309 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73119 avg_ns=73119 max_ns=73119
  fuse_op.getattr: count=26651 total_ns=367780174 avg_ns=13799 max_ns=66384
  fuse_op.lookup: count=79929 total_ns=1059594569 avg_ns=13256 max_ns=165332
  fuse_op.opendir: count=26 total_ns=507821 avg_ns=19531 max_ns=42989
  fuse_op.readdir: count=103 total_ns=377760460 avg_ns=3667577 max_ns=9924428
  fuse_op.readdirplus: count=28 total_ns=185915500 avg_ns=6639839 max_ns=8224181
  fuse_op.releasedir: count=26 total_ns=7862327 avg_ns=302397 max_ns=474697
  fuse_op.statfs: count=2 total_ns=9652 avg_ns=4826 max_ns=6225
  policy_decision: count=454809 total_ns=162499917 avg_ns=357 max_ns=17539
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=454809 total_ns=59328211 avg_ns=130 max_ns=27404
  matcher_candidate_order.path: count=1364427 total_ns=181412622 avg_ns=132 max_ns=17609
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5462192
  matcher_candidate_order_ancestor_steps.descendant: count=1365548
  matcher_candidate_order_ancestor_steps.path: count=4096644
  state_read_lock_wait: count=106738 total_ns=2594024 avg_ns=24 max_ns=19223
  state_read_lock_hold: count=106738 total_ns=7626457 avg_ns=71 max_ns=5233
  state_write_lock_wait: count=80136 total_ns=1915680 avg_ns=23 max_ns=6529
  state_write_lock_hold: count=80136 total_ns=115079635 avg_ns=1436 max_ns=2768472
  open_confined_openat2: count=213449 total_ns=117055596 avg_ns=548 max_ns=58993
  source_root_path: count=213448 total_ns=347000449 avg_ns=1625 max_ns=46030
  resolved_virtual_path: count=213448 total_ns=226349015 avg_ns=1060 max_ns=57025
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=213448 total_ns=226349015 avg_ns=1060 max_ns=57025
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=103 total_ns=309518415 avg_ns=3005033 max_ns=7240469
  readdir_attr_generation_scan: count=103 total_ns=79711782 avg_ns=773900 max_ns=2044478
  readdir_attr_generation_entries: count=80396
  readdir_symlink_visibility: count=103 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=103 total_ns=16811685 avg_ns=163220 max_ns=597915
  readdir_page_commit: count=103 total_ns=56370875 avg_ns=547290 max_ns=2768811
  readdirplus_directory_scan: count=28 total_ns=178782524 avg_ns=6385090 max_ns=7936302
  readdirplus_attr_generation_scan: count=28 total_ns=49580123 avg_ns=1770718 max_ns=2223700
  readdirplus_attr_generation_entries: count=54306
  readdirplus_symlink_visibility: count=28 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=28 total_ns=11131066 avg_ns=397538 max_ns=597854
  readdirplus_page_commit: count=28 total_ns=5127485 avg_ns=183124 max_ns=413965
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
