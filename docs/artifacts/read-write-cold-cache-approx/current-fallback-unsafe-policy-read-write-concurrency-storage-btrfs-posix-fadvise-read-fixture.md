# ScreenFS benchmark result

- timestamp: `2026-06-20T19:02:15.168010+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-concurrency --concurrency-workers 4 --cache-control posix-fadvise-read-fixture --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json --output-md docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md --output-svg docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+26 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+26 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `posix-fadvise-read-fixture`
- cache_control_scope: `requested non-root read-side cold-cache approximation for benchmark-owned source/.screenfs-bench read fixtures`
- cache_control_timing_applied: `True`
- cache_control_applications: `14`
- cache_control_notes: `apply os.posix_fadvise(..., POSIX_FADV_DONTNEED) to selected backing source fixture files before each warmup and measured sample`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| concurrent_rand_read_write_4k | 0.235903 | 1.525063 | 6.465 | 1.772791 | 1.829703 | 1.875234 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=54005 avg_ns=54005 max_ns=54005
  fuse_op.create: count=28 total_ns=6083689 avg_ns=217274 max_ns=396464
  fuse_op.flush: count=28 total_ns=8859151770 avg_ns=316398277 max_ns=509258572
  fuse_op.getattr: count=114689 total_ns=3405012049 avg_ns=29689 max_ns=2161390
  fuse_op.getxattr: count=114716 total_ns=5009703796 avg_ns=43670 max_ns=2043837
  fuse_op.lookup: count=257 total_ns=9034693 avg_ns=35154 max_ns=195377
  fuse_op.open: count=28 total_ns=983160 avg_ns=35112 max_ns=84023
  fuse_op.read: count=111314 total_ns=10416902723 avg_ns=93581 max_ns=3374018
  fuse_op.release: count=56 total_ns=192191 avg_ns=3431 max_ns=22930
  fuse_op.setattr: count=28 total_ns=2257192 avg_ns=80614 max_ns=115455
  fuse_op.statfs: count=2 total_ns=9771 avg_ns=4885 max_ns=6238
  fuse_op.unlink: count=28 total_ns=118050939 avg_ns=4216104 max_ns=9498915
  fuse_op.write: count=114688 total_ns=5808430153 avg_ns=50645 max_ns=2115832
  policy_decision: count=1712088 total_ns=1002995960 avg_ns=585 max_ns=746215
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1482460 total_ns=265675492 avg_ns=179 max_ns=284204
  matcher_candidate_order.path: count=4906636 total_ns=1113340698 avg_ns=226 max_ns=1738329
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1712088
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1712088
  matcher_candidate_order_ancestor_steps: count=23728832
  matcher_candidate_order_ancestor_steps.descendant: count=5473008
  matcher_candidate_order_ancestor_steps.path: count=18255824
  state_read_lock_wait: count=455805 total_ns=49037215 avg_ns=107 max_ns=62577
  state_read_lock_hold: count=455805 total_ns=75370271 avg_ns=165 max_ns=118218
  state_write_lock_wait: count=367 total_ns=24714 avg_ns=67 max_ns=948
  state_write_lock_hold: count=367 total_ns=762649 avg_ns=2078 max_ns=25368
  open_confined_openat2: count=570858 total_ns=648064340 avg_ns=1135 max_ns=2023905
  open_like.pre_open_guard.access: count=1 total_ns=44708 avg_ns=44708 max_ns=44708
  open_like.pre_open_guard.open: count=28 total_ns=764005 avg_ns=27285 max_ns=66933
  open_like.post_open_revalidation.access: count=1 total_ns=5183 avg_ns=5183 max_ns=5183
  open_like.post_open_revalidation.open: count=28 total_ns=126918 avg_ns=4532 max_ns=10590
  stat_child_no_follow: count=456029 total_ns=7716413707 avg_ns=16920 max_ns=3082000
  source_root_path: count=685517 total_ns=2266628723 avg_ns=3306 max_ns=1634915
  resolved_virtual_path: count=1708830 total_ns=7666313991 avg_ns=4486 max_ns=3060674
  resolved_virtual_path_from_path: count=911943 total_ns=6330376320 avg_ns=6941 max_ns=3060674
  resolved_virtual_path_from_path_component_walk: count=911943 total_ns=5812451913 avg_ns=6373 max_ns=3059591
  resolved_virtual_path_from_path_canonicalize: count=2279060 total_ns=5080239843 avg_ns=2229 max_ns=3058464
  resolved_virtual_path_from_path_source_root_confinement: count=2279060 total_ns=468815950 avg_ns=205 max_ns=791059
  resolved_virtual_path_from_path_virtual_conversion: count=911943 total_ns=452885014 avg_ns=496 max_ns=265484
  resolved_virtual_path_from_open_fd: count=796887 total_ns=1335937671 avg_ns=1676 max_ns=1840838
  read_handle_snapshot: count=111314 total_ns=52820013 avg_ns=474 max_ns=66089
  read_guard_path: count=111314 total_ns=3824366721 avg_ns=34356 max_ns=3186397
  read_io: count=111314 total_ns=6486983855 avg_ns=58276 max_ns=3334258
  write_handle_snapshot: count=114688 total_ns=56720167 avg_ns=494 max_ns=118499
  write_guard_mutation: count=114688 total_ns=5180441769 avg_ns=45169 max_ns=2104093
  write_io: count=114688 total_ns=522123724 avg_ns=4552 max_ns=380749
  file_sync.flush: count=28 total_ns=8859092281 avg_ns=316396152 max_ns=509256740
  read_size_bucket.0_4k: count=111167 total_ns=6473099577 avg_ns=58228 max_ns=3334258
  read_size_bucket.4k_64k: count=147 total_ns=13884278 avg_ns=94450 max_ns=1082358
  write_size_bucket.0_4k: count=114688 total_ns=522123724 avg_ns=4552 max_ns=380749
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
  invalidations: count=56 invalidated_entries=28 evicted_entries=0 scanned_entries=350
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
