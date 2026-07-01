# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:58.140606+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-concurrency --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 4096 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+97 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+97 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
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
| concurrent_rand_read_write_4k | 0.063513 | 0.447359 | 7.044 | 0.482601 | 0.491705 | 0.498988 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=45377 avg_ns=45377 max_ns=45377
  fuse_op.create: count=28 total_ns=2591097 avg_ns=92539 max_ns=154434
  fuse_op.flush: count=28 total_ns=6865976572 avg_ns=245213449 max_ns=302491909
  fuse_op.getattr: count=114689 total_ns=584796356 avg_ns=5098 max_ns=1382115
  fuse_op.getxattr: count=114716 total_ns=1608170590 avg_ns=14018 max_ns=2040030
  fuse_op.lookup: count=257 total_ns=2314068 avg_ns=9004 max_ns=37044
  fuse_op.open: count=28 total_ns=609869 avg_ns=21781 max_ns=44124
  fuse_op.read: count=111314 total_ns=184366346 avg_ns=1656 max_ns=527492
  fuse_op.release: count=56 total_ns=195567 avg_ns=3492 max_ns=23499
  fuse_op.setattr: count=28 total_ns=1061392 avg_ns=37906 max_ns=56438
  fuse_op.statfs: count=2 total_ns=8597 avg_ns=4298 max_ns=5196
  fuse_op.unlink: count=28 total_ns=86733170 avg_ns=3097613 max_ns=6799314
  fuse_op.write: count=114688 total_ns=431598222 avg_ns=3763 max_ns=786236
  policy_decision: count=345275 total_ns=199441050 avg_ns=577 max_ns=1176322
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
  matcher_candidate_order.descendant: count=345023 total_ns=67309282 avg_ns=195 max_ns=51444
  matcher_candidate_order.path: count=1035573 total_ns=204973614 avg_ns=197 max_ns=380600
  matcher_candidate_order_by_source.hidden.path: count=345023 total_ns=68158828 avg_ns=197 max_ns=380600
  matcher_candidate_order_by_source.internal_hidden.path: count=345023 total_ns=71329381 avg_ns=206 max_ns=271905
  matcher_candidate_order_by_source.readonly.path: count=252 total_ns=39521 avg_ns=156 max_ns=386
  matcher_candidate_order_by_source.visible.descendant: count=345023 total_ns=67309282 avg_ns=195 max_ns=51444
  matcher_candidate_order_by_source.visible.path: count=345023 total_ns=65406102 avg_ns=189 max_ns=43812
  matcher_candidate_order_by_source.writable.path: count=252 total_ns=39782 avg_ns=157 max_ns=490
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5520216
  matcher_candidate_order_ancestor_steps.descendant: count=1379606
  matcher_candidate_order_ancestor_steps.path: count=4140610
  state_read_lock_wait: count=455805 total_ns=26690039 avg_ns=58 max_ns=18118
  state_read_lock_hold: count=455805 total_ns=42256821 avg_ns=92 max_ns=136723
  state_write_lock_wait: count=367 total_ns=86575 avg_ns=235 max_ns=20125
  state_write_lock_hold: count=367 total_ns=547800 avg_ns=1492 max_ns=15479
  open_confined_openat2: count=344771 total_ns=274196662 avg_ns=795 max_ns=2011570
  open_like.pre_open_guard.access: count=1 total_ns=257 avg_ns=257 max_ns=257
  open_like.pre_open_guard.open: count=28 total_ns=5768 avg_ns=206 max_ns=413
  open_like.post_open_revalidation.access: count=1 total_ns=28267 avg_ns=28267 max_ns=28267
  open_like.post_open_revalidation.open: count=28 total_ns=507837 avg_ns=18137 max_ns=38858
  stat_child_no_follow: count=229942 total_ns=336458629 avg_ns=1463 max_ns=2014107
  stat_child_no_follow.attr_conversion: count=229828 total_ns=3925190 avg_ns=17 max_ns=4161
  stat_child_no_follow.host_fstat: count=229828 total_ns=50856522 avg_ns=221 max_ns=61312
  stat_child_no_follow_context.path_guard_or_metadata: count=229942 total_ns=336458629 avg_ns=1463 max_ns=2014107
  source_root_path: count=114857 total_ns=295386640 avg_ns=2571 max_ns=266088
  resolved_virtual_path: count=114913 total_ns=147025727 avg_ns=1279 max_ns=198682
  resolved_virtual_path_from_path: count=56 total_ns=303340 avg_ns=5416 max_ns=8218
  resolved_virtual_path_from_path_component_walk: count=56 total_ns=268329 avg_ns=4791 max_ns=7010
  resolved_virtual_path_from_path_canonicalize: count=112 total_ns=203513 avg_ns=1817 max_ns=4815
  resolved_virtual_path_from_path_source_root_confinement: count=112 total_ns=38715 avg_ns=345 max_ns=1434
  resolved_virtual_path_from_path_virtual_conversion: count=56 total_ns=29631 avg_ns=529 max_ns=804
  resolved_virtual_path_from_open_fd: count=114857 total_ns=146722387 avg_ns=1277 max_ns=198682
  read_handle_snapshot: count=111314 total_ns=31273406 avg_ns=280 max_ns=57175
  read_guard_path: count=111314 total_ns=2148938 avg_ns=19 max_ns=1245
  read_io: count=111314 total_ns=123096731 avg_ns=1105 max_ns=526870
  write_handle_snapshot: count=114688 total_ns=33012168 avg_ns=287 max_ns=30217
  write_guard_mutation: count=114688 total_ns=2219039 avg_ns=19 max_ns=18652
  write_io: count=114688 total_ns=366573291 avg_ns=3196 max_ns=785584
  file_sync.flush: count=28 total_ns=6865913237 avg_ns=245211187 max_ns=302490967
  read_size_bucket.0_4k: count=111167 total_ns=122704058 avg_ns=1103 max_ns=526870
  read_size_bucket.4k_64k: count=147 total_ns=392673 avg_ns=2671 max_ns=7820
  write_size_bucket.0_4k: count=114688 total_ns=366573291 avg_ns=3196 max_ns=785584
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
