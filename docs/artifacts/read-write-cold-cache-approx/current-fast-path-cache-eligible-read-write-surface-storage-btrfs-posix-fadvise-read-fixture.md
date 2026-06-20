# ScreenFS benchmark result

- timestamp: `2026-06-20T19:01:14.865624+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --cache-control posix-fadvise-read-fixture --read-mib 256 --write-mib 256 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json --output-md docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md --output-svg docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+25 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+25 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `posix-fadvise-read-fixture`
- cache_control_scope: `requested non-root read-side cold-cache approximation for benchmark-owned source/.screenfs-bench read fixtures`
- cache_control_timing_applied: `True`
- cache_control_applications: `42`
- cache_control_notes: `apply os.posix_fadvise(..., POSIX_FADV_DONTNEED) to selected backing source fixture files before each warmup and measured sample ; cache control only applies to read-side fixture workloads`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.108743 | 0.197791 | 1.819 | 0.206082 | 0.208737 | 0.210861 |
| seq_write | 0.091295 | 0.200046 | 2.191 | 0.202176 | 0.202381 | 0.202545 |
| small_read | 0.002648 | 0.026300 | 9.930 | 0.027499 | 0.027546 | 0.027583 |
| small_write | 0.000963 | 0.032249 | 33.490 | 0.034320 | 0.034677 | 0.034962 |
| rand_read_4k | 0.807492 | 1.282738 | 1.589 | 1.305530 | 1.310936 | 1.315260 |
| rand_write_4k | 0.045848 | 0.835199 | 18.217 | 0.915000 | 0.915248 | 0.915446 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=59071 avg_ns=59071 max_ns=59071
  fuse_op.create: count=21 total_ns=3006136 avg_ns=143149 max_ns=293646
  fuse_op.flush: count=21 total_ns=2079586355 avg_ns=99027921 max_ns=302351047
  fuse_op.getattr: count=123684 total_ns=1711474923 avg_ns=13837 max_ns=298606
  fuse_op.getxattr: count=123655 total_ns=2492354295 avg_ns=20155 max_ns=977442
  fuse_op.lookup: count=236 total_ns=4916197 avg_ns=20831 max_ns=69592
  fuse_op.open: count=21 total_ns=453436 avg_ns=21592 max_ns=32788
  fuse_op.read: count=116872 total_ns=6211579607 avg_ns=53148 max_ns=7391447
  fuse_op.release: count=42 total_ns=174265 avg_ns=4149 max_ns=13031
  fuse_op.setattr: count=7 total_ns=399296 avg_ns=57042 max_ns=68449
  fuse_op.statfs: count=2 total_ns=6349 avg_ns=3174 max_ns=3831
  fuse_op.unlink: count=21 total_ns=166053099 avg_ns=7907290 max_ns=12785978
  fuse_op.write: count=123648 total_ns=1030665844 avg_ns=8335 max_ns=1204728
  policy_decision: count=619605 total_ns=252524168 avg_ns=407 max_ns=257519
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=619444 total_ns=92730260 avg_ns=149 max_ns=79579
  matcher_candidate_order.path: count=1858654 total_ns=275957825 avg_ns=148 max_ns=307112
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8918344
  matcher_candidate_order_ancestor_steps.descendant: count=2229306
  matcher_candidate_order_ancestor_steps.path: count=6689038
  state_read_lock_wait: count=488187 total_ns=11062353 avg_ns=22 max_ns=10940
  state_read_lock_hold: count=488187 total_ns=42396291 avg_ns=86 max_ns=53779
  state_write_lock_wait: count=318 total_ns=10863 avg_ns=34 max_ns=293
  state_write_lock_hold: count=318 total_ns=438361 avg_ns=1378 max_ns=17285
  open_confined_openat2: count=371505 total_ns=273445456 avg_ns=736 max_ns=108023
  open_like.pre_open_guard.access: count=1 total_ns=20387 avg_ns=20387 max_ns=20387
  open_like.pre_open_guard.open: count=21 total_ns=299729 avg_ns=14272 max_ns=23498
  open_like.post_open_revalidation.access: count=1 total_ns=33877 avg_ns=33877 max_ns=33877
  open_like.post_open_revalidation.open: count=21 total_ns=69810 avg_ns=3324 max_ns=3932
  stat_child_no_follow: count=247779 total_ns=2958909765 avg_ns=11941 max_ns=648664
  source_root_path: count=371462 total_ns=924915276 avg_ns=2489 max_ns=321199
  resolved_virtual_path: count=619345 total_ns=1557925247 avg_ns=2515 max_ns=332684
  resolved_virtual_path_from_path: count=247820 total_ns=1116021959 avg_ns=4503 max_ns=332684
  resolved_virtual_path_from_path_component_walk: count=247820 total_ns=1003079627 avg_ns=4047 max_ns=331949
  resolved_virtual_path_from_path_canonicalize: count=495360 total_ns=838787343 avg_ns=1693 max_ns=330845
  resolved_virtual_path_from_path_source_root_confinement: count=495360 total_ns=104944588 avg_ns=211 max_ns=25499
  resolved_virtual_path_from_path_virtual_conversion: count=247820 total_ns=97762609 avg_ns=394 max_ns=55612
  resolved_virtual_path_from_open_fd: count=371525 total_ns=441903288 avg_ns=1189 max_ns=262115
  read_handle_snapshot: count=116872 total_ns=26614959 avg_ns=227 max_ns=54016
  read_guard_path: count=116872 total_ns=2100185 avg_ns=17 max_ns=1380
  read_io: count=116872 total_ns=6157877129 avg_ns=52689 max_ns=7387347
  write_handle_snapshot: count=123648 total_ns=26669483 avg_ns=215 max_ns=56640
  write_guard_mutation: count=123648 total_ns=2877550 avg_ns=23 max_ns=1379
  write_io: count=123648 total_ns=978687785 avg_ns=7915 max_ns=1200790
  file_sync.flush: count=21 total_ns=2079536743 avg_ns=99025559 max_ns=302347210
  read_size_bucket.0_4k: count=100800 total_ns=5469351092 avg_ns=54259 max_ns=2280641
  read_size_bucket.4k_64k: count=1505 total_ns=118194704 avg_ns=78534 max_ns=763952
  read_size_bucket.64k_1m: count=14567 total_ns=570331333 avg_ns=39152 max_ns=7387347
  write_size_bucket.0_4k: count=121856 total_ns=451915220 avg_ns=3708 max_ns=801076
  write_size_bucket.64k_1m: count=1792 total_ns=526772565 avg_ns=293957 max_ns=1200790
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
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
