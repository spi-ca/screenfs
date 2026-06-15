# ScreenFS benchmark result

- timestamp: `2026-06-15T22:44:26.502273+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-postmeta-target-perf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --workload-set directory-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface-perf.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001268 | 0.075667 | 59.652 | 0.078527 | 0.078865 | 0.079135 |
| readdirplus_basic | 0.008404 | 0.740428 | 88.099 | 0.748075 | 0.749103 | 0.749925 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=45987 avg_ns=45987 max_ns=45987
  fuse_op.getattr: count=65027 total_ns=1922276023 avg_ns=29561 max_ns=72277
  fuse_op.lookup: count=195057 total_ns=4452391601 avg_ns=22826 max_ns=210458
  fuse_op.opendir: count=26 total_ns=910062 avg_ns=35002 max_ns=55204
  fuse_op.readdir: count=180 total_ns=1435073863 avg_ns=7972632 max_ns=17561365
  fuse_op.readdirplus: count=31 total_ns=502236655 avg_ns=16201182 max_ns=18669603
  fuse_op.releasedir: count=26 total_ns=10866952 avg_ns=417959 max_ns=612019
  fuse_op.statfs: count=2 total_ns=3727 avg_ns=1863 max_ns=2254
  policy_decision: count=1088251 total_ns=739496830 avg_ns=679 max_ns=208795
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1088251 total_ns=163242876 avg_ns=150 max_ns=181393
  matcher_candidate_order.path: count=3264753 total_ns=808958690 avg_ns=247 max_ns=178358
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=35912283
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=35912283
  matcher_candidate_order_ancestor_steps: count=14808676
  matcher_candidate_order_ancestor_steps.descendant: count=3702169
  matcher_candidate_order_ancestor_steps.path: count=11106507
  state_read_lock_wait: count=260322 total_ns=6716793 avg_ns=25 max_ns=6142
  state_read_lock_hold: count=260322 total_ns=30177635 avg_ns=115 max_ns=8697
  state_write_lock_wait: count=195344 total_ns=5118059 avg_ns=26 max_ns=7544
  state_write_lock_hold: count=195344 total_ns=303686908 avg_ns=1554 max_ns=2701634
  open_confined_openat2: count=260401 total_ns=273251673 avg_ns=1049 max_ns=11492
  stat_child_no_follow: count=260163 total_ns=3618611631 avg_ns=13909 max_ns=66195
  source_root_path: count=260400 total_ns=960086703 avg_ns=3686 max_ns=25218
  resolved_virtual_path: count=780723 total_ns=3432453432 avg_ns=4396 max_ns=190300
  resolved_virtual_path_from_path: count=520323 total_ns=2923985697 avg_ns=5619 max_ns=190300
  resolved_virtual_path_from_path_component_walk: count=520323 total_ns=2698725199 avg_ns=5186 max_ns=182706
  resolved_virtual_path_from_path_canonicalize: count=910378 total_ns=2442670634 avg_ns=2683 max_ns=31378
  resolved_virtual_path_from_path_source_root_confinement: count=910378 total_ns=137317176 avg_ns=150 max_ns=12755
  resolved_virtual_path_from_path_virtual_conversion: count=520323 total_ns=186449330 avg_ns=358 max_ns=17503
  resolved_virtual_path_from_open_fd: count=260400 total_ns=508467735 avg_ns=1952 max_ns=20739
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1273988328 avg_ns=7077712 max_ns=15840016
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=62908701 avg_ns=349492 max_ns=953708
  readdir_page_commit: count=180 total_ns=141306317 avg_ns=785035 max_ns=2701764
  readdirplus_directory_scan: count=31 total_ns=494871638 avg_ns=15963601 max_ns=18434888
  readdirplus_attr_generation_scan: count=31 total_ns=144752207 avg_ns=4669426 max_ns=5421306
  readdirplus_attr_generation_entries: count=139868
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=25111892 avg_ns=810061 max_ns=969205
  readdirplus_page_commit: count=31 total_ns=5362262 avg_ns=172976 max_ns=324705
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
