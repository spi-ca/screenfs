# ScreenFS benchmark result

- timestamp: `2026-07-03T01:54:58.280634+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-release-target.gn4OdT/release/screenfs --policy-preset fast-path-cache-eligible --policy-label writeback-exp-rerun-per-open-cache-minimum-baseline --workload-set per-open-cache-minimum --iterations 10 --warmups 3 --output-json /tmp/screenfs-writeback-bench-rerun-20260703-105424/baseline-per-open-cache-minimum.json --output-md /tmp/screenfs-writeback-bench-rerun-20260703-105424/baseline-per-open-cache-minimum.md`
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
- policy_label: `writeback-exp-rerun-per-open-cache-minimum-baseline`
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
| rand_read_4k | 0.021408 | 0.213425 | 9.969 | 0.218083 | 0.219296 | 0.220266 |
| rand_write_4k | 0.023708 | 0.337035 | 14.216 | 0.340498 | 0.340537 | 0.340567 |
| sync_write_4k | 0.000169 | 0.002838 | 16.748 | 0.003042 | 0.003140 | 0.003219 |
| small_open_read_close | 0.038688 | 0.249140 | 6.440 | 0.270931 | 0.273477 | 0.275515 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
