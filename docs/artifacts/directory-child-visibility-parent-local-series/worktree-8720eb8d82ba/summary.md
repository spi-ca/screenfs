# Directory child visibility parent-local series

## Scope

Measure-only evidence for the current parent-local `DirectoryChildVisibilityBatch` slice. The after binary is the current dirty worktree. The before binary is a non-destructive `/tmp` copy of the same dirty snapshot with only the default-hidden parent-local branch disabled (`else if false && visibility_default == Hidden`). This artifact does not make a user-visible latency speed claim because mounted latency ratios remain noisy and inconsistent.

Two subcases are separated:

- **fully-visible subcase**: the scanned directory is itself a visible subtree anchor (`/.screenfs-bench/dir-entries`), so the parent-local mode returns `fully_visible: true`.
- **frontier child-set subcase**: every scanned child file has a visible subtree rule anchored at that direct child path under the scanned parent, so scanning `/.screenfs-bench/dir-entries` exercises the `visible_children` parent-local frontier path directly.

## Provenance

- worktree revision: `8720eb8d82ba`
- source snapshot: current dirty worktree copied to `/tmp/screenfs-parent-local-8720eb8d82ba/{before,after}`
- before binary: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local`
- after binary: `/tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local`
- binary sha256:
```text
84b20a30b24c0da8ff470158a100e6124373c07d54ce8d584558e17e17a99605  /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-after-parent-local
ff65e7e4d5b9d9506096ae5b33185ee9b108263a272aed1a592f13dd87847511  /tmp/screenfs-parent-local-8720eb8d82ba/screenfs-before-no-parent-local
```

## Policy shapes

Fully-visible subcase policy:

```text
--extra-screenfs-arg=--visibility-default hidden
--extra-screenfs-arg=--visible /.screenfs-bench/read/seq.bin
--extra-screenfs-arg=--visible /.screenfs-bench/dir-entries
```

Frontier child-set subcase policy:

```text
--extra-screenfs-arg=--visibility-default hidden
--extra-screenfs-arg=--visible /.screenfs-bench/read/seq.bin
--extra-screenfs-arg=--visible /.screenfs-bench/dir-entries/entry-000000.txt
...
--extra-screenfs-arg=--visible /.screenfs-bench/dir-entries/entry-000999.txt
```

Both policies have no hidden/internal-hidden rules. The fully-visible policy uses a visible subtree rule anchored at the scanned directory, while the frontier policy uses visible subtree rules anchored at direct child paths that classify into `visible_children`; both stay within the implemented parent-local mode. The harness classifies them as named custom unsafe policies because raw policy-shaping arguments override the built-in preset.

## fully-visible subcase directory-surface 5k

- series summary: [`hidden-visible-anchor-directory-surface/hidden-visible-anchor-summary.md`](hidden-visible-anchor-directory-surface/hidden-visible-anchor-summary.md)

### Mounted latency after/before ratios

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

### Perf-counter total_ns after/before ratios

| pair | readdirplus_scan.scan_visibility | readdirplus_directory_scan | fuse_op.readdirplus |
| ---: | ---: | ---: | ---: |
| 1 | 0.006x | 0.176x | 0.218x |
| 2 | 0.008x | 0.221x | 0.279x |
| 3 | 0.007x | 0.207x | 0.257x |
| 4 | 0.007x | 0.202x | 0.253x |

## fully-visible subcase focused readdirplus 20k

- series summary: [`hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-summary.md`](hidden-visible-anchor-readdirplus-20k/hidden-visible-anchor-20k-summary.md)

### Mounted latency after/before ratios

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdirplus_basic` | 1.089x | 0.993x | 0.979x |
| 2 | `readdirplus_basic` | 0.921x | 0.962x | 0.965x |
| 3 | `readdirplus_basic` | 0.854x | 0.858x | 0.840x |

### Perf-counter total_ns after/before ratios

| pair | readdirplus_scan.scan_visibility | readdirplus_directory_scan | fuse_op.readdirplus |
| ---: | ---: | ---: | ---: |
| 1 | 0.009x | 0.276x | 0.305x |
| 2 | 0.008x | 0.234x | 0.258x |
| 3 | 0.008x | 0.221x | 0.242x |

## fully-visible subcase same-binary control 5k

- series summary: [`hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-summary.md`](hidden-visible-anchor-same-binary-control/hidden-visible-anchor-control-summary.md)

### Mounted latency after/before ratios

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdir_basic` | 1.047x | 1.009x | 0.971x |
| 1 | `readdirplus_basic` | 0.834x | 0.879x | 0.887x |
| 2 | `readdir_basic` | 1.306x | 1.191x | 1.159x |
| 2 | `readdirplus_basic` | 1.154x | 1.055x | 0.964x |
| 3 | `readdir_basic` | 1.400x | 1.347x | 1.323x |
| 3 | `readdirplus_basic` | 0.841x | 0.913x | 0.953x |

### Perf-counter total_ns after/before ratios

| pair | readdirplus_scan.scan_visibility | readdirplus_directory_scan | fuse_op.readdirplus |
| ---: | ---: | ---: | ---: |
| 1 | 0.955x | 0.984x | 0.994x |
| 2 | 1.219x | 1.229x | 1.220x |
| 3 | 1.103x | 1.070x | 1.066x |

## frontier child-set subcase readdirplus 1k

- series summary: [`hidden-visible-child-frontier-1k/hidden-visible-child-frontier-1k-summary.md`](hidden-visible-child-frontier-1k/hidden-visible-child-frontier-1k-summary.md)

### Mounted latency after/before ratios

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdirplus_basic` | 1.037x | 0.957x | 0.953x |
| 2 | `readdirplus_basic` | 0.935x | 0.941x | 0.942x |
| 3 | `readdirplus_basic` | 0.731x | 0.949x | 0.954x |

### Perf-counter total_ns after/before ratios

| pair | readdirplus_scan.scan_visibility | readdirplus_directory_scan | fuse_op.readdirplus |
| ---: | ---: | ---: | ---: |
| 1 | 0.052x | 0.181x | 0.491x |
| 2 | 0.043x | 0.164x | 0.425x |
| 3 | 0.036x | 0.130x | 0.342x |

## frontier child-set same-binary control 1k

- series summary: [`hidden-visible-child-frontier-1k-same-binary-control/hidden-visible-child-frontier-1k-control-summary.md`](hidden-visible-child-frontier-1k-same-binary-control/hidden-visible-child-frontier-1k-control-summary.md)

### Mounted latency after/before ratios

| pair | workload | p50 | p95 | p99 |
| ---: | --- | ---: | ---: | ---: |
| 1 | `readdirplus_basic` | 0.838x | 1.006x | 1.007x |
| 2 | `readdirplus_basic` | 1.073x | 1.091x | 1.095x |

### Perf-counter total_ns after/before ratios

| pair | readdirplus_scan.scan_visibility | readdirplus_directory_scan | fuse_op.readdirplus |
| ---: | ---: | ---: | ---: |
| 1 | 0.798x | 0.772x | 0.797x |
| 2 | 1.071x | 1.041x | 1.024x |

## Interpretation

- Direct counter attribution is strong in both candidate subcases:
  - fully-visible 5k/20k candidate runs reduce `readdirplus_scan.scan_visibility.total_ns` to roughly `0.006x`-`0.009x` and `readdirplus_directory_scan.total_ns` to roughly `0.176x`-`0.276x`.
  - frontier child-set 1k candidate runs reduce `readdirplus_scan.scan_visibility.total_ns` to `0.036x`-`0.052x` and `readdirplus_directory_scan.total_ns` to `0.130x`-`0.181x`.
- End-to-end mounted latency is not yet claim-grade:
  - fully-visible 5k `readdirplus_basic` p50 ranges from `0.796x` to `1.373x`; focused 20k p50 ranges from `0.854x` to `1.089x`.
  - frontier child-set 1k `readdirplus_basic` p50 ranges from `0.731x` to `1.037x`; p95/p99 are directionally improved around `0.941x`-`0.957x` / `0.942x`-`0.954x`, but p50 is not stable.
- Same-binary controls remain noisy:
  - fully-visible 5k control `readdirplus_basic` p50 ranges from `0.834x` to `1.154x`.
  - frontier 1k control `readdirplus_basic` ranges from `0.838x` to `1.073x` p50 and `1.006x` to `1.091x` p95.
- Therefore this artifact supports an attribution/slice statement, not a user-visible latency speed claim.

## Answer from this evidence

- The optimized internal `readdirplus_scan.scan_visibility` bucket improved by about **95%-99%** in the measured custom policies.
- The broader `readdirplus_directory_scan` bucket improved by about **72%-87%** in counter totals.
- User-visible mounted `readdirplus_basic` latency is **not claimable yet** because repeated/interleaved pairs and same-binary controls still overlap/noise out the result.

## Next gate before claiming

- Add a less noisy dedicated mounted workload or larger frontier fixture and re-run with same-binary controls.
- Require stable mounted-latency direction outside the control envelope before claiming speed.
- If claiming broader matcher/bridge-visible behavior, add matcher-descendant evidence as required by `docs/benchmarks.md`.
