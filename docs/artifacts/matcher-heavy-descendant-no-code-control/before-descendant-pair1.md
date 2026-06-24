# ScreenFS benchmark result

- timestamp: `2026-06-24T05:39:33.918030+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-matcher-control --screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-descendant-same-binary-control-before-pair1 --workload-set matcher-descendant-directory --output-json docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair1.json --output-md docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair1.md --output-svg docs/artifacts/matcher-heavy-descendant-no-code-control/before-descendant-pair1.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-matcher-control`
- screenfs_bin_sha256: `5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+18 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `matcher-descendant-same-binary-control-before-pair1`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
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
| matcher_descendant_readdir | 0.000027 | 0.000611 | 22.991 | 0.000656 | 0.000691 | 0.000720 |
| matcher_descendant_readdirplus | 0.000063 | 0.003340 | 52.881 | 0.003407 | 0.003433 | 0.003453 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=101934 avg_ns=101934 max_ns=101934
  fuse_op.getattr: count=443 total_ns=6967507 avg_ns=15728 max_ns=39603
  fuse_op.lookup: count=1747 total_ns=20584180 avg_ns=11782 max_ns=73428
  fuse_op.opendir: count=26 total_ns=516894 avg_ns=19880 max_ns=30214
  fuse_op.readdir: count=26 total_ns=370434 avg_ns=14247 max_ns=23537
  fuse_op.readdirplus: count=26 total_ns=11817562 avg_ns=454521 max_ns=504793
  fuse_op.releasedir: count=26 total_ns=60661 avg_ns=2333 max_ns=3676
  fuse_op.statfs: count=2 total_ns=7240 avg_ns=3620 max_ns=3916
  policy_decision: count=4012 total_ns=7756900 avg_ns=1933 max_ns=4784
  matcher_candidates: count=53942
  matcher_candidates_by_source.hidden.path: count=3094
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.visible.descendant: count=50848
  matcher_candidates_by_source.visible.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=53942
  matcher_candidate_order.descendant: count=4012 total_ns=2075784 avg_ns=517 max_ns=4937
  matcher_candidate_order.path: count=12036 total_ns=5668973 avg_ns=471 max_ns=13548
  matcher_candidate_order_by_source.hidden.path: count=4012 total_ns=2372733 avg_ns=591 max_ns=6632
  matcher_candidate_order_by_source.internal_hidden.path: count=4012 total_ns=747869 avg_ns=186 max_ns=13548
  matcher_candidate_order_by_source.visible.descendant: count=4012 total_ns=2075784 avg_ns=517 max_ns=4937
  matcher_candidate_order_by_source.visible.path: count=4012 total_ns=2548371 avg_ns=635 max_ns=7459
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=393176
  matcher_candidate_order_seen_slots.descendant: count=128384
  matcher_candidate_order_seen_slots.path: count=264792
  matcher_candidate_order_ancestor_steps: count=68716
  matcher_candidate_order_ancestor_steps.descendant: count=17179
  matcher_candidate_order_ancestor_steps.path: count=51537
  state_read_lock_wait: count=2269 total_ns=55147 avg_ns=24 max_ns=234
  state_read_lock_hold: count=2269 total_ns=143460 avg_ns=63 max_ns=1775
  state_write_lock_wait: count=1875 total_ns=43943 avg_ns=23 max_ns=286
  state_write_lock_hold: count=1875 total_ns=988745 avg_ns=527 max_ns=39080
  open_confined_openat2: count=3180 total_ns=2285775 avg_ns=718 max_ns=10526
  open_like.pre_open_guard.access: count=1 total_ns=76894 avg_ns=76894 max_ns=76894
  open_like.pre_open_guard.opendir: count=26 total_ns=332814 avg_ns=12800 max_ns=19585
  open_like.post_open_revalidation.access: count=1 total_ns=9878 avg_ns=9878 max_ns=9878
  open_like.post_open_revalidation.opendir: count=26 total_ns=145556 avg_ns=5598 max_ns=8679
  stat_child_no_follow: count=3101 total_ns=3931797 avg_ns=1267 max_ns=17491
  stat_child_no_follow.attr_conversion: count=3099 total_ns=53476 avg_ns=17 max_ns=46
  stat_child_no_follow.host_fstat: count=3099 total_ns=657349 avg_ns=212 max_ns=2474
  stat_child_no_follow_context.path_guard_or_metadata: count=3101 total_ns=3931797 avg_ns=1267 max_ns=17491
  source_root_path: count=2321 total_ns=4331739 avg_ns=1866 max_ns=29998
  resolved_virtual_path: count=2346 total_ns=10011860 avg_ns=4267 max_ns=8117
  resolved_virtual_path_from_path: count=2267 total_ns=9901202 avg_ns=4367 max_ns=8117
  resolved_virtual_path_from_path_component_walk: count=2267 total_ns=8831619 avg_ns=3895 max_ns=6681
  resolved_virtual_path_from_path_canonicalize: count=6275 total_ns=7420698 avg_ns=1182 max_ns=5740
  resolved_virtual_path_from_path_source_root_confinement: count=6275 total_ns=670408 avg_ns=106 max_ns=849
  resolved_virtual_path_from_path_virtual_conversion: count=2267 total_ns=927461 avg_ns=409 max_ns=1348
  resolved_virtual_path_from_open_fd: count=79 total_ns=110658 avg_ns=1400 max_ns=5269
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=26 total_ns=110587 avg_ns=4253 max_ns=10787
  readdir_scan.name_child_path_materialization: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_scan.scan_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_scan: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdir_page_commit: count=26 total_ns=5084 avg_ns=195 max_ns=540
  readdirplus_directory_scan: count=26 total_ns=4994449 avg_ns=192094 max_ns=211836
  readdirplus_scan.name_child_path_materialization: count=26 total_ns=301628 avg_ns=11601 max_ns=17969
  readdirplus_scan.returned_attr_hydration: count=832 total_ns=962440 avg_ns=1156 max_ns=2538
  readdirplus_scan.returned_policy_recheck: count=832 total_ns=4435882 avg_ns=5331 max_ns=15747
  readdirplus_scan.scan_fallback_attr: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_scan.scan_visibility: count=26 total_ns=4416511 avg_ns=169865 max_ns=179327
  readdirplus_attr_generation_scan: count=858 total_ns=962440 avg_ns=1121 max_ns=2538
  readdirplus_attr_generation_entries: count=832
  readdirplus_symlink_visibility: count=26 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=26 total_ns=100089 avg_ns=3849 max_ns=11216
  readdirplus_page_commit: count=26 total_ns=434695 avg_ns=16719 max_ns=39215
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
