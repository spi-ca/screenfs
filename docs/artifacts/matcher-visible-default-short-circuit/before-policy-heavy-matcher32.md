# ScreenFS benchmark result

- timestamp: `2026-06-24T02:25:04.077820+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-visible-default-shortcircuit/before-policy-heavy-matcher32.json --output-md /tmp/screenfs-visible-default-shortcircuit/before-policy-heavy-matcher32.md --output-svg /tmp/screenfs-visible-default-shortcircuit/before-policy-heavy-matcher32.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-visible-before-73522`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/performance-roadmap.md;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/perf.rs; ... (+1 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `367b50287307eb569df4a3d08e42e6db25bf4e7cd1b9db1e3aa5b8fe2be4e673`
- screenfs_source_root: `/tmp/screenfs-visible-before-73522`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/performance-roadmap.md;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/perf.rs; ... (+1 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
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
| metadata_lookup | 0.001956 | 0.008628 | 4.411 | 0.009399 | 0.009868 | 0.010244 |
| metadata_getattr | 0.002197 | 0.011526 | 5.247 | 0.012246 | 0.012681 | 0.013030 |
| metadata_access | 0.001355 | 0.012991 | 9.588 | 0.013529 | 0.013756 | 0.013937 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007832 | 0.008098 | 0.008325 | 0.008506 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=43827736 avg_ns=16850 max_ns=273974
  fuse_op.getattr: count=2601 total_ns=26934638 avg_ns=10355 max_ns=50915
  fuse_op.lookup: count=31205 total_ns=254468115 avg_ns=8154 max_ns=114974
  fuse_op.statfs: count=2 total_ns=4940 avg_ns=2470 max_ns=3825
  policy_decision: count=39008 total_ns=40552151 avg_ns=1039 max_ns=36013
  matcher_candidates: count=418696
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=416096
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=418696
  matcher_candidate_order.descendant: count=39008 total_ns=16220830 avg_ns=415 max_ns=59061
  matcher_candidate_order.path: count=117024 total_ns=42527798 avg_ns=363 max_ns=30564
  matcher_candidate_order_by_source.hidden.path: count=39008 total_ns=19722453 avg_ns=505 max_ns=30564
  matcher_candidate_order_by_source.internal_hidden.path: count=39008 total_ns=4943100 avg_ns=126 max_ns=18676
  matcher_candidate_order_by_source.visible.descendant: count=39008 total_ns=16220830 avg_ns=415 max_ns=59061
  matcher_candidate_order_by_source.visible.path: count=39008 total_ns=17862245 avg_ns=457 max_ns=18637
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3822784
  matcher_candidate_order_seen_slots.descendant: count=1248256
  matcher_candidate_order_seen_slots.path: count=2574528
  matcher_candidate_order_ancestor_steps: count=499276
  matcher_candidate_order_ancestor_steps.descendant: count=124819
  matcher_candidate_order_ancestor_steps.path: count=374457
  state_read_lock_wait: count=36407 total_ns=653512 avg_ns=17 max_ns=889
  state_read_lock_hold: count=36407 total_ns=1958317 avg_ns=53 max_ns=14078
  state_write_lock_wait: count=26003 total_ns=443816 avg_ns=17 max_ns=161
  state_write_lock_hold: count=26003 total_ns=6667248 avg_ns=256 max_ns=49258
  open_confined_openat2: count=39008 total_ns=20245570 avg_ns=519 max_ns=50668
  open_like.pre_open_guard.access: count=2601 total_ns=27675820 avg_ns=10640 max_ns=266480
  open_like.post_open_revalidation.access: count=2601 total_ns=13524197 avg_ns=5199 max_ns=21192
  stat_child_no_follow: count=36407 total_ns=34070790 avg_ns=935 max_ns=57676
  stat_child_no_follow.attr_conversion: count=33805 total_ns=455629 avg_ns=13 max_ns=264
  stat_child_no_follow.host_fstat: count=33805 total_ns=4754085 avg_ns=140 max_ns=6204
  stat_child_no_follow_context.path_guard_or_metadata: count=36407 total_ns=34070790 avg_ns=935 max_ns=57676
  source_root_path: count=33807 total_ns=42926333 avg_ns=1269 max_ns=256807
  resolved_virtual_path: count=33806 total_ns=80010093 avg_ns=2366 max_ns=79485
  resolved_virtual_path_from_path: count=31205 total_ns=77698046 avg_ns=2489 max_ns=79485
  resolved_virtual_path_from_path_component_walk: count=31205 total_ns=66875285 avg_ns=2143 max_ns=79184
  resolved_virtual_path_from_path_canonicalize: count=62409 total_ns=52394994 avg_ns=839 max_ns=78730
  resolved_virtual_path_from_path_source_root_confinement: count=62409 total_ns=9111279 avg_ns=145 max_ns=15889
  resolved_virtual_path_from_path_virtual_conversion: count=31205 total_ns=9252299 avg_ns=296 max_ns=5050
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2312047 avg_ns=888 max_ns=6630
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
