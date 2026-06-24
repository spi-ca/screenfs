# ScreenFS benchmark result

- timestamp: `2026-06-24T05:33:52.291347+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-after-pair1 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair1.json --output-md docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair1.md --output-svg docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair1.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-heavy-no-code-control-after-pair1`
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
| metadata_lookup | 0.001501 | 0.011606 | 7.734 | 0.012023 | 0.012343 | 0.012599 |
| metadata_getattr | 0.001511 | 0.014776 | 9.776 | 0.015126 | 0.015162 | 0.015192 |
| metadata_access | 0.001289 | 0.017042 | 13.222 | 0.018587 | 0.019246 | 0.019773 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.010260 | 0.011775 | 0.011841 | 0.011894 |
| matcher_readonly_access_wok | 0.023840 | 0.024461 | 0.024488 | 0.024511 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=122169547 avg_ns=23489 max_ns=100724
  fuse_op.getattr: count=3017 total_ns=41524181 avg_ns=13763 max_ns=31144
  fuse_op.lookup: count=43269 total_ns=484300288 avg_ns=11192 max_ns=95286
  fuse_op.statfs: count=2 total_ns=5872 avg_ns=2936 max_ns=3284
  policy_decision: count=59288 total_ns=82030354 avg_ns=1383 max_ns=13166
  matcher_candidates: count=614320
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=2600
  matcher_candidates_by_source.visible.descendant: count=609120
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=614320
  matcher_candidate_order.descendant: count=56688 total_ns=32271416 avg_ns=569 max_ns=9984
  matcher_candidate_order.path: count=175264 total_ns=88554929 avg_ns=505 max_ns=10971
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=40073445 avg_ns=706 max_ns=10971
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9408702 avg_ns=165 max_ns=10807
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2931611 avg_ns=1127 max_ns=5445
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=32271416 avg_ns=569 max_ns=9984
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35670427 avg_ns=629 max_ns=7372
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=470744 avg_ns=181 max_ns=371
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1362158 avg_ns=26 max_ns=11789
  state_read_lock_hold: count=51487 total_ns=3513944 avg_ns=68 max_ns=6379
  state_write_lock_wait: count=38067 total_ns=895914 avg_ns=23 max_ns=5596
  state_write_lock_hold: count=38067 total_ns=14662850 avg_ns=385 max_ns=45907
  open_confined_openat2: count=54088 total_ns=46641063 avg_ns=862 max_ns=26844
  open_like.pre_open_guard.access: count=5201 total_ns=100801386 avg_ns=19381 max_ns=82388
  open_like.post_open_revalidation.access: count=2601 total_ns=16935881 avg_ns=6511 max_ns=14457
  stat_child_no_follow: count=51487 total_ns=77208636 avg_ns=1499 max_ns=26932
  stat_child_no_follow.attr_conversion: count=48885 total_ns=851697 avg_ns=17 max_ns=195
  stat_child_no_follow.host_fstat: count=48885 total_ns=11752565 avg_ns=240 max_ns=4926
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=77208636 avg_ns=1499 max_ns=26932
  source_root_path: count=48887 total_ns=93172106 avg_ns=1905 max_ns=36983
  resolved_virtual_path: count=48886 total_ns=164148032 avg_ns=3357 max_ns=31989
  resolved_virtual_path_from_path: count=46285 total_ns=160895143 avg_ns=3476 max_ns=31989
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=140015097 avg_ns=3025 max_ns=31436
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=115986158 avg_ns=1108 max_ns=30574
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11429356 avg_ns=109 max_ns=4015
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17906479 avg_ns=386 max_ns=15347
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3252889 avg_ns=1250 max_ns=6656
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
