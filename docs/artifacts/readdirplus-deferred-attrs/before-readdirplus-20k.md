# ScreenFS benchmark result

- timestamp: `2026-06-20T08:33:52.823951+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.json --output-md docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+35 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+35 more)`
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
| readdirplus_basic | 0.017105 | 2.391141 | 139.793 | 2.433916 | 2.441640 | 2.447820 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47495 avg_ns=47495 max_ns=47495
  fuse_op.getattr: count=140008 total_ns=2120162592 avg_ns=15143 max_ns=293322
  fuse_op.lookup: count=420019 total_ns=5178744448 avg_ns=12329 max_ns=358768
  fuse_op.opendir: count=7 total_ns=213910 avg_ns=30558 max_ns=74658
  fuse_op.readdir: count=176 total_ns=4594462869 avg_ns=26104902 max_ns=53747737
  fuse_op.readdirplus: count=27 total_ns=938274766 avg_ns=34750917 max_ns=65409602
  fuse_op.releasedir: count=7 total_ns=13905684 avg_ns=1986526 max_ns=2912643
  fuse_op.statfs: count=2 total_ns=3295 avg_ns=1647 max_ns=2347
  policy_decision: count=3148016 total_ns=1457603655 avg_ns=463 max_ns=439000
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=3148016 total_ns=421678186 avg_ns=133 max_ns=281737
  matcher_candidate_order.path: count=9444048 total_ns=1584848067 avg_ns=167 max_ns=784694
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3148016
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=3148016
  matcher_candidate_order_ancestor_steps: count=44766696
  matcher_candidate_order_ancestor_steps.descendant: count=11191674
  matcher_candidate_order_ancestor_steps.path: count=33575022
  state_read_lock_wait: count=560238 total_ns=11757097 avg_ns=20 max_ns=30555
  state_read_lock_hold: count=560238 total_ns=37655213 avg_ns=67 max_ns=11209
  state_write_lock_wait: count=420241 total_ns=8395521 avg_ns=19 max_ns=5585
  state_write_lock_hold: count=420241 total_ns=476533193 avg_ns=1133 max_ns=3051296
  open_confined_openat2: count=560260 total_ns=285420965 avg_ns=509 max_ns=116842
  stat_child_no_follow: count=560049 total_ns=3977162091 avg_ns=7101 max_ns=300013
  source_root_path: count=560259 total_ns=859200631 avg_ns=1533 max_ns=262748
  resolved_virtual_path: count=1680354 total_ns=3361888758 avg_ns=2000 max_ns=322256
  resolved_virtual_path_from_path: count=1120095 total_ns=2810038669 avg_ns=2508 max_ns=322256
  resolved_virtual_path_from_path_component_walk: count=1120095 total_ns=2401509538 avg_ns=2144 max_ns=321968
  resolved_virtual_path_from_path_canonicalize: count=1960112 total_ns=1896523030 avg_ns=967 max_ns=312870
  resolved_virtual_path_from_path_source_root_confinement: count=1960112 total_ns=261585270 avg_ns=133 max_ns=253374
  resolved_virtual_path_from_path_virtual_conversion: count=1120095 total_ns=347504685 avg_ns=310 max_ns=281378
  resolved_virtual_path_from_open_fd: count=560259 total_ns=551850089 avg_ns=984 max_ns=273239
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=176 total_ns=4382332239 avg_ns=24899614 max_ns=52841971
  readdir_attr_generation_scan: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=176 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=176 total_ns=297501754 avg_ns=1690350 max_ns=4541431
  readdir_page_commit: count=176 total_ns=179234688 avg_ns=1018378 max_ns=3051814
  readdirplus_directory_scan: count=27 total_ns=922414448 avg_ns=34163498 max_ns=65154329
  readdirplus_attr_generation_scan: count=27 total_ns=202932867 avg_ns=7516032 max_ns=16790763
  readdirplus_attr_generation_entries: count=327378
  readdirplus_symlink_visibility: count=27 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=27 total_ns=49937264 avg_ns=1849528 max_ns=4170155
  readdirplus_page_commit: count=27 total_ns=14213651 avg_ns=526431 max_ns=1506104
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
