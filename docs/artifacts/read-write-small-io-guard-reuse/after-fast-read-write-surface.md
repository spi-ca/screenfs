# ScreenFS benchmark result

- timestamp: `2026-06-20T09:39:00.857620+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-small-io-guard-reuse/after-fast-read-write-surface.json --output-md docs/artifacts/read-write-small-io-guard-reuse/after-fast-read-write-surface.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/after-fast-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/guards.rs; ... (+3 more)`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `59f4f38d60315029c0fe41efb897d1e52033432221ffbc514ec4b270f62ac2fd`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/guards.rs; ... (+3 more)`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.004945 | 0.025323 | 5.121 | 0.025623 | 0.025736 | 0.025827 |
| seq_write | 0.008088 | 0.023448 | 2.899 | 0.024439 | 0.025285 | 0.025962 |
| small_read | 0.000316 | 0.011323 | 35.850 | 0.011878 | 0.011884 | 0.011888 |
| small_write | 0.000576 | 0.019639 | 34.106 | 0.020632 | 0.020641 | 0.020648 |
| rand_read_4k | 0.010329 | 0.226286 | 21.908 | 0.244105 | 0.258173 | 0.269427 |
| rand_write_4k | 0.012127 | 0.378054 | 31.174 | 0.434851 | 0.435556 | 0.436121 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
