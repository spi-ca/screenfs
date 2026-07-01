# ScreenFS benchmark result

- timestamp: `2026-07-01T05:14:15.197551+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --cache-control posix-fadvise-read-fixture --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+64 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+64 more)`
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
| seq_read | 0.096660 | 0.170345 | 1.762 | 0.244386 | 0.246654 | 0.248469 |
| seq_write | 0.082493 | 0.209431 | 2.539 | 0.212748 | 0.213641 | 0.214355 |
| small_read | 0.002718 | 0.020506 | 7.546 | 0.023908 | 0.025025 | 0.025919 |
| small_write | 0.001366 | 0.051109 | 37.421 | 0.052438 | 0.052799 | 0.053088 |
| rand_read_4k | 0.779906 | 1.344403 | 1.724 | 1.474130 | 1.514704 | 1.547163 |
| rand_write_4k | 0.023719 | 1.386593 | 58.459 | 1.388652 | 1.388689 | 1.388720 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=47958 avg_ns=47958 max_ns=47958
  fuse_op.create: count=21 total_ns=2430359 avg_ns=115731 max_ns=209774
  fuse_op.flush: count=21 total_ns=2467200384 avg_ns=117485732 max_ns=349911546
  fuse_op.getattr: count=123684 total_ns=1496568562 avg_ns=12099 max_ns=681333
  fuse_op.getxattr: count=123655 total_ns=2494123885 avg_ns=20170 max_ns=508687
  fuse_op.lookup: count=236 total_ns=4409462 avg_ns=18684 max_ns=83464
  fuse_op.open: count=21 total_ns=381929 avg_ns=18187 max_ns=22754
  fuse_op.read: count=116872 total_ns=7495966386 avg_ns=64138 max_ns=2007736
  fuse_op.release: count=42 total_ns=161792 avg_ns=3852 max_ns=10928
  fuse_op.setattr: count=7 total_ns=417466 avg_ns=59638 max_ns=60635
  fuse_op.statfs: count=2 total_ns=4420 avg_ns=2210 max_ns=3350
  fuse_op.unlink: count=21 total_ns=199795406 avg_ns=9514066 max_ns=20317745
  fuse_op.write: count=123648 total_ns=4028777127 avg_ns=32582 max_ns=4573465
  policy_decision: count=1347459 total_ns=769955347 avg_ns=571 max_ns=261832
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1100002 total_ns=176701105 avg_ns=160 max_ns=47902
  matcher_candidate_order.path: count=3794920 total_ns=778078813 avg_ns=205 max_ns=329722
  matcher_candidate_order_by_source.hidden.path: count=1100002 total_ns=304764458 avg_ns=277 max_ns=329722
  matcher_candidate_order_by_source.internal_hidden.path: count=1100002 total_ns=187322055 avg_ns=170 max_ns=262335
  matcher_candidate_order_by_source.readonly.path: count=247457 total_ns=69721434 avg_ns=281 max_ns=16105
  matcher_candidate_order_by_source.visible.descendant: count=1100002 total_ns=176701105 avg_ns=160 max_ns=47902
  matcher_candidate_order_by_source.visible.path: count=1100002 total_ns=175635664 avg_ns=159 max_ns=54572
  matcher_candidate_order_by_source.writable.path: count=247457 total_ns=40635202 avg_ns=164 max_ns=43847
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1347459
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1347459
  matcher_candidate_order_ancestor_steps: count=19577872
  matcher_candidate_order_ancestor_steps.descendant: count=4399596
  matcher_candidate_order_ancestor_steps.path: count=15178276
  state_read_lock_wait: count=488187 total_ns=13818472 avg_ns=28 max_ns=18171
  state_read_lock_hold: count=488187 total_ns=45346019 avg_ns=92 max_ns=115362
  state_write_lock_wait: count=318 total_ns=14289 avg_ns=44 max_ns=530
  state_write_lock_hold: count=318 total_ns=391636 avg_ns=1231 max_ns=12173
  open_confined_openat2: count=612067 total_ns=459928345 avg_ns=751 max_ns=291297
  open_like.pre_open_guard.access: count=1 total_ns=38830 avg_ns=38830 max_ns=38830
  open_like.pre_open_guard.open: count=21 total_ns=256370 avg_ns=12208 max_ns=15774
  open_like.post_open_revalidation.access: count=1 total_ns=5049 avg_ns=5049 max_ns=5049
  open_like.post_open_revalidation.open: count=21 total_ns=75663 avg_ns=3603 max_ns=4914
  stat_child_no_follow: count=488341 total_ns=654782445 avg_ns=1340 max_ns=305834
  stat_child_no_follow.attr_conversion: count=488234 total_ns=7423443 avg_ns=15 max_ns=5910
  stat_child_no_follow.host_fstat: count=488234 total_ns=100708985 avg_ns=206 max_ns=47315
  stat_child_no_follow_context.path_guard_or_metadata: count=488341 total_ns=654782445 avg_ns=1340 max_ns=305834
  source_root_path: count=611919 total_ns=1355113164 avg_ns=2214 max_ns=471765
  resolved_virtual_path: count=852522 total_ns=3576479082 avg_ns=4195 max_ns=358830
  resolved_virtual_path_from_path: count=488255 total_ns=3172372683 avg_ns=6497 max_ns=358830
  resolved_virtual_path_from_path_component_walk: count=488255 total_ns=2917732174 avg_ns=5975 max_ns=356299
  resolved_virtual_path_from_path_canonicalize: count=1464402 total_ns=2504656776 avg_ns=1710 max_ns=354406
  resolved_virtual_path_from_path_source_root_confinement: count=1464402 total_ns=270878362 avg_ns=184 max_ns=258862
  resolved_virtual_path_from_path_virtual_conversion: count=488255 total_ns=227329824 avg_ns=465 max_ns=48173
  resolved_virtual_path_from_open_fd: count=364267 total_ns=404106399 avg_ns=1109 max_ns=68361
  read_handle_snapshot: count=116872 total_ns=24772115 avg_ns=211 max_ns=65701
  read_guard_path: count=116872 total_ns=1662043580 avg_ns=14221 max_ns=294191
  read_io: count=116872 total_ns=5787508257 avg_ns=49520 max_ns=1992021
  write_handle_snapshot: count=123648 total_ns=30347278 avg_ns=245 max_ns=115746
  write_guard_mutation: count=123648 total_ns=2893028391 avg_ns=23397 max_ns=693885
  write_io: count=123648 total_ns=1082492236 avg_ns=8754 max_ns=4510872
  file_sync.flush: count=21 total_ns=2467160430 avg_ns=117483830 max_ns=349910257
  read_size_bucket.0_4k: count=100800 total_ns=5247983688 avg_ns=52063 max_ns=1992021
  read_size_bucket.4k_64k: count=1505 total_ns=115495586 avg_ns=76741 max_ns=549970
  read_size_bucket.64k_1m: count=14567 total_ns=424028983 avg_ns=29108 max_ns=1820011
  write_size_bucket.0_4k: count=121856 total_ns=507476164 avg_ns=4164 max_ns=698077
  write_size_bucket.64k_1m: count=1792 total_ns=575016072 avg_ns=320879 max_ns=4510872
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
