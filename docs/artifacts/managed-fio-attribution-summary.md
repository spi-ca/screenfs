# Managed passthrough fio attribution: native vs contrib passthrough vs ScreenFS

This artifact reruns the supplemental fio attribution with the managed `contrib/fractal-passthrough` helper. It is attribution evidence, not claim-grade before/after optimization evidence.

## Measured setup

- Passthrough implementation: managed `contrib/fractal-passthrough` helper built at `/tmp/screenfs-goal-bench/passthrough-target/release/fractal_passthrough`
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

## Results

| job | native mean clat µs | passthrough mean clat µs | ScreenFS mean clat µs | passthrough/native | ScreenFS/passthrough |
| --- | ---: | ---: | ---: | ---: | ---: |
| `seq_write_128k` | 58.479 | 52.233 | 178.966 | 0.89x | 3.43x |
| `seq_read_128k` | 39.720 | 78.383 | 104.161 | 1.97x | 1.33x |
| `rand_write_4k` | 0.481 | 6.174 | 51.617 | 12.83x | 8.36x |
| `rand_read_4k` | 0.978 | 13.739 | 35.395 | 14.04x | 2.58x |
| `sync_write_4k` | 1.568 | 9.184 | 41.889 | 5.86x | 4.56x |

## ScreenFS perf split excerpt

`managed-fio-attribution-perf-split.json` preserves the full parsed stderr counter set, including count-only metrics such as `matcher_candidates`, `readdir*_attr_generation_entries`, and `invalidations`.

| counter | count | avg ns | share |
| --- | ---: | ---: | ---: |
| `read_handle_snapshot` | 83159 | 176 | 1.3% of `fuse_op.read` |
| `read_guard_path` | 83159 | 11932 | 91.2% of `fuse_op.read` |
| `read_io` | 83159 | 822 | 6.3% of `fuse_op.read` |
| `write_handle_snapshot` | 57624 | 210 | 0.9% of `fuse_op.write` |
| `write_guard_mutation` | 57624 | 20570 | 90.8% of `fuse_op.write` |
| `write_io` | 57624 | 1706 | 7.5% of `fuse_op.write` |

## Provenance

See `managed-fio-attribution-env.json` for command lines, binary SHA256 values, git status, policy args, and source/mount paths captured during the run.
