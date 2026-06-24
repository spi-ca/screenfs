# ScreenFS benchmark result

- timestamp: `2026-06-23T19:12:01.929743+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-directory-surface.svg --workload-set directory-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+9 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `f463bb601edc6de0f5cbd3cedc8641bbc5989eb23abfe3f4415de94f430cfe94`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+9 more)`
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
| readdir_basic | 0.001239 | 0.079072 | 63.814 | 0.080405 | 0.082676 | 0.084492 |
| readdirplus_basic | 0.007900 | 0.231142 | 29.260 | 0.244217 | 0.245897 | 0.247240 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=80088 avg_ns=80088 max_ns=80088
  fuse_op.getattr: count=65027 total_ns=207112822 avg_ns=3185 max_ns=78813
  fuse_op.lookup: count=195057 total_ns=751386976 avg_ns=3852 max_ns=268796
  fuse_op.opendir: count=26 total_ns=601778 avg_ns=23145 max_ns=49589
  fuse_op.readdir: count=180 total_ns=1394502660 avg_ns=7747237 max_ns=19815430
  fuse_op.readdirplus: count=31 total_ns=395770573 avg_ns=12766792 max_ns=18259539
  fuse_op.releasedir: count=26 total_ns=14331799 avg_ns=551223 max_ns=790458
  fuse_op.statfs: count=2 total_ns=7714 avg_ns=3857 max_ns=4898
  policy_decision: count=828089 total_ns=408102171 avg_ns=492 max_ns=235290
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=828089 total_ns=145157899 avg_ns=175 max_ns=277441
  matcher_candidate_order.path: count=2484267 total_ns=451530006 avg_ns=181 max_ns=276759
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=12467592
  matcher_candidate_order_ancestor_steps.descendant: count=3116898
  matcher_candidate_order_ancestor_steps.path: count=9350694
  state_read_lock_wait: count=260322 total_ns=5591606 avg_ns=21 max_ns=19824
  state_read_lock_hold: count=260322 total_ns=16004224 avg_ns=61 max_ns=20698
  state_write_lock_wait: count=195344 total_ns=4062770 avg_ns=20 max_ns=3262
  state_write_lock_hold: count=195344 total_ns=297516928 avg_ns=1523 max_ns=3791802
  open_confined_openat2: count=266229 total_ns=191945601 avg_ns=720 max_ns=266048
  open_like.pre_open_guard.access: count=1 total_ns=30443 avg_ns=30443 max_ns=30443
  open_like.pre_open_guard.opendir: count=26 total_ns=114766 avg_ns=4414 max_ns=17228
  open_like.post_open_revalidation.access: count=1 total_ns=43858 avg_ns=43858 max_ns=43858
  open_like.post_open_revalidation.opendir: count=26 total_ns=417470 avg_ns=16056 max_ns=30706
  stat_child_no_follow: count=265991 total_ns=337135879 avg_ns=1267 max_ns=266480
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4280376 avg_ns=16 max_ns=3444
  stat_child_no_follow.host_fstat: count=265989 total_ns=57152139 avg_ns=214 max_ns=63114
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=337135879 avg_ns=1267 max_ns=266480
  source_root_path: count=238 total_ns=3692806 avg_ns=15515 max_ns=40401
  resolved_virtual_path: count=238 total_ns=971154 avg_ns=4080 max_ns=14080
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=238 total_ns=971154 avg_ns=4080 max_ns=14080
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1191614690 avg_ns=6620081 max_ns=16243817
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=87930027 avg_ns=488500 max_ns=1480858
  readdir_page_commit: count=180 total_ns=166466676 avg_ns=924814 max_ns=3792264
  readdirplus_directory_scan: count=31 total_ns=377072845 avg_ns=12163640 max_ns=17655073
  readdirplus_attr_generation_scan: count=5859 total_ns=9859193 avg_ns=1682 max_ns=19198
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=28059282 avg_ns=905138 max_ns=1397099
  readdirplus_page_commit: count=31 total_ns=6738066 avg_ns=217356 max_ns=392589
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
