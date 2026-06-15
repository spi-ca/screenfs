# Managed passthrough fio attribution: native vs contrib passthrough vs ScreenFS

This artifact reruns the supplemental fio attribution with the managed `contrib/fractal-passthrough` helper. It is attribution evidence, not claim-grade before/after optimization evidence.

## Measured setup

- Passthrough implementation: managed `contrib/fractal-passthrough` helper built at `/home/spi-ca/Codebase/screenfs/contrib/fractal-passthrough/target/release/fractal_passthrough`
- Policy caveat: this is an explicit custom unsafe shape, not the official harness `fallback-unsafe-policy` preset, because the carve-outs target `/hidden` and `/readonly` instead of `/.screenfs-bench/**`
- Mount/provenance caveats: non-root FUSE run with `fusermount3 3.18.2`; `managed-fio-attribution-env.json` records command lines, binary SHA256 values, and dirty worktree status
- Passthrough safety caveat: `contrib/fractal-passthrough` is only a minimal FUSE floor. It does not implement ScreenFS source-root confinement, visibility/mutability policy, or external-symlink safety checks, so use only a trusted private scratch tree with no external symlinks and do not treat it as a policy-equivalent baseline

## Files

- `docs/artifacts/managed-fio-attribution-env.json`
- `docs/artifacts/managed-fio-attribution-native.json`
- `docs/artifacts/managed-fio-attribution-passthrough-fstype.txt`
- `docs/artifacts/managed-fio-attribution-passthrough.json`
- `docs/artifacts/managed-fio-attribution-passthrough.stderr.log`
- `docs/artifacts/managed-fio-attribution-passthrough.stdout.log`
- `docs/artifacts/managed-fio-attribution-perf-split.json`
- `docs/artifacts/managed-fio-attribution-screenfs-fstype.txt`
- `docs/artifacts/managed-fio-attribution-screenfs.json`
- `docs/artifacts/managed-fio-attribution-screenfs.stderr.log`
- `docs/artifacts/managed-fio-attribution-screenfs.stdout.log`
- `docs/artifacts/managed-fio-attribution-summary.md`
- `docs/artifacts/managed-fio-attribution-summary.png`
- `docs/artifacts/managed-fio-attribution-summary.svg`
- `docs/artifacts/managed-fio-attribution-boxplot.png`
- `docs/artifacts/managed-fio-attribution-boxplot.svg`

## Results

| job | native mean clat µs | passthrough mean clat µs | ScreenFS mean clat µs | passthrough/native | ScreenFS/passthrough |
| --- | ---: | ---: | ---: | ---: | ---: |
| `seq_write_128k` | 15.085 | 49.581 | 133.269 | 3.29x | 2.69x |
| `seq_read_128k` | 13.378 | 54.226 | 91.525 | 4.05x | 1.69x |
| `rand_write_4k` | 0.710 | 10.297 | 84.305 | 14.50x | 8.19x |
| `rand_read_4k` | 0.633 | 18.617 | 64.990 | 29.40x | 3.49x |
| `sync_write_4k` | 1.072 | 11.388 | 89.580 | 10.62x | 7.87x |

## ScreenFS perf split excerpt

`managed-fio-attribution-perf-split.json` preserves the full parsed stderr counter set, including count-only metrics such as `matcher_candidates`, `readdir*_attr_generation_entries`, and `invalidations`.

| counter | count | avg ns | share |
| --- | ---: | ---: | ---: |
| `read_handle_snapshot` | 45259 | 237 | 0.8% of `fuse_op.read` |
| `read_guard_path` | 45259 | 27688 | 95.3% of `fuse_op.read` |
| `read_io` | 45259 | 968 | 3.3% of `fuse_op.read` |
| `write_handle_snapshot` | 35994 | 257 | 0.7% of `fuse_op.write` |
| `write_guard_mutation` | 35994 | 33052 | 94.8% of `fuse_op.write` |
| `write_io` | 35994 | 1381 | 4.0% of `fuse_op.write` |

## Boxplot style contract

`managed-fio-attribution-boxplot.svg` is intentionally kept in the original fixed-scale style so refreshed artifacts remain visually comparable across runs:

- canvas: `1600x900` SVG, with `3200x1800` PNG raster output
- y-axis: linear completion latency in µs, fixed `0..250` tick range
- series colors: native gray, managed passthrough blue, ScreenFS red
- markers: p95 orange dot, mean black diamond; box is p25/p50/p75 and whiskers are p1/p99
- rendering: regenerate PNG from SVG with `rsvg-convert -w 3200 -h 1800 docs/artifacts/managed-fio-attribution-boxplot.svg -o docs/artifacts/managed-fio-attribution-boxplot.png`

Do not switch this artifact to log scale or a different layout unless the summary and README explicitly call out the visual break from prior artifacts.

## Provenance

See `managed-fio-attribution-env.json` for command lines, binary SHA256 values, git status, policy args, and source/mount paths captured during the run.
