# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:37.150902+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-matcher32 --cache-control warm --workload matcher_readonly_access_wok --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 200 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `matcher_readonly_access_wok`
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

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_readonly_access_wok | 0.022529 | 0.022846 | 0.022885 | 0.022917 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=20447938 avg_ns=25528 max_ns=50141
  fuse_op.getattr: count=129 total_ns=2131562 avg_ns=16523 max_ns=23640
  fuse_op.lookup: count=3717 total_ns=45167237 avg_ns=12151 max_ns=67687
  fuse_op.statfs: count=2 total_ns=6847 avg_ns=3423 max_ns=4101
  policy_decision: count=6248 total_ns=10235074 avg_ns=1638 max_ns=4232
  matcher_candidates: count=60288
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=800
  matcher_candidates_by_source.visible.descendant: count=59488
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=60288
  matcher_candidate_order.descendant: count=5448 total_ns=3710745 avg_ns=681 max_ns=21378
  matcher_candidate_order.path: count=17944 total_ns=10733051 avg_ns=598 max_ns=9120
  matcher_candidate_order_by_source.hidden.path: count=5448 total_ns=4617623 avg_ns=847 max_ns=3340
  matcher_candidate_order_by_source.internal_hidden.path: count=5448 total_ns=1068922 avg_ns=196 max_ns=2974
  matcher_candidate_order_by_source.readonly.path: count=800 total_ns=804959 avg_ns=1006 max_ns=2455
  matcher_candidate_order_by_source.visible.descendant: count=5448 total_ns=3710745 avg_ns=681 max_ns=21378
  matcher_candidate_order_by_source.visible.path: count=5448 total_ns=4062102 avg_ns=745 max_ns=9120
  matcher_candidate_order_by_source.writable.path: count=800 total_ns=179445 avg_ns=224 max_ns=558
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=560304
  matcher_candidate_order_seen_slots.descendant: count=174336
  matcher_candidate_order_seen_slots.path: count=385968
  matcher_candidate_order_ancestor_steps: count=94604
  matcher_candidate_order_ancestor_steps.descendant: count=21651
  matcher_candidate_order_ancestor_steps.path: count=72953
  state_read_lock_wait: count=4647 total_ns=109739 avg_ns=23 max_ns=173
  state_read_lock_hold: count=4647 total_ns=291751 avg_ns=62 max_ns=964
  state_write_lock_wait: count=3715 total_ns=86975 avg_ns=23 max_ns=254
  state_write_lock_hold: count=3715 total_ns=1352017 avg_ns=363 max_ns=5001
  open_confined_openat2: count=4648 total_ns=3356376 avg_ns=722 max_ns=12780
  open_like.pre_open_guard.access: count=801 total_ns=20249000 avg_ns=25279 max_ns=40441
  open_like.post_open_revalidation.access: count=1 total_ns=5535 avg_ns=5535 max_ns=5535
  stat_child_no_follow: count=4647 total_ns=6061892 avg_ns=1304 max_ns=16337
  stat_child_no_follow.attr_conversion: count=4645 total_ns=81592 avg_ns=17 max_ns=20
  stat_child_no_follow.host_fstat: count=4645 total_ns=921413 avg_ns=198 max_ns=4419
  stat_child_no_follow_context.path_guard_or_metadata: count=4647 total_ns=6061892 avg_ns=1304 max_ns=16337
  source_root_path: count=4647 total_ns=8316035 avg_ns=1789 max_ns=21688
  resolved_virtual_path: count=4646 total_ns=19837573 avg_ns=4269 max_ns=23361
  resolved_virtual_path_from_path: count=4645 total_ns=19834741 avg_ns=4270 max_ns=23361
  resolved_virtual_path_from_path_component_walk: count=4645 total_ns=17759491 avg_ns=3823 max_ns=22727
  resolved_virtual_path_from_path_canonicalize: count=13001 total_ns=14769582 avg_ns=1136 max_ns=21671
  resolved_virtual_path_from_path_source_root_confinement: count=13001 total_ns=1656034 avg_ns=127 max_ns=1104
  resolved_virtual_path_from_path_virtual_conversion: count=4645 total_ns=1800176 avg_ns=387 max_ns=7547
  resolved_virtual_path_from_open_fd: count=1 total_ns=2832 avg_ns=2832 max_ns=2832
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
