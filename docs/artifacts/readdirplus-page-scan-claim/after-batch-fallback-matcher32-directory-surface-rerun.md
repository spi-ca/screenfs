# ScreenFS benchmark result

- timestamp: `2026-06-24T01:35:36.918643+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface-rerun.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface-rerun.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface-rerun.svg --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-after-rerun-52008`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-after-rerun-52008`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
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
| readdir_basic | 0.001322 | 0.097443 | 73.701 | 0.099682 | 0.101455 | 0.102874 |
| readdirplus_basic | 0.007919 | 0.341424 | 43.116 | 0.350790 | 0.350821 | 0.350846 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=101311 avg_ns=101311 max_ns=101311
  fuse_op.getattr: count=65027 total_ns=618361760 avg_ns=9509 max_ns=277408
  fuse_op.lookup: count=195057 total_ns=1680706657 avg_ns=8616 max_ns=336908
  fuse_op.opendir: count=26 total_ns=482306 avg_ns=18550 max_ns=42179
  fuse_op.readdir: count=180 total_ns=1853477113 avg_ns=10297095 max_ns=25448045
  fuse_op.readdirplus: count=31 total_ns=593493131 avg_ns=19144939 max_ns=28949830
  fuse_op.releasedir: count=26 total_ns=12627827 avg_ns=485685 max_ns=1162897
  fuse_op.statfs: count=2 total_ns=2446 avg_ns=1223 max_ns=1558
  policy_decision: count=833917 total_ns=757520351 avg_ns=908 max_ns=274518
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=833917 total_ns=239819823 avg_ns=287 max_ns=63601
  matcher_candidate_order.path: count=2501751 total_ns=808132385 avg_ns=323 max_ns=268748
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81723866
  matcher_candidate_order_seen_slots.descendant: count=26685344
  matcher_candidate_order_seen_slots.path: count=55038522
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=5123966 avg_ns=19 max_ns=1515
  state_read_lock_hold: count=260322 total_ns=16868246 avg_ns=64 max_ns=61687
  state_write_lock_wait: count=195344 total_ns=3703397 avg_ns=18 max_ns=83287
  state_write_lock_hold: count=195344 total_ns=289232228 avg_ns=1480 max_ns=2939045
  open_confined_openat2: count=266229 total_ns=177490507 avg_ns=666 max_ns=284311
  open_like.pre_open_guard.access: count=1 total_ns=81094 avg_ns=81094 max_ns=81094
  open_like.pre_open_guard.opendir: count=26 total_ns=287227 avg_ns=11047 max_ns=28888
  open_like.post_open_revalidation.access: count=1 total_ns=11676 avg_ns=11676 max_ns=11676
  open_like.post_open_revalidation.opendir: count=26 total_ns=120022 avg_ns=4616 max_ns=7764
  stat_child_no_follow: count=265991 total_ns=305303422 avg_ns=1147 max_ns=321338
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3865728 avg_ns=14 max_ns=6966
  stat_child_no_follow.host_fstat: count=265989 total_ns=46828567 avg_ns=176 max_ns=319859
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=305303422 avg_ns=1147 max_ns=321338
  source_root_path: count=260374 total_ns=363842415 avg_ns=1397 max_ns=263318
  resolved_virtual_path: count=260399 total_ns=779605923 avg_ns=2993 max_ns=271736
  resolved_virtual_path_from_path: count=260161 total_ns=778766544 avg_ns=2993 max_ns=271736
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=674839525 avg_ns=2593 max_ns=271186
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=533612681 avg_ns=911 max_ns=84014
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=83905937 avg_ns=143 max_ns=268133
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=90332754 avg_ns=347 max_ns=46186
  resolved_virtual_path_from_open_fd: count=238 total_ns=839379 avg_ns=3526 max_ns=25961
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1650264442 avg_ns=9168135 max_ns=23426681
  readdir_scan.name_child_path_materialization: count=180 total_ns=111400804 avg_ns=618893 max_ns=1630375
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=1260308033 avg_ns=7001711 max_ns=19415275
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=74982210 avg_ns=416567 max_ns=1271033
  readdir_page_commit: count=180 total_ns=168945274 avg_ns=938584 max_ns=2939730
  readdirplus_directory_scan: count=31 total_ns=551109861 avg_ns=17777737 max_ns=26933250
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=35606868 avg_ns=1148608 max_ns=1862174
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=10866543 avg_ns=1864 max_ns=18486
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=21682883 avg_ns=3720 max_ns=14534
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=447648998 avg_ns=14440290 max_ns=22679323
  readdirplus_attr_generation_scan: count=5859 total_ns=10866543 avg_ns=1854 max_ns=18486
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=24229178 avg_ns=781586 max_ns=1204706
  readdirplus_page_commit: count=31 total_ns=7008818 avg_ns=226090 max_ns=373581
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
