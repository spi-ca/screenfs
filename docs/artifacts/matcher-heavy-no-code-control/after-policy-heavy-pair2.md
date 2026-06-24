# ScreenFS benchmark result

- timestamp: `2026-06-24T05:33:56.511256+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-after-pair2 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair2.json --output-md docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair2.md --output-svg docs/artifacts/matcher-heavy-no-code-control/after-policy-heavy-pair2.svg`
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
- policy_label: `matcher-heavy-no-code-control-after-pair2`
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
| metadata_lookup | 0.001730 | 0.008105 | 4.684 | 0.008951 | 0.009145 | 0.009299 |
| metadata_getattr | 0.001978 | 0.010261 | 5.189 | 0.010933 | 0.011015 | 0.011081 |
| metadata_access | 0.001570 | 0.011039 | 7.031 | 0.011466 | 0.011573 | 0.011658 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.006546 | 0.007190 | 0.007483 | 0.007718 |
| matcher_readonly_access_wok | 0.023957 | 0.024255 | 0.024325 | 0.024381 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=103371362 avg_ns=19875 max_ns=282743
  fuse_op.getattr: count=3017 total_ns=30785270 avg_ns=10203 max_ns=45335
  fuse_op.lookup: count=43269 total_ns=375913787 avg_ns=8687 max_ns=143306
  fuse_op.statfs: count=2 total_ns=6767 avg_ns=3383 max_ns=4663
  policy_decision: count=59288 total_ns=66621120 avg_ns=1123 max_ns=11581
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
  matcher_candidate_order.descendant: count=56688 total_ns=24843462 avg_ns=438 max_ns=26950
  matcher_candidate_order.path: count=175264 total_ns=71232885 avg_ns=406 max_ns=37000
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=31928918 avg_ns=563 max_ns=11586
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7181147 avg_ns=126 max_ns=7570
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2787137 avg_ns=1071 max_ns=4588
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=24843462 avg_ns=438 max_ns=26950
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=28839656 avg_ns=508 max_ns=15275
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=496027 avg_ns=190 max_ns=37000
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1282010 avg_ns=24 max_ns=382
  state_read_lock_hold: count=51487 total_ns=2891595 avg_ns=56 max_ns=16563
  state_write_lock_wait: count=38067 total_ns=705028 avg_ns=18 max_ns=18172
  state_write_lock_hold: count=38067 total_ns=11292960 avg_ns=296 max_ns=9409
  open_confined_openat2: count=54088 total_ns=31080655 avg_ns=574 max_ns=52969
  open_like.pre_open_guard.access: count=5201 total_ns=88326424 avg_ns=16982 max_ns=76097
  open_like.post_open_revalidation.access: count=2601 total_ns=12105159 avg_ns=4654 max_ns=273415
  stat_child_no_follow: count=51487 total_ns=52423883 avg_ns=1018 max_ns=53420
  stat_child_no_follow.attr_conversion: count=48885 total_ns=672901 avg_ns=13 max_ns=130
  stat_child_no_follow.host_fstat: count=48885 total_ns=7615737 avg_ns=155 max_ns=4896
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=52423883 avg_ns=1018 max_ns=53420
  source_root_path: count=48887 total_ns=67287120 avg_ns=1376 max_ns=67745
  resolved_virtual_path: count=48886 total_ns=133622579 avg_ns=2733 max_ns=39175
  resolved_virtual_path_from_path: count=46285 total_ns=131629909 avg_ns=2843 max_ns=39175
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=115208887 avg_ns=2489 max_ns=38821
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=95890988 avg_ns=916 max_ns=38373
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=10046210 avg_ns=96 max_ns=6564
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=14082699 avg_ns=304 max_ns=13913
  resolved_virtual_path_from_open_fd: count=2601 total_ns=1992670 avg_ns=766 max_ns=6685
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
