# ScreenFS benchmark result

- timestamp: `2026-06-15T09:18:56.817392+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --iterations 10 --warmups 3 --perf-counters --small-files 1024 --dir-entries 2048 --metadata-ops 1024 --sync-ops 64 --hidden-misses 512 --matcher-misses 512 --symlink-parent-mutations 64 --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher32 --matcher-extra-rules 32 --workload-set policy-heavy-matrix --output-json docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json --output-md docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.md --output-svg docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.007180 | 0.088711 | 12.356 | 0.089346 | 0.089729 | 0.090036 |
| metadata_getattr | 0.007701 | 0.115882 | 15.048 | 0.116279 | 0.116572 | 0.116807 |
| metadata_access | 0.006728 | 0.120268 | 17.877 | 0.124783 | 0.126365 | 0.127631 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.036375 | 0.039241 | 0.039873 | 0.040378 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=327490781 avg_ns=24599 max_ns=98319
  fuse_op.getattr: count=13313 total_ns=311113799 avg_ns=23369 max_ns=84271
  fuse_op.lookup: count=139781 total_ns=2699089694 avg_ns=19309 max_ns=138062
  fuse_op.statfs: count=2 total_ns=6878 avg_ns=3439 max_ns=4579
  policy_decision: count=492564 total_ns=310988954 avg_ns=631 max_ns=13433
  matcher_candidates: count=6656
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=6656
  matcher_candidate_order.descendant: count=492564 total_ns=54278420 avg_ns=110 max_ns=10821
  matcher_candidate_order.path: count=1477692 total_ns=343055231 avg_ns=232 max_ns=24511
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=16254612
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=16254612
  matcher_candidate_order_ancestor_steps: count=4952228
  matcher_candidate_order_ancestor_steps.descendant: count=1238057
  matcher_candidate_order_ancestor_steps.path: count=3714171
  state_read_lock_wait: count=166407 total_ns=4067352 avg_ns=24 max_ns=4510
  state_read_lock_hold: count=166407 total_ns=11688191 avg_ns=70 max_ns=9565
  state_write_lock_wait: count=119811 total_ns=2876877 avg_ns=24 max_ns=789
  state_write_lock_hold: count=119811 total_ns=55886533 avg_ns=466 max_ns=50113
  open_confined_openat2: count=326158 total_ns=180721397 avg_ns=554 max_ns=22573
  source_root_path: count=485908 total_ns=781125099 avg_ns=1607 max_ns=70839
  resolved_virtual_path: count=472594 total_ns=792071900 avg_ns=1676 max_ns=51025
  resolved_virtual_path_from_path: count=146437 total_ns=452832450 avg_ns=3092 max_ns=51025
  resolved_virtual_path_from_path_component_walk: count=146437 total_ns=389828593 avg_ns=2662 max_ns=50487
  resolved_virtual_path_from_path_canonicalize: count=299529 total_ns=317149917 avg_ns=1058 max_ns=49687
  resolved_virtual_path_from_path_source_root_confinement: count=299529 total_ns=35230929 avg_ns=117 max_ns=7942
  resolved_virtual_path_from_path_virtual_conversion: count=146437 total_ns=53217425 avg_ns=363 max_ns=16105
  resolved_virtual_path_from_open_fd: count=326157 total_ns=339239450 avg_ns=1040 max_ns=24370
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
