# ScreenFS benchmark result

- timestamp: `2026-06-20T17:27:24.008260+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.json --output-md docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.md --output-svg docs/artifacts/read-write-followup-current/current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.svg`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.028452 | 0.116138 | 4.082 | 0.116629 | 0.116691 | 0.116740 |
| seq_write | 0.036098 | 0.100227 | 2.777 | 0.105017 | 0.105616 | 0.106095 |
| small_read | 0.000157 | 0.008635 | 55.069 | 0.009225 | 0.009299 | 0.009358 |
| small_write | 0.000206 | 0.012284 | 59.687 | 0.012373 | 0.012384 | 0.012393 |
| rand_read_4k | 0.000289 | 0.011116 | 38.402 | 0.011640 | 0.011705 | 0.011757 |
| rand_write_4k | 0.000404 | 0.015021 | 37.149 | 0.015138 | 0.015153 | 0.015165 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=32444 avg_ns=32444 max_ns=32444
  fuse_op.create: count=12 total_ns=969441 avg_ns=80786 max_ns=106636
  fuse_op.flush: count=12 total_ns=452881 avg_ns=37740 max_ns=91160
  fuse_op.getattr: count=5141 total_ns=53553327 avg_ns=10416 max_ns=338821
  fuse_op.getxattr: count=5124 total_ns=78820464 avg_ns=15382 max_ns=96954
  fuse_op.lookup: count=137 total_ns=1604122 avg_ns=11708 max_ns=69181
  fuse_op.open: count=12 total_ns=180630 avg_ns=15052 max_ns=18807
  fuse_op.read: count=10256 total_ns=123261422 avg_ns=12018 max_ns=128525
  fuse_op.release: count=24 total_ns=33951 avg_ns=1414 max_ns=2180
  fuse_op.setattr: count=4 total_ns=122458 avg_ns=30614 max_ns=39735
  fuse_op.statfs: count=2 total_ns=3277 avg_ns=1638 max_ns=2617
  fuse_op.unlink: count=12 total_ns=34110882 avg_ns=2842573 max_ns=9277697
  fuse_op.write: count=5120 total_ns=154571760 avg_ns=30189 max_ns=404053
  policy_decision: count=26386 total_ns=10318207 avg_ns=391 max_ns=31653
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=26294 total_ns=3712146 avg_ns=141 max_ns=61631
  matcher_candidate_order.path: count=79066 total_ns=11352436 avg_ns=143 max_ns=16758
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=377656
  matcher_candidate_order_ancestor_steps.descendant: count=94254
  matcher_candidate_order_ancestor_steps.path: count=283402
  state_read_lock_wait: count=25831 total_ns=493566 avg_ns=19 max_ns=376
  state_read_lock_hold: count=25831 total_ns=2079026 avg_ns=80 max_ns=2171
  state_write_lock_wait: count=183 total_ns=4506 avg_ns=24 max_ns=194
  state_write_lock_hold: count=183 total_ns=148114 avg_ns=809 max_ns=6183
  open_confined_openat2: count=15684 total_ns=9712641 avg_ns=619 max_ns=22246
  open_like.pre_open_guard.access: count=1 total_ns=13856 avg_ns=13856 max_ns=13856
  open_like.pre_open_guard.open: count=12 total_ns=115791 avg_ns=9649 max_ns=12339
  open_like.post_open_revalidation.access: count=1 total_ns=15479 avg_ns=15479 max_ns=15479
  open_like.post_open_revalidation.open: count=12 total_ns=36199 avg_ns=3016 max_ns=4155
  stat_child_no_follow: count=10519 total_ns=89979556 avg_ns=8554 max_ns=336542
  source_root_path: count=15659 total_ns=23935666 avg_ns=1528 max_ns=178088
  resolved_virtual_path: count=26237 total_ns=43009054 avg_ns=1639 max_ns=330351
  resolved_virtual_path_from_path: count=10542 total_ns=27909925 avg_ns=2647 max_ns=330351
  resolved_virtual_path_from_path_component_walk: count=10542 total_ns=23989301 avg_ns=2275 max_ns=329854
  resolved_virtual_path_from_path_canonicalize: count=20921 total_ns=18986156 avg_ns=907 max_ns=329221
  resolved_virtual_path_from_path_source_root_confinement: count=20921 total_ns=2905695 avg_ns=138 max_ns=2367
  resolved_virtual_path_from_path_virtual_conversion: count=10542 total_ns=3340370 avg_ns=316 max_ns=19993
  resolved_virtual_path_from_open_fd: count=15695 total_ns=15099129 avg_ns=962 max_ns=28163
  read_handle_snapshot: count=10256 total_ns=1862743 avg_ns=181 max_ns=3544
  read_guard_path: count=10256 total_ns=138216 avg_ns=13 max_ns=422
  read_io: count=10256 total_ns=119605893 avg_ns=11662 max_ns=128014
  write_handle_snapshot: count=5120 total_ns=1341282 avg_ns=261 max_ns=3855
  write_guard_mutation: count=5120 total_ns=76047 avg_ns=14 max_ns=178
  write_io: count=5120 total_ns=151794265 avg_ns=29647 max_ns=402933
  file_sync.flush: count=12 total_ns=444483 avg_ns=37040 max_ns=90316
  read_size_bucket.0_4k: count=2032 total_ns=1798081 avg_ns=884 max_ns=10228
  read_size_bucket.4k_64k: count=12 total_ns=90321 avg_ns=7526 max_ns=17493
  read_size_bucket.64k_1m: count=8212 total_ns=117717491 avg_ns=14334 max_ns=128014
  write_size_bucket.0_4k: count=4096 total_ns=5297739 avg_ns=1293 max_ns=15330
  write_size_bucket.64k_1m: count=1024 total_ns=146496526 avg_ns=143063 max_ns=402933
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
  invalidations: count=24 invalidated_entries=12 evicted_entries=0 scanned_entries=72
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
