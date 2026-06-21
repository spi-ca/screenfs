# ScreenFS benchmark result

- timestamp: `2026-06-21T21:10:44.221785+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload metadata_opendir --iterations 3 --warmups 1 --metadata-ops 200 --output-json docs/artifacts/current-metadata-opendir-smoke.json --output-md docs/artifacts/current-metadata-opendir-smoke.md --output-svg docs/artifacts/current-metadata-opendir-smoke.svg`
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
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_opendir`
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
| metadata_opendir | 0.000790 | 0.012060 | 15.266 | 0.012660 | 0.012735 | 0.012795 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=493033 avg_ns=493033 max_ns=493033
  fuse_op.getattr: count=801 total_ns=9021231 avg_ns=11262 max_ns=168765
  fuse_op.lookup: count=1605 total_ns=16163180 avg_ns=10070 max_ns=247420
  fuse_op.opendir: count=800 total_ns=11768624 avg_ns=14710 max_ns=76512
  fuse_op.releasedir: count=800 total_ns=173336 avg_ns=216 max_ns=1646
  fuse_op.statfs: count=2 total_ns=3213 avg_ns=1606 max_ns=1958
  policy_decision: count=7214 total_ns=2116924 avg_ns=293 max_ns=11362
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=7214 total_ns=626484 avg_ns=86 max_ns=1425
  matcher_candidate_order.path: count=21642 total_ns=2441027 avg_ns=112 max_ns=2178
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7214
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=7214
  matcher_candidate_order_ancestor_steps: count=67320
  matcher_candidate_order_ancestor_steps.descendant: count=16830
  matcher_candidate_order_ancestor_steps.path: count=50490
  state_read_lock_wait: count=3207 total_ns=65467 avg_ns=20 max_ns=711
  state_read_lock_hold: count=3207 total_ns=155641 avg_ns=48 max_ns=3583
  state_write_lock_wait: count=3203 total_ns=60627 avg_ns=18 max_ns=557
  state_write_lock_hold: count=3203 total_ns=414848 avg_ns=129 max_ns=16226
  open_confined_openat2: count=4008 total_ns=1804429 avg_ns=450 max_ns=29058
  open_like.pre_open_guard.access: count=1 total_ns=63073 avg_ns=63073 max_ns=63073
  open_like.pre_open_guard.opendir: count=800 total_ns=8716256 avg_ns=10895 max_ns=64494
  open_like.post_open_revalidation.access: count=1 total_ns=422347 avg_ns=422347 max_ns=422347
  open_like.post_open_revalidation.opendir: count=800 total_ns=2148371 avg_ns=2685 max_ns=7925
  stat_child_no_follow: count=3207 total_ns=19736380 avg_ns=6154 max_ns=233773
  stat_child_no_follow.attr_conversion: count=3205 total_ns=47820 avg_ns=14 max_ns=692
  stat_child_no_follow.directory_revalidation: count=3206 total_ns=15618029 avg_ns=4871 max_ns=230675
  stat_child_no_follow.host_fstat: count=1 total_ns=1789 avg_ns=1789 max_ns=1789
  stat_child_no_follow.host_fstatat: count=3206 total_ns=895697 avg_ns=279 max_ns=85402
  stat_child_no_follow.parent_open: count=3206 total_ns=1980583 avg_ns=617 max_ns=44496
  stat_child_no_follow_context.path_guard_or_metadata: count=3207 total_ns=19736380 avg_ns=6154 max_ns=233773
  source_root_path: count=3207 total_ns=4543772 avg_ns=1416 max_ns=67851
  resolved_virtual_path: count=10418 total_ns=15205218 avg_ns=1459 max_ns=415747
  resolved_virtual_path_from_path: count=6411 total_ns=11154595 avg_ns=1739 max_ns=222526
  resolved_virtual_path_from_path_component_walk: count=6411 total_ns=9108581 avg_ns=1420 max_ns=221416
  resolved_virtual_path_from_path_canonicalize: count=8014 total_ns=7058100 avg_ns=880 max_ns=220742
  resolved_virtual_path_from_path_source_root_confinement: count=8014 total_ns=1161267 avg_ns=144 max_ns=23332
  resolved_virtual_path_from_path_virtual_conversion: count=6411 total_ns=1688013 avg_ns=263 max_ns=2282
  resolved_virtual_path_from_open_fd: count=4007 total_ns=4050623 avg_ns=1010 max_ns=415747
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
