# ScreenFS benchmark result

- timestamp: `2026-06-20T16:06:19.898477+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin /tmp/screenfs-matcher-before-920c00e/target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set policy-heavy-matrix --matcher-extra-rules 32 --policy-label matcher32 --iterations 10 --warmups 3 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/matcher-descendant-combined-probe/before-920c00e-matcher32-policy-heavy-matrix.json --output-md docs/artifacts/matcher-descendant-combined-probe/before-920c00e-matcher32-policy-heavy-matrix.md --output-svg docs/artifacts/matcher-descendant-combined-probe/before-920c00e-matcher32-policy-heavy-matrix.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.json;  M docs/artifacts/current-policy-heavy-matrix-smoke.md;  M docs/artifacts/current-policy-heavy-matrix-smoke.svg;  M docs/benchmarks.md; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-before-920c00e/target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/tmp/screenfs-matcher-before-920c00e`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `True`
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
| metadata_lookup | 0.001110 | 0.014344 | 12.927 | 0.015324 | 0.015459 | 0.015567 |
| metadata_getattr | 0.001267 | 0.020117 | 15.876 | 0.021671 | 0.022334 | 0.022865 |
| metadata_access | 0.001069 | 0.021255 | 19.891 | 0.022036 | 0.022193 | 0.022319 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.013517 | 0.014803 | 0.015096 | 0.015330 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=75750768 avg_ns=29123 max_ns=69781
  fuse_op.getattr: count=2601 total_ns=56491573 avg_ns=21719 max_ns=49854
  fuse_op.lookup: count=31205 total_ns=491647574 avg_ns=15755 max_ns=295496
  fuse_op.statfs: count=2 total_ns=8511 avg_ns=4255 max_ns=6651
  policy_decision: count=75414 total_ns=75781033 avg_ns=1004 max_ns=177601
  matcher_candidates: count=1167624
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1167624
  matcher_candidate_order.descendant: count=75414 total_ns=34207412 avg_ns=453 max_ns=46080
  matcher_candidate_order.path: count=226242 total_ns=82925686 avg_ns=366 max_ns=17358
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7390572
  matcher_candidate_order_seen_slots.descendant: count=2413248
  matcher_candidate_order_seen_slots.path: count=4977324
  matcher_candidate_order_ancestor_steps: count=811320
  matcher_candidate_order_ancestor_steps.descendant: count=202830
  matcher_candidate_order_ancestor_steps.path: count=608490
  state_read_lock_wait: count=36407 total_ns=882942 avg_ns=24 max_ns=510
  state_read_lock_hold: count=36407 total_ns=2476319 avg_ns=68 max_ns=1610
  state_write_lock_wait: count=26003 total_ns=601836 avg_ns=23 max_ns=434
  state_write_lock_hold: count=26003 total_ns=8636043 avg_ns=332 max_ns=29163
  open_confined_openat2: count=39008 total_ns=25321165 avg_ns=649 max_ns=269230
  stat_child_no_follow: count=36407 total_ns=329560312 avg_ns=9052 max_ns=282983
  source_root_path: count=36407 total_ns=68017953 avg_ns=1868 max_ns=27535
  resolved_virtual_path: count=106618 total_ns=199770328 avg_ns=1873 max_ns=48599
  resolved_virtual_path_from_path: count=67611 total_ns=157260772 avg_ns=2325 max_ns=33117
  resolved_virtual_path_from_path_component_walk: count=67611 total_ns=133843868 avg_ns=1979 max_ns=32460
  resolved_virtual_path_from_path_canonicalize: count=104014 total_ns=108253386 avg_ns=1040 max_ns=32054
  resolved_virtual_path_from_path_source_root_confinement: count=104014 total_ns=12457139 avg_ns=119 max_ns=10468
  resolved_virtual_path_from_path_virtual_conversion: count=67611 total_ns=19311229 avg_ns=285 max_ns=6859
  resolved_virtual_path_from_open_fd: count=39007 total_ns=42509556 avg_ns=1089 max_ns=48599
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
