# ScreenFS benchmark result

- timestamp: `2026-06-15T22:44:02.844769+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-postmeta-target-perf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-directory-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-directory-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-directory-surface-perf.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-postmeta-target-perf/release/screenfs`
- screenfs_bin_sha256: `93aa3942c0f6ba6dcf017f3be6197edbcebaff2683ec0da4eef1cf61341d1a6e`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001078 | 0.059355 | 55.082 | 0.060096 | 0.061084 | 0.061875 |
| readdirplus_basic | 0.007894 | 0.532758 | 67.487 | 0.537473 | 0.539728 | 0.541533 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65944 avg_ns=65944 max_ns=65944
  fuse_op.getattr: count=65027 total_ns=1164439731 avg_ns=17907 max_ns=55264
  fuse_op.lookup: count=195057 total_ns=2911338035 avg_ns=14925 max_ns=65364
  fuse_op.opendir: count=26 total_ns=643028 avg_ns=24731 max_ns=39716
  fuse_op.readdir: count=180 total_ns=1104842392 avg_ns=6138013 max_ns=12303852
  fuse_op.readdirplus: count=31 total_ns=400023415 avg_ns=12903981 max_ns=16695316
  fuse_op.releasedir: count=26 total_ns=6593904 avg_ns=253611 max_ns=317270
  fuse_op.statfs: count=2 total_ns=3046 avg_ns=1523 max_ns=1603
  policy_decision: count=1088251 total_ns=454338783 avg_ns=417 max_ns=264470
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1088251 total_ns=159531942 avg_ns=146 max_ns=10770
  matcher_candidate_order.path: count=3264753 total_ns=502313733 avg_ns=153 max_ns=17964
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14808676
  matcher_candidate_order_ancestor_steps.descendant: count=3702169
  matcher_candidate_order_ancestor_steps.path: count=11106507
  state_read_lock_wait: count=260322 total_ns=6927668 avg_ns=26 max_ns=2836
  state_read_lock_hold: count=260322 total_ns=29704123 avg_ns=114 max_ns=3687
  state_write_lock_wait: count=195344 total_ns=4980664 avg_ns=25 max_ns=111
  state_write_lock_hold: count=195344 total_ns=284606790 avg_ns=1456 max_ns=2707065
  open_confined_openat2: count=260401 total_ns=264025834 avg_ns=1013 max_ns=12243
  stat_child_no_follow: count=260163 total_ns=3413249876 avg_ns=13119 max_ns=59763
  source_root_path: count=260400 total_ns=927490675 avg_ns=3561 max_ns=39615
  resolved_virtual_path: count=520562 total_ns=1485570125 avg_ns=2853 max_ns=21932
  resolved_virtual_path_from_path: count=260162 total_ns=992012153 avg_ns=3813 max_ns=21932
  resolved_virtual_path_from_path_component_walk: count=260162 total_ns=891503701 avg_ns=3426 max_ns=21341
  resolved_virtual_path_from_path_canonicalize: count=325109 total_ns=807374350 avg_ns=2483 max_ns=20829
  resolved_virtual_path_from_path_source_root_confinement: count=325109 total_ns=43359209 avg_ns=133 max_ns=9759
  resolved_virtual_path_from_path_virtual_conversion: count=260162 total_ns=81542069 avg_ns=313 max_ns=14167
  resolved_virtual_path_from_open_fd: count=260400 total_ns=493557972 avg_ns=1895 max_ns=19727
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=950588360 avg_ns=5281046 max_ns=10562916
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=54702972 avg_ns=303905 max_ns=789182
  readdir_page_commit: count=180 total_ns=137904893 avg_ns=766138 max_ns=2707175
  readdirplus_directory_scan: count=31 total_ns=393762564 avg_ns=12702018 max_ns=16486480
  readdirplus_attr_generation_scan: count=31 total_ns=129425816 avg_ns=4175026 max_ns=5323530
  readdirplus_attr_generation_entries: count=139868
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=24439725 avg_ns=788378 max_ns=1047895
  readdirplus_page_commit: count=31 total_ns=4690108 avg_ns=151293 max_ns=320397
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
