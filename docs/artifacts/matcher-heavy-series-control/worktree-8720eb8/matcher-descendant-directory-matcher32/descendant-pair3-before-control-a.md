# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:30.879708+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-before-control-a.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-series-before-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-series-control`
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
| matcher_descendant_readdir | 0.000042 | 0.000544 | 12.869 | 0.000711 | 0.000730 | 0.000746 |
| matcher_descendant_readdirplus | 0.000098 | 0.002918 | 29.864 | 0.003015 | 0.003106 | 0.003179 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=123052 avg_ns=123052 max_ns=123052
  fuse_op.getattr: count=443 total_ns=5863089 avg_ns=13234 max_ns=32448
  fuse_op.lookup: count=1747 total_ns=17056580 avg_ns=9763 max_ns=95688
  fuse_op.opendir: count=26 total_ns=463791 avg_ns=17838 max_ns=52193
  fuse_op.readdir: count=26 total_ns=456490 avg_ns=17557 max_ns=101232
  fuse_op.readdirplus: count=26 total_ns=10697020 avg_ns=411423 max_ns=568391
  fuse_op.releasedir: count=26 total_ns=101689 avg_ns=3911 max_ns=15024
  fuse_op.statfs: count=2 total_ns=5437 avg_ns=2718 max_ns=3998
  policy_decision: count=4012 total_ns=6902459 avg_ns=1720 max_ns=23627
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1900232 avg_ns=473 max_ns=9215
  matcher_candidate_order.path: count=12036 total_ns=4869941 avg_ns=404 max_ns=21972
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1933912 avg_ns=482 max_ns=5091
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=593452 avg_ns=147 max_ns=4890
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1900232 avg_ns=473 max_ns=9215
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2342577 avg_ns=583 max_ns=21972
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=63345 avg_ns=27 max_ns=289
  state_read_lock_hold: count=2269 total_ns=151442 avg_ns=66 max_ns=3669
  state_write_lock_wait: count=1875 total_ns=34282 avg_ns=18 max_ns=155
  state_write_lock_hold: count=1875 total_ns=1000188 avg_ns=533 max_ns=45966
  open_confined_openat2: count=3180 total_ns=1755195 avg_ns=551 max_ns=19936
  open_like.pre_open_guard.access: count=1 total_ns=89813 avg_ns=89813 max_ns=89813
  open_like.pre_open_guard.opendir: count=26 total_ns=288451 avg_ns=11094 max_ns=34683
  open_like.post_open_revalidation.access: count=1 total_ns=21231 avg_ns=21231 max_ns=21231
  open_like.post_open_revalidation.opendir: count=26 total_ns=135952 avg_ns=5228 max_ns=14197
  stat_child_no_follow: count=3101 total_ns=2984548 avg_ns=962 max_ns=26524
  stat_child_no_follow.attr_conversion: count=3099 total_ns=43501 avg_ns=14 max_ns=130
  stat_child_no_follow.host_fstat: count=3099 total_ns=413926 avg_ns=133 max_ns=2033
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=2984548 avg_ns=962 max_ns=26524
  source_root_path: count=2321 total_ns=3144680 avg_ns=1354 max_ns=43777
  resolved_virtual_path: count=2346 total_ns=8315282 avg_ns=3544 max_ns=21362
  resolved_virtual_path_from_path: count=2267 total_ns=8189950 avg_ns=3612 max_ns=21362
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=7246637 avg_ns=3196 max_ns=20984
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=6001621 avg_ns=956 max_ns=11027
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=631328 avg_ns=100 max_ns=19147
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=826452 avg_ns=364 max_ns=8106
  resolved_virtual_path_from_open_fd: count=79 total_ns=125332 avg_ns=1586 max_ns=17678
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=153498 avg_ns=5903 max_ns=27273
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6593 avg_ns=253 max_ns=738
  readdirplus_directory_scan: count=26 total_ns=4628875 avg_ns=178033 max_ns=234129
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=286793 avg_ns=11030 max_ns=11957
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=765071 avg_ns=919 max_ns=3069
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4021854 avg_ns=4833 max_ns=8915
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4085091 avg_ns=157118 max_ns=204246
  readdirplus_attr_generation_scan: count=858 total_ns=765071 avg_ns=891 max_ns=3069
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=112146 avg_ns=4313 max_ns=12864
  readdirplus_page_commit: count=26 total_ns=431093 avg_ns=16580 max_ns=46137
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
