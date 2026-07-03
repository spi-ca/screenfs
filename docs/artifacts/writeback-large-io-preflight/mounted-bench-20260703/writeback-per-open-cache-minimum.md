# ScreenFS benchmark result

- timestamp: `2026-07-03T01:07:31.193267+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-release-target.gn4OdT/release/screenfs --policy-preset fast-path-cache-eligible --policy-label writeback-exp-per-open-cache-minimum-writeback --workload-set per-open-cache-minimum --iterations 10 --warmups 3 --output-json /tmp/screenfs-writeback-bench-20260703-100631/writeback-per-open-cache-minimum.json --output-md /tmp/screenfs-writeback-bench-20260703-100631/writeback-per-open-cache-minimum.md --extra-screenfs-arg=--experimental-writeback-cache`
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
- policy_label: `writeback-exp-per-open-cache-minimum-writeback`
- fast_path_cache_eligible: `True`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
- workload_selection: `named-set`
- workload_set: `per-open-cache-minimum`
- comparable_workloads: `rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close`
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
| rand_read_4k | 0.014140 | 0.148712 | 10.517 | 0.152119 | 0.154651 | 0.156677 |
| rand_write_4k | 0.017455 | 0.232521 | 13.321 | 0.235489 | 0.236247 | 0.236854 |
| sync_write_4k | 0.000152 | 0.002058 | 13.518 | 0.002246 | 0.002283 | 0.002313 |
| small_open_read_close | 0.023885 | 0.191686 | 8.025 | 0.194704 | 0.195198 | 0.195593 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
