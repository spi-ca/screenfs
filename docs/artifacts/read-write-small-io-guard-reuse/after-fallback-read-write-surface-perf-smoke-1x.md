# ScreenFS benchmark result

- timestamp: `2026-06-20T09:39:36.244926+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 1 --warmups 1 --read-mib 8 --write-mib 8 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke-1x.json --output-md docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke-1x.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke-1x.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/guards.rs; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a75c06032c0b901a15879e767ac0534a6e55dd72dc2ad6f3df7f6ad77c7de5c8`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/guards.rs; ... (+3 more)`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.002266 | 0.005476 | 2.416 | 0.005476 | 0.005476 | 0.005476 |
| seq_write | 0.001468 | 0.003562 | 2.426 | 0.003562 | 0.003562 | 0.003562 |
| small_read | 0.000285 | 0.009300 | 32.686 | 0.009300 | 0.009300 | 0.009300 |
| small_write | 0.000419 | 0.025548 | 61.000 | 0.025548 | 0.025548 | 0.025548 |
| rand_read_4k | 0.000546 | 0.018406 | 33.712 | 0.018406 | 0.018406 | 0.018406 |
| rand_write_4k | 0.000514 | 0.029031 | 56.441 | 0.029031 | 0.029031 | 0.029031 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=54756 avg_ns=54756 max_ns=54756
  fuse_op.create: count=6 total_ns=596405 avg_ns=99400 max_ns=116507
  fuse_op.flush: count=6 total_ns=214917 avg_ns=35819 max_ns=80148
  fuse_op.getattr: count=2075 total_ns=26418334 avg_ns=12731 max_ns=48205
  fuse_op.getxattr: count=2066 total_ns=41081031 avg_ns=19884 max_ns=131130
  fuse_op.lookup: count=71 total_ns=1088699 avg_ns=15333 max_ns=58183
  fuse_op.open: count=6 total_ns=130734 avg_ns=21789 max_ns=39477
  fuse_op.read: count=1046 total_ns=18426879 avg_ns=17616 max_ns=51162
  fuse_op.release: count=12 total_ns=17542 avg_ns=1461 max_ns=1943
  fuse_op.setattr: count=2 total_ns=78701 avg_ns=39350 max_ns=39774
  fuse_op.statfs: count=2 total_ns=1246 avg_ns=623 max_ns=733
  fuse_op.unlink: count=6 total_ns=1077768 avg_ns=179628 max_ns=324318
  fuse_op.write: count=2064 total_ns=50986362 avg_ns=24702 max_ns=301631
  policy_decision: count=28318 total_ns=12796547 avg_ns=451 max_ns=15071
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=24144 total_ns=3326282 avg_ns=137 max_ns=91529
  matcher_candidate_order.path: count=80780 total_ns=13836244 avg_ns=171 max_ns=35857
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=28318
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=28318
  matcher_candidate_order_ancestor_steps: count=389232
  matcher_candidate_order_ancestor_steps.descendant: count=88972
  matcher_candidate_order_ancestor_steps.path: count=300260
  state_read_lock_wait: count=7349 total_ns=136702 avg_ns=18 max_ns=280
  state_read_lock_hold: count=7349 total_ns=506718 avg_ns=68 max_ns=1042
  state_write_lock_wait: count=93 total_ns=2430 avg_ns=26 max_ns=270
  state_write_lock_hold: count=93 total_ns=66829 avg_ns=718 max_ns=3181
  open_confined_openat2: count=9480 total_ns=4791503 avg_ns=505 max_ns=32409
  stat_child_no_follow: count=7393 total_ns=55755519 avg_ns=7541 max_ns=118213
  source_root_path: count=11533 total_ns=14300611 avg_ns=1239 max_ns=83399
  resolved_virtual_path: count=27354 total_ns=56162634 avg_ns=2053 max_ns=39018
  resolved_virtual_path_from_path: count=14759 total_ns=44604600 avg_ns=3022 max_ns=39018
  resolved_virtual_path_from_path_component_walk: count=14759 total_ns=39065297 avg_ns=2646 max_ns=38688
  resolved_virtual_path_from_path_canonicalize: count=36692 total_ns=31355134 avg_ns=854 max_ns=38111
  resolved_virtual_path_from_path_source_root_confinement: count=36692 total_ns=4493434 avg_ns=122 max_ns=21002
  resolved_virtual_path_from_path_virtual_conversion: count=14759 total_ns=4802682 avg_ns=325 max_ns=20155
  resolved_virtual_path_from_open_fd: count=12595 total_ns=11558034 avg_ns=917 max_ns=13217
  read_handle_snapshot: count=1046 total_ns=199096 avg_ns=190 max_ns=1223
  read_guard_path: count=1046 total_ns=15693668 avg_ns=15003 max_ns=38313
  read_io: count=1046 total_ns=2356426 avg_ns=2252 max_ns=22243
  write_handle_snapshot: count=2064 total_ns=427920 avg_ns=207 max_ns=1761
  write_guard_mutation: count=2064 total_ns=45781887 avg_ns=22181 max_ns=116861
  write_io: count=2064 total_ns=4369721 avg_ns=2117 max_ns=221973
  file_sync.flush: count=6 total_ns=211154 avg_ns=35192 max_ns=79481
  read_size_bucket.0_4k: count=882 total_ns=549891 avg_ns=623 max_ns=3176
  read_size_bucket.4k_64k: count=26 total_ns=61333 avg_ns=2358 max_ns=7193
  read_size_bucket.64k_1m: count=138 total_ns=1745202 avg_ns=12646 max_ns=22243
  write_size_bucket.0_4k: count=2048 total_ns=1986361 avg_ns=969 max_ns=8438
  write_size_bucket.64k_1m: count=16 total_ns=2383360 avg_ns=148960 max_ns=221973
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
  invalidations: count=12 invalidated_entries=6 evicted_entries=0 scanned_entries=36
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
