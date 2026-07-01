# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:31.305900+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload-set metadata-open-path --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 64 --small-files 1024 --dir-entries 2048 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 1024 --sync-4k-fsync-every 32 --hidden-misses 512 --matcher-extra-rules 0 --matcher-misses 512 --symlink-parent-mutations 64 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+53 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+53 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `metadata-open-path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs`
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
| metadata_lookup | 0.003659 | 0.026955 | 7.367 | 0.027458 | 0.027489 | 0.027513 |
| metadata_getattr | 0.003929 | 0.033188 | 8.448 | 0.033865 | 0.033904 | 0.033936 |
| metadata_open | 0.003888 | 0.037458 | 9.634 | 0.043629 | 0.046880 | 0.049481 |
| metadata_readlink | 0.000363 | 0.036913 | 101.824 | 0.037833 | 0.037833 | 0.037834 |
| metadata_access | 0.003664 | 0.034203 | 9.336 | 0.035279 | 0.035882 | 0.036366 |
| metadata_statfs | 0.000442 | 0.004417 | 9.988 | 0.004491 | 0.004526 | 0.004553 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=65556044 avg_ns=4924 max_ns=56742
  fuse_op.getattr: count=13313 total_ns=36430901 avg_ns=2736 max_ns=23052
  fuse_op.lookup: count=199685 total_ns=753677160 avg_ns=3774 max_ns=21714100
  fuse_op.open: count=13312 total_ns=75218054 avg_ns=5650 max_ns=60174
  fuse_op.readlink: count=13312 total_ns=149105661 avg_ns=11200 max_ns=64498
  fuse_op.release: count=13312 total_ns=6507550 avg_ns=488 max_ns=47471
  fuse_op.statfs: count=13314 total_ns=2386876 avg_ns=179 max_ns=6389
  policy_decision: count=279559 total_ns=97356814 avg_ns=348 max_ns=333376
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=279559 total_ns=32944341 avg_ns=117 max_ns=368341
  matcher_candidate_order.path: count=838677 total_ns=109551194 avg_ns=130 max_ns=264669
  matcher_candidate_order_by_source.hidden.path: count=279559 total_ns=36014160 avg_ns=128 max_ns=54208
  matcher_candidate_order_by_source.internal_hidden.path: count=279559 total_ns=38115719 avg_ns=136 max_ns=264669
  matcher_candidate_order_by_source.visible.descendant: count=279559 total_ns=32944341 avg_ns=117 max_ns=368341
  matcher_candidate_order_by_source.visible.path: count=279559 total_ns=35421315 avg_ns=126 max_ns=18976
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=3620936
  matcher_candidate_order_ancestor_steps.descendant: count=905234
  matcher_candidate_order_ancestor_steps.path: count=2715702
  state_read_lock_wait: count=252935 total_ns=4861534 avg_ns=19 max_ns=8135
  state_read_lock_hold: count=252935 total_ns=13437162 avg_ns=53 max_ns=7658
  state_write_lock_wait: count=212995 total_ns=3794672 avg_ns=17 max_ns=18969
  state_write_lock_hold: count=212995 total_ns=71510287 avg_ns=335 max_ns=53728
  open_confined_openat2: count=252935 total_ns=218051745 avg_ns=862 max_ns=21706292
  open_like.pre_open_guard.access: count=13313 total_ns=183219 avg_ns=13 max_ns=338
  open_like.pre_open_guard.open: count=13312 total_ns=198248 avg_ns=14 max_ns=621
  open_like.post_open_revalidation.access: count=13313 total_ns=51668986 avg_ns=3881 max_ns=50637
  open_like.post_open_revalidation.open: count=13312 total_ns=60910943 avg_ns=4575 max_ns=58126
  stat_child_no_follow: count=226310 total_ns=378345373 avg_ns=1671 max_ns=21708088
  stat_child_no_follow.attr_conversion: count=212996 total_ns=2844592 avg_ns=13 max_ns=2785
  stat_child_no_follow.directory_revalidation: count=13312 total_ns=80339192 avg_ns=6035 max_ns=33549
  stat_child_no_follow.host_fstat: count=199684 total_ns=28143750 avg_ns=140 max_ns=12576
  stat_child_no_follow.host_fstatat: count=13312 total_ns=2727488 avg_ns=204 max_ns=11748
  stat_child_no_follow.parent_open: count=13312 total_ns=8465083 avg_ns=635 max_ns=7918
  stat_child_no_follow_context.path_guard_or_metadata: count=212998 total_ns=283848291 avg_ns=1332 max_ns=21708088
  stat_child_no_follow_context.readlink_pre_open: count=13312 total_ns=94497082 avg_ns=7098 max_ns=35568
  source_root_path: count=39937 total_ns=47470323 avg_ns=1188 max_ns=48196
  resolved_virtual_path: count=53249 total_ns=72182997 avg_ns=1355 max_ns=47468
  resolved_virtual_path_from_path: count=13312 total_ns=30718850 avg_ns=2307 max_ns=16088
  resolved_virtual_path_from_path_component_walk: count=13312 total_ns=26428164 avg_ns=1985 max_ns=9506
  resolved_virtual_path_from_path_canonicalize: count=26624 total_ns=21252847 avg_ns=798 max_ns=9191
  resolved_virtual_path_from_path_source_root_confinement: count=26624 total_ns=2904010 avg_ns=109 max_ns=887
  resolved_virtual_path_from_path_virtual_conversion: count=13312 total_ns=3631267 avg_ns=272 max_ns=13743
  resolved_virtual_path_from_open_fd: count=39937 total_ns=41464147 avg_ns=1038 max_ns=47468
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
