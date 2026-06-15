# ScreenFS benchmark result

- timestamp: `2026-06-15T22:44:43.994862+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-postmeta-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set read-only-close-surface --iterations 10 --warmups 3 --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-fast-path-cache-eligible-read-only-close-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-after-postmeta-target/release/screenfs`
- screenfs_bin_sha256: `950112713931c9472410c5fb83a4454d0e94e087269a0fd80dcbbdd48dd4e8fe`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git_dirty_status: `M README.md;  M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-boxplot.png; ... (+21 more)`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- workload_set: `read-only-close-surface`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `read_only_open_close, read_only_open_read_close, write_open_write_close, write_open_fsync_close`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| read_only_open_close | 0.028893 | 0.469445 | 16.248 | 0.471966 | 0.472915 | 0.473674 |
| read_only_open_read_close | 0.031878 | 0.595014 | 18.666 | 0.600258 | 0.602246 | 0.603836 |
| write_open_write_close | 0.000591 | 0.024904 | 42.156 | 0.025746 | 0.026233 | 0.026624 |
| write_open_fsync_close | 0.000636 | 0.027351 | 43.009 | 0.028717 | 0.028966 | 0.029165 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
