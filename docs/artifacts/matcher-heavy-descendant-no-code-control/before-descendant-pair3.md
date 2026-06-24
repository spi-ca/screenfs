# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:38.622969+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-before-pair3 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair3.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair3.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair3.svg`
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
- policy_label: `matcher-descendant-same-binary-control-before-pair3`
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
| matcher_descendant_readdir | 0.000020 | 0.000619 | 30.436 | 0.000675 | 0.000691 | 0.000704 |
| matcher_descendant_readdirplus | 0.000047 | 0.002723 | 58.406 | 0.003169 | 0.003187 | 0.003202 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=96155 avg_ns=96155 max_ns=96155
  fuse_op.getattr: count=443 total_ns=5444030 avg_ns=12289 max_ns=94520
  fuse_op.lookup: count=1747 total_ns=16136924 avg_ns=9236 max_ns=288115
  fuse_op.opendir: count=26 total_ns=505761 avg_ns=19452 max_ns=69399
  fuse_op.readdir: count=26 total_ns=566023 avg_ns=21770 max_ns=101881
  fuse_op.readdirplus: count=26 total_ns=10199574 avg_ns=392291 max_ns=557536
  fuse_op.releasedir: count=26 total_ns=169353 avg_ns=6513 max_ns=17408
  fuse_op.statfs: count=2 total_ns=4831 avg_ns=2415 max_ns=2435
  policy_decision: count=4012 total_ns=6407818 avg_ns=1597 max_ns=24594
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=1770782 avg_ns=441 max_ns=3200
  matcher_candidate_order.path: count=12036 total_ns=4532889 avg_ns=376 max_ns=6169
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=1816316 avg_ns=452 max_ns=5728
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=550490 avg_ns=137 max_ns=6169
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=1770782 avg_ns=441 max_ns=3200
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2166083 avg_ns=539 max_ns=4755
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=61461 avg_ns=27 max_ns=270
  state_read_lock_hold: count=2269 total_ns=157161 avg_ns=69 max_ns=2696
  state_write_lock_wait: count=1875 total_ns=31817 avg_ns=16 max_ns=206
  state_write_lock_hold: count=1875 total_ns=1291515 avg_ns=688 max_ns=281860
  open_confined_openat2: count=3180 total_ns=1669855 avg_ns=525 max_ns=10249
  open_like.pre_open_guard.access: count=1 total_ns=76576 avg_ns=76576 max_ns=76576
  open_like.pre_open_guard.opendir: count=26 total_ns=336510 avg_ns=12942 max_ns=61625
  open_like.post_open_revalidation.access: count=1 total_ns=11882 avg_ns=11882 max_ns=11882
  open_like.post_open_revalidation.opendir: count=26 total_ns=131609 avg_ns=5061 max_ns=12953
  stat_child_no_follow: count=3101 total_ns=2812666 avg_ns=907 max_ns=15592
  stat_child_no_follow.attr_conversion: count=3099 total_ns=40094 avg_ns=12 max_ns=143
  stat_child_no_follow.host_fstat: count=3099 total_ns=395062 avg_ns=127 max_ns=2208
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=2812666 avg_ns=907 max_ns=15592
  source_root_path: count=2321 total_ns=2943144 avg_ns=1268 max_ns=29792
  resolved_virtual_path: count=2346 total_ns=7779971 avg_ns=3316 max_ns=68616
  resolved_virtual_path_from_path: count=2267 total_ns=7664877 avg_ns=3381 max_ns=68616
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=6793771 avg_ns=2996 max_ns=68233
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=5588480 avg_ns=890 max_ns=52440
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=635780 avg_ns=101 max_ns=65211
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=762603 avg_ns=336 max_ns=4020
  resolved_virtual_path_from_open_fd: count=79 total_ns=115094 avg_ns=1456 max_ns=6529
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=196647 avg_ns=7563 max_ns=31071
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6870 avg_ns=264 max_ns=611
  readdirplus_directory_scan: count=26 total_ns=4448744 avg_ns=171105 max_ns=255800
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=278556 avg_ns=10713 max_ns=13266
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=728600 avg_ns=875 max_ns=2004
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=3783143 avg_ns=4547 max_ns=7315
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=3889734 avg_ns=149605 max_ns=225212
  readdirplus_attr_generation_scan: count=858 total_ns=728600 avg_ns=849 max_ns=2004
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=125761 avg_ns=4836 max_ns=13027
  readdirplus_page_commit: count=26 total_ns=398356 avg_ns=15321 max_ns=34089
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
