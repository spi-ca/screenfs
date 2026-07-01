# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:27.912843+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 64 --small-files 1024 --dir-entries 2048 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 1024 --sync-4k-fsync-every 32 --hidden-misses 512 --matcher-extra-rules 0 --matcher-misses 512 --symlink-parent-mutations 64 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+49 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+49 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.000378 | 0.006162 | 16.299 | 0.006764 | 0.007017 | 0.007219 |
| readdirplus_basic | 0.002168 | 0.082467 | 38.042 | 0.083679 | 0.084349 | 0.084885 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73622 avg_ns=73622 max_ns=73622
  fuse_op.getattr: count=26651 total_ns=93514797 avg_ns=3508 max_ns=20527
  fuse_op.lookup: count=79929 total_ns=339278233 avg_ns=4244 max_ns=135072
  fuse_op.opendir: count=26 total_ns=418983 avg_ns=16114 max_ns=33438
  fuse_op.readdir: count=103 total_ns=135456993 avg_ns=1315116 max_ns=3249842
  fuse_op.readdirplus: count=28 total_ns=39579373 avg_ns=1413549 max_ns=1901025
  fuse_op.releasedir: count=26 total_ns=5786032 avg_ns=222539 max_ns=376958
  fuse_op.statfs: count=2 total_ns=2792 avg_ns=1396 max_ns=1505
  policy_decision: count=106633 total_ns=43810405 avg_ns=410 max_ns=128780
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=106633 total_ns=14753407 avg_ns=138 max_ns=9506
  matcher_candidate_order.path: count=319899 total_ns=47098871 avg_ns=147 max_ns=19199
  matcher_candidate_order_by_source.hidden.path: count=106633 total_ns=15549953 avg_ns=145 max_ns=12965
  matcher_candidate_order_by_source.internal_hidden.path: count=106633 total_ns=16125284 avg_ns=151 max_ns=9898
  matcher_candidate_order_by_source.visible.descendant: count=106633 total_ns=14753407 avg_ns=138 max_ns=9506
  matcher_candidate_order_by_source.visible.path: count=106633 total_ns=15423634 avg_ns=144 max_ns=19199
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1385872
  matcher_candidate_order_ancestor_steps.descendant: count=346468
  matcher_candidate_order_ancestor_steps.path: count=1039404
  state_read_lock_wait: count=106738 total_ns=2511584 avg_ns=23 max_ns=10151
  state_read_lock_hold: count=106738 total_ns=6866414 avg_ns=64 max_ns=4229
  state_write_lock_wait: count=80136 total_ns=1817407 avg_ns=22 max_ns=1819
  state_write_lock_hold: count=80136 total_ns=100220517 avg_ns=1250 max_ns=2061379
  open_confined_openat2: count=112052 total_ns=89727533 avg_ns=800 max_ns=46473
  open_like.pre_open_guard.access: count=1 total_ns=332 avg_ns=332 max_ns=332
  open_like.pre_open_guard.opendir: count=26 total_ns=9434 avg_ns=362 max_ns=859
  open_like.post_open_revalidation.access: count=1 total_ns=49559 avg_ns=49559 max_ns=49559
  open_like.post_open_revalidation.opendir: count=26 total_ns=314224 avg_ns=12085 max_ns=26162
  stat_child_no_follow: count=111894 total_ns=156082048 avg_ns=1394 max_ns=49168
  stat_child_no_follow.attr_conversion: count=111892 total_ns=1910603 avg_ns=17 max_ns=378
  stat_child_no_follow.host_fstat: count=111892 total_ns=26627114 avg_ns=237 max_ns=11187
  stat_child_no_follow_context.path_guard_or_metadata: count=111894 total_ns=156082048 avg_ns=1394 max_ns=49168
  source_root_path: count=158 total_ns=1858448 avg_ns=11762 max_ns=54967
  resolved_virtual_path: count=158 total_ns=461684 avg_ns=2922 max_ns=15898
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=158 total_ns=461684 avg_ns=2922 max_ns=15898
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=103 total_ns=68424763 avg_ns=664318 max_ns=1605754
  readdir_scan.name_child_path_materialization: count=103 total_ns=3053364 avg_ns=29644 max_ns=76397
  readdir_scan.returned_child_path_materialization: count=48065 total_ns=10158976 avg_ns=211 max_ns=13390
  readdir_scan.scan_fallback_attr: count=103 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=103 total_ns=1449887 avg_ns=14076 max_ns=33912
  readdir_attr_generation_scan: count=103 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=103 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=103 total_ns=11983728 avg_ns=116346 max_ns=589425
  readdir_page_commit: count=103 total_ns=47617305 avg_ns=462303 max_ns=2061658
  readdirplus_directory_scan: count=28 total_ns=24338755 avg_ns=869241 max_ns=1223906
  readdirplus_scan.name_child_path_materialization: count=28 total_ns=1236996 avg_ns=44178 max_ns=48980
  readdirplus_scan.returned_attr_hydration: count=5262 total_ns=7802943 avg_ns=1482 max_ns=13975
  readdirplus_scan.returned_child_path_materialization: count=5262 total_ns=1300060 avg_ns=247 max_ns=3371
  readdirplus_scan.returned_policy_recheck: count=5262 total_ns=94543 avg_ns=17 max_ns=317
  readdirplus_scan.scan_fallback_attr: count=28 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=28 total_ns=978707 avg_ns=34953 max_ns=37171
  readdirplus_attr_generation_scan: count=5290 total_ns=7802943 avg_ns=1475 max_ns=13975
  readdirplus_attr_generation_entries: count=5262
  readdirplus_symlink_visibility: count=28 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=28 total_ns=6705861 avg_ns=239495 max_ns=295133
  readdirplus_page_commit: count=28 total_ns=4221940 avg_ns=150783 max_ns=298239
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
