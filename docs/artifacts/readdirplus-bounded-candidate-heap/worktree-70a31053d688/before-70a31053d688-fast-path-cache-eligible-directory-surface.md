# ScreenFS benchmark result

- timestamp: `2026-06-21T01:55:57.965216+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-readdirplus-before.UDcAvW/target/release/screenfs --screenfs-source-root /tmp/screenfs-readdirplus-before.UDcAvW --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-directory-surface.svg`
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
- workload_selection: `named-set`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.000697 | 0.046860 | 67.184 | 0.047908 | 0.048043 | 0.048151 |
| readdirplus_basic | 0.003344 | 0.372594 | 111.411 | 0.388462 | 0.390379 | 0.391913 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=175051 avg_ns=175051 max_ns=175051
  fuse_op.getattr: count=65027 total_ns=631912242 avg_ns=9717 max_ns=316401
  fuse_op.lookup: count=195057 total_ns=1683603113 avg_ns=8631 max_ns=355905
  fuse_op.opendir: count=26 total_ns=443117 avg_ns=17042 max_ns=56158
  fuse_op.readdir: count=180 total_ns=982176359 avg_ns=5456535 max_ns=12495787
  fuse_op.readdirplus: count=31 total_ns=315029399 avg_ns=10162238 max_ns=13995627
  fuse_op.releasedir: count=26 total_ns=9992706 avg_ns=384334 max_ns=1020811
  fuse_op.statfs: count=2 total_ns=5735 avg_ns=2867 max_ns=3862
  policy_decision: count=1099907 total_ns=372588506 avg_ns=338 max_ns=307643
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=131645022 avg_ns=119 max_ns=99657
  matcher_candidate_order.path: count=3299721 total_ns=412670180 avg_ns=125 max_ns=56277
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5442117 avg_ns=20 max_ns=7412
  state_read_lock_hold: count=260322 total_ns=15829841 avg_ns=60 max_ns=8659
  state_write_lock_wait: count=195344 total_ns=3761117 avg_ns=19 max_ns=5577
  state_write_lock_hold: count=195344 total_ns=226949173 avg_ns=1161 max_ns=1937953
  open_confined_openat2: count=266229 total_ns=131743154 avg_ns=494 max_ns=70798
  open_like.pre_open_guard.access: count=1 total_ns=139424 avg_ns=139424 max_ns=139424
  open_like.pre_open_guard.opendir: count=26 total_ns=260265 avg_ns=10010 max_ns=31391
  open_like.post_open_revalidation.access: count=1 total_ns=30484 avg_ns=30484 max_ns=30484
  open_like.post_open_revalidation.opendir: count=26 total_ns=69996 avg_ns=2692 max_ns=4352
  stat_child_no_follow: count=265991 total_ns=1771176009 avg_ns=6658 max_ns=354260
  source_root_path: count=266202 total_ns=387122648 avg_ns=1454 max_ns=90044
  resolved_virtual_path: count=532218 total_ns=687578861 avg_ns=1291 max_ns=143701
  resolved_virtual_path_from_path: count=265990 total_ns=449731407 avg_ns=1690 max_ns=143701
  resolved_virtual_path_from_path_component_walk: count=265990 total_ns=366706849 avg_ns=1378 max_ns=95838
  resolved_virtual_path_from_path_canonicalize: count=336765 total_ns=290590898 avg_ns=862 max_ns=95462
  resolved_virtual_path_from_path_source_root_confinement: count=336765 total_ns=39921358 avg_ns=118 max_ns=15245
  resolved_virtual_path_from_path_virtual_conversion: count=265990 total_ns=68920653 avg_ns=259 max_ns=141555
  resolved_virtual_path_from_open_fd: count=266228 total_ns=237847454 avg_ns=893 max_ns=54131
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=852608251 avg_ns=4736712 max_ns=11502024
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=51223477 avg_ns=284574 max_ns=972703
  readdir_page_commit: count=180 total_ns=108684135 avg_ns=603800 max_ns=1938122
  readdirplus_directory_scan: count=31 total_ns=261314619 avg_ns=8429503 max_ns=11585288
  readdirplus_attr_generation_scan: count=5859 total_ns=39250616 avg_ns=6699 max_ns=63630
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16053316 avg_ns=517848 max_ns=694561
  readdirplus_page_commit: count=31 total_ns=4210929 avg_ns=135836 max_ns=245970
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
