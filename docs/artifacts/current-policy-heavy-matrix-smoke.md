# ScreenFS benchmark result

- timestamp: `2026-06-24T03:47:58.297845+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --workload-set policy-heavy-matrix --matcher-extra-rules 32 --iterations 3 --warmups 1 --metadata-ops 200 --matcher-misses 200 --output-json docs/artifacts/current-policy-heavy-matrix-smoke.json --output-md docs/artifacts/current-policy-heavy-matrix-smoke.md --output-svg docs/artifacts/current-policy-heavy-matrix-smoke.svg`
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
- policy_label: `fallback-unsafe-policy-matcher32`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.001359 | 0.011636 | 8.565 | 0.012723 | 0.012859 | 0.012968 |
| metadata_getattr | 0.001375 | 0.016149 | 11.744 | 0.016273 | 0.016289 | 0.016301 |
| metadata_access | 0.001180 | 0.016517 | 13.992 | 0.016770 | 0.016802 | 0.016827 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009645 | 0.009903 | 0.009935 | 0.009961 |
| matcher_readonly_access_wok | 0.024364 | 0.024833 | 0.024891 | 0.024938 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1601 total_ns=38305581 avg_ns=23926 max_ns=95211
  fuse_op.getattr: count=929 total_ns=13189626 avg_ns=14197 max_ns=25170
  fuse_op.lookup: count=13317 total_ns=149552488 avg_ns=11230 max_ns=92938
  fuse_op.statfs: count=2 total_ns=4302 avg_ns=2151 max_ns=2296
  policy_decision: count=18248 total_ns=26032511 avg_ns=1426 max_ns=78762
  matcher_candidates: count=189088
  matcher_candidates_by_source.hidden.path: count=800
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=800
  matcher_candidates_by_source.visible.descendant: count=187488
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=189088
  matcher_candidate_order.descendant: count=17448 total_ns=10569592 avg_ns=605 max_ns=21630
  matcher_candidate_order.path: count=53944 total_ns=28309855 avg_ns=524 max_ns=15795
  matcher_candidate_order_by_source.hidden.path: count=17448 total_ns=12799674 avg_ns=733 max_ns=8084
  matcher_candidate_order_by_source.internal_hidden.path: count=17448 total_ns=3078828 avg_ns=176 max_ns=15795
  matcher_candidate_order_by_source.readonly.path: count=800 total_ns=932976 avg_ns=1166 max_ns=2815
  matcher_candidate_order_by_source.visible.descendant: count=17448 total_ns=10569592 avg_ns=605 max_ns=21630
  matcher_candidate_order_by_source.visible.path: count=17448 total_ns=11319495 avg_ns=648 max_ns=6855
  matcher_candidate_order_by_source.writable.path: count=800 total_ns=178882 avg_ns=223 max_ns=943
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1736304
  matcher_candidate_order_seen_slots.descendant: count=558336
  matcher_candidate_order_seen_slots.path: count=1177968
  matcher_candidate_order_ancestor_steps: count=248204
  matcher_candidate_order_ancestor_steps.descendant: count=60051
  matcher_candidate_order_ancestor_steps.path: count=188153
  state_read_lock_wait: count=15847 total_ns=379586 avg_ns=23 max_ns=2341
  state_read_lock_hold: count=15847 total_ns=1067658 avg_ns=67 max_ns=1636
  state_write_lock_wait: count=11715 total_ns=271258 avg_ns=23 max_ns=350
  state_write_lock_hold: count=11715 total_ns=4610849 avg_ns=393 max_ns=42147
  open_confined_openat2: count=16648 total_ns=13196754 avg_ns=792 max_ns=46372
  open_like.pre_open_guard.access: count=1601 total_ns=31588881 avg_ns=19730 max_ns=74948
  open_like.post_open_revalidation.access: count=801 total_ns=5375902 avg_ns=6711 max_ns=77642
  stat_child_no_follow: count=15847 total_ns=22379263 avg_ns=1412 max_ns=47768
  stat_child_no_follow.attr_conversion: count=15045 total_ns=261795 avg_ns=17 max_ns=633
  stat_child_no_follow.host_fstat: count=15045 total_ns=3663715 avg_ns=243 max_ns=2529
  stat_child_no_follow_context.path_guard_or_metadata: count=15847 total_ns=22379263 avg_ns=1412 max_ns=47768
  source_root_path: count=15047 total_ns=28284658 avg_ns=1879 max_ns=73899
  resolved_virtual_path: count=15046 total_ns=51655982 avg_ns=3433 max_ns=36751
  resolved_virtual_path_from_path: count=14245 total_ns=50696479 avg_ns=3558 max_ns=36751
  resolved_virtual_path_from_path_component_walk: count=14245 total_ns=44301062 avg_ns=3109 max_ns=36050
  resolved_virtual_path_from_path_canonicalize: count=32201 total_ns=35769400 avg_ns=1110 max_ns=34925
  resolved_virtual_path_from_path_source_root_confinement: count=32201 total_ns=4851510 avg_ns=150 max_ns=12390
  resolved_virtual_path_from_path_virtual_conversion: count=14245 total_ns=5484813 avg_ns=385 max_ns=4721
  resolved_virtual_path_from_open_fd: count=801 total_ns=959503 avg_ns=1197 max_ns=5352
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
