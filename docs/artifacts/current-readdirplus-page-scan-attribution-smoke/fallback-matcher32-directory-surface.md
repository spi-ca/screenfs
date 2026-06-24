# ScreenFS benchmark result

- timestamp: `2026-06-24T00:25:32.477554+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `2251a0c12c055bc7d667d81ed3bc6b49e0c7a1e8e8c4cff045b6f4cc19f1da4c`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+11 more)`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.000904 | 0.178264 | 197.128 | 0.181995 | 0.182462 | 0.182835 |
| readdirplus_basic | 0.011512 | 0.330016 | 28.668 | 0.409743 | 0.419709 | 0.427682 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=100486 avg_ns=100486 max_ns=100486
  fuse_op.getattr: count=20009 total_ns=185990698 avg_ns=9295 max_ns=91265
  fuse_op.lookup: count=60021 total_ns=512059098 avg_ns=8531 max_ns=154249
  fuse_op.opendir: count=8 total_ns=306465 avg_ns=38308 max_ns=77769
  fuse_op.readdir: count=54 total_ns=822036262 avg_ns=15222893 max_ns=37993207
  fuse_op.readdirplus: count=13 total_ns=281627175 avg_ns=21663628 max_ns=39652346
  fuse_op.releasedir: count=8 total_ns=5462279 avg_ns=682784 max_ns=913473
  fuse_op.statfs: count=2 total_ns=7287 avg_ns=3643 max_ns=3730
  policy_decision: count=262129 total_ns=317971845 avg_ns=1213 max_ns=75065
  matcher_candidates: count=640608
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=640608
  matcher_candidate_order.descendant: count=262129 total_ns=98350356 avg_ns=375 max_ns=24051
  matcher_candidate_order.path: count=786387 total_ns=336658880 avg_ns=428 max_ns=74792
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=25688642
  matcher_candidate_order_seen_slots.descendant: count=8388128
  matcher_candidate_order_seen_slots.path: count=17300514
  matcher_candidate_order_ancestor_steps: count=3953456
  matcher_candidate_order_ancestor_steps.descendant: count=988364
  matcher_candidate_order_ancestor_steps.path: count=2965092
  state_read_lock_wait: count=80106 total_ns=1531597 avg_ns=19 max_ns=700
  state_read_lock_hold: count=80106 total_ns=4836355 avg_ns=60 max_ns=4986
  state_write_lock_wait: count=60110 total_ns=1110235 avg_ns=18 max_ns=2767
  state_write_lock_hold: count=60110 total_ns=121061115 avg_ns=2013 max_ns=5951792
  open_confined_openat2: count=82467 total_ns=55002905 avg_ns=666 max_ns=36430
  open_like.pre_open_guard.access: count=1 total_ns=81065 avg_ns=81065 max_ns=81065
  open_like.pre_open_guard.opendir: count=8 total_ns=198582 avg_ns=24822 max_ns=53810
  open_like.post_open_revalidation.access: count=1 total_ns=11422 avg_ns=11422 max_ns=11422
  open_like.post_open_revalidation.opendir: count=8 total_ns=76286 avg_ns=9535 max_ns=17298
  stat_child_no_follow: count=82391 total_ns=94742671 avg_ns=1149 max_ns=47279
  stat_child_no_follow.attr_conversion: count=82389 total_ns=1198730 avg_ns=14 max_ns=920
  stat_child_no_follow.host_fstat: count=82389 total_ns=14661203 avg_ns=177 max_ns=17268
  stat_child_no_follow_context.path_guard_or_metadata: count=82391 total_ns=94742671 avg_ns=1149 max_ns=47279
  source_root_path: count=80122 total_ns=111649354 avg_ns=1393 max_ns=50749
  resolved_virtual_path: count=80129 total_ns=237094581 avg_ns=2958 max_ns=27391
  resolved_virtual_path_from_path: count=80053 total_ns=236751737 avg_ns=2957 max_ns=27391
  resolved_virtual_path_from_path_component_walk: count=80053 total_ns=205295913 avg_ns=2564 max_ns=26786
  resolved_virtual_path_from_path_canonicalize: count=180089 total_ns=162946874 avg_ns=904 max_ns=25477
  resolved_virtual_path_from_path_source_root_confinement: count=180089 total_ns=25044978 avg_ns=139 max_ns=22869
  resolved_virtual_path_from_path_virtual_conversion: count=80053 total_ns=27246728 avg_ns=340 max_ns=10615
  resolved_virtual_path_from_open_fd: count=76 total_ns=342844 avg_ns=4511 max_ns=15112
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=726370789 avg_ns=13451310 max_ns=36013125
  readdir_scan.name_child_path_materialization: count=54 total_ns=52272705 avg_ns=968013 max_ns=2665112
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=582954033 avg_ns=10795445 max_ns=29895975
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=32812806 avg_ns=607644 max_ns=1669667
  readdir_page_commit: count=54 total_ns=81639372 avg_ns=1511840 max_ns=5952177
  readdirplus_directory_scan: count=13 total_ns=259412653 avg_ns=19954819 max_ns=37580947
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=18125269 avg_ns=1394251 max_ns=2738338
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=5102549 avg_ns=2184 max_ns=14184
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=11857523 avg_ns=5075 max_ns=12312
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=214514591 avg_ns=16501122 max_ns=31252883
  readdirplus_attr_generation_scan: count=2349 total_ns=5102549 avg_ns=2172 max_ns=14184
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=12306100 avg_ns=946623 max_ns=1855171
  readdirplus_page_commit: count=13 total_ns=3765695 avg_ns=289668 max_ns=481224
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
