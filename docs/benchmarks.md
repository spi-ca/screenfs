# ScreenFS benchmarks

This document defines the formal benchmark workflow for ScreenFS performance work. It is separate from correctness smoke evidence: benchmark results are only performance evidence for the recorded machine, kernel, backing filesystem, policy, and workload.

## Method

The repo-local harness is [`scripts/bench-screenfs.py`](../scripts/bench-screenfs.py). It follows the common recommendations used by Criterion/hyperfine/fio-style benchmarking:

- do not include build time, fixture creation, or mount startup in measured workload timing
- run warmup iterations before measured iterations
- keep every raw sample and report summary statistics, not a single run
- compare ScreenFS against the same workload on the native backing source when possible
- record environment metadata, commands, workload sizes, and policy inputs with the result
- treat cache state explicitly; the default harness evidence is warm-cache local evidence unless a run records separate cache-control steps

The harness intentionally uses only Python standard-library operations plus the built `screenfs` binary. It does not add benchmark-only Rust dependencies to the product crate. For lower-level storage work, use external tools such as `fio` only with a separately recorded job file and environment notes.

## Workloads

The default benchmark creates a temporary source tree and runs these workloads through both the native source path and the ScreenFS mount path:

| workload | Purpose |
| --- | --- |
| `seq_read` | already-open data path read throughput surface |
| `seq_write` | write syscall/data path overhead, without timing fixture setup |
| `small_stat_open_read` | metadata-heavy small-file stat/open/read path |
| `readdir_lstat` | directory listing plus metadata path |
| `symlink_open_read` | visible symlink dereference path |

It also runs this ScreenFS-only contract workload:

| workload | Purpose |
| --- | --- |
| `hidden_stat_miss` | hidden path `ENOENT` path and matcher overhead |

The default policy is intentionally simple but non-empty:

```text
--visibility-default visible
--hidden /.screenfs-bench/hidden
--mutability-default writable
--readonly /.screenfs-bench/readonly
```

Use `--extra-screenfs-arg` for additional one-off policy experiments, but record the full command from the JSON output when comparing results.

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

Useful sizing options:

```text
--read-mib 64
--write-mib 64
--small-files 2000
--dir-entries 5000
--hidden-misses 2000
--iterations 10
--warmups 3
```

Use larger values for stable release evidence. Use smaller values only for smoke-checking the harness itself.

## Reading results

The JSON output is the source of truth. The Markdown output is a human-readable summary, and the optional SVG output is a box plot of raw sample timings. Summaries include p50/median, p90, p95, and p99. Comparable workloads include a `mounted_over_native_median` ratio:

- `1.0`: ScreenFS median approximately matches native path for this workload
- `>1.0`: ScreenFS mount is slower than native path
- `<1.0`: ScreenFS mount measured faster; treat this cautiously and inspect sample variance/cache effects

Do not compare results from different machines, kernels, storage devices, CPU governors, or policy inputs unless those differences are the subject of the experiment.

## Evidence requirements for performance changes

When a performance change is proposed or merged, record at least:

- git revision and whether the worktree was clean
- command line used for the benchmark
- JSON result path or attached result
- kernel, `fusermount3`, rustc/cargo, backing filesystem, CPU/storage notes when relevant
- workload sizes, warmups, iterations, and cache-control assumptions
- before/after p50/median ratios, p90/p95/p99 tail latency, and raw-sample variance for the affected workload
- correctness validation command after the change, typically `cargo test --all-targets --all-features`

For host-side async/io_uring experiments, also follow the selective data-path evidence rules in [`docs/operations.md`](operations.md) and [`docs/artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md).

## Guardrails

- Do not use benchmark results to relax hidden `ENOENT`, bridge-visible, symlink target, or mutability precedence semantics.
- Do not treat warm-cache benchmark evidence as cold-cache or storage-device evidence.
- Do not run privileged mount or namespace steps from this harness; ScreenFS benchmark evidence remains non-root FUSE evidence, and the harness exits when run as root.
- Do not check in large generated result files unless they are explicitly accepted as current baseline artifacts.
