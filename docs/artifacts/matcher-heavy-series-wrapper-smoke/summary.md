# Matcher-heavy series wrapper smoke

Status: **tooling smoke / no performance claim**

This artifact verifies the repeated/interleaved wrapper added as `scripts/bench-screenfs-series.py` can orchestrate the existing `scripts/bench-screenfs.py` harness and emit pair-level child artifacts plus a manifest/summary.

Command shape:

```bash
python3 scripts/bench-screenfs-series.py \
  --before-bin /tmp/screenfs-matcher-control \
  --after-bin /tmp/screenfs-matcher-control \
  --before-label control-a \
  --after-label control-b \
  --pairs 2 \
  --order alternate \
  --output-dir /tmp/screenfs-series-smoke \
  --stem smoke \
  --workload metadata_lookup \
  --workload matcher_hidden_stat_miss \
  -- \
  --screenfs-source-root /home/spi-ca/Codebase/screenfs \
  --perf-counters \
  --iterations 1 --warmups 1 \
  --dir-entries 10 --metadata-ops 5 \
  --matcher-extra-rules 1 --matcher-misses 5 \
  --policy-preset fallback-unsafe-policy \
  --policy-label series-smoke \
  --workload-set policy-heavy-matrix
```

The same binary was used for before and after. `--order alternate` runs pair 1 as before→after and pair 2 as after→before, so this smoke verifies alternating orchestration. The resulting ratios are smoke-only and intentionally not interpreted as performance movement.

Outputs copied here:

- [`smoke-manifest.json`](smoke-manifest.json)
- [`smoke-summary.md`](smoke-summary.md)
- pair child JSON/Markdown/SVG artifacts with `smoke-pair*` stems

Validation commands observed with this change are recorded in [`../current-matcher-series-wrapper-review-closure.md`](../current-matcher-series-wrapper-review-closure.md), including Python compile/unittests, diff/fmt checks, nightly Rust check/clippy/test, and required `/usr/bin/find` file-list checks.
