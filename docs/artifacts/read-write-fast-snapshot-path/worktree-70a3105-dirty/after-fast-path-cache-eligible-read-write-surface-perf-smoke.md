# ScreenFS benchmark result

- timestamp: `2026-06-21T01:21:47.149829+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/perf-counters/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface-perf-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface-perf-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface-perf-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/perf-counters/release/screenfs`
- screenfs_bin_sha256: `15c247c5dcced589d650a87b5f5340db850fa733ea15c97bb706cd0797574845`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.013717 | 0.040818 | 2.976 | 0.041106 | 0.041142 | 0.041171 |
| seq_write | 0.011689 | 0.045187 | 3.866 | 0.045834 | 0.045914 | 0.045979 |
| small_read | 0.001089 | 0.022899 | 21.031 | 0.023195 | 0.023232 | 0.023262 |
| small_write | 0.001042 | 0.033304 | 31.969 | 0.035438 | 0.035705 | 0.035918 |
| rand_read_4k | 0.021968 | 0.410893 | 18.704 | 0.411904 | 0.412030 | 0.412132 |
| rand_write_4k | 0.020599 | 0.526391 | 25.554 | 0.535418 | 0.536547 | 0.537449 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=52974 avg_ns=52974 max_ns=52974
  fuse_op.create: count=12 total_ns=1232870 avg_ns=102739 max_ns=133856
  fuse_op.flush: count=12 total_ns=1172031 avg_ns=97669 max_ns=292167
  fuse_op.getattr: count=69909 total_ns=821568196 avg_ns=11751 max_ns=73436
  fuse_op.getxattr: count=69892 total_ns=1232756326 avg_ns=17638 max_ns=124858
  fuse_op.lookup: count=137 total_ns=2312724 avg_ns=16881 max_ns=54546
  fuse_op.open: count=12 total_ns=247938 avg_ns=20661 max_ns=36404
  fuse_op.read: count=45644 total_ns=168377607 avg_ns=3688 max_ns=65859
  fuse_op.release: count=24 total_ns=78236 avg_ns=3259 max_ns=6877
  fuse_op.setattr: count=4 total_ns=145003 avg_ns=36250 max_ns=36548
  fuse_op.statfs: count=2 total_ns=4104 avg_ns=2052 max_ns=2360
  fuse_op.unlink: count=12 total_ns=21022717 avg_ns=1751893 max_ns=2837040
  fuse_op.write: count=69888 total_ns=200840923 avg_ns=2873 max_ns=690552
  policy_decision: count=350226 total_ns=151542658 avg_ns=432 max_ns=21491
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=350134 total_ns=54376864 avg_ns=155 max_ns=11709
  matcher_candidate_order.path: count=1050586 total_ns=165732764 avg_ns=157 max_ns=21556
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5040952
  matcher_candidate_order_ancestor_steps.descendant: count=1260078
  matcher_candidate_order_ancestor_steps.path: count=3780874
  state_read_lock_wait: count=255523 total_ns=6584219 avg_ns=25 max_ns=4881
  state_read_lock_hold: count=255523 total_ns=22056175 avg_ns=86 max_ns=10072
  state_write_lock_wait: count=183 total_ns=6305 avg_ns=34 max_ns=215
  state_write_lock_hold: count=183 total_ns=227695 avg_ns=1244 max_ns=6864
  open_confined_openat2: count=209988 total_ns=159242549 avg_ns=758 max_ns=27665
  open_like.pre_open_guard.access: count=1 total_ns=22865 avg_ns=22865 max_ns=22865
  open_like.pre_open_guard.open: count=12 total_ns=155499 avg_ns=12958 max_ns=27842
  open_like.post_open_revalidation.access: count=1 total_ns=22274 avg_ns=22274 max_ns=22274
  open_like.post_open_revalidation.open: count=12 total_ns=41449 avg_ns=3454 max_ns=4094
  stat_child_no_follow: count=140055 total_ns=1366667197 avg_ns=9758 max_ns=100569
  source_root_path: count=209963 total_ns=402846430 avg_ns=1918 max_ns=35877
  resolved_virtual_path: count=350077 total_ns=653990621 avg_ns=1868 max_ns=24047
  resolved_virtual_path_from_path: count=140078 total_ns=408345740 avg_ns=2915 max_ns=24047
  resolved_virtual_path_from_path_component_walk: count=140078 total_ns=356045094 avg_ns=2541 max_ns=21614
  resolved_virtual_path_from_path_canonicalize: count=279993 total_ns=281640574 avg_ns=1005 max_ns=20896
  resolved_virtual_path_from_path_source_root_confinement: count=279993 total_ns=39737975 avg_ns=141 max_ns=12214
  resolved_virtual_path_from_path_virtual_conversion: count=140078 total_ns=43717476 avg_ns=312 max_ns=21381
  resolved_virtual_path_from_open_fd: count=209999 total_ns=245644881 avg_ns=1169 max_ns=19421
  read_handle_snapshot: count=45644 total_ns=8975212 avg_ns=196 max_ns=10200
  read_guard_path: count=45644 total_ns=931774 avg_ns=20 max_ns=435
  read_io: count=45644 total_ns=150870422 avg_ns=3305 max_ns=65524
  write_handle_snapshot: count=69888 total_ns=13817698 avg_ns=197 max_ns=9194
  write_guard_mutation: count=69888 total_ns=1435807 avg_ns=20 max_ns=6030
  write_io: count=69888 total_ns=173967127 avg_ns=2489 max_ns=686413
  file_sync.flush: count=12 total_ns=1153143 avg_ns=96095 max_ns=289824
  read_size_bucket.0_4k: count=39044 total_ns=73942467 avg_ns=1893 max_ns=20628
  read_size_bucket.4k_64k: count=4420 total_ns=17807147 avg_ns=4028 max_ns=27643
  read_size_bucket.64k_1m: count=2180 total_ns=59120808 avg_ns=27119 max_ns=65524
  write_size_bucket.0_4k: count=69632 total_ns=118125845 avg_ns=1696 max_ns=580479
  write_size_bucket.64k_1m: count=256 total_ns=55841282 avg_ns=218130 max_ns=686413
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
  invalidations: count=24 invalidated_entries=12 evicted_entries=0 scanned_entries=72
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
