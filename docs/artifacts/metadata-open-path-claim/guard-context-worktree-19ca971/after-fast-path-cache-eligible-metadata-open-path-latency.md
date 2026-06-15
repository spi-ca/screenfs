# ScreenFS benchmark result

- timestamp: `2026-06-15T20:23:51.309782+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target-noperf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 1024 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target-noperf/release/screenfs`
- screenfs_bin_sha256: `e71288f3c75060417dda70e3d3396ed7c6db6bc26d2b3f52eb54daea16a033a9`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `False`
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
| metadata_lookup | 0.006161 | 0.088799 | 14.412 | 0.089431 | 0.090789 | 0.091875 |
| metadata_getattr | 0.006787 | 0.095965 | 14.140 | 0.099674 | 0.100045 | 0.100342 |
| metadata_open | 0.006576 | 0.105461 | 16.038 | 0.110453 | 0.112323 | 0.113820 |
| metadata_readlink | 0.001036 | 0.083007 | 80.087 | 0.087354 | 0.088639 | 0.089666 |
| metadata_access | 0.006204 | 0.101679 | 16.389 | 0.105545 | 0.107246 | 0.108606 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
