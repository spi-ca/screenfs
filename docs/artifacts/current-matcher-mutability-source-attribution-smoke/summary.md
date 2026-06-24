# current matcher mutability source attribution smoke

## Scope

This smoke records mutability-axis matcher source attribution under the matcher-heavy policy shape. It is not a before/after speedup claim.

The first run uses the existing `read-only-close-surface` workload set because it includes write-intent workloads that exercise `RuntimeConfig::mutability_decision()` and the `readonly.path` / `writable.path` source labels. The second run uses the focused `matcher_readonly_access_wok` workload to target synthetic readonly matcher paths with `W_OK` access and produce non-zero `readonly.path` candidates without performing writes.

## Commands

Existing write-intent surface:

```text
python3 scripts/bench-screenfs.py --build --perf-counters \
  --policy-preset fallback-unsafe-policy \
  --matcher-extra-rules 32 \
  --policy-label glob-matcher-heavy-matcher32 \
  --workload-set read-only-close-surface \
  --iterations 3 --warmups 1
```

Focused readonly matcher probe:

```text
python3 scripts/bench-screenfs.py --build --perf-counters \
  --policy-preset fallback-unsafe-policy \
  --matcher-extra-rules 32 \
  --policy-label glob-matcher-heavy-matcher32 \
  --workload matcher_readonly_access_wok \
  --iterations 3 --warmups 1 \
  --matcher-misses 200
```

Artifacts:

- `read-only-close-surface-matcher32.json`
- `read-only-close-surface-matcher32.md`
- `read-only-close-surface-matcher32.svg`
- `matcher-readonly-access-wok.json`
- `matcher-readonly-access-wok.md`
- `matcher-readonly-access-wok.svg`

## Observed split: existing write-intent surface

| counter | value |
| --- | ---: |
| `matcher_candidate_order_by_source.readonly.path.count` | 4136 |
| `matcher_candidate_order_by_source.readonly.path.total_ns` | 4154961 |
| `matcher_candidate_order_by_source.writable.path.count` | 4136 |
| `matcher_candidate_order_by_source.writable.path.total_ns` | 749852 |
| `matcher_candidates_by_source.readonly.path.count` | 0 |
| `matcher_candidates_by_source.writable.path.count` | 0 |
| `write_guard_mutation.count` | 1024 |
| `write_guard_mutation.total_ns` | 37915263 |
| `write_io.count` | 1024 |
| `write_io.total_ns` | 1491090 |

## Observed split: focused readonly matcher probe

| counter | value |
| --- | ---: |
| `matcher_candidate_order_by_source.readonly.path.count` | 800 |
| `matcher_candidate_order_by_source.readonly.path.total_ns` | 863245 |
| `matcher_candidate_order_by_source.writable.path.count` | 800 |
| `matcher_candidate_order_by_source.writable.path.total_ns` | 154453 |
| `matcher_candidates_by_source.readonly.path.count` | 800 |
| `matcher_candidates_by_source.writable.path.count` | 0 |
| `fuse_op.access.count` | 801 |
| `fuse_op.getattr.count` | 1 |
| `policy_decision.count` | 5608 |
| `policy_decision.total_ns` | 7600586 |

## Reading

The existing write-intent surface proves that the source-level mutability labels are emitted by write-intent workloads under matcher32 policy. It does **not** prove a synthetic readonly-rule hit, because the existing `write_open_*` workloads target the regular write fixture rather than `/.screenfs-bench/matcher-heavy/visible-*/readonly-*.txt`. Therefore `readonly.path` and `writable.path` candidate counts are zero there, while their candidate-order timing records the empty-check cost.

The focused `matcher_readonly_access_wok` probe closes that attribution gap: it targets the synthetic readonly matcher paths with `W_OK` access, keeps file contents unchanged, and records non-zero `matcher_candidates_by_source.readonly.path`. The rerun after removing per-iteration fixture `is_file()` checks records `fuse_op.access.count=801` and only one `fuse_op.getattr`, keeping the measured loop focused on access rather than repeated stat probes.
