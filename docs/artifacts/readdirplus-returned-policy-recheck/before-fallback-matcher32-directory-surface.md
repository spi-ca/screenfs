# ScreenFS benchmark result

- timestamp: `2026-06-23T19:12:09.988287+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-returned-policy-recheck/before-fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-before-12398`
- git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- git_dirty_status: `M src/fs/backing.rs`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `683aaf7307506f6e62a4e08a0cdce48ca9e9157d94bd82966ba77431a7f191aa`
- screenfs_source_root: `/tmp/screenfs-readdirplus-before-12398`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `ee98c5a6cead2ff78ce6c9d57216c34f114f52c2`
- screenfs_source_git_dirty_status: `M src/fs/backing.rs`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
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
| readdir_basic | 0.001958 | 0.175853 | 89.796 | 0.179388 | 0.180592 | 0.181555 |
| readdirplus_basic | 0.005398 | 0.336037 | 62.252 | 0.414861 | 0.421416 | 0.426659 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=89885 avg_ns=89885 max_ns=89885
  fuse_op.getattr: count=65027 total_ns=611647886 avg_ns=9406 max_ns=263558
  fuse_op.lookup: count=195057 total_ns=1662833975 avg_ns=8524 max_ns=693402
  fuse_op.opendir: count=26 total_ns=862374 avg_ns=33168 max_ns=78245
  fuse_op.readdir: count=180 total_ns=2470832625 avg_ns=13726847 max_ns=37593639
  fuse_op.readdirplus: count=31 total_ns=761434498 avg_ns=24562403 max_ns=40611374
  fuse_op.releasedir: count=26 total_ns=15745684 avg_ns=605603 max_ns=1056624
  fuse_op.statfs: count=2 total_ns=6125 avg_ns=3062 max_ns=3880
  policy_decision: count=833917 total_ns=977861484 avg_ns=1172 max_ns=385624
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=833917 total_ns=305661481 avg_ns=366 max_ns=80299
  matcher_candidate_order.path: count=2501751 total_ns=1033249397 avg_ns=413 max_ns=359795
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81723866
  matcher_candidate_order_seen_slots.descendant: count=26685344
  matcher_candidate_order_seen_slots.path: count=55038522
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=5408501 avg_ns=20 max_ns=355696
  state_read_lock_hold: count=260322 total_ns=16473415 avg_ns=63 max_ns=59138
  state_write_lock_wait: count=195344 total_ns=3605269 avg_ns=18 max_ns=7910
  state_write_lock_hold: count=195344 total_ns=338648501 avg_ns=1733 max_ns=5983471
  open_confined_openat2: count=266229 total_ns=174003191 avg_ns=653 max_ns=60566
  open_like.pre_open_guard.access: count=1 total_ns=73175 avg_ns=73175 max_ns=73175
  open_like.pre_open_guard.opendir: count=26 total_ns=584117 avg_ns=22466 max_ns=61074
  open_like.post_open_revalidation.access: count=1 total_ns=9116 avg_ns=9116 max_ns=9116
  open_like.post_open_revalidation.opendir: count=26 total_ns=191423 avg_ns=7362 max_ns=12712
  stat_child_no_follow: count=265991 total_ns=300667881 avg_ns=1130 max_ns=71173
  stat_child_no_follow.attr_conversion: count=265989 total_ns=3934871 avg_ns=14 max_ns=25683
  stat_child_no_follow.host_fstat: count=265989 total_ns=46080770 avg_ns=173 max_ns=56282
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=300667881 avg_ns=1130 max_ns=71173
  source_root_path: count=260374 total_ns=358956924 avg_ns=1378 max_ns=139636
  resolved_virtual_path: count=260399 total_ns=777854984 avg_ns=2987 max_ns=328897
  resolved_virtual_path_from_path: count=260161 total_ns=776805118 avg_ns=2985 max_ns=328897
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=673678946 avg_ns=2589 max_ns=328140
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=533049579 avg_ns=910 max_ns=327470
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=82981857 avg_ns=141 max_ns=57619
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=89690568 avg_ns=344 max_ns=57973
  resolved_virtual_path_from_open_fd: count=238 total_ns=1049866 avg_ns=4411 max_ns=19873
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=2213100283 avg_ns=12295001 max_ns=35603116
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=98845668 avg_ns=549142 max_ns=1903207
  readdir_page_commit: count=180 total_ns=217018995 avg_ns=1205661 max_ns=5984179
  readdirplus_directory_scan: count=31 total_ns=710223038 avg_ns=22910420 max_ns=38959299
  readdirplus_attr_generation_scan: count=5859 total_ns=11720307 avg_ns=2000 max_ns=27435
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=33854573 avg_ns=1092083 max_ns=1767584
  readdirplus_page_commit: count=31 total_ns=8435348 avg_ns=272108 max_ns=457900
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
