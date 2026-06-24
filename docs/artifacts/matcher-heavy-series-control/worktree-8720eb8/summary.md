# Matcher-heavy series control (worktree 8720eb8)

Status: **control / no performance claim**

This artifact uses the repeated/interleaved wrapper `scripts/bench-screenfs-series.py` with the same ScreenFS binary on both sides. It validates the new measurement base on the two matcher-heavy surfaces before attempting another matcher optimization.

Both sides used the same binary:

```text
5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78  /tmp/screenfs-series-before-control
5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78  /tmp/screenfs-series-after-control
```

## Policy-heavy matrix control

Artifact directory: [`policy-heavy-matrix-matcher32/`](policy-heavy-matrix-matcher32/)

Command shape:

```bash
python3 scripts/bench-screenfs-series.py \
  --before-bin /tmp/screenfs-series-before-control \
  --after-bin /tmp/screenfs-series-after-control \
  --before-label control-a \
  --after-label control-b \
  --pairs 4 --order alternate \
  --output-dir docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/policy-heavy-matrix-matcher32 \
  --stem policy-heavy \
  --workload metadata_lookup \
  --workload metadata_getattr \
  --workload metadata_access \
  --workload matcher_hidden_stat_miss \
  --workload matcher_readonly_access_wok \
  -- \
  --screenfs-source-root /home/spi-ca/Codebase/screenfs \
  --perf-counters \
  --iterations 10 --warmups 3 \
  --dir-entries 5000 --metadata-ops 200 \
  --matcher-extra-rules 32 --matcher-misses 200 \
  --policy-preset fallback-unsafe-policy \
  --policy-label matcher-series-control \
  --workload-set policy-heavy-matrix
```

After/before p50 / p95 / p99 ranges across four same-binary pairs:

| workload | p50 range | p95 range | p99 range |
| --- | ---: | ---: | ---: |
| `matcher_hidden_stat_miss` | `1.137x`-`1.421x` | `0.848x`-`1.559x` | `0.836x`-`1.564x` |
| `matcher_readonly_access_wok` | `0.811x`-`1.353x` | `0.803x`-`1.440x` | `0.807x`-`1.443x` |
| `metadata_access` | `0.917x`-`1.434x` | `0.815x`-`1.472x` | `0.800x`-`1.431x` |
| `metadata_getattr` | `0.728x`-`1.397x` | `0.743x`-`1.430x` | `0.750x`-`1.417x` |
| `metadata_lookup` | `0.767x`-`1.425x` | `0.739x`-`1.403x` | `0.741x`-`1.432x` |

## Descendant companion control

Artifact directory: [`matcher-descendant-directory-matcher32/`](matcher-descendant-directory-matcher32/)

Command shape:

```bash
python3 scripts/bench-screenfs-series.py \
  --before-bin /tmp/screenfs-series-before-control \
  --after-bin /tmp/screenfs-series-after-control \
  --before-label control-a \
  --after-label control-b \
  --pairs 4 --order alternate \
  --output-dir docs/artifacts/matcher-heavy-series-control/worktree-8720eb8/matcher-descendant-directory-matcher32 \
  --stem descendant \
  --workload matcher_descendant_readdir \
  --workload matcher_descendant_readdirplus \
  -- \
  --screenfs-source-root /home/spi-ca/Codebase/screenfs \
  --perf-counters \
  --iterations 10 --warmups 3 \
  --dir-entries 200 --metadata-ops 512 \
  --matcher-extra-rules 32 --matcher-misses 2000 \
  --policy-preset fallback-unsafe-policy \
  --policy-label matcher-series-control \
  --workload-set matcher-descendant-directory
```

After/before p50 / p95 / p99 ranges across four same-binary pairs:

| workload | p50 range | p95 range | p99 range |
| --- | ---: | ---: | ---: |
| `matcher_descendant_readdir` | `0.876x`-`1.269x` | `0.751x`-`1.085x` | `0.606x`-`1.082x` |
| `matcher_descendant_readdirplus` | `0.735x`-`1.317x` | `0.694x`-`1.814x` | `0.634x`-`2.210x` |

## Interpretation

The repeated/interleaved wrapper works, but the same-binary control still shows very large apparent movement. This confirms the earlier conclusion more strongly: matcher-heavy implementation candidates must not be promoted from a single pair, and even a small number of pairs may be insufficient if directions flip or remain inside the control envelope.

No optimization is implemented or claimed by this artifact. Matcher-heavy remains `smoke` / no kept speedup.

## Next step guidance

Before another matcher implementation attempt, prefer either:

1. a smaller micro/trace probe that isolates matcher candidate collection without FUSE-level noise; or
2. a candidate expected to move ratios well outside the same-binary control envelope, measured with this same `--pairs 4 --order alternate` series on both `policy-heavy-matrix` and `matcher-descendant-directory`.
