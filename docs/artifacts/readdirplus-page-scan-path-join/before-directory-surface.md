# ScreenFS benchmark result

- timestamp: `2026-06-20T11:00:28.734620+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.json --output-md docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+5 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/artifacts/readdirplus-deferred-attrs/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs; ... (+5 more)`
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
| readdir_basic | 0.001447 | 0.075870 | 52.430 | 0.076616 | 0.076829 | 0.076998 |
| readdirplus_basic | 0.008664 | 0.373800 | 43.143 | 0.378990 | 0.381594 | 0.383678 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=87572 avg_ns=87572 max_ns=87572
  fuse_op.getattr: count=65027 total_ns=674678785 avg_ns=10375 max_ns=291157
  fuse_op.lookup: count=195057 total_ns=1810843897 avg_ns=9283 max_ns=274665
  fuse_op.opendir: count=26 total_ns=476952 avg_ns=18344 max_ns=43209
  fuse_op.readdir: count=180 total_ns=1329339492 avg_ns=7385219 max_ns=16383713
  fuse_op.readdirplus: count=31 total_ns=433216664 avg_ns=13974731 max_ns=17939146
  fuse_op.releasedir: count=26 total_ns=16770975 avg_ns=645037 max_ns=1083396
  fuse_op.statfs: count=2 total_ns=4942 avg_ns=2471 max_ns=3248
  policy_decision: count=1099907 total_ns=458752456 avg_ns=417 max_ns=261843
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=164628106 avg_ns=149 max_ns=54310
  matcher_candidate_order.path: count=3299721 total_ns=507721429 avg_ns=153 max_ns=40353
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5778832 avg_ns=22 max_ns=1554
  state_read_lock_hold: count=260322 total_ns=17716495 avg_ns=68 max_ns=7623
  state_write_lock_wait: count=195344 total_ns=4066286 avg_ns=20 max_ns=7925
  state_write_lock_hold: count=195344 total_ns=294347943 avg_ns=1506 max_ns=3262634
  open_confined_openat2: count=266229 total_ns=143270684 avg_ns=538 max_ns=54693
  stat_child_no_follow: count=265991 total_ns=1928527197 avg_ns=7250 max_ns=289214
  source_root_path: count=266202 total_ns=426415320 avg_ns=1601 max_ns=103293
  resolved_virtual_path: count=532218 total_ns=747152660 avg_ns=1403 max_ns=198936
  resolved_virtual_path_from_path: count=265990 total_ns=483168925 avg_ns=1816 max_ns=87426
  resolved_virtual_path_from_path_component_walk: count=265990 total_ns=396138895 avg_ns=1489 max_ns=87120
  resolved_virtual_path_from_path_canonicalize: count=336765 total_ns=315505609 avg_ns=936 max_ns=86773
  resolved_virtual_path_from_path_source_root_confinement: count=336765 total_ns=41403262 avg_ns=122 max_ns=16667
  resolved_virtual_path_from_path_virtual_conversion: count=265990 total_ns=72104448 avg_ns=271 max_ns=51242
  resolved_virtual_path_from_open_fd: count=266228 total_ns=263983735 avg_ns=991 max_ns=198936
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1140828556 avg_ns=6337936 max_ns=13714257
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=73614264 avg_ns=408968 max_ns=1124668
  readdir_page_commit: count=180 total_ns=157966876 avg_ns=877593 max_ns=3263041
  readdirplus_directory_scan: count=31 total_ns=348488862 avg_ns=11241576 max_ns=14751313
  readdirplus_attr_generation_scan: count=5859 total_ns=63235666 avg_ns=10792 max_ns=63995
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=22063679 avg_ns=711731 max_ns=1146488
  readdirplus_page_commit: count=31 total_ns=6738508 avg_ns=217371 max_ns=386449
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
