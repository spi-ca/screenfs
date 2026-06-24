# ScreenFS benchmark result

- timestamp: `2026-06-24T03:49:33.272672+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3 --output-json /tmp/screenfs-no-writable-fastpath-refresh/before-policy-heavy-matrix.json --output-md /tmp/screenfs-no-writable-fastpath-refresh/before-policy-heavy-matrix.md --output-svg /tmp/screenfs-no-writable-fastpath-refresh/before-policy-heavy-matrix.svg --policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- harness_repo_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `367b50287307eb569df4a3d08e42e6db25bf4e7cd1b9db1e3aa5b8fe2be4e673`
- screenfs_source_root: `/tmp/screenfs-nowritable-refresh-before-120467`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git: `8720eb8d82ba11500b9efb9588ac17cb4f3d9de8`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-all-five-performance-completion-audit.md;  M docs/artifacts/current-all-five-retry-completion-audit.md;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-frequency-completion-audit.md;  M docs/artifacts/current-performance-followup-completion-audit.md; ... (+12 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy-matcher32`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `policy-heavy-matrix`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_access`
- screenfs_only_workloads: `matcher_hidden_stat_miss, matcher_readonly_access_wok`
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
| metadata_lookup | 0.001964 | 0.009134 | 4.650 | 0.012497 | 0.012960 | 0.013330 |
| metadata_getattr | 0.002247 | 0.012103 | 5.385 | 0.012591 | 0.012681 | 0.012754 |
| metadata_access | 0.001527 | 0.012897 | 8.447 | 0.013373 | 0.013436 | 0.013487 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.007700 | 0.008662 | 0.008870 | 0.009036 |
| matcher_readonly_access_wok | 0.019292 | 0.019981 | 0.020006 | 0.020027 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=5201 total_ns=99954079 avg_ns=19218 max_ns=111081
  fuse_op.getattr: count=3017 total_ns=33932543 avg_ns=11247 max_ns=119111
  fuse_op.lookup: count=43269 total_ns=383917207 avg_ns=8872 max_ns=273764
  fuse_op.statfs: count=2 total_ns=2319 avg_ns=1159 max_ns=1173
  policy_decision: count=59288 total_ns=68696925 avg_ns=1158 max_ns=112428
  matcher_candidates: count=614320
  matcher_candidates_by_source.hidden.path: count=2600
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=2600
  matcher_candidates_by_source.visible.descendant: count=609120
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=614320
  matcher_candidate_order.descendant: count=56688 total_ns=25689076 avg_ns=453 max_ns=50860
  matcher_candidate_order.path: count=175264 total_ns=72120656 avg_ns=411 max_ns=262042
  matcher_candidate_order_by_source.hidden.path: count=56688 total_ns=32001649 avg_ns=564 max_ns=80418
  matcher_candidate_order_by_source.internal_hidden.path: count=56688 total_ns=7932009 avg_ns=139 max_ns=7970
  matcher_candidate_order_by_source.readonly.path: count=2600 total_ns=2596326 avg_ns=998 max_ns=49809
  matcher_candidate_order_by_source.visible.descendant: count=56688 total_ns=25689076 avg_ns=453 max_ns=50860
  matcher_candidate_order_by_source.visible.path: count=56688 total_ns=29089303 avg_ns=513 max_ns=262042
  matcher_candidate_order_by_source.writable.path: count=2600 total_ns=501369 avg_ns=192 max_ns=537
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=5641224
  matcher_candidate_order_seen_slots.descendant: count=1814016
  matcher_candidate_order_seen_slots.path: count=3827208
  matcher_candidate_order_ancestor_steps: count=806492
  matcher_candidate_order_ancestor_steps.descendant: count=195123
  matcher_candidate_order_ancestor_steps.path: count=611369
  state_read_lock_wait: count=51487 total_ns=942048 avg_ns=18 max_ns=208
  state_read_lock_hold: count=51487 total_ns=2965852 avg_ns=57 max_ns=18714
  state_write_lock_wait: count=38067 total_ns=665041 avg_ns=17 max_ns=183
  state_write_lock_hold: count=38067 total_ns=11429401 avg_ns=300 max_ns=52609
  open_confined_openat2: count=54088 total_ns=29453569 avg_ns=544 max_ns=68340
  open_like.pre_open_guard.access: count=5201 total_ns=83132109 avg_ns=15983 max_ns=110779
  open_like.post_open_revalidation.access: count=2601 total_ns=13597847 avg_ns=5227 max_ns=67892
  stat_child_no_follow: count=51487 total_ns=50078631 avg_ns=972 max_ns=68911
  stat_child_no_follow.attr_conversion: count=48885 total_ns=670863 avg_ns=13 max_ns=230
  stat_child_no_follow.host_fstat: count=48885 total_ns=7010850 avg_ns=143 max_ns=5757
  stat_child_no_follow_context.path_guard_or_metadata: count=51487 total_ns=50078631 avg_ns=972 max_ns=68911
  source_root_path: count=48887 total_ns=63653710 avg_ns=1302 max_ns=71865
  resolved_virtual_path: count=48886 total_ns=138805938 avg_ns=2839 max_ns=225126
  resolved_virtual_path_from_path: count=46285 total_ns=136478320 avg_ns=2948 max_ns=225126
  resolved_virtual_path_from_path_component_walk: count=46285 total_ns=119094146 avg_ns=2573 max_ns=95832
  resolved_virtual_path_from_path_canonicalize: count=104633 total_ns=94221918 avg_ns=900 max_ns=76324
  resolved_virtual_path_from_path_source_root_confinement: count=104633 total_ns=15579716 avg_ns=148 max_ns=46893
  resolved_virtual_path_from_path_virtual_conversion: count=46285 total_ns=14992524 avg_ns=323 max_ns=222911
  resolved_virtual_path_from_open_fd: count=2601 total_ns=2327618 avg_ns=894 max_ns=10294
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
