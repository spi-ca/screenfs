# ScreenFS benchmark series

- timestamp: `2026-06-24T13:04:34.092532+00:00`
- pairs: `4`
- order: `alternate`
- before_bin: `/tmp/screenfs-series-before-control`
- after_bin: `/tmp/screenfs-series-after-control`
- bench_args: `--screenfs-source-root /home/spi-ca/Codebase/screenfs --perf-counters --iterations 10 --warmups 3 --dir-entries 200 --metadata-ops 512 --matcher-extra-rules 32 --matcher-misses 2000 --policy-preset fallback-unsafe-policy --policy-label matcher-series-control --workload-set matcher-descendant-directory`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `matcher_descendant_readdir` | 1.227x | 0.751x | 0.606x |
| 1 | `matcher_descendant_readdirplus` | 0.981x | 1.814x | 2.210x |
| 2 | `matcher_descendant_readdir` | 0.876x | 0.790x | 0.744x |
| 2 | `matcher_descendant_readdirplus` | 0.735x | 0.694x | 0.690x |
| 3 | `matcher_descendant_readdir` | 1.269x | 1.085x | 1.082x |
| 3 | `matcher_descendant_readdirplus` | 1.317x | 1.423x | 1.425x |
| 4 | `matcher_descendant_readdir` | 1.022x | 1.030x | 1.059x |
| 4 | `matcher_descendant_readdirplus` | 0.902x | 0.724x | 0.634x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair1-before-control-a.json` |
| 2 | 1 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair1-after-control-b.json` |
| 3 | 2 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-after-control-b.json` |
| 4 | 2 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair2-before-control-a.json` |
| 5 | 3 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-before-control-a.json` |
| 6 | 3 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair3-after-control-b.json` |
| 7 | 4 | `after` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-after-control-b.json` |
| 8 | 4 | `before` | `docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32/descendant-pair4-before-control-a.json` |
