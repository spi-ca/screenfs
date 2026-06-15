# ScreenFS benchmark result

- timestamp: `2026-06-15T22:44:13.547055+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-after-postmeta-target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --workload-set directory-surface --iterations 10 --warmups 3 --output-json docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface.json --output-md docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface.md --output-svg docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/after-glob-matcher-heavy-directory-surface.svg`
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
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `directory-surface`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `readdir_basic, readdirplus_basic`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| readdir_basic | 0.001062 | 0.040325 | 37.961 | 0.041083 | 0.041182 | 0.041261 |
| readdirplus_basic | 0.007833 | 0.605122 | 77.255 | 0.614008 | 0.615694 | 0.617042 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
