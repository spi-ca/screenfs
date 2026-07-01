# ScreenFS benchmark result

- timestamp: `2026-07-01T05:16:13.547104+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set directory-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+103 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+103 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
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
| readdir_basic | 0.001277 | 0.025896 | 20.275 | 0.026489 | 0.026494 | 0.026497 |
| readdirplus_basic | 0.006553 | 0.233287 | 35.598 | 0.247759 | 0.248165 | 0.248490 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=76166 avg_ns=76166 max_ns=76166
  fuse_op.getattr: count=65027 total_ns=533408735 avg_ns=8202 max_ns=272592
  fuse_op.lookup: count=195057 total_ns=1448918137 avg_ns=7428 max_ns=326484
  fuse_op.opendir: count=26 total_ns=371766 avg_ns=14298 max_ns=41380
  fuse_op.readdir: count=180 total_ns=476924613 avg_ns=2649581 max_ns=5166240
  fuse_op.readdirplus: count=31 total_ns=98492877 avg_ns=3177189 max_ns=4171680
  fuse_op.releasedir: count=26 total_ns=11026355 avg_ns=424090 max_ns=707288
  fuse_op.statfs: count=2 total_ns=4158 avg_ns=2079 max_ns=2751
  policy_decision: count=260164 total_ns=109513932 avg_ns=420 max_ns=32948
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=260164 total_ns=30772096 avg_ns=118 max_ns=47807
  matcher_candidate_order.path: count=780492 total_ns=124158824 avg_ns=159 max_ns=269814
  matcher_candidate_order_by_source.hidden.path: count=260164 total_ns=55639802 avg_ns=213 max_ns=269814
  matcher_candidate_order_by_source.internal_hidden.path: count=260164 total_ns=35322329 avg_ns=135 max_ns=11139
  matcher_candidate_order_by_source.visible.descendant: count=260164 total_ns=30772096 avg_ns=118 max_ns=47807
  matcher_candidate_order_by_source.visible.path: count=260164 total_ns=33196693 avg_ns=127 max_ns=26444
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=260164
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=260164
  matcher_candidate_order_ancestor_steps: count=3381740
  matcher_candidate_order_ancestor_steps.descendant: count=845435
  matcher_candidate_order_ancestor_steps.path: count=2536305
  state_read_lock_wait: count=260322 total_ns=4834219 avg_ns=18 max_ns=6280
  state_read_lock_hold: count=260322 total_ns=15500180 avg_ns=59 max_ns=8326
  state_write_lock_wait: count=195344 total_ns=3441298 avg_ns=17 max_ns=8805
  state_write_lock_hold: count=195344 total_ns=235891613 avg_ns=1207 max_ns=2690445
  open_confined_openat2: count=266229 total_ns=156955042 avg_ns=589 max_ns=269368
  open_like.pre_open_guard.access: count=1 total_ns=57900 avg_ns=57900 max_ns=57900
  open_like.pre_open_guard.opendir: count=26 total_ns=254683 avg_ns=9795 max_ns=31785
  open_like.post_open_revalidation.access: count=1 total_ns=6730 avg_ns=6730 max_ns=6730
  open_like.post_open_revalidation.opendir: count=26 total_ns=59153 avg_ns=2275 max_ns=5018
  stat_child_no_follow: count=265991 total_ns=275528730 avg_ns=1035 max_ns=269822
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3658429 avg_ns=13 max_ns=27502
  stat_child_no_follow.host_fstat: count=265989 total_ns=42822616 avg_ns=160 max_ns=47971
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=275528730 avg_ns=1035 max_ns=269822
  source_root_path: count=260374 total_ns=323611475 avg_ns=1242 max_ns=290303
  resolved_virtual_path: count=260399 total_ns=708804261 avg_ns=2721 max_ns=68203
  resolved_virtual_path_from_path: count=260161 total_ns=708259332 avg_ns=2722 max_ns=68203
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=611110303 avg_ns=2348 max_ns=67572
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=493236883 avg_ns=842 max_ns=66866
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=62441317 avg_ns=106 max_ns=15184
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=84471921 avg_ns=324 max_ns=58958
  resolved_virtual_path_from_open_fd: count=238 total_ns=544929 avg_ns=2289 max_ns=6298
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=330817978 avg_ns=1837877 max_ns=3747138
  readdir_scan.name_child_path_materialization: count=180 total_ns=95283898 avg_ns=529354 max_ns=1184384
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=6804834 avg_ns=37804 max_ns=90806
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=56219478 avg_ns=312330 max_ns=846357
  readdir_page_commit: count=180 total_ns=127923963 avg_ns=710688 max_ns=2690761
  readdirplus_directory_scan: count=31 total_ns=84089296 avg_ns=2712557 max_ns=3650707
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=28393559 avg_ns=915921 max_ns=1187187
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=7610910 avg_ns=1305 max_ns=16050
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=93936 avg_ns=16 max_ns=397
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=2270388 avg_ns=73238 max_ns=93078
  readdirplus_attr_generation_scan: count=5859 total_ns=7610910 avg_ns=1299 max_ns=16050
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=18098097 avg_ns=583809 max_ns=883672
  readdirplus_page_commit: count=31 total_ns=4918919 avg_ns=158674 max_ns=297807
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
