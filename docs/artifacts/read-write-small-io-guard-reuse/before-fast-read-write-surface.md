# ScreenFS benchmark result

- timestamp: `2026-06-20T09:26:29.131800+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-small-io-guard-reuse/before-fast-read-write-surface.json --output-md docs/artifacts/read-write-small-io-guard-reuse/before-fast-read-write-surface.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/before-fast-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ?? docs/artifacts/current-next-performance-candidates.md; ?? docs/artifacts/read-write-small-io-guard-reuse/`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `8bd803047ed36d8fdb22742bef56a8a57bf46f033e1fd1d5d3c810c2716c1112`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ?? docs/artifacts/current-next-performance-candidates.md; ?? docs/artifacts/read-write-small-io-guard-reuse/`
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
| seq_read | 0.014040 | 0.041751 | 2.974 | 0.043309 | 0.043484 | 0.043624 |
| seq_write | 0.014011 | 0.041274 | 2.946 | 0.042155 | 0.042475 | 0.042731 |
| small_read | 0.001154 | 0.017621 | 15.272 | 0.017833 | 0.017880 | 0.017919 |
| small_write | 0.001221 | 0.027061 | 22.169 | 0.027419 | 0.027507 | 0.027578 |
| rand_read_4k | 0.026649 | 0.303006 | 11.370 | 0.365460 | 0.388998 | 0.407828 |
| rand_write_4k | 0.024594 | 0.389283 | 15.828 | 0.408169 | 0.428179 | 0.444186 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
