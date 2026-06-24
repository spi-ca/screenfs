# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:55.168745+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-context.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-context.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-context.svg --workload metadata_lookup --workload metadata_getattr --workload metadata_access`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-toctou-hardening-evidence.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs/backing.rs;  M src/fs/tests/perf.rs; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
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
| metadata_lookup | 0.004770 | 0.024823 | 5.204 | 0.029791 | 0.030164 | 0.030463 |
| metadata_getattr | 0.002873 | 0.036065 | 12.552 | 0.037435 | 0.037487 | 0.037528 |
| metadata_access | 0.002461 | 0.038477 | 15.634 | 0.038803 | 0.038865 | 0.038913 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=98758523 avg_ns=14835 max_ns=100228
  fuse_op.getattr: count=6657 total_ns=68698430 avg_ns=10319 max_ns=122515
  fuse_op.lookup: count=59909 total_ns=546731392 avg_ns=9126 max_ns=312827
  fuse_op.statfs: count=2 total_ns=7441 avg_ns=3720 max_ns=4742
  policy_decision: count=79880 total_ns=38424574 avg_ns=481 max_ns=17295
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=79880 total_ns=11535624 avg_ns=144 max_ns=21980
  matcher_candidate_order.path: count=239640 total_ns=46019239 avg_ns=192 max_ns=21149
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=79880
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=79880
  matcher_candidate_order_ancestor_steps: count=1038412
  matcher_candidate_order_ancestor_steps.descendant: count=259603
  matcher_candidate_order_ancestor_steps.path: count=778809
  state_read_lock_wait: count=73223 total_ns=1674932 avg_ns=22 max_ns=2647
  state_read_lock_hold: count=73223 total_ns=4943243 avg_ns=67 max_ns=13496
  state_write_lock_wait: count=53251 total_ns=1187789 avg_ns=22 max_ns=666
  state_write_lock_hold: count=53251 total_ns=23676457 avg_ns=444 max_ns=305382
  open_confined_openat2: count=79880 total_ns=65514642 avg_ns=820 max_ns=32936
  open_like.pre_open_guard.access: count=6657 total_ns=67300457 avg_ns=10109 max_ns=80818
  open_like.post_open_revalidation.access: count=6657 total_ns=22490879 avg_ns=3378 max_ns=16499
  stat_child_no_follow: count=73223 total_ns=105613188 avg_ns=1442 max_ns=108318
  stat_child_no_follow.attr_conversion: count=66565 total_ns=1124474 avg_ns=16 max_ns=237
  stat_child_no_follow.host_fstat: count=66565 total_ns=16767183 avg_ns=251 max_ns=106289
  stat_child_no_follow_context.path_guard_or_metadata: count=73223 total_ns=105613188 avg_ns=1442 max_ns=108318
  source_root_path: count=73223 total_ns=132032281 avg_ns=1803 max_ns=103731
  resolved_virtual_path: count=73222 total_ns=221172106 avg_ns=3020 max_ns=63524
  resolved_virtual_path_from_path: count=66565 total_ns=212954485 avg_ns=3199 max_ns=63524
  resolved_virtual_path_from_path_component_walk: count=66565 total_ns=184500640 avg_ns=2771 max_ns=62828
  resolved_virtual_path_from_path_canonicalize: count=139785 total_ns=147115605 avg_ns=1052 max_ns=62274
  resolved_virtual_path_from_path_source_root_confinement: count=139785 total_ns=21038194 avg_ns=150 max_ns=7967
  resolved_virtual_path_from_path_virtual_conversion: count=66565 total_ns=24420376 avg_ns=366 max_ns=39790
  resolved_virtual_path_from_open_fd: count=6657 total_ns=8217621 avg_ns=1234 max_ns=7198
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
