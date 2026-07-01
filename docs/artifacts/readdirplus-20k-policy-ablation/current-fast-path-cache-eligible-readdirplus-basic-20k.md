# ScreenFS benchmark result

- timestamp: `2026-07-01T05:16:50.808061+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --workload readdirplus_basic --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 20000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-readdirplus-basic-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+112 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+112 more)`
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
| readdirplus_basic | 0.026795 | 0.962440 | 35.918 | 0.978107 | 0.982740 | 0.986446 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47633 avg_ns=47633 max_ns=47633
  fuse_op.getattr: count=260014 total_ns=900832150 avg_ns=3464 max_ns=66567
  fuse_op.lookup: count=780031 total_ns=3434376459 avg_ns=4402 max_ns=278760
  fuse_op.opendir: count=13 total_ns=328431 avg_ns=25263 max_ns=63965
  fuse_op.readdir: count=332 total_ns=2702508436 avg_ns=8140085 max_ns=12271532
  fuse_op.readdirplus: count=33 total_ns=284754314 avg_ns=8628918 max_ns=10639848
  fuse_op.releasedir: count=13 total_ns=31597081 avg_ns=2430544 max_ns=2963155
  fuse_op.statfs: count=2 total_ns=6795 avg_ns=3397 max_ns=4110
  policy_decision: count=1040072 total_ns=416852947 avg_ns=400 max_ns=43231
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1040072 total_ns=141775573 avg_ns=136 max_ns=28008
  matcher_candidate_order.path: count=3120216 total_ns=457542920 avg_ns=146 max_ns=63358
  matcher_candidate_order_by_source.hidden.path: count=1040072 total_ns=150838844 avg_ns=145 max_ns=22490
  matcher_candidate_order_by_source.internal_hidden.path: count=1040072 total_ns=156693862 avg_ns=150 max_ns=28623
  matcher_candidate_order_by_source.visible.descendant: count=1040072 total_ns=141775573 avg_ns=136 max_ns=28008
  matcher_candidate_order_by_source.visible.path: count=1040072 total_ns=150010214 avg_ns=144 max_ns=63358
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=13520748
  matcher_candidate_order_ancestor_steps.descendant: count=3380187
  matcher_candidate_order_ancestor_steps.path: count=10140561
  state_read_lock_wait: count=1040424 total_ns=24539231 avg_ns=23 max_ns=10198
  state_read_lock_hold: count=1040424 total_ns=74165097 avg_ns=71 max_ns=43080
  state_write_lock_wait: count=780433 total_ns=17478423 avg_ns=22 max_ns=19324
  state_write_lock_hold: count=780433 total_ns=921780885 avg_ns=1181 max_ns=3021588
  open_confined_openat2: count=1046696 total_ns=815821775 avg_ns=779 max_ns=77327
  open_like.pre_open_guard.access: count=1 total_ns=314 avg_ns=314 max_ns=314
  open_like.pre_open_guard.opendir: count=13 total_ns=5006 avg_ns=385 max_ns=470
  open_like.post_open_revalidation.access: count=1 total_ns=36458 avg_ns=36458 max_ns=36458
  open_like.post_open_revalidation.opendir: count=13 total_ns=261495 avg_ns=20115 max_ns=35737
  stat_child_no_follow: count=1046317 total_ns=1436835083 avg_ns=1373 max_ns=116406
  stat_child_no_follow.attr_conversion: count=1046315 total_ns=17788422 avg_ns=17 max_ns=11068
  stat_child_no_follow.host_fstat: count=1046315 total_ns=246235760 avg_ns=235 max_ns=114926
  stat_child_no_follow_context.path_guard_or_metadata: count=1046317 total_ns=1436835083 avg_ns=1373 max_ns=116406
  source_root_path: count=379 total_ns=9646067 avg_ns=25451 max_ns=75121
  resolved_virtual_path: count=379 total_ns=2049141 avg_ns=5406 max_ns=43310
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=379 total_ns=2049141 avg_ns=5406 max_ns=43310
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=332 total_ns=2250309940 avg_ns=6778041 max_ns=10299692
  readdir_scan.name_child_path_materialization: count=332 total_ns=74276161 avg_ns=223723 max_ns=465054
  readdir_scan.returned_child_path_materialization: count=254093 total_ns=60571612 avg_ns=238 max_ns=21291
  readdir_scan.scan_fallback_attr: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=332 total_ns=56446216 avg_ns=170018 max_ns=365906
  readdir_attr_generation_scan: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=332 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=332 total_ns=447795692 avg_ns=1348782 max_ns=3053804
  readdir_page_commit: count=332 total_ns=335560886 avg_ns=1010725 max_ns=3021941
  readdirplus_directory_scan: count=33 total_ns=254682772 avg_ns=7717659 max_ns=9724239
  readdirplus_scan.name_child_path_materialization: count=33 total_ns=9468350 avg_ns=286919 max_ns=440806
  readdirplus_scan.returned_attr_hydration: count=6246 total_ns=9300914 avg_ns=1489 max_ns=26998
  readdirplus_scan.returned_child_path_materialization: count=6246 total_ns=1926309 avg_ns=308 max_ns=4573
  readdirplus_scan.returned_policy_recheck: count=6246 total_ns=109239 avg_ns=17 max_ns=163
  readdirplus_scan.scan_fallback_attr: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=33 total_ns=8114283 avg_ns=245887 max_ns=370543
  readdirplus_attr_generation_scan: count=6279 total_ns=9300914 avg_ns=1481 max_ns=26998
  readdirplus_attr_generation_entries: count=6246
  readdirplus_symlink_visibility: count=33 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=33 total_ns=57318596 avg_ns=1736927 max_ns=2890140
  readdirplus_page_commit: count=33 total_ns=15802547 avg_ns=478865 max_ns=1310801
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
