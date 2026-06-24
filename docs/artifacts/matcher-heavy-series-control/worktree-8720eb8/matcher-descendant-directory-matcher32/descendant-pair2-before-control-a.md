# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:29.889052+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-before-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-before-control-a.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-before-control-a.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-before-control-a.svg`
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
| matcher_descendant_readdir | 0.000029 | 0.000678 | 23.506 | 0.000739 | 0.000817 | 0.000880 |
| matcher_descendant_readdirplus | 0.000073 | 0.003649 | 49.683 | 0.004167 | 0.004387 | 0.004564 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=80773 avg_ns=80773 max_ns=80773
  fuse_op.getattr: count=443 total_ns=7446152 avg_ns=16808 max_ns=74629
  fuse_op.lookup: count=1747 total_ns=22272132 avg_ns=12748 max_ns=112222
  fuse_op.opendir: count=26 total_ns=545359 avg_ns=20975 max_ns=34437
  fuse_op.readdir: count=26 total_ns=601412 avg_ns=23131 max_ns=55148
  fuse_op.readdirplus: count=26 total_ns=13351313 avg_ns=513512 max_ns=1473512
  fuse_op.releasedir: count=26 total_ns=90784 avg_ns=3491 max_ns=6617
  fuse_op.statfs: count=2 total_ns=5950 avg_ns=2975 max_ns=3076
  policy_decision: count=4012 total_ns=8223565 avg_ns=2049 max_ns=28912
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2283535 avg_ns=569 max_ns=26010
  matcher_candidate_order.path: count=12036 total_ns=5974372 avg_ns=496 max_ns=30319
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2462222 avg_ns=613 max_ns=30319
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=755732 avg_ns=188 max_ns=3791
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2283535 avg_ns=569 max_ns=26010
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2756418 avg_ns=687 max_ns=27462
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=60380 avg_ns=26 max_ns=245
  state_read_lock_hold: count=2269 total_ns=179561 avg_ns=79 max_ns=1368
  state_write_lock_wait: count=1875 total_ns=44926 avg_ns=23 max_ns=283
  state_write_lock_hold: count=1875 total_ns=1214618 avg_ns=647 max_ns=82775
  open_confined_openat2: count=3180 total_ns=2823005 avg_ns=887 max_ns=27071
  open_like.pre_open_guard.access: count=1 total_ns=67888 avg_ns=67888 max_ns=67888
  open_like.pre_open_guard.opendir: count=26 total_ns=345770 avg_ns=13298 max_ns=22547
  open_like.post_open_revalidation.access: count=1 total_ns=6573 avg_ns=6573 max_ns=6573
  open_like.post_open_revalidation.opendir: count=26 total_ns=150087 avg_ns=5772 max_ns=7460
  stat_child_no_follow: count=3101 total_ns=4805901 avg_ns=1549 max_ns=32661
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53484 avg_ns=17 max_ns=25
  stat_child_no_follow.host_fstat: count=3099 total_ns=842804 avg_ns=271 max_ns=30232
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=4805901 avg_ns=1549 max_ns=32661
  source_root_path: count=2321 total_ns=4917724 avg_ns=2118 max_ns=63914
  resolved_virtual_path: count=2346 total_ns=10510514 avg_ns=4480 max_ns=64234
  resolved_virtual_path_from_path: count=2267 total_ns=10384990 avg_ns=4580 max_ns=64234
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=9198043 avg_ns=4057 max_ns=63403
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7703639 avg_ns=1227 max_ns=62305
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=682513 avg_ns=108 max_ns=1019
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=1028430 avg_ns=453 max_ns=36537
  resolved_virtual_path_from_open_fd: count=79 total_ns=125524 avg_ns=1588 max_ns=3695
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=219555 avg_ns=8444 max_ns=23258
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=12007 avg_ns=461 max_ns=1369
  readdirplus_directory_scan: count=26 total_ns=5580672 avg_ns=214641 max_ns=591348
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=322380 avg_ns=12399 max_ns=13983
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1371527 avg_ns=1648 max_ns=32757
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4738479 avg_ns=5695 max_ns=35418
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4830681 avg_ns=185795 max_ns=528426
  readdirplus_attr_generation_scan: count=858 total_ns=1371527 avg_ns=1598 max_ns=32757
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=190953 avg_ns=7344 max_ns=29409
  readdirplus_page_commit: count=26 total_ns=541084 avg_ns=20810 max_ns=83013
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
