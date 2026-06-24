# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:36.261657+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-before-pair2 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair2.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair2.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair2.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+19 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+19 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-descendant-same-binary-control-before-pair2`
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
| matcher_descendant_readdir | 0.000027 | 0.000604 | 22.430 | 0.000676 | 0.000695 | 0.000710 |
| matcher_descendant_readdirplus | 0.000064 | 0.003516 | 55.251 | 0.003651 | 0.003707 | 0.003752 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=84484 avg_ns=84484 max_ns=84484
  fuse_op.getattr: count=443 total_ns=7087021 avg_ns=15997 max_ns=52970
  fuse_op.lookup: count=1747 total_ns=21001763 avg_ns=12021 max_ns=76263
  fuse_op.opendir: count=26 total_ns=533520 avg_ns=20520 max_ns=30567
  fuse_op.readdir: count=26 total_ns=568656 avg_ns=21871 max_ns=104309
  fuse_op.readdirplus: count=26 total_ns=12067126 avg_ns=464120 max_ns=651115
  fuse_op.releasedir: count=26 total_ns=79762 avg_ns=3067 max_ns=6718
  fuse_op.statfs: count=2 total_ns=8513 avg_ns=4256 max_ns=4660
  policy_decision: count=4012 total_ns=7788344 avg_ns=1941 max_ns=8121
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2084546 avg_ns=519 max_ns=2523
  matcher_candidate_order.path: count=12036 total_ns=5702470 avg_ns=473 max_ns=6418
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2355678 avg_ns=587 max_ns=2759
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=742088 avg_ns=184 max_ns=6418
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2084546 avg_ns=519 max_ns=2523
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2604704 avg_ns=649 max_ns=1998
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=62730 avg_ns=27 max_ns=390
  state_read_lock_hold: count=2269 total_ns=153870 avg_ns=67 max_ns=2239
  state_write_lock_wait: count=1875 total_ns=45882 avg_ns=24 max_ns=329
  state_write_lock_hold: count=1875 total_ns=1044734 avg_ns=557 max_ns=43107
  open_confined_openat2: count=3180 total_ns=2461625 avg_ns=774 max_ns=11465
  open_like.pre_open_guard.access: count=1 total_ns=65801 avg_ns=65801 max_ns=65801
  open_like.pre_open_guard.opendir: count=26 total_ns=337478 avg_ns=12979 max_ns=18249
  open_like.post_open_revalidation.access: count=1 total_ns=11539 avg_ns=11539 max_ns=11539
  open_like.post_open_revalidation.opendir: count=26 total_ns=145334 avg_ns=5589 max_ns=8610
  stat_child_no_follow: count=3101 total_ns=4169176 avg_ns=1344 max_ns=18370
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53870 avg_ns=17 max_ns=46
  stat_child_no_follow.host_fstat: count=3099 total_ns=667008 avg_ns=215 max_ns=2211
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=4169176 avg_ns=1344 max_ns=18370
  source_root_path: count=2321 total_ns=4484682 avg_ns=1932 max_ns=28693
  resolved_virtual_path: count=2346 total_ns=10082490 avg_ns=4297 max_ns=14945
  resolved_virtual_path_from_path: count=2267 total_ns=9961043 avg_ns=4393 max_ns=14945
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8880835 avg_ns=3917 max_ns=11779
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7437727 avg_ns=1185 max_ns=6473
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=682132 avg_ns=108 max_ns=1178
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=935855 avg_ns=412 max_ns=9001
  resolved_virtual_path_from_open_fd: count=79 total_ns=121447 avg_ns=1537 max_ns=6198
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=233640 avg_ns=8986 max_ns=76322
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=13399 avg_ns=515 max_ns=3810
  readdirplus_directory_scan: count=26 total_ns=5166211 avg_ns=198700 max_ns=322144
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=328895 avg_ns=12649 max_ns=29498
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1017358 avg_ns=1222 max_ns=3055
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4433088 avg_ns=5328 max_ns=9791
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4440249 avg_ns=170778 max_ns=199984
  readdirplus_attr_generation_scan: count=858 total_ns=1017358 avg_ns=1185 max_ns=3055
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=212694 avg_ns=8180 max_ns=95003
  readdirplus_page_commit: count=26 total_ns=456920 avg_ns=17573 max_ns=43258
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
