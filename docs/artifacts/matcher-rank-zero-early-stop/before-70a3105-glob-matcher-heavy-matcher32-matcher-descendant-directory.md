# ScreenFS benchmark result

- timestamp: `2026-06-21T02:12:21.162710+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-rank0-before.0mpY5o/target/release/screenfs --screenfs-source-root /tmp/screenfs-matcher-rank0-before.0mpY5o --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set matcher-descendant-directory --iterations 10 --warmups 3 --output-json /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.json --output-md /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.md --output-svg /tmp/screenfs-matcher-rank0-artifacts.pPT19N/before-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.svg`
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
- policy_label: `glob-matcher-heavy-matcher32`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- workload_set: `matcher-descendant-directory`
- comparable_workloads: `matcher_descendant_readdir, matcher_descendant_readdirplus`
- screenfs_only_workloads: `(none)`
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
| matcher_descendant_readdir | 0.000015 | 0.000961 | 62.345 | 0.001065 | 0.001101 | 0.001130 |
| matcher_descendant_readdirplus | 0.000033 | 0.004380 | 132.006 | 0.004685 | 0.004748 | 0.004798 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=93957 avg_ns=93957 max_ns=93957
  fuse_op.getattr: count=443 total_ns=9675405 avg_ns=21840 max_ns=103780
  fuse_op.lookup: count=1747 total_ns=26075223 avg_ns=14925 max_ns=123002
  fuse_op.opendir: count=26 total_ns=614960 avg_ns=23652 max_ns=42659
  fuse_op.readdir: count=26 total_ns=452027 avg_ns=17385 max_ns=51169
  fuse_op.readdirplus: count=26 total_ns=19445560 avg_ns=747906 max_ns=1250434
  fuse_op.releasedir: count=26 total_ns=108317 avg_ns=4166 max_ns=16894
  fuse_op.statfs: count=2 total_ns=1532 avg_ns=766 max_ns=867
  policy_decision: count=7112 total_ns=10165285 avg_ns=1429 max_ns=27458
  matcher_candidates: count=154742
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=154742
  matcher_candidate_order.descendant: count=7112 total_ns=2858390 avg_ns=401 max_ns=18741
  matcher_candidate_order.path: count=21336 total_ns=7471974 avg_ns=350 max_ns=25867
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=696976
  matcher_candidate_order_seen_slots.descendant: count=227584
  matcher_candidate_order_seen_slots.path: count=469392
  matcher_candidate_order_ancestor_steps: count=107136
  matcher_candidate_order_ancestor_steps.descendant: count=26784
  matcher_candidate_order_ancestor_steps.path: count=80352
  state_read_lock_wait: count=2269 total_ns=42016 avg_ns=18 max_ns=224
  state_read_lock_hold: count=2269 total_ns=168625 avg_ns=74 max_ns=3381
  state_write_lock_wait: count=1875 total_ns=32504 avg_ns=17 max_ns=163
  state_write_lock_hold: count=1875 total_ns=1090282 avg_ns=581 max_ns=135886
  open_confined_openat2: count=3180 total_ns=1617110 avg_ns=508 max_ns=25594
  open_like.pre_open_guard.access: count=1 total_ns=59673 avg_ns=59673 max_ns=59673
  open_like.pre_open_guard.opendir: count=26 total_ns=454787 avg_ns=17491 max_ns=34775
  open_like.post_open_revalidation.access: count=1 total_ns=29134 avg_ns=29134 max_ns=29134
  open_like.post_open_revalidation.opendir: count=26 total_ns=118133 avg_ns=4543 max_ns=5956
  stat_child_no_follow: count=3101 total_ns=27674163 avg_ns=8924 max_ns=95831
  source_root_path: count=3153 total_ns=3980090 avg_ns=1262 max_ns=32956
  resolved_virtual_path: count=8546 total_ns=19566478 avg_ns=2289 max_ns=84551
  resolved_virtual_path_from_path: count=5367 total_ns=16572110 avg_ns=3087 max_ns=84551
  resolved_virtual_path_from_path_component_walk: count=5367 total_ns=14627308 avg_ns=2725 max_ns=84188
  resolved_virtual_path_from_path_canonicalize: count=12780 total_ns=11807756 avg_ns=923 max_ns=83770
  resolved_virtual_path_from_path_source_root_confinement: count=12780 total_ns=1616767 avg_ns=126 max_ns=3831
  resolved_virtual_path_from_path_virtual_conversion: count=5367 total_ns=1685186 avg_ns=313 max_ns=21377
  resolved_virtual_path_from_open_fd: count=3179 total_ns=2994368 avg_ns=941 max_ns=59413
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=158162 avg_ns=6083 max_ns=17693
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=6333 avg_ns=243 max_ns=584
  readdirplus_directory_scan: count=26 total_ns=4743522 avg_ns=182443 max_ns=222773
  readdirplus_attr_generation_scan: count=858 total_ns=8819620 avg_ns=10279 max_ns=69400
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=147342 avg_ns=5667 max_ns=43907
  readdirplus_page_commit: count=26 total_ns=518311 avg_ns=19935 max_ns=135987
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
