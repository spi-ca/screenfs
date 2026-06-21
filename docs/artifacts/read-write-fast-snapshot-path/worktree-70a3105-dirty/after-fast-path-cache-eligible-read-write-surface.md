# ScreenFS benchmark result

- timestamp: `2026-06-21T01:20:29.296034+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /home/spi-ca/Codebase/screenfs/target/release/screenfs --screenfs-source-root /home/spi-ca/Codebase/screenfs --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/after-fast-path-cache-eligible-read-write-surface.svg`
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
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- fast_path_cache_eligible: `True`
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
| seq_read | 0.005443 | 0.026005 | 4.778 | 0.027298 | 0.027469 | 0.027605 |
| seq_write | 0.008217 | 0.027593 | 3.358 | 0.028230 | 0.028626 | 0.028942 |
| small_read | 0.000340 | 0.011061 | 32.538 | 0.012027 | 0.012273 | 0.012469 |
| small_write | 0.000594 | 0.020592 | 34.651 | 0.021734 | 0.022097 | 0.022388 |
| rand_read_4k | 0.010972 | 0.319813 | 29.147 | 0.354332 | 0.360838 | 0.366043 |
| rand_write_4k | 0.012881 | 0.338602 | 26.288 | 0.433766 | 0.434128 | 0.434418 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
