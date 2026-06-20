# ScreenFS benchmark result

- timestamp: `2026-06-20T11:05:06.522038+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.json --output-md docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0bc1d6847824d1577f2afe89dbff9b1b480b45217490d51ec480d1e9a6151e40`
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
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.027695 | 2.355126 | 85.037 | 2.389402 | 2.393601 | 2.396959 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=63401 avg_ns=63401 max_ns=63401
  fuse_op.getattr: count=140008 total_ns=2326944384 avg_ns=16620 max_ns=2038827
  fuse_op.lookup: count=420019 total_ns=5773674772 avg_ns=13746 max_ns=111990
  fuse_op.opendir: count=7 total_ns=171905 avg_ns=24557 max_ns=57925
  fuse_op.readdir: count=176 total_ns=4959447239 avg_ns=28178677 max_ns=54616287
  fuse_op.readdirplus: count=27 total_ns=954248167 avg_ns=35342524 max_ns=55953864
  fuse_op.releasedir: count=7 total_ns=20465525 avg_ns=2923646 max_ns=3260692
  fuse_op.statfs: count=2 total_ns=7199 avg_ns=3599 max_ns=5094
  policy_decision: count=3158180 total_ns=1743873610 avg_ns=552 max_ns=55899
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3158180 total_ns=493494305 avg_ns=156 max_ns=45879
  matcher_candidate_order.path: count=9474540 total_ns=1882411916 avg_ns=198 max_ns=61124
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3158180
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3158180
  matcher_candidate_order_ancestor_steps: count=44908992
  matcher_candidate_order_ancestor_steps.descendant: count=11227248
  matcher_candidate_order_ancestor_steps.path: count=33681744
  state_read_lock_wait: count=560238 total_ns=13284132 avg_ns=23 max_ns=1133
  state_read_lock_hold: count=560238 total_ns=39175974 avg_ns=69 max_ns=15344
  state_write_lock_wait: count=420241 total_ns=9674031 avg_ns=23 max_ns=6598
  state_write_lock_hold: count=420241 total_ns=560819290 avg_ns=1334 max_ns=3260273
  open_confined_openat2: count=565342 total_ns=341566255 avg_ns=604 max_ns=28901
  stat_child_no_follow: count=565131 total_ns=4587494879 avg_ns=8117 max_ns=80566
  source_root_path: count=565334 total_ns=1012969033 avg_ns=1791 max_ns=53623
  resolved_virtual_path: count=1690518 total_ns=3679724524 avg_ns=2176 max_ns=2025328
  resolved_virtual_path_from_path: count=1125177 total_ns=3015789029 avg_ns=2680 max_ns=2025328
  resolved_virtual_path_from_path_component_walk: count=1125177 total_ns=2581121983 avg_ns=2293 max_ns=2022753
  resolved_virtual_path_from_path_canonicalize: count=1970276 total_ns=2098606833 avg_ns=1065 max_ns=2019541
  resolved_virtual_path_from_path_source_root_confinement: count=1970276 total_ns=237468779 avg_ns=120 max_ns=37364
  resolved_virtual_path_from_path_virtual_conversion: count=1125177 total_ns=368288775 avg_ns=327 max_ns=30615
  resolved_virtual_path_from_open_fd: count=565341 total_ns=663935495 avg_ns=1174 max_ns=31917
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=176 total_ns=4702167723 avg_ns=26716862 max_ns=53528641
  readdir_attr_generation_scan: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=176 total_ns=333813231 avg_ns=1896666 max_ns=5045405
  readdir_page_commit: count=176 total_ns=216985780 avg_ns=1232873 max_ns=3224183
  readdirplus_directory_scan: count=27 total_ns=873491363 avg_ns=32351531 max_ns=53208728
  readdirplus_attr_generation_scan: count=5109 total_ns=51203451 avg_ns=10022 max_ns=81069
  readdirplus_attr_generation_entries: count=5082
  readdirplus_symlink_visibility: count=27 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=27 total_ns=52057191 avg_ns=1928044 max_ns=3487016
  readdirplus_page_commit: count=27 total_ns=16454664 avg_ns=609432 max_ns=1492202
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
