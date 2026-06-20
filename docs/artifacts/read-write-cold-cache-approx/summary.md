# Read/write cold-cache approximation coverage

이 디렉터리는 local `/home/...` btrfs backing storage에서 `--cache-control posix-fadvise-read-fixture`로 다시 측정한 네 개의 **coverage-only** artifact를 묶는다. 이 값은 non-root read-side cold-cache approximation이며, literal `drop_caches` cold-cache claim이 아니다.

## Files

- [`current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md`](current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md) / [`current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json`](current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json) / [`current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`](current-fast-path-cache-eligible-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg)
- [`current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md`](current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.md) / [`current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json`](current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.json) / [`current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg`](current-fallback-unsafe-policy-read-write-surface-storage-btrfs-posix-fadvise-read-fixture.svg)
- [`current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md`](current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md) / [`current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json`](current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json) / [`current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`](current-fast-path-cache-eligible-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg)
- [`current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md`](current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.md) / [`current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json`](current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.json) / [`current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg`](current-fallback-unsafe-policy-read-write-concurrency-storage-btrfs-posix-fadvise-read-fixture.svg)

## Approximation contract

- method: `posix-fadvise-read-fixture`
- default contrast: `--cache-control warm` leaves cache state unchanged; these four reruns add only the documented approximation step.
- implementation scope: call `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` before each warmup and measured sample on selected **backing source regular files** under `source/.screenfs-bench`, not on mounted view paths.
- supported read-side workloads only: `seq_read`, `small_read`, `rand_read_4k`, `concurrent_rand_read_write_4k`. Write-side rows are still measured in the same bundle, but cache control is not applied to write fixtures.
- mounted runs evict backing source files rather than mounted-path aliases, so the approximation is source-fixture based even for mounted samples.
- limits: no privileged `drop_caches`, no dentry/inode/device-cache reset, no write-side cold-cache approximation, and no claim that this is strict claim-grade cold-cache evidence.

## Cache-control stats snapshot

| row | applications | selected backing source regular files | workload-specific notes |
| --- | ---: | --- | --- |
| `fast-path-cache-eligible` `read-write-surface` | 42 | `.screenfs-bench/read/seq.bin` | read-side applications come from `seq_read`, `small_read`, `rand_read_4k`; write rows stay read-side-only/no-op |
| `fallback-unsafe-policy` `read-write-surface` | 42 | `.screenfs-bench/read/seq.bin` | read-side applications come from `seq_read`, `small_read`, `rand_read_4k`; write rows stay read-side-only/no-op |
| `fast-path-cache-eligible` `read-write-concurrency` | 14 | `.screenfs-bench/concurrency-read/worker-000000.bin` | `worker-000001.bin`/`000002.bin`/`000003.bin` were skipped as `duplicate-inode`, so one regular backing file was evicted per side |
| `fallback-unsafe-policy` `read-write-concurrency` | 14 | `.screenfs-bench/concurrency-read/worker-000000.bin` | `worker-000001.bin`/`000002.bin`/`000003.bin` were skipped as `duplicate-inode`, so one regular backing file was evicted per side |

The `42` applications on each `read-write-surface` row come from 3 read-side workloads × 7 warmup/measured passes × native+mounted sides. The `14` applications on each concurrency row come from 1 mixed read-side workload × 7 passes × native+mounted sides.

## Key p50 / p95 / p99 from the JSON artifacts

### `read-write-surface`

| policy | workload | native p50/p95/p99 s | mounted p50/p95/p99 s | mounted/native p50 ratio |
| --- | --- | ---: | ---: | ---: |
| `fast-path-cache-eligible` | `seq_read` | 0.108743/0.110590/0.110747 | 0.197791/0.208737/0.210861 | 1.819x |
| `fast-path-cache-eligible` | `seq_write` | 0.091295/0.104930/0.107178 | 0.200046/0.202381/0.202545 | 2.191x |
| `fast-path-cache-eligible` | `small_read` | 0.002648/0.004432/0.004636 | 0.026300/0.027546/0.027583 | 9.930x |
| `fast-path-cache-eligible` | `small_write` | 0.000963/0.001070/0.001080 | 0.032249/0.034677/0.034962 | 33.490x |
| `fast-path-cache-eligible` | `rand_read_4k` | 0.807492/0.816906/0.817243 | 1.282738/1.310936/1.315260 | 1.589x |
| `fast-path-cache-eligible` | `rand_write_4k` | 0.045848/0.047654/0.047982 | 0.835199/0.915248/0.915446 | 18.217x |
| `fallback-unsafe-policy` | `seq_read` | 0.101646/0.103334/0.103576 | 0.238524/0.250833/0.251765 | 2.347x |
| `fallback-unsafe-policy` | `seq_write` | 0.081155/0.091965/0.092527 | 0.152854/0.156598/0.157124 | 1.883x |
| `fallback-unsafe-policy` | `small_read` | 0.002484/0.002770/0.002788 | 0.026639/0.027432/0.027511 | 10.722x |
| `fallback-unsafe-policy` | `small_write` | 0.001028/0.001104/0.001113 | 0.068532/0.082956/0.085684 | 66.670x |
| `fallback-unsafe-policy` | `rand_read_4k` | 0.797193/0.830005/0.831631 | 1.816882/1.870322/1.880671 | 2.279x |
| `fallback-unsafe-policy` | `rand_write_4k` | 0.025612/0.025817/0.025839 | 1.519007/1.588442/1.595510 | 59.308x |

### `read-write-concurrency`

| policy | workload | native p50/p95/p99 s | mounted p50/p95/p99 s | mounted/native p50 ratio |
| --- | --- | ---: | ---: | ---: |
| `fast-path-cache-eligible` | `concurrent_rand_read_write_4k` | 0.243360/0.259253/0.260780 | 0.984798/1.020835/1.024632 | 4.047x |
| `fallback-unsafe-policy` | `concurrent_rand_read_write_4k` | 0.235903/0.239849/0.240138 | 1.525063/1.829703/1.875234 | 6.465x |

## Scope reading

- This bundle closes the **non-root approximation coverage** gap for read-side cold-cache-style reruns on the current harness surface.
- It does **not** claim strict literal cold-cache, and it does not replace any future same-machine before/after pair when a change specifically claims cold-cache-sensitive movement.
- Keep `docs/artifacts/read-write-storage-backed/summary.md` as the warm-cache storage/concurrency companion and read this directory only as the documented `posix-fadvise-read-fixture` approximation layer on top of that surface.
