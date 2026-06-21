# ScreenFS benchmark result

- timestamp: `2026-06-21T01:21:55.557896+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/perf-counters/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface-perf-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface-perf-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface-perf-smoke.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
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
| seq_read | 0.008840 | 0.050843 | 5.751 | 0.052330 | 0.052516 | 0.052664 |
| seq_write | 0.011925 | 0.049626 | 4.162 | 0.050568 | 0.050685 | 0.050780 |
| small_read | 0.000879 | 0.028216 | 32.094 | 0.029073 | 0.029180 | 0.029265 |
| small_write | 0.001224 | 0.055817 | 45.611 | 0.055838 | 0.055840 | 0.055842 |
| rand_read_4k | 0.020762 | 0.553488 | 26.659 | 0.571855 | 0.574151 | 0.575988 |
| rand_write_4k | 0.020626 | 1.075392 | 52.137 | 1.204894 | 1.221081 | 1.234031 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=56348 avg_ns=56348 max_ns=56348
  fuse_op.create: count=12 total_ns=1368184 avg_ns=114015 max_ns=161517
  fuse_op.flush: count=12 total_ns=906670 avg_ns=75555 max_ns=273374
  fuse_op.getattr: count=69909 total_ns=976642258 avg_ns=13970 max_ns=369046
  fuse_op.getxattr: count=69892 total_ns=1573564136 avg_ns=22514 max_ns=1520304
  fuse_op.lookup: count=137 total_ns=2407547 avg_ns=17573 max_ns=69930
  fuse_op.open: count=12 total_ns=265982 avg_ns=22165 max_ns=27726
  fuse_op.read: count=45644 total_ns=853330516 avg_ns=18695 max_ns=405095
  fuse_op.release: count=24 total_ns=56423 avg_ns=2350 max_ns=6650
  fuse_op.setattr: count=4 total_ns=164957 avg_ns=41239 max_ns=48020
  fuse_op.statfs: count=2 total_ns=5433 avg_ns=2716 max_ns=2940
  fuse_op.unlink: count=12 total_ns=20897020 avg_ns=1741418 max_ns=3293444
  fuse_op.write: count=69888 total_ns=1909844397 avg_ns=27327 max_ns=493849
  policy_decision: count=976398 total_ns=475702685 avg_ns=487 max_ns=108514
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=836530 total_ns=120388869 avg_ns=143 max_ns=390772
  matcher_candidate_order.path: count=2789326 total_ns=517895540 avg_ns=185 max_ns=328895
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=976398
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=976398
  matcher_candidate_order_ancestor_steps: count=13479272
  matcher_candidate_order_ancestor_steps.descendant: count=3090106
  matcher_candidate_order_ancestor_steps.path: count=10389166
  state_read_lock_wait: count=255523 total_ns=5261743 avg_ns=20 max_ns=940
  state_read_lock_hold: count=255523 total_ns=21103433 avg_ns=82 max_ns=68990
  state_write_lock_wait: count=183 total_ns=5844 avg_ns=31 max_ns=468
  state_write_lock_hold: count=183 total_ns=200175 avg_ns=1093 max_ns=10289
  open_confined_openat2: count=325544 total_ns=211817569 avg_ns=650 max_ns=1455630
  open_like.pre_open_guard.access: count=1 total_ns=46340 avg_ns=46340 max_ns=46340
  open_like.pre_open_guard.open: count=12 total_ns=189655 avg_ns=15804 max_ns=19899
  open_like.post_open_revalidation.access: count=1 total_ns=5126 avg_ns=5126 max_ns=5126
  open_like.post_open_revalidation.open: count=12 total_ns=39387 avg_ns=3282 max_ns=3971
  stat_child_no_follow: count=255611 total_ns=2183996719 avg_ns=8544 max_ns=1496166
  source_root_path: count=395411 total_ns=590184100 avg_ns=1492 max_ns=324156
  resolved_virtual_path: count=952258 total_ns=2154379394 avg_ns=2262 max_ns=359382
  resolved_virtual_path_from_path: count=511171 total_ns=1708781938 avg_ns=3342 max_ns=359382
  resolved_virtual_path_from_path_component_walk: count=511171 total_ns=1512903918 avg_ns=2959 max_ns=358899
  resolved_virtual_path_from_path_canonicalize: count=1277530 total_ns=1233050569 avg_ns=965 max_ns=358037
  resolved_virtual_path_from_path_source_root_confinement: count=1277530 total_ns=157105595 avg_ns=122 max_ns=343357
  resolved_virtual_path_from_path_virtual_conversion: count=511171 total_ns=168653267 avg_ns=329 max_ns=160697
  resolved_virtual_path_from_open_fd: count=441087 total_ns=445597456 avg_ns=1010 max_ns=255422
  read_handle_snapshot: count=45644 total_ns=9151830 avg_ns=200 max_ns=19265
  read_guard_path: count=45644 total_ns=753496192 avg_ns=16508 max_ns=403737
  read_io: count=45644 total_ns=82317949 avg_ns=1803 max_ns=110437
  write_handle_snapshot: count=69888 total_ns=16249291 avg_ns=232 max_ns=26170
  write_guard_mutation: count=69888 total_ns=1715529367 avg_ns=24546 max_ns=367859
  write_io: count=69888 total_ns=165056705 avg_ns=2361 max_ns=437419
  file_sync.flush: count=12 total_ns=893001 avg_ns=74416 max_ns=271618
  read_size_bucket.0_4k: count=39044 total_ns=36828849 avg_ns=943 max_ns=90470
  read_size_bucket.4k_64k: count=4420 total_ns=7875335 avg_ns=1781 max_ns=30640
  read_size_bucket.64k_1m: count=2180 total_ns=37613765 avg_ns=17254 max_ns=110437
  write_size_bucket.0_4k: count=69632 total_ns=111496719 avg_ns=1601 max_ns=291030
  write_size_bucket.64k_1m: count=256 total_ns=53559986 avg_ns=209218 max_ns=437419
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
