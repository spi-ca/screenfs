# ScreenFS benchmark result

- timestamp: `2026-06-21T01:37:30.420248+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs --screenfs-source-root /tmp/screenfs-openfd-before.pUkWJu --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fast-open-confined-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fast-open-confined-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/open-confined-fd-path/before-clean-head-70a3105-fast-open-confined-surface.svg`
- harness_repo_root: `/tmp/screenfs-openfd-before.pUkWJu`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-openfd-before.pUkWJu/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-openfd-before.pUkWJu`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
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
| metadata_open | 0.001715 | 0.027281 | 15.903 | 0.029067 | 0.029330 | 0.029541 |
| metadata_opendir | 0.000700 | 0.021718 | 31.044 | 0.022191 | 0.022883 | 0.023438 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62366 avg_ns=62366 max_ns=62366
  fuse_op.getattr: count=6657 total_ns=40745934 avg_ns=6120 max_ns=27448
  fuse_op.lookup: count=33285 total_ns=278635910 avg_ns=8371 max_ns=20109144
  fuse_op.open: count=6656 total_ns=75828864 avg_ns=11392 max_ns=328827
  fuse_op.opendir: count=6656 total_ns=59824451 avg_ns=8988 max_ns=30504
  fuse_op.release: count=6656 total_ns=3120547 avg_ns=468 max_ns=7400
  fuse_op.releasedir: count=6656 total_ns=1363438 avg_ns=204 max_ns=2746
  fuse_op.statfs: count=2 total_ns=2571 avg_ns=1285 max_ns=1579
  policy_decision: count=119822 total_ns=25701504 avg_ns=214 max_ns=4512
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=119822 total_ns=9619655 avg_ns=80 max_ns=6868
  matcher_candidate_order.path: count=359466 total_ns=30947935 avg_ns=86 max_ns=6168
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1251448
  matcher_candidate_order_ancestor_steps.descendant: count=312862
  matcher_candidate_order_ancestor_steps.path: count=938586
  state_read_lock_wait: count=53255 total_ns=886177 avg_ns=16 max_ns=298
  state_read_lock_hold: count=53255 total_ns=2705169 avg_ns=50 max_ns=2016
  state_write_lock_wait: count=59907 total_ns=984807 avg_ns=16 max_ns=4375
  state_write_lock_hold: count=59907 total_ns=11605304 avg_ns=193 max_ns=21154
  open_confined_openat2: count=66568 total_ns=79853790 avg_ns=1199 max_ns=20072901
  open_like.pre_open_guard.access: count=1 total_ns=23695 avg_ns=23695 max_ns=23695
  open_like.pre_open_guard.open: count=6656 total_ns=53885031 avg_ns=8095 max_ns=325580
  open_like.pre_open_guard.opendir: count=6656 total_ns=40577141 avg_ns=6096 max_ns=19732
  open_like.post_open_revalidation.access: count=1 total_ns=33661 avg_ns=33661 max_ns=33661
  open_like.post_open_revalidation.open: count=6656 total_ns=16061670 avg_ns=2413 max_ns=10530
  open_like.post_open_revalidation.opendir: count=6656 total_ns=13246113 avg_ns=1990 max_ns=4017
  stat_child_no_follow: count=53255 total_ns=325538071 avg_ns=6112 max_ns=20104554
  source_root_path: count=53255 total_ns=60022358 avg_ns=1127 max_ns=23577
  resolved_virtual_path: count=119821 total_ns=113298754 avg_ns=945 max_ns=18833
  resolved_virtual_path_from_path: count=53254 total_ns=62031867 avg_ns=1164 max_ns=7662
  resolved_virtual_path_from_path_component_walk: count=53254 total_ns=48207516 avg_ns=905 max_ns=7373
  resolved_virtual_path_from_path_canonicalize: count=53253 total_ns=38033016 avg_ns=714 max_ns=7172
  resolved_virtual_path_from_path_source_root_confinement: count=53253 total_ns=5530801 avg_ns=103 max_ns=2334
  resolved_virtual_path_from_path_virtual_conversion: count=53254 total_ns=11387777 avg_ns=213 max_ns=4153
  resolved_virtual_path_from_open_fd: count=66567 total_ns=51266887 avg_ns=770 max_ns=18833
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
