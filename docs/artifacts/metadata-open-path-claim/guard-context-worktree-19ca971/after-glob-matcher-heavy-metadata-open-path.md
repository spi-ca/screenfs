# ScreenFS benchmark result

- timestamp: `2026-06-15T20:50:40.646602+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-glob-matcher-heavy-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-glob-matcher-heavy-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-glob-matcher-heavy-metadata-open-path.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target/release/screenfs`
- screenfs_bin_sha256: `9c998fe7a1356552b25afbfdd6be82fd2bf07d2a37ccae50c731d32fa9a777dd`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.006112 | 0.107419 | 17.574 | 0.108357 | 0.109394 | 0.110223 |
| metadata_getattr | 0.006938 | 0.143214 | 20.642 | 0.143546 | 0.143641 | 0.143717 |
| metadata_open | 0.006616 | 0.160094 | 24.197 | 0.163014 | 0.164349 | 0.165417 |
| metadata_readlink | 0.001058 | 0.152645 | 144.243 | 0.163085 | 0.163746 | 0.164275 |
| metadata_access | 0.006321 | 0.155433 | 24.589 | 0.157187 | 0.158417 | 0.159400 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=528358167 avg_ns=39687 max_ns=76335
  fuse_op.flush: count=13312 total_ns=89220842 avg_ns=6702 max_ns=100932
  fuse_op.getattr: count=13313 total_ns=387831680 avg_ns=29131 max_ns=46297
  fuse_op.lookup: count=199685 total_ns=4703928002 avg_ns=23556 max_ns=19961236
  fuse_op.open: count=13312 total_ns=536537727 avg_ns=40304 max_ns=77848
  fuse_op.readlink: count=13312 total_ns=532866011 avg_ns=40028 max_ns=77547
  fuse_op.release: count=13312 total_ns=15156597 avg_ns=1138 max_ns=8707
  fuse_op.statfs: count=2 total_ns=3977 avg_ns=1988 max_ns=2464
  policy_decision: count=572430 total_ns=469500257 avg_ns=820 max_ns=19927
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=572430 total_ns=74270654 avg_ns=129 max_ns=12684
  matcher_candidate_order.path: count=1717290 total_ns=517203407 avg_ns=301 max_ns=14798
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=18890190
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=18890190
  matcher_candidate_order_ancestor_steps: count=6549624
  matcher_candidate_order_ancestor_steps.descendant: count=1637406
  matcher_candidate_order_ancestor_steps.path: count=4912218
  state_read_lock_wait: count=266247 total_ns=6603670 avg_ns=24 max_ns=240
  state_read_lock_hold: count=266247 total_ns=30528276 avg_ns=114 max_ns=8115
  state_write_lock_wait: count=212995 total_ns=5344593 avg_ns=25 max_ns=8326
  state_write_lock_hold: count=212995 total_ns=120935827 avg_ns=567 max_ns=39875
  open_confined_openat2: count=279560 total_ns=358923546 avg_ns=1283 max_ns=19939965
  stat_child_no_follow: count=252935 total_ns=3338071692 avg_ns=13197 max_ns=19953912
  source_root_path: count=279559 total_ns=1009298624 avg_ns=3610 max_ns=32251
  resolved_virtual_path: count=772114 total_ns=3098114977 avg_ns=4012 max_ns=32010
  resolved_virtual_path_from_path: count=492555 total_ns=2564276748 avg_ns=5206 max_ns=32010
  resolved_virtual_path_from_path_component_walk: count=492555 total_ns=2365779475 avg_ns=4803 max_ns=31409
  resolved_virtual_path_from_path_canonicalize: count=825358 total_ns=2146825058 avg_ns=2601 max_ns=30778
  resolved_virtual_path_from_path_source_root_confinement: count=825358 total_ns=119212250 avg_ns=144 max_ns=18926
  resolved_virtual_path_from_path_virtual_conversion: count=492555 total_ns=163175782 avg_ns=331 max_ns=11742
  resolved_virtual_path_from_open_fd: count=279559 total_ns=533838229 avg_ns=1909 max_ns=14157
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=84163430 avg_ns=6322 max_ns=99849
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
  invalidations: count=0 invalidated_entries=0 evicted_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
