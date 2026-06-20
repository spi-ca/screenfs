# Matcher descendant combined probe experiment

This bundle records a rejected combined matcher probe for the matcher-heavy policy row and the dedicated descendant-directory row. The Rust-side optimization tested here was reverted and is **not** part of the current code. Treat this directory as rejected experiment evidence, not as kept speedup evidence: the state gate failed, and the useful retained outcome is harness/workload coverage only.

## Artifact bundle

- `before-920c00e-matcher32-policy-heavy-matrix.{json,md,svg}`
- `after-worktree-matcher32-policy-heavy-matrix.{json,md,svg}`
- `before-920c00e-glob-matcher-heavy-matcher-descendant-directory.{json,md,svg}`
- `after-worktree-glob-matcher-heavy-matcher-descendant-directory.{json,md,svg}`

## Provenance

- The **before** binary was built from a clean `/tmp/screenfs-matcher-before-920c00e` worktree at `920c00e` and used the `perf-counters` feature.
- The **after** binary was the current `target/release/screenfs` from the repo worktree, also used with `--perf-counters`.
- Both runs used `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --iterations 10 --warmups 3` and split the probe into `--workload-set policy-heavy-matrix` plus `--workload-set matcher-descendant-directory`.
- Do not overclaim this as a clean revision-to-revision result: both artifacts report the same git revision `920c00e`, while the after binary came from the current dirty worktree.

## Mounted latency ratios

### Policy-heavy matrix (`matcher32`)

| workload | after/before p50 | after/before p95 | after/before p99 |
| --- | ---: | ---: | ---: |
| `metadata_access` | `1.039605x` | `1.015808x` | `1.017916x` |
| `metadata_getattr` | `1.016526x` | `0.965077x` | `0.948535x` |
| `metadata_lookup` | `1.065319x` | `1.047297x` | `1.043521x` |

### Descendant-directory row (`glob-matcher-heavy`)

| workload | after/before p50 | after/before p95 | after/before p99 |
| --- | ---: | ---: | ---: |
| `matcher_descendant_readdir` | `0.957930x` | `0.982901x` | `1.019307x` |
| `matcher_descendant_readdirplus` | `1.023877x` | `1.015288x` | `1.019405x` |

## Conclusion

- The state gate failed. The checked workloads did not produce a kept matcher-heavy win: the policy-heavy metadata row was near-neutral to regressive, `matcher_descendant_readdir` improved only marginally and still missed tail non-regression at p99, and `matcher_descendant_readdirplus` regressed.
- Because the gate failed, the Rust optimization was reverted/not kept.
- The retained value is coverage: the harness/workloads remain useful for future matcher-heavy work, including the dedicated non-empty visible descendant `matcher_descendant_readdir` / `matcher_descendant_readdirplus` row and perf-counter collection with `--perf-counters`.
