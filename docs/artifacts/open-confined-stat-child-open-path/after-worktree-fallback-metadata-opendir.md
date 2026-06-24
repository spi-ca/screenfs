# ScreenFS benchmark result

- timestamp: `2026-06-23T17:24:49.772448+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-opendir.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-opendir.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-stat-child-open-path/after-worktree-fallback-metadata-opendir.svg --workload metadata_opendir`
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
- comparable_workloads: `metadata_opendir`
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
| metadata_opendir | 0.001458 | 0.032670 | 22.409 | 0.033974 | 0.033994 | 0.034010 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=96163 avg_ns=96163 max_ns=96163
  fuse_op.getattr: count=6657 total_ns=54525666 avg_ns=8190 max_ns=69100
  fuse_op.lookup: count=13317 total_ns=110731040 avg_ns=8315 max_ns=102091
  fuse_op.opendir: count=6656 total_ns=83192390 avg_ns=12498 max_ns=70133
  fuse_op.releasedir: count=6656 total_ns=2327322 avg_ns=349 max_ns=87663
  fuse_op.statfs: count=2 total_ns=6329 avg_ns=3164 max_ns=3479
  policy_decision: count=33288 total_ns=13289756 avg_ns=399 max_ns=9799
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=33288 total_ns=4028611 avg_ns=121 max_ns=4180
  matcher_candidate_order.path: count=99864 total_ns=16080009 avg_ns=161 max_ns=33603
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=33288
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=33288
  matcher_candidate_order_ancestor_steps: count=372812
  matcher_candidate_order_ancestor_steps.descendant: count=93203
  matcher_candidate_order_ancestor_steps.path: count=279609
  state_read_lock_wait: count=26631 total_ns=645541 avg_ns=24 max_ns=1370
  state_read_lock_hold: count=26631 total_ns=2008102 avg_ns=75 max_ns=2081
  state_write_lock_wait: count=26627 total_ns=634121 avg_ns=23 max_ns=616
  state_write_lock_hold: count=26627 total_ns=5094610 avg_ns=191 max_ns=87556
  open_confined_openat2: count=33288 total_ns=25116176 avg_ns=754 max_ns=54862
  open_like.pre_open_guard.access: count=1 total_ns=76591 avg_ns=76591 max_ns=76591
  open_like.pre_open_guard.opendir: count=6656 total_ns=54441645 avg_ns=8179 max_ns=53581
  open_like.post_open_revalidation.access: count=1 total_ns=11683 avg_ns=11683 max_ns=11683
  open_like.post_open_revalidation.opendir: count=6656 total_ns=19474783 avg_ns=2925 max_ns=19542
  stat_child_no_follow: count=26631 total_ns=38538372 avg_ns=1447 max_ns=55729
  stat_child_no_follow.attr_conversion: count=26629 total_ns=461269 avg_ns=17 max_ns=130
  stat_child_no_follow.host_fstat: count=26629 total_ns=6235669 avg_ns=234 max_ns=5672
  stat_child_no_follow_context.path_guard_or_metadata: count=26631 total_ns=38538372 avg_ns=1447 max_ns=55729
  source_root_path: count=26631 total_ns=49950413 avg_ns=1875 max_ns=60411
  resolved_virtual_path: count=33286 total_ns=78858521 avg_ns=2369 max_ns=64140
  resolved_virtual_path_from_path: count=26629 total_ns=70833892 avg_ns=2660 max_ns=64140
  resolved_virtual_path_from_path_component_walk: count=26629 total_ns=58854237 avg_ns=2210 max_ns=55669
  resolved_virtual_path_from_path_canonicalize: count=46601 total_ns=46442144 avg_ns=996 max_ns=33921
  resolved_virtual_path_from_path_source_root_confinement: count=46601 total_ns=6519255 avg_ns=139 max_ns=54574
  resolved_virtual_path_from_path_virtual_conversion: count=26629 total_ns=10319218 avg_ns=387 max_ns=61646
  resolved_virtual_path_from_open_fd: count=6657 total_ns=8024629 avg_ns=1205 max_ns=17259
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
