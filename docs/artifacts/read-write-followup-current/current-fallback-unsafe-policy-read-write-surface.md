# ScreenFS benchmark result

- timestamp: `2026-07-01T05:15:14.150713+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --read-mib 64 --write-mib 64 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 16384 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+79 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+79 more)`
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
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.006241 | 0.035840 | 5.743 | 0.036715 | 0.036871 | 0.036995 |
| seq_write | 0.008831 | 0.025099 | 2.842 | 0.025593 | 0.025644 | 0.025685 |
| small_read | 0.000369 | 0.014405 | 39.054 | 0.014672 | 0.014853 | 0.014998 |
| small_write | 0.000588 | 0.040505 | 68.937 | 0.041376 | 0.041501 | 0.041601 |
| rand_read_4k | 0.011606 | 0.390321 | 33.630 | 0.525882 | 0.534034 | 0.540555 |
| rand_write_4k | 0.012652 | 0.868498 | 68.644 | 0.883922 | 0.886873 | 0.889233 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=49413 avg_ns=49413 max_ns=49413
  fuse_op.create: count=39 total_ns=2722124 avg_ns=69798 max_ns=119230
  fuse_op.flush: count=39 total_ns=2272475 avg_ns=58268 max_ns=363290
  fuse_op.getattr: count=227202 total_ns=1917400239 avg_ns=8439 max_ns=1840811
  fuse_op.getxattr: count=227149 total_ns=3713898886 avg_ns=16350 max_ns=304493
  fuse_op.lookup: count=434 total_ns=4410754 avg_ns=10163 max_ns=55081
  fuse_op.open: count=39 total_ns=579538 avg_ns=14859 max_ns=62405
  fuse_op.read: count=148343 total_ns=1961981945 avg_ns=13225 max_ns=696247
  fuse_op.release: count=78 total_ns=154594 avg_ns=1981 max_ns=5201
  fuse_op.setattr: count=13 total_ns=475877 avg_ns=36605 max_ns=39998
  fuse_op.statfs: count=2 total_ns=3326 avg_ns=1663 max_ns=2363
  fuse_op.unlink: count=39 total_ns=63199189 avg_ns=1620492 max_ns=3422633
  fuse_op.write: count=227136 total_ns=5294301757 avg_ns=23308 max_ns=790760
  policy_decision: count=2342543 total_ns=1370127993 avg_ns=584 max_ns=121582
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
  matcher_candidate_order.descendant: count=1887972 total_ns=313006064 avg_ns=165 max_ns=19874
  matcher_candidate_order.path: count=6573058 total_ns=1384504573 avg_ns=210 max_ns=65795
  matcher_candidate_order_by_source.hidden.path: count=1887972 total_ns=538584389 avg_ns=285 max_ns=52725
  matcher_candidate_order_by_source.internal_hidden.path: count=1887972 total_ns=325340953 avg_ns=172 max_ns=65795
  matcher_candidate_order_by_source.readonly.path: count=454571 total_ns=133871229 avg_ns=294 max_ns=14781
  matcher_candidate_order_by_source.visible.descendant: count=1887972 total_ns=313006064 avg_ns=165 max_ns=19874
  matcher_candidate_order_by_source.visible.path: count=1887972 total_ns=309743424 avg_ns=164 max_ns=45621
  matcher_candidate_order_by_source.writable.path: count=454571 total_ns=76964578 avg_ns=169 max_ns=9141
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=2342543
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=2342543
  matcher_candidate_order_ancestor_steps: count=33840792
  matcher_candidate_order_ancestor_steps.descendant: count=7551134
  matcher_candidate_order_ancestor_steps.path: count=26289658
  state_read_lock_wait: count=830434 total_ns=18406637 avg_ns=22 max_ns=11739
  state_read_lock_hold: count=830434 total_ns=63349335 avg_ns=76 max_ns=57362
  state_write_lock_wait: count=588 total_ns=16710 avg_ns=28 max_ns=323
  state_write_lock_hold: count=588 total_ns=497793 avg_ns=846 max_ns=9305
  open_confined_openat2: count=1058000 total_ns=739310288 avg_ns=698 max_ns=1797659
  open_like.pre_open_guard.access: count=1 total_ns=42156 avg_ns=42156 max_ns=42156
  open_like.pre_open_guard.open: count=39 total_ns=370593 avg_ns=9502 max_ns=58096
  open_like.post_open_revalidation.access: count=1 total_ns=3607 avg_ns=3607 max_ns=3607
  open_like.post_open_revalidation.open: count=39 total_ns=119256 avg_ns=3057 max_ns=5659
  stat_child_no_follow: count=830720 total_ns=1035203428 avg_ns=1246 max_ns=1804432
  stat_child_no_follow.attr_conversion: count=830523 total_ns=13114985 avg_ns=15 max_ns=10425
  stat_child_no_follow.host_fstat: count=830523 total_ns=150330709 avg_ns=181 max_ns=20802
  stat_child_no_follow_context.path_guard_or_metadata: count=830720 total_ns=1035203428 avg_ns=1246 max_ns=1804432
  source_root_path: count=1057726 total_ns=1610270385 avg_ns=1522 max_ns=364953
  resolved_virtual_path: count=1433360 total_ns=4043376329 avg_ns=2820 max_ns=275105
  resolved_virtual_path_from_path: count=830562 total_ns=3398320375 avg_ns=4091 max_ns=275105
  resolved_virtual_path_from_path_component_walk: count=830562 total_ns=3026227067 avg_ns=3643 max_ns=274669
  resolved_virtual_path_from_path_canonicalize: count=2491017 total_ns=2502160021 avg_ns=1004 max_ns=274104
  resolved_virtual_path_from_path_source_root_confinement: count=2491017 total_ns=282662790 avg_ns=113 max_ns=40454
  resolved_virtual_path_from_path_virtual_conversion: count=830562 total_ns=325426966 avg_ns=391 max_ns=51837
  resolved_virtual_path_from_open_fd: count=602798 total_ns=645055954 avg_ns=1070 max_ns=67074
  read_handle_snapshot: count=148343 total_ns=27805386 avg_ns=187 max_ns=57453
  read_guard_path: count=148343 total_ns=1634439433 avg_ns=11017 max_ns=374925
  read_io: count=148343 total_ns=274425415 avg_ns=1849 max_ns=320989
  write_handle_snapshot: count=227136 total_ns=46651819 avg_ns=205 max_ns=16594
  write_guard_mutation: count=227136 total_ns=4609396112 avg_ns=20293 max_ns=167154
  write_io: count=227136 total_ns=596185931 avg_ns=2624 max_ns=769786
  file_sync.flush: count=39 total_ns=2236377 avg_ns=57343 max_ns=360635
  read_size_bucket.0_4k: count=126893 total_ns=149296322 avg_ns=1176 max_ns=320989
  read_size_bucket.4k_64k: count=14365 total_ns=35913972 avg_ns=2500 max_ns=56566
  read_size_bucket.64k_1m: count=7085 total_ns=89215121 avg_ns=12592 max_ns=276481
  write_size_bucket.0_4k: count=226304 total_ns=482025292 avg_ns=2129 max_ns=769786
  write_size_bucket.64k_1m: count=832 total_ns=114160639 avg_ns=137212 max_ns=278943
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
  invalidations: count=78 invalidated_entries=39 evicted_entries=0 scanned_entries=234
```

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
