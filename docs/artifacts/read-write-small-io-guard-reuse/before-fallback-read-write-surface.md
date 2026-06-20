# ScreenFS benchmark result

- timestamp: `2026-06-20T09:26:15.854292+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-small-io-guard-reuse/before-fallback-read-write-surface.json --output-md docs/artifacts/read-write-small-io-guard-reuse/before-fallback-read-write-surface.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/before-fallback-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `2bf92b4e94a3118d7066f294515107375a463d34`
- git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ?? docs/artifacts/current-next-performance-candidates.md`
- git_worktree_clean: `False`
- screenfs_bin: `target/release/screenfs`
- screenfs_bin_sha256: `8bd803047ed36d8fdb22742bef56a8a57bf46f033e1fd1d5d3c810c2716c1112`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `inferred-from-screenfs-bin`
- screenfs_source_git_dirty_status: `M docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md; ?? docs/artifacts/current-next-performance-candidates.md`
- screenfs_source_git: `2bf92b4e94a3118d7066f294515107375a463d34`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.015024 | 0.060276 | 4.012 | 0.062087 | 0.062672 | 0.063140 |
| seq_write | 0.013531 | 0.045106 | 3.334 | 0.048023 | 0.048449 | 0.048790 |
| small_read | 0.001131 | 0.028163 | 24.894 | 0.031135 | 0.031451 | 0.031704 |
| small_write | 0.001480 | 0.044763 | 30.252 | 0.049514 | 0.049856 | 0.050129 |
| rand_read_4k | 0.026373 | 0.455686 | 17.278 | 0.487772 | 0.521313 | 0.548147 |
| rand_write_4k | 0.024767 | 0.751245 | 30.332 | 0.844364 | 0.845186 | 0.845843 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
