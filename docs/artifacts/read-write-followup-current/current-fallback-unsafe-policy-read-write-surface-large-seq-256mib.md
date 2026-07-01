# ScreenFS benchmark result

- timestamp: `2026-07-01T05:14:54.209210+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --small-io-bytes 1024 --small-io-ops 512 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 512 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+76 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+76 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
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
| seq_read | 0.045144 | 0.164328 | 3.640 | 0.164402 | 0.164411 | 0.164419 |
| seq_write | 0.043584 | 0.153816 | 3.529 | 0.155637 | 0.155865 | 0.156047 |
| small_read | 0.000204 | 0.008585 | 42.178 | 0.009478 | 0.009590 | 0.009679 |
| small_write | 0.000307 | 0.021058 | 68.492 | 0.021118 | 0.021126 | 0.021132 |
| rand_read_4k | 0.000663 | 0.015873 | 23.926 | 0.016167 | 0.016204 | 0.016233 |
| rand_write_4k | 0.000562 | 0.021199 | 37.693 | 0.021732 | 0.021798 | 0.021852 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=50518 avg_ns=50518 max_ns=50518
  fuse_op.create: count=12 total_ns=799348 avg_ns=66612 max_ns=95751
  fuse_op.flush: count=12 total_ns=906490 avg_ns=75540 max_ns=173476
  fuse_op.getattr: count=5141 total_ns=50910281 avg_ns=9902 max_ns=77979
  fuse_op.getxattr: count=5124 total_ns=81349324 avg_ns=15876 max_ns=277907
  fuse_op.lookup: count=137 total_ns=1659137 avg_ns=12110 max_ns=59626
  fuse_op.open: count=12 total_ns=195596 avg_ns=16299 max_ns=22279
  fuse_op.read: count=10256 total_ns=336681385 avg_ns=32827 max_ns=176009
  fuse_op.release: count=24 total_ns=52226 avg_ns=2176 max_ns=4647
  fuse_op.setattr: count=4 total_ns=121735 avg_ns=30433 max_ns=31737
  fuse_op.statfs: count=2 total_ns=7305 avg_ns=3652 max_ns=6485
  fuse_op.unlink: count=12 total_ns=47229035 avg_ns=3935752 max_ns=11971434
  fuse_op.write: count=5120 total_ns=298882490 avg_ns=58375 max_ns=633032
  policy_decision: count=67100 total_ns=37102548 avg_ns=552 max_ns=16804
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
  matcher_candidate_order.descendant: count=56768 total_ns=9160740 avg_ns=161 max_ns=12094
  matcher_candidate_order.path: count=190968 total_ns=38526789 avg_ns=201 max_ns=248179
  matcher_candidate_order_by_source.hidden.path: count=56768 total_ns=15641025 avg_ns=275 max_ns=248179
  matcher_candidate_order_by_source.internal_hidden.path: count=56768 total_ns=9799987 avg_ns=172 max_ns=15177
  matcher_candidate_order_by_source.readonly.path: count=10332 total_ns=2727937 avg_ns=264 max_ns=48443
  matcher_candidate_order_by_source.visible.descendant: count=56768 total_ns=9160740 avg_ns=161 max_ns=12094
  matcher_candidate_order_by_source.visible.path: count=56768 total_ns=8811890 avg_ns=155 max_ns=14145
  matcher_candidate_order_by_source.writable.path: count=10332 total_ns=1545950 avg_ns=149 max_ns=9559
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=67100
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=67100
  matcher_candidate_order_ancestor_steps: count=989884
  matcher_candidate_order_ancestor_steps.descendant: count=226831
  matcher_candidate_order_ancestor_steps.path: count=763053
  state_read_lock_wait: count=25831 total_ns=623341 avg_ns=24 max_ns=623
  state_read_lock_hold: count=25831 total_ns=3654606 avg_ns=141 max_ns=4670
  state_write_lock_wait: count=183 total_ns=6316 avg_ns=34 max_ns=294
  state_write_lock_hold: count=183 total_ns=164947 avg_ns=901 max_ns=6983
  open_confined_openat2: count=31084 total_ns=29236302 avg_ns=940 max_ns=264380
  open_like.pre_open_guard.access: count=1 total_ns=42032 avg_ns=42032 max_ns=42032
  open_like.pre_open_guard.open: count=12 total_ns=116541 avg_ns=9711 max_ns=12951
  open_like.post_open_revalidation.access: count=1 total_ns=4163 avg_ns=4163 max_ns=4163
  open_like.post_open_revalidation.open: count=12 total_ns=42569 avg_ns=3547 max_ns=4924
  stat_child_no_follow: count=25919 total_ns=42721595 avg_ns=1648 max_ns=28182
  stat_child_no_follow.attr_conversion: count=25857 total_ns=397309 avg_ns=15 max_ns=181
  stat_child_no_follow.host_fstat: count=25857 total_ns=5704930 avg_ns=220 max_ns=13746
  stat_child_no_follow_context.path_guard_or_metadata: count=25919 total_ns=42721595 avg_ns=1648 max_ns=28182
  source_root_path: count=30999 total_ns=57026158 avg_ns=1839 max_ns=48371
  resolved_virtual_path: count=46422 total_ns=130961701 avg_ns=2821 max_ns=69483
  resolved_virtual_path_from_path: count=25869 total_ns=108934498 avg_ns=4211 max_ns=69483
  resolved_virtual_path_from_path_component_walk: count=25869 total_ns=94668147 avg_ns=3659 max_ns=69033
  resolved_virtual_path_from_path_canonicalize: count=77397 total_ns=77715402 avg_ns=1004 max_ns=68469
  resolved_virtual_path_from_path_source_root_confinement: count=77397 total_ns=9062105 avg_ns=117 max_ns=13820
  resolved_virtual_path_from_path_virtual_conversion: count=25869 total_ns=12654088 avg_ns=489 max_ns=18530
  resolved_virtual_path_from_open_fd: count=20553 total_ns=22027203 avg_ns=1071 max_ns=19037
  read_handle_snapshot: count=10256 total_ns=2442679 avg_ns=238 max_ns=2212
  read_guard_path: count=10256 total_ns=141550908 avg_ns=13801 max_ns=77776
  read_io: count=10256 total_ns=190233430 avg_ns=18548 max_ns=151338
  write_handle_snapshot: count=5120 total_ns=1582070 avg_ns=308 max_ns=2971
  write_guard_mutation: count=5120 total_ns=105549799 avg_ns=20615 max_ns=68054
  write_io: count=5120 total_ns=190343379 avg_ns=37176 max_ns=595593
  file_sync.flush: count=12 total_ns=894949 avg_ns=74579 max_ns=172011
  read_size_bucket.0_4k: count=2032 total_ns=1607812 avg_ns=791 max_ns=14368
  read_size_bucket.4k_64k: count=12 total_ns=102383 avg_ns=8531 max_ns=16734
  read_size_bucket.64k_1m: count=8212 total_ns=188523235 avg_ns=22957 max_ns=151338
  write_size_bucket.0_4k: count=4096 total_ns=4536834 avg_ns=1107 max_ns=65452
  write_size_bucket.64k_1m: count=1024 total_ns=185806545 avg_ns=181451 max_ns=595593
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
  invalidations: count=24 invalidated_entries=12 evicted_entries=0 scanned_entries=72
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
