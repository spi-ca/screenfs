# ScreenFS benchmark result

- timestamp: `2026-06-15T20:27:15.066792+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target-noperf/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path-latency-ops4096.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path-latency-ops4096.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-fallback-unsafe-policy-metadata-open-path-latency-ops4096.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.024423 | 0.468193 | 19.170 | 0.484983 | 0.488006 | 0.490425 |
| metadata_getattr | 0.027441 | 0.614519 | 22.395 | 0.626654 | 0.626915 | 0.627125 |
| metadata_open | 0.026035 | 0.623411 | 23.946 | 0.631651 | 0.632706 | 0.633551 |
| metadata_readlink | 0.004182 | 0.591145 | 141.353 | 0.594769 | 0.595493 | 0.596072 |
| metadata_access | 0.024570 | 0.614495 | 25.010 | 0.626090 | 0.630422 | 0.633888 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
