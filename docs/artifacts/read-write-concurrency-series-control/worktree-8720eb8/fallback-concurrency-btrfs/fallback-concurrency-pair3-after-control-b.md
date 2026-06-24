# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:59.271625+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-after-control-b.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-after-control-b.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-after-control-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-rw-after-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `read-write-concurrency-control`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
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
| concurrent_rand_read_write_4k | 0.060202 | 0.483360 | 8.029 | 1.324535 | 1.429681 | 1.513799 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=41076 avg_ns=41076 max_ns=41076
  fuse_op.create: count=16 total_ns=6957258 avg_ns=434828 max_ns=3711660
  fuse_op.flush: count=16 total_ns=2614571422 avg_ns=163410713 max_ns=1076354398
  fuse_op.getattr: count=32769 total_ns=619494444 avg_ns=18904 max_ns=2300297
  fuse_op.getxattr: count=32784 total_ns=978612426 avg_ns=29850 max_ns=2508513
  fuse_op.lookup: count=149 total_ns=4377909 avg_ns=29381 max_ns=339280
  fuse_op.open: count=16 total_ns=456104 avg_ns=28506 max_ns=42684
  fuse_op.read: count=32240 total_ns=849281335 avg_ns=26342 max_ns=2277012
  fuse_op.release: count=32 total_ns=89372 avg_ns=2792 max_ns=5811
  fuse_op.setattr: count=16 total_ns=1291803 avg_ns=80737 max_ns=151667
  fuse_op.statfs: count=2 total_ns=6389 avg_ns=3194 max_ns=4012
  fuse_op.unlink: count=16 total_ns=48435856 avg_ns=3027241 max_ns=5195524
  fuse_op.write: count=32768 total_ns=1389367214 avg_ns=42400 max_ns=3864122
  policy_decision: count=360104 total_ns=200607968 avg_ns=557 max_ns=2021281
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=294424 total_ns=49318185 avg_ns=167 max_ns=319046
  matcher_candidate_order.path: count=1014632 total_ns=231656411 avg_ns=228 max_ns=943434
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=93147509 avg_ns=316 max_ns=943434
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=59350218 avg_ns=201 max_ns=175023
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=19490101 avg_ns=296 max_ns=82985
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=49318185 avg_ns=167 max_ns=319046
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=48677591 avg_ns=165 max_ns=218846
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=10990992 avg_ns=167 max_ns=76106
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=10158809 avg_ns=77 max_ns=59019
  state_read_lock_hold: count=130791 total_ns=16859083 avg_ns=128 max_ns=594640
  state_write_lock_wait: count=211 total_ns=12464 avg_ns=59 max_ns=470
  state_write_lock_hold: count=211 total_ns=391924 avg_ns=1857 max_ns=13150
  open_confined_openat2: count=163768 total_ns=198396390 avg_ns=1211 max_ns=2270386
  open_like.pre_open_guard.access: count=1 total_ns=33945 avg_ns=33945 max_ns=33945
  open_like.pre_open_guard.open: count=16 total_ns=281909 avg_ns=17619 max_ns=28328
  open_like.post_open_revalidation.access: count=1 total_ns=3720 avg_ns=3720 max_ns=3720
  open_like.post_open_revalidation.open: count=16 total_ns=116656 avg_ns=7291 max_ns=15749
  stat_child_no_follow: count=130919 total_ns=275033764 avg_ns=2100 max_ns=2274134
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2119753 avg_ns=16 max_ns=25825
  stat_child_no_follow.host_fstat: count=130837 total_ns=37383344 avg_ns=285 max_ns=1384880
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=275033764 avg_ns=2100 max_ns=2274134
  source_root_path: count=163655 total_ns=558700768 avg_ns=3413 max_ns=2866723
  resolved_virtual_path: count=228726 total_ns=1237514011 avg_ns=5410 max_ns=2229021
  resolved_virtual_path_from_path: count=130853 total_ns=1077023664 avg_ns=8230 max_ns=2229021
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=995173130 avg_ns=7605 max_ns=2226606
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=881585091 avg_ns=2247 max_ns=2224687
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=67207258 avg_ns=171 max_ns=1153705
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=72608228 avg_ns=554 max_ns=369070
  resolved_virtual_path_from_open_fd: count=97873 total_ns=160490347 avg_ns=1639 max_ns=2017828
  read_handle_snapshot: count=32240 total_ns=11825076 avg_ns=366 max_ns=25677
  read_guard_path: count=32240 total_ns=788082202 avg_ns=24444 max_ns=2273325
  read_io: count=32240 total_ns=39198319 avg_ns=1215 max_ns=188849
  write_handle_snapshot: count=32768 total_ns=12357564 avg_ns=377 max_ns=332273
  write_guard_mutation: count=32768 total_ns=1210101237 avg_ns=36929 max_ns=3843642
  write_io: count=32768 total_ns=155451764 avg_ns=4744 max_ns=1806885
  file_sync.flush: count=16 total_ns=2614550860 avg_ns=163409428 max_ns=1076352339
  read_size_bucket.0_4k: count=32236 total_ns=39190918 avg_ns=1215 max_ns=188849
  read_size_bucket.4k_64k: count=4 total_ns=7401 avg_ns=1850 max_ns=2321
  write_size_bucket.0_4k: count=32768 total_ns=155451764 avg_ns=4744 max_ns=1806885
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
  invalidations: count=32 invalidated_entries=16 evicted_entries=0 scanned_entries=200
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
