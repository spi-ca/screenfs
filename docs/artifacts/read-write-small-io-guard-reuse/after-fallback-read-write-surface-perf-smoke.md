# ScreenFS benchmark result

- timestamp: `2026-06-20T09:39:11.305719+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 3 --warmups 1 --read-mib 8 --write-mib 8 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke.json --output-md docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface-perf-smoke.svg`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.002015 | 0.005298 | 2.630 | 0.005418 | 0.005433 | 0.005445 |
| seq_write | 0.001043 | 0.003299 | 3.162 | 0.003593 | 0.003630 | 0.003659 |
| small_read | 0.000157 | 0.009383 | 59.745 | 0.010083 | 0.010171 | 0.010241 |
| small_write | 0.000251 | 0.025867 | 103.069 | 0.026715 | 0.026821 | 0.026906 |
| rand_read_4k | 0.000302 | 0.019818 | 65.573 | 0.021191 | 0.021362 | 0.021500 |
| rand_write_4k | 0.000289 | 0.034733 | 120.199 | 0.034977 | 0.035007 | 0.035032 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=63829 avg_ns=63829 max_ns=63829
  fuse_op.create: count=12 total_ns=1217358 avg_ns=101446 max_ns=124045
  fuse_op.flush: count=12 total_ns=920902 avg_ns=76741 max_ns=155315
  fuse_op.getattr: count=4149 total_ns=53515467 avg_ns=12898 max_ns=133384
  fuse_op.getxattr: count=4132 total_ns=89548659 avg_ns=21671 max_ns=93829
  fuse_op.lookup: count=137 total_ns=1970507 avg_ns=14383 max_ns=57960
  fuse_op.open: count=12 total_ns=229116 avg_ns=19093 max_ns=28196
  fuse_op.read: count=2092 total_ns=38728980 avg_ns=18512 max_ns=166239
  fuse_op.release: count=24 total_ns=44411 avg_ns=1850 max_ns=3724
  fuse_op.setattr: count=4 total_ns=192511 avg_ns=48127 max_ns=49374
  fuse_op.statfs: count=2 total_ns=3990 avg_ns=1995 max_ns=2441
  fuse_op.unlink: count=12 total_ns=2109638 avg_ns=175803 max_ns=310030
  fuse_op.write: count=4128 total_ns=108580824 avg_ns=26303 max_ns=172921
  policy_decision: count=56622 total_ns=27240567 avg_ns=481 max_ns=72332
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=48274 total_ns=6920627 avg_ns=143 max_ns=54087
  matcher_candidate_order.path: count=161518 total_ns=29658046 avg_ns=183 max_ns=28821
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=56622
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=56622
  matcher_candidate_order_ancestor_steps: count=778344
  matcher_candidate_order_ancestor_steps.descendant: count=177914
  matcher_candidate_order_ancestor_steps.path: count=600430
  state_read_lock_wait: count=14691 total_ns=321936 avg_ns=21 max_ns=27072
  state_read_lock_hold: count=14691 total_ns=1119926 avg_ns=76 max_ns=38175
  state_write_lock_wait: count=183 total_ns=4880 avg_ns=26 max_ns=210
  state_write_lock_hold: count=183 total_ns=136501 avg_ns=745 max_ns=4313
  open_confined_openat2: count=18952 total_ns=10675824 avg_ns=563 max_ns=66344
  stat_child_no_follow: count=14779 total_ns=119028855 avg_ns=8053 max_ns=118667
  source_root_path: count=23059 total_ns=32303517 avg_ns=1400 max_ns=59218
  resolved_virtual_path: count=54690 total_ns=118837820 avg_ns=2172 max_ns=89558
  resolved_virtual_path_from_path: count=29507 total_ns=93685722 avg_ns=3175 max_ns=89558
  resolved_virtual_path_from_path_component_walk: count=29507 total_ns=82619191 avg_ns=2799 max_ns=88987
  resolved_virtual_path_from_path_canonicalize: count=73370 total_ns=66697282 avg_ns=909 max_ns=60644
  resolved_virtual_path_from_path_source_root_confinement: count=73370 total_ns=9143258 avg_ns=124 max_ns=84967
  resolved_virtual_path_from_path_virtual_conversion: count=29507 total_ns=9534838 avg_ns=323 max_ns=26762
  resolved_virtual_path_from_open_fd: count=25183 total_ns=25152098 avg_ns=998 max_ns=58787
  read_handle_snapshot: count=2092 total_ns=417579 avg_ns=199 max_ns=2232
  read_guard_path: count=2092 total_ns=33163329 avg_ns=15852 max_ns=162626
  read_io: count=2092 total_ns=4768222 avg_ns=2279 max_ns=75315
  write_handle_snapshot: count=4128 total_ns=915411 avg_ns=221 max_ns=38271
  write_guard_mutation: count=4128 total_ns=98366080 avg_ns=23828 max_ns=112284
  write_io: count=4128 total_ns=8492699 avg_ns=2057 max_ns=143759
  file_sync.flush: count=12 total_ns=911671 avg_ns=75972 max_ns=154157
  read_size_bucket.0_4k: count=1764 total_ns=1302717 avg_ns=738 max_ns=7034
  read_size_bucket.4k_64k: count=52 total_ns=150834 avg_ns=2900 max_ns=28590
  read_size_bucket.64k_1m: count=276 total_ns=3314671 avg_ns=12009 max_ns=75315
  write_size_bucket.0_4k: count=4096 total_ns=4481083 avg_ns=1094 max_ns=28714
  write_size_bucket.64k_1m: count=32 total_ns=4011616 avg_ns=125363 max_ns=143759
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
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
