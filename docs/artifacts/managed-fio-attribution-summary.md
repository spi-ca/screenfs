# Managed passthrough fio attribution: native vs contrib passthrough vs ScreenFS

This artifact reruns the supplemental fio attribution with the managed `contrib/fractal-passthrough` helper. It is attribution evidence, not claim-grade before/after optimization evidence.

## Measured setup

- Passthrough implementation: managed `contrib/fractal-passthrough` helper built at `/home/spi-ca/Codebase/screenfs/contrib/fractal-passthrough/target/release/fractal_passthrough`
- Policy caveat: this is an explicit custom unsafe shape, not the official harness `fallback-unsafe-policy` preset, because the carve-outs target `/hidden` and `/readonly` instead of `/.screenfs-bench/**`
- Mount/provenance caveats: non-root FUSE run with `fusermount3 version: 3.18.2`; `managed-fio-attribution-env.json` records command lines, binary SHA256 values, and dirty worktree status
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
| `seq_write_128k` | 57.595 | 71.385 | 110.522 | 1.24x | 1.55x |
| `seq_read_128k` | 42.039 | 73.745 | 97.691 | 1.75x | 1.32x |
| `rand_write_4k` | 0.499 | 7.236 | 38.805 | 14.51x | 5.36x |
| `rand_read_4k` | 1.011 | 13.114 | 27.746 | 12.97x | 2.12x |
| `sync_write_4k` | 1.862 | 8.626 | 36.749 | 4.63x | 4.26x |

## ScreenFS perf split excerpt

`managed-fio-attribution-perf-split.json` preserves the full parsed stderr counter set, including count-only metrics such as `matcher_candidates`, `readdir*_attr_generation_entries`, and `invalidations`.

| counter | count | avg ns | share |
| --- | ---: | ---: | ---: |
| `read_handle_snapshot` | 105120 | 195 | 1.9% of `fuse_op.read` |
| `read_guard_path` | 105120 | 9031 | 86.0% of `fuse_op.read` |
| `read_io` | 105120 | 1114 | 10.6% of `fuse_op.read` |
| `write_handle_snapshot` | 76007 | 219 | 1.4% of `fuse_op.write` |
| `write_guard_mutation` | 76007 | 14408 | 89.0% of `fuse_op.write` |
| `write_io` | 76007 | 1387 | 8.6% of `fuse_op.write` |

## Boxplot style contract

`managed-fio-attribution-boxplot.svg` is intentionally kept in the original fixed-scale style so refreshed artifacts remain visually comparable across runs:

- canvas: `1600x900` SVG, with `3200x1800` PNG raster output
- y-axis: linear completion latency in µs, fixed `0..250` tick range
- series colors: native gray, managed passthrough blue, ScreenFS red
- markers: p95 orange dot, mean black diamond; box is p25/p50/p75 and whiskers are p1/p99
- rendering: regenerate PNG from SVG with `scripts/managed-fio-attribution.py --render-only --png-scale 2` or `rsvg-convert -w 3200 -h 1800 docs/artifacts/managed-fio-attribution-boxplot.svg -o docs/artifacts/managed-fio-attribution-boxplot.png`

Do not switch this artifact to log scale or a different layout unless the summary and README explicitly call out the visual break from prior artifacts.

## Provenance

See `managed-fio-attribution-env.json` for command lines, binary SHA256 values, git status, policy args, and source/mount paths captured during the run.
