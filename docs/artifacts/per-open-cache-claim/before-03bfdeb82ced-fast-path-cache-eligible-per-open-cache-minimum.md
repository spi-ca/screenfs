# ScreenFS benchmark result

- timestamp: `2026-06-15T07:58:45.703816+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --iterations 10 --warmups 3 --policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum --perf-counters --read-mib 16 --write-mib 16 --rand-io-ops 4096 --sync-ops 128 --open-read-close-ops 1024 --screenfs-bin /tmp/screenfs-goal-bench/before-target/release/screenfs --screenfs-source-root /tmp/screenfs-goal-bench/before --output-json docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json --output-md docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.md --output-svg docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_dirty_status: `M .gitignore;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md;  M docs/performance-roadmap.md; ... (+80 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-goal-bench/before-target/release/screenfs`
- screenfs_bin_sha256: `2a3335c58ca1419a2e38ee5ca257f1578c98143c901ff6609819da70bf12eb5d`
- screenfs_source_root: `/tmp/screenfs-goal-bench/before`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `per-open-cache-minimum`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| rand_read_4k | 0.006268 | 0.134331 | 21.432 | 0.135624 | 0.135638 | 0.135650 |
| rand_write_4k | 0.003877 | 0.171172 | 44.154 | 0.173194 | 0.173343 | 0.173462 |
| sync_write_4k | 0.000146 | 0.005437 | 37.218 | 0.005526 | 0.005527 | 0.005528 |
| small_open_read_close | 0.005878 | 0.158687 | 26.998 | 0.161717 | 0.166215 | 0.169813 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73040 avg_ns=73040 max_ns=73040
  fuse_op.create: count=26 total_ns=2458518 avg_ns=94558 max_ns=138942
  fuse_op.flush: count=13351 total_ns=66823017 avg_ns=5005 max_ns=361715
  fuse_op.fsync: count=52 total_ns=382763 avg_ns=7360 max_ns=19250
  fuse_op.getattr: count=79886 total_ns=1080286375 avg_ns=13522 max_ns=464870
  fuse_op.getxattr: count=54925 total_ns=799118343 avg_ns=14549 max_ns=67274
  fuse_op.lookup: count=40175 total_ns=552607517 avg_ns=13755 max_ns=86206
  fuse_op.open: count=13325 total_ns=184863057 avg_ns=13873 max_ns=156342
  fuse_op.read: count=48763 total_ns=498715132 avg_ns=10227 max_ns=73938
  fuse_op.release: count=13351 total_ns=8345982 avg_ns=625 max_ns=11862
  fuse_op.setattr: count=13 total_ns=483086 avg_ns=37160 max_ns=39126
  fuse_op.statfs: count=2 total_ns=6645 avg_ns=3322 max_ns=3356
  fuse_op.unlink: count=26 total_ns=10059118 avg_ns=386889 max_ns=877773
  fuse_op.write: count=54912 total_ns=740482895 avg_ns=13484 max_ns=209182
  policy_decision: count=883097 total_ns=349116795 avg_ns=395 max_ns=35339
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=827977 total_ns=122094855 avg_ns=147 max_ns=14734
  matcher_candidate_order.path: count=2594171 total_ns=399296888 avg_ns=153 max_ns=52039
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=11555916
  matcher_candidate_order_ancestor_steps.descendant: count=2778791
  matcher_candidate_order_ancestor_steps.path: count=8777125
  state_read_lock_wait: count=305455 total_ns=7470488 avg_ns=24 max_ns=6588
  state_read_lock_hold: count=305455 total_ns=22819356 avg_ns=74 max_ns=11907
  state_write_lock_wait: count=66875 total_ns=1618032 avg_ns=24 max_ns=641
  state_write_lock_hold: count=66875 total_ns=28664452 avg_ns=428 max_ns=53236
  open_confined_openat2: count=480637 total_ns=308949687 avg_ns=642 max_ns=54631
  source_root_path: count=480662 total_ns=806466275 avg_ns=1677 max_ns=59222
  resolved_virtual_path: count=480714 total_ns=523375941 avg_ns=1088 max_ns=52524
  resolved_virtual_path_from_path: count=52 total_ns=211938 avg_ns=4075 max_ns=6517
  resolved_virtual_path_from_path_component_walk: count=52 total_ns=178355 avg_ns=3429 max_ns=4994
  resolved_virtual_path_from_path_canonicalize: count=104 total_ns=125202 avg_ns=1203 max_ns=3201
  resolved_virtual_path_from_path_source_root_confinement: count=104 total_ns=21421 avg_ns=205 max_ns=999
  resolved_virtual_path_from_path_virtual_conversion: count=52 total_ns=20576 avg_ns=395 max_ns=709
  resolved_virtual_path_from_open_fd: count=480662 total_ns=523164003 avg_ns=1088 max_ns=52524
  read_size_bucket.0_4k: count=44889 total_ns=82244070 avg_ns=1832 max_ns=26570
  read_size_bucket.4k_64k: count=3874 total_ns=18842770 avg_ns=4863 max_ns=20154
  write_size_bucket.0_4k: count=54912 total_ns=120842856 avg_ns=2200 max_ns=198000
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
  invalidations: count=52 invalidated_entries=26 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
