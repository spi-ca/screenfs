# ScreenFS benchmark result

- timestamp: `2026-06-15T20:46:50.676534+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target-noperf-rerun3/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after3-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target-noperf-rerun3/release/screenfs`
- screenfs_bin_sha256: `df73f8827a902bc9a6a9f0d0fc853025bebfb464e2cb7047cde640659d1ac86d`
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
| metadata_lookup | 0.025644 | 0.359967 | 14.037 | 0.363403 | 0.364227 | 0.364886 |
| metadata_getattr | 0.028270 | 0.385476 | 13.636 | 0.405305 | 0.415257 | 0.423218 |
| metadata_open | 0.027634 | 0.417272 | 15.100 | 0.419413 | 0.421520 | 0.423206 |
| metadata_readlink | 0.004387 | 0.332088 | 75.691 | 0.338143 | 0.339027 | 0.339734 |
| metadata_access | 0.025553 | 0.405981 | 15.888 | 0.414904 | 0.417493 | 0.419563 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
