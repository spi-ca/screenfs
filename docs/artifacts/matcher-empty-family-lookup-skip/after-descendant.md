# ScreenFS benchmark result

- timestamp: `2026-06-24T05:21:39.271266+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy-path-only-after --workload-set matcher-descendant-directory --output-json /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant.json --output-md /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant.md --output-svg /tmp/screenfs-matcher-empty-family/formal/after-path-only-descendant.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-empty-family/formal/screenfs-after-path-only`
- screenfs_bin_sha256: `5d1d2cee33f097a723efd3196a68074f2af19b05c5026698fbecfc1789bf4ae7`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-path-only-after`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `matcher-descendant-directory`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
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
| matcher_descendant_readdir | 0.000030 | 0.000606 | 20.104 | 0.000672 | 0.000708 | 0.000737 |
| matcher_descendant_readdirplus | 0.000072 | 0.003615 | 50.410 | 0.004388 | 0.004415 | 0.004437 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=121394 avg_ns=121394 max_ns=121394
  fuse_op.getattr: count=443 total_ns=7302875 avg_ns=16485 max_ns=37777
  fuse_op.lookup: count=1747 total_ns=22019601 avg_ns=12604 max_ns=103377
  fuse_op.opendir: count=26 total_ns=524928 avg_ns=20189 max_ns=26400
  fuse_op.readdir: count=26 total_ns=470382 avg_ns=18091 max_ns=44329
  fuse_op.readdirplus: count=26 total_ns=12594609 avg_ns=484408 max_ns=757708
  fuse_op.releasedir: count=26 total_ns=77567 avg_ns=2983 max_ns=6340
  fuse_op.statfs: count=2 total_ns=5362 avg_ns=2681 max_ns=2957
  policy_decision: count=4012 total_ns=8143918 avg_ns=2029 max_ns=13439
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2248188 avg_ns=560 max_ns=18787
  matcher_candidate_order.path: count=12036 total_ns=5852562 avg_ns=486 max_ns=6226
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2385327 avg_ns=594 max_ns=4004
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=784607 avg_ns=195 max_ns=6226
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2248188 avg_ns=560 max_ns=18787
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2682628 avg_ns=668 max_ns=5173
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=56898 avg_ns=25 max_ns=366
  state_read_lock_hold: count=2269 total_ns=164925 avg_ns=72 max_ns=1558
  state_write_lock_wait: count=1875 total_ns=45443 avg_ns=24 max_ns=327
  state_write_lock_hold: count=1875 total_ns=1130518 avg_ns=602 max_ns=57249
  open_confined_openat2: count=3180 total_ns=2577137 avg_ns=810 max_ns=11998
  open_like.pre_open_guard.access: count=1 total_ns=98212 avg_ns=98212 max_ns=98212
  open_like.pre_open_guard.opendir: count=26 total_ns=333829 avg_ns=12839 max_ns=16175
  open_like.post_open_revalidation.access: count=1 total_ns=9915 avg_ns=9915 max_ns=9915
  open_like.post_open_revalidation.opendir: count=26 total_ns=149964 avg_ns=5767 max_ns=6990
  stat_child_no_follow: count=3101 total_ns=4381895 avg_ns=1413 max_ns=20417
  stat_child_no_follow.attr_conversion: count=3099 total_ns=54110 avg_ns=17 max_ns=248
  stat_child_no_follow.host_fstat: count=3099 total_ns=700232 avg_ns=225 max_ns=2970
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=4381895 avg_ns=1413 max_ns=20417
  source_root_path: count=2321 total_ns=4611155 avg_ns=1986 max_ns=30971
  resolved_virtual_path: count=2346 total_ns=10359399 avg_ns=4415 max_ns=24958
  resolved_virtual_path_from_path: count=2267 total_ns=10229383 avg_ns=4512 max_ns=24958
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=9078395 avg_ns=4004 max_ns=24090
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7556516 avg_ns=1204 max_ns=14896
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=686934 avg_ns=109 max_ns=1250
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=1011015 avg_ns=445 max_ns=5922
  resolved_virtual_path_from_open_fd: count=79 total_ns=130016 avg_ns=1645 max_ns=6600
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=154111 avg_ns=5927 max_ns=21367
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=9722 avg_ns=373 max_ns=1615
  readdirplus_directory_scan: count=26 total_ns=5301408 avg_ns=203900 max_ns=310759
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=313840 avg_ns=12070 max_ns=15365
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1039913 avg_ns=1249 max_ns=4871
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4676856 avg_ns=5621 max_ns=16911
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4604613 avg_ns=177100 max_ns=268765
  readdirplus_attr_generation_scan: count=858 total_ns=1039913 avg_ns=1212 max_ns=4871
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=184739 avg_ns=7105 max_ns=43582
  readdirplus_page_commit: count=26 total_ns=478563 avg_ns=18406 max_ns=57488
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
