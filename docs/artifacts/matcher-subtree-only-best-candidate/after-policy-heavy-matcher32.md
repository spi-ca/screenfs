# ScreenFS benchmark result

- timestamp: `2026-06-24T02:08:21.180204+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-subtree-fastpath/after-policy-heavy-matcher32.json --output-md /tmp/screenfs-matcher-subtree-fastpath/after-policy-heavy-matcher32.md --output-svg /tmp/screenfs-matcher-subtree-fastpath/after-policy-heavy-matcher32.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-matcher-after-66015`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M src/matcher/index.rs`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `c3d805f50de9f65c7a2d3c13f92a50e177e85e9d75293504c4557323aeb838e3`
- screenfs_source_root: `/tmp/screenfs-matcher-after-66015`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M src/matcher/index.rs`
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
| metadata_lookup | 0.001317 | 0.011167 | 8.479 | 0.011571 | 0.011633 | 0.011681 |
| metadata_getattr | 0.001417 | 0.015063 | 10.628 | 0.016085 | 0.016153 | 0.016208 |
| metadata_access | 0.001219 | 0.016699 | 13.698 | 0.018036 | 0.018104 | 0.018159 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.008958 | 0.010587 | 0.010844 | 0.011049 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=55750483 avg_ns=21434 max_ns=191661
  fuse_op.getattr: count=2601 total_ns=35190804 avg_ns=13529 max_ns=83482
  fuse_op.lookup: count=31205 total_ns=321193474 avg_ns=10293 max_ns=266796
  fuse_op.statfs: count=2 total_ns=3473 avg_ns=1736 max_ns=2435
  policy_decision: count=39008 total_ns=50624640 avg_ns=1297 max_ns=71403
  matcher_candidates: count=418696
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=418696
  matcher_candidate_order.descendant: count=39008 total_ns=21690407 avg_ns=556 max_ns=49933
  matcher_candidate_order.path: count=117024 total_ns=56717426 avg_ns=484 max_ns=65020
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3822784
  matcher_candidate_order_seen_slots.descendant: count=1248256
  matcher_candidate_order_seen_slots.path: count=2574528
  matcher_candidate_order_ancestor_steps: count=499276
  matcher_candidate_order_ancestor_steps.descendant: count=124819
  matcher_candidate_order_ancestor_steps.path: count=374457
  state_read_lock_wait: count=36407 total_ns=857053 avg_ns=23 max_ns=19392
  state_read_lock_hold: count=36407 total_ns=2390303 avg_ns=65 max_ns=30752
  state_write_lock_wait: count=26003 total_ns=606050 avg_ns=23 max_ns=20901
  state_write_lock_hold: count=26003 total_ns=8702416 avg_ns=334 max_ns=42356
  open_confined_openat2: count=39008 total_ns=31585211 avg_ns=809 max_ns=70925
  open_like.pre_open_guard.access: count=2601 total_ns=35043700 avg_ns=13473 max_ns=163107
  open_like.post_open_revalidation.access: count=2601 total_ns=17062466 avg_ns=6559 max_ns=75981
  stat_child_no_follow: count=36407 total_ns=50734421 avg_ns=1393 max_ns=71515
  stat_child_no_follow.attr_conversion: count=33805 total_ns=570624 avg_ns=16 max_ns=265
  stat_child_no_follow.host_fstat: count=33805 total_ns=8084700 avg_ns=239 max_ns=42186
  stat_child_no_follow_context.path_guard_or_metadata: count=36407 total_ns=50734421 avg_ns=1393 max_ns=71515
  source_root_path: count=33807 total_ns=62096299 avg_ns=1836 max_ns=74872
  resolved_virtual_path: count=33806 total_ns=98124963 avg_ns=2902 max_ns=263041
  resolved_virtual_path_from_path: count=31205 total_ns=95032940 avg_ns=3045 max_ns=263041
  resolved_virtual_path_from_path_component_walk: count=31205 total_ns=81924987 avg_ns=2625 max_ns=152400
  resolved_virtual_path_from_path_canonicalize: count=62409 total_ns=65194382 avg_ns=1044 max_ns=151605
  resolved_virtual_path_from_path_source_root_confinement: count=62409 total_ns=9376815 avg_ns=150 max_ns=37026
  resolved_virtual_path_from_path_virtual_conversion: count=31205 total_ns=11132647 avg_ns=356 max_ns=262176
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3092023 avg_ns=1188 max_ns=18308
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
