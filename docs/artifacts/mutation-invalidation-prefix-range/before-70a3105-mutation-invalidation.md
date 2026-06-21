# ScreenFS benchmark result

- timestamp: `2026-06-21T02:31:47.004261+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-prefix-range-before.XsiQ3s/target/release/screenfs --screenfs-source-root /tmp/screenfs-prefix-range-before.XsiQ3s --perf-counters --policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --symlink-parent-mutations 20 --small-files 200 --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-mutation-invalidation.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-mutation-invalidation.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/mutation-invalidation-prefix-range/before-70a3105-mutation-invalidation.svg`
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
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir, subtree_rename_cached_unrelated`
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
| symlink_parent_mkdir_rmdir | 0.016054 | 0.016715 | 0.016900 | 0.017047 |
| pinned_symlink_parent_mkdir_rmdir | 0.022032 | 0.022883 | 0.023971 | 0.024842 |
| subtree_rename_cached_unrelated | 0.020159 | 0.021560 | 0.021683 | 0.021781 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=39959 avg_ns=39959 max_ns=39959
  fuse_op.getattr: count=3771 total_ns=63774950 avg_ns=16911 max_ns=127493
  fuse_op.lookup: count=27798 total_ns=355175044 avg_ns=12776 max_ns=266204
  fuse_op.mkdir: count=520 total_ns=45705505 avg_ns=87895 max_ns=225674
  fuse_op.opendir: count=273 total_ns=4567055 avg_ns=16729 max_ns=24591
  fuse_op.readdirplus: count=273 total_ns=15822690 avg_ns=57958 max_ns=79655
  fuse_op.readlink: count=2392 total_ns=42001673 avg_ns=17559 max_ns=67973
  fuse_op.releasedir: count=273 total_ns=150813 avg_ns=552 max_ns=3458
  fuse_op.rename: count=26 total_ns=3899654 avg_ns=149986 max_ns=228631
  fuse_op.rmdir: count=520 total_ns=35788018 avg_ns=68823 max_ns=124897
  fuse_op.statfs: count=2 total_ns=3529 avg_ns=1764 max_ns=2663
  policy_decision: count=95211 total_ns=40645743 avg_ns=426 max_ns=26668
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=91935 total_ns=11466058 avg_ns=124 max_ns=30511
  matcher_candidate_order.path: count=282357 total_ns=45612762 avg_ns=161 max_ns=116913
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=95211
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=95211
  matcher_candidate_order_ancestor_steps: count=1291408
  matcher_candidate_order_ancestor_steps.descendant: count=315832
  matcher_candidate_order_ancestor_steps.path: count=975576
  state_read_lock_wait: count=35600 total_ns=642989 avg_ns=18 max_ns=4668
  state_read_lock_hold: count=35600 total_ns=2203894 avg_ns=61 max_ns=4094
  state_write_lock_wait: count=28342 total_ns=486559 avg_ns=17 max_ns=263
  state_write_lock_hold: count=28342 total_ns=10679097 avg_ns=376 max_ns=55182
  open_confined_openat2: count=41841 total_ns=18763683 avg_ns=448 max_ns=50842
  open_like.pre_open_guard.access: count=1 total_ns=30883 avg_ns=30883 max_ns=30883
  open_like.pre_open_guard.opendir: count=273 total_ns=3506958 avg_ns=12846 max_ns=19986
  open_like.post_open_revalidation.access: count=1 total_ns=5461 avg_ns=5461 max_ns=5461
  open_like.post_open_revalidation.opendir: count=273 total_ns=753919 avg_ns=2761 max_ns=3659
  stat_child_no_follow: count=40202 total_ns=272286277 avg_ns=6772 max_ns=147924
  source_root_path: count=40449 total_ns=51223251 avg_ns=1266 max_ns=64386
  resolved_virtual_path: count=118484 total_ns=258553085 avg_ns=2182 max_ns=257179
  resolved_virtual_path_from_path: count=76644 total_ns=222993317 avg_ns=2909 max_ns=257179
  resolved_virtual_path_from_path_component_walk: count=76644 total_ns=196592118 avg_ns=2565 max_ns=256408
  resolved_virtual_path_from_path_canonicalize: count=171817 total_ns=159842664 avg_ns=930 max_ns=255886
  resolved_virtual_path_from_path_source_root_confinement: count=171817 total_ns=21739543 avg_ns=126 max_ns=29444
  resolved_virtual_path_from_path_virtual_conversion: count=76644 total_ns=22733621 avg_ns=296 max_ns=16591
  resolved_virtual_path_from_open_fd: count=41840 total_ns=35559768 avg_ns=849 max_ns=48321
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
  readdirplus_directory_scan: count=273 total_ns=1869689 avg_ns=6848 max_ns=15305
  readdirplus_attr_generation_scan: count=806 total_ns=4914340 avg_ns=6097 max_ns=34214
  readdirplus_attr_generation_entries: count=533
  readdirplus_symlink_visibility: count=273 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=273 total_ns=77084 avg_ns=282 max_ns=7545
  readdirplus_page_commit: count=273 total_ns=318991 avg_ns=1168 max_ns=3383
  invalidations: count=1066 invalidated_entries=572 evicted_entries=0 scanned_entries=15392
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
