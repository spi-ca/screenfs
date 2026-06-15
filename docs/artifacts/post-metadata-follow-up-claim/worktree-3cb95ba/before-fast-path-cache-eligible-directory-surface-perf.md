# ScreenFS benchmark result

- timestamp: `2026-06-15T22:09:18.450620+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-postmeta-target-perf/release/screenfs --screenfs-source-root /tmp/screenfs-before-postmeta-3cb95ba --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-directory-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-directory-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-directory-surface-perf.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-env.json;  M docs/artifacts/managed-fio-attribution-native.json; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-postmeta-target-perf/release/screenfs`
- screenfs_bin_sha256: `9c998fe7a1356552b25afbfdd6be82fd2bf07d2a37ccae50c731d32fa9a777dd`
- screenfs_source_root: `/tmp/screenfs-before-postmeta-3cb95ba`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `True`
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
| readdir_basic | 0.001227 | 0.070190 | 57.182 | 0.071454 | 0.071915 | 0.072283 |
| readdirplus_basic | 0.007992 | 0.551295 | 68.984 | 0.579097 | 0.587055 | 0.593422 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=69321 avg_ns=69321 max_ns=69321
  fuse_op.getattr: count=65027 total_ns=1173771462 avg_ns=18050 max_ns=283647
  fuse_op.lookup: count=195057 total_ns=2933096399 avg_ns=15037 max_ns=237248
  fuse_op.opendir: count=26 total_ns=642873 avg_ns=24725 max_ns=41608
  fuse_op.readdir: count=180 total_ns=1427695039 avg_ns=7931639 max_ns=16052319
  fuse_op.readdirplus: count=31 total_ns=394857426 avg_ns=12737336 max_ns=16354350
  fuse_op.releasedir: count=26 total_ns=9638120 avg_ns=370696 max_ns=583754
  fuse_op.statfs: count=2 total_ns=3476 avg_ns=1738 max_ns=2134
  policy_decision: count=1088251 total_ns=451309506 avg_ns=414 max_ns=17794
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1088251 total_ns=165202283 avg_ns=151 max_ns=22693
  matcher_candidate_order.path: count=3264753 total_ns=491503035 avg_ns=150 max_ns=31099
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14808676
  matcher_candidate_order_ancestor_steps.descendant: count=3702169
  matcher_candidate_order_ancestor_steps.path: count=11106507
  state_read_lock_wait: count=260322 total_ns=6618195 avg_ns=25 max_ns=7454
  state_read_lock_hold: count=260322 total_ns=32350878 avg_ns=124 max_ns=9959
  state_write_lock_wait: count=195344 total_ns=4987639 avg_ns=25 max_ns=7774
  state_write_lock_hold: count=195344 total_ns=296791686 avg_ns=1519 max_ns=2770390
  open_confined_openat2: count=260401 total_ns=267841671 avg_ns=1028 max_ns=54503
  stat_child_no_follow: count=260163 total_ns=3441028646 avg_ns=13226 max_ns=281632
  source_root_path: count=260400 total_ns=934907109 avg_ns=3590 max_ns=28444
  resolved_virtual_path: count=520562 total_ns=1483131488 avg_ns=2849 max_ns=265402
  resolved_virtual_path_from_path: count=260162 total_ns=992495606 avg_ns=3814 max_ns=224264
  resolved_virtual_path_from_path_component_walk: count=260162 total_ns=893270557 avg_ns=3433 max_ns=223573
  resolved_virtual_path_from_path_canonicalize: count=325109 total_ns=808938128 avg_ns=2488 max_ns=223092
  resolved_virtual_path_from_path_source_root_confinement: count=325109 total_ns=43960138 avg_ns=135 max_ns=11433
  resolved_virtual_path_from_path_virtual_conversion: count=260162 total_ns=80165406 avg_ns=308 max_ns=13826
  resolved_virtual_path_from_open_fd: count=260400 total_ns=490635882 avg_ns=1884 max_ns=265402
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1269078441 avg_ns=7050435 max_ns=15144141
  readdir_attr_generation_scan: count=180 total_ns=406004117 avg_ns=2255578 max_ns=5297349
  readdir_attr_generation_entries: count=427820
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=71402093 avg_ns=396678 max_ns=974119
  readdir_page_commit: count=180 total_ns=141389037 avg_ns=785494 max_ns=2770520
  readdirplus_directory_scan: count=31 total_ns=387971398 avg_ns=12515206 max_ns=15965956
  readdirplus_attr_generation_scan: count=31 total_ns=129102073 avg_ns=4164583 max_ns=5634746
  readdirplus_attr_generation_entries: count=139868
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=22929821 avg_ns=739671 max_ns=926657
  readdirplus_page_commit: count=31 total_ns=5217738 avg_ns=168314 max_ns=313113
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
