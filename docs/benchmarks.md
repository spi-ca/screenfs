# ScreenFS benchmarks

This document defines the formal benchmark workflow for ScreenFS performance work. It is separate from correctness smoke evidence: benchmark results are only performance evidence for the recorded machine, kernel, backing filesystem, policy, and workload.

## Method

The repo-local harness is [`scripts/bench-screenfs.py`](../scripts/bench-screenfs.py). It follows the common recommendations used by Criterion/hyperfine/fio-style benchmarking:

- do not include build time, fixture creation, or mount startup in measured workload timing
- run warmup iterations before measured iterations
- keep every raw sample and report summary statistics, not a single run
- compare ScreenFS against the same workload on the native backing source when possible
- record environment metadata, the harness command line, workload sizes, policy inputs, binary SHA, and dirty/worktree state with the result
- treat cache state explicitly; the default harness evidence is warm-cache local evidence unless a run records separate cache-control steps

The harness intentionally uses only Python standard-library operations plus the built `screenfs` binary. It does not add benchmark-only Rust dependencies to the product crate. For lower-level storage work, use external tools such as `fio` only with a separately recorded job file and environment notes.

For optimization priorities and deferred performance candidates, see [`performance-roadmap.md`](performance-roadmap.md). This document remains the measurement workflow and evidence contract.

ScreenFS also has optional internal attribution counters for benchmark/debug attribution. They are compiled only when the binary is built with `--features perf-counters`; default builds do not include the instrumentation code or config surface. In perf-enabled builds, counters are still disabled by default and should stay off for normal production use unless a run explicitly needs attribution evidence:

```yaml
perf:
  enabled: true
```

When enabled, ScreenFS prints a stderr summary at shutdown with FUSE operation latency, policy decision latency, matcher candidate count, state lock read/write wait/hold latency, `open_confined_openat2` latency, `resolved_virtual_path` latency, `read_size_bucket.*` / `write_size_bucket.*` latency, `readdir_attr_generation_scan` and `readdirplus_attr_generation_scan` latency plus scanned entry counts, and invalidation/eviction counts. Treat these counters as attribution evidence for a benchmark or smoke run, not as standalone performance claims. With `--perf-counters --build`, the benchmark harness builds with `--features perf-counters`, enables `perf.enabled`, parses the shutdown summary into `screenfs.perf_summary` in the JSON output, and includes the raw summary in the Markdown report. If `--perf-counters` is used with an existing `--screenfs-bin`, that binary must already be built with the `perf-counters` feature.

## Workloads

The default benchmark creates a temporary source tree and runs these workloads through both the native source path and the ScreenFS mount path:

| workload | Purpose |
| --- | --- |
| `seq_read` | already-open data path read throughput surface |
| `seq_write` | write syscall/data path overhead, without timing fixture setup |
| `small_read` | repeated small-buffer reads from the sequential-read fixture to probe small read syscall/data-path overhead |
| `small_write` | repeated small-buffer writes to a side-specific output file, removed after each iteration |
| `write_fsync_close` | repeated open/write/file-fsync/close/cleanup cycles as an end-to-end sync-surface probe; it does not fsync parent directories |
| `small_stat_open_read` | metadata-heavy small-file stat/open/read path |
| `readdir_lstat` | directory listing plus metadata path |
| `symlink_open_read` | visible symlink dereference path |

It also runs these ScreenFS-only contract workloads:

| workload | Purpose |
| --- | --- |
| `hidden_stat_miss` | hidden path `ENOENT` path and matcher overhead |
| `symlink_parent_mkdir_rmdir` | repeated mkdir/rmdir under a visible symlink parent alias as an intended mounted probe for symlink-parent mutation guard/path-resolution paths |

The default policy is intentionally simple but non-empty:

```text
--visibility-default visible
--hidden /.screenfs-bench/hidden
--mutability-default writable
--readonly /.screenfs-bench/readonly
```

Use `--extra-screenfs-arg` for additional one-off policy experiments, but record the full harness command line from the JSON output when comparing results.

## Running

Run as a regular user only; do not use `sudo`. Build once, then benchmark the release binary:

```bash
cargo build --release
scripts/bench-screenfs.py \
  --iterations 10 \
  --warmups 3 \
  --output-json docs/artifacts/current-benchmark-result.json \
  --output-md docs/artifacts/current-benchmark-result.md \
  --output-svg docs/artifacts/current-benchmark-boxplot.svg
```

Or let the harness build first, without including build time in measured workloads:

```bash
scripts/bench-screenfs.py --build --output-json /tmp/screenfs-bench.json --output-md /tmp/screenfs-bench.md
```

On this environment, the default release run is typically around a minute. Use the smaller sizing knobs below for quick harness smoke checks only, not for performance evidence.

Useful sizing options:

```text
--read-mib 64
--write-mib 64
--small-io-bytes 4096
--small-io-ops 1024
--sync-bytes 4096
--sync-ops 128
--small-files 2000
--dir-entries 5000
--hidden-misses 2000
--symlink-parent-mutations 2000
--iterations 10
--warmups 3
```

Use larger values for stable release evidence.

## Formal performance-claim bar

Small smoke runs, such as `--iterations 1 --warmups 1`, only prove that the harness, mount, workload plumbing, and result serialization work. Do not use them as evidence for a performance claim.

For a claim-grade before/after comparison, use at least:

```text
--iterations 10
--warmups 3
```

Then inspect the JSON raw samples for the affected workload, not just the Markdown table. A claim should report p50/median plus p90/p95/p99 tail latency, raw-sample spread or variance, and whether samples overlap enough to make the result inconclusive. If repeated runs disagree, record the result as inconclusive instead of selecting the favorable run.

## Reading results

The JSON output is the source of truth. The Markdown output is a human-readable summary, and the optional SVG output is a box plot of raw sample timings. The harness records the full benchmark command line, git worktree clean/dirty state, and the `screenfs` binary SHA256 so human review does not lose dirty/uncommitted binary provenance. Summaries include p50/median, p90, p95, and p99. Comparable workloads include a `mounted_over_native_median` ratio:

- `1.0`: ScreenFS median approximately matches native path for this workload
- `>1.0`: ScreenFS mount is slower than native path
- `<1.0`: ScreenFS mount measured faster; treat this cautiously and inspect sample variance/cache effects

Do not compare results from different machines, kernels, storage devices, CPU governors, or policy inputs unless those differences are the subject of the experiment.

## Evidence requirements for performance changes

Any new or expanded performance claim must be backed by benchmark and/or perf-counter evidence that directly measures the changed surface. Reuse the repo-local harness where it fits, but do not treat unrelated workload wins as proof for a different hot path.

When a performance change is proposed or merged, record at least:

- git revision, whether the worktree was clean, and a visible dirty-status summary in the human-readable report when the worktree was not clean
- benchmark command line used for the run
- JSON result path or attached result
- `screenfs` binary path plus binary SHA256/provenance, especially when comparing dirty or otherwise uncommitted binaries
- kernel, `fusermount3`, rustc/cargo, backing filesystem, CPU/storage notes when relevant
- whether `perf.enabled` counters were enabled and the stderr counter summary when used for attribution
- workload sizes, warmups, iterations, and cache-control assumptions
- before/after p50/median ratios, p90/p95/p99 tail latency, and raw-sample variance for the affected workload
- separate post-change correctness validation command and result, typically `cargo test --all-targets --all-features`; record that in final evidence alongside the benchmark because the harness does not run correctness validation for you

For host-side async/io_uring experiments, also follow the selective data-path evidence rules in [`docs/operations.md`](operations.md) and [`docs/artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md).

## Current harness coverage limits

The current harness is useful for mounted-vs-native comparisons on the listed workloads, but it does not directly isolate all planned performance surfaces.

It does not directly measure:

- matcher bucket/index cost or policy evaluator hot paths
- `open_confined` / `openat2` call frequency or latency
- mutation invalidation breadth / cache-eviction cost
- internal helper/offload attribution for the small-buffer or sync-surface path without separate counters/traces

The `small_read` and `small_write` comparable workloads probe the small-buffer data path end to end. The `write_fsync_close` comparable workload probes the sync surface end to end through repeated open/write/file-fsync/close cycles plus cleanup, but it does not fsync parent directories. These workloads can show mounted-vs-native deltas on those surfaces, but they do not by themselves prove that time moved in a specific internal helper, queueing layer, or offload path, including internal offload helpers; use counters, traces, or focused artifacts when you need attribution.

The `symlink_parent_mkdir_rmdir` ScreenFS-only workload is an intended mounted workload/probe for symlink-parent mutation guard/path-resolution paths around the current `src/fs/guards.rs` request-local reuse area. Each mkdir/rmdir under a visible symlink parent alias is meant to traverse the mounted mutation path and can support before/after investigation of guard/path-resolution behavior. But without counters or traces, it does not prove that a live FUSE request hit a specific internal helper such as `RequestPathResolver`, `guarded_child_mutation`, or `guard_opened_directory_at_path`. It also is not a speedup claim by itself; request-local reuse changes still need before/after comparisons, tail-latency checks, and preferably raw-sample review on this workload before claiming lower guard/path-resolution overhead.

For claims on the remaining uncovered surfaces, add dedicated perf counters, traces, microbenchmarks, or focused benchmark artifacts alongside the main harness output.

## Guardrails

- Do not use benchmark results to relax hidden `ENOENT`, bridge-visible, symlink target, or mutability precedence semantics.
- Do not treat warm-cache benchmark evidence as cold-cache or storage-device evidence.
- Do not run privileged mount or namespace steps from this harness; ScreenFS benchmark evidence remains non-root FUSE evidence, and the harness exits when run as root.
- Do not check in large generated result files unless they are explicitly accepted as current baseline artifacts.
