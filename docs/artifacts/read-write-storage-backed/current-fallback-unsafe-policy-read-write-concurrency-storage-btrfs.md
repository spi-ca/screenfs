# ScreenFS benchmark result

- timestamp: `2026-06-20T18:10:16.991342+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-concurrency --concurrency-workers 4 --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.json --output-md docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.md --output-svg docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
- screenfs_only_workloads: `(none)`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| concurrent_rand_read_write_4k | 0.069259 | 1.087217 | 15.698 | 1.186490 | 1.204061 | 1.218119 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=55977 avg_ns=55977 max_ns=55977
  fuse_op.create: count=28 total_ns=5020677 avg_ns=179309 max_ns=239309
  fuse_op.flush: count=28 total_ns=6442840651 avg_ns=230101451 max_ns=423588466
  fuse_op.getattr: count=114689 total_ns=3405344080 avg_ns=29691 max_ns=4076363
  fuse_op.getxattr: count=114716 total_ns=4940822660 avg_ns=43070 max_ns=2735614
  fuse_op.lookup: count=257 total_ns=7542454 avg_ns=29348 max_ns=76430
  fuse_op.open: count=28 total_ns=850228 avg_ns=30365 max_ns=52451
  fuse_op.read: count=111314 total_ns=4097101871 avg_ns=36806 max_ns=2063199
  fuse_op.release: count=56 total_ns=183946 avg_ns=3284 max_ns=23331
  fuse_op.setattr: count=28 total_ns=2275564 avg_ns=81270 max_ns=205759
  fuse_op.statfs: count=2 total_ns=7792 avg_ns=3896 max_ns=4232
  fuse_op.unlink: count=28 total_ns=98839335 avg_ns=3529976 max_ns=6672391
  fuse_op.write: count=114688 total_ns=5901727410 avg_ns=51458 max_ns=2726759
  policy_decision: count=1712088 total_ns=961890561 avg_ns=561 max_ns=1814322
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1482460 total_ns=253050344 avg_ns=170 max_ns=320236
  matcher_candidate_order.path: count=4906636 total_ns=1057018851 avg_ns=215 max_ns=1112600
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1712088
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1712088
  matcher_candidate_order_ancestor_steps: count=23728832
  matcher_candidate_order_ancestor_steps.descendant: count=5473008
  matcher_candidate_order_ancestor_steps.path: count=18255824
  state_read_lock_wait: count=455805 total_ns=45980828 avg_ns=100 max_ns=183411
  state_read_lock_hold: count=455805 total_ns=63176427 avg_ns=138 max_ns=237612
  state_write_lock_wait: count=367 total_ns=29386 avg_ns=80 max_ns=6888
  state_write_lock_hold: count=367 total_ns=652963 avg_ns=1779 max_ns=30090
  open_confined_openat2: count=570858 total_ns=592922015 avg_ns=1038 max_ns=1305406
  open_like.pre_open_guard.access: count=1 total_ns=43553 avg_ns=43553 max_ns=43553
  open_like.pre_open_guard.open: count=28 total_ns=660241 avg_ns=23580 max_ns=41359
  open_like.post_open_revalidation.access: count=1 total_ns=4403 avg_ns=4403 max_ns=4403
  open_like.post_open_revalidation.open: count=28 total_ns=109702 avg_ns=3917 max_ns=6036
  stat_child_no_follow: count=456029 total_ns=7579825979 avg_ns=16621 max_ns=3085766
  source_root_path: count=685517 total_ns=2168005312 avg_ns=3162 max_ns=2022974
  resolved_virtual_path: count=1708830 total_ns=7491543830 avg_ns=4384 max_ns=4047930
  resolved_virtual_path_from_path: count=911943 total_ns=6210114771 avg_ns=6809 max_ns=4047930
  resolved_virtual_path_from_path_component_walk: count=911943 total_ns=5725592191 avg_ns=6278 max_ns=4044489
  resolved_virtual_path_from_path_canonicalize: count=2279060 total_ns=5034436708 avg_ns=2208 max_ns=4043141
  resolved_virtual_path_from_path_source_root_confinement: count=2279060 total_ns=449037178 avg_ns=197 max_ns=736712
  resolved_virtual_path_from_path_virtual_conversion: count=911943 total_ns=421660638 avg_ns=462 max_ns=320525
  resolved_virtual_path_from_open_fd: count=796887 total_ns=1281429059 avg_ns=1608 max_ns=2028244
  read_handle_snapshot: count=111314 total_ns=48423434 avg_ns=435 max_ns=249221
  read_guard_path: count=111314 total_ns=3870175027 avg_ns=34768 max_ns=2060272
  read_io: count=111314 total_ns=139197353 avg_ns=1250 max_ns=523457
  write_handle_snapshot: count=114688 total_ns=50846410 avg_ns=443 max_ns=213530
  write_guard_mutation: count=114688 total_ns=5347986026 avg_ns=46630 max_ns=2706106
  write_io: count=114688 total_ns=457870655 avg_ns=3992 max_ns=1250630
  file_sync.flush: count=28 total_ns=6442790390 avg_ns=230099656 max_ns=423586358
  read_size_bucket.0_4k: count=111167 total_ns=138778971 avg_ns=1248 max_ns=523457
  read_size_bucket.4k_64k: count=147 total_ns=418382 avg_ns=2846 max_ns=8243
  write_size_bucket.0_4k: count=114688 total_ns=457870655 avg_ns=3992 max_ns=1250630
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
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
