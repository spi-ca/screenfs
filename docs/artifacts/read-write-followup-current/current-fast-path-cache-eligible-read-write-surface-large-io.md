# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:15.834690+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 64 --write-mib 64 --small-io-bytes 1024 --small-io-ops 512 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 512 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+82 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+82 more)`
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
| seq_read | 0.006360 | 0.024663 | 3.878 | 0.026461 | 0.026611 | 0.026730 |
| seq_write | 0.008141 | 0.024055 | 2.955 | 0.025044 | 0.025281 | 0.025471 |
| small_read | 0.000160 | 0.004024 | 25.077 | 0.004155 | 0.004172 | 0.004185 |
| small_write | 0.000230 | 0.008862 | 38.496 | 0.010046 | 0.010396 | 0.010677 |
| rand_read_4k | 0.000254 | 0.007195 | 28.309 | 0.007563 | 0.007638 | 0.007698 |
| rand_write_4k | 0.000367 | 0.009333 | 25.463 | 0.009867 | 0.009907 | 0.009939 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=57857 avg_ns=57857 max_ns=57857
  fuse_op.create: count=21 total_ns=1029187 avg_ns=49008 max_ns=74123
  fuse_op.flush: count=21 total_ns=658130 avg_ns=31339 max_ns=103251
  fuse_op.getattr: count=7652 total_ns=22651428 avg_ns=2960 max_ns=29388
  fuse_op.getxattr: count=7623 total_ns=62408172 avg_ns=8186 max_ns=343347
  fuse_op.lookup: count=236 total_ns=1181167 avg_ns=5004 max_ns=85179
  fuse_op.open: count=21 total_ns=332945 avg_ns=15854 max_ns=32021
  fuse_op.read: count=7126 total_ns=50146853 avg_ns=7037 max_ns=113956
  fuse_op.release: count=42 total_ns=61428 avg_ns=1462 max_ns=2516
  fuse_op.setattr: count=7 total_ns=135760 avg_ns=19394 max_ns=25994
  fuse_op.statfs: count=2 total_ns=2156 avg_ns=1078 max_ns=1198
  fuse_op.unlink: count=21 total_ns=14272852 avg_ns=679659 max_ns=2101950
  fuse_op.write: count=7616 total_ns=73248508 avg_ns=9617 max_ns=255181
  policy_decision: count=23709 total_ns=10407392 avg_ns=438 max_ns=264992
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
  matcher_candidate_order.descendant: count=23548 total_ns=3363816 avg_ns=142 max_ns=12403
  matcher_candidate_order.path: count=70966 total_ns=10269931 avg_ns=144 max_ns=10620
  matcher_candidate_order_by_source.hidden.path: count=23548 total_ns=3321827 avg_ns=141 max_ns=7071
  matcher_candidate_order_by_source.internal_hidden.path: count=23548 total_ns=3634901 avg_ns=154 max_ns=10620
  matcher_candidate_order_by_source.readonly.path: count=161 total_ns=19760 avg_ns=122 max_ns=195
  matcher_candidate_order_by_source.visible.descendant: count=23548 total_ns=3363816 avg_ns=142 max_ns=12403
  matcher_candidate_order_by_source.visible.path: count=23548 total_ns=3273645 avg_ns=139 max_ns=974
  matcher_candidate_order_by_source.writable.path: count=161 total_ns=19798 avg_ns=122 max_ns=306
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=376252
  matcher_candidate_order_ancestor_steps.descendant: count=93783
  matcher_candidate_order_ancestor_steps.path: count=282469
  state_read_lock_wait: count=30345 total_ns=540806 avg_ns=17 max_ns=2354
  state_read_lock_hold: count=30345 total_ns=2102915 avg_ns=69 max_ns=1132
  state_write_lock_wait: count=318 total_ns=7876 avg_ns=24 max_ns=318
  state_write_lock_hold: count=318 total_ns=197814 avg_ns=622 max_ns=4154
  open_confined_openat2: count=23387 total_ns=12713463 avg_ns=543 max_ns=21737
  open_like.pre_open_guard.access: count=1 total_ns=456 avg_ns=456 max_ns=456
  open_like.pre_open_guard.open: count=21 total_ns=2691 avg_ns=128 max_ns=244
  open_like.post_open_revalidation.access: count=1 total_ns=41335 avg_ns=41335 max_ns=41335
  open_like.post_open_revalidation.open: count=21 total_ns=282606 avg_ns=13457 max_ns=28429
  stat_child_no_follow: count=15693 total_ns=16296050 avg_ns=1038 max_ns=27459
  stat_child_no_follow.attr_conversion: count=15607 total_ns=205049 avg_ns=13 max_ns=67
  stat_child_no_follow.host_fstat: count=15607 total_ns=2252537 avg_ns=144 max_ns=1645
  stat_child_no_follow_context.path_guard_or_metadata: count=15693 total_ns=16296050 avg_ns=1038 max_ns=27459
  source_root_path: count=7715 total_ns=10991954 avg_ns=1424 max_ns=336753
  resolved_virtual_path: count=7757 total_ns=7668770 avg_ns=988 max_ns=47961
  resolved_virtual_path_from_path: count=42 total_ns=144107 avg_ns=3431 max_ns=4285
  resolved_virtual_path_from_path_component_walk: count=42 total_ns=124577 avg_ns=2966 max_ns=3710
  resolved_virtual_path_from_path_canonicalize: count=84 total_ns=85252 avg_ns=1014 max_ns=2599
  resolved_virtual_path_from_path_source_root_confinement: count=84 total_ns=19114 avg_ns=227 max_ns=763
  resolved_virtual_path_from_path_virtual_conversion: count=42 total_ns=16819 avg_ns=400 max_ns=553
  resolved_virtual_path_from_open_fd: count=7715 total_ns=7524663 avg_ns=975 max_ns=47961
  read_handle_snapshot: count=7126 total_ns=1315890 avg_ns=184 max_ns=62036
  read_guard_path: count=7126 total_ns=96748 avg_ns=13 max_ns=469
  read_io: count=7126 total_ns=47588923 avg_ns=6678 max_ns=113353
  write_handle_snapshot: count=7616 total_ns=1513618 avg_ns=198 max_ns=2807
  write_guard_mutation: count=7616 total_ns=102115 avg_ns=13 max_ns=268
  write_io: count=7616 total_ns=70309546 avg_ns=9231 max_ns=254340
  file_sync.flush: count=21 total_ns=646788 avg_ns=30799 max_ns=102255
  read_size_bucket.0_4k: count=3486 total_ns=2118215 avg_ns=607 max_ns=12533
  read_size_bucket.4k_64k: count=21 total_ns=76007 avg_ns=3619 max_ns=8511
  read_size_bucket.64k_1m: count=3619 total_ns=45394701 avg_ns=12543 max_ns=113353
  write_size_bucket.0_4k: count=7168 total_ns=8148087 avg_ns=1136 max_ns=10969
  write_size_bucket.64k_1m: count=448 total_ns=62161459 avg_ns=138753 max_ns=254340
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
