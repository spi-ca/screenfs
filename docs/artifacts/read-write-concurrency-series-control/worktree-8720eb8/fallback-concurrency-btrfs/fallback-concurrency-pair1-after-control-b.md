# ScreenFS benchmark result

- timestamp: `2026-06-24T15:13:42.218538+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rw-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency --output-json docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-after-control-b.json --output-md docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-after-control-b.md --output-svg docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-after-control-b.svg`
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
| concurrent_rand_read_write_4k | 0.062807 | 0.387002 | 6.162 | 0.393725 | 0.394566 | 0.395238 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=229537 avg_ns=229537 max_ns=229537
  fuse_op.create: count=16 total_ns=2381058 avg_ns=148816 max_ns=271009
  fuse_op.flush: count=16 total_ns=681155991 avg_ns=42572249 max_ns=78490675
  fuse_op.getattr: count=32769 total_ns=620060453 avg_ns=18922 max_ns=1847123
  fuse_op.getxattr: count=32784 total_ns=982519605 avg_ns=29969 max_ns=1174205
  fuse_op.lookup: count=149 total_ns=4772881 avg_ns=32032 max_ns=119566
  fuse_op.open: count=16 total_ns=642506 avg_ns=40156 max_ns=74890
  fuse_op.read: count=32240 total_ns=855862875 avg_ns=26546 max_ns=1667225
  fuse_op.release: count=32 total_ns=107294 avg_ns=3352 max_ns=13194
  fuse_op.setattr: count=16 total_ns=1047935 avg_ns=65495 max_ns=114984
  fuse_op.statfs: count=2 total_ns=3612 avg_ns=1806 max_ns=1806
  fuse_op.unlink: count=16 total_ns=27703884 avg_ns=1731492 max_ns=3379220
  fuse_op.write: count=32768 total_ns=1421703181 avg_ns=43386 max_ns=2556744
  policy_decision: count=360104 total_ns=190076529 avg_ns=527 max_ns=76871
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
  matcher_candidate_order.descendant: count=294424 total_ns=46208522 avg_ns=156 max_ns=48208
  matcher_candidate_order.path: count=1014632 total_ns=226759239 avg_ns=223 max_ns=473096
  matcher_candidate_order_by_source.hidden.path: count=294424 total_ns=88938593 avg_ns=302 max_ns=473096
  matcher_candidate_order_by_source.internal_hidden.path: count=294424 total_ns=58934993 avg_ns=200 max_ns=62444
  matcher_candidate_order_by_source.readonly.path: count=65680 total_ns=18717620 avg_ns=284 max_ns=45003
  matcher_candidate_order_by_source.visible.descendant: count=294424 total_ns=46208522 avg_ns=156 max_ns=48208
  matcher_candidate_order_by_source.visible.path: count=294424 total_ns=48591007 avg_ns=165 max_ns=58920
  matcher_candidate_order_by_source.writable.path: count=65680 total_ns=11577026 avg_ns=176 max_ns=58552
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=360104
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=360104
  matcher_candidate_order_ancestor_steps: count=5234956
  matcher_candidate_order_ancestor_steps.descendant: count=1177411
  matcher_candidate_order_ancestor_steps.path: count=4057545
  state_read_lock_wait: count=130791 total_ns=11223657 avg_ns=85 max_ns=61779
  state_read_lock_hold: count=130791 total_ns=14897389 avg_ns=113 max_ns=74627
  state_write_lock_wait: count=211 total_ns=11869 avg_ns=56 max_ns=1023
  state_write_lock_hold: count=211 total_ns=340703 avg_ns=1614 max_ns=10777
  open_confined_openat2: count=163768 total_ns=178998859 avg_ns=1093 max_ns=1129385
  open_like.pre_open_guard.access: count=1 total_ns=133954 avg_ns=133954 max_ns=133954
  open_like.pre_open_guard.open: count=16 total_ns=427717 avg_ns=26732 max_ns=39333
  open_like.post_open_revalidation.access: count=1 total_ns=40018 avg_ns=40018 max_ns=40018
  open_like.post_open_revalidation.open: count=16 total_ns=152187 avg_ns=9511 max_ns=29695
  stat_child_no_follow: count=130919 total_ns=249659055 avg_ns=1906 max_ns=746987
  stat_child_no_follow.attr_conversion: count=130837 total_ns=2077867 avg_ns=15 max_ns=38393
  stat_child_no_follow.host_fstat: count=130837 total_ns=34883731 avg_ns=266 max_ns=194499
  stat_child_no_follow_context.path_guard_or_metadata: count=130919 total_ns=249659055 avg_ns=1906 max_ns=746987
  source_root_path: count=163655 total_ns=533401192 avg_ns=3259 max_ns=1326474
  resolved_virtual_path: count=228726 total_ns=1198830267 avg_ns=5241 max_ns=2464006
  resolved_virtual_path_from_path: count=130853 total_ns=1044483704 avg_ns=7982 max_ns=2464006
  resolved_virtual_path_from_path_component_walk: count=130853 total_ns=967749493 avg_ns=7395 max_ns=2460277
  resolved_virtual_path_from_path_canonicalize: count=392313 total_ns=861292652 avg_ns=2195 max_ns=2458454
  resolved_virtual_path_from_path_source_root_confinement: count=392313 total_ns=64098638 avg_ns=163 max_ns=93164
  resolved_virtual_path_from_path_virtual_conversion: count=130853 total_ns=67905020 avg_ns=518 max_ns=80117
  resolved_virtual_path_from_open_fd: count=97873 total_ns=154346563 avg_ns=1577 max_ns=841596
  read_handle_snapshot: count=32240 total_ns=11823662 avg_ns=366 max_ns=25326
  read_guard_path: count=32240 total_ns=797469662 avg_ns=24735 max_ns=1660653
  read_io: count=32240 total_ns=35802990 avg_ns=1110 max_ns=1069170
  write_handle_snapshot: count=32768 total_ns=12178447 avg_ns=371 max_ns=74753
  write_guard_mutation: count=32768 total_ns=1269594841 avg_ns=38744 max_ns=2534771
  write_io: count=32768 total_ns=127734578 avg_ns=3898 max_ns=255811
  file_sync.flush: count=16 total_ns=681131388 avg_ns=42570711 max_ns=78489678
  read_size_bucket.0_4k: count=32236 total_ns=35793217 avg_ns=1110 max_ns=1069170
  read_size_bucket.4k_64k: count=4 total_ns=9773 avg_ns=2443 max_ns=3426
  write_size_bucket.0_4k: count=32768 total_ns=127734578 avg_ns=3898 max_ns=255811
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
