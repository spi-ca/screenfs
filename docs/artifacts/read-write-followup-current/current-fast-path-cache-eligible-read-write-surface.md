# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:27.433015+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+88 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+88 more)`
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
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.006191 | 0.026483 | 4.278 | 0.027604 | 0.027862 | 0.028068 |
| seq_write | 0.008566 | 0.024418 | 2.850 | 0.025402 | 0.025570 | 0.025705 |
| small_read | 0.000338 | 0.008422 | 24.922 | 0.009181 | 0.009241 | 0.009290 |
| small_write | 0.000580 | 0.018460 | 31.817 | 0.018775 | 0.018790 | 0.018803 |
| rand_read_4k | 0.011377 | 0.183240 | 16.106 | 0.184263 | 0.184812 | 0.185251 |
| rand_write_4k | 0.012439 | 0.295832 | 23.782 | 0.299473 | 0.301923 | 0.303882 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=39331 avg_ns=39331 max_ns=39331
  fuse_op.create: count=39 total_ns=1973023 avg_ns=50590 max_ns=83772
  fuse_op.flush: count=39 total_ns=977811 avg_ns=25072 max_ns=92505
  fuse_op.getattr: count=227202 total_ns=637601729 avg_ns=2806 max_ns=64716
  fuse_op.getxattr: count=227149 total_ns=1760283092 avg_ns=7749 max_ns=352710
  fuse_op.lookup: count=434 total_ns=2060403 avg_ns=4747 max_ns=25393
  fuse_op.open: count=39 total_ns=716273 avg_ns=18365 max_ns=32958
  fuse_op.read: count=148343 total_ns=239422606 avg_ns=1613 max_ns=127688
  fuse_op.release: count=78 total_ns=120929 avg_ns=1550 max_ns=3323
  fuse_op.setattr: count=13 total_ns=248049 avg_ns=19080 max_ns=23941
  fuse_op.statfs: count=2 total_ns=3731 avg_ns=1865 max_ns=2949
  fuse_op.unlink: count=39 total_ns=52034736 avg_ns=1334224 max_ns=2543399
  fuse_op.write: count=227136 total_ns=529084972 avg_ns=2329 max_ns=413003
  policy_decision: count=683001 total_ns=292443429 avg_ns=428 max_ns=317341
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
  matcher_candidate_order.descendant: count=682702 total_ns=95851907 avg_ns=140 max_ns=16910
  matcher_candidate_order.path: count=2048704 total_ns=293628473 avg_ns=143 max_ns=62238
  matcher_candidate_order_by_source.hidden.path: count=682702 total_ns=96410168 avg_ns=141 max_ns=19494
  matcher_candidate_order_by_source.internal_hidden.path: count=682702 total_ns=101806376 avg_ns=149 max_ns=62238
  matcher_candidate_order_by_source.readonly.path: count=299 total_ns=36642 avg_ns=122 max_ns=199
  matcher_candidate_order_by_source.visible.descendant: count=682702 total_ns=95851907 avg_ns=140 max_ns=16910
  matcher_candidate_order_by_source.visible.path: count=682702 total_ns=95338948 avg_ns=139 max_ns=25148
  matcher_candidate_order_by_source.writable.path: count=299 total_ns=36339 avg_ns=121 max_ns=205
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=10922308
  matcher_candidate_order_ancestor_steps.descendant: count=2730057
  matcher_candidate_order_ancestor_steps.path: count=8192251
  state_read_lock_wait: count=830434 total_ns=14474793 avg_ns=17 max_ns=14467
  state_read_lock_hold: count=830434 total_ns=50077074 avg_ns=60 max_ns=27186
  state_write_lock_wait: count=588 total_ns=13777 avg_ns=23 max_ns=321
  state_write_lock_hold: count=588 total_ns=469993 avg_ns=799 max_ns=36528
  open_confined_openat2: count=682403 total_ns=333467301 avg_ns=488 max_ns=343448
  open_like.pre_open_guard.access: count=1 total_ns=309 avg_ns=309 max_ns=309
  open_like.pre_open_guard.open: count=39 total_ns=5691 avg_ns=145 max_ns=255
  open_like.post_open_revalidation.access: count=1 total_ns=25575 avg_ns=25575 max_ns=25575
  open_like.post_open_revalidation.open: count=39 total_ns=612503 avg_ns=15705 max_ns=26680
  stat_child_no_follow: count=455123 total_ns=431095925 avg_ns=947 max_ns=344049
  stat_child_no_follow.attr_conversion: count=454965 total_ns=5948706 avg_ns=13 max_ns=5704
  stat_child_no_follow.host_fstat: count=454965 total_ns=62367251 avg_ns=137 max_ns=47604
  stat_child_no_follow_context.path_guard_or_metadata: count=455123 total_ns=431095925 avg_ns=947 max_ns=344049
  source_root_path: count=227319 total_ns=283054766 avg_ns=1245 max_ns=54817
  resolved_virtual_path: count=227397 total_ns=208702204 avg_ns=917 max_ns=331806
  resolved_virtual_path_from_path: count=78 total_ns=266273 avg_ns=3413 max_ns=7914
  resolved_virtual_path_from_path_component_walk: count=78 total_ns=230342 avg_ns=2953 max_ns=7495
  resolved_virtual_path_from_path_canonicalize: count=156 total_ns=155293 avg_ns=995 max_ns=2849
  resolved_virtual_path_from_path_source_root_confinement: count=156 total_ns=33502 avg_ns=214 max_ns=771
  resolved_virtual_path_from_path_virtual_conversion: count=78 total_ns=30996 avg_ns=397 max_ns=618
  resolved_virtual_path_from_open_fd: count=227319 total_ns=208435931 avg_ns=916 max_ns=331806
  read_handle_snapshot: count=148343 total_ns=26323976 avg_ns=177 max_ns=28509
  read_guard_path: count=148343 total_ns=1926612 avg_ns=12 max_ns=2102
  read_io: count=148343 total_ns=187439594 avg_ns=1263 max_ns=127057
  write_handle_snapshot: count=227136 total_ns=41529416 avg_ns=182 max_ns=27277
  write_guard_mutation: count=227136 total_ns=2994494 avg_ns=13 max_ns=689
  write_io: count=227136 total_ns=450140974 avg_ns=1981 max_ns=412237
  file_sync.flush: count=39 total_ns=951033 avg_ns=24385 max_ns=91749
  read_size_bucket.0_4k: count=126893 total_ns=88884356 avg_ns=700 max_ns=78536
  read_size_bucket.4k_64k: count=14365 total_ns=19648766 avg_ns=1367 max_ns=11847
  read_size_bucket.64k_1m: count=7085 total_ns=78906472 avg_ns=11137 max_ns=127057
  write_size_bucket.0_4k: count=226304 total_ns=334423628 avg_ns=1477 max_ns=153052
  write_size_bucket.64k_1m: count=832 total_ns=115717346 avg_ns=139083 max_ns=412237
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
