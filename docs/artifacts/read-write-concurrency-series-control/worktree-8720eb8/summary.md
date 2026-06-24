# Read/write concurrency series control (worktree 8720eb8)

Status: **control / no performance claim**

## Purpose

This is the first read/write follow-up measurement after switching away from matcher-heavy. The selected low-risk target was current `fallback-unsafe-policy` `read-write-concurrency` on local btrfs-backed TMPDIR, using the repeated/interleaved series wrapper and the same binary on both sides. The goal was to establish a current same-binary control before considering any read/write implementation candidate.

## Command shape

```bash
TMPDIR=/home/spi-ca/.cache/screenfs-bench-storage \
python3 scripts/bench-screenfs-series.py \
  --before-bin /tmp/screenfs-rw-before-control \
  --after-bin /tmp/screenfs-rw-after-control \
  --before-label control-a \
  --after-label control-b \
  --pairs 3 \
  --order alternate \
  --output-dir docs/artifacts/read-write-concurrency-series-control/worktree-8720eb8/fallback-concurrency-btrfs \
  --stem fallback-concurrency \
  --workload concurrent_rand_read_write_4k \
  -- \
  --screenfs-source-root /home/spi-ca/Codebase/screenfs \
  --perf-counters \
  --iterations 3 --warmups 1 \
  --read-mib 256 --write-mib 256 \
  --rand-io-ops 2048 \
  --concurrency-workers 4 \
  --policy-preset fallback-unsafe-policy \
  --policy-label read-write-concurrency-control \
  --workload-set read-write-concurrency
```

Both control binaries were identical:

```text
5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78  /tmp/screenfs-rw-before-control
5dcc2a2a633ff80c7c12becc36e4f9ffebf0b3602f583954a0e4f8d204d6bb78  /tmp/screenfs-rw-after-control
```

The source filesystem in the child artifacts is local btrfs under `/home/spi-ca/.cache/screenfs-bench-storage/...`.

## Historical blocker context

The first attempt to collect this control was blocked because `/dev/fuse` was absent and `fusermount3` reported `fuse device /dev/fuse not found. Kernel module not loaded?`. That blocker is preserved in [`../../current-read-write-followup-attempt-validation.log`](../../current-read-write-followup-attempt-validation.log). After `/dev/fuse` became available, the same command shape was rerun successfully and produced the control artifacts below.

## Results

Series artifact directory: [`fallback-concurrency-btrfs/`](fallback-concurrency-btrfs/)

After/before p50 / p95 / p99 for `concurrent_rand_read_write_4k`:

| pair | p50 | p95 | p99 |
| ---: | ---: | ---: | ---: |
| 1 | `0.974x` | `0.921x` | `0.917x` |
| 2 | `0.983x` | `0.976x` | `0.975x` |
| 3 | `1.236x` | `3.266x` | `3.425x` |

Observed same-binary range:

| metric | min | median | max |
| --- | ---: | ---: | ---: |
| p50 | `0.974x` | `0.983x` | `1.236x` |
| p95 | `0.921x` | `0.976x` | `3.266x` |
| p99 | `0.917x` | `0.975x` | `3.425x` |

## Counter context

Pair 1 source filesystem was btrfs. Perf counters remained guard-dominant in the fallback row:

| side | `read_guard_path.avg_ns` | `read_io.avg_ns` | `write_guard_mutation.avg_ns` | `write_io.avg_ns` | `state_read_lock_wait.avg_ns` | `state_read_lock_hold.avg_ns` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| before | `23188` | `954` | `36288` | `3744` | `76` | `105` |
| after | `24735` | `1110` | `38744` | `3898` | `85` | `113` |

This supports the earlier coverage interpretation that fallback concurrency is guard-heavy and not obviously dominated by state read-lock wait/hold.

## Decision

No read/write performance claim and no implementation attempt. The same-binary control itself shows a large tail outlier in pair 3, so a future read/write concurrency candidate must beat this control envelope with repeated/interleaved before/after evidence before any claim.

Current read/write state remains:

- kept claim: existing fallback small-I/O baseline at [`../../read-write-small-io-guard-reuse/summary.md`](../../read-write-small-io-guard-reuse/summary.md);
- follow-up lane: `smoke` / coverage-only for storage-backed, concurrency, larger-sequential, and cold-cache-approx rows.
