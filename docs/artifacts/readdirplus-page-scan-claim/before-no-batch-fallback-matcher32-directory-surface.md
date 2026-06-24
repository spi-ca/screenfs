# ScreenFS benchmark result

- timestamp: `2026-06-24T01:33:41.877601+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/readdirplus-page-scan-claim/before-no-batch-fallback-matcher32-directory-surface.svg --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- harness_repo_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `a132ead172582dc33427c0c8142623e6edaaf4a6e201ade98854f9e035232883`
- screenfs_source_root: `/tmp/screenfs-readdirplus-batch-before-48607`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `e7e522cbbaf04290026ca5b1ed30335d5a97222b`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/config.rs;  M src/config_tests.rs; ... (+2 more)`
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
| readdir_basic | 0.001262 | 0.097027 | 76.881 | 0.098551 | 0.099798 | 0.100794 |
| readdirplus_basic | 0.007217 | 0.376314 | 52.143 | 0.436463 | 0.438301 | 0.439771 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=78455 avg_ns=78455 max_ns=78455
  fuse_op.getattr: count=65027 total_ns=689636602 avg_ns=10605 max_ns=310599
  fuse_op.lookup: count=195057 total_ns=1870578235 avg_ns=9589 max_ns=366973
  fuse_op.opendir: count=26 total_ns=502203 avg_ns=19315 max_ns=42829
  fuse_op.readdir: count=180 total_ns=1849091833 avg_ns=10272732 max_ns=26501204
  fuse_op.readdirplus: count=31 total_ns=586098161 avg_ns=18906392 max_ns=29050222
  fuse_op.releasedir: count=26 total_ns=13982641 avg_ns=537793 max_ns=860797
  fuse_op.statfs: count=2 total_ns=1799 avg_ns=899 max_ns=973
  policy_decision: count=833917 total_ns=772847234 avg_ns=926 max_ns=276864
  matcher_candidates: count=2081760
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=2081760
  matcher_candidate_order.descendant: count=833917 total_ns=248457519 avg_ns=297 max_ns=315917
  matcher_candidate_order.path: count=2501751 total_ns=826395115 avg_ns=330 max_ns=288130
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=81723866
  matcher_candidate_order_seen_slots.descendant: count=26685344
  matcher_candidate_order_seen_slots.path: count=55038522
  matcher_candidate_order_ancestor_steps: count=12560840
  matcher_candidate_order_ancestor_steps.descendant: count=3140210
  matcher_candidate_order_ancestor_steps.path: count=9420630
  state_read_lock_wait: count=260322 total_ns=5587788 avg_ns=21 max_ns=5880
  state_read_lock_hold: count=260322 total_ns=17561759 avg_ns=67 max_ns=49837
  state_write_lock_wait: count=195344 total_ns=4110500 avg_ns=21 max_ns=43981
  state_write_lock_hold: count=195344 total_ns=286589456 avg_ns=1467 max_ns=3237979
  open_confined_openat2: count=266229 total_ns=206644794 avg_ns=776 max_ns=298650
  open_like.pre_open_guard.access: count=1 total_ns=64206 avg_ns=64206 max_ns=64206
  open_like.pre_open_guard.opendir: count=26 total_ns=317965 avg_ns=12229 max_ns=30193
  open_like.post_open_revalidation.access: count=1 total_ns=8280 avg_ns=8280 max_ns=8280
  open_like.post_open_revalidation.opendir: count=26 total_ns=123004 avg_ns=4730 max_ns=7039
  stat_child_no_follow: count=265991 total_ns=352532694 avg_ns=1325 max_ns=299432
  stat_child_no_follow.attr_conversion: count=265989 total_ns=4223378 avg_ns=15 max_ns=8912
  stat_child_no_follow.host_fstat: count=265989 total_ns=56911499 avg_ns=213 max_ns=186105
  stat_child_no_follow_context.path_guard_or_metadata: count=265991 total_ns=352532694 avg_ns=1325 max_ns=299432
  source_root_path: count=260374 total_ns=423617202 avg_ns=1626 max_ns=204737
  resolved_virtual_path: count=260399 total_ns=868402059 avg_ns=3334 max_ns=359725
  resolved_virtual_path_from_path: count=260161 total_ns=867582055 avg_ns=3334 max_ns=359725
  resolved_virtual_path_from_path_component_walk: count=260161 total_ns=752045672 avg_ns=2890 max_ns=358564
  resolved_virtual_path_from_path_canonicalize: count=585269 total_ns=595836100 avg_ns=1018 max_ns=357632
  resolved_virtual_path_from_path_source_root_confinement: count=585269 total_ns=85269438 avg_ns=145 max_ns=328046
  resolved_virtual_path_from_path_virtual_conversion: count=260161 total_ns=100681770 avg_ns=386 max_ns=327084
  resolved_virtual_path_from_open_fd: count=238 total_ns=820004 avg_ns=3445 max_ns=22205
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=1662663919 avg_ns=9237021 max_ns=24573333
  readdir_scan.name_child_path_materialization: count=180 total_ns=112496816 avg_ns=624982 max_ns=1684561
  readdir_scan.scan_fallback_attr: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=180 total_ns=1262412316 avg_ns=7013401 max_ns=20326595
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=83909176 avg_ns=466162 max_ns=1392811
  readdir_page_commit: count=180 total_ns=151832100 avg_ns=843511 max_ns=3238692
  readdirplus_directory_scan: count=31 total_ns=545307047 avg_ns=17590549 max_ns=26849977
  readdirplus_scan.name_child_path_materialization: count=31 total_ns=35954478 avg_ns=1159821 max_ns=1657526
  readdirplus_scan.returned_attr_hydration: count=5828 total_ns=11004665 avg_ns=1888 max_ns=47687
  readdirplus_scan.returned_policy_recheck: count=5828 total_ns=20318418 avg_ns=3486 max_ns=20473
  readdirplus_scan.scan_fallback_attr: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=31 total_ns=440459618 avg_ns=14208374 max_ns=22628386
  readdirplus_attr_generation_scan: count=5859 total_ns=11004665 avg_ns=1878 max_ns=47687
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=27644357 avg_ns=891753 max_ns=1391419
  readdirplus_page_commit: count=31 total_ns=6753278 avg_ns=217847 max_ns=357031
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
