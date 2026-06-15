# ScreenFS benchmark result

- timestamp: `2026-06-15T20:46:21.188121+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target-noperf-rerun3/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971-rerun3 --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-guard-context-target-noperf-rerun3/release/screenfs`
- screenfs_bin_sha256: `1a468fb6f4e65df843d698a79e17d99a2b3d20ec91089bc2f95c81675e38c536`
- screenfs_source_root: `/tmp/screenfs-before-guard-context-19ca971-rerun3`
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
| metadata_lookup | 0.025267 | 0.350009 | 13.852 | 0.357858 | 0.360156 | 0.361994 |
| metadata_getattr | 0.027327 | 0.450389 | 16.482 | 0.460789 | 0.461407 | 0.461902 |
| metadata_open | 0.026498 | 0.453988 | 17.133 | 0.456896 | 0.458110 | 0.459081 |
| metadata_readlink | 0.004114 | 0.389090 | 94.570 | 0.394961 | 0.397083 | 0.398781 |
| metadata_access | 0.024254 | 0.438585 | 18.083 | 0.445837 | 0.447306 | 0.448482 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
