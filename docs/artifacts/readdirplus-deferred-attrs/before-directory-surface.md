# ScreenFS benchmark result

- timestamp: `2026-06-20T08:32:10.580207+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-surface --iterations 10 --warmups 3 --dir-entries 5000 --output-json docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.json --output-md docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.md --output-svg docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+34 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `0eb7e90b2f6428fbf2c56ac278461585516dc7bc2f5a6640e7c30e1622542f63`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+34 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.000656 | 0.070411 | 107.364 | 0.071205 | 0.071321 | 0.071414 |
| readdirplus_basic | 0.003381 | 0.463973 | 137.230 | 0.477989 | 0.478372 | 0.478679 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=44072 avg_ns=44072 max_ns=44072
  fuse_op.getattr: count=65027 total_ns=1070284613 avg_ns=16459 max_ns=96329
  fuse_op.lookup: count=195057 total_ns=2645577374 avg_ns=13563 max_ns=114563
  fuse_op.opendir: count=26 total_ns=611415 avg_ns=23515 max_ns=62603
  fuse_op.readdir: count=180 total_ns=1343105316 avg_ns=7461696 max_ns=15104251
  fuse_op.readdirplus: count=31 total_ns=445285864 avg_ns=14364060 max_ns=19458884
  fuse_op.releasedir: count=26 total_ns=14860203 avg_ns=571546 max_ns=1105168
  fuse_op.statfs: count=2 total_ns=4500 avg_ns=2250 max_ns=2648
  policy_decision: count=1088251 total_ns=534023043 avg_ns=490 max_ns=70569
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1088251 total_ns=153052900 avg_ns=140 max_ns=36866
  matcher_candidate_order.path: count=3264753 total_ns=584377013 avg_ns=178 max_ns=69785
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1088251
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1088251
  matcher_candidate_order_ancestor_steps: count=14808676
  matcher_candidate_order_ancestor_steps.descendant: count=3702169
  matcher_candidate_order_ancestor_steps.path: count=11106507
  state_read_lock_wait: count=260322 total_ns=6106511 avg_ns=23 max_ns=19164
  state_read_lock_hold: count=260322 total_ns=18523277 avg_ns=71 max_ns=8787
  state_write_lock_wait: count=195344 total_ns=4507906 avg_ns=23 max_ns=19320
  state_write_lock_hold: count=195344 total_ns=280518518 avg_ns=1436 max_ns=2100341
  open_confined_openat2: count=260401 total_ns=156222990 avg_ns=599 max_ns=64137
  stat_child_no_follow: count=260163 total_ns=2091350384 avg_ns=8038 max_ns=80472
  source_root_path: count=260400 total_ns=486931986 avg_ns=1869 max_ns=76826
  resolved_virtual_path: count=780723 total_ns=1673815733 avg_ns=2143 max_ns=88299
  resolved_virtual_path_from_path: count=520323 total_ns=1387944383 avg_ns=2667 max_ns=88299
  resolved_virtual_path_from_path_component_walk: count=520323 total_ns=1199008940 avg_ns=2304 max_ns=87643
  resolved_virtual_path_from_path_canonicalize: count=910378 total_ns=967621972 avg_ns=1062 max_ns=63590
  resolved_virtual_path_from_path_source_root_confinement: count=910378 total_ns=124515071 avg_ns=136 max_ns=83943
  resolved_virtual_path_from_path_virtual_conversion: count=520323 total_ns=157700144 avg_ns=303 max_ns=54234
  resolved_virtual_path_from_open_fd: count=260400 total_ns=285871350 avg_ns=1097 max_ns=55730
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1178459209 avg_ns=6546995 max_ns=14206656
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=68812018 avg_ns=382288 max_ns=1029284
  readdir_page_commit: count=180 total_ns=136876126 avg_ns=760422 max_ns=2100441
  readdirplus_directory_scan: count=31 total_ns=438197292 avg_ns=14135396 max_ns=19225535
  readdirplus_attr_generation_scan: count=31 total_ns=112753590 avg_ns=3637212 max_ns=5164796
  readdirplus_attr_generation_entries: count=139868
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=23569084 avg_ns=760293 max_ns=1029725
  readdirplus_page_commit: count=31 total_ns=5267939 avg_ns=169933 max_ns=243259
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
