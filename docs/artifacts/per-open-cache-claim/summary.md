# Per-open cache before/after claim-grade comparison

This compares the pre-change `HEAD` build from `/tmp/screenfs-goal-bench/before` with the current worktree build using `--policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum`, `--iterations 10`, and `--warmups 3`.

## Measured setup

- Policy/workload scope: `--policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum`
- Iteration shape: `--iterations 10 --warmups 3`
- Mount/provenance caveats: non-root FUSE run with `fusermount3 3.18.2`; each JSON records the dirty worktree status, ScreenFS source provenance, command line, binary SHA256, and machine/filesystem metadata
- Claim scope: same-machine tmpfs-backed benchmark pair only; do not generalize beyond the recorded kernel, cache assumptions, and policy shape

## Files

- `docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json`
- `docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.md`
- `docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.png`
- `docs/artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.svg`
- `docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json`
- `docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.md`
- `docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.png`
- `docs/artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.svg`
- `docs/artifacts/per-open-cache-claim/summary.md`
- `docs/artifacts/per-open-cache-claim/summary.png`
- `docs/artifacts/per-open-cache-claim/summary.svg`

## Mounted ScreenFS headline results (median/tail first)

| workload | before p50 µs | after p50 µs | after/before p50 | before p95 µs | after p95 µs | before p99 µs | after p99 µs | before stdev µs | after stdev µs |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `rand_read_4k` | 134330.995 | 111830.271 | 0.832x | 135638.278 | 115627.049 | 135650.007 | 115629.842 | 1340.027 | 1777.484 |
| `rand_write_4k` | 171172.396 | 122292.977 | 0.714x | 173342.841 | 124023.522 | 173461.619 | 124120.444 | 2602.889 | 1404.073 |
| `sync_write_4k` | 5436.551 | 4010.213 | 0.738x | 5527.103 | 4333.212 | 5528.169 | 4384.761 | 93.652 | 165.280 |
| `small_open_read_close` | 158687.173 | 158113.081 | 0.996x | 166214.845 | 160322.823 | 169813.094 | 160492.228 | 4099.523 | 3029.579 |

Mean values remain available in the companion JSON and Markdown reports; this top-level summary intentionally leads with p50/tail/spread per the benchmark contract.

## Bottleneck split evidence

| counter | before avg ns | after avg ns | note |
| --- | ---: | ---: | --- |
| `fuse_op.read` | 10227 | 2251 |  |
| `fuse_op.write` | 13484 | 2364 |  |
| `read_guard_path` | n/a | 20 | after-only split counter |
| `write_guard_mutation` | n/a | 19 | after-only split counter |
| `read_io` | n/a | 1850 | after-only split counter |
| `write_io` | n/a | 1958 | after-only split counter |
| `fuse_op.flush` | 5005 | 5387 |  |
| `fuse_op.fsync` | 7360 | 8195 |  |
| `fuse_op.open` | 13873 | 14305 |  |
| `fuse_op.getattr` | 13522 | 14144 |  |
| `fuse_op.lookup` | 13755 | 14192 |  |

## Correctness validation

Post-change correctness validation observed for this benchmark bundle: `cargo fmt --check`, `cargo check`, `cargo clippy --all-targets --all-features`, and `cargo test --all-targets --all-features` passed; the full test run reported 169 library tests and 1 main test passing, including the focused per-open cache regression tests listed in `../current-fio-per-open-cache-summary.md`.

## Provenance

Each JSON file is `screenfs-benchmark-v2` and records `policy`, `workloads`, ScreenFS source provenance, binary SHA256, command line, dirty worktree status, and perf-counter stderr summary.
