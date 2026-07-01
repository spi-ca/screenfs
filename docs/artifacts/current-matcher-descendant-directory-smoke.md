# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:36.036169+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher32 --workload-set matcher-descendant-directory --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 200 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-descendant-directory-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-descendant-directory-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-descendant-directory-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg; ?? docs/artifacts/current-benchmark-rerun.log`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg; ?? docs/artifacts/current-benchmark-rerun.log`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
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
| matcher_descendant_readdir | 0.000019 | 0.000540 | 28.545 | 0.000599 | 0.000607 | 0.000612 |
| matcher_descendant_readdirplus | 0.000032 | 0.002766 | 87.249 | 0.002801 | 0.002805 | 0.002808 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=59188 avg_ns=59188 max_ns=59188
  fuse_op.getattr: count=137 total_ns=1731591 avg_ns=12639 max_ns=22679
  fuse_op.lookup: count=541 total_ns=5217031 avg_ns=9643 max_ns=54526
  fuse_op.opendir: count=8 total_ns=101111 avg_ns=12638 max_ns=16734
  fuse_op.readdir: count=8 total_ns=80608 avg_ns=10076 max_ns=17381
  fuse_op.readdirplus: count=8 total_ns=3175948 avg_ns=396993 max_ns=442116
  fuse_op.releasedir: count=8 total_ns=40804 avg_ns=5100 max_ns=14105
  fuse_op.statfs: count=2 total_ns=2829 avg_ns=1414 max_ns=1575
  policy_decision: count=1216 total_ns=2154500 avg_ns=1771 max_ns=24097
  matcher_candidates: count=15872
  matcher_candidates_by_source.hidden.path: count=928
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=14944
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=15872
  matcher_candidate_order.descendant: count=1216 total_ns=646926 avg_ns=532 max_ns=2916
  matcher_candidate_order.path: count=3648 total_ns=1445799 avg_ns=396 max_ns=5564
  matcher_candidate_order_by_source.hidden.path: count=1216 total_ns=556013 avg_ns=457 max_ns=2219
  matcher_candidate_order_by_source.internal_hidden.path: count=1216 total_ns=214685 avg_ns=176 max_ns=2952
  matcher_candidate_order_by_source.visible.descendant: count=1216 total_ns=646926 avg_ns=532 max_ns=2916
  matcher_candidate_order_by_source.visible.path: count=1216 total_ns=675101 avg_ns=555 max_ns=5564
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=119168
  matcher_candidate_order_seen_slots.descendant: count=38912
  matcher_candidate_order_seen_slots.path: count=80256
  matcher_candidate_order_ancestor_steps: count=20812
  matcher_candidate_order_ancestor_steps.descendant: count=5203
  matcher_candidate_order_ancestor_steps.path: count=15609
  state_read_lock_wait: count=703 total_ns=12683 avg_ns=18 max_ns=187
  state_read_lock_hold: count=703 total_ns=52263 avg_ns=74 max_ns=2666
  state_write_lock_wait: count=579 total_ns=9808 avg_ns=16 max_ns=52
  state_write_lock_hold: count=579 total_ns=313366 avg_ns=541 max_ns=36514
  open_confined_openat2: count=984 total_ns=504961 avg_ns=513 max_ns=7800
  open_like.pre_open_guard.access: count=1 total_ns=49562 avg_ns=49562 max_ns=49562
  open_like.pre_open_guard.opendir: count=8 total_ns=80885 avg_ns=10110 max_ns=13921
  open_like.post_open_revalidation.access: count=1 total_ns=5347 avg_ns=5347 max_ns=5347
  open_like.post_open_revalidation.opendir: count=8 total_ns=10203 avg_ns=1275 max_ns=1860
  stat_child_no_follow: count=959 total_ns=876195 avg_ns=913 max_ns=9979
  stat_child_no_follow.attr_conversion: count=957 total_ns=12371 avg_ns=12 max_ns=32
  stat_child_no_follow.host_fstat: count=957 total_ns=129923 avg_ns=135 max_ns=2050
  stat_child_no_follow_context.path_guard_or_metadata: count=959 total_ns=876195 avg_ns=913 max_ns=9979
  source_root_path: count=719 total_ns=901753 avg_ns=1254 max_ns=20137
  resolved_virtual_path: count=726 total_ns=2433258 avg_ns=3351 max_ns=9490
  resolved_virtual_path_from_path: count=701 total_ns=2402800 avg_ns=3427 max_ns=9490
  resolved_virtual_path_from_path_component_walk: count=701 total_ns=2101022 avg_ns=2997 max_ns=8102
  resolved_virtual_path_from_path_canonicalize: count=1937 total_ns=1716679 avg_ns=886 max_ns=6517
  resolved_virtual_path_from_path_source_root_confinement: count=1937 total_ns=207518 avg_ns=107 max_ns=919
  resolved_virtual_path_from_path_virtual_conversion: count=701 total_ns=268721 avg_ns=383 max_ns=3775
  resolved_virtual_path_from_open_fd: count=25 total_ns=30458 avg_ns=1218 max_ns=2861
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=8 total_ns=32857 avg_ns=4107 max_ns=6863
  readdir_scan.name_child_path_materialization: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=8 total_ns=1278 avg_ns=159 max_ns=206
  readdirplus_directory_scan: count=8 total_ns=1365573 avg_ns=170696 max_ns=191973
  readdirplus_scan.name_child_path_materialization: count=8 total_ns=72286 avg_ns=9035 max_ns=10864
  readdirplus_scan.returned_attr_hydration: count=256 total_ns=216739 avg_ns=846 max_ns=1492
  readdirplus_scan.returned_policy_recheck: count=256 total_ns=1227890 avg_ns=4796 max_ns=10693
  readdirplus_scan.scan_fallback_attr: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=8 total_ns=1204825 avg_ns=150603 max_ns=161221
  readdirplus_attr_generation_scan: count=264 total_ns=216739 avg_ns=820 max_ns=1492
  readdirplus_attr_generation_entries: count=256
  readdirplus_symlink_visibility: count=8 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=8 total_ns=31605 avg_ns=3950 max_ns=7457
  readdirplus_page_commit: count=8 total_ns=135604 avg_ns=16950 max_ns=36642
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
