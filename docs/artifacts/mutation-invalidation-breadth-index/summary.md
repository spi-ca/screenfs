# Mutation invalidation breadth-index experiment

This bundle records a rejected mutation-invalidation breadth-index experiment for the current breadth-first lane. The Rust child-index implementation tested here was reverted and is **not** part of the current code. Treat this directory as rejected/slice-context evidence only, not as kept speedup evidence or an eviction-breadth claim.

## Artifact bundle

- `before-920c00e-mutation-invalidation.{json,md,svg}`
- `after-worktree-mutation-invalidation.{json,md,svg}`
- `before-920c00e-subtree-rename-cached-unrelated.{json,md,svg}`
- `after-worktree-subtree-rename-cached-unrelated.{json,md,svg}`

## Provenance

- The **before** binary was built with `--features perf-counters` from the clean `/tmp/screenfs-matcher-before-920c00e` worktree at `920c00e`.
- The **after** binary was the current repo worktree `target/release/screenfs`, also run with `--perf-counters`.
- Both runs used `--policy-preset fallback-unsafe-policy --iterations 10 --warmups 3` and split the evidence into `--workload-set mutation-invalidation` plus a focused `--workload subtree_rename_cached_unrelated` row.
- Do not overclaim this as a clean revision-to-revision comparison: both artifacts report git revision `920c00e`, but the after binary came from the current dirty worktree while the before binary came from the clean `/tmp` worktree.

## Mounted latency ratios

### `--workload-set mutation-invalidation`

| workload | after/before p50 | after/before p95 | after/before p99 |
| --- | ---: | ---: | ---: |
| `symlink_parent_mkdir_rmdir` | `1.239281x` | `1.232617x` | `1.230222x` |
| `pinned_symlink_parent_mkdir_rmdir` | `1.295362x` | `1.334113x` | `1.330349x` |
| `subtree_rename_cached_unrelated` | `1.439480x` | `1.401463x` | `1.386509x` |

### Focused `--workload subtree_rename_cached_unrelated`

| workload | after/before p50 | after/before p95 | after/before p99 |
| --- | ---: | ---: | ---: |
| `subtree_rename_cached_unrelated` | `1.027902x` | `1.005004x` | `0.995190x` |

## Selected counter deltas

| row | counter | before | after |
| --- | --- | ---: | ---: |
| mutation-invalidation set | `invalidations.scanned_entries` | 15392 | 572 |
| focused subtree row | `invalidations.scanned_entries` | 10764 | 52 |
| mutation-invalidation set | `invalidations.evicted_entries` | 0 | 0 |
| focused subtree row | `invalidations.evicted_entries` | 0 | 0 |

## Conclusion

- The breadth index clearly reduced scan breadth on the measured rows: `invalidations.scanned_entries` dropped `15392 -> 572` on the full set and `10764 -> 52` on the focused subtree row.
- The latency gate still failed. The full mutation-invalidation row regressed materially on all three workloads, and the focused subtree row stayed near-neutral rather than claim-grade.
- Therefore this experiment is rejected/slice-context only. The child-index Rust implementation was reverted/not kept.
- This bundle does **not** support an eviction-breadth claim. Mounted observability still shows `evicted_entries=0`, and post-`FORGET` behavior remains a limitation/state-test-only scope unless a dedicated mounted driver is added later.
