# ScreenFS benchmark result

- timestamp: `2026-06-20T17:04:26.442059+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload readdirplus_basic --dir-entries 20000 --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.json --output-md docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.md --output-svg docs/artifacts/readdirplus-20k-policy-ablation/current-fallback-unsafe-policy-readdirplus-basic-20k.svg`
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
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdirplus_basic | 0.016032 | 2.219389 | 138.435 | 2.375610 | 2.401589 | 2.422372 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=121219 avg_ns=121219 max_ns=121219
  fuse_op.getattr: count=260014 total_ns=4055067287 avg_ns=15595 max_ns=1051493
  fuse_op.lookup: count=780031 total_ns=9942605550 avg_ns=12746 max_ns=1824319
  fuse_op.opendir: count=13 total_ns=269087 avg_ns=20699 max_ns=57050
  fuse_op.readdir: count=332 total_ns=8607742602 avg_ns=25926935 max_ns=54886206
  fuse_op.readdirplus: count=33 total_ns=1093385578 avg_ns=33132896 max_ns=56621899
  fuse_op.releasedir: count=13 total_ns=31300623 avg_ns=2407740 max_ns=3615450
  fuse_op.statfs: count=2 total_ns=8147 avg_ns=4073 max_ns=4323
  policy_decision: count=5737598 total_ns=2805410440 avg_ns=488 max_ns=1028753
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=5737598 total_ns=799976468 avg_ns=139 max_ns=480246
  matcher_candidate_order.path: count=17212794 total_ns=3056621318 avg_ns=177 max_ns=356181
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5737598
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=5737598
  matcher_candidate_order_ancestor_steps: count=81373824
  matcher_candidate_order_ancestor_steps.descendant: count=20343456
  matcher_candidate_order_ancestor_steps.path: count=61030368
  state_read_lock_wait: count=1040424 total_ns=23119193 avg_ns=22 max_ns=17801
  state_read_lock_hold: count=1040424 total_ns=69679596 avg_ns=66 max_ns=301840
  state_write_lock_wait: count=780433 total_ns=16559373 avg_ns=21 max_ns=7967
  state_write_lock_hold: count=780433 total_ns=870293347 avg_ns=1115 max_ns=3614645
  open_confined_openat2: count=1046710 total_ns=593273585 avg_ns=566 max_ns=1345417
  open_like.pre_open_guard.access: count=1 total_ns=106880 avg_ns=106880 max_ns=106880
  open_like.pre_open_guard.opendir: count=13 total_ns=197390 avg_ns=15183 max_ns=49303
  open_like.post_open_revalidation.access: count=1 total_ns=7899 avg_ns=7899 max_ns=7899
  open_like.post_open_revalidation.opendir: count=13 total_ns=41165 avg_ns=3166 max_ns=5671
  stat_child_no_follow: count=1046331 total_ns=7763199321 avg_ns=7419 max_ns=1811031
  source_root_path: count=1046696 total_ns=1705314639 avg_ns=1629 max_ns=759391
  resolved_virtual_path: count=3133122 total_ns=6405235143 avg_ns=2044 max_ns=1799878
  resolved_virtual_path_from_path: count=2086413 total_ns=5336120432 avg_ns=2557 max_ns=1799878
  resolved_virtual_path_from_path_component_walk: count=2086413 total_ns=4560025867 avg_ns=2185 max_ns=1798053
  resolved_virtual_path_from_path_canonicalize: count=3652688 total_ns=3618297571 avg_ns=990 max_ns=1797177
  resolved_virtual_path_from_path_source_root_confinement: count=3652688 total_ns=496854952 avg_ns=136 max_ns=287099
  resolved_virtual_path_from_path_virtual_conversion: count=2086413 total_ns=658389130 avg_ns=315 max_ns=147549
  resolved_virtual_path_from_open_fd: count=1046709 total_ns=1069114711 avg_ns=1021 max_ns=548075
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=8223101288 avg_ns=24768377 max_ns=53819622
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=541382941 avg_ns=1630671 max_ns=5766255
  readdir_page_commit: count=332 total_ns=319918922 avg_ns=963611 max_ns=2917118
  readdirplus_directory_scan: count=33 total_ns=1018163675 avg_ns=30853444 max_ns=54087066
  readdirplus_attr_generation_scan: count=6279 total_ns=48066025 avg_ns=7655 max_ns=67616
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=58856683 avg_ns=1783535 max_ns=3566400
  readdirplus_page_commit: count=33 total_ns=13789497 avg_ns=417863 max_ns=1374878
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
