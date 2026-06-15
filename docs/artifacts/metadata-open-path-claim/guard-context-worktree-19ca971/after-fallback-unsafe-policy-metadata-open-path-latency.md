# ScreenFS benchmark result

- timestamp: `2026-06-15T20:24:15.127534+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target-noperf/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --policy-label fallback-unsafe-policy --iterations 10 --warmups 3 --metadata-ops 1024 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path-latency.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path-latency.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after-fallback-unsafe-policy-metadata-open-path-latency.svg`
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
| metadata_lookup | 0.006154 | 0.114144 | 18.547 | 0.121642 | 0.122736 | 0.123612 |
| metadata_getattr | 0.006815 | 0.136036 | 19.962 | 0.142382 | 0.142871 | 0.143263 |
| metadata_open | 0.006445 | 0.136525 | 21.182 | 0.140508 | 0.141443 | 0.142191 |
| metadata_readlink | 0.001024 | 0.124855 | 121.926 | 0.130722 | 0.130914 | 0.131067 |
| metadata_access | 0.006018 | 0.134471 | 22.346 | 0.140552 | 0.142185 | 0.143492 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
