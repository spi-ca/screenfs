# ScreenFS benchmark result

- timestamp: `2026-06-24T05:33:50.008154+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-heavy-no-code-control-before-pair1 --workload-set policy-heavy-matrix --output-json docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair1.json --output-md docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair1.md --output-svg docs/artifacts/matcher-heavy-no-code-control/before-policy-heavy-pair1.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+17 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-heavy-no-code-control-before-pair1`
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
| metadata_lookup | 0.001943 | 0.011201 | 5.766 | 0.011541 | 0.011607 | 0.011659 |
| metadata_getattr | 0.002135 | 0.014926 | 6.990 | 0.016502 | 0.017041 | 0.017473 |
| metadata_access | 0.001428 | 0.016133 | 11.295 | 0.017139 | 0.017720 | 0.018185 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009418 | 0.009574 | 0.009618 | 0.009653 |
| matcher_readonly_access_wok | 0.022656 | 0.022928 | 0.022957 | 0.022979 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=119242603 avg_ns=22926 max_ns=110565
  fuse_op.getattr: count=3017 total_ns=41648789 avg_ns=13804 max_ns=69995
  fuse_op.lookup: count=43269 total_ns=470786132 avg_ns=10880 max_ns=132972
  fuse_op.statfs: count=2 total_ns=9142 avg_ns=4571 max_ns=5296
  policy_decision: count=59288 total_ns=81455719 avg_ns=1373 max_ns=20726
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
  matcher_candidate_order.descendant: count=56688 total_ns=31411705 avg_ns=554 max_ns=12323
  matcher_candidate_order.path: count=175264 total_ns=87213636 avg_ns=497 max_ns=15700
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=39730745 avg_ns=700 max_ns=8899
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=9123618 avg_ns=160 max_ns=15700
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2626544 avg_ns=1010 max_ns=2099
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=31411705 avg_ns=554 max_ns=12323
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=35266429 avg_ns=622 max_ns=12615
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=466300 avg_ns=179 max_ns=2792
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=1334096 avg_ns=25 max_ns=358
  state_read_lock_hold: count=51487 total_ns=3177936 avg_ns=61 max_ns=3497
  state_write_lock_wait: count=38067 total_ns=882458 avg_ns=23 max_ns=1761
  state_write_lock_hold: count=38067 total_ns=13397520 avg_ns=351 max_ns=20323
  open_confined_openat2: count=54088 total_ns=43438677 avg_ns=803 max_ns=23788
  open_like.pre_open_guard.access: count=5201 total_ns=98523341 avg_ns=18943 max_ns=91986
  open_like.post_open_revalidation.access: count=2601 total_ns=16586089 avg_ns=6376 max_ns=12800
  stat_child_no_follow: count=51487 total_ns=71754394 avg_ns=1393 max_ns=34332
  stat_child_no_follow.attr_conversion: count=48885 total_ns=848321 avg_ns=17 max_ns=384
  stat_child_no_follow.host_fstat: count=48885 total_ns=11013934 avg_ns=225 max_ns=13385
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=71754394 avg_ns=1393 max_ns=34332
  source_root_path: count=48887 total_ns=89571910 avg_ns=1832 max_ns=44363
  resolved_virtual_path: count=48886 total_ns=162782838 avg_ns=3329 max_ns=35806
  resolved_virtual_path_from_path: count=46285 total_ns=159618570 avg_ns=3448 max_ns=35806
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=138970350 avg_ns=3002 max_ns=35318
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=115157046 avg_ns=1100 max_ns=16125
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=11500802 avg_ns=109 max_ns=5676
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=17767047 avg_ns=383 max_ns=11296
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3164268 avg_ns=1216 max_ns=7670
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
