# ScreenFS benchmark series

- timestamp: `2026-06-24T22:18:02.237934+00:00`
- pairs: `4`
- order: `alternate`
- before_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local`
- after_bin: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- bench_args: `--screenfs-source-root /tmp/screenfs-parent-local-8720eb8d82ba/after --perf-counters --iterations 10 --warmups 3 --dir-entries 5000 --workload-set directory-surface --policy-preset fast-path-cache-eligible --policy-label parent-local-hidden-visible-anchor --extra-screenfs-arg=--visibility-default --extra-screenfs-arg=hidden --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/read/seq.bin --extra-screenfs-arg=--visible --extra-screenfs-arg=/.screenfs-bench/dir-entries`

## Pair ratios

After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdir_basic` | 0.633x | 0.670x | 0.671x |
| 1 | `readdirplus_basic` | 0.921x | 0.927x | 0.914x |
| 2 | `readdir_basic` | 0.673x | 0.594x | 0.560x |
| 2 | `readdirplus_basic` | 1.373x | 1.461x | 1.400x |
| 3 | `readdir_basic` | 0.875x | 0.843x | 0.816x |
| 3 | `readdirplus_basic` | 1.128x | 1.172x | 1.094x |
| 4 | `readdir_basic` | 0.851x | 0.891x | 0.908x |
| 4 | `readdirplus_basic` | 0.796x | 0.717x | 0.719x |

## Runs

| order | pair | side | json |
| ---: | ---: | --- | --- |
| 1 | 1 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-before-no-parent-local.json` |
| 2 | 1 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair1-after-parent-local.json` |
| 3 | 2 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-after-parent-local.json` |
| 4 | 2 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair2-before-no-parent-local.json` |
| 5 | 3 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-before-no-parent-local.json` |
| 6 | 3 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair3-after-parent-local.json` |
| 7 | 4 | `after` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-after-parent-local.json` |
| 8 | 4 | `before` | `docs/artifacts/directory-child-visibility-parent-local-series/worktree-8720eb8d82ba/hidden-visible-anchor-directory-surface/hidden-visible-anchor-pair4-before-no-parent-local.json` |
