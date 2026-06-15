# ScreenFS benchmark result

- timestamp: `2026-06-15T20:49:11.689845+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 1024 --perf-counters --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path.svg`
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
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.006090 | 0.086856 | 14.263 | 0.092163 | 0.093352 | 0.094304 |
| metadata_getattr | 0.006738 | 0.105543 | 15.663 | 0.109966 | 0.111171 | 0.112135 |
| metadata_open | 0.006445 | 0.116768 | 18.117 | 0.124709 | 0.125063 | 0.125346 |
| metadata_readlink | 0.001010 | 0.094068 | 93.113 | 0.095880 | 0.095897 | 0.095911 |
| metadata_access | 0.006093 | 0.112150 | 18.406 | 0.113917 | 0.114836 | 0.115571 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.

```text
screenfs perf counters:
  fuse_op.access: count=13313 total_ns=337889210 avg_ns=25380 max_ns=157598
  fuse_op.flush: count=13312 total_ns=85282977 avg_ns=6406 max_ns=84400
  fuse_op.getattr: count=13313 total_ns=229387674 avg_ns=17230 max_ns=43903
  fuse_op.lookup: count=199685 total_ns=2996567742 avg_ns=15006 max_ns=17932164
  fuse_op.open: count=13312 total_ns=339863273 avg_ns=25530 max_ns=62228
  fuse_op.readlink: count=13312 total_ns=254020622 avg_ns=19082 max_ns=60404
  fuse_op.release: count=13312 total_ns=12169038 avg_ns=914 max_ns=14046
  fuse_op.statfs: count=2 total_ns=3066 avg_ns=1533 max_ns=1663
  policy_decision: count=545806 total_ns=182822071 avg_ns=334 max_ns=28454
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=545806 total_ns=68467646 avg_ns=125 max_ns=9889
  matcher_candidate_order.path: count=1637418 total_ns=195375717 avg_ns=119 max_ns=51829
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=6123640
  matcher_candidate_order_ancestor_steps.descendant: count=1530910
  matcher_candidate_order_ancestor_steps.path: count=4592730
  state_read_lock_wait: count=266247 total_ns=6971815 avg_ns=26 max_ns=11101
  state_read_lock_hold: count=266247 total_ns=28188776 avg_ns=105 max_ns=8286
  state_write_lock_wait: count=212995 total_ns=5389901 avg_ns=25 max_ns=6312
  state_write_lock_hold: count=212995 total_ns=115764389 avg_ns=543 max_ns=185812
  open_confined_openat2: count=279560 total_ns=337235852 avg_ns=1206 max_ns=17916785
  stat_child_no_follow: count=252935 total_ns=3109360359 avg_ns=12293 max_ns=17929850
  source_root_path: count=279559 total_ns=982735795 avg_ns=3515 max_ns=188347
  resolved_virtual_path: count=532493 total_ns=1421253788 avg_ns=2669 max_ns=191021
  resolved_virtual_path_from_path: count=252934 total_ns=898959259 avg_ns=3554 max_ns=55525
  resolved_virtual_path_from_path_component_walk: count=252934 total_ns=812310279 avg_ns=3211 max_ns=54954
  resolved_virtual_path_from_path_canonicalize: count=306181 total_ns=734455145 avg_ns=2398 max_ns=54432
  resolved_virtual_path_from_path_source_root_confinement: count=306181 total_ns=39459956 avg_ns=128 max_ns=9599
  resolved_virtual_path_from_path_virtual_conversion: count=252934 total_ns=68960207 avg_ns=272 max_ns=8666
  resolved_virtual_path_from_open_fd: count=279559 total_ns=522294529 avg_ns=1868 max_ns=191021
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  file_sync.flush: count=13312 total_ns=80948784 avg_ns=6080 max_ns=83929
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
