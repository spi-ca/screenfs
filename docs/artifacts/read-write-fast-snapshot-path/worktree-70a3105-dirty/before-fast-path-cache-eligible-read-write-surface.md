# ScreenFS benchmark result

- timestamp: `2026-06-21T01:19:56.701624+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rwfast-before.GLYI2j/target/release/screenfs --screenfs-source-root /tmp/screenfs-rwfast-before.GLYI2j --policy-preset fast-path-cache-eligible --workload-set read-write-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fast-path-cache-eligible-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fast-path-cache-eligible-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fast-path-cache-eligible-read-write-surface.svg`
- harness_repo_root: `/tmp/screenfs-rwfast-before.GLYI2j`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-rwfast-before.GLYI2j/target/release/screenfs`
- screenfs_bin_sha256: `4385bb6f8f6ff0545ee9c480052aab824ba9267d0deb3007bcf2871485bc6e4d`
- screenfs_source_root: `/tmp/screenfs-rwfast-before.GLYI2j`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
- policy_preset: `fast-path-cache-eligible`
- policy_bucket: `fast-path-cache-eligible`
- policy_label: `fast-path-cache-eligible`
- fast_path_cache_eligible: `True`
- workload_selection: `named-set`
- policy_notes: `effective policy remains cache-eligible for the current per-open read/write fast path`
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
| seq_read | 0.007742 | 0.029651 | 3.830 | 0.030547 | 0.030762 | 0.030933 |
| seq_write | 0.008734 | 0.026177 | 2.997 | 0.027265 | 0.027507 | 0.027700 |
| small_read | 0.000385 | 0.013178 | 34.258 | 0.020649 | 0.021857 | 0.022824 |
| small_write | 0.000564 | 0.028178 | 49.965 | 0.029561 | 0.029737 | 0.029878 |
| rand_read_4k | 0.012139 | 0.350215 | 28.851 | 0.365347 | 0.365930 | 0.366397 |
| rand_write_4k | 0.012404 | 0.352105 | 28.387 | 0.453569 | 0.458721 | 0.462843 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
