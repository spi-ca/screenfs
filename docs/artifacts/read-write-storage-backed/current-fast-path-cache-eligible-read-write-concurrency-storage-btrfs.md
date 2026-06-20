# ScreenFS benchmark result

- timestamp: `2026-06-20T18:09:42.385752+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-concurrency --concurrency-workers 4 --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.json --output-md docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.md --output-svg docs/artifacts/read-write-storage-backed/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.svg`
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
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- screenfs_only_workloads: `(none)`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| concurrent_rand_read_write_4k | 0.065776 | 0.773291 | 11.756 | 0.826058 | 0.840162 | 0.851445 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=64332 avg_ns=64332 max_ns=64332
  fuse_op.create: count=28 total_ns=4042587 avg_ns=144378 max_ns=214919
  fuse_op.flush: count=28 total_ns=9375073384 avg_ns=334824049 max_ns=469678459
  fuse_op.getattr: count=114689 total_ns=2262448684 avg_ns=19726 max_ns=1849975
  fuse_op.getxattr: count=114716 total_ns=3386744850 avg_ns=29522 max_ns=1603028
  fuse_op.lookup: count=257 total_ns=5660855 avg_ns=22026 max_ns=73482
  fuse_op.open: count=28 total_ns=633686 avg_ns=22631 max_ns=46791
  fuse_op.read: count=111314 total_ns=223113645 avg_ns=2004 max_ns=1631268
  fuse_op.release: count=56 total_ns=149406 avg_ns=2667 max_ns=6826
  fuse_op.setattr: count=28 total_ns=1643205 avg_ns=58685 max_ns=93658
  fuse_op.statfs: count=2 total_ns=4717 avg_ns=2358 max_ns=2401
  fuse_op.unlink: count=28 total_ns=89865792 avg_ns=3209492 max_ns=6230263
  fuse_op.write: count=114688 total_ns=521851306 avg_ns=4550 max_ns=1147389
  policy_decision: count=575274 total_ns=268401575 avg_ns=466 max_ns=1159896
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=575022 total_ns=98077565 avg_ns=170 max_ns=134691
  matcher_candidate_order.path: count=1725570 total_ns=299019492 avg_ns=173 max_ns=261846
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8279048
  matcher_candidate_order_ancestor_steps.descendant: count=2069314
  matcher_candidate_order_ancestor_steps.path: count=6209734
  state_read_lock_wait: count=455805 total_ns=40921095 avg_ns=89 max_ns=308642
  state_read_lock_hold: count=455805 total_ns=60295011 avg_ns=132 max_ns=255693
  state_write_lock_wait: count=367 total_ns=19538 avg_ns=53 max_ns=2036
  state_write_lock_hold: count=367 total_ns=592614 avg_ns=1614 max_ns=22139
  open_confined_openat2: count=344800 total_ns=339326177 avg_ns=984 max_ns=795024
  open_like.pre_open_guard.access: count=1 total_ns=22650 avg_ns=22650 max_ns=22650
  open_like.pre_open_guard.open: count=28 total_ns=448731 avg_ns=16026 max_ns=36010
  open_like.post_open_revalidation.access: count=1 total_ns=36562 avg_ns=36562 max_ns=36562
  open_like.post_open_revalidation.open: count=28 total_ns=100158 avg_ns=3577 max_ns=5051
  stat_child_no_follow: count=229971 total_ns=3663620711 avg_ns=15930 max_ns=1847715
  source_root_path: count=344743 total_ns=1085346534 avg_ns=3148 max_ns=1836777
  resolved_virtual_path: count=574853 total_ns=1748751961 avg_ns=3042 max_ns=1579820
  resolved_virtual_path_from_path: count=230026 total_ns=1204908440 avg_ns=5238 max_ns=1168548
  resolved_virtual_path_from_path_component_walk: count=230026 total_ns=1084943958 avg_ns=4716 max_ns=1168044
  resolved_virtual_path_from_path_canonicalize: count=459737 total_ns=932130960 avg_ns=2027 max_ns=1167195
  resolved_virtual_path_from_path_source_root_confinement: count=459737 total_ns=100274367 avg_ns=218 max_ns=1109790
  resolved_virtual_path_from_path_virtual_conversion: count=230026 total_ns=102642757 avg_ns=446 max_ns=69010
  resolved_virtual_path_from_open_fd: count=344827 total_ns=543843521 avg_ns=1577 max_ns=1579820
  read_handle_snapshot: count=111314 total_ns=45050432 avg_ns=404 max_ns=54704
  read_guard_path: count=111314 total_ns=2338685 avg_ns=21 max_ns=477
  read_io: count=111314 total_ns=140397788 avg_ns=1261 max_ns=1630534
  write_handle_snapshot: count=114688 total_ns=46708703 avg_ns=407 max_ns=309031
  write_guard_mutation: count=114688 total_ns=2317557 avg_ns=20 max_ns=398
  write_io: count=114688 total_ns=438855881 avg_ns=3826 max_ns=1145677
  file_sync.flush: count=28 total_ns=9375015619 avg_ns=334821986 max_ns=469675836
  read_size_bucket.0_4k: count=111167 total_ns=139970611 avg_ns=1259 max_ns=1630534
  read_size_bucket.4k_64k: count=147 total_ns=427177 avg_ns=2905 max_ns=6988
  write_size_bucket.0_4k: count=114688 total_ns=438855881 avg_ns=3826 max_ns=1145677
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
