# ScreenFS benchmark result

- timestamp: `2026-06-24T03:47:56.663002+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload matcher_readonly_access_wok --iterations 3 --warmups 1 --matcher-misses 200 --output-json docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.json --output-md docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.md --output-svg docs/artifacts/current-matcher-mutability-source-attribution-smoke/matcher-readonly-access-wok.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `(none)`
- screenfs_only_workloads: `matcher_readonly_access_wok`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_readonly_access_wok | 0.019273 | 0.019281 | 0.019282 | 0.019282 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=801 total_ns=17012608 avg_ns=21239 max_ns=103248
  fuse_op.getattr: count=129 total_ns=1850837 avg_ns=14347 max_ns=93737
  fuse_op.lookup: count=3717 total_ns=37799594 avg_ns=10169 max_ns=79464
  fuse_op.statfs: count=2 total_ns=1976 avg_ns=988 max_ns=1053
  policy_decision: count=6248 total_ns=8048235 avg_ns=1288 max_ns=12295
  matcher_candidates: count=60288
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=800
  matcher_candidates_by_source.visible.descendant: count=59488
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=60288
  matcher_candidate_order.descendant: count=5448 total_ns=2710913 avg_ns=497 max_ns=19063
  matcher_candidate_order.path: count=17944 total_ns=8741561 avg_ns=487 max_ns=6363
  matcher_candidate_order_by_source.hidden.path: count=5448 total_ns=3646029 avg_ns=669 max_ns=6363
  matcher_candidate_order_by_source.internal_hidden.path: count=5448 total_ns=920579 avg_ns=168 max_ns=5494
  matcher_candidate_order_by_source.readonly.path: count=800 total_ns=775234 avg_ns=969 max_ns=4178
  matcher_candidate_order_by_source.visible.descendant: count=5448 total_ns=2710913 avg_ns=497 max_ns=19063
  matcher_candidate_order_by_source.visible.path: count=5448 total_ns=3247277 avg_ns=596 max_ns=4051
  matcher_candidate_order_by_source.writable.path: count=800 total_ns=152442 avg_ns=190 max_ns=583
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=560304
  matcher_candidate_order_seen_slots.descendant: count=174336
  matcher_candidate_order_seen_slots.path: count=385968
  matcher_candidate_order_ancestor_steps: count=94604
  matcher_candidate_order_ancestor_steps.descendant: count=21651
  matcher_candidate_order_ancestor_steps.path: count=72953
  state_read_lock_wait: count=4647 total_ns=84395 avg_ns=18 max_ns=231
  state_read_lock_hold: count=4647 total_ns=302724 avg_ns=65 max_ns=8266
  state_write_lock_wait: count=3715 total_ns=65293 avg_ns=17 max_ns=202
  state_write_lock_hold: count=3715 total_ns=1265866 avg_ns=340 max_ns=53056
  open_confined_openat2: count=4648 total_ns=2631302 avg_ns=566 max_ns=14176
  open_like.pre_open_guard.access: count=801 total_ns=16784581 avg_ns=20954 max_ns=102221
  open_like.post_open_revalidation.access: count=1 total_ns=9741 avg_ns=9741 max_ns=9741
  stat_child_no_follow: count=4647 total_ns=4784208 avg_ns=1029 max_ns=46379
  stat_child_no_follow.attr_conversion: count=4645 total_ns=63722 avg_ns=13 max_ns=58
  stat_child_no_follow.host_fstat: count=4645 total_ns=701431 avg_ns=151 max_ns=4296
  stat_child_no_follow_context.path_guard_or_metadata: count=4647 total_ns=4784208 avg_ns=1029 max_ns=46379
  source_root_path: count=4647 total_ns=6276826 avg_ns=1350 max_ns=48552
  resolved_virtual_path: count=4646 total_ns=17284239 avg_ns=3720 max_ns=53218
  resolved_virtual_path_from_path: count=4645 total_ns=17278435 avg_ns=3719 max_ns=53218
  resolved_virtual_path_from_path_component_walk: count=4645 total_ns=15390280 avg_ns=3313 max_ns=52759
  resolved_virtual_path_from_path_canonicalize: count=13001 total_ns=12403243 avg_ns=954 max_ns=52062
  resolved_virtual_path_from_path_source_root_confinement: count=13001 total_ns=1898167 avg_ns=146 max_ns=6670
  resolved_virtual_path_from_path_virtual_conversion: count=4645 total_ns=1653433 avg_ns=355 max_ns=12193
  resolved_virtual_path_from_open_fd: count=1 total_ns=5804 avg_ns=5804 max_ns=5804
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
