# ScreenFS benchmark result

- timestamp: `2026-06-20T16:07:11.674076+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set policy-heavy-matrix --matcher-extra-rules 32 --policy-label matcher32 --iterations 10 --warmups 3 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/matcher-descendant-combined-probe/after-worktree-matcher32-policy-heavy-matrix.json --output-md docs/artifacts/matcher-descendant-combined-probe/after-worktree-matcher32-policy-heavy-matrix.md --output-svg docs/artifacts/matcher-descendant-combined-probe/after-worktree-matcher32-policy-heavy-matrix.svg`
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
- policy_label: `matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.001182 | 0.015280 | 12.923 | 0.016122 | 0.016190 | 0.016245 |
| metadata_getattr | 0.001294 | 0.020449 | 15.797 | 0.021387 | 0.021554 | 0.021688 |
| metadata_access | 0.001103 | 0.022097 | 20.035 | 0.022325 | 0.022544 | 0.022719 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.013802 | 0.015899 | 0.016438 | 0.016869 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=78873482 avg_ns=30324 max_ns=61277
  fuse_op.getattr: count=2601 total_ns=57939923 avg_ns=22276 max_ns=63324
  fuse_op.lookup: count=31205 total_ns=507523408 avg_ns=16264 max_ns=120481
  fuse_op.statfs: count=2 total_ns=4373 avg_ns=2186 max_ns=2638
  policy_decision: count=75414 total_ns=102059636 avg_ns=1353 max_ns=11115
  matcher_candidates: count=1167624
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1167624
  matcher_candidate_order.descendant: count=75414 total_ns=34379238 avg_ns=455 max_ns=8191
  matcher_candidate_order.path: count=226242 total_ns=81440688 avg_ns=359 max_ns=21359
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7390572
  matcher_candidate_order_seen_slots.descendant: count=2413248
  matcher_candidate_order_seen_slots.path: count=4977324
  matcher_candidate_order_ancestor_steps: count=811320
  matcher_candidate_order_ancestor_steps.descendant: count=202830
  matcher_candidate_order_ancestor_steps.path: count=608490
  state_read_lock_wait: count=36407 total_ns=888054 avg_ns=24 max_ns=2706
  state_read_lock_hold: count=36407 total_ns=2571537 avg_ns=70 max_ns=8107
  state_write_lock_wait: count=26003 total_ns=596937 avg_ns=22 max_ns=524
  state_write_lock_hold: count=26003 total_ns=9003702 avg_ns=346 max_ns=24817
  open_confined_openat2: count=39008 total_ns=24718375 avg_ns=633 max_ns=41467
  stat_child_no_follow: count=36407 total_ns=339906396 avg_ns=9336 max_ns=111822
  source_root_path: count=36407 total_ns=68339473 avg_ns=1877 max_ns=67088
  resolved_virtual_path: count=106618 total_ns=197822162 avg_ns=1855 max_ns=47878
  resolved_virtual_path_from_path: count=67611 total_ns=156428429 avg_ns=2313 max_ns=47878
  resolved_virtual_path_from_path_component_walk: count=67611 total_ns=133368828 avg_ns=1972 max_ns=47512
  resolved_virtual_path_from_path_canonicalize: count=104014 total_ns=107435783 avg_ns=1032 max_ns=47056
  resolved_virtual_path_from_path_source_root_confinement: count=104014 total_ns=12589733 avg_ns=121 max_ns=5911
  resolved_virtual_path_from_path_virtual_conversion: count=67611 total_ns=18928170 avg_ns=279 max_ns=5245
  resolved_virtual_path_from_open_fd: count=39007 total_ns=41393733 avg_ns=1061 max_ns=21363
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
