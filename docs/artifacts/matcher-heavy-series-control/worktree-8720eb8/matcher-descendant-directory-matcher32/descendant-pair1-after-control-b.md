# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:27.890368+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair1-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair1-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair1-after-control-b.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+28 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-series-after-control`
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
| matcher_descendant_readdir | 0.000023 | 0.000758 | 33.481 | 0.000853 | 0.000861 | 0.000867 |
| matcher_descendant_readdirplus | 0.000053 | 0.002893 | 54.345 | 0.004094 | 0.005746 | 0.007068 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=88670 avg_ns=88670 max_ns=88670
  fuse_op.getattr: count=443 total_ns=6508179 avg_ns=14691 max_ns=79013
  fuse_op.lookup: count=1747 total_ns=19381803 avg_ns=11094 max_ns=128592
  fuse_op.opendir: count=26 total_ns=528113 avg_ns=20312 max_ns=37024
  fuse_op.readdir: count=26 total_ns=584481 avg_ns=22480 max_ns=46805
  fuse_op.readdirplus: count=26 total_ns=12202141 avg_ns=469313 max_ns=590614
  fuse_op.releasedir: count=26 total_ns=83237 avg_ns=3201 max_ns=7441
  fuse_op.statfs: count=2 total_ns=1952 avg_ns=976 max_ns=1066
  policy_decision: count=4012 total_ns=7556331 avg_ns=1883 max_ns=90657
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2106295 avg_ns=524 max_ns=25885
  matcher_candidate_order.path: count=12036 total_ns=5337978 avg_ns=443 max_ns=18878
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2193133 avg_ns=546 max_ns=18878
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=685169 avg_ns=170 max_ns=4891
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2106295 avg_ns=524 max_ns=25885
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2459676 avg_ns=613 max_ns=5230
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=62839 avg_ns=27 max_ns=261
  state_read_lock_hold: count=2269 total_ns=164731 avg_ns=72 max_ns=1859
  state_write_lock_wait: count=1875 total_ns=63784 avg_ns=34 max_ns=25443
  state_write_lock_hold: count=1875 total_ns=1050591 avg_ns=560 max_ns=42588
  open_confined_openat2: count=3180 total_ns=2371721 avg_ns=745 max_ns=62138
  open_like.pre_open_guard.access: count=1 total_ns=68005 avg_ns=68005 max_ns=68005
  open_like.pre_open_guard.opendir: count=26 total_ns=333920 avg_ns=12843 max_ns=27087
  open_like.post_open_revalidation.access: count=1 total_ns=8503 avg_ns=8503 max_ns=8503
  open_like.post_open_revalidation.opendir: count=26 total_ns=148580 avg_ns=5714 max_ns=7818
  stat_child_no_follow: count=3101 total_ns=3864929 avg_ns=1246 max_ns=62786
  stat_child_no_follow.attr_conversion: count=3099 total_ns=47355 avg_ns=15 max_ns=50
  stat_child_no_follow.host_fstat: count=3099 total_ns=582289 avg_ns=187 max_ns=2730
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3864929 avg_ns=1246 max_ns=62786
  source_root_path: count=2321 total_ns=3916909 avg_ns=1687 max_ns=30870
  resolved_virtual_path: count=2346 total_ns=9248032 avg_ns=3942 max_ns=68874
  resolved_virtual_path_from_path: count=2267 total_ns=9115203 avg_ns=4020 max_ns=68874
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8053213 avg_ns=3552 max_ns=68202
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=6697306 avg_ns=1067 max_ns=67049
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=639429 avg_ns=101 max_ns=1749
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=934565 avg_ns=412 max_ns=14509
  resolved_virtual_path_from_open_fd: count=79 total_ns=132829 avg_ns=1681 max_ns=6790
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=198114 avg_ns=7619 max_ns=22406
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=8591 avg_ns=330 max_ns=1008
  readdirplus_directory_scan: count=26 total_ns=5301068 avg_ns=203887 max_ns=302011
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=351470 avg_ns=13518 max_ns=35459
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1044043 avg_ns=1254 max_ns=62882
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4297837 avg_ns=5165 max_ns=18340
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4477799 avg_ns=172223 max_ns=249858
  readdirplus_attr_generation_scan: count=858 total_ns=1044043 avg_ns=1216 max_ns=62882
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=159998 avg_ns=6153 max_ns=22109
  readdirplus_page_commit: count=26 total_ns=467244 avg_ns=17970 max_ns=42876
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
