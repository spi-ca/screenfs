# Current read/write follow-up coverage

This directory records current `read-write-surface` follow-up coverage for both policy presets across the default row, an alternate reduced small/random mix rerun, and a separate larger-sequential 256MiB rerun. Read it as **current warm-cache/local coverage only**, not as a before/after speedup claim.

The kept fallback small-I/O claim remains separate at [`../read-write-small-io-guard-reuse/summary.md`](../read-write-small-io-guard-reuse/summary.md).

## Files

- default current rows:
  - [`current-fast-path-cache-eligible-read-write-surface.md`](current-fast-path-cache-eligible-read-write-surface.md)
  - [`current-fallback-unsafe-policy-read-write-surface.md`](current-fallback-unsafe-policy-read-write-surface.md)
- alternate same-64MiB sequential-size rows (legacy `large-io` filenames; reduced small/random mix coverage only):
  - [`current-fast-path-cache-eligible-read-write-surface-large-io.md`](current-fast-path-cache-eligible-read-write-surface-large-io.md)
  - [`current-fallback-unsafe-policy-read-write-surface-large-io.md`](current-fallback-unsafe-policy-read-write-surface-large-io.md)
- larger sequential 256MiB coverage rows:
  - [`current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.md`](current-fast-path-cache-eligible-read-write-surface-large-seq-256mib.md)
  - [`current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.md`](current-fallback-unsafe-policy-read-write-surface-large-seq-256mib.md)
- companion `.json` / `.svg` files for each row

## What this bundle covers

| row | policy | run shape | current coverage |
| --- | --- | --- | --- |
| default current | `fast-path-cache-eligible` | `--workload-set read-write-surface --iterations 10 --warmups 3` | current fast-policy row with split counters for `read_handle_snapshot`, `read_guard_path`, `read_io`, `write_handle_snapshot`, `write_guard_mutation`, `write_io`, and size buckets |
| default current | `fallback-unsafe-policy` | `--workload-set read-write-surface --iterations 10 --warmups 3` | current fallback/unsafe row on the same workload surface with the same split counters |
| alternate same-64MiB current | `fast-path-cache-eligible` | `--workload-set read-write-surface --read-mib 64 --write-mib 64 --iterations 5 --warmups 2 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024` | alternate current fast-policy coverage with the default sequential size but reduced small/random work and 1KiB small I/O |
| alternate same-64MiB current | `fallback-unsafe-policy` | `--workload-set read-write-surface --read-mib 64 --write-mib 64 --iterations 5 --warmups 2 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024` | alternate current fallback/unsafe coverage for the same reduced small/random mix shape |
| larger sequential 256MiB current | `fast-path-cache-eligible` | `--workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 3 --warmups 1 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024` | actual larger-sequential current coverage row; still coverage-only because it keeps the mixed small/random rows, uses only `3` iterations / `1` warmup, and is warm-cache/local |
| larger sequential 256MiB current | `fallback-unsafe-policy` | `--workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 3 --warmups 1 --small-io-ops 512 --rand-io-ops 512 --small-io-bytes 1024` | actual larger-sequential current fallback/unsafe coverage with the same coverage-only caveat |

## Mounted p50 snapshot

### Default current rows

| policy | `seq_read` | `seq_write` | `small_read` | `small_write` | `rand_read_4k` | `rand_write_4k` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `fast-path-cache-eligible` mounted/native | 2.688x | 3.115x | 25.678x | 25.068x | 16.509x | 24.385x |
| `fallback-unsafe-policy` mounted/native | 3.924x | 3.537x | 35.597x | 72.764x | 29.580x | 52.063x |

### Alternate same-64MiB sequential-size rows

| policy | `seq_read` | `seq_write` | `small_read` | `small_write` | `rand_read_4k` | `rand_write_4k` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `fast-path-cache-eligible` mounted/native | 2.830x | 3.256x | 46.333x | 44.686x | 26.826x | 24.041x |
| `fallback-unsafe-policy` mounted/native | 3.851x | 2.344x | 42.200x | 81.853x | 51.767x | 42.592x |

### Larger sequential 256MiB coverage rows

| policy | `seq_read` | `seq_write` | `small_read` | `small_write` | `rand_read_4k` | `rand_write_4k` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `fast-path-cache-eligible` mounted/native | 4.082x | 2.777x | 55.069x | 59.687x | 38.402x | 37.149x |
| `fallback-unsafe-policy` mounted/native | 4.422x | 3.330x | 56.477x | 96.832x | 44.819x | 46.521x |

These ratios are descriptive current snapshots only. They are not after/before ratios and do not by themselves prove a kept fast-policy or larger-sequential optimization claim.

## Guard-cost interpretation from current perf counters

| row | `read_guard_path.avg_ns` | `write_guard_mutation.avg_ns` | `read_io.avg_ns` | `write_io.avg_ns` |
| --- | ---: | ---: | ---: | ---: |
| default `fast-path-cache-eligible` | 16 | 16 | 2470 | 2284 |
| default `fallback-unsafe-policy` | 18105 | 24486 | 2576 | 2433 |
| alternate same-64MiB `fast-path-cache-eligible` | 19 | 15 | 13665 | 11158 |
| alternate same-64MiB `fallback-unsafe-policy` | 17784 | 20868 | 12700 | 10082 |
| larger sequential 256MiB `fast-path-cache-eligible` | 13 | 14 | 11662 | 29647 |
| larger sequential 256MiB `fallback-unsafe-policy` | 20172 | 26454 | 17119 | 36631 |

Current interpretation:

- fast-path guard counters stay near zero on both the default row (`~16ns` / `~16ns`) and the actual larger-sequential 256MiB row (`~13ns` / `~14ns`)
- fallback rows remain guard-heavy on the default row (`~18us` read guard / `~24us` write guard versus `~2.6us` / `~2.4us` I/O); on the actual larger-sequential 256MiB row the read guard remains above read I/O (`~20us` vs `~17us`), while write I/O exceeds the write guard (`~37us` vs `~26us`)
- the legacy `*-large-io.*` rows use the same default `64MiB` sequential size as the default row, so read them only as alternate reduced small/random mix coverage, not as larger-sequential evidence
- the 256MiB rows are the actual larger-sequential coverage here, but they are still coverage-only because they use `--iterations 3 --warmups 1` and keep the same warm-cache/local caveat

## What this bundle does not claim

- no before/after speedup claim
- no separate fast-policy `read-write-surface` claim
- no larger-sequential optimization claim
- no storage-backed or cold-cache claim
- no concurrency claim
- no same-bundle perf-before attribution pair

## Separate kept claim

[`../read-write-small-io-guard-reuse/summary.md`](../read-write-small-io-guard-reuse/summary.md) remains the kept claim-grade fallback small-I/O baseline/context. Preserve that claim as separate from this current-coverage bundle.
