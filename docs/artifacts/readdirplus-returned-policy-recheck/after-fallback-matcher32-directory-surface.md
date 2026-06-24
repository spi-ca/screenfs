# ScreenFS benchmark result

- timestamp: `2026-06-23T19:12:17.964330+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
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
| readdir_basic | 0.001232 | 0.105074 | 85.308 | 0.107899 | 0.109241 | 0.110314 |
| readdirplus_basic | 0.007868 | 0.417222 | 53.026 | 0.422946 | 0.424500 | 0.425744 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=86803 avg_ns=86803 max_ns=86803
  fuse_op.getattr: count=65027 total_ns=728525230 avg_ns=11203 max_ns=122373
  fuse_op.lookup: count=195057 total_ns=1982965019 avg_ns=10166 max_ns=170979
  fuse_op.opendir: count=26 total_ns=531629 avg_ns=20447 max_ns=43713
  fuse_op.readdir: count=180 total_ns=1929301709 avg_ns=10718342 max_ns=22804929
  fuse_op.readdirplus: count=31 total_ns=579156075 avg_ns=18682454 max_ns=24774574
  fuse_op.releasedir: count=26 total_ns=14291270 avg_ns=549664 max_ns=1065028
  fuse_op.statfs: count=2 total_ns=5763 avg_ns=2881 max_ns=3535
  policy_decision: count=828089 total_ns=826958279 avg_ns=998 max_ns=84201
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=828089 total_ns=268676909 avg_ns=324 max_ns=69082
  matcher_candidate_order.path: count=2484267 total_ns=885564308 avg_ns=356 max_ns=156376
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81152722
  matcher_candidate_order_seen_slots.descendant: count=26498848
  matcher_candidate_order_seen_slots.path: count=54653874
  matcher_candidate_order_ancestor_steps: count=12467592
  matcher_candidate_order_ancestor_steps.descendant: count=3116898
  matcher_candidate_order_ancestor_steps.path: count=9350694
  state_read_lock_wait: count=260322 total_ns=5922343 avg_ns=22 max_ns=25546
  state_read_lock_hold: count=260322 total_ns=19547283 avg_ns=75 max_ns=70761
  state_write_lock_wait: count=195344 total_ns=4451127 avg_ns=22 max_ns=834
  state_write_lock_hold: count=195344 total_ns=306813987 avg_ns=1570 max_ns=3308873
  open_confined_openat2: count=266229 total_ns=220505314 avg_ns=828 max_ns=68969
  open_like.pre_open_guard.access: count=1 total_ns=74334 avg_ns=74334 max_ns=74334
  open_like.pre_open_guard.opendir: count=26 total_ns=332708 avg_ns=12796 max_ns=29941
  open_like.post_open_revalidation.access: count=1 total_ns=6308 avg_ns=6308 max_ns=6308
  open_like.post_open_revalidation.opendir: count=26 total_ns=131826 avg_ns=5070 max_ns=8355
  stat_child_no_follow: count=265991 total_ns=383079498 avg_ns=1440 max_ns=71666
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4525955 avg_ns=17 max_ns=27173
  stat_child_no_follow.host_fstat: count=265989 total_ns=65628040 avg_ns=246 max_ns=69581
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=383079498 avg_ns=1440 max_ns=71666
  source_root_path: count=260374 total_ns=462875593 avg_ns=1777 max_ns=104953
  resolved_virtual_path: count=260399 total_ns=907098792 avg_ns=3483 max_ns=149959
  resolved_virtual_path_from_path: count=260161 total_ns=906136359 avg_ns=3482 max_ns=149959
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=777613157 avg_ns=2988 max_ns=147695
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=612813602 avg_ns=1047 max_ns=146076
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=85899247 avg_ns=146 max_ns=64034
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=112373213 avg_ns=431 max_ns=69890
  resolved_virtual_path_from_open_fd: count=238 total_ns=962433 avg_ns=4043 max_ns=27132
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1727568774 avg_ns=9597604 max_ns=20742103
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=88842214 avg_ns=493567 max_ns=1209700
  readdir_page_commit: count=180 total_ns=165992690 avg_ns=922181 max_ns=3309219
  readdirplus_directory_scan: count=31 total_ns=559324465 avg_ns=18042724 max_ns=23808404
  readdirplus_attr_generation_scan: count=5859 total_ns=10390108 avg_ns=1773 max_ns=71767
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=28667191 avg_ns=924748 max_ns=1291008
  readdirplus_page_commit: count=31 total_ns=7059258 avg_ns=227718 max_ns=656261
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
