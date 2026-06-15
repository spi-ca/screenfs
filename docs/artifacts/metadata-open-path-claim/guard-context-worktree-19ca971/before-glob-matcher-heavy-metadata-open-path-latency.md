# ScreenFS benchmark result

- timestamp: `2026-06-15T20:22:20.993420+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /dev/shm/screenfs-before-guard-context-target-noperf/release/screenfs --screenfs-source-root /tmp/screenfs-before-guard-context-19ca971 --policy-preset fallback-unsafe-policy --policy-label glob-matcher-heavy --matcher-extra-rules 32 --iterations 10 --warmups 3 --metadata-ops 1024 --workload metadata_lookup --workload metadata_getattr --workload metadata_open --workload metadata_readlink --workload metadata_access --output-json docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path-latency.json --output-md docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path-latency.md --output-svg docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/before-glob-matcher-heavy-metadata-open-path-latency.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- git_dirty_status: `M docs/artifacts/metadata-open-path-claim/summary.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/backing.rs; ... (+11 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/dev/shm/screenfs-before-guard-context-target-noperf/release/screenfs`
- screenfs_bin_sha256: `1a468fb6f4e65df843d698a79e17d99a2b3d20ec91089bc2f95c81675e38c536`
- screenfs_source_root: `/tmp/screenfs-before-guard-context-19ca971`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `19ca971446a4d298da689c813946dfbfa7d5c6ab`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `custom-unsafe-policy`
- policy_label: `glob-matcher-heavy`
- fast_path_cache_eligible: `False`
- workload_selection: `explicit`
- workload_set: `explicit`
- policy_notes: `matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected`
- comparable_workloads: `metadata_lookup, metadata_getattr, metadata_open, metadata_readlink, metadata_access`
- screenfs_only_workloads: `(none)`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| metadata_lookup | 0.006226 | 0.119288 | 19.160 | 0.125407 | 0.126750 | 0.127825 |
| metadata_getattr | 0.006977 | 0.157093 | 22.516 | 0.158425 | 0.159177 | 0.159779 |
| metadata_open | 0.006737 | 0.158997 | 23.599 | 0.159828 | 0.159898 | 0.159955 |
| metadata_readlink | 0.001070 | 0.154148 | 144.125 | 0.157468 | 0.159722 | 0.161525 |
| metadata_access | 0.006307 | 0.153826 | 24.390 | 0.159120 | 0.159473 | 0.159755 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.
- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.
