# ScreenFS benchmark result

- timestamp: `2026-06-24T01:35:05.642906+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface-rerun.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface-rerun.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface-rerun.svg --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-before-rerun-52008`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a132ead172582dc33427c0c8142623e6edaaf4a6e201ade98854f9e035232883`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-before-rerun-52008`
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
| readdir_basic | 0.001504 | 0.109664 | 72.891 | 0.111195 | 0.111782 | 0.112251 |
| readdirplus_basic | 0.008830 | 0.384453 | 43.539 | 0.430162 | 0.430331 | 0.430466 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=45382 avg_ns=45382 max_ns=45382
  fuse_op.getattr: count=65027 total_ns=686987177 avg_ns=10564 max_ns=291841
  fuse_op.lookup: count=195057 total_ns=1867627825 avg_ns=9574 max_ns=288998
  fuse_op.opendir: count=26 total_ns=581243 avg_ns=22355 max_ns=51753
  fuse_op.readdir: count=180 total_ns=1967642362 avg_ns=10931346 max_ns=27014090
  fuse_op.readdirplus: count=31 total_ns=605100876 avg_ns=19519383 max_ns=26503212
  fuse_op.releasedir: count=26 total_ns=14325701 avg_ns=550988 max_ns=964133
  fuse_op.statfs: count=2 total_ns=1789 avg_ns=894 max_ns=1018
  policy_decision: count=833917 total_ns=804372503 avg_ns=964 max_ns=397199
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=833917 total_ns=259499879 avg_ns=311 max_ns=395116
  matcher_candidate_order.path: count=2501751 total_ns=860061156 avg_ns=343 max_ns=448802
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81723866
  matcher_candidate_order_seen_slots.descendant: count=26685344
  matcher_candidate_order_seen_slots.path: count=55038522
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=5881129 avg_ns=22 max_ns=261065
  state_read_lock_hold: count=260322 total_ns=17470605 avg_ns=67 max_ns=13929
  state_write_lock_wait: count=195344 total_ns=4080153 avg_ns=20 max_ns=6734
  state_write_lock_hold: count=195344 total_ns=297598800 avg_ns=1523 max_ns=3909988
  open_confined_openat2: count=266229 total_ns=207603132 avg_ns=779 max_ns=276565
  open_like.pre_open_guard.access: count=1 total_ns=27500 avg_ns=27500 max_ns=27500
  open_like.pre_open_guard.opendir: count=26 total_ns=361712 avg_ns=13912 max_ns=36844
  open_like.post_open_revalidation.access: count=1 total_ns=8867 avg_ns=8867 max_ns=8867
  open_like.post_open_revalidation.opendir: count=26 total_ns=149188 avg_ns=5738 max_ns=15844
  stat_child_no_follow: count=265991 total_ns=352784787 avg_ns=1326 max_ns=283501
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4238099 avg_ns=15 max_ns=8312
  stat_child_no_follow.host_fstat: count=265989 total_ns=57391835 avg_ns=215 max_ns=64485
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=352784787 avg_ns=1326 max_ns=283501
  source_root_path: count=260374 total_ns=424926682 avg_ns=1631 max_ns=265321
  resolved_virtual_path: count=260399 total_ns=859625863 avg_ns=3301 max_ns=278203
  resolved_virtual_path_from_path: count=260161 total_ns=858675003 avg_ns=3300 max_ns=278203
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=737045396 avg_ns=2833 max_ns=277758
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=582303128 avg_ns=994 max_ns=277018
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=84567693 avg_ns=144 max_ns=28226
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=106821242 avg_ns=410 max_ns=264169
  resolved_virtual_path_from_open_fd: count=238 total_ns=950860 avg_ns=3995 max_ns=24209
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1767831378 avg_ns=9821285 max_ns=24354868
  readdir_scan.name_child_path_materialization: count=180 total_ns=121526798 avg_ns=675148 max_ns=1676699
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=1341327387 avg_ns=7451818 max_ns=20030040
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=92140466 avg_ns=511891 max_ns=1467184
  readdir_page_commit: count=180 total_ns=163007054 avg_ns=905594 max_ns=3910695
  readdirplus_directory_scan: count=31 total_ns=560558334 avg_ns=18082526 max_ns=24590520
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=37146578 avg_ns=1198276 max_ns=1649424
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=11840237 avg_ns=2031 max_ns=49250
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=22254795 avg_ns=3818 max_ns=29997
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=451135240 avg_ns=14552749 max_ns=20230328
  readdirplus_attr_generation_scan: count=5859 total_ns=11840237 avg_ns=2020 max_ns=49250
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=29093672 avg_ns=938505 max_ns=1347010
  readdirplus_page_commit: count=31 total_ns=7394257 avg_ns=238524 max_ns=436645
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
