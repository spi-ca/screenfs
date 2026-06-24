# ScreenFS benchmark result

- timestamp: `2026-06-24T01:32:58.398604+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-directory-surface.svg --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-after-48607`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-after-48607`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.001671 | 0.016196 | 9.692 | 0.019902 | 0.029405 | 0.037007 |
| readdirplus_basic | 0.005395 | 0.202072 | 37.454 | 0.204726 | 0.205273 | 0.205710 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=86866 avg_ns=86866 max_ns=86866
  fuse_op.getattr: count=65027 total_ns=211031180 avg_ns=3245 max_ns=48398
  fuse_op.lookup: count=195057 total_ns=782382338 avg_ns=4011 max_ns=957566
  fuse_op.opendir: count=26 total_ns=639546 avg_ns=24597 max_ns=56096
  fuse_op.readdir: count=180 total_ns=443091992 avg_ns=2461622 max_ns=6441034
  fuse_op.readdirplus: count=31 total_ns=91401316 avg_ns=2948429 max_ns=4099793
  fuse_op.releasedir: count=26 total_ns=13039627 avg_ns=501524 max_ns=1173044
  fuse_op.statfs: count=2 total_ns=6967 avg_ns=3483 max_ns=3728
  policy_decision: count=266229 total_ns=98373864 avg_ns=369 max_ns=317757
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=266229 total_ns=37119392 avg_ns=139 max_ns=27755
  matcher_candidate_order.path: count=798687 total_ns=116297578 avg_ns=145 max_ns=26804
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=3477832
  matcher_candidate_order_ancestor_steps.descendant: count=869458
  matcher_candidate_order_ancestor_steps.path: count=2608374
  state_read_lock_wait: count=260322 total_ns=7315765 avg_ns=28 max_ns=262219
  state_read_lock_hold: count=260322 total_ns=16590058 avg_ns=63 max_ns=47998
  state_write_lock_wait: count=195344 total_ns=4316213 avg_ns=22 max_ns=10983
  state_write_lock_hold: count=195344 total_ns=271197225 avg_ns=1388 max_ns=3614828
  open_confined_openat2: count=266229 total_ns=198997093 avg_ns=747 max_ns=258214
  open_like.pre_open_guard.access: count=1 total_ns=37556 avg_ns=37556 max_ns=37556
  open_like.pre_open_guard.opendir: count=26 total_ns=133059 avg_ns=5117 max_ns=12617
  open_like.post_open_revalidation.access: count=1 total_ns=42420 avg_ns=42420 max_ns=42420
  open_like.post_open_revalidation.opendir: count=26 total_ns=432986 avg_ns=16653 max_ns=46667
  stat_child_no_follow: count=265991 total_ns=348456885 avg_ns=1310 max_ns=258673
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4416752 avg_ns=16 max_ns=580
  stat_child_no_follow.host_fstat: count=265989 total_ns=59848306 avg_ns=225 max_ns=53577
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=348456885 avg_ns=1310 max_ns=258673
  source_root_path: count=238 total_ns=2975054 avg_ns=12500 max_ns=57646
  resolved_virtual_path: count=238 total_ns=689082 avg_ns=2895 max_ns=11335
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=238 total_ns=689082 avg_ns=2895 max_ns=11335
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=282508027 avg_ns=1569489 max_ns=4816576
  readdir_scan.name_child_path_materialization: count=180 total_ns=97028709 avg_ns=539048 max_ns=1938063
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=6667755 avg_ns=37043 max_ns=141791
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=56560229 avg_ns=314223 max_ns=1444553
  readdir_page_commit: count=180 total_ns=135540212 avg_ns=753001 max_ns=3615083
  readdirplus_directory_scan: count=31 total_ns=68664987 avg_ns=2214999 max_ns=2995626
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=27488113 avg_ns=886713 max_ns=1324241
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6627951 avg_ns=1137 max_ns=34179
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=9493951 avg_ns=1629 max_ns=8455
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2060399 avg_ns=66464 max_ns=97665
  readdirplus_attr_generation_scan: count=5859 total_ns=6627951 avg_ns=1131 max_ns=34179
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=17121405 avg_ns=552303 max_ns=816249
  readdirplus_page_commit: count=31 total_ns=4667700 avg_ns=150570 max_ns=376141
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
