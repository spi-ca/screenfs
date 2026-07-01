# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:25.680452+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher32 --workload-set policy-heavy-matrix --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 64 --small-files 1024 --dir-entries 2048 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 1024 --sync-4k-fsync-every 32 --hidden-misses 512 --matcher-extra-rules 32 --matcher-misses 512 --symlink-parent-mutations 64 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+45 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+45 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss, matcher_readonly_access_wok`
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
| metadata_lookup | 0.006165 | 0.058260 | 9.451 | 0.060778 | 0.060902 | 0.061001 |
| metadata_getattr | 0.006382 | 0.080451 | 12.605 | 0.081312 | 0.081601 | 0.081832 |
| metadata_access | 0.005848 | 0.086535 | 14.797 | 0.087318 | 0.087450 | 0.087555 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.019696 | 0.020145 | 0.020209 | 0.020261 |
| matcher_readonly_access_wok | 0.043170 | 0.043502 | 0.043615 | 0.043705 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=19969 total_ns=412005453 avg_ns=20632 max_ns=172430
  fuse_op.getattr: count=13729 total_ns=185447349 avg_ns=13507 max_ns=77471
  fuse_op.lookup: count=168069 total_ns=1746272493 avg_ns=10390 max_ns=270801
  fuse_op.statfs: count=2 total_ns=5509 avg_ns=2754 max_ns=3435
  policy_decision: count=228392 total_ns=303771836 avg_ns=1330 max_ns=91264
  matcher_candidates: count=2169952
  matcher_candidates_by_source.hidden.path: count=6656
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=6656
  matcher_candidates_by_source.visible.descendant: count=2156640
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2169952
  matcher_candidate_order.descendant: count=221736 total_ns=132157513 avg_ns=596 max_ns=88330
  matcher_candidate_order.path: count=678520 total_ns=322236393 avg_ns=474 max_ns=140087
  matcher_candidate_order_by_source.hidden.path: count=221736 total_ns=147431989 avg_ns=664 max_ns=49371
  matcher_candidate_order_by_source.internal_hidden.path: count=221736 total_ns=34833925 avg_ns=157 max_ns=14647
  matcher_candidate_order_by_source.readonly.path: count=6656 total_ns=6124693 avg_ns=920 max_ns=140087
  matcher_candidate_order_by_source.visible.descendant: count=221736 total_ns=132157513 avg_ns=596 max_ns=88330
  matcher_candidate_order_by_source.visible.path: count=221736 total_ns=132587785 avg_ns=597 max_ns=17632
  matcher_candidate_order_by_source.writable.path: count=6656 total_ns=1258001 avg_ns=189 max_ns=8700
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=21949776
  matcher_candidate_order_seen_slots.descendant: count=7095552
  matcher_candidate_order_seen_slots.path: count=14854224
  matcher_candidate_order_ancestor_steps: count=3053516
  matcher_candidate_order_ancestor_steps.descendant: count=746739
  matcher_candidate_order_ancestor_steps.path: count=2306777
  state_read_lock_wait: count=201767 total_ns=4443588 avg_ns=22 max_ns=2375
  state_read_lock_hold: count=201767 total_ns=11925544 avg_ns=59 max_ns=47625
  state_write_lock_wait: count=148099 total_ns=3150819 avg_ns=21 max_ns=7350
  state_write_lock_hold: count=148099 total_ns=62537224 avg_ns=422 max_ns=63363
  open_confined_openat2: count=215080 total_ns=167713591 avg_ns=779 max_ns=47379
  open_like.pre_open_guard.access: count=19969 total_ns=307965721 avg_ns=15422 max_ns=172135
  open_like.post_open_revalidation.access: count=13313 total_ns=85236591 avg_ns=6402 max_ns=67951
  stat_child_no_follow: count=201767 total_ns=266364467 avg_ns=1320 max_ns=47738
  stat_child_no_follow.attr_conversion: count=188453 total_ns=3017917 avg_ns=16 max_ns=5376
  stat_child_no_follow.host_fstat: count=188453 total_ns=37961447 avg_ns=201 max_ns=15492
  stat_child_no_follow_context.path_guard_or_metadata: count=201767 total_ns=266364467 avg_ns=1320 max_ns=47738
  source_root_path: count=195111 total_ns=320985737 avg_ns=1645 max_ns=74495
  resolved_virtual_path: count=195110 total_ns=565728819 avg_ns=2899 max_ns=262509
  resolved_virtual_path_from_path: count=181797 total_ns=550688553 avg_ns=3029 max_ns=262509
  resolved_virtual_path_from_path_component_walk: count=181797 total_ns=478410124 avg_ns=2631 max_ns=262045
  resolved_virtual_path_from_path_canonicalize: count=398537 total_ns=389676545 avg_ns=977 max_ns=261456
  resolved_virtual_path_from_path_source_root_confinement: count=398537 total_ns=48379955 avg_ns=121 max_ns=18897
  resolved_virtual_path_from_path_virtual_conversion: count=181797 total_ns=61941687 avg_ns=340 max_ns=18283
  resolved_virtual_path_from_open_fd: count=13313 total_ns=15040266 avg_ns=1129 max_ns=63557
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
