# ScreenFS benchmark result

- timestamp: `2026-06-20T17:03:56.196651+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set directory-surface --dir-entries 5000 --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.json --output-md docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.md --output-svg docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-directory-surface-5k.svg`
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
| readdir_basic | 0.000697 | 0.052120 | 74.763 | 0.053975 | 0.056525 | 0.058564 |
| readdirplus_basic | 0.003417 | 0.343249 | 100.452 | 0.359732 | 0.360142 | 0.360470 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=62756 avg_ns=62756 max_ns=62756
  fuse_op.getattr: count=65027 total_ns=798266964 avg_ns=12275 max_ns=324377
  fuse_op.lookup: count=195057 total_ns=1938203462 avg_ns=9936 max_ns=689371
  fuse_op.opendir: count=26 total_ns=450420 avg_ns=17323 max_ns=59585
  fuse_op.readdir: count=180 total_ns=1017204941 avg_ns=5651138 max_ns=11568478
  fuse_op.readdirplus: count=31 total_ns=329477141 avg_ns=10628294 max_ns=12431703
  fuse_op.releasedir: count=26 total_ns=9238544 avg_ns=355328 max_ns=604652
  fuse_op.statfs: count=2 total_ns=2530 avg_ns=1265 max_ns=1498
  policy_decision: count=1099907 total_ns=420995861 avg_ns=382 max_ns=338776
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=121780224 avg_ns=110 max_ns=265184
  matcher_candidate_order.path: count=3299721 total_ns=463466781 avg_ns=140 max_ns=321329
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1099907
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1099907
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=4491467 avg_ns=17 max_ns=16391
  state_read_lock_hold: count=260322 total_ns=14550392 avg_ns=55 max_ns=14356
  state_write_lock_wait: count=195344 total_ns=3161876 avg_ns=16 max_ns=3687
  state_write_lock_hold: count=195344 total_ns=199380369 avg_ns=1020 max_ns=2178260
  open_confined_openat2: count=266229 total_ns=97629406 avg_ns=366 max_ns=58810
  open_like.pre_open_guard.access: count=1 total_ns=51954 avg_ns=51954 max_ns=51954
  open_like.pre_open_guard.opendir: count=26 total_ns=327645 avg_ns=12601 max_ns=45781
  open_like.post_open_revalidation.access: count=1 total_ns=4654 avg_ns=4654 max_ns=4654
  open_like.post_open_revalidation.opendir: count=26 total_ns=68784 avg_ns=2645 max_ns=3430
  stat_child_no_follow: count=265991 total_ns=1506498886 avg_ns=5663 max_ns=374575
  source_root_path: count=266202 total_ns=305790088 avg_ns=1148 max_ns=280909
  resolved_virtual_path: count=792379 total_ns=1273794619 avg_ns=1607 max_ns=368371
  resolved_virtual_path_from_path: count=526151 total_ns=1070893029 avg_ns=2035 max_ns=368371
  resolved_virtual_path_from_path_component_walk: count=526151 total_ns=917917458 avg_ns=1744 max_ns=354016
  resolved_virtual_path_from_path_canonicalize: count=922034 total_ns=728915819 avg_ns=790 max_ns=353307
  resolved_virtual_path_from_path_source_root_confinement: count=922034 total_ns=114445447 avg_ns=124 max_ns=92050
  resolved_virtual_path_from_path_virtual_conversion: count=526151 total_ns=128956853 avg_ns=245 max_ns=29740
  resolved_virtual_path_from_open_fd: count=266228 total_ns=202901590 avg_ns=762 max_ns=134957
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=896540262 avg_ns=4980779 max_ns=10890042
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=48615812 avg_ns=270087 max_ns=629493
  readdir_page_commit: count=180 total_ns=102437925 avg_ns=569099 max_ns=2178377
  readdirplus_directory_scan: count=31 total_ns=276325085 avg_ns=8913712 max_ns=10632517
  readdirplus_attr_generation_scan: count=5859 total_ns=38172888 avg_ns=6515 max_ns=157026
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=15799012 avg_ns=509645 max_ns=661485
  readdirplus_page_commit: count=31 total_ns=3923898 avg_ns=126577 max_ns=225214
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
