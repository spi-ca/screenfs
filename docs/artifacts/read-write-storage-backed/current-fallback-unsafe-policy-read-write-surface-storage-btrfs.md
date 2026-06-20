# ScreenFS benchmark result

- timestamp: `2026-06-20T18:10:07.027120+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.json --output-md docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.md --output-svg docs/artifacts/read-write-storage-backed/current-fallback-unsafe-policy-read-write-surface-storage-btrfs.svg`
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
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.048588 | 0.196820 | 4.051 | 0.200249 | 0.201008 | 0.201616 |
| seq_write | 0.077392 | 0.191252 | 2.471 | 0.202961 | 0.204840 | 0.206343 |
| small_read | 0.000819 | 0.027007 | 32.963 | 0.027320 | 0.027374 | 0.027418 |
| small_write | 0.001820 | 0.068683 | 37.729 | 0.069726 | 0.069806 | 0.069869 |
| rand_read_4k | 0.023776 | 0.968289 | 40.726 | 0.975795 | 0.976014 | 0.976190 |
| rand_write_4k | 0.041455 | 1.609473 | 38.825 | 1.612224 | 1.612509 | 1.612736 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=65125 avg_ns=65125 max_ns=65125
  fuse_op.create: count=21 total_ns=3437684 avg_ns=163699 max_ns=246255
  fuse_op.flush: count=21 total_ns=2188992592 avg_ns=104237742 max_ns=362796009
  fuse_op.getattr: count=123684 total_ns=2667703541 avg_ns=21568 max_ns=630489
  fuse_op.getxattr: count=123655 total_ns=3772222476 avg_ns=30506 max_ns=319493
  fuse_op.lookup: count=236 total_ns=5625920 avg_ns=23838 max_ns=74531
  fuse_op.open: count=21 total_ns=596622 avg_ns=28410 max_ns=65566
  fuse_op.read: count=116872 total_ns=3472303498 avg_ns=29710 max_ns=555658
  fuse_op.release: count=42 total_ns=148961 avg_ns=3546 max_ns=6848
  fuse_op.setattr: count=7 total_ns=589476 avg_ns=84210 max_ns=132747
  fuse_op.statfs: count=2 total_ns=5848 avg_ns=2924 max_ns=4314
  fuse_op.unlink: count=21 total_ns=155905920 avg_ns=7424091 max_ns=12893789
  fuse_op.write: count=123648 total_ns=4857602143 avg_ns=39285 max_ns=1341420
  policy_decision: count=1835799 total_ns=960220382 avg_ns=523 max_ns=312031
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1588342 total_ns=245740983 avg_ns=154 max_ns=88656
  matcher_candidate_order.path: count=5259940 total_ns=1045036050 avg_ns=198 max_ns=262351
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1835799
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1835799
  matcher_candidate_order_ancestor_steps: count=25436832
  matcher_candidate_order_ancestor_steps.descendant: count=5864336
  matcher_candidate_order_ancestor_steps.path: count=19572496
  state_read_lock_wait: count=488187 total_ns=12042930 avg_ns=24 max_ns=4021
  state_read_lock_hold: count=488187 total_ns=46106688 avg_ns=94 max_ns=60618
  state_write_lock_wait: count=318 total_ns=13189 avg_ns=41 max_ns=427
  state_write_lock_hold: count=318 total_ns=430855 avg_ns=1354 max_ns=22721
  open_confined_openat2: count=612067 total_ns=463610606 avg_ns=757 max_ns=554904
  open_like.pre_open_guard.access: count=1 total_ns=56327 avg_ns=56327 max_ns=56327
  open_like.pre_open_guard.open: count=21 total_ns=458698 avg_ns=21842 max_ns=55644
  open_like.post_open_revalidation.access: count=1 total_ns=3889 avg_ns=3889 max_ns=3889
  open_like.post_open_revalidation.open: count=21 total_ns=67934 avg_ns=3234 max_ns=4179
  stat_child_no_follow: count=488341 total_ns=5962991666 avg_ns=12210 max_ns=619366
  source_root_path: count=735679 total_ns=1809630632 avg_ns=2459 max_ns=499588
  resolved_virtual_path: count=1829202 total_ns=6775254019 avg_ns=3703 max_ns=585801
  resolved_virtual_path_from_path: count=976595 total_ns=5730679916 avg_ns=5868 max_ns=290517
  resolved_virtual_path_from_path_component_walk: count=976595 total_ns=5257664526 avg_ns=5383 max_ns=289819
  resolved_virtual_path_from_path_canonicalize: count=2440802 total_ns=4511316433 avg_ns=1848 max_ns=288889
  resolved_virtual_path_from_path_source_root_confinement: count=2440802 total_ns=481750864 avg_ns=197 max_ns=95057
  resolved_virtual_path_from_path_virtual_conversion: count=976595 total_ns=414166318 avg_ns=424 max_ns=86158
  resolved_virtual_path_from_open_fd: count=852607 total_ns=1044574103 avg_ns=1225 max_ns=585801
  read_handle_snapshot: count=116872 total_ns=26617277 avg_ns=227 max_ns=9205
  read_guard_path: count=116872 total_ns=2901664507 avg_ns=24827 max_ns=550785
  read_io: count=116872 total_ns=520166499 avg_ns=4450 max_ns=107546
  write_handle_snapshot: count=123648 total_ns=28568593 avg_ns=231 max_ns=60861
  write_guard_mutation: count=123648 total_ns=3838952397 avg_ns=31047 max_ns=358485
  write_io: count=123648 total_ns=964941389 avg_ns=7803 max_ns=1250010
  file_sync.flush: count=21 total_ns=2188941937 avg_ns=104235330 max_ns=362793032
  read_size_bucket.0_4k: count=100800 total_ns=169650131 avg_ns=1683 max_ns=107546
  read_size_bucket.4k_64k: count=1505 total_ns=5368413 avg_ns=3567 max_ns=15988
  read_size_bucket.64k_1m: count=14567 total_ns=345147955 avg_ns=23693 max_ns=99717
  write_size_bucket.0_4k: count=121856 total_ns=484463505 avg_ns=3975 max_ns=279481
  write_size_bucket.64k_1m: count=1792 total_ns=480477884 avg_ns=268123 max_ns=1250010
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
