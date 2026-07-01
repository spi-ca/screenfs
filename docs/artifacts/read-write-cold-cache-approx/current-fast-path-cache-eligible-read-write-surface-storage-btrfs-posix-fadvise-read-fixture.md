# ScreenFS benchmark result

- timestamp: `2026-07-01T05:14:48.853934+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --cache-control posix-fadvise-read-fixture --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+70 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+70 more)`
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
- cache_control: `posix-fadvise-read-fixture`
- cache_control_scope: `requested non-root read-side cold-cache approximation for benchmark-owned source/.screenfs-bench read fixtures`
- cache_control_timing_applied: `True`
- cache_control_applications: `42`
- cache_control_notes: `apply os.posix_fadvise(..., POSIX_FADV_DONTNEED) to selected backing source fixture files before each warmup and measured sample ; cache control only applies to read-side fixture workloads`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.065070 | 0.164133 | 2.522 | 0.187636 | 0.190308 | 0.192446 |
| seq_write | 0.068535 | 0.206913 | 3.019 | 0.218675 | 0.222262 | 0.225131 |
| small_read | 0.002438 | 0.013368 | 5.483 | 0.014209 | 0.014427 | 0.014601 |
| small_write | 0.001405 | 0.029607 | 21.077 | 0.030618 | 0.030656 | 0.030686 |
| rand_read_4k | 0.773248 | 1.122509 | 1.452 | 1.123859 | 1.124121 | 1.124330 |
| rand_write_4k | 0.022295 | 0.794267 | 35.626 | 0.862655 | 0.885246 | 0.903318 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=79757 avg_ns=79757 max_ns=79757
  fuse_op.create: count=21 total_ns=2286532 avg_ns=108882 max_ns=216172
  fuse_op.flush: count=21 total_ns=2495940643 avg_ns=118854316 max_ns=455600284
  fuse_op.getattr: count=123684 total_ns=516983200 avg_ns=4179 max_ns=217691
  fuse_op.getxattr: count=123655 total_ns=1448818609 avg_ns=11716 max_ns=108103
  fuse_op.lookup: count=236 total_ns=2112304 avg_ns=8950 max_ns=50646
  fuse_op.open: count=21 total_ns=564748 avg_ns=26892 max_ns=40406
  fuse_op.read: count=116872 total_ns=6151679814 avg_ns=52636 max_ns=7448431
  fuse_op.release: count=42 total_ns=179875 avg_ns=4282 max_ns=10491
  fuse_op.setattr: count=7 total_ns=246850 avg_ns=35264 max_ns=36854
  fuse_op.statfs: count=2 total_ns=6978 avg_ns=3489 max_ns=4829
  fuse_op.unlink: count=21 total_ns=194735371 avg_ns=9273112 max_ns=17344425
  fuse_op.write: count=123648 total_ns=1127104053 avg_ns=9115 max_ns=3180809
  policy_decision: count=371805 total_ns=198037589 avg_ns=532 max_ns=70913
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
  matcher_candidate_order.descendant: count=371644 total_ns=67291654 avg_ns=181 max_ns=17757
  matcher_candidate_order.path: count=1115254 total_ns=197530373 avg_ns=177 max_ns=29298
  matcher_candidate_order_by_source.hidden.path: count=371644 total_ns=63276865 avg_ns=170 max_ns=25529
  matcher_candidate_order_by_source.internal_hidden.path: count=371644 total_ns=71609562 avg_ns=192 max_ns=29298
  matcher_candidate_order_by_source.readonly.path: count=161 total_ns=24373 avg_ns=151 max_ns=520
  matcher_candidate_order_by_source.visible.descendant: count=371644 total_ns=67291654 avg_ns=181 max_ns=17757
  matcher_candidate_order_by_source.visible.path: count=371644 total_ns=62595234 avg_ns=168 max_ns=21614
  matcher_candidate_order_by_source.writable.path: count=161 total_ns=24339 avg_ns=151 max_ns=463
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5945788
  matcher_candidate_order_ancestor_steps.descendant: count=1486167
  matcher_candidate_order_ancestor_steps.path: count=4459621
  state_read_lock_wait: count=488187 total_ns=12464276 avg_ns=25 max_ns=7580
  state_read_lock_hold: count=488187 total_ns=49521124 avg_ns=101 max_ns=37222
  state_write_lock_wait: count=318 total_ns=15217 avg_ns=47 max_ns=692
  state_write_lock_hold: count=318 total_ns=415438 avg_ns=1306 max_ns=11028
  open_confined_openat2: count=371483 total_ns=356237426 avg_ns=958 max_ns=204932
  open_like.pre_open_guard.access: count=1 total_ns=378 avg_ns=378 max_ns=378
  open_like.pre_open_guard.open: count=21 total_ns=4673 avg_ns=222 max_ns=407
  open_like.post_open_revalidation.access: count=1 total_ns=54359 avg_ns=54359 max_ns=54359
  open_like.post_open_revalidation.open: count=21 total_ns=476197 avg_ns=22676 max_ns=33961
  stat_child_no_follow: count=247757 total_ns=449108230 avg_ns=1812 max_ns=210264
  stat_child_no_follow.attr_conversion: count=247671 total_ns=4317179 avg_ns=17 max_ns=4136
  stat_child_no_follow.host_fstat: count=247671 total_ns=61693258 avg_ns=249 max_ns=37962
  stat_child_no_follow_context.path_guard_or_metadata: count=247757 total_ns=449108230 avg_ns=1812 max_ns=210264
  source_root_path: count=123747 total_ns=362558841 avg_ns=2929 max_ns=71826
  resolved_virtual_path: count=123789 total_ns=167780257 avg_ns=1355 max_ns=41235
  resolved_virtual_path_from_path: count=42 total_ns=270464 avg_ns=6439 max_ns=8326
  resolved_virtual_path_from_path_component_walk: count=42 total_ns=235934 avg_ns=5617 max_ns=7367
  resolved_virtual_path_from_path_canonicalize: count=84 total_ns=171247 avg_ns=2038 max_ns=4599
  resolved_virtual_path_from_path_source_root_confinement: count=84 total_ns=36583 avg_ns=435 max_ns=1751
  resolved_virtual_path_from_path_virtual_conversion: count=42 total_ns=28012 avg_ns=666 max_ns=1026
  resolved_virtual_path_from_open_fd: count=123747 total_ns=167509793 avg_ns=1353 max_ns=41235
  read_handle_snapshot: count=116872 total_ns=26804857 avg_ns=229 max_ns=15771
  read_guard_path: count=116872 total_ns=2279362 avg_ns=19 max_ns=22301
  read_io: count=116872 total_ns=6100122110 avg_ns=52194 max_ns=7445433
  write_handle_snapshot: count=123648 total_ns=29818058 avg_ns=241 max_ns=14005
  write_guard_mutation: count=123648 total_ns=2232547 avg_ns=18 max_ns=2462
  write_io: count=123648 total_ns=1072358959 avg_ns=8672 max_ns=3176460
  file_sync.flush: count=21 total_ns=2495890156 avg_ns=118851912 max_ns=455596976
  read_size_bucket.0_4k: count=100800 total_ns=5563635303 avg_ns=55194 max_ns=4069690
  read_size_bucket.4k_64k: count=1505 total_ns=122512773 avg_ns=81403 max_ns=1055906
  read_size_bucket.64k_1m: count=14567 total_ns=413974034 avg_ns=28418 max_ns=7445433
  write_size_bucket.0_4k: count=121856 total_ns=513785535 avg_ns=4216 max_ns=786623
  write_size_bucket.64k_1m: count=1792 total_ns=558573424 avg_ns=311703 max_ns=3176460
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
