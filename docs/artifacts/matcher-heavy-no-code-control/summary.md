# Matcher-heavy same-binary control

Status: **control / noise envelope**

## Purpose

After `matcher-empty-family-lookup-skip` produced one promising policy-heavy pair and then failed an immediate rerun, this control measures the current matcher-heavy `policy-heavy-matrix` with the **same ScreenFS binary on both sides of each pair**. It is not an implementation attempt and not a performance claim. Its purpose is to quantify how much apparent before/after movement can appear when the candidate delta is zero. The artifact provenance is still from the current dirty worktree, so read this as a within-pair same-binary noise control, not as a clean-baseline or docs-only artifact.

## Setup

All pairs used the same binary:

```text
/tmp/screenfs-matcher-control
sha256: 5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78
```

Common command shape:

```bash
python3 scripts/bench-screenfs.py \
  --screenfs-bin /tmp/screenfs-matcher-control \
  --screenfs-source-root /home/spi-ca/Codebase/screenfs \
  --perf-counters \
  --iterations 10 --warmups 3 \
  --dir-entries 5000 --metadata-ops 200 \
  --matcher-extra-rules 32 --matcher-misses 200 \
  --policy-preset fallback-unsafe-policy \
  --workload-set policy-heavy-matrix \
  ...
```

Each pair ran `before` then `after` with only the `--policy-label` and output paths changed.

## Results

After/before mounted p50 / p95 / p99, despite before and after using the same binary:

| workload | pair 1 | pair 2 | pair 3 | observed range |
| --- | --- | --- | --- | --- |
| `metadata_lookup` | `1.036x / 1.063x / 1.081x` | `0.732x / 0.802x / 0.807x` | `1.019x / 1.014x / 1.017x` | p50 `0.732x`-`1.036x`, p99 `0.807x`-`1.081x` |
| `metadata_getattr` | `0.990x / 0.890x / 0.869x` | `0.669x / 0.683x / 0.686x` | `0.996x / 0.978x / 0.983x` | p50 `0.669x`-`0.996x`, p99 `0.686x`-`0.983x` |
| `metadata_access` | `1.056x / 1.086x / 1.087x` | `0.697x / 0.719x / 0.724x` | `0.993x / 0.922x / 0.907x` | p50 `0.697x`-`1.056x`, p99 `0.724x`-`1.087x` |
| `matcher_hidden_stat_miss` | `1.089x / 1.231x / 1.232x` | `0.696x / 0.762x / 0.785x` | `0.897x / 0.825x / 0.820x` | p50 `0.696x`-`1.089x`, p99 `0.785x`-`1.232x` |
| `matcher_readonly_access_wok` | `1.052x / 1.067x / 1.067x` | `1.069x / 0.973x / 0.974x` | `1.023x / 1.008x / 1.019x` | p50 `1.023x`-`1.069x`, p99 `0.974x`-`1.067x` |

## Interpretation

Single policy-heavy pairs are too noisy to support matcher-heavy claims. The same-binary control itself produced apparent improvements as large as:

- `matcher_hidden_stat_miss` pair 2: `0.696x / 0.762x / 0.785x`
- `metadata_getattr` pair 2: `0.669x / 0.683x / 0.686x`
- `metadata_access` pair 2: `0.697x / 0.719x / 0.724x`

It also produced apparent regressions without a code change:

- `matcher_hidden_stat_miss` pair 1: `1.089x / 1.231x / 1.232x`
- `metadata_access` pair 1: `1.056x / 1.086x / 1.087x`

## Decision for future matcher-heavy work

Do not promote a matcher-heavy implementation from a single before/after pair. Future matcher-heavy candidates should use repeated or interleaved same-shape pairs and compare against this same-binary control envelope. At minimum, a candidate should show a stable direction across repeated pairs for the targeted workload(s), plus the existing `policy-heavy-matrix` and companion `matcher-descendant-directory` gates.

## Artifacts

Policy-heavy same-binary control pairs:

- Pair 1: [`before-policy-heavy-pair1.json`](before-policy-heavy-pair1.json), [`after-policy-heavy-pair1.json`](after-policy-heavy-pair1.json)
- Pair 2: [`before-policy-heavy-pair2.json`](before-policy-heavy-pair2.json), [`after-policy-heavy-pair2.json`](after-policy-heavy-pair2.json)
- Pair 3: [`before-policy-heavy-pair3.json`](before-policy-heavy-pair3.json), [`after-policy-heavy-pair3.json`](after-policy-heavy-pair3.json)

Markdown and SVG companions with the same stems are stored alongside the JSON files.
