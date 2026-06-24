# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:35.436245+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/after-policy-heavy-matrix.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/after-policy-heavy-matrix.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/after-policy-heavy-matrix.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss, matcher_readonly_access_wok`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.001744 | 0.011893 | 6.819 | 0.012193 | 0.012359 | 0.012492 |
| metadata_getattr | 0.001938 | 0.016107 | 8.312 | 0.016934 | 0.017011 | 0.017073 |
| metadata_access | 0.001720 | 0.017091 | 9.938 | 0.018948 | 0.019198 | 0.019398 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009455 | 0.009654 | 0.009876 | 0.010053 |
| matcher_readonly_access_wok | 0.023355 | 0.024847 | 0.024907 | 0.024955 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=124293200 avg_ns=23897 max_ns=126974
  fuse_op.getattr: count=3017 total_ns=42612884 avg_ns=14124 max_ns=57990
  fuse_op.lookup: count=43269 total_ns=489038150 avg_ns=11302 max_ns=194880
  fuse_op.statfs: count=2 total_ns=7602 avg_ns=3801 max_ns=4836
  policy_decision: count=59288 total_ns=83760815 avg_ns=1412 max_ns=15050
  matcher_candidates: count=614320
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=2600
  matcher_candidates_by_source.visible.descendant: count=609120
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=614320
  matcher_candidate_order.descendant: count=56688 total_ns=33582130 avg_ns=592 max_ns=26383
  matcher_candidate_order.path: count=175264 total_ns=91154685 avg_ns=520 max_ns=9562
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=41275850 avg_ns=728 max_ns=9562
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9941244 avg_ns=175 max_ns=7893
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=3051022 avg_ns=1173 max_ns=4135
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=33582130 avg_ns=592 max_ns=26383
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=36305853 avg_ns=640 max_ns=6871
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=580716 avg_ns=223 max_ns=3498
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1222640 avg_ns=23 max_ns=3831
  state_read_lock_hold: count=51487 total_ns=3494543 avg_ns=67 max_ns=7871
  state_write_lock_wait: count=38067 total_ns=886924 avg_ns=23 max_ns=4023
  state_write_lock_hold: count=38067 total_ns=14562890 avg_ns=382 max_ns=184132
  open_confined_openat2: count=54088 total_ns=43845510 avg_ns=810 max_ns=57753
  open_like.pre_open_guard.access: count=5201 total_ns=102771190 avg_ns=19759 max_ns=104463
  open_like.post_open_revalidation.access: count=2601 total_ns=17132731 avg_ns=6586 max_ns=19929
  stat_child_no_follow: count=51487 total_ns=72804397 avg_ns=1414 max_ns=58342
  stat_child_no_follow.attr_conversion: count=48885 total_ns=845782 avg_ns=17 max_ns=124
  stat_child_no_follow.host_fstat: count=48885 total_ns=11758991 avg_ns=240 max_ns=5875
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=72804397 avg_ns=1414 max_ns=58342
  source_root_path: count=48887 total_ns=96462102 avg_ns=1973 max_ns=71705
  resolved_virtual_path: count=48886 total_ns=167908504 avg_ns=3434 max_ns=23691
  resolved_virtual_path_from_path: count=46285 total_ns=164720742 avg_ns=3558 max_ns=23691
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=144276330 avg_ns=3117 max_ns=22747
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=116577136 avg_ns=1114 max_ns=13220
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=15593532 avg_ns=149 max_ns=8863
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17503486 avg_ns=378 max_ns=9409
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3187762 avg_ns=1225 max_ns=7738
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
