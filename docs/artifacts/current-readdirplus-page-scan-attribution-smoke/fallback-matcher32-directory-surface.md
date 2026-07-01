# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:14.233557+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --cache-control warm --workload-set directory-surface --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fallback-matcher32-directory-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+36 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+36 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.001485 | 0.024993 | 16.825 | 0.025279 | 0.025315 | 0.025343 |
| readdirplus_basic | 0.006402 | 0.350553 | 54.756 | 0.355904 | 0.356573 | 0.357108 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61406 avg_ns=61406 max_ns=61406
  fuse_op.getattr: count=20009 total_ns=229345737 avg_ns=11462 max_ns=55380
  fuse_op.lookup: count=60021 total_ns=627357825 avg_ns=10452 max_ns=64040
  fuse_op.opendir: count=8 total_ns=143120 avg_ns=17890 max_ns=27029
  fuse_op.readdir: count=54 total_ns=181715180 avg_ns=3365095 max_ns=5112625
  fuse_op.readdirplus: count=13 total_ns=43913138 avg_ns=3377933 max_ns=3949775
  fuse_op.releasedir: count=8 total_ns=5422053 avg_ns=677756 max_ns=942433
  fuse_op.statfs: count=2 total_ns=5845 avg_ns=2922 max_ns=3856
  policy_decision: count=80056 total_ns=67090159 avg_ns=838 max_ns=19780
  matcher_candidates: count=640608
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=640608
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=640608
  matcher_candidate_order.descendant: count=80056 total_ns=30845262 avg_ns=385 max_ns=22260
  matcher_candidate_order.path: count=240168 total_ns=73124192 avg_ns=304 max_ns=36281
  matcher_candidate_order_by_source.hidden.path: count=80056 total_ns=30606101 avg_ns=382 max_ns=36281
  matcher_candidate_order_by_source.internal_hidden.path: count=80056 total_ns=12678880 avg_ns=158 max_ns=16047
  matcher_candidate_order_by_source.visible.descendant: count=80056 total_ns=30845262 avg_ns=385 max_ns=22260
  matcher_candidate_order_by_source.visible.path: count=80056 total_ns=29839211 avg_ns=372 max_ns=12717
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=7845488
  matcher_candidate_order_seen_slots.descendant: count=2561792
  matcher_candidate_order_seen_slots.path: count=5283696
  matcher_candidate_order_ancestor_steps: count=1040588
  matcher_candidate_order_ancestor_steps.descendant: count=260147
  matcher_candidate_order_ancestor_steps.path: count=780441
  state_read_lock_wait: count=80106 total_ns=1916398 avg_ns=23 max_ns=1817
  state_read_lock_hold: count=80106 total_ns=5515991 avg_ns=68 max_ns=10228
  state_write_lock_wait: count=60110 total_ns=1415554 avg_ns=23 max_ns=592
  state_write_lock_hold: count=60110 total_ns=92491118 avg_ns=1538 max_ns=2634002
  open_confined_openat2: count=82467 total_ns=70986287 avg_ns=860 max_ns=14115
  open_like.pre_open_guard.access: count=1 total_ns=48584 avg_ns=48584 max_ns=48584
  open_like.pre_open_guard.opendir: count=8 total_ns=100895 avg_ns=12611 max_ns=20268
  open_like.post_open_revalidation.access: count=1 total_ns=7723 avg_ns=7723 max_ns=7723
  open_like.post_open_revalidation.opendir: count=8 total_ns=22064 avg_ns=2758 max_ns=3911
  stat_child_no_follow: count=82391 total_ns=122470634 avg_ns=1486 max_ns=18135
  stat_child_no_follow.attr_conversion: count=82389 total_ns=1424232 avg_ns=17 max_ns=313
  stat_child_no_follow.host_fstat: count=82389 total_ns=20557367 avg_ns=249 max_ns=9081
  stat_child_no_follow_context.path_guard_or_metadata: count=82391 total_ns=122470634 avg_ns=1486 max_ns=18135
  source_root_path: count=80122 total_ns=142044604 avg_ns=1772 max_ns=34167
  resolved_virtual_path: count=80129 total_ns=274343119 avg_ns=3423 max_ns=40803
  resolved_virtual_path_from_path: count=80053 total_ns=274134066 avg_ns=3424 max_ns=40803
  resolved_virtual_path_from_path_component_walk: count=80053 total_ns=235432151 avg_ns=2940 max_ns=39347
  resolved_virtual_path_from_path_canonicalize: count=180089 total_ns=189115316 avg_ns=1050 max_ns=38092
  resolved_virtual_path_from_path_source_root_confinement: count=180089 total_ns=21193381 avg_ns=117 max_ns=11039
  resolved_virtual_path_from_path_virtual_conversion: count=80053 total_ns=33852982 avg_ns=422 max_ns=14053
  resolved_virtual_path_from_open_fd: count=76 total_ns=209053 avg_ns=2750 max_ns=10215
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=54 total_ns=127654085 avg_ns=2363964 max_ns=3870020
  readdir_scan.name_child_path_materialization: count=54 total_ns=32061399 avg_ns=593729 max_ns=1194621
  readdir_scan.scan_fallback_attr: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=54 total_ns=2386793 avg_ns=44199 max_ns=89415
  readdir_attr_generation_scan: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=54 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=54 total_ns=19762425 avg_ns=365970 max_ns=811451
  readdir_page_commit: count=54 total_ns=47171129 avg_ns=873539 max_ns=2634353
  readdirplus_directory_scan: count=13 total_ns=37256047 avg_ns=2865849 max_ns=3427325
  readdirplus_scan.name_child_path_materialization: count=13 total_ns=10933067 avg_ns=841005 max_ns=1181638
  readdirplus_scan.returned_attr_hydration: count=2336 total_ns=3332089 avg_ns=1426 max_ns=11616
  readdirplus_scan.returned_policy_recheck: count=2336 total_ns=42689 avg_ns=18 max_ns=395
  readdirplus_scan.scan_fallback_attr: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=13 total_ns=912496 avg_ns=70192 max_ns=92272
  readdirplus_attr_generation_scan: count=2349 total_ns=3332089 avg_ns=1418 max_ns=11616
  readdirplus_attr_generation_entries: count=2336
  readdirplus_symlink_visibility: count=13 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=13 total_ns=7289444 avg_ns=560726 max_ns=871841
  readdirplus_page_commit: count=13 total_ns=2342429 avg_ns=180186 max_ns=299568
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
