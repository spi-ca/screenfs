# ScreenFS benchmark result

- timestamp: `2026-07-01T05:12:45.557375+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --iterations 1 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 200 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 20 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-set-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-set-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-mutation-invalidation-set-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+14 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+14 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `mutation-invalidation`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `symlink_parent_mkdir_rmdir, pinned_symlink_parent_mkdir_rmdir, subtree_rename_cached_unrelated`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `1`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| symlink_parent_mkdir_rmdir | 0.014126 | 0.014126 | 0.014126 | 0.014126 |
| pinned_symlink_parent_mkdir_rmdir | 0.019692 | 0.019692 | 0.019692 | 0.019692 |
| subtree_rename_cached_unrelated | 0.013536 | 0.013536 | 0.013536 | 0.013536 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=54858 avg_ns=54858 max_ns=54858
  fuse_op.getattr: count=581 total_ns=6038161 avg_ns=10392 max_ns=28609
  fuse_op.lookup: count=4280 total_ns=40436436 avg_ns=9447 max_ns=60905
  fuse_op.mkdir: count=80 total_ns=5215018 avg_ns=65187 max_ns=115896
  fuse_op.opendir: count=42 total_ns=515807 avg_ns=12281 max_ns=18819
  fuse_op.readdirplus: count=42 total_ns=1480650 avg_ns=35253 max_ns=107816
  fuse_op.readlink: count=368 total_ns=8010185 avg_ns=21766 max_ns=58092
  fuse_op.releasedir: count=42 total_ns=28803 avg_ns=685 max_ns=2528
  fuse_op.rename: count=4 total_ns=444279 avg_ns=111069 max_ns=122328
  fuse_op.rmdir: count=80 total_ns=3943087 avg_ns=49288 max_ns=65689
  fuse_op.statfs: count=2 total_ns=3981 avg_ns=1990 max_ns=2579
  policy_decision: count=8589 total_ns=5133157 avg_ns=597 max_ns=7175
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
  matcher_candidate_order.descendant: count=8085 total_ns=1304567 avg_ns=161 max_ns=1175
  matcher_candidate_order.path: count=25263 total_ns=5358500 avg_ns=212 max_ns=18103
  matcher_candidate_order_by_source.hidden.path: count=8085 total_ns=2336726 avg_ns=289 max_ns=18103
  matcher_candidate_order_by_source.internal_hidden.path: count=8085 total_ns=1427774 avg_ns=176 max_ns=3902
  matcher_candidate_order_by_source.readonly.path: count=504 total_ns=161099 avg_ns=319 max_ns=974
  matcher_candidate_order_by_source.visible.descendant: count=8085 total_ns=1304567 avg_ns=161 max_ns=1175
  matcher_candidate_order_by_source.visible.path: count=8085 total_ns=1343440 avg_ns=166 max_ns=1803
  matcher_candidate_order_by_source.writable.path: count=504 total_ns=89461 avg_ns=177 max_ns=410
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=8589
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=8589
  matcher_candidate_order_ancestor_steps: count=128020
  matcher_candidate_order_ancestor_steps.descendant: count=30925
  matcher_candidate_order_ancestor_steps.path: count=97095
  state_read_lock_wait: count=5482 total_ns=114344 avg_ns=20 max_ns=380
  state_read_lock_hold: count=5482 total_ns=353477 avg_ns=64 max_ns=3809
  state_write_lock_wait: count=4362 total_ns=89134 avg_ns=20 max_ns=328
  state_write_lock_hold: count=4362 total_ns=1915067 avg_ns=439 max_ns=44680
  open_confined_openat2: count=6443 total_ns=4502467 avg_ns=698 max_ns=15970
  open_like.pre_open_guard.access: count=1 total_ns=45181 avg_ns=45181 max_ns=45181
  open_like.pre_open_guard.opendir: count=42 total_ns=396495 avg_ns=9440 max_ns=13779
  open_like.post_open_revalidation.access: count=1 total_ns=5106 avg_ns=5106 max_ns=5106
  open_like.post_open_revalidation.opendir: count=42 total_ns=56218 avg_ns=1338 max_ns=1964
  stat_child_no_follow: count=6190 total_ns=10342562 avg_ns=1670 max_ns=30486
  stat_child_no_follow.attr_conversion: count=5608 total_ns=85091 avg_ns=15 max_ns=50
  stat_child_no_follow.directory_revalidation: count=368 total_ns=2859442 avg_ns=7770 max_ns=23166
  stat_child_no_follow.host_fstat: count=5240 total_ns=901677 avg_ns=172 max_ns=2388
  stat_child_no_follow.host_fstatat: count=368 total_ns=142337 avg_ns=386 max_ns=1976
  stat_child_no_follow.parent_open: count=368 total_ns=371212 avg_ns=1008 max_ns=5308
  stat_child_no_follow_context.path_guard_or_metadata: count=5822 total_ns=6874730 avg_ns=1180 max_ns=17182
  stat_child_no_follow_context.readlink_pre_open: count=368 total_ns=3467832 avg_ns=9423 max_ns=30486
  source_root_path: count=5810 total_ns=8892627 avg_ns=1530 max_ns=23137
  resolved_virtual_path: count=6599 total_ns=22526516 avg_ns=3413 max_ns=15343
  resolved_virtual_path_from_path: count=5978 total_ns=21794697 avg_ns=3645 max_ns=15343
  resolved_virtual_path_from_path_component_walk: count=5978 total_ns=19204686 avg_ns=3212 max_ns=14872
  resolved_virtual_path_from_path_canonicalize: count=15734 total_ns=15709604 avg_ns=998 max_ns=14034
  resolved_virtual_path_from_path_source_root_confinement: count=15734 total_ns=1830013 avg_ns=116 max_ns=8011
  resolved_virtual_path_from_path_virtual_conversion: count=5978 total_ns=2263399 avg_ns=378 max_ns=2472
  resolved_virtual_path_from_open_fd: count=621 total_ns=731819 avg_ns=1178 max_ns=11041
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
  readdirplus_directory_scan: count=42 total_ns=263017 avg_ns=6262 max_ns=43843
  readdirplus_scan.name_child_path_materialization: count=42 total_ns=35837 avg_ns=853 max_ns=2692
  readdirplus_scan.returned_attr_hydration: count=82 total_ns=94834 avg_ns=1156 max_ns=2007
  readdirplus_scan.returned_policy_recheck: count=82 total_ns=2416 avg_ns=29 max_ns=295
  readdirplus_scan.scan_fallback_attr: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=42 total_ns=3524 avg_ns=83 max_ns=907
  readdirplus_attr_generation_scan: count=124 total_ns=94834 avg_ns=764 max_ns=2007
  readdirplus_attr_generation_entries: count=82
  readdirplus_symlink_visibility: count=42 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=42 total_ns=26482 avg_ns=630 max_ns=3790
  readdirplus_page_commit: count=42 total_ns=71308 avg_ns=1697 max_ns=4678
  invalidations: count=164 invalidated_entries=88 evicted_entries=0 scanned_entries=2368
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
