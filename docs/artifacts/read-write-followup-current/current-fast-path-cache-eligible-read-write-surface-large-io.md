# ScreenFS benchmark result

- timestamp: `2026-06-20T17:21:07.485378+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 64 --write-mib 64 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.json --output-md docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.md --output-svg docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-io.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.013538 | 0.038312 | 2.830 | 0.040017 | 0.040480 | 0.040851 |
| seq_write | 0.011063 | 0.036020 | 3.256 | 0.036665 | 0.036809 | 0.036924 |
| small_read | 0.000199 | 0.009214 | 46.333 | 0.009557 | 0.009639 | 0.009704 |
| small_write | 0.000300 | 0.013383 | 44.686 | 0.014481 | 0.014776 | 0.015011 |
| rand_read_4k | 0.000386 | 0.010354 | 26.826 | 0.011518 | 0.011532 | 0.011544 |
| rand_write_4k | 0.000527 | 0.012662 | 24.041 | 0.012956 | 0.012968 | 0.012977 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61761 avg_ns=61761 max_ns=61761
  fuse_op.create: count=21 total_ns=1801224 avg_ns=85772 max_ns=111395
  fuse_op.flush: count=21 total_ns=1026658 avg_ns=48888 max_ns=182815
  fuse_op.getattr: count=7652 total_ns=86108565 avg_ns=11253 max_ns=76448
  fuse_op.getxattr: count=7623 total_ns=123565835 avg_ns=16209 max_ns=106294
  fuse_op.lookup: count=236 total_ns=2903510 avg_ns=12303 max_ns=66298
  fuse_op.open: count=21 total_ns=375106 avg_ns=17862 max_ns=25960
  fuse_op.read: count=7126 total_ns=100545017 avg_ns=14109 max_ns=75090
  fuse_op.release: count=42 total_ns=81355 avg_ns=1937 max_ns=4380
  fuse_op.setattr: count=7 total_ns=216023 avg_ns=30860 max_ns=34846
  fuse_op.statfs: count=2 total_ns=1916 avg_ns=958 max_ns=1304
  fuse_op.unlink: count=21 total_ns=19161412 avg_ns=912448 max_ns=2634021
  fuse_op.write: count=7616 total_ns=88502571 avg_ns=11620 max_ns=366602
  policy_decision: count=39445 total_ns=15342365 avg_ns=388 max_ns=29740
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=39284 total_ns=5467702 avg_ns=139 max_ns=808
  matcher_candidate_order.path: count=118174 total_ns=17075071 avg_ns=144 max_ns=14524
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=564040
  matcher_candidate_order_ancestor_steps.descendant: count=140730
  matcher_candidate_order_ancestor_steps.path: count=423310
  state_read_lock_wait: count=30345 total_ns=681904 avg_ns=22 max_ns=666
  state_read_lock_hold: count=30345 total_ns=2901159 avg_ns=95 max_ns=3046
  state_write_lock_wait: count=318 total_ns=10275 avg_ns=32 max_ns=334
  state_write_lock_hold: count=318 total_ns=274947 avg_ns=864 max_ns=6986
  open_confined_openat2: count=23409 total_ns=17246292 avg_ns=736 max_ns=88128
  open_like.pre_open_guard.access: count=1 total_ns=20388 avg_ns=20388 max_ns=20388
  open_like.pre_open_guard.open: count=21 total_ns=238301 avg_ns=11347 max_ns=18971
  open_like.post_open_revalidation.access: count=1 total_ns=36313 avg_ns=36313 max_ns=36313
  open_like.post_open_revalidation.open: count=21 total_ns=68928 avg_ns=3282 max_ns=3745
  stat_child_no_follow: count=15715 total_ns=146459825 avg_ns=9319 max_ns=95913
  source_root_path: count=23366 total_ns=41037826 avg_ns=1756 max_ns=51514
  resolved_virtual_path: count=39185 total_ns=66100243 avg_ns=1686 max_ns=36943
  resolved_virtual_path_from_path: count=15756 total_ns=42028512 avg_ns=2667 max_ns=28570
  resolved_virtual_path_from_path_component_walk: count=15756 total_ns=35955043 avg_ns=2281 max_ns=27872
  resolved_virtual_path_from_path_canonicalize: count=31232 total_ns=28144994 avg_ns=901 max_ns=10302
  resolved_virtual_path_from_path_source_root_confinement: count=31232 total_ns=4184003 avg_ns=133 max_ns=15371
  resolved_virtual_path_from_path_virtual_conversion: count=15756 total_ns=5034083 avg_ns=319 max_ns=4273
  resolved_virtual_path_from_open_fd: count=23429 total_ns=24071731 avg_ns=1027 max_ns=36943
  read_handle_snapshot: count=7126 total_ns=1543898 avg_ns=216 max_ns=15148
  read_guard_path: count=7126 total_ns=140638 avg_ns=19 max_ns=3929
  read_io: count=7126 total_ns=97382423 avg_ns=13665 max_ns=73925
  write_handle_snapshot: count=7616 total_ns=1845403 avg_ns=242 max_ns=2778
  write_guard_mutation: count=7616 total_ns=118188 avg_ns=15 max_ns=522
  write_io: count=7616 total_ns=84984514 avg_ns=11158 max_ns=353903
  file_sync.flush: count=21 total_ns=1003520 avg_ns=47786 max_ns=181417
  read_size_bucket.0_4k: count=3486 total_ns=2551603 avg_ns=731 max_ns=23269
  read_size_bucket.4k_64k: count=21 total_ns=167451 avg_ns=7973 max_ns=22973
  read_size_bucket.64k_1m: count=3619 total_ns=94663369 avg_ns=26157 max_ns=73925
  write_size_bucket.0_4k: count=7168 total_ns=8399826 avg_ns=1171 max_ns=61027
  write_size_bucket.64k_1m: count=448 total_ns=76584688 avg_ns=170947 max_ns=353903
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
