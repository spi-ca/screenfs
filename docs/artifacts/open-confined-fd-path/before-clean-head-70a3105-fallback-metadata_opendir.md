# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:31.445854+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs --screenfs-source-root /tmp/screenfs-openfd-before.pUkWJu --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_opendir.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_opendir.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-metadata_opendir.svg`
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
| metadata_opendir | 0.000608 | 0.028093 | 46.189 | 0.031631 | 0.036226 | 0.039901 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61267 avg_ns=61267 max_ns=61267
  fuse_op.getattr: count=6657 total_ns=61678292 avg_ns=9265 max_ns=33684
  fuse_op.lookup: count=13317 total_ns=111334044 avg_ns=8360 max_ns=337505
  fuse_op.opendir: count=6656 total_ns=83177880 avg_ns=12496 max_ns=665295
  fuse_op.releasedir: count=6656 total_ns=1458242 avg_ns=219 max_ns=6604
  fuse_op.statfs: count=2 total_ns=5673 avg_ns=2836 max_ns=3835
  policy_decision: count=59918 total_ns=14251745 avg_ns=237 max_ns=4427
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=59918 total_ns=4346542 avg_ns=72 max_ns=2169
  matcher_candidate_order.path: count=179754 total_ns=17133479 avg_ns=95 max_ns=18566
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=59918
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=59918
  matcher_candidate_order_ancestor_steps: count=559224
  matcher_candidate_order_ancestor_steps.descendant: count=139806
  matcher_candidate_order_ancestor_steps.path: count=419418
  state_read_lock_wait: count=26631 total_ns=468679 avg_ns=17 max_ns=386
  state_read_lock_hold: count=26631 total_ns=1367354 avg_ns=51 max_ns=1298
  state_write_lock_wait: count=26627 total_ns=466188 avg_ns=17 max_ns=8496
  state_write_lock_hold: count=26627 total_ns=3091276 avg_ns=116 max_ns=27105
  open_confined_openat2: count=33288 total_ns=13416521 avg_ns=403 max_ns=9728
  open_like.pre_open_guard.access: count=1 total_ns=52761 avg_ns=52761 max_ns=52761
  open_like.pre_open_guard.opendir: count=6656 total_ns=61313509 avg_ns=9211 max_ns=351349
  open_like.post_open_revalidation.access: count=1 total_ns=3679 avg_ns=3679 max_ns=3679
  open_like.post_open_revalidation.opendir: count=6656 total_ns=15363753 avg_ns=2308 max_ns=312478
  stat_child_no_follow: count=26631 total_ns=134299131 avg_ns=5042 max_ns=331540
  source_root_path: count=26631 total_ns=32703994 avg_ns=1228 max_ns=324823
  resolved_virtual_path: count=86546 total_ns=103549421 avg_ns=1196 max_ns=310280
  resolved_virtual_path_from_path: count=53259 total_ns=78202240 avg_ns=1468 max_ns=18422
  resolved_virtual_path_from_path_component_walk: count=53259 total_ns=63601969 avg_ns=1194 max_ns=17608
  resolved_virtual_path_from_path_canonicalize: count=66574 total_ns=49649396 avg_ns=745 max_ns=16866
  resolved_virtual_path_from_path_source_root_confinement: count=66574 total_ns=7908651 avg_ns=118 max_ns=4675
  resolved_virtual_path_from_path_virtual_conversion: count=53259 total_ns=12110524 avg_ns=227 max_ns=2181
  resolved_virtual_path_from_open_fd: count=33287 total_ns=25347181 avg_ns=761 max_ns=310280
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
