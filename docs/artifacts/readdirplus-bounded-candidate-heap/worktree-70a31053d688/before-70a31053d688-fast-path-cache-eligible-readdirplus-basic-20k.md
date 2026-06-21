# ScreenFS benchmark result

- timestamp: `2026-06-21T01:56:46.852066+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-readdirplus-before.UDcAvW/target/release/screenfs --screenfs-source-root /tmp/screenfs-readdirplus-before.UDcAvW --perf-counters --policy-preset fast-path-cache-eligible --workload readdirplus_basic --dir-entries 20000 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.svg`
- harness_repo_root: `/tmp/screenfs-readdirplus-before.UDcAvW`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-readdirplus-before.UDcAvW/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-readdirplus-before.UDcAvW`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
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
| readdirplus_basic | 0.026499 | 2.096830 | 79.128 | 2.190671 | 2.211302 | 2.227807 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61047 avg_ns=61047 max_ns=61047
  fuse_op.getattr: count=260014 total_ns=2849538425 avg_ns=10959 max_ns=355641
  fuse_op.lookup: count=780031 total_ns=7767903903 avg_ns=9958 max_ns=409487
  fuse_op.opendir: count=13 total_ns=232332 avg_ns=17871 max_ns=45595
  fuse_op.readdir: count=332 total_ns=8104869087 avg_ns=24412256 max_ns=49842128
  fuse_op.readdirplus: count=33 total_ns=1070033975 avg_ns=32425271 max_ns=51571170
  fuse_op.releasedir: count=13 total_ns=28472985 avg_ns=2190229 max_ns=3483447
  fuse_op.statfs: count=2 total_ns=6221 avg_ns=3110 max_ns=4625
  policy_decision: count=5737598 total_ns=2278317019 avg_ns=397 max_ns=659267
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=5737598 total_ns=811330053 avg_ns=141 max_ns=275021
  matcher_candidate_order.path: count=17212794 total_ns=2514520587 avg_ns=146 max_ns=391065
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=81373824
  matcher_candidate_order_ancestor_steps.descendant: count=20343456
  matcher_candidate_order_ancestor_steps.path: count=61030368
  state_read_lock_wait: count=1040424 total_ns=24157692 avg_ns=23 max_ns=46567
  state_read_lock_hold: count=1040424 total_ns=75145026 avg_ns=72 max_ns=42824
  state_write_lock_wait: count=780433 total_ns=16414013 avg_ns=21 max_ns=10777
  state_write_lock_hold: count=780433 total_ns=913872940 avg_ns=1170 max_ns=3482917
  open_confined_openat2: count=1046710 total_ns=629000303 avg_ns=600 max_ns=262937
  open_like.pre_open_guard.access: count=1 total_ns=23140 avg_ns=23140 max_ns=23140
  open_like.pre_open_guard.opendir: count=13 total_ns=147867 avg_ns=11374 max_ns=24470
  open_like.post_open_revalidation.access: count=1 total_ns=33100 avg_ns=33100 max_ns=33100
  open_like.post_open_revalidation.opendir: count=13 total_ns=38905 avg_ns=2992 max_ns=3531
  stat_child_no_follow: count=1046331 total_ns=8011011652 avg_ns=7656 max_ns=405850
  source_root_path: count=1046696 total_ns=1840987043 avg_ns=1758 max_ns=348311
  resolved_virtual_path: count=2093039 total_ns=3086417463 avg_ns=1474 max_ns=323653
  resolved_virtual_path_from_path: count=1046330 total_ns=1976814446 avg_ns=1889 max_ns=323653
  resolved_virtual_path_from_path_component_walk: count=1046330 total_ns=1573685822 avg_ns=1504 max_ns=168417
  resolved_virtual_path_from_path_canonicalize: count=1312549 total_ns=1226858739 avg_ns=934 max_ns=167872
  resolved_virtual_path_from_path_source_root_confinement: count=1312549 total_ns=161889973 avg_ns=123 max_ns=96612
  resolved_virtual_path_from_path_virtual_conversion: count=1046330 total_ns=342790193 avg_ns=327 max_ns=321411
  resolved_virtual_path_from_open_fd: count=1046709 total_ns=1109603017 avg_ns=1060 max_ns=265580
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=7715433405 avg_ns=23239257 max_ns=48888228
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=593526165 avg_ns=1787729 max_ns=5254080
  readdir_page_commit: count=332 total_ns=321442381 avg_ns=968199 max_ns=3217993
  readdirplus_directory_scan: count=33 total_ns=994926369 avg_ns=30149283 max_ns=49095346
  readdirplus_attr_generation_scan: count=6279 total_ns=49135699 avg_ns=7825 max_ns=97836
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=67969210 avg_ns=2059673 max_ns=3847564
  readdirplus_page_commit: count=33 total_ns=13821941 avg_ns=418846 max_ns=1093890
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
