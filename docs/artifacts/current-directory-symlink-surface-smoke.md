# ScreenFS benchmark result

- timestamp: `2026-06-20T08:41:38.035469+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-symlink-surface --iterations 3 --warmups 1 --dir-entries 200 --output-json docs/artifacts/current-directory-symlink-surface-smoke.json --output-md docs/artifacts/current-directory-symlink-surface-smoke.md --output-svg docs/artifacts/current-directory-symlink-surface-smoke.svg`
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
- workload_selection: `named-set`
- workload_set: `directory-symlink-surface`
- comparable_workloads: `readdir_symlink_visibility, readdirplus_symlink_visibility`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_symlink_visibility | 0.000063 | 0.005073 | 81.149 | 0.005624 | 0.005692 | 0.005748 |
| readdirplus_symlink_visibility | 0.000175 | 0.018376 | 104.976 | 0.019168 | 0.019267 | 0.019346 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47953 avg_ns=47953 max_ns=47953
  fuse_op.getattr: count=817 total_ns=11974921 avg_ns=14657 max_ns=101860
  fuse_op.lookup: count=2445 total_ns=26789853 avg_ns=10956 max_ns=190316
  fuse_op.opendir: count=8 total_ns=107266 avg_ns=13408 max_ns=14540
  fuse_op.readdir: count=15 total_ns=1339346 avg_ns=89289 max_ns=273206
  fuse_op.readdirplus: count=9 total_ns=38722311 avg_ns=4302479 max_ns=5064401
  fuse_op.releasedir: count=8 total_ns=120104 avg_ns=15013 max_ns=17232
  fuse_op.statfs: count=2 total_ns=6163 avg_ns=3081 max_ns=4051
  policy_decision: count=16230 total_ns=6325858 avg_ns=389 max_ns=18731
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=16230 total_ns=1918306 avg_ns=118 max_ns=14313
  matcher_candidate_order.path: count=48690 total_ns=7136445 avg_ns=146 max_ns=86982
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=16230
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=16230
  matcher_candidate_order_ancestor_steps: count=220216
  matcher_candidate_order_ancestor_steps.descendant: count=55054
  matcher_candidate_order_ancestor_steps.path: count=165162
  state_read_lock_wait: count=3295 total_ns=58477 avg_ns=17 max_ns=220
  state_read_lock_hold: count=3295 total_ns=201065 avg_ns=61 max_ns=3680
  state_write_lock_wait: count=2491 total_ns=42041 avg_ns=16 max_ns=156
  state_write_lock_hold: count=2491 total_ns=1855505 avg_ns=744 max_ns=222188
  open_confined_openat2: count=4872 total_ns=1893968 avg_ns=388 max_ns=14904
  stat_child_no_follow: count=4839 total_ns=29273758 avg_ns=6049 max_ns=56946
  source_root_path: count=8095 total_ns=9067797 avg_ns=1120 max_ns=79449
  resolved_virtual_path: count=16218 total_ns=33600147 avg_ns=2071 max_ns=52880
  resolved_virtual_path_from_path: count=11347 total_ns=29822128 avg_ns=2628 max_ns=52880
  resolved_virtual_path_from_path_component_walk: count=11347 total_ns=26052324 avg_ns=2295 max_ns=52580
  resolved_virtual_path_from_path_canonicalize: count=24214 total_ns=21179164 avg_ns=874 max_ns=52207
  resolved_virtual_path_from_path_source_root_confinement: count=24214 total_ns=2816190 avg_ns=116 max_ns=30315
  resolved_virtual_path_from_path_virtual_conversion: count=11347 total_ns=3235481 avg_ns=285 max_ns=9836
  resolved_virtual_path_from_open_fd: count=4871 total_ns=3778019 avg_ns=775 max_ns=9036
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=15 total_ns=1048434 avg_ns=69895 max_ns=199108
  readdir_attr_generation_scan: count=15 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=15 total_ns=488223 avg_ns=32548 max_ns=99764
  readdir_candidate_selection: count=15 total_ns=11573 avg_ns=771 max_ns=2465
  readdir_page_commit: count=15 total_ns=98083 avg_ns=6538 max_ns=29514
  readdirplus_directory_scan: count=9 total_ns=14347145 avg_ns=1594127 max_ns=1941347
  readdirplus_attr_generation_scan: count=1561 total_ns=10165968 avg_ns=6512 max_ns=57005
  readdirplus_attr_generation_entries: count=1552
  readdirplus_symlink_visibility: count=9 total_ns=10691360 avg_ns=1187928 max_ns=1475756
  readdirplus_candidate_selection: count=9 total_ns=292840 avg_ns=32537 max_ns=84054
  readdirplus_page_commit: count=9 total_ns=854424 avg_ns=94936 max_ns=222302
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
