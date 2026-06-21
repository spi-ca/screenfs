# ScreenFS benchmark result

- timestamp: `2026-06-21T02:12:20.322545+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-rank0-before.0mpY5o/target/release/screenfs --screenfs-source-root /tmp/screenfs-matcher-rank0-before.0mpY5o --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label matcher32-policy-heavy --workload-set policy-heavy-matrix --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-matcher32-policy-heavy-matrix.json --output-md /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-matcher32-policy-heavy-matrix.md --output-svg /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-matcher32-policy-heavy-matrix.svg`
- harness_repo_root: `/tmp/screenfs-matcher-rank0-before.0mpY5o`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-matcher-rank0-before.0mpY5o/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-matcher-rank0-before.0mpY5o`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher32-policy-heavy`
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
| metadata_lookup | 0.001799 | 0.031634 | 17.582 | 0.033889 | 0.033902 | 0.033912 |
| metadata_getattr | 0.002141 | 0.042954 | 20.063 | 0.044902 | 0.044983 | 0.045048 |
| metadata_access | 0.001777 | 0.046249 | 26.025 | 0.061223 | 0.063837 | 0.065928 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.139445 | 0.153768 | 0.155443 | 0.156784 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=6657 total_ns=168783267 avg_ns=25354 max_ns=261324
  fuse_op.getattr: count=6657 total_ns=120031328 avg_ns=18030 max_ns=175265
  fuse_op.lookup: count=137909 total_ns=1888717649 avg_ns=13695 max_ns=1569171
  fuse_op.statfs: count=2 total_ns=5230 avg_ns=2615 max_ns=2689
  policy_decision: count=309102 total_ns=259592256 avg_ns=839 max_ns=1554572
  matcher_candidates: count=6103152
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=6103152
  matcher_candidate_order.descendant: count=309102 total_ns=110092002 avg_ns=356 max_ns=77238
  matcher_candidate_order.path: count=927306 total_ns=270552623 avg_ns=291 max_ns=207843
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=30291996
  matcher_candidate_order_seen_slots.descendant: count=9891264
  matcher_candidate_order_seen_slots.path: count=20400732
  matcher_candidate_order_ancestor_steps: count=3237432
  matcher_candidate_order_ancestor_steps.descendant: count=809358
  matcher_candidate_order_ancestor_steps.path: count=2428074
  state_read_lock_wait: count=151223 total_ns=3279180 avg_ns=21 max_ns=60970
  state_read_lock_hold: count=151223 total_ns=10424514 avg_ns=68 max_ns=60080
  state_write_lock_wait: count=105251 total_ns=2131381 avg_ns=20 max_ns=16725
  state_write_lock_hold: count=105251 total_ns=33501185 avg_ns=318 max_ns=124143
  open_confined_openat2: count=157880 total_ns=86252886 avg_ns=546 max_ns=78631
  open_like.pre_open_guard.access: count=6657 total_ns=123329033 avg_ns=18526 max_ns=253033
  open_like.post_open_revalidation.access: count=6657 total_ns=37597724 avg_ns=5647 max_ns=90881
  stat_child_no_follow: count=151223 total_ns=1182150050 avg_ns=7817 max_ns=242869
  source_root_path: count=151223 total_ns=247700683 avg_ns=1637 max_ns=129150
  resolved_virtual_path: count=427666 total_ns=686624549 avg_ns=1605 max_ns=174267
  resolved_virtual_path_from_path: count=269787 total_ns=535605150 avg_ns=1985 max_ns=174267
  resolved_virtual_path_from_path_component_walk: count=269787 total_ns=446752886 avg_ns=1655 max_ns=173847
  resolved_virtual_path_from_path_canonicalize: count=382318 total_ns=347984369 avg_ns=910 max_ns=148978
  resolved_virtual_path_from_path_source_root_confinement: count=382318 total_ns=53022642 avg_ns=138 max_ns=171755
  resolved_virtual_path_from_path_virtual_conversion: count=269787 total_ns=73868201 avg_ns=273 max_ns=107472
  resolved_virtual_path_from_open_fd: count=157879 total_ns=151019399 avg_ns=956 max_ns=96862
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
