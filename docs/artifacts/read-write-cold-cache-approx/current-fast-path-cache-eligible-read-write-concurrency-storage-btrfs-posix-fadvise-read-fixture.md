# ScreenFS benchmark result

- timestamp: `2026-06-20T19:01:25.461044+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-concurrency --concurrency-workers 4 --cache-control posix-fadvise-read-fixture --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json --output-md docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md --output-svg docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`
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
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
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
| concurrent_rand_read_write_4k | 0.243360 | 0.984798 | 4.047 | 1.016089 | 1.020835 | 1.024632 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=58146 avg_ns=58146 max_ns=58146
  fuse_op.create: count=28 total_ns=4126622 avg_ns=147379 max_ns=222808
  fuse_op.flush: count=28 total_ns=7094412646 avg_ns=253371880 max_ns=434636060
  fuse_op.getattr: count=114689 total_ns=2131538640 avg_ns=18585 max_ns=1879127
  fuse_op.getxattr: count=114716 total_ns=3184498903 avg_ns=27759 max_ns=1923156
  fuse_op.lookup: count=257 total_ns=6183039 avg_ns=24058 max_ns=87873
  fuse_op.open: count=28 total_ns=655792 avg_ns=23421 max_ns=42824
  fuse_op.read: count=111314 total_ns=7636371974 avg_ns=68602 max_ns=4291366
  fuse_op.release: count=56 total_ns=135275 avg_ns=2415 max_ns=5582
  fuse_op.setattr: count=28 total_ns=1586636 avg_ns=56665 max_ns=96595
  fuse_op.statfs: count=2 total_ns=4234 avg_ns=2117 max_ns=2566
  fuse_op.unlink: count=28 total_ns=136155300 avg_ns=4862689 max_ns=7996235
  fuse_op.write: count=114688 total_ns=608233053 avg_ns=5303 max_ns=928186
  policy_decision: count=575274 total_ns=269937396 avg_ns=469 max_ns=1055191
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=575022 total_ns=97256253 avg_ns=169 max_ns=50827
  matcher_candidate_order.path: count=1725570 total_ns=303704958 avg_ns=176 max_ns=1859364
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=8279048
  matcher_candidate_order_ancestor_steps.descendant: count=2069314
  matcher_candidate_order_ancestor_steps.path: count=6209734
  state_read_lock_wait: count=455805 total_ns=37098890 avg_ns=81 max_ns=312064
  state_read_lock_hold: count=455805 total_ns=72280692 avg_ns=158 max_ns=175634
  state_write_lock_wait: count=367 total_ns=22020 avg_ns=60 max_ns=1463
  state_write_lock_hold: count=367 total_ns=656391 avg_ns=1788 max_ns=29002
  open_confined_openat2: count=344800 total_ns=383131698 avg_ns=1111 max_ns=1827312
  open_like.pre_open_guard.access: count=1 total_ns=21999 avg_ns=21999 max_ns=21999
  open_like.pre_open_guard.open: count=28 total_ns=458971 avg_ns=16391 max_ns=34212
  open_like.post_open_revalidation.access: count=1 total_ns=31881 avg_ns=31881 max_ns=31881
  open_like.post_open_revalidation.open: count=28 total_ns=98388 avg_ns=3513 max_ns=4098
  stat_child_no_follow: count=229971 total_ns=3606598270 avg_ns=15682 max_ns=1860681
  source_root_path: count=344743 total_ns=1120698265 avg_ns=3250 max_ns=669304
  resolved_virtual_path: count=574853 total_ns=1707072449 avg_ns=2969 max_ns=1194693
  resolved_virtual_path_from_path: count=230026 total_ns=1179276700 avg_ns=5126 max_ns=1194693
  resolved_virtual_path_from_path_component_walk: count=230026 total_ns=1055340138 avg_ns=4587 max_ns=1192869
  resolved_virtual_path_from_path_canonicalize: count=459737 total_ns=892571061 avg_ns=1941 max_ns=865087
  resolved_virtual_path_from_path_source_root_confinement: count=459737 total_ns=102162939 avg_ns=222 max_ns=1184132
  resolved_virtual_path_from_path_virtual_conversion: count=230026 total_ns=106203358 avg_ns=461 max_ns=49235
  resolved_virtual_path_from_open_fd: count=344827 total_ns=527795749 avg_ns=1530 max_ns=584350
  read_handle_snapshot: count=111314 total_ns=48400504 avg_ns=434 max_ns=25591
  read_guard_path: count=111314 total_ns=2372962 avg_ns=21 max_ns=7245
  read_io: count=111314 total_ns=7542330279 avg_ns=67757 max_ns=4289693
  write_handle_snapshot: count=114688 total_ns=50187901 avg_ns=437 max_ns=93252
  write_guard_mutation: count=114688 total_ns=2362385 avg_ns=20 max_ns=3490
  write_io: count=114688 total_ns=519150622 avg_ns=4526 max_ns=926888
  file_sync.flush: count=28 total_ns=7094360438 avg_ns=253370015 max_ns=434633473
  read_size_bucket.0_4k: count=111167 total_ns=7528156873 avg_ns=67719 max_ns=4289693
  read_size_bucket.4k_64k: count=147 total_ns=14173406 avg_ns=96417 max_ns=1440972
  write_size_bucket.0_4k: count=114688 total_ns=519150622 avg_ns=4526 max_ns=926888
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
