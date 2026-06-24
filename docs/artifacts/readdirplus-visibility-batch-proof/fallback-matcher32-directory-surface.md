# ScreenFS benchmark result

- timestamp: `2026-06-24T01:13:21.974564+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/tests/perf.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/tests/perf.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001901 | 0.174720 | 91.913 | 0.178545 | 0.179023 | 0.179405 |
| readdirplus_basic | 0.010002 | 0.339186 | 33.912 | 0.397923 | 0.405265 | 0.411138 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=85467 avg_ns=85467 max_ns=85467
  fuse_op.getattr: count=20009 total_ns=185603428 avg_ns=9275 max_ns=95532
  fuse_op.lookup: count=60021 total_ns=510732584 avg_ns=8509 max_ns=327823
  fuse_op.opendir: count=8 total_ns=274264 avg_ns=34283 max_ns=64449
  fuse_op.readdir: count=54 total_ns=828426568 avg_ns=15341232 max_ns=37515376
  fuse_op.readdirplus: count=13 total_ns=256490383 avg_ns=19730029 max_ns=38890463
  fuse_op.releasedir: count=8 total_ns=4918765 avg_ns=614845 max_ns=960419
  fuse_op.statfs: count=2 total_ns=5113 avg_ns=2556 max_ns=2962
  policy_decision: count=262129 total_ns=311732918 avg_ns=1189 max_ns=81604
  matcher_candidates: count=640608
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=640608
  matcher_candidate_order.descendant: count=262129 total_ns=96803675 avg_ns=369 max_ns=39115
  matcher_candidate_order.path: count=786387 total_ns=330877945 avg_ns=420 max_ns=287175
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=25688642
  matcher_candidate_order_seen_slots.descendant: count=8388128
  matcher_candidate_order_seen_slots.path: count=17300514
  matcher_candidate_order_ancestor_steps: count=3953456
  matcher_candidate_order_ancestor_steps.descendant: count=988364
  matcher_candidate_order_ancestor_steps.path: count=2965092
  state_read_lock_wait: count=80106 total_ns=1551386 avg_ns=19 max_ns=2950
  state_read_lock_hold: count=80106 total_ns=4890151 avg_ns=61 max_ns=7420
  state_write_lock_wait: count=60110 total_ns=1104665 avg_ns=18 max_ns=2757
  state_write_lock_hold: count=60110 total_ns=128987752 avg_ns=2145 max_ns=5842951
  open_confined_openat2: count=82467 total_ns=54661113 avg_ns=662 max_ns=103341
  open_like.pre_open_guard.access: count=1 total_ns=61661 avg_ns=61661 max_ns=61661
  open_like.pre_open_guard.opendir: count=8 total_ns=181958 avg_ns=22744 max_ns=48270
  open_like.post_open_revalidation.access: count=1 total_ns=12927 avg_ns=12927 max_ns=12927
  open_like.post_open_revalidation.opendir: count=8 total_ns=60070 avg_ns=7508 max_ns=11976
  stat_child_no_follow: count=82391 total_ns=93416084 avg_ns=1133 max_ns=112326
  stat_child_no_follow.attr_conversion: count=82389 total_ns=1189806 avg_ns=14 max_ns=1062
  stat_child_no_follow.host_fstat: count=82389 total_ns=14358817 avg_ns=174 max_ns=13291
  stat_child_no_follow_context.path_guard_or_metadata: count=82391 total_ns=93416084 avg_ns=1133 max_ns=112326
  source_root_path: count=80122 total_ns=110622123 avg_ns=1380 max_ns=91974
  resolved_virtual_path: count=80129 total_ns=237199613 avg_ns=2960 max_ns=320696
  resolved_virtual_path_from_path: count=80053 total_ns=236857081 avg_ns=2958 max_ns=320696
  resolved_virtual_path_from_path_component_walk: count=80053 total_ns=205633703 avg_ns=2568 max_ns=320310
  resolved_virtual_path_from_path_canonicalize: count=180089 total_ns=163217248 avg_ns=906 max_ns=82169
  resolved_virtual_path_from_path_source_root_confinement: count=180089 total_ns=25260286 avg_ns=140 max_ns=14811
  resolved_virtual_path_from_path_virtual_conversion: count=80053 total_ns=27033909 avg_ns=337 max_ns=61285
  resolved_virtual_path_from_open_fd: count=76 total_ns=342532 avg_ns=4507 max_ns=15134
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=723428388 avg_ns=13396822 max_ns=35407659
  readdir_scan.name_child_path_materialization: count=54 total_ns=52544268 avg_ns=973042 max_ns=2560351
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=582016825 avg_ns=10778089 max_ns=29557222
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=31875330 avg_ns=590283 max_ns=1596296
  readdir_page_commit: count=54 total_ns=90230317 avg_ns=1670931 max_ns=5843410
  readdirplus_directory_scan: count=13 total_ns=238047997 avg_ns=18311384 max_ns=36861863
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=16462654 avg_ns=1266358 max_ns=2680256
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=4262639 avg_ns=1824 max_ns=22922
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=9679781 avg_ns=4143 max_ns=11350
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=198734888 avg_ns=15287299 max_ns=30931330
  readdirplus_attr_generation_scan: count=2349 total_ns=4262639 avg_ns=1814 max_ns=22922
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=10046022 avg_ns=772770 max_ns=1587128
  readdirplus_page_commit: count=13 total_ns=3278817 avg_ns=252216 max_ns=384249
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
