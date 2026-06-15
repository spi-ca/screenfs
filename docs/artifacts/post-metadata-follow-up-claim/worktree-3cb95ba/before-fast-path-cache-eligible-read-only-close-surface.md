# ScreenFS benchmark result

- timestamp: `2026-06-15T22:10:34.950440+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-postmeta-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-postmeta-3cb95ba --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set read-only-close-surface --iterations 10 --warmups 3 --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-read-only-close-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- git_dirty_status: `M docs/artifacts/current-perf-counter-benchmark-result.json;  M docs/artifacts/current-perf-counter-benchmark-result.md;  M docs/artifacts/current-perf-counter-benchmark-result.svg;  M docs/artifacts/managed-fio-attribution-env.json;  M docs/artifacts/managed-fio-attribution-native.json; ... (+18 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-postmeta-target/release/screenfs`
- screenfs_bin_sha256: `b219e2a9cab23b2759fdd24c2b57d39bffdd184d353817a5b40b6ca4f21beba2`
- screenfs_source_root: `/tmp/screenfs-before-postmeta-3cb95ba`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `3cb95ba86f4124cb6edf4f35e8ddf65f9b485b91`
- screenfs_source_git_worktree_clean: `True`
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
| read_only_open_close | 0.027149 | 0.499734 | 18.407 | 0.501925 | 0.502819 | 0.503533 |
| read_only_open_read_close | 0.030901 | 0.661905 | 21.420 | 0.670678 | 0.671752 | 0.672612 |
| write_open_write_close | 0.000591 | 0.024523 | 41.507 | 0.025381 | 0.025569 | 0.025719 |
| write_open_fsync_close | 0.000634 | 0.027528 | 43.415 | 0.027739 | 0.027751 | 0.027761 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
