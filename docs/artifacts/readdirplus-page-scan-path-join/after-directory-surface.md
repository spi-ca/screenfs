# ScreenFS benchmark result

- timestamp: `2026-06-20T11:04:47.784508+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.json --output-md docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.svg`
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
| readdir_basic | 0.001909 | 0.118201 | 61.929 | 0.120492 | 0.121769 | 0.122790 |
| readdirplus_basic | 0.005362 | 0.367979 | 68.627 | 0.381103 | 0.383095 | 0.384689 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=74144 avg_ns=74144 max_ns=74144
  fuse_op.getattr: count=65027 total_ns=681634034 avg_ns=10482 max_ns=122840
  fuse_op.lookup: count=195057 total_ns=1836292217 avg_ns=9414 max_ns=340824
  fuse_op.opendir: count=26 total_ns=619934 avg_ns=23843 max_ns=47034
  fuse_op.readdir: count=180 total_ns=1677204661 avg_ns=9317803 max_ns=24599054
  fuse_op.readdirplus: count=31 total_ns=531887310 avg_ns=17157655 max_ns=27491120
  fuse_op.releasedir: count=26 total_ns=17450644 avg_ns=671178 max_ns=935826
  fuse_op.statfs: count=2 total_ns=6902 avg_ns=3451 max_ns=3539
  policy_decision: count=1099907 total_ns=568464682 avg_ns=516 max_ns=35393
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=201495295 avg_ns=183 max_ns=32427
  matcher_candidate_order.path: count=3299721 total_ns=621951272 avg_ns=188 max_ns=61565
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5834287 avg_ns=22 max_ns=11104
  state_read_lock_hold: count=260322 total_ns=16437602 avg_ns=63 max_ns=9204
  state_write_lock_wait: count=195344 total_ns=4196884 avg_ns=21 max_ns=4365
  state_write_lock_hold: count=195344 total_ns=345880824 avg_ns=1770 max_ns=5136878
  open_confined_openat2: count=266229 total_ns=149595960 avg_ns=561 max_ns=28174
  stat_child_no_follow: count=265991 total_ns=1961137326 avg_ns=7372 max_ns=110978
  source_root_path: count=266202 total_ns=434005293 avg_ns=1630 max_ns=42263
  resolved_virtual_path: count=532218 total_ns=755457813 avg_ns=1419 max_ns=42814
  resolved_virtual_path_from_path: count=265990 total_ns=488221787 avg_ns=1835 max_ns=42814
  resolved_virtual_path_from_path_component_walk: count=265990 total_ns=402323225 avg_ns=1512 max_ns=42258
  resolved_virtual_path_from_path_canonicalize: count=336765 total_ns=321130858 avg_ns=953 max_ns=28772
  resolved_virtual_path_from_path_source_root_confinement: count=336765 total_ns=42349331 avg_ns=125 max_ns=40119
  resolved_virtual_path_from_path_virtual_conversion: count=265990 total_ns=70704780 avg_ns=265 max_ns=12793
  resolved_virtual_path_from_open_fd: count=266228 total_ns=267236026 avg_ns=1003 max_ns=41275
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1441604026 avg_ns=8008911 max_ns=22725283
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=96890478 avg_ns=538280 max_ns=1624830
  readdir_page_commit: count=180 total_ns=204327309 avg_ns=1135151 max_ns=5137226
  readdirplus_directory_scan: count=31 total_ns=434796375 avg_ns=14025689 max_ns=22833086
  readdirplus_attr_generation_scan: count=5859 total_ns=71503347 avg_ns=12204 max_ns=63194
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=29110242 avg_ns=939040 max_ns=1550274
  readdirplus_page_commit: count=31 total_ns=7360319 avg_ns=237429 max_ns=364049
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
