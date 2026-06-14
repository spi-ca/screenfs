# Current perf-counter benchmark baseline summary

This artifact records the smoke-sized perf-enabled benchmark runs used to choose and check the first low-risk performance cleanup. It is attribution evidence only, not a formal performance claim. The runs used `--iterations 3 --warmups 1` with reduced workload sizes, so they do not satisfy the claim-grade bar in `docs/benchmarks.md`.

## Commands

Before cleanup, the working-copy smoke used the same reduced sizing and wrote temporary `/tmp` JSON/Markdown artifacts. The compact structured subset from that run is embedded below.

The current checked-in smoke artifact was generated after the cleanup with the perf feature enabled by the harness:

```bash
scripts/bench-screenfs.py --build --perf-counters --iterations 3 --warmups 1 \
  --read-mib 4 --write-mib 4 --small-io-ops 64 --sync-ops 16 \
  --small-files 200 --dir-entries 500 --hidden-misses 200 \
  --symlink-parent-mutations 200 \
  --output-json docs/artifacts/current-perf-counter-benchmark-result.json \
  --output-md docs/artifacts/current-perf-counter-benchmark-result.md \
  --output-svg docs/artifacts/current-perf-counter-benchmark-result.svg
```

## Baseline attribution

Selected counters from `/tmp/screenfs-perf-baseline-before.json`:

| counter | count | total ns | avg ns |
| --- | ---: | ---: | ---: |
| `resolved_virtual_path` | 145166 | 394848335 | 2719 |
| `policy_decision` | 171732 | 183484872 | 1068 |
| `open_confined_openat2` | 98298 | 47566036 | 483 |
| `state_write_lock_hold` | 34743 | 91983927 | 2647 |
| `readdirplus_attr_generation_scan` | 9 | 11652350 | 1294705 |
| `readdir_attr_generation_scan` | 15 | 6856919 | 457127 |

This identified path-resolution and policy-decision surfaces as safe first investigation targets. The selected cleanup avoids duplicate `source_root_path()` retrieval in `ScreenFs::resolved_virtual_path()` and avoids a redundant visible-descendant bridge lookup after `visibility_decision()` has already returned `BridgeVisible`.

## Smoke before/after check

Selected counters from `/tmp/screenfs-perf-baseline-after-policy.json`:

| counter | before total ns | after total ns | after/before |
| --- | ---: | ---: | ---: |
| `resolved_virtual_path` | 394848335 | 472670600 | 1.197 |
| `policy_decision` | 183484872 | 209047361 | 1.139 |
| `open_confined_openat2` | 47566036 | 70168820 | 1.475 |
| `readdirplus_attr_generation_scan` | 11652350 | 15207698 | 1.305 |
| `readdir_attr_generation_scan` | 6856919 | 10163071 | 1.482 |

Selected mounted p50 timings:

| workload | before p50 s | after p50 s | after/before |
| --- | ---: | ---: | ---: |
| `readdir_lstat` | 0.042442 | 0.062015 | 1.461 |
| `small_stat_open_read` | 0.048325 | 0.068267 | 1.413 |
| `symlink_open_read` | 0.000282 | 0.000311 | 1.106 |

The smoke comparison does not support a user-visible speedup claim. Treat the code change as low-risk attribution-guided cleanup only; any future performance claim needs claim-grade before/after evidence with the required iteration count, raw-sample review, and tail-latency analysis.

## Checked-in benchmark artifacts

- [`current-perf-counter-benchmark-result.json`](current-perf-counter-benchmark-result.json) — machine-readable benchmark result and `screenfs.perf_summary` source of truth for the checked-in smoke run
- [`current-perf-counter-benchmark-result.md`](current-perf-counter-benchmark-result.md) — human-readable summary with raw perf counter block
- [`current-perf-counter-benchmark-result.svg`](current-perf-counter-benchmark-result.svg) — box plot generated from the same raw samples

## Embedded structured data

The compact structured subset below preserves the temporary before/after smoke comparison that selected the cleanup, so readers do not need the temporary `/tmp` JSON files to audit the recorded numbers.

```json
{
  "schema": "screenfs-perf-counter-baseline-summary-v1",
  "claim_grade": false,
  "before": {
    "artifact": "/tmp/screenfs-perf-baseline-before.json",
    "metrics": {
      "resolved_virtual_path": {"count": 145166, "total_ns": 394848335, "avg_ns": 2719},
      "policy_decision": {"count": 171732, "total_ns": 183484872, "avg_ns": 1068},
      "open_confined_openat2": {"count": 98298, "total_ns": 47566036, "avg_ns": 483},
      "state_write_lock_hold": {"count": 34743, "total_ns": 91983927, "avg_ns": 2647},
      "readdirplus_attr_generation_scan": {"count": 9, "total_ns": 11652350, "avg_ns": 1294705},
      "readdir_attr_generation_scan": {"count": 15, "total_ns": 6856919, "avg_ns": 457127}
    }
  },
  "after": {
    "artifact": "/tmp/screenfs-perf-baseline-after-policy.json",
    "metrics": {
      "resolved_virtual_path": {"count": 145166, "total_ns": 472670600, "avg_ns": 3256},
      "policy_decision": {"count": 171732, "total_ns": 209047361, "avg_ns": 1217},
      "open_confined_openat2": {"count": 98298, "total_ns": 70168820, "avg_ns": 713},
      "state_write_lock_hold": {"count": 34743, "total_ns": 102702431, "avg_ns": 2956},
      "readdirplus_attr_generation_scan": {"count": 9, "total_ns": 15207698, "avg_ns": 1689744},
      "readdir_attr_generation_scan": {"count": 15, "total_ns": 10163071, "avg_ns": 677538}
    },
    "mounted_p50_sec": {
      "readdir_lstat": 0.062015,
      "small_stat_open_read": 0.068267,
      "symlink_open_read": 0.000311
    }
  },
  "conclusion": "no user-visible speedup claim"
}
```
