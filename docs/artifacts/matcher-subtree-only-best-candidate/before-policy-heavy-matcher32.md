# ScreenFS benchmark result

- timestamp: `2026-06-24T02:07:50.244352+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-subtree-fastpath/before-policy-heavy-matcher32.json --output-md /tmp/screenfs-matcher-subtree-fastpath/before-policy-heavy-matcher32.md --output-svg /tmp/screenfs-matcher-subtree-fastpath/before-policy-heavy-matcher32.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-matcher-before-66015`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_worktree_clean: `True`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `99cc2d492f24c66320bc97284933be7091170e727b90e635e54b3f056d388e79`
- screenfs_source_root: `/tmp/screenfs-matcher-before-66015`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
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
| metadata_lookup | 0.001925 | 0.009181 | 4.769 | 0.009536 | 0.009767 | 0.009952 |
| metadata_getattr | 0.002140 | 0.011828 | 5.527 | 0.012652 | 0.012964 | 0.013213 |
| metadata_access | 0.001609 | 0.012206 | 7.588 | 0.013102 | 0.013284 | 0.013430 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007614 | 0.007832 | 0.008021 | 0.008173 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=2601 total_ns=41487934 avg_ns=15950 max_ns=108788
  fuse_op.getattr: count=2601 total_ns=27343727 avg_ns=10512 max_ns=67079
  fuse_op.lookup: count=31205 total_ns=249140339 avg_ns=7983 max_ns=324882
  fuse_op.statfs: count=2 total_ns=3686 avg_ns=1843 max_ns=2538
  policy_decision: count=39008 total_ns=40434270 avg_ns=1036 max_ns=59267
  matcher_candidates: count=418696
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=418696
  matcher_candidate_order.descendant: count=39008 total_ns=15860654 avg_ns=406 max_ns=102910
  matcher_candidate_order.path: count=117024 total_ns=43993407 avg_ns=375 max_ns=277477
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=3822784
  matcher_candidate_order_seen_slots.descendant: count=1248256
  matcher_candidate_order_seen_slots.path: count=2574528
  matcher_candidate_order_ancestor_steps: count=499276
  matcher_candidate_order_ancestor_steps.descendant: count=124819
  matcher_candidate_order_ancestor_steps.path: count=374457
  state_read_lock_wait: count=36407 total_ns=650799 avg_ns=17 max_ns=986
  state_read_lock_hold: count=36407 total_ns=1951012 avg_ns=53 max_ns=48220
  state_write_lock_wait: count=26003 total_ns=438578 avg_ns=16 max_ns=194
  state_write_lock_hold: count=26003 total_ns=6372948 avg_ns=245 max_ns=19368
  open_confined_openat2: count=39008 total_ns=20157941 avg_ns=516 max_ns=73028
  open_like.pre_open_guard.access: count=2601 total_ns=26126351 avg_ns=10044 max_ns=88872
  open_like.post_open_revalidation.access: count=2601 total_ns=12874845 avg_ns=4949 max_ns=95139
  stat_child_no_follow: count=36407 total_ns=33874315 avg_ns=930 max_ns=73441
  stat_child_no_follow.attr_conversion: count=33805 total_ns=448944 avg_ns=13 max_ns=2199
  stat_child_no_follow.host_fstat: count=33805 total_ns=4601764 avg_ns=136 max_ns=51290
  stat_child_no_follow_context.path_guard_or_metadata: count=36407 total_ns=33874315 avg_ns=930 max_ns=73441
  source_root_path: count=33807 total_ns=42187639 avg_ns=1247 max_ns=64180
  resolved_virtual_path: count=33806 total_ns=78783661 avg_ns=2330 max_ns=90699
  resolved_virtual_path_from_path: count=31205 total_ns=76448983 avg_ns=2449 max_ns=54303
  resolved_virtual_path_from_path_component_walk: count=31205 total_ns=66095270 avg_ns=2118 max_ns=53904
  resolved_virtual_path_from_path_canonicalize: count=62409 total_ns=51988931 avg_ns=833 max_ns=53167
  resolved_virtual_path_from_path_source_root_confinement: count=62409 total_ns=8956493 avg_ns=143 max_ns=46686
  resolved_virtual_path_from_path_virtual_conversion: count=31205 total_ns=8818516 avg_ns=282 max_ns=8443
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2334678 avg_ns=897 max_ns=90699
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
