# ScreenFS benchmark result

- timestamp: `2026-06-24T02:25:35.218898+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-visible-default-shortcircuit/after-policy-heavy-matcher32.json --output-md /tmp/screenfs-visible-default-shortcircuit/after-policy-heavy-matcher32.md --output-svg /tmp/screenfs-visible-default-shortcircuit/after-policy-heavy-matcher32.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-visible-after-73522`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `4175e8c0164c496dfccb134572f825e22417daac8b8be012fe7a101a3a76bc64`
- screenfs_source_root: `/tmp/screenfs-visible-after-73522`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs;  M src/fs.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss`
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
| metadata_lookup | 0.001681 | 0.010843 | 6.450 | 0.012181 | 0.012508 | 0.012769 |
| metadata_getattr | 0.001682 | 0.015405 | 9.158 | 0.016417 | 0.016835 | 0.017169 |
| metadata_access | 0.001447 | 0.015966 | 11.033 | 0.017790 | 0.017796 | 0.017801 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009516 | 0.010566 | 0.010575 | 0.010582 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=51183076 avg_ns=19678 max_ns=109590
  fuse_op.getattr: count=2601 total_ns=33237835 avg_ns=12778 max_ns=32471
  fuse_op.lookup: count=31205 total_ns=321066210 avg_ns=10288 max_ns=84829
  fuse_op.statfs: count=2 total_ns=8694 avg_ns=4347 max_ns=5070
  policy_decision: count=39008 total_ns=32613995 avg_ns=836 max_ns=9392
  matcher_candidates: count=418696
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=416096
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=418696
  matcher_candidate_order.descendant: count=39008 total_ns=22175430 avg_ns=568 max_ns=49651
  matcher_candidate_order.path: count=117024 total_ns=56378129 avg_ns=481 max_ns=12457
  matcher_candidate_order_by_source.hidden.path: count=39008 total_ns=26436848 avg_ns=677 max_ns=12457
  matcher_candidate_order_by_source.internal_hidden.path: count=39008 total_ns=6298666 avg_ns=161 max_ns=11715
  matcher_candidate_order_by_source.visible.descendant: count=39008 total_ns=22175430 avg_ns=568 max_ns=49651
  matcher_candidate_order_by_source.visible.path: count=39008 total_ns=23642615 avg_ns=606 max_ns=7323
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3822784
  matcher_candidate_order_seen_slots.descendant: count=1248256
  matcher_candidate_order_seen_slots.path: count=2574528
  matcher_candidate_order_ancestor_steps: count=499276
  matcher_candidate_order_ancestor_steps.descendant: count=124819
  matcher_candidate_order_ancestor_steps.path: count=374457
  state_read_lock_wait: count=36407 total_ns=879380 avg_ns=24 max_ns=2308
  state_read_lock_hold: count=36407 total_ns=2383021 avg_ns=65 max_ns=1806
  state_write_lock_wait: count=26003 total_ns=601109 avg_ns=23 max_ns=438
  state_write_lock_hold: count=26003 total_ns=9309336 avg_ns=358 max_ns=24160
  open_confined_openat2: count=39008 total_ns=32012725 avg_ns=820 max_ns=18864
  open_like.pre_open_guard.access: count=2601 total_ns=32691549 avg_ns=12568 max_ns=70639
  open_like.post_open_revalidation.access: count=2601 total_ns=14934556 avg_ns=5741 max_ns=17379
  stat_child_no_follow: count=36407 total_ns=52542700 avg_ns=1443 max_ns=19731
  stat_child_no_follow.attr_conversion: count=33805 total_ns=588532 avg_ns=17 max_ns=52
  stat_child_no_follow.host_fstat: count=33805 total_ns=8388861 avg_ns=248 max_ns=8663
  stat_child_no_follow_context.path_guard_or_metadata: count=36407 total_ns=52542700 avg_ns=1443 max_ns=19731
  source_root_path: count=33807 total_ns=62880143 avg_ns=1859 max_ns=56136
  resolved_virtual_path: count=33806 total_ns=99025532 avg_ns=2929 max_ns=19320
  resolved_virtual_path_from_path: count=31205 total_ns=95948374 avg_ns=3074 max_ns=19320
  resolved_virtual_path_from_path_component_walk: count=31205 total_ns=82824049 avg_ns=2654 max_ns=18410
  resolved_virtual_path_from_path_canonicalize: count=62409 total_ns=65784809 avg_ns=1054 max_ns=17529
  resolved_virtual_path_from_path_source_root_confinement: count=62409 total_ns=9470896 avg_ns=151 max_ns=8010
  resolved_virtual_path_from_path_virtual_conversion: count=31205 total_ns=11149965 avg_ns=357 max_ns=9556
  resolved_virtual_path_from_open_fd: count=2601 total_ns=3077158 avg_ns=1183 max_ns=12006
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
