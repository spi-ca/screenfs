# ScreenFS benchmark result

- timestamp: `2026-06-21T02:12:25.459363+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label matcher32-policy-heavy --workload-set policy-heavy-matrix --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-matcher32-policy-heavy-matrix.json --output-md /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-matcher32-policy-heavy-matrix.md --output-svg /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-matcher32-policy-heavy-matrix.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/matcher/index.rs; ?? docs/artifacts/matcher-rank-zero-early-stop/; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `072027fe34bd1d72a3ebeb38854ac2646214260e6722fc67af7e08eae7a7c3d3`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/matcher/index.rs; ?? docs/artifacts/matcher-rank-zero-early-stop/; ... (+3 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher32-policy-heavy`
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
| metadata_lookup | 0.001894 | 0.033605 | 17.747 | 0.036291 | 0.036757 | 0.037130 |
| metadata_getattr | 0.002090 | 0.042664 | 20.417 | 0.045449 | 0.046148 | 0.046708 |
| metadata_access | 0.001810 | 0.043868 | 24.231 | 0.071873 | 0.072205 | 0.072470 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.140690 | 0.152905 | 0.155397 | 0.157391 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=171533836 avg_ns=25767 max_ns=90963
  fuse_op.getattr: count=6657 total_ns=119897251 avg_ns=18010 max_ns=224084
  fuse_op.lookup: count=137909 total_ns=1884354709 avg_ns=13663 max_ns=672288
  fuse_op.statfs: count=2 total_ns=7263 avg_ns=3631 max_ns=4841
  policy_decision: count=309102 total_ns=264157783 avg_ns=854 max_ns=334632
  matcher_candidates: count=6103152
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=6103152
  matcher_candidate_order.descendant: count=309102 total_ns=111255924 avg_ns=359 max_ns=124600
  matcher_candidate_order.path: count=927306 total_ns=268406861 avg_ns=289 max_ns=191251
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=30291996
  matcher_candidate_order_seen_slots.descendant: count=9891264
  matcher_candidate_order_seen_slots.path: count=20400732
  matcher_candidate_order_ancestor_steps: count=3237432
  matcher_candidate_order_ancestor_steps.descendant: count=809358
  matcher_candidate_order_ancestor_steps.path: count=2428074
  state_read_lock_wait: count=151223 total_ns=3171589 avg_ns=20 max_ns=4812
  state_read_lock_hold: count=151223 total_ns=9920476 avg_ns=65 max_ns=27113
  state_write_lock_wait: count=105251 total_ns=2142373 avg_ns=20 max_ns=15962
  state_write_lock_hold: count=105251 total_ns=34281405 avg_ns=325 max_ns=49779
  open_confined_openat2: count=157880 total_ns=87451855 avg_ns=553 max_ns=69240
  open_like.pre_open_guard.access: count=6657 total_ns=125356767 avg_ns=18830 max_ns=83394
  open_like.post_open_revalidation.access: count=6657 total_ns=37810325 avg_ns=5679 max_ns=39583
  stat_child_no_follow: count=151223 total_ns=1181917472 avg_ns=7815 max_ns=415538
  source_root_path: count=151223 total_ns=247225935 avg_ns=1634 max_ns=205868
  resolved_virtual_path: count=427666 total_ns=690944529 avg_ns=1615 max_ns=405175
  resolved_virtual_path_from_path: count=269787 total_ns=537522120 avg_ns=1992 max_ns=331760
  resolved_virtual_path_from_path_component_walk: count=269787 total_ns=445721625 avg_ns=1652 max_ns=331418
  resolved_virtual_path_from_path_canonicalize: count=382318 total_ns=352865677 avg_ns=922 max_ns=315601
  resolved_virtual_path_from_path_source_root_confinement: count=382318 total_ns=47982017 avg_ns=125 max_ns=60026
  resolved_virtual_path_from_path_virtual_conversion: count=269787 total_ns=76691320 avg_ns=284 max_ns=282589
  resolved_virtual_path_from_open_fd: count=157879 total_ns=153422409 avg_ns=971 max_ns=405175
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
