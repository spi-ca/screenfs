# ScreenFS benchmark result

- timestamp: `2026-06-15T22:11:37.573089+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-postmeta-target/release/screenfs --screenfs-source-root /tmp/screenfs-before-postmeta-3cb95ba --policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set metadata-open-path --iterations 10 --warmups 3 --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-metadata-open-path.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-metadata-open-path.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/before-fast-path-cache-eligible-metadata-open-path.svg`
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
- workload_set: `metadata-open-path`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access, metadata_statfs`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.003109 | 0.036990 | 11.899 | 0.039546 | 0.040358 | 0.041007 |
| metadata_getattr | 0.003494 | 0.048025 | 13.743 | 0.054117 | 0.055899 | 0.057325 |
| metadata_open | 0.003259 | 0.052979 | 16.255 | 0.055859 | 0.056977 | 0.057872 |
| metadata_readlink | 0.000539 | 0.041836 | 77.562 | 0.044661 | 0.045822 | 0.046752 |
| metadata_access | 0.003091 | 0.051444 | 16.642 | 0.053279 | 0.054379 | 0.055259 |
| metadata_statfs | 0.000630 | 0.004019 | 6.376 | 0.004133 | 0.004213 | 0.004277 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
