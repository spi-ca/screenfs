# ScreenFS benchmark result

- timestamp: `2026-07-01T05:13:43.552947+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --cache-control posix-fadvise-read-fixture --workload-set read-write-concurrency --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 4096 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+61 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+61 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-concurrency`
- comparable_workloads: `concurrent_rand_read_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `posix-fadvise-read-fixture`
- cache_control_scope: `requested non-root read-side cold-cache approximation for benchmark-owned source/.screenfs-bench read fixtures`
- cache_control_timing_applied: `True`
- cache_control_applications: `14`
- cache_control_notes: `apply os.posix_fadvise(..., POSIX_FADV_DONTNEED) to selected backing source fixture files before each warmup and measured sample`
- concurrency_workers: `4`
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| concurrent_rand_read_write_4k | 0.226313 | 1.125759 | 4.974 | 1.155640 | 1.157640 | 1.159241 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=64290 avg_ns=64290 max_ns=64290
  fuse_op.create: count=28 total_ns=3409866 avg_ns=121780 max_ns=160959
  fuse_op.flush: count=28 total_ns=6362713909 avg_ns=227239782 max_ns=288432497
  fuse_op.getattr: count=114689 total_ns=1697865818 avg_ns=14804 max_ns=846800
  fuse_op.getxattr: count=114716 total_ns=2711462437 avg_ns=23636 max_ns=1126688
  fuse_op.lookup: count=257 total_ns=5544377 avg_ns=21573 max_ns=81005
  fuse_op.open: count=28 total_ns=652237 avg_ns=23294 max_ns=33029
  fuse_op.read: count=111314 total_ns=7895924977 avg_ns=70933 max_ns=2477104
  fuse_op.release: count=56 total_ns=191031 avg_ns=3411 max_ns=15920
  fuse_op.setattr: count=28 total_ns=1597575 avg_ns=57056 max_ns=80559
  fuse_op.statfs: count=2 total_ns=5894 avg_ns=2947 max_ns=3939
  fuse_op.unlink: count=28 total_ns=113586165 avg_ns=4056648 max_ns=8798718
  fuse_op.write: count=114688 total_ns=3844544174 avg_ns=33521 max_ns=2302891
  policy_decision: count=1256060 total_ns=831207986 avg_ns=661 max_ns=2015403
  matcher_candidates: count=0
  matcher_candidates_by_source.hidden.path: count=0
  matcher_candidates_by_source.internal_hidden.path: count=0
  matcher_candidates_by_source.readonly.path: count=0
  matcher_candidates_by_source.visible.descendant: count=0
  matcher_candidates_by_source.visible.path: count=0
  matcher_candidates_by_source.writable.path: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=1026432 total_ns=194700809 avg_ns=189 max_ns=75365
  matcher_candidate_order.path: count=3538552 total_ns=837957300 avg_ns=236 max_ns=361642
  matcher_candidate_order_by_source.hidden.path: count=1026432 total_ns=325947217 avg_ns=317 max_ns=265876
  matcher_candidate_order_by_source.internal_hidden.path: count=1026432 total_ns=203579753 avg_ns=198 max_ns=361642
  matcher_candidate_order_by_source.readonly.path: count=229628 total_ns=72556250 avg_ns=315 max_ns=50212
  matcher_candidate_order_by_source.visible.descendant: count=1026432 total_ns=194700809 avg_ns=189 max_ns=75365
  matcher_candidate_order_by_source.visible.path: count=1026432 total_ns=193271588 avg_ns=188 max_ns=261299
  matcher_candidate_order_by_source.writable.path: count=229628 total_ns=42602492 avg_ns=185 max_ns=39940
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=1256060
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=1256060
  matcher_candidate_order_ancestor_steps: count=18257756
  matcher_candidate_order_ancestor_steps.descendant: count=4105239
  matcher_candidate_order_ancestor_steps.path: count=14152517
  state_read_lock_wait: count=455805 total_ns=18068531 avg_ns=39 max_ns=17077
  state_read_lock_hold: count=455805 total_ns=43361946 avg_ns=95 max_ns=42416
  state_write_lock_wait: count=367 total_ns=69870 avg_ns=190 max_ns=13775
  state_write_lock_hold: count=367 total_ns=572114 avg_ns=1558 max_ns=15552
  open_confined_openat2: count=570858 total_ns=510464644 avg_ns=894 max_ns=2008842
  open_like.pre_open_guard.access: count=1 total_ns=53214 avg_ns=53214 max_ns=53214
  open_like.pre_open_guard.open: count=28 total_ns=432248 avg_ns=15437 max_ns=22502
  open_like.post_open_revalidation.access: count=1 total_ns=5811 avg_ns=5811 max_ns=5811
  open_like.post_open_revalidation.open: count=28 total_ns=144117 avg_ns=5147 max_ns=9372
  stat_child_no_follow: count=456029 total_ns=732225249 avg_ns=1605 max_ns=2011149
  stat_child_no_follow.attr_conversion: count=455887 total_ns=7502279 avg_ns=16 max_ns=4480
  stat_child_no_follow.host_fstat: count=455887 total_ns=106849166 avg_ns=234 max_ns=280348
  stat_child_no_follow_context.path_guard_or_metadata: count=456029 total_ns=732225249 avg_ns=1605 max_ns=2011149
  source_root_path: count=570661 total_ns=1458454855 avg_ns=2555 max_ns=970476
  resolved_virtual_path: count=796774 total_ns=3775576646 avg_ns=4738 max_ns=2054543
  resolved_virtual_path_from_path: count=455915 total_ns=3322960741 avg_ns=7288 max_ns=2054543
  resolved_virtual_path_from_path_component_walk: count=455915 total_ns=3046711937 avg_ns=6682 max_ns=2052295
  resolved_virtual_path_from_path_canonicalize: count=1367319 total_ns=2619841240 avg_ns=1916 max_ns=2050451
  resolved_virtual_path_from_path_source_root_confinement: count=1367319 total_ns=266437407 avg_ns=194 max_ns=2013587
  resolved_virtual_path_from_path_virtual_conversion: count=455915 total_ns=249443058 avg_ns=547 max_ns=1765863
  resolved_virtual_path_from_open_fd: count=340859 total_ns=452615905 avg_ns=1327 max_ns=451421
  read_handle_snapshot: count=111314 total_ns=28599238 avg_ns=256 max_ns=30117
  read_guard_path: count=111314 total_ns=2100156630 avg_ns=18866 max_ns=2073864
  read_io: count=111314 total_ns=5740110574 avg_ns=51566 max_ns=2452054
  write_handle_snapshot: count=114688 total_ns=30088290 avg_ns=262 max_ns=164567
  write_guard_mutation: count=114688 total_ns=3242552063 avg_ns=28272 max_ns=2280389
  write_io: count=114688 total_ns=545154571 avg_ns=4753 max_ns=694104
  file_sync.flush: count=28 total_ns=6362628209 avg_ns=227236721 max_ns=288428975
  read_size_bucket.0_4k: count=111167 total_ns=5728694004 avg_ns=51532 max_ns=2452054
  read_size_bucket.4k_64k: count=147 total_ns=11416570 avg_ns=77663 max_ns=535180
  write_size_bucket.0_4k: count=114688 total_ns=545154571 avg_ns=4753 max_ns=694104
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
  invalidations: count=56 invalidated_entries=28 evicted_entries=0 scanned_entries=350
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
