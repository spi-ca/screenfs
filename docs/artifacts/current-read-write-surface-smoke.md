# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:11.671175+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 1 --warmups 1 --read-mib 8 --write-mib 8 --small-io-bytes 1024 --small-io-ops 512 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 512 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-read-write-surface-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-read-write-surface-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-read-write-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+33 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+33 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
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
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.001493 | 0.005160 | 3.457 | 0.005160 | 0.005160 | 0.005160 |
| seq_write | 0.001228 | 0.004967 | 4.044 | 0.004967 | 0.004967 | 0.004967 |
| small_read | 0.000221 | 0.009143 | 41.462 | 0.009143 | 0.009143 | 0.009143 |
| small_write | 0.000322 | 0.025458 | 78.987 | 0.025458 | 0.025458 | 0.025458 |
| rand_read_4k | 0.000583 | 0.018691 | 32.083 | 0.018691 | 0.018691 | 0.018691 |
| rand_write_4k | 0.000485 | 0.026521 | 54.734 | 0.026521 | 0.026521 | 0.026521 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=43793 avg_ns=43793 max_ns=43793
  fuse_op.create: count=6 total_ns=543148 avg_ns=90524 max_ns=110790
  fuse_op.flush: count=6 total_ns=472800 avg_ns=78800 max_ns=148499
  fuse_op.getattr: count=2075 total_ns=21216105 avg_ns=10224 max_ns=37062
  fuse_op.getxattr: count=2066 total_ns=34549976 avg_ns=16723 max_ns=46722
  fuse_op.lookup: count=71 total_ns=1067632 avg_ns=15037 max_ns=56456
  fuse_op.open: count=6 total_ns=109827 avg_ns=18304 max_ns=21923
  fuse_op.read: count=1046 total_ns=18291294 avg_ns=17486 max_ns=70360
  fuse_op.release: count=12 total_ns=33317 avg_ns=2776 max_ns=5581
  fuse_op.setattr: count=2 total_ns=72348 avg_ns=36174 max_ns=37456
  fuse_op.statfs: count=2 total_ns=3795 avg_ns=1897 max_ns=2455
  fuse_op.unlink: count=6 total_ns=1093375 avg_ns=182229 max_ns=353948
  fuse_op.write: count=2064 total_ns=48655829 avg_ns=23573 max_ns=254948
  policy_decision: count=20926 total_ns=12824270 avg_ns=612 max_ns=12428
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
  matcher_candidate_order.descendant: count=16752 total_ns=2878031 avg_ns=171 max_ns=957
  matcher_candidate_order.path: count=58604 total_ns=13024651 avg_ns=222 max_ns=13647
  matcher_candidate_order_by_source.hidden.path: count=16752 total_ns=5160858 avg_ns=308 max_ns=13647
  matcher_candidate_order_by_source.internal_hidden.path: count=16752 total_ns=3040497 avg_ns=181 max_ns=4984
  matcher_candidate_order_by_source.readonly.path: count=4174 total_ns=1259457 avg_ns=301 max_ns=1808
  matcher_candidate_order_by_source.visible.descendant: count=16752 total_ns=2878031 avg_ns=171 max_ns=957
  matcher_candidate_order_by_source.visible.path: count=16752 total_ns=2854533 avg_ns=170 max_ns=1104
  matcher_candidate_order_by_source.writable.path: count=4174 total_ns=709306 avg_ns=169 max_ns=320
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=20926
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=20926
  matcher_candidate_order_ancestor_steps: count=300868
  matcher_candidate_order_ancestor_steps.descendant: count=66881
  matcher_candidate_order_ancestor_steps.path: count=233987
  state_read_lock_wait: count=7349 total_ns=181884 avg_ns=24 max_ns=1529
  state_read_lock_hold: count=7349 total_ns=599181 avg_ns=81 max_ns=916
  state_write_lock_wait: count=93 total_ns=3425 avg_ns=36 max_ns=184
  state_write_lock_hold: count=93 total_ns=101454 avg_ns=1090 max_ns=7653
  open_confined_openat2: count=9480 total_ns=7849640 avg_ns=828 max_ns=10421
  open_like.pre_open_guard.access: count=1 total_ns=36020 avg_ns=36020 max_ns=36020
  open_like.pre_open_guard.open: count=6 total_ns=65144 avg_ns=10857 max_ns=12200
  open_like.post_open_revalidation.access: count=1 total_ns=4278 avg_ns=4278 max_ns=4278
  open_like.post_open_revalidation.open: count=6 total_ns=24384 avg_ns=4064 max_ns=5254
  stat_child_no_follow: count=7393 total_ns=10851520 avg_ns=1467 max_ns=15732
  stat_child_no_follow.attr_conversion: count=7361 total_ns=127304 avg_ns=17 max_ns=20
  stat_child_no_follow.host_fstat: count=7361 total_ns=1534935 avg_ns=208 max_ns=4092
  stat_child_no_follow_context.path_guard_or_metadata: count=7393 total_ns=10851520 avg_ns=1467 max_ns=15732
  source_root_path: count=9437 total_ns=16467381 avg_ns=1744 max_ns=33561
  resolved_virtual_path: count=12570 total_ns=38892445 avg_ns=3094 max_ns=23796
  resolved_virtual_path_from_path: count=7367 total_ns=32891962 avg_ns=4464 max_ns=22168
  resolved_virtual_path_from_path_component_walk: count=7367 total_ns=29233842 avg_ns=3968 max_ns=21632
  resolved_virtual_path_from_path_canonicalize: count=21993 total_ns=24083741 avg_ns=1095 max_ns=20754
  resolved_virtual_path_from_path_source_root_confinement: count=21993 total_ns=2800655 avg_ns=127 max_ns=861
  resolved_virtual_path_from_path_virtual_conversion: count=7367 total_ns=3202971 avg_ns=434 max_ns=16157
  resolved_virtual_path_from_open_fd: count=5203 total_ns=6000483 avg_ns=1153 max_ns=23796
  read_handle_snapshot: count=1046 total_ns=221707 avg_ns=211 max_ns=1709
  read_guard_path: count=1046 total_ns=14139540 avg_ns=13517 max_ns=40490
  read_io: count=1046 total_ns=3718273 avg_ns=3554 max_ns=42805
  write_handle_snapshot: count=2064 total_ns=424179 avg_ns=205 max_ns=1379
  write_guard_mutation: count=2064 total_ns=42786406 avg_ns=20729 max_ns=53922
  write_io: count=2064 total_ns=5080802 avg_ns=2461 max_ns=214267
  file_sync.flush: count=6 total_ns=467194 avg_ns=77865 max_ns=147094
  read_size_bucket.0_4k: count=882 total_ns=1324988 avg_ns=1502 max_ns=10054
  read_size_bucket.4k_64k: count=26 total_ns=122483 avg_ns=4710 max_ns=12433
  read_size_bucket.64k_1m: count=138 total_ns=2270802 avg_ns=16455 max_ns=42805
  write_size_bucket.0_4k: count=2048 total_ns=2460994 avg_ns=1201 max_ns=11670
  write_size_bucket.64k_1m: count=16 total_ns=2619808 avg_ns=163738 max_ns=214267
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
  invalidations: count=12 invalidated_entries=6 evicted_entries=0 scanned_entries=36
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
