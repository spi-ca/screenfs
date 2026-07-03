# ScreenFS benchmark result

- timestamp: `2026-07-03T01:06:41.077656+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-release-target.gn4OdT/release/screenfs --policy-preset fast-path-cache-eligible --policy-label writeback-exp-readwrite-baseline --workload-set read-write-surface --iterations 10 --warmups 3 --output-json /tmp/screenfs-writeback-bench-20260703-100631/baseline-read-write-surface.json --output-md /tmp/screenfs-writeback-bench-20260703-100631/baseline-read-write-surface.md`
- harness_repo_root: `/home/spi-ca/Codebase/screenfs`
- git: `29f0c974548bb323f63687ee18e62ec2627412d7`
- git_dirty_status: `M README.md;  M docs/README.md;  M docs/architecture.md;  M docs/benchmarks.md;  M docs/design.md; ... (+15 more)`
- git_worktree_clean: `False`
- screenfs_bin: `/tmp/screenfs-release-target.gn4OdT/release/screenfs`
- screenfs_bin_sha256: `4dfd6aed0c02f8fb95914205df84225a95795fafd19c3499ba034d7137dd5417`
- screenfs_source_root: `unknown`
- screenfs_source_root_origin: `unknown`
- screenfs_source_git: `unknown`
- screenfs_source_git_worktree_clean: `unknown`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `writeback-exp-readwrite-baseline`
- fast_path_cache_eligible: `True`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
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
| seq_read | 0.006389 | 0.029650 | 4.641 | 0.030245 | 0.030253 | 0.030259 |
| seq_write | 0.006433 | 0.034768 | 5.405 | 0.036300 | 0.036303 | 0.036306 |
| small_read | 0.000727 | 0.009007 | 12.389 | 0.009321 | 0.009440 | 0.009535 |
| small_write | 0.001686 | 0.020431 | 12.118 | 0.021041 | 0.021140 | 0.021220 |
| rand_read_4k | 0.020609 | 0.214916 | 10.429 | 0.219217 | 0.220475 | 0.221482 |
| rand_write_4k | 0.024339 | 0.233748 | 9.604 | 0.236466 | 0.239641 | 0.242181 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
