# ScreenFS benchmark result

- timestamp: `2026-06-24T01:33:34.160421+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fast-readdirplus-20k.svg --dir-entries 20000 --workload readdirplus_basic`
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
| readdirplus_basic | 0.031767 | 0.870445 | 27.401 | 0.988238 | 0.991761 | 0.994580 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=80919 avg_ns=80919 max_ns=80919
  fuse_op.getattr: count=260014 total_ns=825165275 avg_ns=3173 max_ns=265661
  fuse_op.lookup: count=780031 total_ns=3107127203 avg_ns=3983 max_ns=2607154
  fuse_op.opendir: count=13 total_ns=405900 avg_ns=31223 max_ns=58298
  fuse_op.readdir: count=332 total_ns=2655241979 avg_ns=7997716 max_ns=16907993
  fuse_op.readdirplus: count=33 total_ns=324889382 avg_ns=9845132 max_ns=24971739
  fuse_op.releasedir: count=13 total_ns=30747378 avg_ns=2365182 max_ns=3325975
  fuse_op.statfs: count=2 total_ns=6711 avg_ns=3355 max_ns=4769
  policy_decision: count=1046710 total_ns=373482773 avg_ns=356 max_ns=263620
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1046710 total_ns=136606472 avg_ns=130 max_ns=143759
  matcher_candidate_order.path: count=3140130 total_ns=444760152 avg_ns=141 max_ns=132638
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=13625380
  matcher_candidate_order_ancestor_steps.descendant: count=3406345
  matcher_candidate_order_ancestor_steps.path: count=10219035
  state_read_lock_wait: count=1040424 total_ns=34084339 avg_ns=32 max_ns=2564287
  state_read_lock_hold: count=1040424 total_ns=66545743 avg_ns=63 max_ns=64844
  state_write_lock_wait: count=780433 total_ns=16153508 avg_ns=20 max_ns=49045
  state_write_lock_hold: count=780433 total_ns=918974154 avg_ns=1177 max_ns=6019510
  open_confined_openat2: count=1046710 total_ns=732451227 avg_ns=699 max_ns=577283
  open_like.pre_open_guard.access: count=1 total_ns=30888 avg_ns=30888 max_ns=30888
  open_like.pre_open_guard.opendir: count=13 total_ns=72355 avg_ns=5565 max_ns=11790
  open_like.post_open_revalidation.access: count=1 total_ns=40833 avg_ns=40833 max_ns=40833
  open_like.post_open_revalidation.opendir: count=13 total_ns=278595 avg_ns=21430 max_ns=40536
  stat_child_no_follow: count=1046331 total_ns=1287052830 avg_ns=1230 max_ns=579864
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=16584041 avg_ns=15 max_ns=62058
  stat_child_no_follow.host_fstat: count=1046329 total_ns=210331278 avg_ns=201 max_ns=264917
  stat_child_no_follow_context.path_guard_or_metadata: count=1046331 total_ns=1287052830 avg_ns=1230 max_ns=579864
  source_root_path: count=379 total_ns=7237648 avg_ns=19096 max_ns=87849
  resolved_virtual_path: count=379 total_ns=1547956 avg_ns=4084 max_ns=34514
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=379 total_ns=1547956 avg_ns=4084 max_ns=34514
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=2220923794 avg_ns=6689529 max_ns=15229907
  readdir_scan.name_child_path_materialization: count=332 total_ns=711664289 avg_ns=2143567 max_ns=6412845
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=51058078 avg_ns=153789 max_ns=400656
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=485546711 avg_ns=1462490 max_ns=4449562
  readdir_page_commit: count=332 total_ns=364427593 avg_ns=1097673 max_ns=6019870
  readdirplus_directory_scan: count=33 total_ns=282788095 avg_ns=8569336 max_ns=23115200
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=102736112 avg_ns=3113215 max_ns=10117937
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=9469566 avg_ns=1516 max_ns=23826
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=13125035 avg_ns=2101 max_ns=14093
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=7765745 avg_ns=235325 max_ns=649674
  readdirplus_attr_generation_scan: count=6279 total_ns=9469566 avg_ns=1508 max_ns=23826
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=65427349 avg_ns=1982646 max_ns=6167000
  readdirplus_page_commit: count=33 total_ns=16530025 avg_ns=500909 max_ns=1326377
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
