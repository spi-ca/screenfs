# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:34.920268+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-concurrency --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 4096 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+91 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+91 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
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
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| concurrent_rand_read_write_4k | 0.058545 | 0.774540 | 13.230 | 1.032428 | 1.061107 | 1.084049 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=74855 avg_ns=74855 max_ns=74855
  fuse_op.create: count=28 total_ns=3506970 avg_ns=125248 max_ns=206599
  fuse_op.flush: count=28 total_ns=8842039861 avg_ns=315787137 max_ns=543895506
  fuse_op.getattr: count=114689 total_ns=1758501518 avg_ns=15332 max_ns=2045812
  fuse_op.getxattr: count=114716 total_ns=2742819338 avg_ns=23909 max_ns=1298356
  fuse_op.lookup: count=257 total_ns=5454234 avg_ns=21222 max_ns=77645
  fuse_op.open: count=28 total_ns=683141 avg_ns=24397 max_ns=51852
  fuse_op.read: count=111314 total_ns=2461194017 avg_ns=22110 max_ns=2330119
  fuse_op.release: count=56 total_ns=165936 avg_ns=2963 max_ns=19670
  fuse_op.setattr: count=28 total_ns=1586192 avg_ns=56649 max_ns=93600
  fuse_op.statfs: count=2 total_ns=5598 avg_ns=2799 max_ns=3517
  fuse_op.unlink: count=28 total_ns=89145311 avg_ns=3183761 max_ns=7213745
  fuse_op.write: count=114688 total_ns=4007851581 avg_ns=34945 max_ns=2085510
  policy_decision: count=1256060 total_ns=851015002 avg_ns=677 max_ns=2019403
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
  matcher_candidate_order.descendant: count=1026432 total_ns=198581544 avg_ns=193 max_ns=1267677
  matcher_candidate_order.path: count=3538552 total_ns=857448952 avg_ns=242 max_ns=399988
  matcher_candidate_order_by_source.hidden.path: count=1026432 total_ns=339175313 avg_ns=330 max_ns=399988
  matcher_candidate_order_by_source.internal_hidden.path: count=1026432 total_ns=204669830 avg_ns=199 max_ns=71883
  matcher_candidate_order_by_source.readonly.path: count=229628 total_ns=74139273 avg_ns=322 max_ns=283033
  matcher_candidate_order_by_source.visible.descendant: count=1026432 total_ns=198581544 avg_ns=193 max_ns=1267677
  matcher_candidate_order_by_source.visible.path: count=1026432 total_ns=196053091 avg_ns=191 max_ns=388223
  matcher_candidate_order_by_source.writable.path: count=229628 total_ns=43411445 avg_ns=189 max_ns=274330
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1256060
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1256060
  matcher_candidate_order_ancestor_steps: count=18257756
  matcher_candidate_order_ancestor_steps.descendant: count=4105239
  matcher_candidate_order_ancestor_steps.path: count=14152517
  state_read_lock_wait: count=455805 total_ns=19991175 avg_ns=43 max_ns=416788
  state_read_lock_hold: count=455805 total_ns=40318596 avg_ns=88 max_ns=51911
  state_write_lock_wait: count=367 total_ns=46686 avg_ns=127 max_ns=17137
  state_write_lock_hold: count=367 total_ns=509972 avg_ns=1389 max_ns=20061
  open_confined_openat2: count=570858 total_ns=503718905 avg_ns=882 max_ns=2022217
  open_like.pre_open_guard.access: count=1 total_ns=62263 avg_ns=62263 max_ns=62263
  open_like.pre_open_guard.open: count=28 total_ns=460755 avg_ns=16455 max_ns=41301
  open_like.post_open_revalidation.access: count=1 total_ns=6986 avg_ns=6986 max_ns=6986
  open_like.post_open_revalidation.open: count=28 total_ns=143882 avg_ns=5138 max_ns=10726
  stat_child_no_follow: count=456029 total_ns=713794162 avg_ns=1565 max_ns=2023989
  stat_child_no_follow.attr_conversion: count=455887 total_ns=7620495 avg_ns=16 max_ns=7941
  stat_child_no_follow.host_fstat: count=455887 total_ns=103490694 avg_ns=227 max_ns=117252
  stat_child_no_follow_context.path_guard_or_metadata: count=456029 total_ns=713794162 avg_ns=1565 max_ns=2023989
  source_root_path: count=570661 total_ns=1476982305 avg_ns=2588 max_ns=1118397
  resolved_virtual_path: count=796774 total_ns=3801243024 avg_ns=4770 max_ns=2308098
  resolved_virtual_path_from_path: count=455915 total_ns=3346140565 avg_ns=7339 max_ns=2308098
  resolved_virtual_path_from_path_component_walk: count=455915 total_ns=3080758589 avg_ns=6757 max_ns=1623180
  resolved_virtual_path_from_path_canonicalize: count=1367319 total_ns=2666304799 avg_ns=1950 max_ns=1621464
  resolved_virtual_path_from_path_source_root_confinement: count=1367319 total_ns=261366165 avg_ns=191 max_ns=62895
  resolved_virtual_path_from_path_virtual_conversion: count=455915 total_ns=234964220 avg_ns=515 max_ns=291716
  resolved_virtual_path_from_open_fd: count=340859 total_ns=455102459 avg_ns=1335 max_ns=553422
  read_handle_snapshot: count=111314 total_ns=28704892 avg_ns=257 max_ns=417313
  read_guard_path: count=111314 total_ns=2200097640 avg_ns=19764 max_ns=2327060
  read_io: count=111314 total_ns=207050412 avg_ns=1860 max_ns=448751
  write_handle_snapshot: count=114688 total_ns=30024655 avg_ns=261 max_ns=56030
  write_guard_mutation: count=114688 total_ns=3447030026 avg_ns=30055 max_ns=2071082
  write_io: count=114688 total_ns=503965602 avg_ns=4394 max_ns=1038519
  file_sync.flush: count=28 total_ns=8841995396 avg_ns=315785549 max_ns=543893605
  read_size_bucket.0_4k: count=111167 total_ns=206404047 avg_ns=1856 max_ns=448751
  read_size_bucket.4k_64k: count=147 total_ns=646365 avg_ns=4397 max_ns=10194
  write_size_bucket.0_4k: count=114688 total_ns=503965602 avg_ns=4394 max_ns=1038519
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
  invalidations: count=56 invalidated_entries=28 evicted_entries=0 scanned_entries=350
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
