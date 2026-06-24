# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:38.429318+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-before-control-a.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-before-control-a.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-before-control-a.svg`
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
| concurrent_rand_read_write_4k | 0.075361 | 0.397411 | 5.273 | 0.424891 | 0.428326 | 0.431074 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=96768 avg_ns=96768 max_ns=96768
  fuse_op.create: count=16 total_ns=2602742 avg_ns=162671 max_ns=252110
  fuse_op.flush: count=16 total_ns=889614473 avg_ns=55600904 max_ns=87858008
  fuse_op.getattr: count=32769 total_ns=586366162 avg_ns=17893 max_ns=3053970
  fuse_op.getxattr: count=32784 total_ns=930734028 avg_ns=28389 max_ns=1196959
  fuse_op.lookup: count=149 total_ns=4456910 avg_ns=29912 max_ns=124479
  fuse_op.open: count=16 total_ns=552353 avg_ns=34522 max_ns=65358
  fuse_op.read: count=32240 total_ns=799273285 avg_ns=24791 max_ns=2012765
  fuse_op.release: count=32 total_ns=80002 avg_ns=2500 max_ns=8858
  fuse_op.setattr: count=16 total_ns=1049762 avg_ns=65610 max_ns=109129
  fuse_op.statfs: count=2 total_ns=6797 avg_ns=3398 max_ns=4945
  fuse_op.unlink: count=16 total_ns=30760570 avg_ns=1922535 max_ns=3738251
  fuse_op.write: count=32768 total_ns=1335016338 avg_ns=40741 max_ns=3398833
  policy_decision: count=360104 total_ns=186540029 avg_ns=518 max_ns=386595
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
  matcher_candidate_order.descendant: count=294424 total_ns=47342607 avg_ns=160 max_ns=1725407
  matcher_candidate_order.path: count=1014632 total_ns=216537616 avg_ns=213 max_ns=275037
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=84534309 avg_ns=287 max_ns=275037
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=55538651 avg_ns=188 max_ns=208481
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=17906013 avg_ns=272 max_ns=60768
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=47342607 avg_ns=160 max_ns=1725407
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=47364246 avg_ns=160 max_ns=62590
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=11194397 avg_ns=170 max_ns=29612
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=9994431 avg_ns=76 max_ns=25734
  state_read_lock_hold: count=130791 total_ns=13855798 avg_ns=105 max_ns=65347
  state_write_lock_wait: count=211 total_ns=13716 avg_ns=65 max_ns=1494
  state_write_lock_hold: count=211 total_ns=350885 avg_ns=1662 max_ns=13085
  open_confined_openat2: count=163768 total_ns=163581987 avg_ns=998 max_ns=870305
  open_like.pre_open_guard.access: count=1 total_ns=81140 avg_ns=81140 max_ns=81140
  open_like.pre_open_guard.open: count=16 total_ns=363729 avg_ns=22733 max_ns=43260
  open_like.post_open_revalidation.access: count=1 total_ns=8352 avg_ns=8352 max_ns=8352
  open_like.post_open_revalidation.open: count=16 total_ns=123410 avg_ns=7713 max_ns=15785
  stat_child_no_follow: count=130919 total_ns=233405388 avg_ns=1782 max_ns=872025
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2002730 avg_ns=15 max_ns=25652
  stat_child_no_follow.host_fstat: count=130837 total_ns=31913130 avg_ns=243 max_ns=196978
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=233405388 avg_ns=1782 max_ns=872025
  source_root_path: count=163655 total_ns=495162534 avg_ns=3025 max_ns=1158881
  resolved_virtual_path: count=228726 total_ns=1139308523 avg_ns=4981 max_ns=3359183
  resolved_virtual_path_from_path: count=130853 total_ns=996832195 avg_ns=7617 max_ns=3359183
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=919115825 avg_ns=7024 max_ns=3357625
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=816504494 avg_ns=2081 max_ns=3356472
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=61960502 avg_ns=157 max_ns=68452
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=69319115 avg_ns=529 max_ns=2973706
  resolved_virtual_path_from_open_fd: count=97873 total_ns=142476328 avg_ns=1455 max_ns=123368
  read_handle_snapshot: count=32240 total_ns=10912982 avg_ns=338 max_ns=54862
  read_guard_path: count=32240 total_ns=747596138 avg_ns=23188 max_ns=2006329
  read_io: count=32240 total_ns=30761400 avg_ns=954 max_ns=185466
  write_handle_snapshot: count=32768 total_ns=11309231 avg_ns=345 max_ns=65791
  write_guard_mutation: count=32768 total_ns=1189106331 avg_ns=36288 max_ns=3393310
  write_io: count=32768 total_ns=122692904 avg_ns=3744 max_ns=1869186
  file_sync.flush: count=16 total_ns=889599866 avg_ns=55599991 max_ns=87857158
  read_size_bucket.0_4k: count=32236 total_ns=30756135 avg_ns=954 max_ns=185466
  read_size_bucket.4k_64k: count=4 total_ns=5265 avg_ns=1316 max_ns=1518
  write_size_bucket.0_4k: count=32768 total_ns=122692904 avg_ns=3744 max_ns=1869186
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
