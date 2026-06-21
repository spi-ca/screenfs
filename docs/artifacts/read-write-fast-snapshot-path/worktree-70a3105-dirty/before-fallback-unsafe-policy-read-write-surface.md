# ScreenFS benchmark result

- timestamp: `2026-06-21T01:20:18.001885+00:00`
- harness_command_line: `python3 scripts/bench-screenfs.py --screenfs-bin /tmp/screenfs-rwfast-before.GLYI2j/target/release/screenfs --screenfs-source-root /tmp/screenfs-rwfast-before.GLYI2j --policy-preset fallback-unsafe-policy --workload-set read-write-surface --iterations 10 --warmups 3 --output-json /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fallback-unsafe-policy-read-write-surface.json --output-md /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fallback-unsafe-policy-read-write-surface.md --output-svg /home/spi-ca/Codebase/screenfs/docs/artifacts/read-write-fast-snapshot-path/worktree-70a3105-dirty/before-fallback-unsafe-policy-read-write-surface.svg`
- harness_repo_root: `/tmp/screenfs-rwfast-before.GLYI2j`
- git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- git_worktree_clean: `True`
- screenfs_bin: `/tmp/screenfs-rwfast-before.GLYI2j/target/release/screenfs`
- screenfs_bin_sha256: `4385bb6f8f6ff0545ee9c480052aab824ba9267d0deb3007bcf2871485bc6e4d`
- screenfs_source_root: `/tmp/screenfs-rwfast-before.GLYI2j`
- screenfs_source_root_origin: `cli`
- screenfs_source_git: `70a31053d6883cb76b8e7a7ffeebb3530d621f19`
- screenfs_source_git_worktree_clean: `True`
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
| seq_read | 0.006868 | 0.046631 | 6.789 | 0.050924 | 0.052017 | 0.052892 |
| seq_write | 0.009593 | 0.033031 | 3.443 | 0.034881 | 0.040556 | 0.045097 |
| small_read | 0.000381 | 0.019487 | 51.156 | 0.020358 | 0.020576 | 0.020750 |
| small_write | 0.000708 | 0.043192 | 61.022 | 0.044809 | 0.045684 | 0.046384 |
| rand_read_4k | 0.012787 | 0.587871 | 45.975 | 0.620441 | 0.620761 | 0.621017 |
| rand_write_4k | 0.014577 | 0.835206 | 57.296 | 0.897517 | 0.905127 | 0.911214 |

## ScreenFS-only workloads

| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |
| --- | ---: | ---: | ---: | ---: |

## Perf counters

Perf counters were not enabled or no `screenfs perf counters:` summary was captured.

## Notes

- Fixture creation, build time, and mount startup are not included in workload timings.
- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.
- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.
