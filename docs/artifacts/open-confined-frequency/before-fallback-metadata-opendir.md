# ScreenFS benchmark result

- timestamp: `2026-06-20T12:41:08.250663+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.json --output-md docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.md --output-svg docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+9 more)`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_opendir | 0.001280 | 0.038814 | 30.331 | 0.039871 | 0.040179 | 0.040426 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61915 avg_ns=61915 max_ns=61915
  fuse_op.getattr: count=6657 total_ns=80955911 avg_ns=12161 max_ns=54558
  fuse_op.lookup: count=13317 total_ns=147310815 avg_ns=11061 max_ns=70451
  fuse_op.opendir: count=6656 total_ns=107911095 avg_ns=16212 max_ns=70837
  fuse_op.releasedir: count=6656 total_ns=2187447 avg_ns=328 max_ns=3369
  fuse_op.statfs: count=2 total_ns=7421 avg_ns=3710 max_ns=4893
  policy_decision: count=59918 total_ns=19543827 avg_ns=326 max_ns=7538
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=5955296 avg_ns=99 max_ns=8069
  matcher_candidate_order.path: count=179754 total_ns=22911276 avg_ns=127 max_ns=9275
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=559224
  matcher_candidate_order_ancestor_steps.descendant: count=139806
  matcher_candidate_order_ancestor_steps.path: count=419418
  state_read_lock_wait: count=26631 total_ns=660602 avg_ns=24 max_ns=408
  state_read_lock_hold: count=26631 total_ns=1597621 avg_ns=59 max_ns=1176
  state_write_lock_wait: count=26627 total_ns=626093 avg_ns=23 max_ns=568
  state_write_lock_hold: count=26627 total_ns=4600774 avg_ns=172 max_ns=53339
  open_confined_openat2: count=33288 total_ns=21589058 avg_ns=648 max_ns=11601
  stat_child_no_follow: count=26631 total_ns=186528681 avg_ns=7004 max_ns=56790
  source_root_path: count=26631 total_ns=50300139 avg_ns=1888 max_ns=35680
  resolved_virtual_path: count=86546 total_ns=134126543 avg_ns=1549 max_ns=19161
  resolved_virtual_path_from_path: count=53259 total_ns=98623219 avg_ns=1851 max_ns=13666
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=80926103 avg_ns=1519 max_ns=13225
  resolved_virtual_path_from_path_canonicalize: count=66574 total_ns=64143921 avg_ns=963 max_ns=12740
  resolved_virtual_path_from_path_source_root_confinement: count=66574 total_ns=8146448 avg_ns=122 max_ns=4574
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=14516952 avg_ns=272 max_ns=7801
  resolved_virtual_path_from_open_fd: count=33287 total_ns=35503324 avg_ns=1066 max_ns=19161
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
