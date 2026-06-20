# ScreenFS benchmark result

- timestamp: `2026-06-20T17:03:25.766255+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fast-path-cache-eligible --workload-set directory-surface --dir-entries 5000 --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.json --output-md docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.md --output-svg docs/artifacts/readdirplus-20k-policy-ablation/current-fast-path-cache-eligible-directory-surface-5k.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+18 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.000902 | 0.048110 | 53.341 | 0.048748 | 0.048969 | 0.049145 |
| readdirplus_basic | 0.003334 | 0.370511 | 111.135 | 0.382178 | 0.385661 | 0.388448 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=76351 avg_ns=76351 max_ns=76351
  fuse_op.getattr: count=65027 total_ns=685915947 avg_ns=10548 max_ns=327178
  fuse_op.lookup: count=195057 total_ns=1815747171 avg_ns=9308 max_ns=337131
  fuse_op.opendir: count=26 total_ns=428607 avg_ns=16484 max_ns=46548
  fuse_op.readdir: count=180 total_ns=1027860571 avg_ns=5710336 max_ns=12166796
  fuse_op.readdirplus: count=31 total_ns=327198255 avg_ns=10554782 max_ns=14124336
  fuse_op.releasedir: count=26 total_ns=12085235 avg_ns=464816 max_ns=940392
  fuse_op.statfs: count=2 total_ns=5725 avg_ns=2862 max_ns=3465
  policy_decision: count=1099907 total_ns=391038083 avg_ns=355 max_ns=113083
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1099907 total_ns=139152394 avg_ns=126 max_ns=331463
  matcher_candidate_order.path: count=3299721 total_ns=434284773 avg_ns=131 max_ns=258563
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=14971860
  matcher_candidate_order_ancestor_steps.descendant: count=3742965
  matcher_candidate_order_ancestor_steps.path: count=11228895
  state_read_lock_wait: count=260322 total_ns=5706188 avg_ns=21 max_ns=6614
  state_read_lock_hold: count=260322 total_ns=17162224 avg_ns=65 max_ns=9646
  state_write_lock_wait: count=195344 total_ns=4099769 avg_ns=20 max_ns=696
  state_write_lock_hold: count=195344 total_ns=244605170 avg_ns=1252 max_ns=1956763
  open_confined_openat2: count=266229 total_ns=150626708 avg_ns=565 max_ns=71063
  open_like.pre_open_guard.access: count=1 total_ns=31947 avg_ns=31947 max_ns=31947
  open_like.pre_open_guard.opendir: count=26 total_ns=303177 avg_ns=11660 max_ns=39569
  open_like.post_open_revalidation.access: count=1 total_ns=38228 avg_ns=38228 max_ns=38228
  open_like.post_open_revalidation.opendir: count=26 total_ns=68915 avg_ns=2650 max_ns=3171
  stat_child_no_follow: count=265991 total_ns=1927207007 avg_ns=7245 max_ns=324705
  source_root_path: count=266202 total_ns=434414467 avg_ns=1631 max_ns=87336
  resolved_virtual_path: count=532218 total_ns=740869212 avg_ns=1392 max_ns=317648
  resolved_virtual_path_from_path: count=265990 total_ns=481362729 avg_ns=1809 max_ns=317648
  resolved_virtual_path_from_path_component_walk: count=265990 total_ns=392884948 avg_ns=1477 max_ns=317159
  resolved_virtual_path_from_path_canonicalize: count=336765 total_ns=311071362 avg_ns=923 max_ns=316546
  resolved_virtual_path_from_path_source_root_confinement: count=336765 total_ns=41132702 avg_ns=122 max_ns=55249
  resolved_virtual_path_from_path_virtual_conversion: count=265990 total_ns=73300553 avg_ns=275 max_ns=24942
  resolved_virtual_path_from_open_fd: count=266228 total_ns=259506483 avg_ns=974 max_ns=95047
  read_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_guard_path: count=0 total_ns=0 avg_ns=0 max_ns=0
  read_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_handle_snapshot: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_guard_mutation: count=0 total_ns=0 avg_ns=0 max_ns=0
  write_io: count=0 total_ns=0 avg_ns=0 max_ns=0
  readdir_directory_scan: count=180 total_ns=891073345 avg_ns=4950407 max_ns=11246037
  readdir_attr_generation_scan: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_attr_generation_entries: count=0
  readdir_symlink_visibility: count=180 total_ns=0 avg_ns=0 max_ns=0
  readdir_candidate_selection: count=180 total_ns=56751817 avg_ns=315287 max_ns=918589
  readdir_page_commit: count=180 total_ns=114510822 avg_ns=636171 max_ns=1956916
  readdirplus_directory_scan: count=31 total_ns=271007630 avg_ns=8742181 max_ns=11703487
  readdirplus_attr_generation_scan: count=5859 total_ns=41261806 avg_ns=7042 max_ns=57488
  readdirplus_attr_generation_entries: count=5828
  readdirplus_symlink_visibility: count=31 total_ns=0 avg_ns=0 max_ns=0
  readdirplus_candidate_selection: count=31 total_ns=18006488 avg_ns=580854 max_ns=897741
  readdirplus_page_commit: count=31 total_ns=4359352 avg_ns=140624 max_ns=217289
  invalidations: count=0 invalidated_entries=0 evicted_entries=0 scanned_entries=0
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
