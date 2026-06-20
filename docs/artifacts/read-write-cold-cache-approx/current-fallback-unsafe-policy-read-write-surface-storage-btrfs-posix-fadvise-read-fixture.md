# ScreenFS benchmark result

- timestamp: `2026-06-20T19:02:01.318244+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --cache-control posix-fadvise-read-fixture --read-mib 256 --write-mib 256 --iterations 5 --warmups 2 --output-json docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json --output-md docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md --output-svg docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`
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
| seq_read | 0.101646 | 0.238524 | 2.347 | 0.249667 | 0.250833 | 0.251765 |
| seq_write | 0.081155 | 0.152854 | 1.883 | 0.155941 | 0.156598 | 0.157124 |
| small_read | 0.002484 | 0.026639 | 10.722 | 0.027334 | 0.027432 | 0.027511 |
| small_write | 0.001028 | 0.068532 | 66.670 | 0.079546 | 0.082956 | 0.085684 |
| rand_read_4k | 0.797193 | 1.816882 | 2.279 | 1.857386 | 1.870322 | 1.880671 |
| rand_write_4k | 0.025612 | 1.519007 | 59.308 | 1.579606 | 1.588442 | 1.595510 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=57723 avg_ns=57723 max_ns=57723
  fuse_op.create: count=21 total_ns=3136162 avg_ns=149341 max_ns=216898
  fuse_op.flush: count=21 total_ns=2503341341 avg_ns=119206730 max_ns=347268488
  fuse_op.getattr: count=123684 total_ns=2746805596 avg_ns=22208 max_ns=1002531
  fuse_op.getxattr: count=123655 total_ns=3386453462 avg_ns=27386 max_ns=2096624
  fuse_op.lookup: count=236 total_ns=6003897 avg_ns=25440 max_ns=84100
  fuse_op.open: count=21 total_ns=565065 avg_ns=26907 max_ns=40145
  fuse_op.read: count=116872 total_ns=9226669643 avg_ns=78946 max_ns=2773076
  fuse_op.release: count=42 total_ns=135818 avg_ns=3233 max_ns=7232
  fuse_op.setattr: count=7 total_ns=469528 avg_ns=67075 max_ns=73419
  fuse_op.statfs: count=2 total_ns=7060 avg_ns=3530 max_ns=3613
  fuse_op.unlink: count=21 total_ns=139656326 avg_ns=6650301 max_ns=11923814
  fuse_op.write: count=123648 total_ns=4409206326 avg_ns=35659 max_ns=2630142
  policy_decision: count=1835799 total_ns=894461397 avg_ns=487 max_ns=271906
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1588342 total_ns=230930499 avg_ns=145 max_ns=514040
  matcher_candidate_order.path: count=5259940 total_ns=973134870 avg_ns=185 max_ns=259120
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1835799
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1835799
  matcher_candidate_order_ancestor_steps: count=25436832
  matcher_candidate_order_ancestor_steps.descendant: count=5864336
  matcher_candidate_order_ancestor_steps.path: count=19572496
  state_read_lock_wait: count=488187 total_ns=11046910 avg_ns=22 max_ns=8070
  state_read_lock_hold: count=488187 total_ns=50397112 avg_ns=103 max_ns=57623
  state_write_lock_wait: count=318 total_ns=12240 avg_ns=38 max_ns=1118
  state_write_lock_hold: count=318 total_ns=383621 avg_ns=1206 max_ns=22355
  open_confined_openat2: count=612067 total_ns=425165915 avg_ns=694 max_ns=2018295
  open_like.pre_open_guard.access: count=1 total_ns=48308 avg_ns=48308 max_ns=48308
  open_like.pre_open_guard.open: count=21 total_ns=434546 avg_ns=20692 max_ns=32073
  open_like.post_open_revalidation.access: count=1 total_ns=4860 avg_ns=4860 max_ns=4860
  open_like.post_open_revalidation.open: count=21 total_ns=68450 avg_ns=3259 max_ns=4021
  stat_child_no_follow: count=488341 total_ns=5685959111 avg_ns=11643 max_ns=2068498
  source_root_path: count=735679 total_ns=1698314479 avg_ns=2308 max_ns=471394
  resolved_virtual_path: count=1829202 total_ns=6429437543 avg_ns=3514 max_ns=1604949
  resolved_virtual_path_from_path: count=976595 total_ns=5465130449 avg_ns=5596 max_ns=1604949
  resolved_virtual_path_from_path_component_walk: count=976595 total_ns=5002945004 avg_ns=5122 max_ns=1604170
  resolved_virtual_path_from_path_canonicalize: count=2440802 total_ns=4280027887 avg_ns=1753 max_ns=1602933
  resolved_virtual_path_from_path_source_root_confinement: count=2440802 total_ns=467089848 avg_ns=191 max_ns=121762
  resolved_virtual_path_from_path_virtual_conversion: count=976595 total_ns=405348836 avg_ns=415 max_ns=395874
  resolved_virtual_path_from_open_fd: count=852607 total_ns=964307094 avg_ns=1131 max_ns=329193
  read_handle_snapshot: count=116872 total_ns=32054945 avg_ns=274 max_ns=58349
  read_guard_path: count=116872 total_ns=2854911177 avg_ns=24427 max_ns=1497436
  read_io: count=116872 total_ns=6310750094 avg_ns=53997 max_ns=2739294
  write_handle_snapshot: count=123648 total_ns=27764808 avg_ns=224 max_ns=15707
  write_guard_mutation: count=123648 total_ns=3480781603 avg_ns=28150 max_ns=707125
  write_io: count=123648 total_ns=876307283 avg_ns=7087 max_ns=2569610
  file_sync.flush: count=21 total_ns=2503307273 avg_ns=119205108 max_ns=347267439
  read_size_bucket.0_4k: count=100800 total_ns=5708945309 avg_ns=56636 max_ns=2517948
  read_size_bucket.4k_64k: count=1505 total_ns=121496666 avg_ns=80728 max_ns=2739294
  read_size_bucket.64k_1m: count=14567 total_ns=480308119 avg_ns=32972 max_ns=2332273
  write_size_bucket.0_4k: count=121856 total_ns=421346037 avg_ns=3457 max_ns=503250
  write_size_bucket.64k_1m: count=1792 total_ns=454961246 avg_ns=253884 max_ns=2569610
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
