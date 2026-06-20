# ScreenFS benchmark result

- timestamp: `2026-06-20T17:03:50.143880+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload readdirplus_basic --dir-entries 20000 --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.json --output-md docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.md --output-svg docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+19 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+19 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.016505 | 1.923972 | 116.567 | 1.955062 | 1.956231 | 1.957167 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=48709 avg_ns=48709 max_ns=48709
  fuse_op.getattr: count=260014 total_ns=2665731290 avg_ns=10252 max_ns=335241
  fuse_op.lookup: count=780031 total_ns=7099039463 avg_ns=9100 max_ns=330304
  fuse_op.opendir: count=13 total_ns=201546 avg_ns=15503 max_ns=27529
  fuse_op.readdir: count=332 total_ns=7500113171 avg_ns=22590702 max_ns=46756734
  fuse_op.readdirplus: count=33 total_ns=966476747 avg_ns=29287174 max_ns=48094715
  fuse_op.releasedir: count=13 total_ns=24370315 avg_ns=1874639 max_ns=2825469
  fuse_op.statfs: count=2 total_ns=3555 avg_ns=1777 max_ns=1936
  policy_decision: count=5737598 total_ns=2195706003 avg_ns=382 max_ns=279985
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=5737598 total_ns=769993402 avg_ns=134 max_ns=85332
  matcher_candidate_order.path: count=17212794 total_ns=2413697451 avg_ns=140 max_ns=319163
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=81373824
  matcher_candidate_order_ancestor_steps.descendant: count=20343456
  matcher_candidate_order_ancestor_steps.path: count=61030368
  state_read_lock_wait: count=1040424 total_ns=21971974 avg_ns=21 max_ns=143001
  state_read_lock_hold: count=1040424 total_ns=64578277 avg_ns=62 max_ns=21701
  state_write_lock_wait: count=780433 total_ns=15811589 avg_ns=20 max_ns=7461
  state_write_lock_hold: count=780433 total_ns=836496008 avg_ns=1071 max_ns=2824870
  open_confined_openat2: count=1046710 total_ns=543342172 avg_ns=519 max_ns=175505
  open_like.pre_open_guard.access: count=1 total_ns=21967 avg_ns=21967 max_ns=21967
  open_like.pre_open_guard.opendir: count=13 total_ns=133449 avg_ns=10265 max_ns=21712
  open_like.post_open_revalidation.access: count=1 total_ns=22422 avg_ns=22422 max_ns=22422
  open_like.post_open_revalidation.opendir: count=13 total_ns=36738 avg_ns=2826 max_ns=3369
  stat_child_no_follow: count=1046331 total_ns=7329249429 avg_ns=7004 max_ns=331748
  source_root_path: count=1046696 total_ns=1635282342 avg_ns=1562 max_ns=296663
  resolved_virtual_path: count=2093039 total_ns=2853518581 avg_ns=1363 max_ns=323932
  resolved_virtual_path_from_path: count=1046330 total_ns=1869299801 avg_ns=1786 max_ns=323665
  resolved_virtual_path_from_path_component_walk: count=1046330 total_ns=1511476246 avg_ns=1444 max_ns=323081
  resolved_virtual_path_from_path_canonicalize: count=1312549 total_ns=1196952998 avg_ns=911 max_ns=265342
  resolved_virtual_path_from_path_source_root_confinement: count=1312549 total_ns=153972231 avg_ns=117 max_ns=20638
  resolved_virtual_path_from_path_virtual_conversion: count=1046330 total_ns=299496634 avg_ns=286 max_ns=92131
  resolved_virtual_path_from_open_fd: count=1046709 total_ns=984218780 avg_ns=940 max_ns=323932
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=7133560988 avg_ns=21486629 max_ns=45813852
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=486351766 avg_ns=1464914 max_ns=3535475
  readdir_page_commit: count=332 total_ns=306453096 avg_ns=923051 max_ns=2342536
  readdirplus_directory_scan: count=33 total_ns=899635104 avg_ns=27261669 max_ns=45761850
  readdirplus_attr_generation_scan: count=6279 total_ns=43509086 avg_ns=6929 max_ns=61829
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=60287368 avg_ns=1826889 max_ns=6374529
  readdirplus_page_commit: count=33 total_ns=12143782 avg_ns=367993 max_ns=914224
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
