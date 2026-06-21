# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:29.064023+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs --screenfs-source-root /tmp/screenfs-openfd-before.pUkWJu --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fallback-open-confined-surface.svg`
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
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
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
| metadata_open | 0.001807 | 0.032916 | 18.213 | 0.035977 | 0.037219 | 0.038213 |
| metadata_opendir | 0.000627 | 0.027170 | 43.302 | 0.028484 | 0.029034 | 0.029474 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=24692 avg_ns=24692 max_ns=24692
  fuse_op.getattr: count=6657 total_ns=59328779 avg_ns=8912 max_ns=29788
  fuse_op.lookup: count=33285 total_ns=348725010 avg_ns=10476 max_ns=14582175
  fuse_op.open: count=6656 total_ns=102922328 avg_ns=15463 max_ns=96124
  fuse_op.opendir: count=6656 total_ns=78735375 avg_ns=11829 max_ns=50156
  fuse_op.release: count=6656 total_ns=2704333 avg_ns=406 max_ns=2559
  fuse_op.releasedir: count=6656 total_ns=1281691 avg_ns=192 max_ns=2714
  fuse_op.statfs: count=2 total_ns=2823 avg_ns=1411 max_ns=2092
  policy_decision: count=119822 total_ns=31989434 avg_ns=266 max_ns=6399
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=9543391 avg_ns=79 max_ns=994
  matcher_candidate_order.path: count=359466 total_ns=37812509 avg_ns=105 max_ns=5273
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=119822
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=119822
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=882085 avg_ns=16 max_ns=312
  state_read_lock_hold: count=53255 total_ns=2801674 avg_ns=52 max_ns=8896
  state_write_lock_wait: count=59907 total_ns=959149 avg_ns=16 max_ns=290
  state_write_lock_hold: count=59907 total_ns=11451311 avg_ns=191 max_ns=36503
  open_confined_openat2: count=66568 total_ns=70114227 avg_ns=1053 max_ns=14570097
  open_like.pre_open_guard.access: count=1 total_ns=19272 avg_ns=19272 max_ns=19272
  open_like.pre_open_guard.open: count=6656 total_ns=80228384 avg_ns=12053 max_ns=92322
  open_like.pre_open_guard.opendir: count=6656 total_ns=58327038 avg_ns=8763 max_ns=33657
  open_like.post_open_revalidation.access: count=1 total_ns=2548 avg_ns=2548 max_ns=2548
  open_like.post_open_revalidation.open: count=6656 total_ns=16971429 avg_ns=2549 max_ns=7714
  open_like.post_open_revalidation.opendir: count=6656 total_ns=14424569 avg_ns=2167 max_ns=14293
  stat_child_no_follow: count=53255 total_ns=322676386 avg_ns=6059 max_ns=14578018
  source_root_path: count=53255 total_ns=60964058 avg_ns=1144 max_ns=20033
  resolved_virtual_path: count=173074 total_ns=235726380 avg_ns=1361 max_ns=82767
  resolved_virtual_path_from_path: count=106507 total_ns=184570186 avg_ns=1732 max_ns=82767
  resolved_virtual_path_from_path_component_walk: count=106507 total_ns=154798057 avg_ns=1453 max_ns=82368
  resolved_virtual_path_from_path_canonicalize: count=159758 total_ns=122408243 avg_ns=766 max_ns=81755
  resolved_virtual_path_from_path_source_root_confinement: count=159758 total_ns=18772870 avg_ns=117 max_ns=12801
  resolved_virtual_path_from_path_virtual_conversion: count=106507 total_ns=25026556 avg_ns=234 max_ns=6887
  resolved_virtual_path_from_open_fd: count=66567 total_ns=51156194 avg_ns=768 max_ns=12759
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
