# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:01.220762+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/before-readonly-access-wok.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/before-readonly-access-wok.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/before-readonly-access-wok.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload matcher_readonly_access_wok --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `367b50287307eb569df4a3d08e42e6db25bf4e7cd1b9db1e3aa5b8fe2be4e673`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
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
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_readonly_access_wok | 0.025062 | 0.027688 | 0.029130 | 0.030283 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=71936719 avg_ns=27657 max_ns=2918458
  fuse_op.getattr: count=417 total_ns=7970963 avg_ns=19115 max_ns=422252
  fuse_op.lookup: count=12069 total_ns=156606899 avg_ns=12975 max_ns=319420
  fuse_op.statfs: count=2 total_ns=5548 avg_ns=2774 max_ns=3063
  policy_decision: count=20288 total_ns=32577467 avg_ns=1605 max_ns=41119
  matcher_candidates: count=195720
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=2600
  matcher_candidates_by_source.visible.descendant: count=193120
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=195720
  matcher_candidate_order.descendant: count=17688 total_ns=11287566 avg_ns=638 max_ns=10710
  matcher_candidate_order.path: count=58264 total_ns=35851384 avg_ns=615 max_ns=95786
  matcher_candidate_order_by_source.hidden.path: count=17688 total_ns=15084505 avg_ns=852 max_ns=22170
  matcher_candidate_order_by_source.internal_hidden.path: count=17688 total_ns=3711534 avg_ns=209 max_ns=95786
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=3179138 avg_ns=1222 max_ns=4309
  matcher_candidate_order_by_source.visible.descendant: count=17688 total_ns=11287566 avg_ns=638 max_ns=10710
  matcher_candidate_order_by_source.visible.path: count=17688 total_ns=13294960 avg_ns=751 max_ns=46515
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=581247 avg_ns=223 max_ns=4198
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1819224
  matcher_candidate_order_seen_slots.descendant: count=566016
  matcher_candidate_order_seen_slots.path: count=1253208
  matcher_candidate_order_ancestor_steps: count=307292
  matcher_candidate_order_ancestor_steps.descendant: count=70323
  matcher_candidate_order_ancestor_steps.path: count=236969
  state_read_lock_wait: count=15087 total_ns=353282 avg_ns=23 max_ns=399
  state_read_lock_hold: count=15087 total_ns=1107974 avg_ns=73 max_ns=2659
  state_write_lock_wait: count=12067 total_ns=273058 avg_ns=22 max_ns=396
  state_write_lock_hold: count=12067 total_ns=4951531 avg_ns=410 max_ns=8684
  open_confined_openat2: count=15088 total_ns=14399941 avg_ns=954 max_ns=187570
  open_like.pre_open_guard.access: count=2601 total_ns=71185295 avg_ns=27368 max_ns=2917366
  open_like.post_open_revalidation.access: count=1 total_ns=6833 avg_ns=6833 max_ns=6833
  stat_child_no_follow: count=15087 total_ns=24007175 avg_ns=1591 max_ns=188317
  stat_child_no_follow.attr_conversion: count=15085 total_ns=255810 avg_ns=16 max_ns=188
  stat_child_no_follow.host_fstat: count=15085 total_ns=3645402 avg_ns=241 max_ns=8226
  stat_child_no_follow_context.path_guard_or_metadata: count=15087 total_ns=24007175 avg_ns=1591 max_ns=188317
  source_root_path: count=15087 total_ns=29370002 avg_ns=1946 max_ns=300497
  resolved_virtual_path: count=15086 total_ns=71951124 avg_ns=4769 max_ns=2868669
  resolved_virtual_path_from_path: count=15085 total_ns=71947063 avg_ns=4769 max_ns=2868669
  resolved_virtual_path_from_path_component_walk: count=15085 total_ns=64015724 avg_ns=4243 max_ns=2865597
  resolved_virtual_path_from_path_canonicalize: count=42233 total_ns=52759574 avg_ns=1249 max_ns=2862632
  resolved_virtual_path_from_path_source_root_confinement: count=42233 total_ns=6598045 avg_ns=156 max_ns=2711
  resolved_virtual_path_from_path_virtual_conversion: count=15085 total_ns=6979638 avg_ns=462 max_ns=388771
  resolved_virtual_path_from_open_fd: count=1 total_ns=4061 avg_ns=4061 max_ns=4061
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
