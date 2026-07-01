# ScreenFS benchmark result

- timestamp: `2026-07-01T05:16:08.708164+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+100 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+100 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.022372 | 0.099909 | 4.466 | 0.100369 | 0.100458 | 0.100530 |
| seq_write | 0.070013 | 0.151119 | 2.158 | 0.162299 | 0.163354 | 0.164198 |
| small_read | 0.000341 | 0.008503 | 24.945 | 0.009246 | 0.009413 | 0.009547 |
| small_write | 0.000809 | 0.021623 | 26.737 | 0.022058 | 0.022063 | 0.022066 |
| rand_read_4k | 0.012273 | 0.211535 | 17.236 | 0.212175 | 0.212183 | 0.212190 |
| rand_write_4k | 0.023462 | 0.648756 | 27.652 | 0.655968 | 0.656482 | 0.656893 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=40437 avg_ns=40437 max_ns=40437
  fuse_op.create: count=21 total_ns=1563748 avg_ns=74464 max_ns=104874
  fuse_op.flush: count=21 total_ns=2552177212 avg_ns=121532248 max_ns=337574866
  fuse_op.getattr: count=123684 total_ns=360272564 avg_ns=2912 max_ns=68721
  fuse_op.getxattr: count=123655 total_ns=1026764405 avg_ns=8303 max_ns=340443
  fuse_op.lookup: count=236 total_ns=1446740 avg_ns=6130 max_ns=23320
  fuse_op.open: count=21 total_ns=452027 avg_ns=21525 max_ns=34913
  fuse_op.read: count=116872 total_ns=278830088 avg_ns=2385 max_ns=387289
  fuse_op.release: count=42 total_ns=105500 avg_ns=2511 max_ns=4552
  fuse_op.setattr: count=7 total_ns=260907 avg_ns=37272 max_ns=54591
  fuse_op.statfs: count=2 total_ns=4442 avg_ns=2221 max_ns=2327
  fuse_op.unlink: count=21 total_ns=125608745 avg_ns=5981368 max_ns=11685304
  fuse_op.write: count=123648 total_ns=779983622 avg_ns=6308 max_ns=6281386
  policy_decision: count=371805 total_ns=157514949 avg_ns=423 max_ns=331711
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
  matcher_candidate_order.descendant: count=371644 total_ns=51633393 avg_ns=138 max_ns=25991
  matcher_candidate_order.path: count=1115254 total_ns=159494316 avg_ns=143 max_ns=64498
  matcher_candidate_order_by_source.hidden.path: count=371644 total_ns=52141312 avg_ns=140 max_ns=18239
  matcher_candidate_order_by_source.internal_hidden.path: count=371644 total_ns=55923843 avg_ns=150 max_ns=48199
  matcher_candidate_order_by_source.readonly.path: count=161 total_ns=19763 avg_ns=122 max_ns=178
  matcher_candidate_order_by_source.visible.descendant: count=371644 total_ns=51633393 avg_ns=138 max_ns=25991
  matcher_candidate_order_by_source.visible.path: count=371644 total_ns=51389900 avg_ns=138 max_ns=64498
  matcher_candidate_order_by_source.writable.path: count=161 total_ns=19498 avg_ns=121 max_ns=181
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5945788
  matcher_candidate_order_ancestor_steps.descendant: count=1486167
  matcher_candidate_order_ancestor_steps.path: count=4459621
  state_read_lock_wait: count=488187 total_ns=8456643 avg_ns=17 max_ns=14759
  state_read_lock_hold: count=488187 total_ns=32345178 avg_ns=66 max_ns=28103
  state_write_lock_wait: count=318 total_ns=10908 avg_ns=34 max_ns=324
  state_write_lock_hold: count=318 total_ns=275210 avg_ns=865 max_ns=5656
  open_confined_openat2: count=371483 total_ns=180167942 avg_ns=484 max_ns=60702
  open_like.pre_open_guard.access: count=1 total_ns=258 avg_ns=258 max_ns=258
  open_like.pre_open_guard.open: count=21 total_ns=3303 avg_ns=157 max_ns=453
  open_like.post_open_revalidation.access: count=1 total_ns=29563 avg_ns=29563 max_ns=29563
  open_like.post_open_revalidation.open: count=21 total_ns=400050 avg_ns=19050 max_ns=31473
  stat_child_no_follow: count=247757 total_ns=241746509 avg_ns=975 max_ns=61275
  stat_child_no_follow.attr_conversion: count=247671 total_ns=3214868 avg_ns=12 max_ns=4069
  stat_child_no_follow.host_fstat: count=247671 total_ns=40141425 avg_ns=162 max_ns=16500
  stat_child_no_follow_context.path_guard_or_metadata: count=247757 total_ns=241746509 avg_ns=975 max_ns=61275
  source_root_path: count=123747 total_ns=231226314 avg_ns=1868 max_ns=328676
  resolved_virtual_path: count=123789 total_ns=115831001 avg_ns=935 max_ns=22262
  resolved_virtual_path_from_path: count=42 total_ns=213435 avg_ns=5081 max_ns=6333
  resolved_virtual_path_from_path_component_walk: count=42 total_ns=188238 avg_ns=4481 max_ns=5411
  resolved_virtual_path_from_path_canonicalize: count=84 total_ns=138783 avg_ns=1652 max_ns=4063
  resolved_virtual_path_from_path_source_root_confinement: count=84 total_ns=29689 avg_ns=353 max_ns=957
  resolved_virtual_path_from_path_virtual_conversion: count=42 total_ns=22143 avg_ns=527 max_ns=857
  resolved_virtual_path_from_open_fd: count=123747 total_ns=115617566 avg_ns=934 max_ns=22262
  read_handle_snapshot: count=116872 total_ns=21493481 avg_ns=183 max_ns=28357
  read_guard_path: count=116872 total_ns=1537995 avg_ns=13 max_ns=5250
  read_io: count=116872 total_ns=236232901 avg_ns=2021 max_ns=386724
  write_handle_snapshot: count=123648 total_ns=23078230 avg_ns=186 max_ns=19434
  write_guard_mutation: count=123648 total_ns=1607454 avg_ns=13 max_ns=4675
  write_io: count=123648 total_ns=734919109 avg_ns=5943 max_ns=6279942
  file_sync.flush: count=21 total_ns=2552154667 avg_ns=121531174 max_ns=337573816
  read_size_bucket.0_4k: count=100800 total_ns=78733538 avg_ns=781 max_ns=386724
  read_size_bucket.4k_64k: count=1505 total_ns=2445827 avg_ns=1625 max_ns=12995
  read_size_bucket.64k_1m: count=14567 total_ns=155053536 avg_ns=10644 max_ns=108809
  write_size_bucket.0_4k: count=121856 total_ns=288361345 avg_ns=2366 max_ns=318416
  write_size_bucket.64k_1m: count=1792 total_ns=446557764 avg_ns=249195 max_ns=6279942
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
  invalidations: count=42 invalidated_entries=21 evicted_entries=0 scanned_entries=126
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
