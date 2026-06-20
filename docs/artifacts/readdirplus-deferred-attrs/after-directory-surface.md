# ScreenFS benchmark result

- timestamp: `2026-06-20T08:41:19.009744+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-surface --iterations 10 --warmups 3 --dir-entries 5000 --output-json docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.json --output-md docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.md --output-svg docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+36 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `4d1cca3219e371f9cc936238aa0170b1e22015125eec98a5eaf042487649d1bd`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M scripts/bench-screenfs.py;  M scripts/test_bench_screenfs.py; ... (+36 more)`
- screenfs_source_git: `41cb2d212a461f1a061dfc8924614b4bef73f9d2`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001145 | 0.054051 | 47.193 | 0.055254 | 0.055994 | 0.056587 |
| readdirplus_basic | 0.003777 | 0.334404 | 88.541 | 0.349467 | 0.350182 | 0.350755 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=49841 avg_ns=49841 max_ns=49841
  fuse_op.getattr: count=65027 total_ns=787005506 avg_ns=12102 max_ns=272155
  fuse_op.lookup: count=195057 total_ns=1920217591 avg_ns=9844 max_ns=352045
  fuse_op.opendir: count=26 total_ns=461061 avg_ns=17733 max_ns=29428
  fuse_op.readdir: count=180 total_ns=1060322042 avg_ns=5890678 max_ns=11904740
  fuse_op.readdirplus: count=31 total_ns=340853944 avg_ns=10995288 max_ns=14838955
  fuse_op.releasedir: count=26 total_ns=8908987 avg_ns=342653 max_ns=447721
  fuse_op.statfs: count=2 total_ns=2412 avg_ns=1206 max_ns=1536
  policy_decision: count=1099907 total_ns=421906857 avg_ns=383 max_ns=285363
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=122299140 avg_ns=111 max_ns=96224
  matcher_candidate_order.path: count=3299721 total_ns=459542180 avg_ns=139 max_ns=262265
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1099907
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1099907
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=4409948 avg_ns=16 max_ns=2638
  state_read_lock_hold: count=260322 total_ns=13918713 avg_ns=53 max_ns=9392
  state_write_lock_wait: count=195344 total_ns=3168391 avg_ns=16 max_ns=2991
  state_write_lock_hold: count=195344 total_ns=206765926 avg_ns=1058 max_ns=2088899
  open_confined_openat2: count=266229 total_ns=96775485 avg_ns=363 max_ns=254969
  stat_child_no_follow: count=265991 total_ns=1481681963 avg_ns=5570 max_ns=284402
  source_root_path: count=266228 total_ns=298723192 avg_ns=1122 max_ns=263936
  resolved_virtual_path: count=792379 total_ns=1255289539 avg_ns=1584 max_ns=278919
  resolved_virtual_path_from_path: count=526151 total_ns=1054997119 avg_ns=2005 max_ns=278919
  resolved_virtual_path_from_path_component_walk: count=526151 total_ns=902391200 avg_ns=1715 max_ns=278322
  resolved_virtual_path_from_path_canonicalize: count=922034 total_ns=713366098 avg_ns=773 max_ns=277691
  resolved_virtual_path_from_path_source_root_confinement: count=922034 total_ns=114362646 avg_ns=124 max_ns=86781
  resolved_virtual_path_from_path_virtual_conversion: count=526151 total_ns=128081176 avg_ns=243 max_ns=71183
  resolved_virtual_path_from_open_fd: count=266228 total_ns=200292420 avg_ns=752 max_ns=75298
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=933832601 avg_ns=5187958 max_ns=10523648
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=48223085 avg_ns=267906 max_ns=677323
  readdir_page_commit: count=180 total_ns=108423441 avg_ns=602352 max_ns=2089018
  readdirplus_directory_scan: count=31 total_ns=288113522 avg_ns=9293984 max_ns=13154869
  readdirplus_attr_generation_scan: count=5859 total_ns=37427780 avg_ns=6388 max_ns=71984
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16291212 avg_ns=525522 max_ns=708145
  readdirplus_page_commit: count=31 total_ns=4061663 avg_ns=131021 max_ns=252408
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
