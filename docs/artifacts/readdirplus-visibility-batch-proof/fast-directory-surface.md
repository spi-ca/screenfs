# ScreenFS benchmark result

- timestamp: `2026-06-24T01:13:18.586088+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 3 --warmups 1 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fast-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fast-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-visibility-batch-proof/fast-directory-surface.svg --workload-set directory-surface`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/tests/perf.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs;  M src/fs/tests/perf.rs; ... (+2 more)`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001335 | 0.025628 | 19.193 | 0.028715 | 0.029101 | 0.029409 |
| readdirplus_basic | 0.007924 | 0.198407 | 25.040 | 0.199677 | 0.199835 | 0.199962 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=78392 avg_ns=78392 max_ns=78392
  fuse_op.getattr: count=20009 total_ns=67400541 avg_ns=3368 max_ns=17996
  fuse_op.lookup: count=60021 total_ns=246587848 avg_ns=4108 max_ns=34456
  fuse_op.opendir: count=8 total_ns=229621 avg_ns=28702 max_ns=64191
  fuse_op.readdir: count=54 total_ns=174201814 avg_ns=3225959 max_ns=6636314
  fuse_op.readdirplus: count=13 total_ns=43565152 avg_ns=3351165 max_ns=4773343
  fuse_op.releasedir: count=8 total_ns=4814523 avg_ns=601815 max_ns=954756
  fuse_op.statfs: count=2 total_ns=6034 avg_ns=3017 max_ns=4101
  policy_decision: count=82467 total_ns=31069786 avg_ns=376 max_ns=9938
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=82467 total_ns=12145803 avg_ns=147 max_ns=20007
  matcher_candidate_order.path: count=247401 total_ns=36962225 avg_ns=149 max_ns=14911
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=1078864
  matcher_candidate_order_ancestor_steps.descendant: count=269716
  matcher_candidate_order_ancestor_steps.path: count=809148
  state_read_lock_wait: count=80106 total_ns=1937118 avg_ns=24 max_ns=28560
  state_read_lock_hold: count=80106 total_ns=5255763 avg_ns=65 max_ns=8561
  state_write_lock_wait: count=60110 total_ns=1412039 avg_ns=23 max_ns=4645
  state_write_lock_hold: count=60110 total_ns=97897456 avg_ns=1628 max_ns=3552478
  open_confined_openat2: count=82467 total_ns=65262158 avg_ns=791 max_ns=16764
  open_like.pre_open_guard.access: count=1 total_ns=27134 avg_ns=27134 max_ns=27134
  open_like.pre_open_guard.opendir: count=8 total_ns=37550 avg_ns=4693 max_ns=9765
  open_like.post_open_revalidation.access: count=1 total_ns=44707 avg_ns=44707 max_ns=44707
  open_like.post_open_revalidation.opendir: count=8 total_ns=170673 avg_ns=21334 max_ns=50319
  stat_child_no_follow: count=82391 total_ns=114227065 avg_ns=1386 max_ns=24061
  stat_child_no_follow.attr_conversion: count=82389 total_ns=1419253 avg_ns=17 max_ns=136
  stat_child_no_follow.host_fstat: count=82389 total_ns=20165655 avg_ns=244 max_ns=9324
  stat_child_no_follow_context.path_guard_or_metadata: count=82391 total_ns=114227065 avg_ns=1386 max_ns=24061
  source_root_path: count=76 total_ns=1282442 avg_ns=16874 max_ns=43547
  resolved_virtual_path: count=76 total_ns=273136 avg_ns=3593 max_ns=7808
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=76 total_ns=273136 avg_ns=3593 max_ns=7808
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=110751251 avg_ns=2050949 max_ns=3886122
  readdir_scan.name_child_path_materialization: count=54 total_ns=34393724 avg_ns=636920 max_ns=1437963
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=2556101 avg_ns=47335 max_ns=113982
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=20854942 avg_ns=386202 max_ns=1175366
  readdir_page_commit: count=54 total_ns=54038327 avg_ns=1000709 max_ns=3552878
  readdirplus_directory_scan: count=13 total_ns=32469063 avg_ns=2497620 max_ns=3704570
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=11738155 avg_ns=902935 max_ns=1516332
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=3489566 avg_ns=1493 max_ns=8804
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=4241504 avg_ns=1815 max_ns=6046
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=956758 avg_ns=73596 max_ns=122688
  readdirplus_attr_generation_scan: count=2349 total_ns=3489566 avg_ns=1485 max_ns=8804
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=6944116 avg_ns=534162 max_ns=930596
  readdirplus_page_commit: count=13 total_ns=2457822 avg_ns=189063 max_ns=384707
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
