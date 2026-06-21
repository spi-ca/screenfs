# ScreenFS benchmark result

- timestamp: `2026-06-21T01:20:50.609195+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fallback-unsafe-policy-read-write-surface.svg`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+2 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `65ef338d3c3d7280c4237fd3e325d93a3fae3f22df1978f2fe138fb702cb0661`
- screenfs_source_root: `/home/spi-ca/Codebase/screenfs`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_dirty_status: `M docs/artifacts/current-next-performance-candidates.md;  M docs/benchmarks.md;  M docs/performance-roadmap.md;  M src/fs.rs;  M src/fs/state.rs; ... (+2 more)`
- screenfs_source_git_worktree_clean: `False`
- policy_preset: `fallback-unsafe-policy`
- policy_bucket: `fallback-unsafe-policy`
- policy_label: `fallback-unsafe-policy`
- fast_path_cache_eligible: `False`
- workload_selection: `named-set`
- workload_set: `read-write-surface`
- comparable_workloads: `seq_read, seq_write, small_read, small_write, rand_read_4k, rand_write_4k`
- screenfs_only_workloads: `(none)`
- cache_control: `warm`
- cache_control_scope: `no explicit cache eviction; existing warm-cache behavior`
- cache_control_timing_applied: `False`
- cache_control_applications: `0`
- cache_control_notes: `warm mode leaves cache state unchanged before warmups and measured samples`
- concurrency_workers: `4`
- iterations: `10`, warmups: `3`

## Comparable workloads

| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| seq_read | 0.006529 | 0.038478 | 5.893 | 0.039292 | 0.039644 | 0.039926 |
| seq_write | 0.009366 | 0.031762 | 3.391 | 0.044160 | 0.046960 | 0.049199 |
| small_read | 0.000389 | 0.023844 | 61.327 | 0.025515 | 0.026672 | 0.027598 |
| small_write | 0.000719 | 0.057439 | 79.898 | 0.060140 | 0.060425 | 0.060653 |
| rand_read_4k | 0.012483 | 0.582522 | 46.664 | 0.739943 | 0.751114 | 0.760052 |
| rand_write_4k | 0.014328 | 0.857796 | 59.869 | 0.900267 | 0.904386 | 0.907681 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
