# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:18.566711+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --small-io-bytes 1024 --small-io-ops 512 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 512 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+85 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+85 more)`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.045251 | 0.126555 | 2.797 | 0.127433 | 0.127543 | 0.127630 |
| seq_write | 0.044518 | 0.145902 | 3.277 | 0.146572 | 0.146656 | 0.146723 |
| small_read | 0.000209 | 0.004984 | 23.876 | 0.004990 | 0.004990 | 0.004991 |
| small_write | 0.000293 | 0.012096 | 41.247 | 0.013403 | 0.013566 | 0.013697 |
| rand_read_4k | 0.000405 | 0.007472 | 18.430 | 0.007611 | 0.007628 | 0.007642 |
| rand_write_4k | 0.000576 | 0.010622 | 18.432 | 0.010833 | 0.010859 | 0.010881 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=45462 avg_ns=45462 max_ns=45462
  fuse_op.create: count=12 total_ns=850223 avg_ns=70851 max_ns=114929
  fuse_op.flush: count=12 total_ns=1044660 avg_ns=87055 max_ns=160226
  fuse_op.getattr: count=5141 total_ns=25017866 avg_ns=4866 max_ns=50695
  fuse_op.getxattr: count=5124 total_ns=70677427 avg_ns=13793 max_ns=281389
  fuse_op.lookup: count=137 total_ns=957220 avg_ns=6987 max_ns=48862
  fuse_op.open: count=12 total_ns=240731 avg_ns=20060 max_ns=35307
  fuse_op.read: count=10256 total_ns=178210728 avg_ns=17376 max_ns=89325
  fuse_op.release: count=24 total_ns=49371 avg_ns=2057 max_ns=3695
  fuse_op.setattr: count=4 total_ns=72613 avg_ns=18153 max_ns=18940
  fuse_op.statfs: count=2 total_ns=2418 avg_ns=1209 max_ns=1252
  fuse_op.unlink: count=12 total_ns=48453878 avg_ns=4037823 max_ns=12640656
  fuse_op.write: count=5120 total_ns=189088270 avg_ns=36931 max_ns=484031
  policy_decision: count=15855 total_ns=8507179 avg_ns=536 max_ns=9642
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
  matcher_candidate_order.descendant: count=15763 total_ns=3358687 avg_ns=213 max_ns=6787
  matcher_candidate_order.path: count=47473 total_ns=8932599 avg_ns=188 max_ns=12652
  matcher_candidate_order_by_source.hidden.path: count=15763 total_ns=2725648 avg_ns=172 max_ns=12652
  matcher_candidate_order_by_source.internal_hidden.path: count=15763 total_ns=3626554 avg_ns=230 max_ns=7546
  matcher_candidate_order_by_source.readonly.path: count=92 total_ns=12870 avg_ns=139 max_ns=321
  matcher_candidate_order_by_source.visible.descendant: count=15763 total_ns=3358687 avg_ns=213 max_ns=6787
  matcher_candidate_order_by_source.visible.path: count=15763 total_ns=2554752 avg_ns=162 max_ns=2499
  matcher_candidate_order_by_source.writable.path: count=92 total_ns=12775 avg_ns=138 max_ns=269
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=251896
  matcher_candidate_order_ancestor_steps.descendant: count=62814
  matcher_candidate_order_ancestor_steps.path: count=189082
  state_read_lock_wait: count=25831 total_ns=661789 avg_ns=25 max_ns=492
  state_read_lock_hold: count=25831 total_ns=3491227 avg_ns=135 max_ns=2997
  state_write_lock_wait: count=183 total_ns=7891 avg_ns=43 max_ns=522
  state_write_lock_hold: count=183 total_ns=191284 avg_ns=1045 max_ns=10408
  open_confined_openat2: count=15671 total_ns=20061088 avg_ns=1280 max_ns=64217
  open_like.pre_open_guard.access: count=1 total_ns=238 avg_ns=238 max_ns=238
  open_like.pre_open_guard.open: count=12 total_ns=1993 avg_ns=166 max_ns=434
  open_like.post_open_revalidation.access: count=1 total_ns=37398 avg_ns=37398 max_ns=37398
  open_like.post_open_revalidation.open: count=12 total_ns=202191 avg_ns=16849 max_ns=29371
  stat_child_no_follow: count=10506 total_ns=25846538 avg_ns=2460 max_ns=48617
  stat_child_no_follow.attr_conversion: count=10456 total_ns=166050 avg_ns=15 max_ns=303
  stat_child_no_follow.host_fstat: count=10456 total_ns=3012292 avg_ns=288 max_ns=3540
  stat_child_no_follow_context.path_guard_or_metadata: count=10506 total_ns=25846538 avg_ns=2460 max_ns=48617
  source_root_path: count=5177 total_ns=16005792 avg_ns=3091 max_ns=31477
  resolved_virtual_path: count=5201 total_ns=7981276 avg_ns=1534 max_ns=274313
  resolved_virtual_path_from_path: count=24 total_ns=102743 avg_ns=4280 max_ns=7398
  resolved_virtual_path_from_path_component_walk: count=24 total_ns=88809 avg_ns=3700 max_ns=6572
  resolved_virtual_path_from_path_canonicalize: count=48 total_ns=57201 avg_ns=1191 max_ns=3140
  resolved_virtual_path_from_path_source_root_confinement: count=48 total_ns=16661 avg_ns=347 max_ns=1541
  resolved_virtual_path_from_path_virtual_conversion: count=24 total_ns=11835 avg_ns=493 max_ns=1123
  resolved_virtual_path_from_open_fd: count=5177 total_ns=7878533 avg_ns=1521 max_ns=274313
  read_handle_snapshot: count=10256 total_ns=2410140 avg_ns=234 max_ns=2485
  read_guard_path: count=10256 total_ns=212010 avg_ns=20 max_ns=2485
  read_io: count=10256 total_ns=173330080 avg_ns=16900 max_ns=87739
  write_handle_snapshot: count=5120 total_ns=1632837 avg_ns=318 max_ns=2169
  write_guard_mutation: count=5120 total_ns=99661 avg_ns=19 max_ns=533
  write_io: count=5120 total_ns=185874436 avg_ns=36303 max_ns=482002
  file_sync.flush: count=12 total_ns=1030390 avg_ns=85865 max_ns=158519
  read_size_bucket.0_4k: count=2032 total_ns=1388868 avg_ns=683 max_ns=47131
  read_size_bucket.4k_64k: count=12 total_ns=109927 avg_ns=9160 max_ns=20939
  read_size_bucket.64k_1m: count=8212 total_ns=171831285 avg_ns=20924 max_ns=87739
  write_size_bucket.0_4k: count=4096 total_ns=5037093 avg_ns=1229 max_ns=15099
  write_size_bucket.64k_1m: count=1024 total_ns=180837343 avg_ns=176598 max_ns=482002
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
  invalidations: count=24 invalidated_entries=12 evicted_entries=0 scanned_entries=72
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
