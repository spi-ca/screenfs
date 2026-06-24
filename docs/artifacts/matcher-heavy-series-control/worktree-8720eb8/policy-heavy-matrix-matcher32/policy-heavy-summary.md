# ScreenFS benchmark series

- timestamp: `2026-06-24T13:04:25.775379+00:00`
- pairs: `4`
- order: `alternate`
- before_bin: `/tmp/screenfs-series-before-control`
- after_bin: `/tmp/screenfs-series-after-control`
- bench_args: `--screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --metadata-ops 200 --matcher-extra-rules 32 --matcher-misses 200 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set policy-heavy-matrix`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `metadata_lookup` | 0.767x | 0.739x | 0.741x |
| 1 | `metadata_getattr` | 0.728x | 0.743x | 0.750x |
| 1 | `metadata_access` | 1.081x | 1.034x | 1.045x |
| 1 | `matcher_hidden_stat_miss` | 1.188x | 1.141x | 1.127x |
| 1 | `matcher_readonly_access_wok` | 0.811x | 0.803x | 0.807x |
| 2 | `metadata_lookup` | 1.018x | 0.986x | 0.950x |
| 2 | `metadata_getattr` | 0.952x | 0.989x | 1.005x |
| 2 | `metadata_access` | 1.434x | 1.472x | 1.431x |
| 2 | `matcher_hidden_stat_miss` | 1.363x | 1.494x | 1.504x |
| 2 | `matcher_readonly_access_wok` | 1.007x | 0.895x | 0.922x |
| 3 | `metadata_lookup` | 1.425x | 1.403x | 1.432x |
| 3 | `metadata_getattr` | 1.397x | 1.430x | 1.417x |
| 3 | `metadata_access` | 1.382x | 1.404x | 1.414x |
| 3 | `matcher_hidden_stat_miss` | 1.421x | 1.559x | 1.564x |
| 3 | `matcher_readonly_access_wok` | 1.353x | 1.440x | 1.443x |
| 4 | `metadata_lookup` | 1.088x | 0.893x | 0.849x |
| 4 | `metadata_getattr` | 0.952x | 0.817x | 0.836x |
| 4 | `metadata_access` | 0.917x | 0.815x | 0.800x |
| 4 | `matcher_hidden_stat_miss` | 1.137x | 0.848x | 0.836x |
| 4 | `matcher_readonly_access_wok` | 1.245x | 1.211x | 1.204x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-before-control-a.json` |
| 2 | 1 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair1-after-control-b.json` |
| 3 | 2 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-after-control-b.json` |
| 4 | 2 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair2-before-control-a.json` |
| 5 | 3 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair3-before-control-a.json` |
| 6 | 3 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair3-after-control-b.json` |
| 7 | 4 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-after-control-b.json` |
| 8 | 4 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32/policy-heavy-pair4-before-control-a.json` |
