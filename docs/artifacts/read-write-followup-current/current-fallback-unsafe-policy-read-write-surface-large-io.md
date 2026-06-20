# ScreenFS benchmark result

- timestamp: `2026-06-20T17:21:35.581304+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 64 --write-mib 64 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.json --output-md docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.md --output-svg docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.012605 | 0.048538 | 3.851 | 0.048993 | 0.049050 | 0.049096 |
| seq_write | 0.011796 | 0.027652 | 2.344 | 0.028137 | 0.028292 | 0.028417 |
| small_read | 0.000200 | 0.008436 | 42.200 | 0.009065 | 0.009088 | 0.009106 |
| small_write | 0.000304 | 0.024913 | 81.853 | 0.026909 | 0.027388 | 0.027771 |
| rand_read_4k | 0.000387 | 0.020046 | 51.767 | 0.020325 | 0.020381 | 0.020425 |
| rand_write_4k | 0.000585 | 0.024934 | 42.592 | 0.025985 | 0.025988 | 0.025990 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=56541 avg_ns=56541 max_ns=56541
  fuse_op.create: count=21 total_ns=1983699 avg_ns=94461 max_ns=131085
  fuse_op.flush: count=21 total_ns=727346 avg_ns=34635 max_ns=143239
  fuse_op.getattr: count=7652 total_ns=96270199 avg_ns=12581 max_ns=98089
  fuse_op.getxattr: count=7623 total_ns=142150304 avg_ns=18647 max_ns=982760
  fuse_op.lookup: count=236 total_ns=3216787 avg_ns=13630 max_ns=77368
  fuse_op.open: count=21 total_ns=397324 avg_ns=18920 max_ns=26098
  fuse_op.read: count=7126 total_ns=220449073 avg_ns=30935 max_ns=273258
  fuse_op.release: count=42 total_ns=67773 avg_ns=1613 max_ns=4246
  fuse_op.setattr: count=7 total_ns=265803 avg_ns=37971 max_ns=46918
  fuse_op.statfs: count=2 total_ns=3166 avg_ns=1583 max_ns=2435
  fuse_op.unlink: count=21 total_ns=16776088 avg_ns=798861 max_ns=2754514
  fuse_op.write: count=7616 total_ns=238793343 avg_ns=31354 max_ns=416101
  policy_decision: count=114177 total_ns=49261795 avg_ns=431 max_ns=27074
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=98784 total_ns=12800108 avg_ns=129 max_ns=17515
  matcher_candidate_order.path: count=327138 total_ns=53888844 avg_ns=164 max_ns=322518
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=114177
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=114177
  matcher_candidate_order_ancestor_steps: count=1578760
  matcher_candidate_order_ancestor_steps.descendant: count=363946
  matcher_candidate_order_ancestor_steps.path: count=1214814
  state_read_lock_wait: count=30345 total_ns=582191 avg_ns=19 max_ns=1101
  state_read_lock_hold: count=30345 total_ns=2583513 avg_ns=85 max_ns=3553
  state_write_lock_wait: count=318 total_ns=7738 avg_ns=24 max_ns=166
  state_write_lock_hold: count=318 total_ns=241759 avg_ns=760 max_ns=8507
  open_confined_openat2: count=38193 total_ns=20830611 avg_ns=545 max_ns=68765
  open_like.pre_open_guard.access: count=1 total_ns=41264 avg_ns=41264 max_ns=41264
  open_like.pre_open_guard.open: count=21 total_ns=291510 avg_ns=13881 max_ns=18342
  open_like.post_open_revalidation.access: count=1 total_ns=11022 avg_ns=11022 max_ns=11022
  open_like.post_open_revalidation.open: count=21 total_ns=60280 avg_ns=2870 max_ns=3538
  stat_child_no_follow: count=30499 total_ns=235956133 avg_ns=7736 max_ns=322481
  source_root_path: count=45773 total_ns=60912777 avg_ns=1330 max_ns=320804
  resolved_virtual_path: count=113866 total_ns=230074257 avg_ns=2020 max_ns=325963
  resolved_virtual_path_from_path: count=60911 total_ns=180971229 avg_ns=2971 max_ns=325963
  resolved_virtual_path_from_path_component_walk: count=60911 total_ns=159496333 avg_ns=2618 max_ns=325448
  resolved_virtual_path_from_path_canonicalize: count=151592 total_ns=129647674 avg_ns=855 max_ns=324631
  resolved_virtual_path_from_path_source_root_confinement: count=151592 total_ns=16908008 avg_ns=111 max_ns=30908
  resolved_virtual_path_from_path_virtual_conversion: count=60911 total_ns=18492696 avg_ns=303 max_ns=5952
  resolved_virtual_path_from_open_fd: count=52955 total_ns=49103028 avg_ns=927 max_ns=74572
  read_handle_snapshot: count=7126 total_ns=1636693 avg_ns=229 max_ns=3659
  read_guard_path: count=7126 total_ns=126729561 avg_ns=17784 max_ns=88468
  read_io: count=7126 total_ns=90506976 avg_ns=12700 max_ns=258361
  write_handle_snapshot: count=7616 total_ns=1607942 avg_ns=211 max_ns=6196
  write_guard_mutation: count=7616 total_ns=158937413 avg_ns=20868 max_ns=100751
  write_io: count=7616 total_ns=76789829 avg_ns=10082 max_ns=312875
  file_sync.flush: count=21 total_ns=708552 avg_ns=33740 max_ns=142132
  read_size_bucket.0_4k: count=3486 total_ns=2450816 avg_ns=703 max_ns=258361
  read_size_bucket.4k_64k: count=21 total_ns=56924 avg_ns=2710 max_ns=8870
  read_size_bucket.64k_1m: count=3619 total_ns=87999236 avg_ns=24315 max_ns=116742
  write_size_bucket.0_4k: count=7168 total_ns=6686300 avg_ns=932 max_ns=47807
  write_size_bucket.64k_1m: count=448 total_ns=70103529 avg_ns=156481 max_ns=312875
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
