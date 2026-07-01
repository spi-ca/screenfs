# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:52.848895+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+94 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+94 more)`
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
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.047499 | 0.172900 | 3.640 | 0.174530 | 0.174532 | 0.174533 |
| seq_write | 0.086128 | 0.153395 | 1.781 | 0.209467 | 0.211175 | 0.212541 |
| small_read | 0.000739 | 0.017574 | 23.794 | 0.018286 | 0.018497 | 0.018666 |
| small_write | 0.001350 | 0.049408 | 36.595 | 0.050238 | 0.050484 | 0.050680 |
| rand_read_4k | 0.024393 | 0.536805 | 22.007 | 0.541979 | 0.542182 | 0.542344 |
| rand_write_4k | 0.039345 | 1.081174 | 27.479 | 1.283895 | 1.304806 | 1.321536 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73188 avg_ns=73188 max_ns=73188
  fuse_op.create: count=21 total_ns=2166364 avg_ns=103160 max_ns=158382
  fuse_op.flush: count=21 total_ns=2385018642 avg_ns=113572316 max_ns=338094005
  fuse_op.getattr: count=123684 total_ns=1312264777 avg_ns=10609 max_ns=642970
  fuse_op.getxattr: count=123655 total_ns=2075816599 avg_ns=16787 max_ns=978368
  fuse_op.lookup: count=236 total_ns=4165361 avg_ns=17649 max_ns=69473
  fuse_op.open: count=21 total_ns=377980 avg_ns=17999 max_ns=26955
  fuse_op.read: count=116872 total_ns=2063021765 avg_ns=17651 max_ns=667732
  fuse_op.release: count=42 total_ns=116791 avg_ns=2780 max_ns=5611
  fuse_op.setattr: count=7 total_ns=377533 avg_ns=53933 max_ns=59552
  fuse_op.statfs: count=2 total_ns=3928 avg_ns=1964 max_ns=2109
  fuse_op.unlink: count=21 total_ns=148065117 avg_ns=7050719 max_ns=15295993
  fuse_op.write: count=123648 total_ns=3355307730 avg_ns=27135 max_ns=5596923
  policy_decision: count=1347459 total_ns=673730280 avg_ns=500 max_ns=330401
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
  matcher_candidate_order.descendant: count=1100002 total_ns=158007953 avg_ns=143 max_ns=257786
  matcher_candidate_order.path: count=3794920 total_ns=682812117 avg_ns=179 max_ns=267095
  matcher_candidate_order_by_source.hidden.path: count=1100002 total_ns=262946050 avg_ns=239 max_ns=54531
  matcher_candidate_order_by_source.internal_hidden.path: count=1100002 total_ns=168402508 avg_ns=153 max_ns=267095
  matcher_candidate_order_by_source.readonly.path: count=247457 total_ns=57329373 avg_ns=231 max_ns=85182
  matcher_candidate_order_by_source.visible.descendant: count=1100002 total_ns=158007953 avg_ns=143 max_ns=257786
  matcher_candidate_order_by_source.visible.path: count=1100002 total_ns=158671462 avg_ns=144 max_ns=18281
  matcher_candidate_order_by_source.writable.path: count=247457 total_ns=35462724 avg_ns=143 max_ns=45880
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1347459
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1347459
  matcher_candidate_order_ancestor_steps: count=19577872
  matcher_candidate_order_ancestor_steps.descendant: count=4399596
  matcher_candidate_order_ancestor_steps.path: count=15178276
  state_read_lock_wait: count=488187 total_ns=14366096 avg_ns=29 max_ns=8032
  state_read_lock_hold: count=488187 total_ns=34755667 avg_ns=71 max_ns=1059459
  state_write_lock_wait: count=318 total_ns=10722 avg_ns=33 max_ns=414
  state_write_lock_hold: count=318 total_ns=357142 avg_ns=1123 max_ns=16792
  open_confined_openat2: count=612067 total_ns=330070248 avg_ns=539 max_ns=296650
  open_like.pre_open_guard.access: count=1 total_ns=57761 avg_ns=57761 max_ns=57761
  open_like.pre_open_guard.open: count=21 total_ns=253275 avg_ns=12060 max_ns=17229
  open_like.post_open_revalidation.access: count=1 total_ns=9223 avg_ns=9223 max_ns=9223
  open_like.post_open_revalidation.open: count=21 total_ns=71565 avg_ns=3407 max_ns=5535
  stat_child_no_follow: count=488341 total_ns=505980722 avg_ns=1036 max_ns=321868
  stat_child_no_follow.attr_conversion: count=488234 total_ns=6552050 avg_ns=13 max_ns=5143
  stat_child_no_follow.host_fstat: count=488234 total_ns=87234673 avg_ns=178 max_ns=319519
  stat_child_no_follow_context.path_guard_or_metadata: count=488341 total_ns=505980722 avg_ns=1036 max_ns=321868
  source_root_path: count=611919 total_ns=1129111660 avg_ns=1845 max_ns=814724
  resolved_virtual_path: count=852522 total_ns=3176244309 avg_ns=3725 max_ns=337896
  resolved_virtual_path_from_path: count=488255 total_ns=2826994772 avg_ns=5789 max_ns=337896
  resolved_virtual_path_from_path_component_walk: count=488255 total_ns=2607935098 avg_ns=5341 max_ns=330158
  resolved_virtual_path_from_path_canonicalize: count=1464402 total_ns=2237220083 avg_ns=1527 max_ns=329068
  resolved_virtual_path_from_path_source_root_confinement: count=1464402 total_ns=251194379 avg_ns=171 max_ns=324363
  resolved_virtual_path_from_path_virtual_conversion: count=488255 total_ns=195152105 avg_ns=399 max_ns=332086
  resolved_virtual_path_from_open_fd: count=364267 total_ns=349249537 avg_ns=958 max_ns=316794
  read_handle_snapshot: count=116872 total_ns=24214240 avg_ns=207 max_ns=325120
  read_guard_path: count=116872 total_ns=1595494897 avg_ns=13651 max_ns=341398
  read_io: count=116872 total_ns=424181322 avg_ns=3629 max_ns=94901
  write_handle_snapshot: count=123648 total_ns=26680000 avg_ns=215 max_ns=1059919
  write_guard_mutation: count=123648 total_ns=2477546214 avg_ns=20037 max_ns=861896
  write_io: count=123648 total_ns=829770055 avg_ns=6710 max_ns=5568660
  file_sync.flush: count=21 total_ns=2384988903 avg_ns=113570900 max_ns=338092806
  read_size_bucket.0_4k: count=100800 total_ns=95453773 avg_ns=946 max_ns=94901
  read_size_bucket.4k_64k: count=1505 total_ns=3170406 avg_ns=2106 max_ns=11243
  read_size_bucket.64k_1m: count=14567 total_ns=325557143 avg_ns=22348 max_ns=74736
  write_size_bucket.0_4k: count=121856 total_ns=338053030 avg_ns=2774 max_ns=390481
  write_size_bucket.64k_1m: count=1792 total_ns=491717025 avg_ns=274395 max_ns=5568660
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
