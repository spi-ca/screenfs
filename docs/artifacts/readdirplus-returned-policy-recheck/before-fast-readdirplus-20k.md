# ScreenFS benchmark result

- timestamp: `2026-06-23T19:12:39.910157+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-readdirplus-20k.svg --dir-entries 20000 --workload readdirplus_basic`
- harness_repo_root: `/tmp/screenfs-readdirplus-before-12398`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M src/fs/backing.rs`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/tmp/screenfs-readdirplus-before-12398`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M src/fs/backing.rs`
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
| readdirplus_basic | 0.022615 | 1.496229 | 66.161 | 1.593076 | 1.594207 | 1.595111 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=97930 avg_ns=97930 max_ns=97930
  fuse_op.getattr: count=260014 total_ns=815392889 avg_ns=3135 max_ns=265596
  fuse_op.lookup: count=780031 total_ns=3086204302 avg_ns=3956 max_ns=563175
  fuse_op.opendir: count=13 total_ns=333310 avg_ns=25639 max_ns=63360
  fuse_op.readdir: count=332 total_ns=9859814874 avg_ns=29698237 max_ns=84088082
  fuse_op.readdirplus: count=33 total_ns=1386673982 avg_ns=42020423 max_ns=81824786
  fuse_op.releasedir: count=13 total_ns=25121243 avg_ns=1932403 max_ns=2862069
  fuse_op.statfs: count=2 total_ns=4672 avg_ns=2336 max_ns=2620
  policy_decision: count=4691268 total_ns=2495831602 avg_ns=532 max_ns=439755
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=4691268 total_ns=881495851 avg_ns=187 max_ns=879465
  matcher_candidate_order.path: count=14073804 total_ns=2720670228 avg_ns=193 max_ns=344492
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=71938308
  matcher_candidate_order_ancestor_steps.descendant: count=17984577
  matcher_candidate_order_ancestor_steps.path: count=53953731
  state_read_lock_wait: count=1040424 total_ns=22019112 avg_ns=21 max_ns=27427
  state_read_lock_hold: count=1040424 total_ns=66916435 avg_ns=64 max_ns=283838
  state_write_lock_wait: count=780433 total_ns=15882763 avg_ns=20 max_ns=19542
  state_write_lock_hold: count=780433 total_ns=977965346 avg_ns=1253 max_ns=6322989
  open_confined_openat2: count=1046710 total_ns=726728171 avg_ns=694 max_ns=319460
  open_like.pre_open_guard.access: count=1 total_ns=37315 avg_ns=37315 max_ns=37315
  open_like.pre_open_guard.opendir: count=13 total_ns=68155 avg_ns=5242 max_ns=21754
  open_like.post_open_revalidation.access: count=1 total_ns=50840 avg_ns=50840 max_ns=50840
  open_like.post_open_revalidation.opendir: count=13 total_ns=230637 avg_ns=17741 max_ns=36077
  stat_child_no_follow: count=1046331 total_ns=1284596201 avg_ns=1227 max_ns=319900
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=16412937 avg_ns=15 max_ns=32562
  stat_child_no_follow.host_fstat: count=1046329 total_ns=213247595 avg_ns=203 max_ns=64102
  stat_child_no_follow_context.path_guard_or_metadata: count=1046331 total_ns=1284596201 avg_ns=1227 max_ns=319900
  source_root_path: count=379 total_ns=6753661 avg_ns=17819 max_ns=121361
  resolved_virtual_path: count=379 total_ns=1319248 avg_ns=3480 max_ns=16762
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=379 total_ns=1319248 avg_ns=3480 max_ns=16762
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9347771527 avg_ns=28155938 max_ns=82974348
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=721903694 avg_ns=2174408 max_ns=5743685
  readdir_page_commit: count=332 total_ns=429752825 avg_ns=1294436 max_ns=6323611
  readdirplus_directory_scan: count=33 total_ns=1336292351 avg_ns=40493707 max_ns=80321768
  readdirplus_attr_generation_scan: count=6279 total_ns=10557417 avg_ns=1681 max_ns=51243
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=90189585 avg_ns=2733017 max_ns=5395277
  readdirplus_page_commit: count=33 total_ns=21947801 avg_ns=665084 max_ns=2681205
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
