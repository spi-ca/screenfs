# matcher no-writable mutability fast path (rejected)

## Scope

This bundle records a rejected matcher-heavy policy-path experiment. The attempted implementation changed `RuntimeConfig::mutability_decision()` for `writable_rule_count == 0` so default-readonly policy returned `Readonly` immediately and default-writable policy queried only the readonly matcher. Shapes with writable rules kept the existing `choose_axis(readonly, writable)` path.

The intended safety argument was that without writable rules no positive writable carve-out can override readonly/default behavior. Focused config tests passed, but the full matcher-heavy policy matrix did not clear the benchmark gate after the readonly access probe was tightened to verify the target exists before checking `W_OK`.

## Focused validation before benchmarking

```text
cargo fmt --check
CARGO_INCREMENTAL=0 cargo test mutability_without_writable_rules_keeps_default_writable_readonly_match_and_miss --lib
CARGO_INCREMENTAL=0 cargo test mutability_without_writable_rules_keeps_default_readonly --lib
CARGO_INCREMENTAL=0 cargo test mutability_with_writable_rules_keeps_more_specific_override_semantics --lib
```

## Benchmark method

The comparison used temporary non-destructive worktrees from the same `HEAD` plus the current patch. The before side applied the current patch and then reverted only the no-writable mutability fast path, so both sides included the same matcher source-attribution counters and benchmark harness changes.

All rows used:

```text
scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3
```

Rows:

- `before-policy-heavy-matrix.*` / `after-policy-heavy-matrix.*`: `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy-matcher32 --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- `before-readonly-access-wok.*` / `after-readonly-access-wok.*`: same policy, explicit `--workload matcher_readonly_access_wok --matcher-misses 200`
- `before-fallback-no-matcher-policy-heavy.*` / `after-fallback-no-matcher-policy-heavy.*`: `--policy-preset fallback-unsafe-policy --policy-label fallback-no-matcher --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`
- `before-fast-policy-heavy.*` / `after-fast-policy-heavy.*`: `--policy-preset fast-path-cache-eligible --policy-label fast-path-cache-eligible --workload-set policy-heavy-matrix --metadata-ops 200 --matcher-misses 200`

## Result summary

After/before ratios, lower is better:

| row | workload | p50 | p95 | p99 | result |
| --- | --- | ---: | ---: | ---: | --- |
| focused matcher readonly | `matcher_readonly_access_wok` | 0.791x | 0.721x | 0.699x | focused pass |
| matcher32 policy-heavy | `metadata_access` | 1.325x | 1.429x | 1.438x | fail |
| matcher32 policy-heavy | `metadata_getattr` | 1.331x | 1.341x | 1.339x | fail |
| matcher32 policy-heavy | `metadata_lookup` | 1.302x | 0.954x | 0.937x | fail median |
| matcher32 policy-heavy | `matcher_hidden_stat_miss` | 1.228x | 1.113x | 1.112x | fail |
| matcher32 policy-heavy | `matcher_readonly_access_wok` | 1.211x | 1.245x | 1.246x | fail |
| fallback no-matcher context | `metadata_access` | 1.283x | 1.331x | 1.333x | fail |
| fallback no-matcher context | `metadata_getattr` | 1.348x | 1.352x | 1.353x | fail |
| fallback no-matcher context | `metadata_lookup` | 1.352x | 1.400x | 1.428x | fail |
| fast context | `metadata_access` | 1.346x | 1.301x | 1.292x | fail |
| fast context | `metadata_getattr` | 1.273x | 1.294x | 1.311x | fail |
| fast context | `metadata_lookup` | 1.220x | 1.238x | 1.221x | fail |

## Decision

Rejected. The focused `matcher_readonly_access_wok` row improved, but the required `policy-heavy-matrix` and companion context rows regressed materially. The Rust implementation is not kept.

## Scope boundaries

- This is rejected evidence only; do not cite it as a kept matcher-heavy speedup.
- It does not change hidden `ENOENT`, bridge-visible semantics, symlink target visibility, stable cookies/shared cookie domain, or cross-request cache behavior in the current code.
- Future matcher-heavy work should continue from the source-attribution smokes and needs fresh before/after evidence before keeping any runtime shortcut.
