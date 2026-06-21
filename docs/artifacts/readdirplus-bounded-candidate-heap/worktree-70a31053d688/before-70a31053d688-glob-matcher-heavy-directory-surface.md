# ScreenFS benchmark result

- timestamp: `2026-06-21T01:56:10.993014+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-readdirplus-before.UDcAvW/target/release/screenfs --screenfs-source-root /tmp/screenfs-readdirplus-before.UDcAvW --perf-counters --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-glob-matcher-heavy-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-glob-matcher-heavy-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-bounded-candidate-heap/worktree-70a31053d688/before-70a31053d688-glob-matcher-heavy-directory-surface.svg`
- harness_repo_root: `/tmp/screenfs-readdirplus-before.UDcAvW`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-readdirplus-before.UDcAvW/target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/tmp/screenfs-readdirplus-before.UDcAvW`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- workload_set: `directory-surface`
- comparable_workloads: `readdir_basic, readdirplus_basic`
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
| readdir_basic | 0.000711 | 0.064331 | 90.522 | 0.065672 | 0.065768 | 0.065844 |
| readdirplus_basic | 0.003576 | 0.402829 | 112.644 | 0.503070 | 0.525750 | 0.543895 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=81302 avg_ns=81302 max_ns=81302
  fuse_op.getattr: count=65027 total_ns=956197189 avg_ns=14704 max_ns=1127932
  fuse_op.lookup: count=195057 total_ns=2327630370 avg_ns=11933 max_ns=683549
  fuse_op.opendir: count=26 total_ns=520501 avg_ns=20019 max_ns=47842
  fuse_op.readdir: count=180 total_ns=1312065258 avg_ns=7289251 max_ns=18083795
  fuse_op.readdirplus: count=31 total_ns=429246866 avg_ns=13846673 max_ns=20741753
  fuse_op.releasedir: count=26 total_ns=10507753 avg_ns=404144 max_ns=914763
  fuse_op.statfs: count=2 total_ns=1997 avg_ns=998 max_ns=1212
  policy_decision: count=1099907 total_ns=695421605 avg_ns=632 max_ns=277130
  matcher_candidates: count=6246880
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=6246880
  matcher_candidate_order.descendant: count=1099907 total_ns=234266276 avg_ns=212 max_ns=258857
  matcher_candidate_order.path: count=3299721 total_ns=748275022 avg_ns=226 max_ns=317732
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=107790886
  matcher_candidate_order_seen_slots.descendant: count=35197024
  matcher_candidate_order_seen_slots.path: count=72593862
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=7804739 avg_ns=29 max_ns=25005
  state_read_lock_hold: count=260322 total_ns=15022673 avg_ns=57 max_ns=82114
  state_write_lock_wait: count=195344 total_ns=3453619 avg_ns=17 max_ns=5045
  state_write_lock_hold: count=195344 total_ns=216845951 avg_ns=1110 max_ns=1964161
  open_confined_openat2: count=266229 total_ns=117741489 avg_ns=442 max_ns=271375
  open_like.pre_open_guard.access: count=1 total_ns=71378 avg_ns=71378 max_ns=71378
  open_like.pre_open_guard.opendir: count=26 total_ns=382285 avg_ns=14703 max_ns=39933
  open_like.post_open_revalidation.access: count=1 total_ns=4001 avg_ns=4001 max_ns=4001
  open_like.post_open_revalidation.opendir: count=26 total_ns=85660 avg_ns=3294 max_ns=4813
  stat_child_no_follow: count=265991 total_ns=1798684859 avg_ns=6762 max_ns=1105524
  source_root_path: count=266202 total_ns=350916769 avg_ns=1318 max_ns=257488
  resolved_virtual_path: count=792379 total_ns=1383596248 avg_ns=1746 max_ns=345264
  resolved_virtual_path_from_path: count=526151 total_ns=1158405423 avg_ns=2201 max_ns=317156
  resolved_virtual_path_from_path_component_walk: count=526151 total_ns=991761188 avg_ns=1884 max_ns=316837
  resolved_virtual_path_from_path_canonicalize: count=922034 total_ns=786163862 avg_ns=852 max_ns=316561
  resolved_virtual_path_from_path_source_root_confinement: count=922034 total_ns=119266897 avg_ns=129 max_ns=26408
  resolved_virtual_path_from_path_virtual_conversion: count=526151 total_ns=141029995 avg_ns=268 max_ns=260342
  resolved_virtual_path_from_open_fd: count=266228 total_ns=225190825 avg_ns=845 max_ns=345264
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1185844364 avg_ns=6588024 max_ns=16987504
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=54025297 avg_ns=300140 max_ns=1024960
  readdir_page_commit: count=180 total_ns=105873824 avg_ns=588187 max_ns=1964282
  readdirplus_directory_scan: count=31 total_ns=367208187 avg_ns=11845425 max_ns=17832289
  readdirplus_attr_generation_scan: count=5859 total_ns=42727446 avg_ns=7292 max_ns=97694
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=16567257 avg_ns=534427 max_ns=997081
  readdirplus_page_commit: count=31 total_ns=4153713 avg_ns=133990 max_ns=276805
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
