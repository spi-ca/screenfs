# ScreenFS benchmark result

- timestamp: `2026-07-01T05:14:23.943541+00:00`
- harness_command_line: `/usr/bin/python3 scripts/bench-screenfs.py --screenfs-bin target/release/screenfs --screenfs-source-root . --perf-counters --policy-preset fast-path-cache-eligible --cache-control posix-fadvise-read-fixture --workload-set read-write-concurrency --iterations 5 --warmups 2 --read-mib 256 --write-mib 256 --small-io-bytes 4096 --small-io-ops 1024 --sync-bytes 4096 --sync-ops 128 --small-files 2000 --dir-entries 5000 --rand-io-ops 4096 --concurrency-workers 4 --open-read-close-ops 4096 --metadata-ops 512 --sync-4k-fsync-every 32 --hidden-misses 2000 --matcher-extra-rules 0 --matcher-misses 2000 --symlink-parent-mutations 2000 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-cold-cache-approx/current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+67 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- screenfs_source_root: `.`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-directory-symlink-surface-smoke.json;  M docs/artifacts/current-directory-symlink-surface-smoke.md;  M docs/artifacts/current-directory-symlink-surface-smoke.svg;  M docs/artifacts/current-matcher-descendant-directory-smoke.json;  M docs/artifacts/current-matcher-descendant-directory-smoke.md; ... (+67 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
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
| concurrent_rand_read_write_4k | 0.224925 | 0.774616 | 3.444 | 0.807760 | 0.818334 | 0.826792 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=61026 avg_ns=61026 max_ns=61026
  fuse_op.create: count=28 total_ns=3030367 avg_ns=108227 max_ns=195846
  fuse_op.flush: count=28 total_ns=5969900574 avg_ns=213210734 max_ns=265325006
  fuse_op.getattr: count=114689 total_ns=505101275 avg_ns=4404 max_ns=118000
  fuse_op.getxattr: count=114716 total_ns=1495056384 avg_ns=13032 max_ns=206772
  fuse_op.lookup: count=257 total_ns=2545768 avg_ns=9905 max_ns=38025
  fuse_op.open: count=28 total_ns=626001 avg_ns=22357 max_ns=34766
  fuse_op.read: count=111314 total_ns=5734851255 avg_ns=51519 max_ns=1022196
  fuse_op.release: count=56 total_ns=228873 avg_ns=4087 max_ns=17203
  fuse_op.setattr: count=28 total_ns=1117026 avg_ns=39893 max_ns=78428
  fuse_op.statfs: count=2 total_ns=5216 avg_ns=2608 max_ns=2911
  fuse_op.unlink: count=28 total_ns=93566414 avg_ns=3341657 max_ns=5622640
  fuse_op.write: count=114688 total_ns=627915704 avg_ns=5474 max_ns=712676
  policy_decision: count=345275 total_ns=202739092 avg_ns=587 max_ns=57977
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
  matcher_candidate_order.descendant: count=345023 total_ns=67291165 avg_ns=195 max_ns=23371
  matcher_candidate_order.path: count=1035573 total_ns=204367024 avg_ns=197 max_ns=39703
  matcher_candidate_order_by_source.hidden.path: count=345023 total_ns=65901244 avg_ns=191 max_ns=24699
  matcher_candidate_order_by_source.internal_hidden.path: count=345023 total_ns=73132014 avg_ns=211 max_ns=39703
  matcher_candidate_order_by_source.readonly.path: count=252 total_ns=42456 avg_ns=168 max_ns=508
  matcher_candidate_order_by_source.visible.descendant: count=345023 total_ns=67291165 avg_ns=195 max_ns=23371
  matcher_candidate_order_by_source.visible.path: count=345023 total_ns=65248105 avg_ns=189 max_ns=15118
  matcher_candidate_order_by_source.writable.path: count=252 total_ns=43205 avg_ns=171 max_ns=510
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=0
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=0
  matcher_candidate_order_ancestor_steps: count=5520216
  matcher_candidate_order_ancestor_steps.descendant: count=1379606
  matcher_candidate_order_ancestor_steps.path: count=4140610
  state_read_lock_wait: count=455805 total_ns=17603412 avg_ns=38 max_ns=14160
  state_read_lock_hold: count=455805 total_ns=46180785 avg_ns=101 max_ns=15895
  state_write_lock_wait: count=367 total_ns=130726 avg_ns=356 max_ns=21355
  state_write_lock_hold: count=367 total_ns=613286 avg_ns=1671 max_ns=22142
  open_confined_openat2: count=344771 total_ns=336437835 avg_ns=975 max_ns=184886
  open_like.pre_open_guard.access: count=1 total_ns=360 avg_ns=360 max_ns=360
  open_like.pre_open_guard.open: count=28 total_ns=5620 avg_ns=200 max_ns=513
  open_like.post_open_revalidation.access: count=1 total_ns=42154 avg_ns=42154 max_ns=42154
  open_like.post_open_revalidation.open: count=28 total_ns=519494 avg_ns=18553 max_ns=28700
  stat_child_no_follow: count=229942 total_ns=408193154 avg_ns=1775 max_ns=108262
  stat_child_no_follow.attr_conversion: count=229828 total_ns=4054601 avg_ns=17 max_ns=977
  stat_child_no_follow.host_fstat: count=229828 total_ns=57840578 avg_ns=251 max_ns=39235
  stat_child_no_follow_context.path_guard_or_metadata: count=229942 total_ns=408193154 avg_ns=1775 max_ns=108262
  source_root_path: count=114857 total_ns=343138654 avg_ns=2987 max_ns=148802
  resolved_virtual_path: count=114913 total_ns=171153546 avg_ns=1489 max_ns=63134
  resolved_virtual_path_from_path: count=56 total_ns=337164 avg_ns=6020 max_ns=16371
  resolved_virtual_path_from_path_component_walk: count=56 total_ns=294955 avg_ns=5267 max_ns=11734
  resolved_virtual_path_from_path_canonicalize: count=112 total_ns=223583 avg_ns=1996 max_ns=9155
  resolved_virtual_path_from_path_source_root_confinement: count=112 total_ns=43375 avg_ns=387 max_ns=1615
  resolved_virtual_path_from_path_virtual_conversion: count=56 total_ns=37027 avg_ns=661 max_ns=4438
  resolved_virtual_path_from_open_fd: count=114857 total_ns=170816382 avg_ns=1487 max_ns=63134
  read_handle_snapshot: count=111314 total_ns=27788339 avg_ns=249 max_ns=16414
  read_guard_path: count=111314 total_ns=2108639 avg_ns=18 max_ns=1134
  read_io: count=111314 total_ns=5680327077 avg_ns=51029 max_ns=1021439
  write_handle_snapshot: count=114688 total_ns=28395397 avg_ns=247 max_ns=23632
  write_guard_mutation: count=114688 total_ns=2159976 avg_ns=18 max_ns=4012
  write_io: count=114688 total_ns=573855863 avg_ns=5003 max_ns=712024
  file_sync.flush: count=28 total_ns=5969836517 avg_ns=213208447 max_ns=265322388
  read_size_bucket.0_4k: count=111167 total_ns=5668721944 avg_ns=50992 max_ns=1021439
  read_size_bucket.4k_64k: count=147 total_ns=11605133 avg_ns=78946 max_ns=746950
  write_size_bucket.0_4k: count=114688 total_ns=573855863 avg_ns=5003 max_ns=712024
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
