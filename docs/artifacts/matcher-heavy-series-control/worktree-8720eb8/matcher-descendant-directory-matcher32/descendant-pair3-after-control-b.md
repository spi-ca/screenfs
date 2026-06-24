# ScreenFS benchmark result

- timestamp: `2026-06-24T13:04:31.943574+00:00`
- harness_command_line: `/usr/bin/python3 /home/spi-ca/Codebase/screenfs/scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-series-after-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-after-control-b.json --output-md docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-after-control-b.md --output-svg docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-after-control-b.svg`
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
| matcher_descendant_readdir | 0.000031 | 0.000690 | 22.119 | 0.000775 | 0.000793 | 0.000806 |
| matcher_descendant_readdirplus | 0.000074 | 0.003843 | 52.191 | 0.004284 | 0.004421 | 0.004531 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=95910 avg_ns=95910 max_ns=95910
  fuse_op.getattr: count=443 total_ns=7372633 avg_ns=16642 max_ns=43022
  fuse_op.lookup: count=1747 total_ns=22800631 avg_ns=13051 max_ns=84125
  fuse_op.opendir: count=26 total_ns=579683 avg_ns=22295 max_ns=42317
  fuse_op.readdir: count=26 total_ns=627720 avg_ns=24143 max_ns=58215
  fuse_op.readdirplus: count=26 total_ns=12834427 avg_ns=493631 max_ns=686220
  fuse_op.releasedir: count=26 total_ns=104923 avg_ns=4035 max_ns=9596
  fuse_op.statfs: count=2 total_ns=5015 avg_ns=2507 max_ns=2513
  policy_decision: count=4012 total_ns=8174146 avg_ns=2037 max_ns=28164
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2327077 avg_ns=580 max_ns=62378
  matcher_candidate_order.path: count=12036 total_ns=5867937 avg_ns=487 max_ns=25961
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2446738 avg_ns=609 max_ns=19854
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=755586 avg_ns=188 max_ns=4687
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2327077 avg_ns=580 max_ns=62378
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2665613 avg_ns=664 max_ns=25961
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=59918 avg_ns=26 max_ns=330
  state_read_lock_hold: count=2269 total_ns=220579 avg_ns=97 max_ns=25605
  state_write_lock_wait: count=1875 total_ns=46585 avg_ns=24 max_ns=313
  state_write_lock_hold: count=1875 total_ns=1264663 avg_ns=674 max_ns=68767
  open_confined_openat2: count=3180 total_ns=2863255 avg_ns=900 max_ns=27905
  open_like.pre_open_guard.access: count=1 total_ns=76923 avg_ns=76923 max_ns=76923
  open_like.pre_open_guard.opendir: count=26 total_ns=363693 avg_ns=13988 max_ns=28552
  open_like.post_open_revalidation.access: count=1 total_ns=11868 avg_ns=11868 max_ns=11868
  open_like.post_open_revalidation.opendir: count=26 total_ns=163836 avg_ns=6301 max_ns=14649
  stat_child_no_follow: count=3101 total_ns=4766654 avg_ns=1537 max_ns=28886
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53345 avg_ns=17 max_ns=29
  stat_child_no_follow.host_fstat: count=3099 total_ns=788533 avg_ns=254 max_ns=2429
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=4766654 avg_ns=1537 max_ns=28886
  source_root_path: count=2321 total_ns=5190127 avg_ns=2236 max_ns=35364
  resolved_virtual_path: count=2346 total_ns=10474222 avg_ns=4464 max_ns=33026
  resolved_virtual_path_from_path: count=2267 total_ns=10333234 avg_ns=4558 max_ns=33026
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=9183118 avg_ns=4050 max_ns=32184
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7666958 avg_ns=1221 max_ns=31238
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=691537 avg_ns=110 max_ns=1235
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=1001733 avg_ns=441 max_ns=1706
  resolved_virtual_path_from_open_fd: count=79 total_ns=140988 avg_ns=1784 max_ns=8746
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=227485 avg_ns=8749 max_ns=26375
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=13321 avg_ns=512 max_ns=1166
  readdirplus_directory_scan: count=26 total_ns=5370104 avg_ns=206542 max_ns=306866
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=333962 avg_ns=12844 max_ns=18358
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=1196298 avg_ns=1437 max_ns=19204
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4598227 avg_ns=5526 max_ns=67679
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4565001 avg_ns=175576 max_ns=236873
  readdirplus_attr_generation_scan: count=858 total_ns=1196298 avg_ns=1394 max_ns=19204
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=211286 avg_ns=8126 max_ns=21428
  readdirplus_page_commit: count=26 total_ns=564714 avg_ns=21719 max_ns=69051
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
