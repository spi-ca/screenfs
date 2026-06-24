# ScreenFS benchmark result

- timestamp: `2026-06-24T01:33:49.991376+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/after-batch-fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-after-48607`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-after-48607`
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
| readdir_basic | 0.001276 | 0.100994 | 79.162 | 0.101856 | 0.102399 | 0.102834 |
| readdirplus_basic | 0.007308 | 0.420602 | 57.557 | 0.427026 | 0.428722 | 0.430079 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=86591 avg_ns=86591 max_ns=86591
  fuse_op.getattr: count=65027 total_ns=744280890 avg_ns=11445 max_ns=296785
  fuse_op.lookup: count=195057 total_ns=2036213521 avg_ns=10439 max_ns=295681
  fuse_op.opendir: count=26 total_ns=544744 avg_ns=20951 max_ns=59161
  fuse_op.readdir: count=180 total_ns=1924867829 avg_ns=10693710 max_ns=21371510
  fuse_op.readdirplus: count=31 total_ns=576608320 avg_ns=18600268 max_ns=21768902
  fuse_op.releasedir: count=26 total_ns=15696176 avg_ns=603699 max_ns=1061518
  fuse_op.statfs: count=2 total_ns=3926 avg_ns=1963 max_ns=2320
  policy_decision: count=833917 total_ns=803566956 avg_ns=963 max_ns=86106
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=833917 total_ns=259729869 avg_ns=311 max_ns=52275
  matcher_candidate_order.path: count=2501751 total_ns=866757466 avg_ns=346 max_ns=346084
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81723866
  matcher_candidate_order_seen_slots.descendant: count=26685344
  matcher_candidate_order_seen_slots.path: count=55038522
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=6188161 avg_ns=23 max_ns=868
  state_read_lock_hold: count=260322 total_ns=18665528 avg_ns=71 max_ns=117622
  state_write_lock_wait: count=195344 total_ns=4441078 avg_ns=22 max_ns=12781
  state_write_lock_hold: count=195344 total_ns=300234272 avg_ns=1536 max_ns=3095658
  open_confined_openat2: count=266229 total_ns=233433441 avg_ns=876 max_ns=121092
  open_like.pre_open_guard.access: count=1 total_ns=73197 avg_ns=73197 max_ns=73197
  open_like.pre_open_guard.opendir: count=26 total_ns=325713 avg_ns=12527 max_ns=26941
  open_like.post_open_revalidation.access: count=1 total_ns=6394 avg_ns=6394 max_ns=6394
  open_like.post_open_revalidation.opendir: count=26 total_ns=131583 avg_ns=5060 max_ns=7346
  stat_child_no_follow: count=265991 total_ns=392758265 avg_ns=1476 max_ns=122274
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4542941 avg_ns=17 max_ns=21938
  stat_child_no_follow.host_fstat: count=265989 total_ns=66063067 avg_ns=248 max_ns=105343
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=392758265 avg_ns=1476 max_ns=122274
  source_root_path: count=260374 total_ns=472709998 avg_ns=1815 max_ns=172266
  resolved_virtual_path: count=260399 total_ns=927239053 avg_ns=3560 max_ns=245457
  resolved_virtual_path_from_path: count=260161 total_ns=926374260 avg_ns=3560 max_ns=245457
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=793399829 avg_ns=3049 max_ns=244871
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=626681513 avg_ns=1070 max_ns=241799
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=86438374 avg_ns=147 max_ns=110460
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=117277712 avg_ns=450 max_ns=49202
  resolved_virtual_path_from_open_fd: count=238 total_ns=864793 avg_ns=3633 max_ns=19323
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1734145236 avg_ns=9634140 max_ns=20079564
  readdir_scan.name_child_path_materialization: count=180 total_ns=117570891 avg_ns=653171 max_ns=1414941
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=1305771675 avg_ns=7254287 max_ns=15824181
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=85113118 avg_ns=472850 max_ns=1275000
  readdir_page_commit: count=180 total_ns=154364693 avg_ns=857581 max_ns=3096113
  readdirplus_directory_scan: count=31 total_ns=537762556 avg_ns=17347179 max_ns=20356663
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=35776300 avg_ns=1154074 max_ns=1443844
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=11137313 avg_ns=1911 max_ns=18710
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=18566222 avg_ns=3185 max_ns=12002
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=431886614 avg_ns=13931826 max_ns=16153243
  readdirplus_attr_generation_scan: count=5859 total_ns=11137313 avg_ns=1900 max_ns=18710
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=26489525 avg_ns=854500 max_ns=1225056
  readdirplus_page_commit: count=31 total_ns=6477436 avg_ns=208949 max_ns=331605
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
