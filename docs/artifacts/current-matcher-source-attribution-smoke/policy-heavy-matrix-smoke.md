# ScreenFS benchmark result

- timestamp: `2026-06-24T03:13:26.366880+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload metadata_lookup --workload metadata_getattr --workload metadata_access --workload matcher_hidden_stat_miss --iterations 3 --warmups 1 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.json --output-md docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.md --output-svg docs/artifacts/current-matcher-source-attribution-smoke/policy-heavy-matrix-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+10 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs_workloads.py; ... (+10 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
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
| metadata_lookup | 0.002056 | 0.009522 | 4.632 | 0.010097 | 0.010169 | 0.010227 |
| metadata_getattr | 0.002345 | 0.011987 | 5.113 | 0.012036 | 0.012042 | 0.012047 |
| metadata_access | 0.001917 | 0.012367 | 6.452 | 0.013566 | 0.013716 | 0.013835 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.008297 | 0.008426 | 0.008442 | 0.008455 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=12967207 avg_ns=16188 max_ns=93899
  fuse_op.getattr: count=801 total_ns=8464829 avg_ns=10567 max_ns=53023
  fuse_op.lookup: count=9605 total_ns=80386256 avg_ns=8369 max_ns=89623
  fuse_op.statfs: count=2 total_ns=3426 avg_ns=1713 max_ns=2008
  policy_decision: count=12008 total_ns=12549046 avg_ns=1045 max_ns=52809
  matcher_candidates: count=128896
  matcher_candidates_by_source.hidden.path: count=800
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=128096
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=128896
  matcher_candidate_order.descendant: count=12008 total_ns=5153210 avg_ns=429 max_ns=51881
  matcher_candidate_order.path: count=36024 total_ns=13200127 avg_ns=366 max_ns=17486
  matcher_candidate_order_by_source.hidden.path: count=12008 total_ns=6128931 avg_ns=510 max_ns=16496
  matcher_candidate_order_by_source.internal_hidden.path: count=12008 total_ns=1560300 avg_ns=129 max_ns=4748
  matcher_candidate_order_by_source.visible.descendant: count=12008 total_ns=5153210 avg_ns=429 max_ns=51881
  matcher_candidate_order_by_source.visible.path: count=12008 total_ns=5510896 avg_ns=458 max_ns=17486
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1176784
  matcher_candidate_order_seen_slots.descendant: count=384256
  matcher_candidate_order_seen_slots.path: count=792528
  matcher_candidate_order_ancestor_steps: count=153676
  matcher_candidate_order_ancestor_steps.descendant: count=38419
  matcher_candidate_order_ancestor_steps.path: count=115257
  state_read_lock_wait: count=11207 total_ns=205107 avg_ns=18 max_ns=302
  state_read_lock_hold: count=11207 total_ns=642857 avg_ns=57 max_ns=4569
  state_write_lock_wait: count=8003 total_ns=137948 avg_ns=17 max_ns=192
  state_write_lock_hold: count=8003 total_ns=2180719 avg_ns=272 max_ns=6120
  open_confined_openat2: count=12008 total_ns=6620639 avg_ns=551 max_ns=18192
  open_like.pre_open_guard.access: count=801 total_ns=8119572 avg_ns=10136 max_ns=76907
  open_like.post_open_revalidation.access: count=801 total_ns=4082458 avg_ns=5096 max_ns=22752
  stat_child_no_follow: count=11207 total_ns=10998466 avg_ns=981 max_ns=18783
  stat_child_no_follow.attr_conversion: count=10405 total_ns=140416 avg_ns=13 max_ns=112
  stat_child_no_follow.host_fstat: count=10405 total_ns=1512959 avg_ns=145 max_ns=2531
  stat_child_no_follow_context.path_guard_or_metadata: count=11207 total_ns=10998466 avg_ns=981 max_ns=18783
  source_root_path: count=10407 total_ns=13725971 avg_ns=1318 max_ns=37004
  resolved_virtual_path: count=10406 total_ns=24477272 avg_ns=2352 max_ns=31521
  resolved_virtual_path_from_path: count=9605 total_ns=23766354 avg_ns=2474 max_ns=31521
  resolved_virtual_path_from_path_component_walk: count=9605 total_ns=20406564 avg_ns=2124 max_ns=20876
  resolved_virtual_path_from_path_canonicalize: count=19209 total_ns=15949673 avg_ns=830 max_ns=17913
  resolved_virtual_path_from_path_source_root_confinement: count=19209 total_ns=2789124 avg_ns=145 max_ns=3638
  resolved_virtual_path_from_path_virtual_conversion: count=9605 total_ns=2877600 avg_ns=299 max_ns=9812
  resolved_virtual_path_from_open_fd: count=801 total_ns=710918 avg_ns=887 max_ns=7027
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
