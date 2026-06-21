# Read/write fast snapshot-path attempt

This directory records a post-`70a3105` read/write follow-up attempt. The selected sub-axis was **fast-policy cache-eligible small-I/O** on `--policy-preset fast-path-cache-eligible --workload-set read-write-surface`.

## Result

Rejected / not kept. The implementation avoided cloning the opened handle `VirtualPath` when the relevant per-open read/write guard cache allowed the data-path guard to be skipped, while keeping fallback/unsafe paths on the existing request-local guard + opened-fd revalidation chain. Focused Rust tests and reviews found no behavior blocker, but the same-machine benchmark gate did not pass and the Rust implementation was reverted.

## Gate used

Per `docs/benchmarks.md`, a separate fast-policy read/write claim needs:

- before/after `fast-path-cache-eligible` `read-write-surface` rows with `--iterations 10 --warmups 3`
- companion `fallback-unsafe-policy` `read-write-surface` row as non-regression context
- at least 3 of `small_read`, `small_write`, `rand_read_4k`, `rand_write_4k` at after/before `<= 0.90x` median and `<= 0.95x` p95/p99
- the remaining small-I/O row plus `seq_read` / `seq_write` within `<= 1.05x` median and `<= 1.10x` p95/p99

## Evidence bundle

Artifact directory: [`worktree-70a3105-dirty/`](worktree-70a3105-dirty/)

Baseline was collected from a detached clean `HEAD` temp worktree at `70a3105`; after rows were collected from the dirty worktree containing the candidate. The JSON files are the source of truth:

- `before-fast-path-cache-eligible-read-write-surface.json`
- `after-fast-path-cache-eligible-read-write-surface.json`
- `before-fallback-unsafe-policy-read-write-surface.json`
- `after-fallback-unsafe-policy-read-write-surface.json`
- after-only perf-counter smokes: `after-*-read-write-surface-perf-smoke.json`

Fast-policy mounted after/before ratios:

| workload | p50 | p95 | p99 | gate interpretation |
| --- | ---: | ---: | ---: | --- |
| `seq_read` | `0.877017x` | `0.892957x` | `0.892417x` | non-regression passes |
| `seq_write` | `1.054102x` | `1.040682x` | `1.044843x` | median misses non-regression budget |
| `small_read` | `0.839349x` | `0.561498x` | `0.546321x` | small-I/O improvement passes |
| `small_write` | `0.730772x` | `0.743086x` | `0.749309x` | small-I/O improvement passes |
| `rand_read_4k` | `0.913190x` | `0.986083x` | `0.999034x` | misses improvement gate |
| `rand_write_4k` | `0.961652x` | `0.946388x` | `0.938586x` | median misses improvement gate |

Only 2 of the 4 targeted small-I/O rows cleared the full p50/p95/p99 improvement gate, and `seq_write` missed the non-regression budget. The companion fallback row also failed non-regression on multiple rows (`small_read` p50 `1.223570x`, `small_write` p50 `1.329846x`, `rand_read_4k` p95 `1.209990x`). Therefore this is rejected evidence, not a kept speedup claim.

## Safety constraints checked before rejection

The attempted implementation preserved these intended constraints while it was under test:

- cache-safe read/write skipped policy rechecks and did not materialize a path snapshot
- fallback/unsafe read/write still materialized the file handle path
- fallback/unsafe read/write still ran policy, `source_root_path`, `resolved_virtual_path_from_path`, and `resolved_virtual_path_from_open_fd` revalidation counters
- a mixed cache shape (`read` guard skipped, `write` guard retained) was covered by a focused test before the code was reverted

## Validation and review notes

- `cargo fmt --check` passed after formatting.
- `cargo check --features perf-counters` passed for the candidate.
- Focused perf tests passed for cache-safe, fallback, and mixed guard-cache data paths before revert.
- Subagent reviewer follow-up reported no findings after the mixed-cache test was added.
- Subagent security reviewer found no guardrail blocker for the candidate shape.

Because the benchmark gate failed, this lane remains `smoke` for fast-policy read/write follow-up beyond the existing fallback small-I/O claim.
