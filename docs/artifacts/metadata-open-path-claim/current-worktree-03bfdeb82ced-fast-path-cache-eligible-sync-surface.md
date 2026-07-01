# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:32.590221+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set sync-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 64 --small-files 1024 --dir-entries 2048 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 1024 --sync-4k-fsync-every 32 --hidden-misses 512 --matcher-extra-rules 0 --matcher-misses 512 --symlink-parent-mutations 64 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+57 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+57 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `sync-surface`
- comparable_workloads: `sync_flush_only, sync_fsync_only, sync_release_flush`
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
| sync_flush_only | 0.000273 | 0.007261 | 26.605 | 0.008445 | 0.008615 | 0.008750 |
| sync_fsync_only | 0.000087 | 0.002167 | 24.910 | 0.002603 | 0.002757 | 0.002881 |
| sync_release_flush | 0.000184 | 0.006244 | 33.986 | 0.006907 | 0.007161 | 0.007365 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47165 avg_ns=47165 max_ns=47165
  fuse_op.create: count=39 total_ns=2138870 avg_ns=54842 max_ns=99592
  fuse_op.flush: count=1677 total_ns=8504251 avg_ns=5071 max_ns=124297
  fuse_op.fsync: count=832 total_ns=3515712 avg_ns=4225 max_ns=43628
  fuse_op.getattr: count=833 total_ns=3038783 avg_ns=3647 max_ns=17370
  fuse_op.getxattr: count=4134 total_ns=41605900 avg_ns=10064 max_ns=46729
  fuse_op.lookup: count=5153 total_ns=20285699 avg_ns=3936 max_ns=46559
  fuse_op.open: count=1638 total_ns=28262399 avg_ns=17254 max_ns=40795
  fuse_op.release: count=1677 total_ns=941575 avg_ns=561 max_ns=16825
  fuse_op.statfs: count=2 total_ns=4983 avg_ns=2491 max_ns=2955
  fuse_op.unlink: count=39 total_ns=1583306 avg_ns=40597 max_ns=74155
  fuse_op.write: count=2496 total_ns=3665325 avg_ns=1468 max_ns=11500
  policy_decision: count=25019 total_ns=12133855 avg_ns=484 max_ns=8731
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
  matcher_candidate_order.descendant: count=21470 total_ns=3581829 avg_ns=166 max_ns=1657
  matcher_candidate_order.path: count=71508 total_ns=12099477 avg_ns=169 max_ns=5897
  matcher_candidate_order_by_source.hidden.path: count=21470 total_ns=3548269 avg_ns=165 max_ns=4375
  matcher_candidate_order_by_source.internal_hidden.path: count=21470 total_ns=3804323 avg_ns=177 max_ns=5897
  matcher_candidate_order_by_source.readonly.path: count=3549 total_ns=609660 avg_ns=171 max_ns=521
  matcher_candidate_order_by_source.visible.descendant: count=21470 total_ns=3581829 avg_ns=166 max_ns=1657
  matcher_candidate_order_by_source.visible.path: count=21470 total_ns=3529404 avg_ns=164 max_ns=1666
  matcher_candidate_order_by_source.writable.path: count=3549 total_ns=607821 avg_ns=171 max_ns=664
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=349720
  matcher_candidate_order_ancestor_steps.descendant: count=80410
  matcher_candidate_order_ancestor_steps.path: count=269310
  state_read_lock_wait: count=16842 total_ns=411465 avg_ns=24 max_ns=379
  state_read_lock_hold: count=16842 total_ns=1186380 avg_ns=70 max_ns=2841
  state_write_lock_wait: count=8505 total_ns=220454 avg_ns=25 max_ns=364
  state_write_lock_hold: count=8505 total_ns=1719874 avg_ns=202 max_ns=15929
  open_confined_openat2: count=17921 total_ns=13331186 avg_ns=743 max_ns=11917
  open_like.pre_open_guard.access: count=1 total_ns=225 avg_ns=225 max_ns=225
  open_like.pre_open_guard.open: count=1638 total_ns=10588141 avg_ns=6464 max_ns=13451
  open_like.post_open_revalidation.access: count=1 total_ns=33132 avg_ns=33132 max_ns=33132
  open_like.post_open_revalidation.open: count=1638 total_ns=13927642 avg_ns=8502 max_ns=20178
  stat_child_no_follow: count=12070 total_ns=16745136 avg_ns=1387 max_ns=18698
  stat_child_no_follow.attr_conversion: count=11912 total_ns=205814 avg_ns=17 max_ns=20
  stat_child_no_follow.host_fstat: count=11912 total_ns=2479184 avg_ns=208 max_ns=12338
  stat_child_no_follow_context.path_guard_or_metadata: count=12070 total_ns=16745136 avg_ns=1387 max_ns=18698
  source_root_path: count=5890 total_ns=11457498 avg_ns=1945 max_ns=25098
  resolved_virtual_path: count=5968 total_ns=7345179 avg_ns=1230 max_ns=9019
  resolved_virtual_path_from_path: count=78 total_ns=279595 avg_ns=3584 max_ns=8434
  resolved_virtual_path_from_path_component_walk: count=78 total_ns=240473 avg_ns=3082 max_ns=8026
  resolved_virtual_path_from_path_canonicalize: count=156 total_ns=172307 avg_ns=1104 max_ns=3237
  resolved_virtual_path_from_path_source_root_confinement: count=156 total_ns=30664 avg_ns=196 max_ns=1059
  resolved_virtual_path_from_path_virtual_conversion: count=78 total_ns=31645 avg_ns=405 max_ns=699
  resolved_virtual_path_from_open_fd: count=5890 total_ns=7065584 avg_ns=1199 max_ns=9019
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=2496 total_ns=521712 avg_ns=209 max_ns=1556
  write_guard_mutation: count=2496 total_ns=46610 avg_ns=18 max_ns=336
  write_io: count=2496 total_ns=2688910 avg_ns=1077 max_ns=10189
  file_sync.flush: count=1677 total_ns=8091494 avg_ns=4824 max_ns=123529
  file_sync.fsync: count=832 total_ns=3320899 avg_ns=3991 max_ns=41444
  write_size_bucket.0_4k: count=2496 total_ns=2688910 avg_ns=1077 max_ns=10189
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
  invalidations: count=78 invalidated_entries=39 evicted_entries=0 scanned_entries=234
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
