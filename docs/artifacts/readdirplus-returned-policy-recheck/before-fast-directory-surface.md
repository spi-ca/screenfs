# ScreenFS benchmark result

- timestamp: `2026-06-23T19:11:35.211908+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fast-directory-surface.svg --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-before-12398`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M src/fs/backing.rs`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/tmp/screenfs-readdirplus-before-12398`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M src/fs/backing.rs`
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
| readdir_basic | 0.001117 | 0.070548 | 63.141 | 0.073568 | 0.074365 | 0.075003 |
| readdirplus_basic | 0.007579 | 0.242990 | 32.060 | 0.245750 | 0.247373 | 0.248671 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=71157 avg_ns=71157 max_ns=71157
  fuse_op.getattr: count=65027 total_ns=218370271 avg_ns=3358 max_ns=65926
  fuse_op.lookup: count=195057 total_ns=796245124 avg_ns=4082 max_ns=321099
  fuse_op.opendir: count=26 total_ns=657822 avg_ns=25300 max_ns=55067
  fuse_op.readdir: count=180 total_ns=1347383804 avg_ns=7485465 max_ns=15504168
  fuse_op.readdirplus: count=31 total_ns=387305693 avg_ns=12493732 max_ns=16037496
  fuse_op.releasedir: count=26 total_ns=14803266 avg_ns=569356 max_ns=1012134
  fuse_op.statfs: count=2 total_ns=3755 avg_ns=1877 max_ns=2713
  policy_decision: count=833917 total_ns=390040910 avg_ns=467 max_ns=275984
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=833917 total_ns=138445953 avg_ns=166 max_ns=68975
  matcher_candidate_order.path: count=2501751 total_ns=433233164 avg_ns=173 max_ns=82292
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=5808445 avg_ns=22 max_ns=28512
  state_read_lock_hold: count=260322 total_ns=17378600 avg_ns=66 max_ns=37146
  state_write_lock_wait: count=195344 total_ns=4343965 avg_ns=22 max_ns=63317
  state_write_lock_hold: count=195344 total_ns=290381359 avg_ns=1486 max_ns=2998651
  open_confined_openat2: count=266229 total_ns=201306845 avg_ns=756 max_ns=69963
  open_like.pre_open_guard.access: count=1 total_ns=25480 avg_ns=25480 max_ns=25480
  open_like.pre_open_guard.opendir: count=26 total_ns=109807 avg_ns=4223 max_ns=11408
  open_like.post_open_revalidation.access: count=1 total_ns=40607 avg_ns=40607 max_ns=40607
  open_like.post_open_revalidation.opendir: count=26 total_ns=479064 avg_ns=18425 max_ns=40114
  stat_child_no_follow: count=265991 total_ns=355155945 avg_ns=1335 max_ns=66287
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4443320 avg_ns=16 max_ns=25279
  stat_child_no_follow.host_fstat: count=265989 total_ns=62923983 avg_ns=236 max_ns=64920
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=355155945 avg_ns=1335 max_ns=66287
  source_root_path: count=238 total_ns=4345751 avg_ns=18259 max_ns=54653
  resolved_virtual_path: count=238 total_ns=998674 avg_ns=4196 max_ns=17152
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=238 total_ns=998674 avg_ns=4196 max_ns=17152
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1162039645 avg_ns=6455775 max_ns=14429271
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=79101659 avg_ns=439453 max_ns=1212353
  readdir_page_commit: count=180 total_ns=152821711 avg_ns=849009 max_ns=2999385
  readdirplus_directory_scan: count=31 total_ns=355468733 avg_ns=11466733 max_ns=14710985
  readdirplus_attr_generation_scan: count=5859 total_ns=10645025 avg_ns=1816 max_ns=66353
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=24436930 avg_ns=788288 max_ns=1031437
  readdirplus_page_commit: count=31 total_ns=6301356 avg_ns=203269 max_ns=339349
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
