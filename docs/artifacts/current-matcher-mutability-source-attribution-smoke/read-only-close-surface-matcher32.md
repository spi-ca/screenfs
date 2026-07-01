# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:42.377931+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-matcher32 --cache-control warm --workload-set read-only-close-surface --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/read-only-close-surface-matcher32.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+5 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+5 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-only-close-surface`
- comparable_workloads: `read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| read_only_open_close | 0.027700 | 0.362791 | 13.097 | 0.363794 | 0.363919 | 0.364020 |
| read_only_open_read_close | 0.030815 | 0.569079 | 18.468 | 0.573827 | 0.574421 | 0.574895 |
| write_open_write_close | 0.000318 | 0.027345 | 85.867 | 0.027359 | 0.027360 | 0.027362 |
| write_open_fsync_close | 0.000338 | 0.028221 | 83.556 | 0.028310 | 0.028321 | 0.028330 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61153 avg_ns=61153 max_ns=61153
  fuse_op.create: count=8 total_ns=879906 avg_ns=109988 max_ns=131513
  fuse_op.flush: count=1024 total_ns=5010372 avg_ns=4892 max_ns=166442
  fuse_op.fsync: count=512 total_ns=2232975 avg_ns=4361 max_ns=18954
  fuse_op.getattr: count=16385 total_ns=223623687 avg_ns=13648 max_ns=76170
  fuse_op.getxattr: count=2040 total_ns=47397298 avg_ns=23233 max_ns=44458
  fuse_op.lookup: count=101405 total_ns=1186252603 avg_ns=11698 max_ns=112823
  fuse_op.open: count=33784 total_ns=750025554 avg_ns=22200 max_ns=85031
  fuse_op.read: count=16384 total_ns=354827543 avg_ns=21656 max_ns=172421
  fuse_op.release: count=33792 total_ns=17787754 avg_ns=526 max_ns=11574
  fuse_op.statfs: count=2 total_ns=3513 avg_ns=1756 max_ns=2234
  fuse_op.unlink: count=8 total_ns=2545504 avg_ns=318188 max_ns=390200
  fuse_op.write: count=1024 total_ns=39193006 avg_ns=38274 max_ns=65729
  policy_decision: count=232608 total_ns=378957359 avg_ns=1629 max_ns=145637
  matcher_candidates: count=1081696
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=1081696
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=1081696
  matcher_candidate_order.descendant: count=228472 total_ns=147727884 avg_ns=646 max_ns=34389
  matcher_candidate_order.path: count=693688 total_ns=402732825 avg_ns=580 max_ns=38340
  matcher_candidate_order_by_source.hidden.path: count=228472 total_ns=189989970 avg_ns=831 max_ns=38340
  matcher_candidate_order_by_source.internal_hidden.path: count=228472 total_ns=38744403 avg_ns=169 max_ns=12007
  matcher_candidate_order_by_source.readonly.path: count=4136 total_ns=4061877 avg_ns=982 max_ns=5298
  matcher_candidate_order_by_source.visible.descendant: count=228472 total_ns=147727884 avg_ns=646 max_ns=34389
  matcher_candidate_order_by_source.visible.path: count=228472 total_ns=169214425 avg_ns=740 max_ns=20363
  matcher_candidate_order_by_source.writable.path: count=4136 total_ns=722150 avg_ns=174 max_ns=513
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=22526744
  matcher_candidate_order_seen_slots.descendant: count=7311104
  matcher_candidate_order_seen_slots.path: count=15215640
  matcher_candidate_order_ancestor_steps: count=3282668
  matcher_candidate_order_ancestor_steps.descendant: count=812411
  matcher_candidate_order_ancestor_steps.path: count=2470257
  state_read_lock_wait: count=172575 total_ns=4157214 avg_ns=24 max_ns=1346
  state_read_lock_hold: count=172575 total_ns=13107098 avg_ns=75 max_ns=8420
  state_write_lock_wait: count=168987 total_ns=4049545 avg_ns=23 max_ns=5253
  state_write_lock_hold: count=168987 total_ns=70107916 avg_ns=414 max_ns=318154
  open_confined_openat2: count=206944 total_ns=185507199 avg_ns=896 max_ns=36231
  open_like.pre_open_guard.access: count=1 total_ns=48723 avg_ns=48723 max_ns=48723
  open_like.pre_open_guard.open: count=33784 total_ns=465248809 avg_ns=13771 max_ns=62748
  open_like.post_open_revalidation.access: count=1 total_ns=7238 avg_ns=7238 max_ns=7238
  open_like.post_open_revalidation.open: count=33784 total_ns=236447876 avg_ns=6998 max_ns=22082
  stat_child_no_follow: count=171103 total_ns=263739957 avg_ns=1541 max_ns=38263
  stat_child_no_follow.attr_conversion: count=171061 total_ns=2960435 avg_ns=17 max_ns=298
  stat_child_no_follow.host_fstat: count=171061 total_ns=40174982 avg_ns=234 max_ns=9445
  stat_child_no_follow_context.path_guard_or_metadata: count=171103 total_ns=263739957 avg_ns=1541 max_ns=38263
  source_root_path: count=173111 total_ns=324931991 avg_ns=1877 max_ns=40433
  resolved_virtual_path: count=224326 total_ns=676540291 avg_ns=3015 max_ns=91010
  resolved_virtual_path_from_path: count=171069 total_ns=609236157 avg_ns=3561 max_ns=44457
  resolved_virtual_path_from_path_component_walk: count=171069 total_ns=532844945 avg_ns=3114 max_ns=43357
  resolved_virtual_path_from_path_canonicalize: count=411753 total_ns=433212497 avg_ns=1052 max_ns=42889
  resolved_virtual_path_from_path_source_root_confinement: count=411753 total_ns=52736090 avg_ns=128 max_ns=9323
  resolved_virtual_path_from_path_virtual_conversion: count=171069 total_ns=65981712 avg_ns=385 max_ns=11084
  resolved_virtual_path_from_open_fd: count=53257 total_ns=67304134 avg_ns=1263 max_ns=91010
  read_handle_snapshot: count=16384 total_ns=3832311 avg_ns=233 max_ns=9280
  read_guard_path: count=16384 total_ns=331528240 avg_ns=20234 max_ns=170191
  read_io: count=16384 total_ns=16632298 avg_ns=1015 max_ns=22913
  write_handle_snapshot: count=1024 total_ns=203480 avg_ns=198 max_ns=806
  write_guard_mutation: count=1024 total_ns=37722576 avg_ns=36838 max_ns=58759
  write_io: count=1024 total_ns=1091385 avg_ns=1065 max_ns=9268
  file_sync.flush: count=1024 total_ns=4748085 avg_ns=4636 max_ns=165572
  file_sync.fsync: count=512 total_ns=2116954 avg_ns=4134 max_ns=18338
  read_size_bucket.0_4k: count=16384 total_ns=16632298 avg_ns=1015 max_ns=22913
  write_size_bucket.0_4k: count=1024 total_ns=1091385 avg_ns=1065 max_ns=9268
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
  invalidations: count=16 invalidated_entries=8 evicted_entries=0 scanned_entries=16056
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
