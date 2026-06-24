# ScreenFS benchmark series

- timestamp: `2026-06-24T07:48:58.356584+00:00`
- pairs: `2`
- order: `alternate`
- before_bin: `/tmp/screenfs-matcher-control`
- after_bin: `/tmp/screenfs-matcher-control`
- bench_args: `--screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 1 --warmups 1 --dir-entries 10 --metadata-ops 5 --matcher-extra-rules 1 --matcher-misses 5 --policy-preset fallback-unsafe-policy --policy-label series-smoke --workload-set policy-heavy-matrix`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `metadata_lookup` | 4.344x | 4.344x | 4.344x |
| 1 | `matcher_hidden_stat_miss` | 0.677x | 0.677x | 0.677x |
| 2 | `metadata_lookup` | 0.539x | 0.539x | 0.539x |
| 2 | `matcher_hidden_stat_miss` | 0.882x | 0.882x | 0.882x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `/tmp/screenfs-series-smoke/smoke-pair1-before-control-a.json` |
| 2 | 1 | `after` | `/tmp/screenfs-series-smoke/smoke-pair1-after-control-b.json` |
| 3 | 2 | `after` | `/tmp/screenfs-series-smoke/smoke-pair2-after-control-b.json` |
| 4 | 2 | `before` | `/tmp/screenfs-series-smoke/smoke-pair2-before-control-a.json` |
