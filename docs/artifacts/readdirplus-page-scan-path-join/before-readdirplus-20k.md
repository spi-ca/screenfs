# ScreenFS benchmark result

- timestamp: `2026-06-20T11:00:46.962475+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.json --output-md docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+6 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+6 more)`
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
| readdirplus_basic | 0.023487 | 2.272759 | 96.765 | 2.440164 | 2.449740 | 2.457400 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=86014 avg_ns=86014 max_ns=86014
  fuse_op.getattr: count=140008 total_ns=1939172127 avg_ns=13850 max_ns=383928
  fuse_op.lookup: count=420019 total_ns=4747705055 avg_ns=11303 max_ns=330055
  fuse_op.opendir: count=7 total_ns=165736 avg_ns=23676 max_ns=71325
  fuse_op.readdir: count=176 total_ns=6178075893 avg_ns=35102703 max_ns=89515310
  fuse_op.readdirplus: count=27 total_ns=1167339461 avg_ns=43234794 max_ns=72753002
  fuse_op.releasedir: count=7 total_ns=13198696 avg_ns=1885528 max_ns=2875793
  fuse_op.statfs: count=2 total_ns=7298 avg_ns=3649 max_ns=5111
  policy_decision: count=3158180 total_ns=1902356850 avg_ns=602 max_ns=379486
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3158180 total_ns=548513928 avg_ns=173 max_ns=268609
  matcher_candidate_order.path: count=9474540 total_ns=2053686643 avg_ns=216 max_ns=395027
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3158180
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3158180
  matcher_candidate_order_ancestor_steps: count=44908992
  matcher_candidate_order_ancestor_steps.descendant: count=11227248
  matcher_candidate_order_ancestor_steps.path: count=33681744
  state_read_lock_wait: count=560238 total_ns=15555920 avg_ns=27 max_ns=9953
  state_read_lock_hold: count=560238 total_ns=34102970 avg_ns=60 max_ns=11441
  state_write_lock_wait: count=420241 total_ns=7766562 avg_ns=18 max_ns=8731
  state_write_lock_hold: count=420241 total_ns=551114659 avg_ns=1311 max_ns=5983130
  open_confined_openat2: count=565342 total_ns=261473269 avg_ns=462 max_ns=68602
  stat_child_no_follow: count=565131 total_ns=3700867919 avg_ns=6548 max_ns=378190
  source_root_path: count=565334 total_ns=781911440 avg_ns=1383 max_ns=195610
  resolved_virtual_path: count=1690518 total_ns=3040888844 avg_ns=1798 max_ns=288263
  resolved_virtual_path_from_path: count=1125177 total_ns=2540097498 avg_ns=2257 max_ns=288263
  resolved_virtual_path_from_path_component_walk: count=1125177 total_ns=2171473043 avg_ns=1929 max_ns=287833
  resolved_virtual_path_from_path_canonicalize: count=1970276 total_ns=1753423741 avg_ns=889 max_ns=287270
  resolved_virtual_path_from_path_source_root_confinement: count=1970276 total_ns=220079711 avg_ns=111 max_ns=264748
  resolved_virtual_path_from_path_virtual_conversion: count=1125177 total_ns=312396127 avg_ns=277 max_ns=60751
  resolved_virtual_path_from_open_fd: count=565341 total_ns=500791346 avg_ns=885 max_ns=239550
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=176 total_ns=5859505123 avg_ns=33292642 max_ns=87998788
  readdir_attr_generation_scan: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=176 total_ns=359398318 avg_ns=2042035 max_ns=5277177
  readdir_page_commit: count=176 total_ns=275835049 avg_ns=1567244 max_ns=5983741
  readdirplus_directory_scan: count=27 total_ns=1076329283 avg_ns=39864047 max_ns=69480159
  readdirplus_attr_generation_scan: count=5109 total_ns=55792580 avg_ns=10920 max_ns=251733
  readdirplus_attr_generation_entries: count=5082
  readdirplus_symlink_visibility: count=27 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=27 total_ns=62968025 avg_ns=2332149 max_ns=4241119
  readdirplus_page_commit: count=27 total_ns=19660325 avg_ns=728160 max_ns=1643810
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
