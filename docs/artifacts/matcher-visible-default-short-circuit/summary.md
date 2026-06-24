# matcher visible-default hidden-miss short-circuit (rejected)

## Scope

This bundle records a rejected matcher-heavy policy-path experiment. The attempted change optimized `RuntimeConfig::is_visible_by_rules()` for `visibility.default=visible`: if `hidden_matcher.best_descriptor(path)` returned `None`, the function returned visible without querying `visible_matcher.best_descriptor(path)`. If a hidden rule matched, the existing `choose_axis()` comparison with visible carve-outs stayed intact. Default-hidden behavior was unchanged.

The intended safety argument was that under default-visible policy, a hidden miss is visible regardless of positive visible rules; visible rules can only matter when a hidden rule matched and may need a more-specific carve-out comparison.

## Validation before benchmarking

Focused checks on the attempted implementation passed:

```text
cargo fmt --check
CARGO_INCREMENTAL=0 cargo test default_visible_hidden_miss_stays_visible_with_visible_rules_present --lib
CARGO_INCREMENTAL=0 cargo test default_visible_hidden_match_allows_more_specific_visible_carve_out --lib
CARGO_INCREMENTAL=0 cargo test default_hidden_visible_carve_out_and_hidden_override_remain_unchanged --lib
```

## Benchmark command

The before/after comparison used temporary non-destructive worktrees from `HEAD` and the current patch. Both sides included the new measure-only matcher source attribution counters; the before side then reverted only the visible-default short-circuit block so the benchmark isolated this runtime shortcut.

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
| `metadata_access` | 1.229x | 1.294x | 1.277x | fail |
| `metadata_getattr` | 1.337x | 1.328x | 1.318x | fail |
| `metadata_lookup` | 1.257x | 1.267x | 1.246x | fail |
| `matcher_hidden_stat_miss` | 1.215x | 1.270x | 1.244x | fail |

Counter context:

| counter | before | after |
| --- | ---: | ---: |
| `policy_decision.total_ns` | 40552151 | 32613995 |
| `matcher_candidate_order_by_source.visible.path.total_ns` | 17862245 | 23642615 |
| `matcher_candidates_by_source.visible.path.count` | 0 | 0 |
| `matcher_candidate_order_by_source.hidden.path.total_ns` | 19722453 | 26436848 |

## Decision

Rejected. Although focused semantics tests passed and `policy_decision.total_ns` decreased in this run, mounted policy-heavy latency regressed across the required matcher32 rows. The implementation is not kept.

The useful retained lesson is attribution: in this subtree-heavy workload, `visible.path` candidate count was zero, while hidden path and visible-descendant accounting dominated the matcher source counters. Future work should continue with source-level attribution or workload coverage rather than keeping this shortcut.
