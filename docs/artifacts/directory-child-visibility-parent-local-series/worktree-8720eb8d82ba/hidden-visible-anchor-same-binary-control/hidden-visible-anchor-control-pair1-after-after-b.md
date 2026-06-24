# ScreenFS benchmark result

- timestamp: `2026-06-24T22:18:24.905548+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local --screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries --output-json docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-after-after-b.json --output-md docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-after-after-b.md --output-svg docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-after-after-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+42 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- screenfs_bin_sha256: `84b20a30b24c0da8ff470158a100e6124373c07d54ce8d584558e17e17a99605`
- screenfs_source_root: `/tmp/screenfs-parent-local-8720eb8d82ba/after`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `unknown`
- screenfs_source_git_worktree_clean: `unknown`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `parent-local-hidden-visible-anchor-control`
- fast_path_cache_eligible: `False`
- policy_notes: `extra_screenfs_arg included policy-shaping flags (--visibility-default, --visible, --visible); treat the run as a named custom unsafe matrix entry unless independently reviewed`
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
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001036 | 0.048555 | 46.875 | 0.050955 | 0.052041 | 0.052910 |
| readdirplus_basic | 0.004035 | 0.305954 | 75.828 | 0.366600 | 0.378943 | 0.388818 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73206 avg_ns=73206 max_ns=73206
  fuse_op.getattr: count=65027 total_ns=587736053 avg_ns=9038 max_ns=336585
  fuse_op.lookup: count=195055 total_ns=1599104515 avg_ns=8198 max_ns=653730
  fuse_op.opendir: count=26 total_ns=477553 avg_ns=18367 max_ns=69076
  fuse_op.readdir: count=180 total_ns=1215364514 avg_ns=6752025 max_ns=16324474
  fuse_op.readdirplus: count=31 total_ns=93298911 avg_ns=3009642 max_ns=4109689
  fuse_op.releasedir: count=26 total_ns=10657747 avg_ns=409913 max_ns=763355
  fuse_op.statfs: count=2 total_ns=9181 avg_ns=4590 max_ns=6264
  policy_decision: count=694047 total_ns=351898863 avg_ns=507 max_ns=332734
  matcher_candidates: count=824445
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=195454
  matcher_candidates_by_source.visible.path: count=628991
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=824445
  matcher_candidate_order.descendant: count=694047 total_ns=149626004 avg_ns=215 max_ns=289991
  matcher_candidate_order.path: count=2082141 total_ns=310184907 avg_ns=148 max_ns=319311
  matcher_candidate_order_by_source.hidden.path: count=694047 total_ns=72704825 avg_ns=104 max_ns=55805
  matcher_candidate_order_by_source.internal_hidden.path: count=694047 total_ns=79896918 avg_ns=115 max_ns=319311
  matcher_candidate_order_by_source.visible.descendant: count=694047 total_ns=149626004 avg_ns=215 max_ns=289991
  matcher_candidate_order_by_source.visible.path: count=694047 total_ns=157583164 avg_ns=227 max_ns=268074
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2776188
  matcher_candidate_order_seen_slots.descendant: count=1388094
  matcher_candidate_order_seen_slots.path: count=1388094
  matcher_candidate_order_ancestor_steps: count=10322936
  matcher_candidate_order_ancestor_steps.descendant: count=2580734
  matcher_candidate_order_ancestor_steps.path: count=7742202
  state_read_lock_wait: count=260320 total_ns=5182123 avg_ns=19 max_ns=1978
  state_read_lock_hold: count=260320 total_ns=16694496 avg_ns=64 max_ns=49489
  state_write_lock_wait: count=195344 total_ns=3673064 avg_ns=18 max_ns=1167
  state_write_lock_hold: count=195344 total_ns=236582544 avg_ns=1211 max_ns=2896220
  open_confined_openat2: count=266227 total_ns=173662439 avg_ns=652 max_ns=309314
  open_like.pre_open_guard.access: count=1 total_ns=62411 avg_ns=62411 max_ns=62411
  open_like.pre_open_guard.opendir: count=26 total_ns=324685 avg_ns=12487 max_ns=55359
  open_like.post_open_revalidation.access: count=1 total_ns=6814 avg_ns=6814 max_ns=6814
  open_like.post_open_revalidation.opendir: count=26 total_ns=97694 avg_ns=3757 max_ns=6586
  stat_child_no_follow: count=265989 total_ns=302712075 avg_ns=1138 max_ns=310249
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3878326 avg_ns=14 max_ns=4173
  stat_child_no_follow.host_fstat: count=265989 total_ns=46443322 avg_ns=174 max_ns=81349
  stat_child_no_follow_context.path_guard_or_metadata: count=265989 total_ns=302712075 avg_ns=1138 max_ns=310249
  source_root_path: count=260372 total_ns=365570770 avg_ns=1404 max_ns=306834
  resolved_virtual_path: count=260399 total_ns=762709433 avg_ns=2929 max_ns=332098
  resolved_virtual_path_from_path: count=260161 total_ns=762113929 avg_ns=2929 max_ns=332098
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=657616951 avg_ns=2527 max_ns=331601
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=542345070 avg_ns=926 max_ns=330930
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=57597318 avg_ns=98 max_ns=282612
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=90330093 avg_ns=347 max_ns=258537
  resolved_virtual_path_from_open_fd: count=238 total_ns=595504 avg_ns=2502 max_ns=14518
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1073961621 avg_ns=5966453 max_ns=15435246
  readdir_scan.name_child_path_materialization: count=180 total_ns=91378453 avg_ns=507658 max_ns=1332336
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=787173113 avg_ns=4373183 max_ns=11467470
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=53354048 avg_ns=296411 max_ns=1017578
  readdir_page_commit: count=180 total_ns=119722129 avg_ns=665122 max_ns=2896834
  readdirplus_directory_scan: count=31 total_ns=69413567 avg_ns=2239147 max_ns=3177457
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=26923069 avg_ns=868486 max_ns=1185592
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=6326864 avg_ns=1085 max_ns=35775
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=11283042 avg_ns=1936 max_ns=17376
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=1980857 avg_ns=63898 max_ns=110796
  readdirplus_attr_generation_scan: count=5859 total_ns=6326864 avg_ns=1079 max_ns=35775
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16276093 avg_ns=525035 max_ns=739369
  readdirplus_page_commit: count=31 total_ns=4446250 avg_ns=143427 max_ns=278842
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
