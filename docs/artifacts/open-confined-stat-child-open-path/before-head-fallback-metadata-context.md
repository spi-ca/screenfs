# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:52.534950+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-context.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-context.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/before-head-fallback-metadata-context.svg --workload metadata_lookup --workload metadata_getattr --workload metadata_access`
- harness_repo_root: `/tmp/screenfs-bench-before-open-confined-168995`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `ca7372519260f85471e13bfc19189bf974d6fc0a973eb7e02270315c8d89cb97`
- screenfs_source_root: `/tmp/screenfs-bench-before-open-confined-168995`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
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
| metadata_lookup | 0.004196 | 0.036989 | 8.815 | 0.038894 | 0.038986 | 0.039060 |
| metadata_getattr | 0.004438 | 0.035103 | 7.911 | 0.036085 | 0.036541 | 0.036906 |
| metadata_access | 0.003773 | 0.036711 | 9.731 | 0.037634 | 0.037704 | 0.037759 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=111582087 avg_ns=16761 max_ns=281844
  fuse_op.getattr: count=6657 total_ns=89662752 avg_ns=13468 max_ns=95551
  fuse_op.lookup: count=59909 total_ns=720471870 avg_ns=12026 max_ns=305787
  fuse_op.statfs: count=2 total_ns=4338 avg_ns=2169 max_ns=3439
  policy_decision: count=153102 total_ns=52272737 avg_ns=341 max_ns=73502
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=153102 total_ns=15727123 avg_ns=102 max_ns=63969
  matcher_candidate_order.path: count=459306 total_ns=61385861 avg_ns=133 max_ns=64999
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=153102
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=153102
  matcher_candidate_order_ancestor_steps: count=1677432
  matcher_candidate_order_ancestor_steps.descendant: count=419358
  matcher_candidate_order_ancestor_steps.path: count=1258074
  state_read_lock_wait: count=73223 total_ns=2148958 avg_ns=29 max_ns=3994
  state_read_lock_hold: count=73223 total_ns=4850549 avg_ns=66 max_ns=47827
  state_write_lock_wait: count=53251 total_ns=999942 avg_ns=18 max_ns=327
  state_write_lock_hold: count=53251 total_ns=18051868 avg_ns=338 max_ns=53176
  open_confined_openat2: count=79880 total_ns=38993575 avg_ns=488 max_ns=75064
  open_like.pre_open_guard.access: count=6657 total_ns=86333784 avg_ns=12968 max_ns=276268
  open_like.post_open_revalidation.access: count=6657 total_ns=18203988 avg_ns=2734 max_ns=18698
  stat_child_no_follow: count=73223 total_ns=507603025 avg_ns=6932 max_ns=118699
  stat_child_no_follow.attr_conversion: count=66565 total_ns=951776 avg_ns=14 max_ns=10348
  stat_child_no_follow.directory_revalidation: count=73222 total_ns=399494327 avg_ns=5455 max_ns=117365
  stat_child_no_follow.host_fstat: count=1 total_ns=1983 avg_ns=1983 max_ns=1983
  stat_child_no_follow.host_fstatat: count=73222 total_ns=29767664 avg_ns=406 max_ns=58874
  stat_child_no_follow.parent_open: count=73222 total_ns=49892449 avg_ns=681 max_ns=55832
  stat_child_no_follow_context.path_guard_or_metadata: count=73223 total_ns=507603025 avg_ns=6932 max_ns=118699
  source_root_path: count=73223 total_ns=109101368 avg_ns=1489 max_ns=78897
  resolved_virtual_path: count=219666 total_ns=366809776 avg_ns=1669 max_ns=294632
  resolved_virtual_path_from_path: count=139787 total_ns=293053004 avg_ns=2096 max_ns=294632
  resolved_virtual_path_from_path_component_walk: count=139787 total_ns=247014147 avg_ns=1767 max_ns=294163
  resolved_virtual_path_from_path_canonicalize: count=226318 total_ns=196186992 avg_ns=866 max_ns=293417
  resolved_virtual_path_from_path_source_root_confinement: count=226318 total_ns=30100512 avg_ns=133 max_ns=85895
  resolved_virtual_path_from_path_virtual_conversion: count=139787 total_ns=38593117 avg_ns=276 max_ns=263499
  resolved_virtual_path_from_open_fd: count=79879 total_ns=73756772 avg_ns=923 max_ns=67471
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
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
