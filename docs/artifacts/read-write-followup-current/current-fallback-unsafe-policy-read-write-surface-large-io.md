# ScreenFS benchmark result

- timestamp: `2026-07-01T05:14:51.149832+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 5 --warmups 2 --read-mib 64 --write-mib 64 --small-io-bytes 1024 --small-io-ops 512 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 512 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-io.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+73 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+73 more)`
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
- iterations: `5`, warmups: `2`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.006411 | 0.036368 | 5.673 | 0.037327 | 0.037509 | 0.037655 |
| seq_write | 0.008678 | 0.025328 | 2.918 | 0.026139 | 0.026206 | 0.026259 |
| small_read | 0.000155 | 0.006637 | 42.762 | 0.006756 | 0.006776 | 0.006792 |
| small_write | 0.000204 | 0.019322 | 94.806 | 0.019862 | 0.019874 | 0.019883 |
| rand_read_4k | 0.000235 | 0.014880 | 63.339 | 0.015171 | 0.015183 | 0.015193 |
| rand_write_4k | 0.000322 | 0.020364 | 63.294 | 0.020654 | 0.020654 | 0.020655 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=73201 avg_ns=73201 max_ns=73201
  fuse_op.create: count=21 total_ns=1300762 avg_ns=61941 max_ns=88913
  fuse_op.flush: count=21 total_ns=624718 avg_ns=29748 max_ns=133039
  fuse_op.getattr: count=7652 total_ns=60687372 avg_ns=7930 max_ns=34579
  fuse_op.getxattr: count=7623 total_ns=96537167 avg_ns=12663 max_ns=35778
  fuse_op.lookup: count=236 total_ns=2159212 avg_ns=9149 max_ns=51137
  fuse_op.open: count=21 total_ns=266243 avg_ns=12678 max_ns=17109
  fuse_op.read: count=7126 total_ns=127434567 avg_ns=17883 max_ns=149603
  fuse_op.release: count=42 total_ns=55089 avg_ns=1311 max_ns=2529
  fuse_op.setattr: count=7 total_ns=194177 avg_ns=27739 max_ns=30519
  fuse_op.statfs: count=2 total_ns=2187 avg_ns=1093 max_ns=1150
  fuse_op.unlink: count=21 total_ns=14963559 avg_ns=712550 max_ns=2075947
  fuse_op.write: count=7616 total_ns=197942998 avg_ns=25990 max_ns=322545
  policy_decision: count=83679 total_ns=40629204 avg_ns=485 max_ns=25841
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
  matcher_candidate_order.descendant: count=68286 total_ns=9597831 avg_ns=140 max_ns=6625
  matcher_candidate_order.path: count=235644 total_ns=41150076 avg_ns=174 max_ns=20270
  matcher_candidate_order_by_source.hidden.path: count=68286 total_ns=15720536 avg_ns=230 max_ns=17963
  matcher_candidate_order_by_source.internal_hidden.path: count=68286 total_ns=10154126 avg_ns=148 max_ns=16415
  matcher_candidate_order_by_source.readonly.path: count=15393 total_ns=3441765 avg_ns=223 max_ns=14667
  matcher_candidate_order_by_source.visible.descendant: count=68286 total_ns=9597831 avg_ns=140 max_ns=6625
  matcher_candidate_order_by_source.visible.path: count=68286 total_ns=9692890 avg_ns=141 max_ns=20270
  matcher_candidate_order_by_source.writable.path: count=15393 total_ns=2140759 avg_ns=139 max_ns=4944
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=83679
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=83679
  matcher_candidate_order_ancestor_steps: count=1213904
  matcher_candidate_order_ancestor_steps.descendant: count=272732
  matcher_candidate_order_ancestor_steps.path: count=941172
  state_read_lock_wait: count=30345 total_ns=536382 avg_ns=17 max_ns=588
  state_read_lock_hold: count=30345 total_ns=2259167 avg_ns=74 max_ns=26576
  state_write_lock_wait: count=318 total_ns=7259 avg_ns=22 max_ns=243
  state_write_lock_hold: count=318 total_ns=187244 avg_ns=588 max_ns=3705
  open_confined_openat2: count=38193 total_ns=19807965 avg_ns=518 max_ns=93469
  open_like.pre_open_guard.access: count=1 total_ns=57328 avg_ns=57328 max_ns=57328
  open_like.pre_open_guard.open: count=21 total_ns=165899 avg_ns=7899 max_ns=9881
  open_like.post_open_revalidation.access: count=1 total_ns=10738 avg_ns=10738 max_ns=10738
  open_like.post_open_revalidation.open: count=21 total_ns=63879 avg_ns=3041 max_ns=4452
  stat_child_no_follow: count=30499 total_ns=29967692 avg_ns=982 max_ns=97382
  stat_child_no_follow.attr_conversion: count=30392 total_ns=396686 avg_ns=13 max_ns=136
  stat_child_no_follow.host_fstat: count=30392 total_ns=4467088 avg_ns=146 max_ns=8756
  stat_child_no_follow_context.path_guard_or_metadata: count=30499 total_ns=29967692 avg_ns=982 max_ns=97382
  source_root_path: count=38045 total_ns=44538191 avg_ns=1170 max_ns=26592
  resolved_virtual_path: count=52870 total_ns=126855061 avg_ns=2399 max_ns=55552
  resolved_virtual_path_from_path: count=30413 total_ns=107316714 avg_ns=3528 max_ns=55552
  resolved_virtual_path_from_path_component_walk: count=30413 total_ns=95082725 avg_ns=3126 max_ns=55180
  resolved_virtual_path_from_path_canonicalize: count=90876 total_ns=78132565 avg_ns=859 max_ns=54717
  resolved_virtual_path_from_path_source_root_confinement: count=90876 total_ns=9723410 avg_ns=106 max_ns=9382
  resolved_virtual_path_from_path_virtual_conversion: count=30413 total_ns=10761868 avg_ns=353 max_ns=13799
  resolved_virtual_path_from_open_fd: count=22457 total_ns=19538347 avg_ns=870 max_ns=9817
  read_handle_snapshot: count=7126 total_ns=1347682 avg_ns=189 max_ns=1886
  read_guard_path: count=7126 total_ns=76268320 avg_ns=10702 max_ns=116938
  read_io: count=7126 total_ns=48573076 avg_ns=6816 max_ns=134435
  write_handle_snapshot: count=7616 total_ns=1514772 avg_ns=198 max_ns=26710
  write_guard_mutation: count=7616 total_ns=126763217 avg_ns=16644 max_ns=72765
  write_io: count=7616 total_ns=68239078 avg_ns=8959 max_ns=288249
  file_sync.flush: count=21 total_ns=614922 avg_ns=29282 max_ns=132630
  read_size_bucket.0_4k: count=3486 total_ns=1980366 avg_ns=568 max_ns=11243
  read_size_bucket.4k_64k: count=21 total_ns=83339 avg_ns=3968 max_ns=12164
  read_size_bucket.64k_1m: count=3619 total_ns=46509371 avg_ns=12851 max_ns=134435
  write_size_bucket.0_4k: count=7168 total_ns=6551174 avg_ns=913 max_ns=15886
  write_size_bucket.64k_1m: count=448 total_ns=61687904 avg_ns=137696 max_ns=288249
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
  invalidations: count=42 invalidated_entries=21 evicted_entries=0 scanned_entries=126
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
