# ScreenFS benchmark result

- timestamp: `2026-06-21T02:12:26.421566+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set matcher-descendant-directory --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.json --output-md /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.md --output-svg /tmp/screenfs-matcher-rank0-artifacts.pPT19N/after-worktree-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.svg`
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
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
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
| matcher_descendant_readdir | 0.000020 | 0.001264 | 62.096 | 0.001478 | 0.001680 | 0.001841 |
| matcher_descendant_readdirplus | 0.000049 | 0.006436 | 130.243 | 0.008825 | 0.008939 | 0.009031 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=39384 avg_ns=39384 max_ns=39384
  fuse_op.getattr: count=443 total_ns=14007039 avg_ns=31618 max_ns=88065
  fuse_op.lookup: count=1747 total_ns=38397201 avg_ns=21978 max_ns=106075
  fuse_op.opendir: count=26 total_ns=881148 avg_ns=33890 max_ns=65159
  fuse_op.readdir: count=26 total_ns=666058 avg_ns=25617 max_ns=63479
  fuse_op.readdirplus: count=26 total_ns=25476925 avg_ns=979881 max_ns=1294698
  fuse_op.releasedir: count=26 total_ns=100229 avg_ns=3854 max_ns=9555
  fuse_op.statfs: count=2 total_ns=5887 avg_ns=2943 max_ns=4454
  policy_decision: count=7112 total_ns=13978417 avg_ns=1965 max_ns=14681
  matcher_candidates: count=154742
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=154742
  matcher_candidate_order.descendant: count=7112 total_ns=4102049 avg_ns=576 max_ns=84540
  matcher_candidate_order.path: count=21336 total_ns=10333268 avg_ns=484 max_ns=4349
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=696976
  matcher_candidate_order_seen_slots.descendant: count=227584
  matcher_candidate_order_seen_slots.path: count=469392
  matcher_candidate_order_ancestor_steps: count=107136
  matcher_candidate_order_ancestor_steps.descendant: count=26784
  matcher_candidate_order_ancestor_steps.path: count=80352
  state_read_lock_wait: count=2269 total_ns=60461 avg_ns=26 max_ns=394
  state_read_lock_hold: count=2269 total_ns=219809 avg_ns=96 max_ns=2694
  state_write_lock_wait: count=1875 total_ns=50407 avg_ns=26 max_ns=288
  state_write_lock_hold: count=1875 total_ns=1310036 avg_ns=698 max_ns=37809
  open_confined_openat2: count=3180 total_ns=3151726 avg_ns=991 max_ns=81273
  open_like.pre_open_guard.access: count=1 total_ns=28544 avg_ns=28544 max_ns=28544
  open_like.pre_open_guard.opendir: count=26 total_ns=651488 avg_ns=25057 max_ns=52434
  open_like.post_open_revalidation.access: count=1 total_ns=7167 avg_ns=7167 max_ns=7167
  open_like.post_open_revalidation.opendir: count=26 total_ns=166693 avg_ns=6411 max_ns=9157
  stat_child_no_follow: count=3101 total_ns=41721144 avg_ns=13454 max_ns=95518
  source_root_path: count=3153 total_ns=7196644 avg_ns=2282 max_ns=28656
  resolved_virtual_path: count=8546 total_ns=26860141 avg_ns=3143 max_ns=74874
  resolved_virtual_path_from_path: count=5367 total_ns=22302695 avg_ns=4155 max_ns=19634
  resolved_virtual_path_from_path_component_walk: count=5367 total_ns=19688640 avg_ns=3668 max_ns=16557
  resolved_virtual_path_from_path_canonicalize: count=12780 total_ns=16088179 avg_ns=1258 max_ns=15785
  resolved_virtual_path_from_path_source_root_confinement: count=12780 total_ns=1814307 avg_ns=141 max_ns=3522
  resolved_virtual_path_from_path_virtual_conversion: count=5367 total_ns=2252728 avg_ns=419 max_ns=4601
  resolved_virtual_path_from_open_fd: count=3179 total_ns=4557446 avg_ns=1433 max_ns=74874
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=258623 avg_ns=9947 max_ns=40805
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=10021 avg_ns=385 max_ns=1136
  readdirplus_directory_scan: count=26 total_ns=6061664 avg_ns=233140 max_ns=315720
  readdirplus_attr_generation_scan: count=858 total_ns=12164405 avg_ns=14177 max_ns=29524
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=175971 avg_ns=6768 max_ns=22917
  readdirplus_page_commit: count=26 total_ns=536005 avg_ns=20615 max_ns=37937
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
