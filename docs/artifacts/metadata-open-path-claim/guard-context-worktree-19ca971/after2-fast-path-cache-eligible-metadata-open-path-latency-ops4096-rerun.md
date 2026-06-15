# ScreenFS benchmark result

- timestamp: `2026-06-15T20:44:49.005478+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-guard-context-target-noperf-rerun/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --iterations 10 --warmups 3 --metadata-ops 4096 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after2-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after2-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/after2-fast-path-cache-eligible-metadata-open-path-latency-ops4096-rerun.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-guard-context-target-noperf-rerun/release/screenfs`
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
| metadata_lookup | 0.024337 | 0.349637 | 14.367 | 0.355302 | 0.357710 | 0.359637 |
| metadata_getattr | 0.026730 | 0.377142 | 14.109 | 0.391130 | 0.391428 | 0.391667 |
| metadata_open | 0.025710 | 0.438800 | 17.067 | 0.448310 | 0.456434 | 0.462933 |
| metadata_readlink | 0.004107 | 0.343773 | 83.710 | 0.353792 | 0.362043 | 0.368643 |
| metadata_access | 0.024338 | 0.425433 | 17.480 | 0.434640 | 0.435687 | 0.436526 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
