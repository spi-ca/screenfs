# ScreenFS benchmark series

- timestamp: `2026-06-24T22:18:49.058863+00:00`
- pairs: `3`
- order: `alternate`
- before_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- after_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- bench_args: `--screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor-control --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdir_basic` | 1.047x | 1.009x | 0.971x |
| 1 | `readdirplus_basic` | 0.834x | 0.879x | 0.887x |
| 2 | `readdir_basic` | 1.306x | 1.191x | 1.159x |
| 2 | `readdirplus_basic` | 1.154x | 1.055x | 0.964x |
| 3 | `readdir_basic` | 1.400x | 1.347x | 1.323x |
| 3 | `readdirplus_basic` | 0.841x | 0.913x | 0.953x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-before-after-a.json` |
| 2 | 1 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair1-after-after-b.json` |
| 3 | 2 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-after-after-b.json` |
| 4 | 2 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair2-before-after-a.json` |
| 5 | 3 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-before-after-a.json` |
| 6 | 3 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-pair3-after-after-b.json` |
