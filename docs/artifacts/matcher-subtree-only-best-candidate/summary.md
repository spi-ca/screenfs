# matcher subtree-only best-candidate fast path (rejected)

## Scope

This bundle records a rejected matcher-heavy policy-path experiment. The attempted change added a conservative `MatcherIndex::best_matching_candidate()` fast path for subtree-only indexes: when no direct-child glob or recursive descriptors existed, it walked ancestors from deepest to root and returned the first matching descriptor from the first populated subtree anchor.

The intended safety argument was that subtree-only descriptor specificity follows anchor depth, while same-anchor vectors are already stored in match order. Mixed-family matchers fell back to the existing `visit_path_candidates()` / `update_best_candidate()` path. Focused unit tests compared the fast path against `candidate_order(path).find(matches)` for subtree-only conflicts, same-anchor ties, and mixed-family fallback.

## Validation before benchmarking

Focused checks on the attempted implementation passed:

```text
cargo fmt --check
CARGO_INCREMENTAL=0 cargo test matcher::index::tests --lib
CARGO_INCREMENTAL=0 cargo test --all-targets --all-features matcher
```

## Benchmark command

The before/after comparison used temporary non-destructive worktrees from `HEAD` and the current patch, with the attempted implementation applied only to the `after` worktree:

```text
scripts/bench-screenfs.py --build --perf-counters \
  --iterations 10 --warmups 3 \
  --policy-preset fallback-unsafe-policy \
  --matcher-extra-rules 32 \
  --policy-label glob-matcher-heavy \
  --workload-set policy-heavy-matrix \
  --metadata-ops 200 \
  --matcher-misses 200
```

Artifacts:

- `before-policy-heavy-matcher32.{json,md,svg}`
- `after-policy-heavy-matcher32.{json,md,svg}`

## Result

Mounted after/before ratios, lower is better:

| workload | p50 | p95 | p99 | result |
| --- | ---: | ---: | ---: | --- |
| `metadata_access` | 1.368x | 1.363x | 1.352x | fail |
| `metadata_getattr` | 1.274x | 1.246x | 1.227x | fail |
| `metadata_lookup` | 1.216x | 1.191x | 1.174x | fail |
| `matcher_hidden_stat_miss` | 1.177x | 1.352x | 1.352x | fail |

Counter context:

| counter | before | after |
| --- | ---: | ---: |
| `policy_decision.total_ns` | 40434270 | 50624640 |
| `matcher_candidate_order.path.total_ns` | 43993407 | 56717426 |
| `matcher_candidate_order.descendant.total_ns` | 15860654 | 21690407 |
| `matcher_candidates.count` | 418696 | 418696 |
| `matcher_family_candidates.subtree.count` | 418696 | 418696 |

## Decision

Rejected. Although the focused semantics tests passed, the policy-heavy matrix regressed across the targeted metadata and matcher-hidden rows. The implementation is not kept.

Future matcher-heavy work should return to measure-first attribution, especially per-axis/per-matcher splits or workload coverage that exercises direct-child glob and recursive matcher families before trying another runtime shortcut.
