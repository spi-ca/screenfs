# Matcher rank-0 early-stop candidate

This directory records a post-`70a3105` matcher-heavy policy-path retry. The candidate added a minimal early-stop to `MatcherIndex::best_matching_candidate()`: once a **matching** candidate with match-order rank `0` is found, traversal can return because no later candidate can outrank it. It intentionally did **not** revive the previously rejected combined-probe optimization.

## Result

Rejected / not kept. Focused matcher tests and review showed the candidate preserved winner semantics, but same-machine benchmark evidence failed the matcher-heavy gate and regressed the descendant companion row. The Rust implementation was reverted; this artifact remains rejected evidence only.

## Target lane and rows

- active lane: matcher-heavy policy path (`smoke` -> candidate under test)
- target rows collected with clean `HEAD` before and dirty candidate after, all using `--perf-counters --iterations 10 --warmups 3`:
  - `--policy-preset fallback-unsafe-policy --workload-set policy-heavy-matrix --matcher-extra-rules 32 --policy-label matcher32-policy-heavy`
  - `--policy-preset fallback-unsafe-policy --workload-set matcher-descendant-directory --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32`

The primary gate is the `policy-heavy-matrix` row: `matcher_hidden_stat_miss` plus at least one claimed metadata/directory workload in the same policy row must show after/before `<= 0.90x` p50 and `<= 0.95x` p95/p99. The `matcher-descendant-directory` row is companion coverage/non-regression for non-empty visible-descendant semantics; it is not an independent descendant speedup claim for this path-side candidate.

## Benchmark evidence

JSON source of truth:

- `before-70a3105-matcher32-policy-heavy-matrix.json`
- `after-worktree-70a3105-matcher32-policy-heavy-matrix.json`
- `before-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.json`
- `after-worktree-70a3105-glob-matcher-heavy-matcher32-matcher-descendant-directory.json`

After/before ratios:

| row | workload | p50 | p95 | p99 | interpretation |
| --- | --- | ---: | ---: | ---: | --- |
| `policy-heavy-matrix` | `metadata_lookup` | `1.062319x` | `1.084224x` | `1.094899x` | regresses / no claim |
| `policy-heavy-matrix` | `metadata_getattr` | `0.993247x` | `1.025899x` | `1.036851x` | neutral, misses improvement gate |
| `policy-heavy-matrix` | `metadata_access` | `0.948519x` | `1.131084x` | `1.099231x` | p50 below 1 but p95/p99 regress |
| `policy-heavy-matrix` | `matcher_hidden_stat_miss` | `1.008929x` | `0.999705x` | `1.003873x` | misses required improvement gate |
| `matcher-descendant-directory` | `matcher_descendant_readdir` | `1.314779x` | `1.525326x` | `1.628683x` | companion row regresses badly |
| `matcher-descendant-directory` | `matcher_descendant_readdirplus` | `1.469457x` | `1.882706x` | `1.882035x` | companion row regresses badly |

Selected counters in the primary row did not support a keepable speedup claim: `policy_decision.avg_ns` moved from `839ns` to `854ns`, `matcher_candidate_order.path.avg_ns` moved only from `291ns` to `289ns`, and aggregate candidate counts / seen slots / ancestor steps were unchanged. The candidate therefore provided no claim-grade improvement and had unacceptable companion-row regressions.

## Counters watched

- `policy_decision`
- `matcher_candidates`
- `matcher_family_candidates.subtree`
- `matcher_family_candidates.direct_child_glob`
- `matcher_family_candidates.recursive`
- `matcher_candidate_order.path`
- `matcher_candidate_order.descendant`
- `matcher_candidate_order_duplicates{,.path,.descendant}`
- `matcher_candidate_order_seen_slots{,.path,.descendant}`
- `matcher_candidate_order_ancestor_steps{,.path,.descendant}`

## Safety constraints checked before rejection

The attempted implementation preserved these intended constraints while it was under test:

- exact winner semantics matched `candidate_order(path).find(matches)`
- most-specific rule wins and existing tie/order behavior remained unchanged
- hidden `ENOENT` precedence over readonly `EROFS` was not changed
- bridge-visible and descendant semantics were not rewritten
- no new cache, no metric renaming, and no `candidate_order()` / metrics shape change

## Local validation and review

Local validation passed before benchmark rejection:

- `cargo fmt --check`
- `cargo check --features perf-counters`
- `cargo test best_matching_candidate -- --nocapture`

Subagent implementation review reported no findings on winner semantics, metrics shape, test adequacy, or artifact scope. Subagent security review reported no guardrail blocker. Because the benchmark gate failed, the matcher-heavy lane remains `smoke` with this rank-0 early-stop attempt recorded as rejected evidence.
