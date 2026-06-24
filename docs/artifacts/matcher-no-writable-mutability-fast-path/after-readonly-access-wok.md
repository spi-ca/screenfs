# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:31.320228+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/after-readonly-access-wok.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/after-readonly-access-wok.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/after-readonly-access-wok.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload matcher_readonly_access_wok --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9e02d18073a0855dd1b5f1780bd57651a8474a9a971e0223e8e8e047c160a803`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-after-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+13 more)`
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
| matcher_readonly_access_wok | 0.019814 | 0.020749 | 0.020989 | 0.021180 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=59567350 avg_ns=22901 max_ns=2032239
  fuse_op.getattr: count=417 total_ns=6169655 avg_ns=14795 max_ns=64617
  fuse_op.lookup: count=12069 total_ns=125769624 avg_ns=10420 max_ns=274524
  fuse_op.statfs: count=2 total_ns=2269 avg_ns=1134 max_ns=1315
  policy_decision: count=20288 total_ns=28901612 avg_ns=1424 max_ns=2008274
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
  matcher_candidate_order.descendant: count=17688 total_ns=9077821 avg_ns=513 max_ns=8395
  matcher_candidate_order.path: count=58264 total_ns=29742170 avg_ns=510 max_ns=256738
  matcher_candidate_order_by_source.hidden.path: count=17688 total_ns=12123085 avg_ns=685 max_ns=19065
  matcher_candidate_order_by_source.internal_hidden.path: count=17688 total_ns=3145570 avg_ns=177 max_ns=139091
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2815725 avg_ns=1082 max_ns=143772
  matcher_candidate_order_by_source.visible.descendant: count=17688 total_ns=9077821 avg_ns=513 max_ns=8395
  matcher_candidate_order_by_source.visible.path: count=17688 total_ns=11131596 avg_ns=629 max_ns=256738
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=526194 avg_ns=202 max_ns=603
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1819224
  matcher_candidate_order_seen_slots.descendant: count=566016
  matcher_candidate_order_seen_slots.path: count=1253208
  matcher_candidate_order_ancestor_steps: count=307292
  matcher_candidate_order_ancestor_steps.descendant: count=70323
  matcher_candidate_order_ancestor_steps.path: count=236969
  state_read_lock_wait: count=15087 total_ns=283141 avg_ns=18 max_ns=628
  state_read_lock_hold: count=15087 total_ns=920955 avg_ns=61 max_ns=1642
  state_write_lock_wait: count=12067 total_ns=214240 avg_ns=17 max_ns=995
  state_write_lock_hold: count=12067 total_ns=4052929 avg_ns=335 max_ns=12121
  open_confined_openat2: count=15088 total_ns=8642600 avg_ns=572 max_ns=26385
  open_like.pre_open_guard.access: count=2601 total_ns=58831370 avg_ns=22618 max_ns=2031879
  open_like.post_open_revalidation.access: count=1 total_ns=10361 avg_ns=10361 max_ns=10361
  stat_child_no_follow: count=15087 total_ns=15516634 avg_ns=1028 max_ns=57325
  stat_child_no_follow.attr_conversion: count=15085 total_ns=209240 avg_ns=13 max_ns=146
  stat_child_no_follow.host_fstat: count=15085 total_ns=2275767 avg_ns=150 max_ns=14725
  stat_child_no_follow_context.path_guard_or_metadata: count=15087 total_ns=15516634 avg_ns=1028 max_ns=57325
  source_root_path: count=15087 total_ns=20286000 avg_ns=1344 max_ns=106811
  resolved_virtual_path: count=15086 total_ns=58392937 avg_ns=3870 max_ns=114762
  resolved_virtual_path_from_path: count=15085 total_ns=58386722 avg_ns=3870 max_ns=114762
  resolved_virtual_path_from_path_component_walk: count=15085 total_ns=52263706 avg_ns=3464 max_ns=113837
  resolved_virtual_path_from_path_canonicalize: count=42233 total_ns=42253850 avg_ns=1000 max_ns=112715
  resolved_virtual_path_from_path_source_root_confinement: count=42233 total_ns=6309788 avg_ns=149 max_ns=6751
  resolved_virtual_path_from_path_virtual_conversion: count=15085 total_ns=5353267 avg_ns=354 max_ns=50610
  resolved_virtual_path_from_open_fd: count=1 total_ns=6215 avg_ns=6215 max_ns=6215
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
