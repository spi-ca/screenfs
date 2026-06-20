# ScreenFS benchmark result

- timestamp: `2026-06-20T08:41:37.065406+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.json --output-md docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+36 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `4d1cca3219e371f9cc936238aa0170b1e22015125eec98a5eaf042487649d1bd`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+36 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.028776 | 2.458977 | 85.451 | 2.491584 | 2.494807 | 2.497385 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=68848 avg_ns=68848 max_ns=68848
  fuse_op.getattr: count=140008 total_ns=2243953967 avg_ns=16027 max_ns=416472
  fuse_op.lookup: count=420019 total_ns=5549023307 avg_ns=13211 max_ns=2091003
  fuse_op.opendir: count=7 total_ns=199897 avg_ns=28556 max_ns=43879
  fuse_op.readdir: count=176 total_ns=4856995113 avg_ns=27596563 max_ns=56000355
  fuse_op.readdirplus: count=27 total_ns=909196081 avg_ns=33673928 max_ns=56302605
  fuse_op.releasedir: count=7 total_ns=16087722 avg_ns=2298246 max_ns=3269995
  fuse_op.statfs: count=2 total_ns=6304 avg_ns=3152 max_ns=5238
  policy_decision: count=3158180 total_ns=1543804256 avg_ns=488 max_ns=330807
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3158180 total_ns=446241457 avg_ns=141 max_ns=295271
  matcher_candidate_order.path: count=9474540 total_ns=1684071470 avg_ns=177 max_ns=316812
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3158180
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3158180
  matcher_candidate_order_ancestor_steps: count=44908992
  matcher_candidate_order_ancestor_steps.descendant: count=11227248
  matcher_candidate_order_ancestor_steps.path: count=33681744
  state_read_lock_wait: count=560238 total_ns=12364927 avg_ns=22 max_ns=13761
  state_read_lock_hold: count=560238 total_ns=39913519 avg_ns=71 max_ns=105659
  state_write_lock_wait: count=420241 total_ns=8861451 avg_ns=21 max_ns=16712
  state_write_lock_hold: count=420241 total_ns=500827637 avg_ns=1191 max_ns=3269337
  open_confined_openat2: count=565342 total_ns=322208802 avg_ns=569 max_ns=192624
  stat_child_no_follow: count=565131 total_ns=4373645633 avg_ns=7739 max_ns=2079473
  source_root_path: count=565341 total_ns=982172773 avg_ns=1737 max_ns=2041085
  resolved_virtual_path: count=1690518 total_ns=3556469609 avg_ns=2103 max_ns=329717
  resolved_virtual_path_from_path: count=1125177 total_ns=2934849899 avg_ns=2608 max_ns=329717
  resolved_virtual_path_from_path_component_walk: count=1125177 total_ns=2496244548 avg_ns=2218 max_ns=329294
  resolved_virtual_path_from_path_canonicalize: count=1970276 total_ns=1969093189 avg_ns=999 max_ns=328640
  resolved_virtual_path_from_path_source_root_confinement: count=1970276 total_ns=271867199 avg_ns=137 max_ns=27889
  resolved_virtual_path_from_path_virtual_conversion: count=1125177 total_ns=375015275 avg_ns=333 max_ns=263557
  resolved_virtual_path_from_open_fd: count=565341 total_ns=621619710 avg_ns=1099 max_ns=213462
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=176 total_ns=4635179523 avg_ns=26336247 max_ns=55023144
  readdir_attr_generation_scan: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=176 total_ns=291774767 avg_ns=1657811 max_ns=4468290
  readdir_page_commit: count=176 total_ns=185235067 avg_ns=1052471 max_ns=2773394
  readdirplus_directory_scan: count=27 total_ns=843851072 avg_ns=31253743 max_ns=53791560
  readdirplus_attr_generation_scan: count=5109 total_ns=40751062 avg_ns=7976 max_ns=77387
  readdirplus_attr_generation_entries: count=5082
  readdirplus_symlink_visibility: count=27 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=27 total_ns=46969798 avg_ns=1739622 max_ns=3623418
  readdirplus_page_commit: count=27 total_ns=13606518 avg_ns=503945 max_ns=1184575
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
