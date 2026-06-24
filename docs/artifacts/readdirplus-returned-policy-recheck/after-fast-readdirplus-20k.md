# ScreenFS benchmark result

- timestamp: `2026-06-23T19:13:02.145064+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/after-fast-readdirplus-20k.svg --dir-entries 20000 --workload readdirplus_basic`
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
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
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
| readdirplus_basic | 0.023003 | 1.511071 | 65.689 | 1.532510 | 1.533546 | 1.534375 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=94523 avg_ns=94523 max_ns=94523
  fuse_op.getattr: count=260014 total_ns=902738673 avg_ns=3471 max_ns=100329
  fuse_op.lookup: count=780031 total_ns=3406424807 avg_ns=4367 max_ns=283185
  fuse_op.opendir: count=13 total_ns=400103 avg_ns=30777 max_ns=44266
  fuse_op.readdir: count=332 total_ns=9571557496 avg_ns=28829992 max_ns=79405855
  fuse_op.readdirplus: count=33 total_ns=1343090717 avg_ns=40699718 max_ns=68927091
  fuse_op.releasedir: count=13 total_ns=31181200 avg_ns=2398553 max_ns=3102006
  fuse_op.statfs: count=2 total_ns=7555 avg_ns=3777 max_ns=4920
  policy_decision: count=4685022 total_ns=2358138096 avg_ns=503 max_ns=284540
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=4685022 total_ns=834844303 avg_ns=178 max_ns=68758
  matcher_candidate_order.path: count=14055066 total_ns=2584821425 avg_ns=183 max_ns=266370
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=71838372
  matcher_candidate_order_ancestor_steps.descendant: count=17959593
  matcher_candidate_order_ancestor_steps.path: count=53878779
  state_read_lock_wait: count=1040424 total_ns=24285158 avg_ns=23 max_ns=61259
  state_read_lock_hold: count=1040424 total_ns=74850029 avg_ns=71 max_ns=62771
  state_write_lock_wait: count=780433 total_ns=17393659 avg_ns=22 max_ns=15787
  state_write_lock_hold: count=780433 total_ns=1021534015 avg_ns=1308 max_ns=6693866
  open_confined_openat2: count=1046710 total_ns=819907578 avg_ns=783 max_ns=269878
  open_like.pre_open_guard.access: count=1 total_ns=42894 avg_ns=42894 max_ns=42894
  open_like.pre_open_guard.opendir: count=13 total_ns=67599 avg_ns=5199 max_ns=18409
  open_like.post_open_revalidation.access: count=1 total_ns=44229 avg_ns=44229 max_ns=44229
  open_like.post_open_revalidation.opendir: count=13 total_ns=291796 avg_ns=22445 max_ns=30844
  stat_child_no_follow: count=1046331 total_ns=1442122564 avg_ns=1378 max_ns=270739
  stat_child_no_follow.attr_conversion: count=1046329 total_ns=18065903 avg_ns=17 max_ns=62433
  stat_child_no_follow.host_fstat: count=1046329 total_ns=256113815 avg_ns=244 max_ns=71254
  stat_child_no_follow_context.path_guard_or_metadata: count=1046331 total_ns=1442122564 avg_ns=1378 max_ns=270739
  source_root_path: count=379 total_ns=7753153 avg_ns=20456 max_ns=68611
  resolved_virtual_path: count=379 total_ns=1447785 avg_ns=3820 max_ns=10808
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=379 total_ns=1447785 avg_ns=3820 max_ns=10808
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=9072099747 avg_ns=27325601 max_ns=74599284
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=727894428 avg_ns=2192453 max_ns=5092552
  readdir_page_commit: count=332 total_ns=412378377 avg_ns=1242103 max_ns=6694292
  readdirplus_directory_scan: count=33 total_ns=1304432411 avg_ns=39528254 max_ns=68095769
  readdirplus_attr_generation_scan: count=6279 total_ns=11151711 avg_ns=1776 max_ns=49220
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=85651288 avg_ns=2595493 max_ns=4258592
  readdirplus_page_commit: count=33 total_ns=25070355 avg_ns=759707 max_ns=2641987
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
