# Matcher-heavy descendant same-binary control

Status: **control / noise envelope**

## Purpose

This companion control measures `matcher-descendant-directory` with the same ScreenFS binary on both sides of each before/after pair. It complements [`../matcher-heavy-no-code-control/summary.md`](../matcher-heavy-no-code-control/summary.md), which covers `policy-heavy-matrix`.

It is not an implementation attempt and not a performance claim. The goal is to quantify apparent before/after movement for the descendant workloads when the candidate delta is zero. Artifact provenance is still from the current dirty worktree, so read this as a within-pair same-binary noise control, not as a clean-baseline artifact.

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
  --dir-entries 200 --metadata-ops 512 \
  --matcher-extra-rules 32 --matcher-misses 2000 \
  --policy-preset fallback-unsafe-policy \
  --workload-set matcher-descendant-directory \
  ...
```

Each pair ran `before` then `after` with only the `--policy-label` and output paths changed.

## Results

After/before mounted p50 / p95 / p99, despite before and after using the same binary:

| workload | pair 1 | pair 2 | pair 3 | observed range |
| --- | --- | --- | --- | --- |
| `matcher_descendant_readdir` | `0.864x / 0.941x / 0.981x` | `0.979x / 0.869x / 0.854x` | `0.952x / 0.868x / 0.856x` | p50 `0.864x`-`0.979x`, p99 `0.854x`-`0.981x` |
| `matcher_descendant_readdirplus` | `0.895x / 0.942x / 0.938x` | `0.984x / 0.991x / 0.982x` | `1.225x / 1.059x / 1.057x` | p50 `0.895x`-`1.225x`, p99 `0.938x`-`1.057x` |

## Interpretation

The descendant companion workloads also have enough same-binary movement to require repeated/interleaved evidence before any descendant-specific matcher claim. The most notable no-candidate movements were:

- apparent `matcher_descendant_readdir` p95/p99 improvements around `0.868x`-`0.856x` in pairs 2 and 3;
- apparent `matcher_descendant_readdirplus` p50 regression of `1.225x` in pair 3;
- apparent `matcher_descendant_readdirplus` p50 improvement of `0.895x` in pair 1.

This means a candidate that only improves one descendant pair, or only improves one percentile while another pair flips, should remain `smoke` or `rejected` rather than being kept.

## Decision for future matcher-heavy work

A future matcher-heavy candidate needs stable repeated/interleaved evidence on both surfaces when it touches descendant matching:

- `policy-heavy-matrix`, compared against [`../matcher-heavy-no-code-control/summary.md`](../matcher-heavy-no-code-control/summary.md);
- `matcher-descendant-directory`, compared against this control.

Do not use a single descendant before/after pair to promote a claim.

## Artifacts

Descendant same-binary control pairs:

- Pair 1: [`before-descendant-pair1.json`](before-descendant-pair1.json), [`after-descendant-pair1.json`](after-descendant-pair1.json)
- Pair 2: [`before-descendant-pair2.json`](before-descendant-pair2.json), [`after-descendant-pair2.json`](after-descendant-pair2.json)
- Pair 3: [`before-descendant-pair3.json`](before-descendant-pair3.json), [`after-descendant-pair3.json`](after-descendant-pair3.json)

Markdown and SVG companions with the same stems are stored alongside the JSON files.
