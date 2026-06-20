# ScreenFS benchmark result

- timestamp: `2026-06-20T09:38:50.700710+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --build --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --output-json docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface.json --output-md docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface.md --output-svg docs/artifacts/read-write-small-io-guard-reuse/after-fallback-read-write-surface.svg`
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
| seq_read | 0.007607 | 0.036612 | 4.813 | 0.036771 | 0.036896 | 0.036997 |
| seq_write | 0.007956 | 0.025178 | 3.165 | 0.025649 | 0.026282 | 0.026788 |
| small_read | 0.000342 | 0.015559 | 45.515 | 0.016172 | 0.016315 | 0.016429 |
| small_write | 0.000636 | 0.035811 | 56.348 | 0.037371 | 0.037904 | 0.038330 |
| rand_read_4k | 0.011711 | 0.405002 | 34.583 | 0.423764 | 0.425331 | 0.426584 |
| rand_write_4k | 0.012319 | 0.680022 | 55.199 | 0.812274 | 0.831931 | 0.847657 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
