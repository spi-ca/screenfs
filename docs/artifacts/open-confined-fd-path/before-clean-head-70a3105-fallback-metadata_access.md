# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:32.533111+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs --screenfs-source-root /tmp/screenfs-openfd-before.pUkWJu --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_access --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_access.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_access.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_access.svg`
- harness_repo_root: `/tmp/screenfs-openfd-before.pUkWJu`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-openfd-before.pUkWJu`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_access`
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
| metadata_access | 0.001551 | 0.031752 | 20.468 | 0.032599 | 0.033025 | 0.033365 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=101108502 avg_ns=15188 max_ns=336679
  fuse_op.getattr: count=1 total_ns=15292 avg_ns=15292 max_ns=15292
  fuse_op.lookup: count=19973 total_ns=191630253 avg_ns=9594 max_ns=659057
  fuse_op.statfs: count=2 total_ns=2551 avg_ns=1275 max_ns=1654
  policy_decision: count=59918 total_ns=18298967 avg_ns=305 max_ns=3611
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=5458243 avg_ns=91 max_ns=3522
  matcher_candidate_order.path: count=179754 total_ns=21374163 avg_ns=118 max_ns=8505
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=692344
  matcher_candidate_order_ancestor_steps.descendant: count=173086
  matcher_candidate_order_ancestor_steps.path: count=519258
  state_read_lock_wait: count=26631 total_ns=434556 avg_ns=16 max_ns=123
  state_read_lock_hold: count=26631 total_ns=1406861 avg_ns=52 max_ns=1260
  state_write_lock_wait: count=19971 total_ns=312677 avg_ns=15 max_ns=160
  state_write_lock_hold: count=19971 total_ns=6821546 avg_ns=341 max_ns=319369
  open_confined_openat2: count=33288 total_ns=12194594 avg_ns=366 max_ns=10227
  open_like.pre_open_guard.access: count=6657 total_ns=78713841 avg_ns=11824 max_ns=333190
  open_like.post_open_revalidation.access: count=6657 total_ns=16552481 avg_ns=2486 max_ns=4823
  stat_child_no_follow: count=26631 total_ns=145135891 avg_ns=5449 max_ns=332536
  source_root_path: count=26631 total_ns=29480668 avg_ns=1107 max_ns=27033
  resolved_virtual_path: count=86546 total_ns=131155904 avg_ns=1515 max_ns=20508
  resolved_virtual_path_from_path: count=53259 total_ns=106508006 avg_ns=1999 max_ns=20508
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=91620491 avg_ns=1720 max_ns=20116
  resolved_virtual_path_from_path_canonicalize: count=93198 total_ns=73239953 avg_ns=785 max_ns=7692
  resolved_virtual_path_from_path_source_root_confinement: count=93198 total_ns=10909113 avg_ns=117 max_ns=3643
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=12532906 avg_ns=235 max_ns=3886
  resolved_virtual_path_from_open_fd: count=33287 total_ns=24647898 avg_ns=740 max_ns=4587
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
