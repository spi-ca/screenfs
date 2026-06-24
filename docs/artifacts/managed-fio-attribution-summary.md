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
| `seq_write_128k` | 28.542 | 37.872 | 129.200 | 1.33x | 3.41x |
| `seq_read_128k` | 14.863 | 54.377 | 119.726 | 3.66x | 2.20x |
| `rand_write_4k` | 0.424 | 6.483 | 37.695 | 15.29x | 5.81x |
| `rand_read_4k` | 0.794 | 11.956 | 32.851 | 15.06x | 2.75x |
| `sync_write_4k` | 0.457 | 8.069 | 46.200 | 17.65x | 5.73x |

## ScreenFS perf split excerpt

`managed-fio-attribution-perf-split.json` preserves the full parsed stderr counter set, including count-only metrics such as `matcher_candidates`, `readdir*_attr_generation_entries`, and `invalidations`.

| counter | count | avg ns | share |
| --- | ---: | ---: | ---: |
| `read_handle_snapshot` | 88725 | 272 | 2.2% of `fuse_op.read` |
| `read_guard_path` | 88725 | 10585 | 84.4% of `fuse_op.read` |
| `read_io` | 88725 | 1512 | 12.1% of `fuse_op.read` |
| `write_handle_snapshot` | 78242 | 228 | 1.4% of `fuse_op.write` |
| `write_guard_mutation` | 78242 | 14014 | 88.5% of `fuse_op.write` |
| `write_io` | 78242 | 1409 | 8.9% of `fuse_op.write` |

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
