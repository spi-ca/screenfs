# ScreenFS benchmark series

- timestamp: `2026-06-24T15:13:59.737909+00:00`
- pairs: `3`
- order: `alternate`
- before_bin: `/tmp/screenfs-rw-before-control`
- after_bin: `/tmp/screenfs-rw-after-control`
- bench_args: `--screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 3 --warmups 1 --read-mib 256 --write-mib 256 --rand-io-ops 2048 --concurrency-workers 4 --policy-preset fallback-unsafe-policy --policy-label read-write-concurrency-control --workload-set read-write-concurrency`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `concurrent_rand_read_write_4k` | 0.974x | 0.921x | 0.917x |
| 2 | `concurrent_rand_read_write_4k` | 0.983x | 0.976x | 0.975x |
| 3 | `concurrent_rand_read_write_4k` | 1.236x | 3.266x | 3.425x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-before-control-a.json` |
| 2 | 1 | `after` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair1-after-control-b.json` |
| 3 | 2 | `after` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-after-control-b.json` |
| 4 | 2 | `before` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair2-before-control-a.json` |
| 5 | 3 | `before` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-before-control-a.json` |
| 6 | 3 | `after` | `docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs/fallback-concurrency-pair3-after-control-b.json` |
