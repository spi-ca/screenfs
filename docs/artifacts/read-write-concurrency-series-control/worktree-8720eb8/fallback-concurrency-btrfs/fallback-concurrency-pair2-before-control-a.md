# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:50.010958+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-before-control-a.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-before-control-a.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-before-control-a.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+35 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-rw-before-control`
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
| concurrent_rand_read_write_4k | 0.074680 | 0.427928 | 5.730 | 0.439333 | 0.440759 | 0.441900 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=102794 avg_ns=102794 max_ns=102794
  fuse_op.create: count=16 total_ns=1952667 avg_ns=122041 max_ns=193299
  fuse_op.flush: count=16 total_ns=1660287676 avg_ns=103767979 max_ns=157757649
  fuse_op.getattr: count=32769 total_ns=588756895 avg_ns=17966 max_ns=1907251
  fuse_op.getxattr: count=32784 total_ns=942947619 avg_ns=28762 max_ns=2658114
  fuse_op.lookup: count=149 total_ns=3303960 avg_ns=22174 max_ns=69535
  fuse_op.open: count=16 total_ns=469070 avg_ns=29316 max_ns=99464
  fuse_op.read: count=32240 total_ns=808938498 avg_ns=25091 max_ns=1977100
  fuse_op.release: count=32 total_ns=93557 avg_ns=2923 max_ns=5590
  fuse_op.setattr: count=16 total_ns=1064941 avg_ns=66558 max_ns=163993
  fuse_op.statfs: count=2 total_ns=6255 avg_ns=3127 max_ns=3296
  fuse_op.unlink: count=16 total_ns=30269276 avg_ns=1891829 max_ns=3829868
  fuse_op.write: count=32768 total_ns=1368221722 avg_ns=41754 max_ns=2290671
  policy_decision: count=360104 total_ns=190248176 avg_ns=528 max_ns=682960
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
  matcher_candidate_order.descendant: count=294424 total_ns=45694088 avg_ns=155 max_ns=106416
  matcher_candidate_order.path: count=1014632 total_ns=222153731 avg_ns=218 max_ns=2071543
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=87389359 avg_ns=296 max_ns=2071543
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=57044513 avg_ns=193 max_ns=1060282
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=19348064 avg_ns=294 max_ns=1133591
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=45694088 avg_ns=155 max_ns=106416
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=47245285 avg_ns=160 max_ns=105266
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=11126510 avg_ns=169 max_ns=109357
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=10226080 avg_ns=78 max_ns=54143
  state_read_lock_hold: count=130791 total_ns=14636225 avg_ns=111 max_ns=290881
  state_write_lock_wait: count=211 total_ns=13170 avg_ns=62 max_ns=1687
  state_write_lock_hold: count=211 total_ns=296385 avg_ns=1404 max_ns=12840
  open_confined_openat2: count=163768 total_ns=168991589 avg_ns=1031 max_ns=632526
  open_like.pre_open_guard.access: count=1 total_ns=78304 avg_ns=78304 max_ns=78304
  open_like.pre_open_guard.open: count=16 total_ns=329316 avg_ns=20582 max_ns=82323
  open_like.post_open_revalidation.access: count=1 total_ns=17039 avg_ns=17039 max_ns=17039
  open_like.post_open_revalidation.open: count=16 total_ns=89025 avg_ns=5564 max_ns=10373
  stat_child_no_follow: count=130919 total_ns=238529897 avg_ns=1821 max_ns=1764014
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2075474 avg_ns=15 max_ns=28252
  stat_child_no_follow.host_fstat: count=130837 total_ns=33548047 avg_ns=256 max_ns=1759368
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=238529897 avg_ns=1821 max_ns=1764014
  source_root_path: count=163655 total_ns=512015567 avg_ns=3128 max_ns=1957609
  resolved_virtual_path: count=228726 total_ns=1166558707 avg_ns=5100 max_ns=2640150
  resolved_virtual_path_from_path: count=130853 total_ns=1019108194 avg_ns=7788 max_ns=2640150
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=943217124 avg_ns=7208 max_ns=2638620
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=837275324 avg_ns=2134 max_ns=2637772
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=64131419 avg_ns=163 max_ns=619270
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=67257215 avg_ns=513 max_ns=250583
  resolved_virtual_path_from_open_fd: count=97873 total_ns=147450513 avg_ns=1506 max_ns=1166004
  read_handle_snapshot: count=32240 total_ns=11268124 avg_ns=349 max_ns=54837
  read_guard_path: count=32240 total_ns=753681710 avg_ns=23377 max_ns=1975381
  read_io: count=32240 total_ns=33919893 avg_ns=1052 max_ns=756812
  write_handle_snapshot: count=32768 total_ns=11937364 avg_ns=364 max_ns=60224
  write_guard_mutation: count=32768 total_ns=1213636727 avg_ns=37037 max_ns=2280433
  write_io: count=32768 total_ns=131042755 avg_ns=3999 max_ns=312948
  file_sync.flush: count=16 total_ns=1660266783 avg_ns=103766673 max_ns=157757092
  read_size_bucket.0_4k: count=32236 total_ns=33913253 avg_ns=1052 max_ns=756812
  read_size_bucket.4k_64k: count=4 total_ns=6640 avg_ns=1660 max_ns=2146
  write_size_bucket.0_4k: count=32768 total_ns=131042755 avg_ns=3999 max_ns=312948
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
