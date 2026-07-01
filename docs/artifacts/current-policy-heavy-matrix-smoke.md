# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:10.478064+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy-matcher32 --cache-control warm --workload-set policy-heavy-matrix --iterations 3 --warmups 1 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 200 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 32 --matcher-misses 200 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/current-policy-heavy-matrix-smoke.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/current-policy-heavy-matrix-smoke.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/current-policy-heavy-matrix-smoke.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+30 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+30 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `fallback-unsafe-policy-matcher32`
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
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.001728 | 0.012196 | 7.058 | 0.013174 | 0.013296 | 0.013394 |
| metadata_getattr | 0.001824 | 0.014858 | 8.144 | 0.015356 | 0.015418 | 0.015467 |
| metadata_access | 0.000828 | 0.016694 | 20.160 | 0.016739 | 0.016745 | 0.016749 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |
| matcher_hidden_stat_miss | 0.009867 | 0.010069 | 0.010094 | 0.010114 |
| matcher_readonly_access_wok | 0.022924 | 0.022979 | 0.022986 | 0.022991 |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1601 total_ns=37664996 avg_ns=23525 max_ns=70196
  fuse_op.getattr: count=929 total_ns=12999809 avg_ns=13993 max_ns=42709
  fuse_op.lookup: count=13317 total_ns=148162252 avg_ns=11125 max_ns=87402
  fuse_op.statfs: count=2 total_ns=6731 avg_ns=3365 max_ns=4531
  policy_decision: count=18248 total_ns=26858924 avg_ns=1471 max_ns=12208
  matcher_candidates: count=189088
  matcher_candidates_by_source.hidden.path: count=800
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=800
  matcher_candidates_by_source.visible.descendant: count=187488
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=189088
  matcher_candidate_order.descendant: count=17448 total_ns=11181080 avg_ns=640 max_ns=17433
  matcher_candidate_order.path: count=53944 total_ns=28140592 avg_ns=521 max_ns=12770
  matcher_candidate_order_by_source.hidden.path: count=17448 total_ns=12741740 avg_ns=730 max_ns=12561
  matcher_candidate_order_by_source.internal_hidden.path: count=17448 total_ns=2994445 avg_ns=171 max_ns=12770
  matcher_candidate_order_by_source.readonly.path: count=800 total_ns=837912 avg_ns=1047 max_ns=2052
  matcher_candidate_order_by_source.visible.descendant: count=17448 total_ns=11181080 avg_ns=640 max_ns=17433
  matcher_candidate_order_by_source.visible.path: count=17448 total_ns=11386085 avg_ns=652 max_ns=9936
  matcher_candidate_order_by_source.writable.path: count=800 total_ns=180410 avg_ns=225 max_ns=2438
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1736304
  matcher_candidate_order_seen_slots.descendant: count=558336
  matcher_candidate_order_seen_slots.path: count=1177968
  matcher_candidate_order_ancestor_steps: count=248204
  matcher_candidate_order_ancestor_steps.descendant: count=60051
  matcher_candidate_order_ancestor_steps.path: count=188153
  state_read_lock_wait: count=15847 total_ns=378484 avg_ns=23 max_ns=376
  state_read_lock_hold: count=15847 total_ns=1016337 avg_ns=64 max_ns=2698
  state_write_lock_wait: count=11715 total_ns=271575 avg_ns=23 max_ns=443
  state_write_lock_hold: count=11715 total_ns=4332155 avg_ns=369 max_ns=13847
  open_confined_openat2: count=16648 total_ns=13564034 avg_ns=814 max_ns=16487
  open_like.pre_open_guard.access: count=1601 total_ns=31117681 avg_ns=19436 max_ns=56638
  open_like.post_open_revalidation.access: count=801 total_ns=5268566 avg_ns=6577 max_ns=13088
  stat_child_no_follow: count=15847 total_ns=22653615 avg_ns=1429 max_ns=79132
  stat_child_no_follow.attr_conversion: count=15045 total_ns=261461 avg_ns=17 max_ns=578
  stat_child_no_follow.host_fstat: count=15045 total_ns=3530162 avg_ns=234 max_ns=42078
  stat_child_no_follow_context.path_guard_or_metadata: count=15847 total_ns=22653615 avg_ns=1429 max_ns=79132
  source_root_path: count=15047 total_ns=27540399 avg_ns=1830 max_ns=28797
  resolved_virtual_path: count=15046 total_ns=49482173 avg_ns=3288 max_ns=18786
  resolved_virtual_path_from_path: count=14245 total_ns=48530621 avg_ns=3406 max_ns=18786
  resolved_virtual_path_from_path_component_walk: count=14245 total_ns=42460023 avg_ns=2980 max_ns=18234
  resolved_virtual_path_from_path_canonicalize: count=32201 total_ns=34412020 avg_ns=1068 max_ns=17165
  resolved_virtual_path_from_path_source_root_confinement: count=32201 total_ns=4326642 avg_ns=134 max_ns=1853
  resolved_virtual_path_from_path_virtual_conversion: count=14245 total_ns=5204356 avg_ns=365 max_ns=3231
  resolved_virtual_path_from_open_fd: count=801 total_ns=951552 avg_ns=1187 max_ns=5919
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
