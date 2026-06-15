# ScreenFS benchmark result

- timestamp: `2026-06-15T20:49:40.466830+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target-noperf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency-ops4096.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency-ops4096.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fast-path-cache-eligible-metadata-open-path-latency-ops4096.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target-noperf/release/screenfs`
- screenfs_bin_sha256: `b219e2a9cab23b2759fdd24c2b57d39bffdd184d353817a5b40b6ca4f21beba2`
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
| metadata_lookup | 0.024573 | 0.295485 | 12.025 | 0.301526 | 0.303037 | 0.304245 |
| metadata_getattr | 0.027668 | 0.388141 | 14.029 | 0.396129 | 0.398023 | 0.399538 |
| metadata_open | 0.026929 | 0.421639 | 15.657 | 0.427543 | 0.431118 | 0.433978 |
| metadata_readlink | 0.004125 | 0.337201 | 81.737 | 0.340233 | 0.340348 | 0.340440 |
| metadata_access | 0.025074 | 0.415391 | 16.566 | 0.421067 | 0.422986 | 0.424521 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
