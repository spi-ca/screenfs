# Read/write small-I/O guard reuse summary

This bundle records the claim-grade `read-write-surface` before/after matrix for the read/write small-I/O guard reuse work. Claim scope is the `fallback-unsafe-policy` row only; the `fast-path-cache-eligible` row is retained as context/non-regression evidence. Perf attribution after the implementation comes from the companion fallback perf smokes, and the only available pre-change perf comparison in this area is the existing [`../current-read-write-surface-smoke.json`](../current-read-write-surface-smoke.json) context artifact rather than a same-bundle perf-before pair. Current validation for the maintained bundle is captured in [`validation.log`](validation.log).

## Files

- `before-fallback-read-write-surface.{json,md,svg}` / `after-fallback-read-write-surface.{json,md,svg}`
- `before-fast-read-write-surface.{json,md,svg}` / `after-fast-read-write-surface.{json,md,svg}`
- `after-fallback-read-write-surface-perf-smoke.{json,md,svg}`
- `after-fallback-read-write-surface-perf-smoke-1x.{json,md,svg}`
- `summary.md`
- `validation.log`

## Latency ratios (after / before)

| policy | workload | p50 | p95 | p99 |
| --- | --- | ---: | ---: | ---: |
| fallback-unsafe-policy | `small_read` | 0.552451x | 0.518731x | 0.518190x |
| fallback-unsafe-policy | `small_write` | 0.800006x | 0.760275x | 0.764632x |
| fallback-unsafe-policy | `rand_read_4k` | 0.888775x | 0.815883x | 0.778230x |
| fallback-unsafe-policy | `rand_write_4k` | 0.905193x | 0.984318x | 1.002145x |
| fallback-unsafe-policy | `seq_read` | 0.607403x | 0.588722x | 0.585948x |
| fallback-unsafe-policy | `seq_write` | 0.558202x | 0.542460x | 0.549048x |
| fast-path-cache-eligible | `small_read` | 0.642575x | 0.664625x | 0.663454x |
| fast-path-cache-eligible | `small_write` | 0.725739x | 0.750402x | 0.748735x |
| fast-path-cache-eligible | `rand_read_4k` | 0.746802x | 0.663686x | 0.660638x |
| fast-path-cache-eligible | `rand_write_4k` | 0.971154x | 1.017230x | 0.981842x |
| fast-path-cache-eligible | `seq_read` | 0.606518x | 0.591854x | 0.592033x |
| fast-path-cache-eligible | `seq_write` | 0.568105x | 0.595299x | 0.607578x |

## Gate read

- Targeted `fallback-unsafe-policy` row satisfies the current docs gate: `small_read`, `small_write`, and `rand_read_4k` are all `<= 0.90x` at median and `<= 0.95x` at p95/p99; the remaining `rand_write_4k` row stays within the non-regression budget (`0.905193x` p50, `0.984318x` p95, `1.002145x` p99), and `seq_read` / `seq_write` also remain inside budget.
- The `fast-path-cache-eligible` row improves numerically on most rows and keeps `rand_write_4k` within non-regression (`0.971154x` p50, `1.017230x` p95, `0.981842x` p99), but this bundle keeps that row as context/non-regression rather than a separate fast-policy claim.

## Perf-counter attribution context

- [`after-fallback-read-write-surface-perf-smoke.json`](after-fallback-read-write-surface-perf-smoke.json) (`--iterations 3 --warmups 1`) still shows guard work dominating the fallback data path after the change: `read_guard_path` is `0.856292x` of `fuse_op.read.total_ns`, `write_guard_mutation` is `0.905925x` of `fuse_op.write.total_ns`, while `read_io` is `0.123118x` and `write_io` is `0.078215x`. The 0–4 KiB size buckets remain non-zero (`read_size_bucket.0_4k.count=1764`, `write_size_bucket.0_4k.count=4096`).
- [`after-fallback-read-write-surface-perf-smoke-1x.json`](after-fallback-read-write-surface-perf-smoke-1x.json) uses the same small smoke shape as the existing [`../current-read-write-surface-smoke.json`](../current-read-write-surface-smoke.json), so it can provide limited pre-change context: `source_root_path.count` drops from `17785` to `11533` (`0.648468x`) while `resolved_virtual_path_from_path.count` stays `14759 -> 14759`. Treat that as cross-bundle context only, not as a same-bundle perf-before attribution pair.

## Notes

- Do not generalize this bundle to broad storage-backed, cold-cache, or concurrency improvement.
- Keep `current-read-write-surface-smoke*` as coverage/pre-change context; this bundle does not overwrite that prefix.
- All ratios above are computed from the checked-in JSON `mounted_p50_sec`, `mounted_p95_sec`, and `mounted_p99_sec` values.
