# ScreenFS benchmark result

- timestamp: `2026-06-20T18:09:35.377719+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.json --output-md docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.md --output-svg docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-surface-storage-btrfs.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+24 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+24 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- screenfs_only_workloads: `(none)`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.028937 | 0.146183 | 5.052 | 0.148841 | 0.149046 | 0.149209 |
| seq_write | 0.079242 | 0.180255 | 2.275 | 0.186350 | 0.186420 | 0.186476 |
| small_read | 0.001059 | 0.022932 | 21.656 | 0.024229 | 0.024421 | 0.024573 |
| small_write | 0.001670 | 0.039295 | 23.525 | 0.041403 | 0.041820 | 0.042154 |
| rand_read_4k | 0.024403 | 0.469885 | 19.255 | 0.476915 | 0.477422 | 0.477827 |
| rand_write_4k | 0.044432 | 0.925242 | 20.824 | 0.939233 | 0.940208 | 0.940988 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=103586 avg_ns=103586 max_ns=103586
  fuse_op.create: count=21 total_ns=3069340 avg_ns=146159 max_ns=247928
  fuse_op.flush: count=21 total_ns=2133057124 avg_ns=101574148 max_ns=309698045
  fuse_op.getattr: count=123684 total_ns=1678366495 avg_ns=13569 max_ns=328664
  fuse_op.getxattr: count=123655 total_ns=2680904656 avg_ns=21680 max_ns=297877
  fuse_op.lookup: count=236 total_ns=5064566 avg_ns=21460 max_ns=65212
  fuse_op.open: count=21 total_ns=485871 avg_ns=23136 max_ns=40995
  fuse_op.read: count=116872 total_ns=582459156 avg_ns=4983 max_ns=327575
  fuse_op.release: count=42 total_ns=192576 avg_ns=4585 max_ns=10930
  fuse_op.setattr: count=7 total_ns=400195 avg_ns=57170 max_ns=62329
  fuse_op.statfs: count=2 total_ns=7436 avg_ns=3718 max_ns=4208
  fuse_op.unlink: count=21 total_ns=159828373 avg_ns=7610874 max_ns=12947643
  fuse_op.write: count=123648 total_ns=1023638285 avg_ns=8278 max_ns=3552721
  policy_decision: count=619605 total_ns=261767769 avg_ns=422 max_ns=55837
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=619444 total_ns=96325274 avg_ns=155 max_ns=35788
  matcher_candidate_order.path: count=1858654 total_ns=285761013 avg_ns=153 max_ns=318828
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8918344
  matcher_candidate_order_ancestor_steps.descendant: count=2229306
  matcher_candidate_order_ancestor_steps.path: count=6689038
  state_read_lock_wait: count=488187 total_ns=11633911 avg_ns=23 max_ns=11502
  state_read_lock_hold: count=488187 total_ns=39761029 avg_ns=81 max_ns=9356
  state_write_lock_wait: count=318 total_ns=19380 avg_ns=60 max_ns=1011
  state_write_lock_hold: count=318 total_ns=455849 avg_ns=1433 max_ns=12223
  open_confined_openat2: count=371505 total_ns=286788964 avg_ns=771 max_ns=47702
  open_like.pre_open_guard.access: count=1 total_ns=30051 avg_ns=30051 max_ns=30051
  open_like.pre_open_guard.open: count=21 total_ns=314866 avg_ns=14993 max_ns=33389
  open_like.post_open_revalidation.access: count=1 total_ns=49954 avg_ns=49954 max_ns=49954
  open_like.post_open_revalidation.open: count=21 total_ns=71001 avg_ns=3381 max_ns=3884
  stat_child_no_follow: count=247779 total_ns=3042429459 avg_ns=12278 max_ns=270461
  source_root_path: count=371462 total_ns=984856982 avg_ns=2651 max_ns=278448
  resolved_virtual_path: count=619345 total_ns=1612992613 avg_ns=2604 max_ns=258602
  resolved_virtual_path_from_path: count=247820 total_ns=1151480522 avg_ns=4646 max_ns=230050
  resolved_virtual_path_from_path_component_walk: count=247820 total_ns=1037557404 avg_ns=4186 max_ns=228347
  resolved_virtual_path_from_path_canonicalize: count=495360 total_ns=870751660 avg_ns=1757 max_ns=66607
  resolved_virtual_path_from_path_source_root_confinement: count=495360 total_ns=106982527 avg_ns=215 max_ns=18254
  resolved_virtual_path_from_path_virtual_conversion: count=247820 total_ns=98311324 avg_ns=396 max_ns=36381
  resolved_virtual_path_from_open_fd: count=371525 total_ns=461512091 avg_ns=1242 max_ns=258602
  read_handle_snapshot: count=116872 total_ns=25465623 avg_ns=217 max_ns=15467
  read_guard_path: count=116872 total_ns=2085278 avg_ns=17 max_ns=1338
  read_io: count=116872 total_ns=533818492 avg_ns=4567 max_ns=326295
  write_handle_snapshot: count=123648 total_ns=27478167 avg_ns=222 max_ns=49958
  write_guard_mutation: count=123648 total_ns=2368671 avg_ns=19 max_ns=11821
  write_io: count=123648 total_ns=970913485 avg_ns=7852 max_ns=3549396
  file_sync.flush: count=21 total_ns=2133002476 avg_ns=101571546 max_ns=309694704
  read_size_bucket.0_4k: count=100800 total_ns=163555533 avg_ns=1622 max_ns=326295
  read_size_bucket.4k_64k: count=1505 total_ns=5979341 avg_ns=3972 max_ns=19523
  read_size_bucket.64k_1m: count=14567 total_ns=364283618 avg_ns=25007 max_ns=148996
  write_size_bucket.0_4k: count=121856 total_ns=486271020 avg_ns=3990 max_ns=373480
  write_size_bucket.64k_1m: count=1792 total_ns=484642465 avg_ns=270447 max_ns=3549396
  readdir_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_directory_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_scan: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_attr_generation_entries: count=0
  readdirplus_symlink_visibility: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_page_commit: count=0 total_ns=0 avg_ns=0 max_ns=0
  invalidations: count=42 invalidated_entries=21 evicted_entries=0 scanned_entries=126
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
