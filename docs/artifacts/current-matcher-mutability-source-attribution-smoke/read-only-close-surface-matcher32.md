# ScreenFS benchmark result

- timestamp: `2026-06-24T02:41:11.862249+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set read-only-close-surface --iterations 3 --warmups 1 --output-json docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.json --output-md docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.md --output-svg docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/perf.rs; ... (+4 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `367b50287307eb569df4a3d08e42e6db25bf4e7cd1b9db1e3aa5b8fe2be4e673`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/perf.rs; ... (+4 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-only-close-surface`
- comparable_workloads: `read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close`
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
| read_only_open_close | 0.022564 | 0.258109 | 11.439 | 0.259302 | 0.259451 | 0.259570 |
| read_only_open_read_close | 0.024813 | 0.468436 | 18.878 | 0.578768 | 0.592560 | 0.603593 |
| write_open_write_close | 0.000302 | 0.028365 | 93.852 | 0.028373 | 0.028374 | 0.028375 |
| write_open_fsync_close | 0.000317 | 0.028782 | 90.694 | 0.030972 | 0.031246 | 0.031465 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=91317 avg_ns=91317 max_ns=91317
  fuse_op.create: count=8 total_ns=1031156 avg_ns=128894 max_ns=177475
  fuse_op.flush: count=1024 total_ns=6290227 avg_ns=6142 max_ns=294933
  fuse_op.fsync: count=512 total_ns=2774459 avg_ns=5418 max_ns=35188
  fuse_op.getattr: count=16385 total_ns=189832064 avg_ns=11585 max_ns=109009
  fuse_op.getxattr: count=2040 total_ns=48460565 avg_ns=23755 max_ns=68118
  fuse_op.lookup: count=101405 total_ns=952772600 avg_ns=9395 max_ns=268677
  fuse_op.open: count=33784 total_ns=602689802 avg_ns=17839 max_ns=220448
  fuse_op.read: count=16384 total_ns=302046627 avg_ns=18435 max_ns=1034602
  fuse_op.release: count=33792 total_ns=17029886 avg_ns=503 max_ns=49497
  fuse_op.statfs: count=2 total_ns=5456 avg_ns=2728 max_ns=3604
  fuse_op.unlink: count=8 total_ns=2827992 avg_ns=353499 max_ns=453480
  fuse_op.write: count=1024 total_ns=39884213 avg_ns=38949 max_ns=99308
  policy_decision: count=232608 total_ns=301088065 avg_ns=1294 max_ns=59009
  matcher_candidates: count=1081696
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=1081696
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1081696
  matcher_candidate_order.descendant: count=228472 total_ns=112367400 avg_ns=491 max_ns=50332
  matcher_candidate_order.path: count=693688 total_ns=325787966 avg_ns=469 max_ns=57659
  matcher_candidate_order_by_source.hidden.path: count=228472 total_ns=151756575 avg_ns=664 max_ns=57659
  matcher_candidate_order_by_source.internal_hidden.path: count=228472 total_ns=33152338 avg_ns=145 max_ns=7002
  matcher_candidate_order_by_source.readonly.path: count=4136 total_ns=4154961 avg_ns=1004 max_ns=9138
  matcher_candidate_order_by_source.visible.descendant: count=228472 total_ns=112367400 avg_ns=491 max_ns=50332
  matcher_candidate_order_by_source.visible.path: count=228472 total_ns=135974240 avg_ns=595 max_ns=52671
  matcher_candidate_order_by_source.writable.path: count=4136 total_ns=749852 avg_ns=181 max_ns=9569
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=22526744
  matcher_candidate_order_seen_slots.descendant: count=7311104
  matcher_candidate_order_seen_slots.path: count=15215640
  matcher_candidate_order_ancestor_steps: count=3282668
  matcher_candidate_order_ancestor_steps.descendant: count=812411
  matcher_candidate_order_ancestor_steps.path: count=2470257
  state_read_lock_wait: count=172575 total_ns=3252695 avg_ns=18 max_ns=13191
  state_read_lock_hold: count=172575 total_ns=13144777 avg_ns=76 max_ns=72996
  state_write_lock_wait: count=168987 total_ns=3099164 avg_ns=18 max_ns=9962
  state_write_lock_hold: count=168987 total_ns=58810204 avg_ns=348 max_ns=364531
  open_confined_openat2: count=206944 total_ns=134675099 avg_ns=650 max_ns=103865
  open_like.pre_open_guard.access: count=1 total_ns=72663 avg_ns=72663 max_ns=72663
  open_like.pre_open_guard.open: count=33784 total_ns=377036403 avg_ns=11160 max_ns=190599
  open_like.post_open_revalidation.access: count=1 total_ns=11427 avg_ns=11427 max_ns=11427
  open_like.post_open_revalidation.open: count=33784 total_ns=188417719 avg_ns=5577 max_ns=64271
  stat_child_no_follow: count=171103 total_ns=196120560 avg_ns=1146 max_ns=109026
  stat_child_no_follow.attr_conversion: count=171061 total_ns=2385152 avg_ns=13 max_ns=1170
  stat_child_no_follow.host_fstat: count=171061 total_ns=29263770 avg_ns=171 max_ns=108314
  stat_child_no_follow_context.path_guard_or_metadata: count=171103 total_ns=196120560 avg_ns=1146 max_ns=109026
  source_root_path: count=173111 total_ns=240482692 avg_ns=1389 max_ns=88253
  resolved_virtual_path: count=224326 total_ns=588676567 avg_ns=2624 max_ns=669562
  resolved_virtual_path_from_path: count=171069 total_ns=535649335 avg_ns=3131 max_ns=669562
  resolved_virtual_path_from_path_component_walk: count=171069 total_ns=468886509 avg_ns=2740 max_ns=353741
  resolved_virtual_path_from_path_canonicalize: count=411753 total_ns=374904981 avg_ns=910 max_ns=262320
  resolved_virtual_path_from_path_source_root_confinement: count=411753 total_ns=54748977 avg_ns=132 max_ns=18255
  resolved_virtual_path_from_path_virtual_conversion: count=171069 total_ns=57990905 avg_ns=338 max_ns=315773
  resolved_virtual_path_from_open_fd: count=53257 total_ns=53027232 avg_ns=995 max_ns=18946
  read_handle_snapshot: count=16384 total_ns=4729007 avg_ns=288 max_ns=73091
  read_guard_path: count=16384 total_ns=280234237 avg_ns=17104 max_ns=684216
  read_io: count=16384 total_ns=13658859 avg_ns=833 max_ns=65815
  write_handle_snapshot: count=1024 total_ns=259562 avg_ns=253 max_ns=1195
  write_guard_mutation: count=1024 total_ns=37915263 avg_ns=37026 max_ns=96354
  write_io: count=1024 total_ns=1491090 avg_ns=1456 max_ns=17237
  file_sync.flush: count=1024 total_ns=5982042 avg_ns=5841 max_ns=293040
  file_sync.fsync: count=512 total_ns=2634665 avg_ns=5145 max_ns=33906
  read_size_bucket.0_4k: count=16384 total_ns=13658859 avg_ns=833 max_ns=65815
  write_size_bucket.0_4k: count=1024 total_ns=1491090 avg_ns=1456 max_ns=17237
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
  invalidations: count=16 invalidated_entries=8 evicted_entries=0 scanned_entries=16056
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
