# current matcher source attribution smoke

## Scope

This smoke records the measure-only matcher source attribution counters added for the matcher-heavy policy path. It is not a before/after speedup claim.

The new perf-counter labels preserve the existing aggregate counters and add source-level splits:

- `matcher_candidates_by_source.<source>`
- `matcher_candidate_order_by_source.<source>`

The instrumentation label set is:

- `internal_hidden.path`
- `hidden.path`
- `visible.path`
- `visible.descendant`
- `readonly.path`
- `writable.path`

## Command

```text
python3 scripts/bench-screenfs.py --build --perf-counters \
  --policy-preset fallback-unsafe-policy \
  --matcher-extra-rules 32 \
  --policy-label glob-matcher-heavy \
  --workload metadata_lookup \
  --workload metadata_getattr \
  --workload metadata_access \
  --workload matcher_hidden_stat_miss \
  --iterations 3 --warmups 1 \
  --metadata-ops 200 \
  --matcher-misses 200
```

Artifacts:

- `policy-heavy-matrix-smoke.json`
- `policy-heavy-matrix-smoke.md`
- `policy-heavy-matrix-smoke.svg`

## Observed split in this visibility-only policy-heavy smoke

| counter | value |
| --- | ---: |
| `matcher_candidates_by_source.hidden.path.count` | 800 |
| `matcher_candidates_by_source.internal_hidden.path.count` | 0 |
| `matcher_candidates_by_source.visible.descendant.count` | 128096 |
| `matcher_candidates_by_source.visible.path.count` | 0 |
| `matcher_candidate_order_by_source.hidden.path.total_ns` | 6128931 |
| `matcher_candidate_order_by_source.internal_hidden.path.total_ns` | 1560300 |
| `matcher_candidate_order_by_source.visible.descendant.total_ns` | 5153210 |
| `matcher_candidate_order_by_source.visible.path.total_ns` | 5510896 |

## Reading

This visibility-only policy-heavy smoke remains subtree-family heavy and exercises visibility decisions, not mutability decisions. It uses explicit workload selection because the current `policy-heavy-matrix` set also includes `matcher_readonly_access_wok` for mutability attribution. The observed source split therefore includes `internal_hidden.path`, `hidden.path`, `visible.path`, and `visible.descendant`; it does not prove `readonly.path` or `writable.path` runtime attribution. Within this visibility-only smoke, `visible.descendant` accounts for most candidate volume, while `hidden.path` accounts for the hidden matcher hits. `visible.path` has zero matching candidates in this workload, but still has candidate-order accounting cost in perf-counter builds.

This artifact should guide the next implementation attempt; it does not prove a user-visible latency improvement by itself.
