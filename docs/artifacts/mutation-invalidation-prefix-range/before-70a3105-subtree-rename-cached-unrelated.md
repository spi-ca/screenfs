# ScreenFS benchmark result

- timestamp: `2026-06-21T02:31:47.996927+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-prefix-range-before.XsiQ3s/target/release/screenfs --screenfs-source-root /tmp/screenfs-prefix-range-before.XsiQ3s --perf-counters --policy-preset fallback-unsafe-policy --workload subtree_rename_cached_unrelated --small-files 200 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-subtree-rename-cached-unrelated.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-subtree-rename-cached-unrelated.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-subtree-rename-cached-unrelated.svg`
- harness_repo_root: `/tmp/screenfs-prefix-range-before.XsiQ3s`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-prefix-range-before.XsiQ3s/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-prefix-range-before.XsiQ3s`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `subtree_rename_cached_unrelated`
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

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| subtree_rename_cached_unrelated | 0.020531 | 0.024637 | 0.025056 | 0.025391 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=113595 avg_ns=113595 max_ns=113595
  fuse_op.getattr: count=2679 total_ns=48772330 avg_ns=18205 max_ns=237445
  fuse_op.lookup: count=10937 total_ns=146405926 avg_ns=13386 max_ns=112268
  fuse_op.rename: count=26 total_ns=4157996 avg_ns=159922 max_ns=279328
  fuse_op.statfs: count=2 total_ns=3819 avg_ns=1909 max_ns=2798
  policy_decision: count=27962 total_ns=12020860 avg_ns=429 max_ns=8368
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=27806 total_ns=3534551 avg_ns=127 max_ns=26207
  matcher_candidate_order.path: count=83730 total_ns=14072247 avg_ns=168 max_ns=220530
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=27962
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=27962
  matcher_candidate_order_ancestor_steps: count=365600
  matcher_candidate_order_ancestor_steps.descendant: count=91140
  matcher_candidate_order_ancestor_steps.path: count=274460
  state_read_lock_wait: count=13669 total_ns=264061 avg_ns=19 max_ns=544
  state_read_lock_hold: count=13669 total_ns=891817 avg_ns=65 max_ns=12509
  state_write_lock_wait: count=10909 total_ns=197638 avg_ns=18 max_ns=274
  state_write_lock_hold: count=10909 total_ns=5479466 avg_ns=502 max_ns=82621
  open_confined_openat2: count=13878 total_ns=7261492 avg_ns=523 max_ns=48806
  open_like.pre_open_guard.access: count=1 total_ns=99490 avg_ns=99490 max_ns=99490
  open_like.post_open_revalidation.access: count=1 total_ns=8675 avg_ns=8675 max_ns=8675
  stat_child_no_follow: count=13825 total_ns=107030106 avg_ns=7741 max_ns=229104
  source_root_path: count=13799 total_ns=19686299 avg_ns=1426 max_ns=50574
  resolved_virtual_path: count=41420 total_ns=97489060 avg_ns=2353 max_ns=93931
  resolved_virtual_path_from_path: count=27543 total_ns=85017486 avg_ns=3086 max_ns=93931
  resolved_virtual_path_from_path_component_walk: count=27543 total_ns=75107838 avg_ns=2726 max_ns=93536
  resolved_virtual_path_from_path_canonicalize: count=62604 total_ns=60757256 avg_ns=970 max_ns=92917
  resolved_virtual_path_from_path_source_root_confinement: count=62604 total_ns=8354747 avg_ns=133 max_ns=27719
  resolved_virtual_path_from_path_virtual_conversion: count=27543 total_ns=8499843 avg_ns=308 max_ns=12964
  resolved_virtual_path_from_open_fd: count=13877 total_ns=12471574 avg_ns=898 max_ns=21825
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
  invalidations: count=26 invalidated_entries=52 evicted_entries=0 scanned_entries=10764
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
