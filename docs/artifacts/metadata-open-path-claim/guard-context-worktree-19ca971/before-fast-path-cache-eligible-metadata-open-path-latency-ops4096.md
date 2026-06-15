# ScreenFS benchmark result

- timestamp: `2026-06-15T20:26:02.886936+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target-noperf/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path-latency-ops4096.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path-latency-ops4096.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fast-path-cache-eligible-metadata-open-path-latency-ops4096.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-guard-context-target-noperf/release/screenfs`
- screenfs_bin_sha256: `1a468fb6f4e65df843d698a79e17d99a2b3d20ec91089bc2f95c81675e38c536`
- screenfs_source_root: `/tmp/screenfs-before-guard-context-19ca971`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.023605 | 0.347583 | 14.725 | 0.354210 | 0.355825 | 0.357117 |
| metadata_getattr | 0.026020 | 0.452467 | 17.389 | 0.470496 | 0.473864 | 0.476558 |
| metadata_open | 0.025176 | 0.457995 | 18.191 | 0.481476 | 0.489662 | 0.496211 |
| metadata_readlink | 0.004039 | 0.389969 | 96.542 | 0.401498 | 0.405230 | 0.408215 |
| metadata_access | 0.023843 | 0.453971 | 19.040 | 0.463316 | 0.463746 | 0.464089 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
