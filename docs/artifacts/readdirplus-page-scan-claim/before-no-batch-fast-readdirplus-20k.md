# ScreenFS benchmark result

- timestamp: `2026-06-24T01:33:20.184409+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-readdirplus-20k.svg --dir-entries 20000 --workload readdirplus_basic`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a132ead172582dc33427c0c8142623e6edaaf4a6e201ade98854f9e035232883`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.023032 | 1.483325 | 64.402 | 1.548782 | 1.564025 | 1.576219 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=88052 avg_ns=88052 max_ns=88052
  fuse_op.getattr: count=260014 total_ns=834042635 avg_ns=3207 max_ns=275010
  fuse_op.lookup: count=780031 total_ns=3133041694 avg_ns=4016 max_ns=275166
  fuse_op.opendir: count=13 total_ns=331250 avg_ns=25480 max_ns=50834
  fuse_op.readdir: count=332 total_ns=9798718092 avg_ns=29514211 max_ns=79633576
  fuse_op.readdirplus: count=33 total_ns=1384467685 avg_ns=41953566 max_ns=77883361
  fuse_op.releasedir: count=13 total_ns=26659981 avg_ns=2050767 max_ns=2848174
  fuse_op.statfs: count=2 total_ns=3185 avg_ns=1592 max_ns=2039
  policy_decision: count=4691268 total_ns=2412122117 avg_ns=514 max_ns=543308
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=4691268 total_ns=853412610 avg_ns=181 max_ns=2018343
  matcher_candidate_order.path: count=14073804 total_ns=2627225205 avg_ns=186 max_ns=628119
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=71938308
  matcher_candidate_order_ancestor_steps.descendant: count=17984577
  matcher_candidate_order_ancestor_steps.path: count=53953731
  state_read_lock_wait: count=1040424 total_ns=29264934 avg_ns=28 max_ns=49401
  state_read_lock_hold: count=1040424 total_ns=66817631 avg_ns=64 max_ns=74739
  state_write_lock_wait: count=780433 total_ns=16526113 avg_ns=21 max_ns=24460
  state_write_lock_hold: count=780433 total_ns=959420312 avg_ns=1229 max_ns=5311138
  open_confined_openat2: count=1046710 total_ns=751080512 avg_ns=717 max_ns=135735
  open_like.pre_open_guard.access: count=1 total_ns=34834 avg_ns=34834 max_ns=34834
  open_like.pre_open_guard.opendir: count=13 total_ns=50174 avg_ns=3859 max_ns=12035
  open_like.post_open_revalidation.access: count=1 total_ns=46044 avg_ns=46044 max_ns=46044
  open_like.post_open_revalidation.opendir: count=13 total_ns=248614 avg_ns=19124 max_ns=34889
  stat_child_no_follow: count=1046331 total_ns=1317305596 avg_ns=1258 max_ns=136650
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=16769780 avg_ns=16 max_ns=19126
  stat_child_no_follow.host_fstat: count=1046329 total_ns=218959030 avg_ns=209 max_ns=62376
  stat_child_no_follow_context.path_guard_or_metadata: count=1046331 total_ns=1317305596 avg_ns=1258 max_ns=136650
  source_root_path: count=379 total_ns=7408332 avg_ns=19547 max_ns=83536
  resolved_virtual_path: count=379 total_ns=1523366 avg_ns=4019 max_ns=26909
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=379 total_ns=1523366 avg_ns=4019 max_ns=26909
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9311724130 avg_ns=28047361 max_ns=78599165
  readdir_scan.name_child_path_materialization: count=332 total_ns=885935995 avg_ns=2668481 max_ns=7739047
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=6535901568 avg_ns=19686450 max_ns=60298839
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=717589817 avg_ns=2161415 max_ns=5602209
  readdir_page_commit: count=332 total_ns=402981706 avg_ns=1213800 max_ns=5311456
  readdirplus_directory_scan: count=33 total_ns=1335045479 avg_ns=40455923 max_ns=76921920
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=129621202 avg_ns=3927915 max_ns=11183873
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=10828333 avg_ns=1733 max_ns=48843
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=14804802 avg_ns=2370 max_ns=17688
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=985986273 avg_ns=29878371 max_ns=56650133
  readdirplus_attr_generation_scan: count=6279 total_ns=10828333 avg_ns=1724 max_ns=48843
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=88074335 avg_ns=2668919 max_ns=4673471
  readdirplus_page_commit: count=33 total_ns=20405083 avg_ns=618335 max_ns=1431246
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
