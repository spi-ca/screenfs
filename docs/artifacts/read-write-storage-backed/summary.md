# Current storage-backed read/write coverage

This directory records **current storage-backed coverage only** for four read/write artifacts on a local `/home/...` btrfs path. Read it as warm-cache/local evidence from a **dirty worktree**, not as a before/after speedup claim.

All four JSON artifacts record `git_worktree_clean=False` / `screenfs_source_git_worktree_clean=False`, and their `paths.workdir` / `environment.source_filesystem.path` live under `/home/spi-ca/.cache/screenfs-bench-storage/screenfs-bench-*` with `environment.source_filesystem.stat_f_type=btrfs`. In practice, the temporary workdir/TMPDIR-backed benchmark path was the local `/home/spi-ca/.cache/screenfs-bench-storage/...` btrfs tree.

## Files

- [`current-fast-path-cache-eligible-read-write-surface-storage-btrfs.md`](current-fast-path-cache-eligible-read-write-surface-storage-btrfs.md)
- [`current-fallback-unsafe-policy-read-write-surface-storage-btrfs.md`](current-fallback-unsafe-policy-read-write-surface-storage-btrfs.md)
- [`current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.md`](current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs.md)
- [`current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.md`](current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs.md)

## Run shapes

| row | policy | command shape | what it covers |
| --- | --- | --- | --- |
| storage-backed surface | `fast-path-cache-eligible` | `--workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 5 --warmups 2` | current warm-cache/local larger-sequential mixed read/write coverage on local btrfs |
| storage-backed surface | `fallback-unsafe-policy` | `--workload-set read-write-surface --read-mib 256 --write-mib 256 --iterations 5 --warmups 2` | same storage-backed surface under fallback/unsafe policy |
| storage-backed concurrency | `fast-path-cache-eligible` | `--workload-set read-write-concurrency --concurrency-workers 4 --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2` | current warm-cache/local concurrent mixed random read/write coverage on local btrfs |
| storage-backed concurrency | `fallback-unsafe-policy` | `--workload-set read-write-concurrency --concurrency-workers 4 --read-mib 256 --write-mib 256 --rand-io-ops 4096 --iterations 5 --warmups 2` | same concurrency surface under fallback/unsafe policy |

## Mounted p50 / p95 / p99 snapshot

### `read-write-surface` rows

| policy | workload | p50 s | p95 s | p99 s |
| --- | --- | ---: | ---: | ---: |
| `fast-path-cache-eligible` | `seq_read` | 0.146183 | 0.149046 | 0.149209 |
| `fast-path-cache-eligible` | `seq_write` | 0.180255 | 0.186420 | 0.186476 |
| `fast-path-cache-eligible` | `small_read` | 0.022932 | 0.024421 | 0.024573 |
| `fast-path-cache-eligible` | `small_write` | 0.039295 | 0.041820 | 0.042154 |
| `fast-path-cache-eligible` | `rand_read_4k` | 0.469885 | 0.477422 | 0.477827 |
| `fast-path-cache-eligible` | `rand_write_4k` | 0.925242 | 0.940208 | 0.940988 |
| `fallback-unsafe-policy` | `seq_read` | 0.196820 | 0.201008 | 0.201616 |
| `fallback-unsafe-policy` | `seq_write` | 0.191252 | 0.204840 | 0.206343 |
| `fallback-unsafe-policy` | `small_read` | 0.027007 | 0.027374 | 0.027418 |
| `fallback-unsafe-policy` | `small_write` | 0.068683 | 0.069806 | 0.069869 |
| `fallback-unsafe-policy` | `rand_read_4k` | 0.968289 | 0.976014 | 0.976190 |
| `fallback-unsafe-policy` | `rand_write_4k` | 1.609473 | 1.612509 | 1.612736 |

### `read-write-concurrency` rows

| policy | workload | p50 s | p95 s | p99 s |
| --- | --- | ---: | ---: | ---: |
| `fast-path-cache-eligible` | `concurrent_rand_read_write_4k` | 0.773291 | 0.840162 | 0.851445 |
| `fallback-unsafe-policy` | `concurrent_rand_read_write_4k` | 1.087217 | 1.204061 | 1.218119 |

These numbers are descriptive current snapshots only. They are not after/before ratios and do not by themselves prove a kept fast-policy, storage-backed, or concurrency optimization claim.

## Guard-counter interpretation from the JSON artifacts

Perf counters here are process-lifetime aggregates with warmups and cleanup included. Read the split counters as attribution/context, not as latency percentiles.

| row | `read_guard_path.avg_ns` | `read_io.avg_ns` | `write_guard_mutation.avg_ns` | `write_io.avg_ns` | interpretation |
| --- | ---: | ---: | ---: | ---: | --- |
| surface `fast-path-cache-eligible` | 17 | 4567 | 19 | 7852 | guard cost stays near zero relative to data-path I/O |
| surface `fallback-unsafe-policy` | 24827 | 4450 | 31047 | 7803 | both read and write remain guard-dominated on local storage-backed coverage |
| concurrency `fast-path-cache-eligible` | 21 | 1261 | 20 | 3826 | even with `--concurrency-workers 4`, fast-path guard cost stays near zero relative to I/O |
| concurrency `fallback-unsafe-policy` | 34768 | 1250 | 46630 | 3992 | concurrent fallback row remains strongly guard-dominated |

Additional concurrency context from the same JSON:

- fast concurrency `state_read_lock_wait` / `state_read_lock_hold` avg: `89ns` / `132ns`
- fallback concurrency `state_read_lock_wait` / `state_read_lock_hold` avg: `100ns` / `138ns`
- those lock averages are tiny next to the fallback guard counters above, so the checked-in concurrency coverage reads primarily as policy-guard cost rather than obvious `RwLock` contention

## What this bundle does and does not prove

What it covers now:

- local btrfs-backed `read-write-surface` coverage for both policy presets
- local btrfs-backed `read-write-concurrency` / `concurrent_rand_read_write_4k` coverage for both policy presets with `--concurrency-workers 4`
- JSON-backed guard/I/O split interpretation for storage-backed and concurrent rows

What it still does **not** prove:

- no before/after speedup claim
- no cold-cache claim from this storage-backed bundle itself; read-side cold-cache approximation coverage is recorded separately in [`../read-write-cold-cache-approx/summary.md`](../read-write-cold-cache-approx/summary.md)
- no privileged cache-drop run
- no proof that these warm-cache local rows generalize to stricter storage-backed environments
- no replacement for a claim-grade fast-policy/storage/concurrency pair if a future change claims movement on those axes

## Cold-cache follow-up status

Storage-backed and concurrency coverage exist here as coverage-only evidence. The cold-cache follow-up for this run is recorded separately in [`../read-write-cold-cache-approx/summary.md`](../read-write-cold-cache-approx/summary.md) as a non-root read-side `posix-fadvise-read-fixture` approximation. Strict literal cold-cache remains an explicit non-claim under the current harness contract, not a remaining blocker for this storage-backed/concurrency coverage bundle.
