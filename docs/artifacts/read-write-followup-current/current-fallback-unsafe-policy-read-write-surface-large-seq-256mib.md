# ScreenFS benchmark result

- timestamp: `2026-06-20T17:27:27.170802+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --perf-counters --screenfs-bin target/release/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024 --output-json docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.json --output-md docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.md --output-svg docs/artifacts/read-write-followup-current/current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59186deeaba2e91862cbf5c05f63a556346f7f0b491793bccc7d7aa145b95422`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-metadata-opendir-smoke.json;  M docs/artifacts/current-metadata-opendir-smoke.md;  M docs/artifacts/current-metadata-opendir-smoke.svg;  M docs/artifacts/current-next-performance-candidates.md;  M docs/artifacts/current-open-confined-surface-smoke.json; ... (+20 more)`
- screenfs_source_git: `920c00e4308596bc285b236ae58139b30e24b0cb`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `3`, warmups: `1`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.040765 | 0.180244 | 4.422 | 0.183739 | 0.184176 | 0.184525 |
| seq_write | 0.047254 | 0.157359 | 3.330 | 0.158262 | 0.158375 | 0.158465 |
| small_read | 0.000222 | 0.012538 | 56.477 | 0.012679 | 0.012696 | 0.012711 |
| small_write | 0.000299 | 0.028997 | 96.832 | 0.030769 | 0.030991 | 0.031168 |
| rand_read_4k | 0.000489 | 0.021927 | 44.819 | 0.022739 | 0.022840 | 0.022921 |
| rand_write_4k | 0.000612 | 0.028448 | 46.521 | 0.028829 | 0.028877 | 0.028915 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.

```text
screenfs perf counters:
  fuse_op.access: count=1 total_ns=75173 avg_ns=75173 max_ns=75173
  fuse_op.create: count=12 total_ns=1319918 avg_ns=109993 max_ns=135127
  fuse_op.flush: count=12 total_ns=816129 avg_ns=68010 max_ns=152237
  fuse_op.getattr: count=5141 total_ns=83482439 avg_ns=16238 max_ns=342463
  fuse_op.getxattr: count=5124 total_ns=116651411 avg_ns=22765 max_ns=188273
  fuse_op.lookup: count=137 total_ns=2449992 avg_ns=17883 max_ns=82755
  fuse_op.open: count=12 total_ns=282616 avg_ns=23551 max_ns=30459
  fuse_op.read: count=10256 total_ns=387806592 avg_ns=37812 max_ns=223545
  fuse_op.release: count=24 total_ns=54010 avg_ns=2250 max_ns=3878
  fuse_op.setattr: count=4 total_ns=160970 avg_ns=40242 max_ns=42875
  fuse_op.statfs: count=2 total_ns=1330 avg_ns=665 max_ns=891
  fuse_op.unlink: count=12 total_ns=42715423 avg_ns=3559618 max_ns=10667406
  fuse_op.write: count=5120 total_ns=326513239 avg_ns=63772 max_ns=449981
  policy_decision: count=93018 total_ns=46604175 avg_ns=501 max_ns=27396
  matcher_candidates: count=0
  matcher_family_candidates.direct_child_glob: count=0
  matcher_family_candidates.recursive: count=0
  matcher_family_candidates.subtree: count=0
  matcher_candidate_order.descendant: count=82686 total_ns=13288675 avg_ns=160 max_ns=348596
  matcher_candidate_order.path: count=268722 total_ns=51662319 avg_ns=192 max_ns=17471
  matcher_candidate_order_duplicates: count=0
  matcher_candidate_order_duplicates.descendant: count=0
  matcher_candidate_order_duplicates.path: count=0
  matcher_candidate_order_seen_slots: count=93018
  matcher_candidate_order_seen_slots.descendant: count=0
  matcher_candidate_order_seen_slots.path: count=93018
  matcher_candidate_order_ancestor_steps: count=1300248
  matcher_candidate_order_ancestor_steps.descendant: count=304422
  matcher_candidate_order_ancestor_steps.path: count=995826
  state_read_lock_wait: count=25831 total_ns=737829 avg_ns=28 max_ns=1531
  state_read_lock_hold: count=25831 total_ns=3680975 avg_ns=142 max_ns=24316
  state_write_lock_wait: count=183 total_ns=6554 avg_ns=35 max_ns=301
  state_write_lock_hold: count=183 total_ns=210106 avg_ns=1148 max_ns=14434
  open_confined_openat2: count=31084 total_ns=27776142 avg_ns=893 max_ns=147761
  open_like.pre_open_guard.access: count=1 total_ns=63169 avg_ns=63169 max_ns=63169
  open_like.pre_open_guard.open: count=12 total_ns=201766 avg_ns=16813 max_ns=21721
  open_like.post_open_revalidation.access: count=1 total_ns=5758 avg_ns=5758 max_ns=5758
  open_like.post_open_revalidation.open: count=12 total_ns=40915 avg_ns=3409 max_ns=4372
  stat_child_no_follow: count=25919 total_ns=262696867 avg_ns=10135 max_ns=336684
  source_root_path: count=36183 total_ns=67613663 avg_ns=1868 max_ns=57952
  resolved_virtual_path: count=98258 total_ns=233688509 avg_ns=2378 max_ns=142298
  resolved_virtual_path_from_path: count=51787 total_ns=180975701 avg_ns=3494 max_ns=142298
  resolved_virtual_path_from_path_component_walk: count=51787 total_ns=159648796 avg_ns=3082 max_ns=141523
  resolved_virtual_path_from_path_canonicalize: count=129070 total_ns=130175791 avg_ns=1008 max_ns=124430
  resolved_virtual_path_from_path_source_root_confinement: count=129070 total_ns=15940190 avg_ns=123 max_ns=19553
  resolved_virtual_path_from_path_virtual_conversion: count=51787 total_ns=18251836 avg_ns=352 max_ns=17075
  resolved_virtual_path_from_open_fd: count=46471 total_ns=52712808 avg_ns=1134 max_ns=39387
  read_handle_snapshot: count=10256 total_ns=2644730 avg_ns=257 max_ns=3451
  read_guard_path: count=10256 total_ns=206892153 avg_ns=20172 max_ns=87965
  read_io: count=10256 total_ns=175576747 avg_ns=17119 max_ns=127999
  write_handle_snapshot: count=5120 total_ns=1793992 avg_ns=350 max_ns=3217
  write_guard_mutation: count=5120 total_ns=135447353 avg_ns=26454 max_ns=374649
  write_io: count=5120 total_ns=187552169 avg_ns=36631 max_ns=382067
  file_sync.flush: count=12 total_ns=800324 avg_ns=66693 max_ns=150752
  read_size_bucket.0_4k: count=2032 total_ns=1833421 avg_ns=902 max_ns=14340
  read_size_bucket.4k_64k: count=12 total_ns=99659 avg_ns=8304 max_ns=18391
  read_size_bucket.64k_1m: count=8212 total_ns=173643667 avg_ns=21145 max_ns=127999
  write_size_bucket.0_4k: count=4096 total_ns=4589305 avg_ns=1120 max_ns=86651
  write_size_bucket.64k_1m: count=1024 total_ns=182962864 avg_ns=178674 max_ns=382067
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
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
