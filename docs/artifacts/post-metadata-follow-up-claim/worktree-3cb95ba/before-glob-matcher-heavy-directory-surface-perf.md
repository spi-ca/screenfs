# ScreenFS benchmark result

- timestamp: `2026-06-15T22:10:03.595743+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-postmeta-target-perf/release/screenfs --screenfs-source-root /tmp/screenfs-before-postmeta-3cb95ba --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --workload-set directory-surface --iterations 10 --warmups 3 --perf-counters --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-glob-matcher-heavy-directory-surface-perf.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-glob-matcher-heavy-directory-surface-perf.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-glob-matcher-heavy-directory-surface-perf.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-env.json;  M docs/artifacts/managed-fio-attribution-native.json; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-postmeta-target-perf/release/screenfs`
- screenfs_bin_sha256: `9c998fe7a1356552b25afbfdd6be82fd2bf07d2a37ccae50c731d32fa9a777dd`
- screenfs_source_root: `/tmp/screenfs-before-postmeta-3cb95ba`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001173 | 0.082093 | 69.982 | 0.082575 | 0.082739 | 0.082870 |
| readdirplus_basic | 0.007866 | 0.727642 | 92.508 | 0.741923 | 0.743101 | 0.744043 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=63730 avg_ns=63730 max_ns=63730
  fuse_op.getattr: count=65027 total_ns=1889547727 avg_ns=29057 max_ns=237098
  fuse_op.lookup: count=195057 total_ns=4358971417 avg_ns=22347 max_ns=387694
  fuse_op.opendir: count=26 total_ns=881938 avg_ns=33920 max_ns=49494
  fuse_op.readdir: count=180 total_ns=1633092356 avg_ns=9072735 max_ns=18311613
  fuse_op.readdirplus: count=31 total_ns=464238859 avg_ns=14975447 max_ns=17173561
  fuse_op.releasedir: count=26 total_ns=8565701 avg_ns=329450 max_ns=477824
  fuse_op.statfs: count=2 total_ns=3186 avg_ns=1593 max_ns=1633
  policy_decision: count=1088251 total_ns=704199608 avg_ns=647 max_ns=198936
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1088251 total_ns=165112447 avg_ns=151 max_ns=67097
  matcher_candidate_order.path: count=3264753 total_ns=780608128 avg_ns=239 max_ns=180362
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=35912283
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=35912283
  matcher_candidate_order_ancestor_steps: count=14808676
  matcher_candidate_order_ancestor_steps.descendant: count=3702169
  matcher_candidate_order_ancestor_steps.path: count=11106507
  state_read_lock_wait: count=260322 total_ns=6601394 avg_ns=25 max_ns=681
  state_read_lock_hold: count=260322 total_ns=31665609 avg_ns=121 max_ns=356254
  state_write_lock_wait: count=195344 total_ns=5026311 avg_ns=25 max_ns=5290
  state_write_lock_hold: count=195344 total_ns=290677235 avg_ns=1488 max_ns=2783043
  open_confined_openat2: count=260401 total_ns=270310986 avg_ns=1038 max_ns=22913
  stat_child_no_follow: count=260163 total_ns=3562503413 avg_ns=13693 max_ns=218904
  source_root_path: count=260400 total_ns=938788346 avg_ns=3605 max_ns=202342
  resolved_virtual_path: count=780723 total_ns=3360427168 avg_ns=4304 max_ns=202794
  resolved_virtual_path_from_path: count=520323 total_ns=2855148394 avg_ns=5487 max_ns=202794
  resolved_virtual_path_from_path_component_walk: count=520323 total_ns=2639707677 avg_ns=5073 max_ns=202313
  resolved_virtual_path_from_path_canonicalize: count=910378 total_ns=2393222617 avg_ns=2628 max_ns=201793
  resolved_virtual_path_from_path_source_root_confinement: count=910378 total_ns=133990687 avg_ns=147 max_ns=19446
  resolved_virtual_path_from_path_virtual_conversion: count=520323 total_ns=177201286 avg_ns=340 max_ns=15139
  resolved_virtual_path_from_open_fd: count=260400 total_ns=505278774 avg_ns=1940 max_ns=181734
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1475848827 avg_ns=8199160 max_ns=16220788
  readdir_attr_generation_scan: count=180 total_ns=395722581 avg_ns=2198458 max_ns=4541363
  readdir_attr_generation_entries: count=427820
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=70778453 avg_ns=393213 max_ns=957881
  readdir_page_commit: count=180 total_ns=140169060 avg_ns=778717 max_ns=2783163
  readdirplus_directory_scan: count=31 total_ns=457180468 avg_ns=14747757 max_ns=16913238
  readdirplus_attr_generation_scan: count=31 total_ns=126184156 avg_ns=4070456 max_ns=4878871
  readdirplus_attr_generation_entries: count=139868
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=23417338 avg_ns=755398 max_ns=943771
  readdirplus_page_commit: count=31 total_ns=5064312 avg_ns=163364 max_ns=304777
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
