# ScreenFS benchmark result

- timestamp: `2026-06-24T01:32:31.560390+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fast-directory-surface.svg --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a132ead172582dc33427c0c8142623e6edaaf4a6e201ade98854f9e035232883`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
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
| readdir_basic | 0.001851 | 0.132699 | 71.688 | 0.137839 | 0.139422 | 0.140689 |
| readdirplus_basic | 0.005207 | 0.239477 | 45.993 | 0.242781 | 0.243100 | 0.243355 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=87170 avg_ns=87170 max_ns=87170
  fuse_op.getattr: count=65027 total_ns=208942230 avg_ns=3213 max_ns=151254
  fuse_op.lookup: count=195057 total_ns=767477697 avg_ns=3934 max_ns=369113
  fuse_op.opendir: count=26 total_ns=1005320 avg_ns=38666 max_ns=88729
  fuse_op.readdir: count=180 total_ns=1910947077 avg_ns=10616372 max_ns=28255012
  fuse_op.readdirplus: count=31 total_ns=532217879 avg_ns=17168318 max_ns=29063322
  fuse_op.releasedir: count=26 total_ns=21142070 avg_ns=813156 max_ns=1277461
  fuse_op.statfs: count=2 total_ns=8317 avg_ns=4158 max_ns=4959
  policy_decision: count=833917 total_ns=520998580 avg_ns=624 max_ns=102149
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=833917 total_ns=183508376 avg_ns=220 max_ns=672927
  matcher_candidate_order.path: count=2501751 total_ns=570628646 avg_ns=228 max_ns=661014
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=6793663 avg_ns=26 max_ns=1278
  state_read_lock_hold: count=260322 total_ns=16419320 avg_ns=63 max_ns=13898
  state_write_lock_wait: count=195344 total_ns=4231019 avg_ns=21 max_ns=5329
  state_write_lock_hold: count=195344 total_ns=357540406 avg_ns=1830 max_ns=5643277
  open_confined_openat2: count=266229 total_ns=194530940 avg_ns=730 max_ns=176516
  open_like.pre_open_guard.access: count=1 total_ns=34984 avg_ns=34984 max_ns=34984
  open_like.pre_open_guard.opendir: count=26 total_ns=183498 avg_ns=7057 max_ns=22270
  open_like.post_open_revalidation.access: count=1 total_ns=44828 avg_ns=44828 max_ns=44828
  open_like.post_open_revalidation.opendir: count=26 total_ns=723903 avg_ns=27842 max_ns=72217
  stat_child_no_follow: count=265991 total_ns=343248079 avg_ns=1290 max_ns=396865
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4425097 avg_ns=16 max_ns=2416
  stat_child_no_follow.host_fstat: count=265989 total_ns=60183330 avg_ns=226 max_ns=34642
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=343248079 avg_ns=1290 max_ns=396865
  source_root_path: count=238 total_ns=5194659 avg_ns=21826 max_ns=83688
  resolved_virtual_path: count=238 total_ns=1327814 avg_ns=5579 max_ns=17793
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=238 total_ns=1327814 avg_ns=5579 max_ns=17793
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1653523094 avg_ns=9186239 max_ns=26313819
  readdir_scan.name_child_path_materialization: count=180 total_ns=165261177 avg_ns=918117 max_ns=2742679
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=1202638380 avg_ns=6681324 max_ns=20200419
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=102896273 avg_ns=571645 max_ns=3022482
  readdir_page_commit: count=180 total_ns=213829897 avg_ns=1187943 max_ns=5643754
  readdirplus_directory_scan: count=31 total_ns=492435898 avg_ns=15885028 max_ns=27216292
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=47986659 avg_ns=1547956 max_ns=2807480
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=12002050 avg_ns=2059 max_ns=397045
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=16953933 avg_ns=2909 max_ns=665091
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=377733942 avg_ns=12184965 max_ns=21143694
  readdirplus_attr_generation_scan: count=5859 total_ns=12002050 avg_ns=2048 max_ns=397045
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=32060535 avg_ns=1034210 max_ns=1799560
  readdirplus_page_commit: count=31 total_ns=7747767 avg_ns=249927 max_ns=373238
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
