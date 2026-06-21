# ScreenFS benchmark result

- timestamp: `2026-06-21T21:10:42.811311+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set open-confined-surface --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-open-confined-surface-smoke.json --output-md docs/artifacts/current-open-confined-surface-smoke.md --output-svg docs/artifacts/current-open-confined-surface-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `542eec53b6c147decc2d4ddcfd97f59c0824433e`
- git_dirty_status: `M .pi/extensions/guardrails.json;  M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `ca7372519260f85471e13bfc19189bf974d6fc0a973eb7e02270315c8d89cb97`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `542eec53b6c147decc2d4ddcfd97f59c0824433e`
- screenfs_source_git_dirty_status: `M .pi/extensions/guardrails.json;  M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md; ... (+9 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `open-confined-surface`
- comparable_workloads: `metadata_open, metadata_opendir`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_open | 0.001813 | 0.021104 | 11.639 | 0.021423 | 0.021463 | 0.021495 |
| metadata_opendir | 0.000516 | 0.015274 | 29.611 | 0.015739 | 0.015797 | 0.015844 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=81325 avg_ns=81325 max_ns=81325
  fuse_op.getattr: count=801 total_ns=9993226 avg_ns=12475 max_ns=47346
  fuse_op.lookup: count=4005 total_ns=73790995 avg_ns=18424 max_ns=10290066
  fuse_op.open: count=800 total_ns=17830971 avg_ns=22288 max_ns=91749
  fuse_op.opendir: count=800 total_ns=13391180 avg_ns=16738 max_ns=70987
  fuse_op.release: count=800 total_ns=938149 avg_ns=1172 max_ns=4614
  fuse_op.releasedir: count=800 total_ns=249215 avg_ns=311 max_ns=2488
  fuse_op.statfs: count=2 total_ns=7648 avg_ns=3824 max_ns=5228
  policy_decision: count=14414 total_ns=5451950 avg_ns=378 max_ns=11158
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=14414 total_ns=1624735 avg_ns=112 max_ns=657
  matcher_candidate_order.path: count=43242 total_ns=6284856 avg_ns=145 max_ns=4183
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=14414
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=14414
  matcher_candidate_order_ancestor_steps: count=150520
  matcher_candidate_order_ancestor_steps.descendant: count=37630
  matcher_candidate_order_ancestor_steps.path: count=112890
  state_read_lock_wait: count=6407 total_ns=152877 avg_ns=23 max_ns=350
  state_read_lock_hold: count=6407 total_ns=514518 avg_ns=80 max_ns=4261
  state_write_lock_wait: count=7203 total_ns=168296 avg_ns=23 max_ns=356
  state_write_lock_hold: count=7203 total_ns=2369232 avg_ns=328 max_ns=51557
  open_confined_openat2: count=8008 total_ns=24832783 avg_ns=3100 max_ns=10241310
  open_like.pre_open_guard.access: count=1 total_ns=65277 avg_ns=65277 max_ns=65277
  open_like.pre_open_guard.open: count=800 total_ns=13698993 avg_ns=17123 max_ns=86074
  open_like.pre_open_guard.opendir: count=800 total_ns=9899062 avg_ns=12373 max_ns=65759
  open_like.post_open_revalidation.access: count=1 total_ns=5647 avg_ns=5647 max_ns=5647
  open_like.post_open_revalidation.open: count=800 total_ns=2896039 avg_ns=3620 max_ns=27064
  open_like.post_open_revalidation.opendir: count=800 total_ns=2393038 avg_ns=2991 max_ns=13912
  stat_child_no_follow: count=6407 total_ns=71934271 avg_ns=11227 max_ns=10281277
  stat_child_no_follow.attr_conversion: count=6405 total_ns=111342 avg_ns=17 max_ns=45
  stat_child_no_follow.directory_revalidation: count=6406 total_ns=40361300 avg_ns=6300 max_ns=75381
  stat_child_no_follow.host_fstat: count=1 total_ns=2495 avg_ns=2495 max_ns=2495
  stat_child_no_follow.host_fstatat: count=6406 total_ns=2938781 avg_ns=458 max_ns=14722
  stat_child_no_follow.parent_open: count=6406 total_ns=25163039 avg_ns=3928 max_ns=10243251
  stat_child_no_follow_context.path_guard_or_metadata: count=6407 total_ns=71934271 avg_ns=11227 max_ns=10281277
  source_root_path: count=6407 total_ns=12582204 avg_ns=1963 max_ns=33273
  resolved_virtual_path: count=20818 total_ns=39311823 avg_ns=1888 max_ns=62993
  resolved_virtual_path_from_path: count=12811 total_ns=29436031 avg_ns=2297 max_ns=62993
  resolved_virtual_path_from_path_component_walk: count=12811 total_ns=24772524 avg_ns=1933 max_ns=19392
  resolved_virtual_path_from_path_canonicalize: count=19214 total_ns=19686446 avg_ns=1024 max_ns=14765
  resolved_virtual_path_from_path_source_root_confinement: count=19214 total_ns=2606179 avg_ns=135 max_ns=5519
  resolved_virtual_path_from_path_virtual_conversion: count=12811 total_ns=3884167 avg_ns=303 max_ns=56766
  resolved_virtual_path_from_open_fd: count=8007 total_ns=9875792 avg_ns=1233 max_ns=55538
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
