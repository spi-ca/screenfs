# ScreenFS benchmark result

- timestamp: `2026-06-21T01:56:19.519253+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-glob-matcher-heavy-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-glob-matcher-heavy-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/after-worktree-70a31053d688-glob-matcher-heavy-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/tests/state_cache.rs; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `0a05d5096ce79b996b3e7ff85f7031cff909bae46f001f1c3ab093dad3458000`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/tests/state_cache.rs; ... (+3 more)`
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
| readdir_basic | 0.001210 | 0.095237 | 78.739 | 0.096390 | 0.096493 | 0.096575 |
| readdirplus_basic | 0.006667 | 0.458350 | 68.751 | 0.559811 | 0.570796 | 0.579585 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=40620 avg_ns=40620 max_ns=40620
  fuse_op.getattr: count=65027 total_ns=1090348182 avg_ns=16767 max_ns=277646
  fuse_op.lookup: count=195057 total_ns=2686916766 avg_ns=13775 max_ns=291316
  fuse_op.opendir: count=26 total_ns=582369 avg_ns=22398 max_ns=40005
  fuse_op.readdir: count=180 total_ns=1757995182 avg_ns=9766639 max_ns=20442905
  fuse_op.readdirplus: count=31 total_ns=567440139 avg_ns=18304520 max_ns=21457535
  fuse_op.releasedir: count=26 total_ns=9809003 avg_ns=377269 max_ns=740652
  fuse_op.statfs: count=2 total_ns=2522 avg_ns=1261 max_ns=1412
  policy_decision: count=1099907 total_ns=847488496 avg_ns=770 max_ns=368170
  matcher_candidates: count=6246880
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=6246880
  matcher_candidate_order.descendant: count=1099907 total_ns=284618344 avg_ns=258 max_ns=1554264
  matcher_candidate_order.path: count=3299721 total_ns=902917295 avg_ns=273 max_ns=288120
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=107790886
  matcher_candidate_order_seen_slots.descendant: count=35197024
  matcher_candidate_order_seen_slots.path: count=72593862
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5562435 avg_ns=21 max_ns=1154
  state_read_lock_hold: count=260322 total_ns=19196459 avg_ns=73 max_ns=15731
  state_write_lock_wait: count=195344 total_ns=4013624 avg_ns=20 max_ns=1015
  state_write_lock_hold: count=195344 total_ns=260931380 avg_ns=1335 max_ns=2929358
  open_confined_openat2: count=266229 total_ns=154536951 avg_ns=580 max_ns=62913
  open_like.pre_open_guard.access: count=1 total_ns=33962 avg_ns=33962 max_ns=33962
  open_like.pre_open_guard.opendir: count=26 total_ns=421573 avg_ns=16214 max_ns=31965
  open_like.post_open_revalidation.access: count=1 total_ns=3213 avg_ns=3213 max_ns=3213
  open_like.post_open_revalidation.opendir: count=26 total_ns=101874 avg_ns=3918 max_ns=4760
  stat_child_no_follow: count=265991 total_ns=2125609728 avg_ns=7991 max_ns=283209
  source_root_path: count=266202 total_ns=439104504 avg_ns=1649 max_ns=265685
  resolved_virtual_path: count=792379 total_ns=1563626094 avg_ns=1973 max_ns=275632
  resolved_virtual_path_from_path: count=526151 total_ns=1295690757 avg_ns=2462 max_ns=275632
  resolved_virtual_path_from_path_component_walk: count=526151 total_ns=1106026094 avg_ns=2102 max_ns=270633
  resolved_virtual_path_from_path_canonicalize: count=922034 total_ns=890042389 avg_ns=965 max_ns=270149
  resolved_virtual_path_from_path_source_root_confinement: count=922034 total_ns=113569859 avg_ns=123 max_ns=124115
  resolved_virtual_path_from_path_virtual_conversion: count=526151 total_ns=160644861 avg_ns=305 max_ns=273595
  resolved_virtual_path_from_open_fd: count=266228 total_ns=267935337 avg_ns=1006 max_ns=265128
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1572202801 avg_ns=8734460 max_ns=18651883
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=123276441 avg_ns=684869 max_ns=2191472
  readdir_page_commit: count=180 total_ns=131122515 avg_ns=728458 max_ns=2930043
  readdirplus_directory_scan: count=31 total_ns=486712500 avg_ns=15700403 max_ns=18693618
  readdirplus_attr_generation_scan: count=5859 total_ns=55852000 avg_ns=9532 max_ns=73860
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=39087345 avg_ns=1260882 max_ns=1873574
  readdirplus_page_commit: count=31 total_ns=5466142 avg_ns=176327 max_ns=294866
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
