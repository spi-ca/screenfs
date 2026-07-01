# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:20.800901+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --cache-control warm --workload readdirplus_basic --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 20000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-readdirplus-page-scan-attribution-smoke/fast-readdirplus-20k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+42 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+42 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `readdirplus_basic`
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
| readdirplus_basic | 0.015843 | 0.968130 | 61.108 | 0.974863 | 0.975705 | 0.976378 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=44129 avg_ns=44129 max_ns=44129
  fuse_op.getattr: count=80005 total_ns=271188570 avg_ns=3389 max_ns=25079
  fuse_op.lookup: count=240013 total_ns=1034399922 avg_ns=4309 max_ns=279033
  fuse_op.opendir: count=4 total_ns=106524 avg_ns=26631 max_ns=30153
  fuse_op.readdir: count=98 total_ns=812862557 avg_ns=8294515 max_ns=11303679
  fuse_op.readdirplus: count=24 total_ns=153569657 avg_ns=6398735 max_ns=9712503
  fuse_op.releasedir: count=4 total_ns=9165517 avg_ns=2291379 max_ns=2587683
  fuse_op.statfs: count=2 total_ns=6010 avg_ns=3005 max_ns=4103
  policy_decision: count=320027 total_ns=125924404 avg_ns=393 max_ns=16804
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=320027 total_ns=42964777 avg_ns=134 max_ns=18720
  matcher_candidate_order.path: count=960081 total_ns=139506871 avg_ns=145 max_ns=21772
  matcher_candidate_order_by_source.hidden.path: count=320027 total_ns=46045818 avg_ns=143 max_ns=9881
  matcher_candidate_order_by_source.internal_hidden.path: count=320027 total_ns=47772350 avg_ns=149 max_ns=8958
  matcher_candidate_order_by_source.visible.descendant: count=320027 total_ns=42964777 avg_ns=134 max_ns=18720
  matcher_candidate_order_by_source.visible.path: count=320027 total_ns=45688703 avg_ns=142 max_ns=21772
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=4160280
  matcher_candidate_order_ancestor_steps.descendant: count=1040070
  matcher_candidate_order_ancestor_steps.path: count=3120210
  state_read_lock_wait: count=320145 total_ns=7375610 avg_ns=23 max_ns=12012
  state_read_lock_hold: count=320145 total_ns=22413504 avg_ns=70 max_ns=9418
  state_write_lock_wait: count=240145 total_ns=5259872 avg_ns=21 max_ns=3076
  state_write_lock_hold: count=240145 total_ns=307382846 avg_ns=1279 max_ns=3002022
  open_confined_openat2: count=324653 total_ns=241309633 avg_ns=743 max_ns=70594
  open_like.pre_open_guard.access: count=1 total_ns=294 avg_ns=294 max_ns=294
  open_like.pre_open_guard.opendir: count=4 total_ns=1270 avg_ns=317 max_ns=447
  open_like.post_open_revalidation.access: count=1 total_ns=33458 avg_ns=33458 max_ns=33458
  open_like.post_open_revalidation.opendir: count=4 total_ns=82841 avg_ns=20710 max_ns=24342
  stat_child_no_follow: count=324526 total_ns=428699716 avg_ns=1321 max_ns=275175
  stat_child_no_follow.attr_conversion: count=324524 total_ns=5398451 avg_ns=16 max_ns=7493
  stat_child_no_follow.host_fstat: count=324524 total_ns=73157738 avg_ns=225 max_ns=14614
  stat_child_no_follow_context.path_guard_or_metadata: count=324526 total_ns=428699716 avg_ns=1321 max_ns=275175
  source_root_path: count=127 total_ns=3082622 avg_ns=24272 max_ns=67872
  resolved_virtual_path: count=127 total_ns=509964 avg_ns=4015 max_ns=7313
  resolved_virtual_path_from_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_component_walk: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_canonicalize: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_source_root_confinement: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_path_virtual_conversion: count=0 total_ns=0 avg_ns=0 max_ns=0
  resolved_virtual_path_from_open_fd: count=127 total_ns=509964 avg_ns=4015 max_ns=7313
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=98 total_ns=661169072 avg_ns=6746623 max_ns=10162085
  readdir_scan.name_child_path_materialization: count=98 total_ns=21027762 avg_ns=214569 max_ns=420773
  readdir_scan.returned_child_path_materialization: count=75614 total_ns=16861119 avg_ns=222 max_ns=10702
  readdir_scan.scan_fallback_attr: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=98 total_ns=16187249 avg_ns=165176 max_ns=359977
  readdir_attr_generation_scan: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=98 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=98 total_ns=128377558 avg_ns=1309975 max_ns=2710935
  readdir_page_commit: count=98 total_ns=117697473 avg_ns=1200994 max_ns=3002386
  readdirplus_directory_scan: count=24 total_ns=130382658 avg_ns=5432610 max_ns=9137749
  readdirplus_scan.name_child_path_materialization: count=24 total_ns=4361354 avg_ns=181723 max_ns=415442
  readdirplus_scan.returned_attr_hydration: count=4500 total_ns=5754011 avg_ns=1278 max_ns=19906
  readdirplus_scan.returned_child_path_materialization: count=4500 total_ns=1207651 avg_ns=268 max_ns=5273
  readdirplus_scan.returned_policy_recheck: count=4500 total_ns=73093 avg_ns=16 max_ns=213
  readdirplus_scan.scan_fallback_attr: count=24 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=24 total_ns=3777346 avg_ns=157389 max_ns=362945
  readdirplus_attr_generation_scan: count=4524 total_ns=5754011 avg_ns=1271 max_ns=19906
  readdirplus_attr_generation_entries: count=4500
  readdirplus_symlink_visibility: count=24 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=24 total_ns=26410494 avg_ns=1100437 max_ns=2487207
  readdirplus_page_commit: count=24 total_ns=14198914 avg_ns=591621 max_ns=2595061
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
