# Current matcher-heavy series wrapper completion audit

Goal ID: `89acc27a-f09a-421e-8ae3-23191b30953a`

This audit maps the active matcher-heavy measurement-base objective to current evidence. It is a tooling/readout document, not a performance claim.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Continue matcher-heavy work without relying on a single before/after pair | `scripts/bench-screenfs-series.py` adds a thin repeated/interleaved wrapper around `scripts/bench-screenfs.py`. It supports `--pairs`, `--order before-after|after-before|alternate`, explicit before/after binaries and labels, child artifact paths, a manifest JSON, and a summary Markdown with per-pair after/before p50/p95/p99 ratios. | Satisfied |
| Preserve existing benchmark harness behavior | The wrapper calls the existing harness as a subprocess and rejects managed passthrough args (`--screenfs-bin`, `--output-json`, `--output-md`, `--output-svg`) so child output/provenance is owned by the wrapper. It does not change `scripts/bench-screenfs.py` benchmark execution semantics. | Satisfied |
| Avoid repeating rejected matcher optimizations | No matcher optimization was implemented for this goal. The wrapper is measurement tooling only; it does not reintroduce no-writable mutability fast path, visible-default short-circuit, subtree-only best-candidate, rank-zero early-stop, or empty-family lookup skip. | Satisfied |
| Preserve ScreenFS visibility/mutability guardrails | No FUSE policy or matcher semantics code was changed by this measurement-base step. The wrapper operates outside ScreenFS request handling and only orchestrates benchmark subprocesses. | Satisfied |
| Provide smoke evidence for the repeated/interleaved workflow | [`matcher-heavy-series-wrapper-smoke/summary.md`](matcher-heavy-series-wrapper-smoke/summary.md) records a tooling smoke using the same binary on both sides, `--pairs 2`, and `--order alternate`. The generated [`matcher-heavy-series-wrapper-smoke/smoke-summary.md`](matcher-heavy-series-wrapper-smoke/smoke-summary.md) shows pair 1 before→after and pair 2 after→before execution order. | Satisfied |
| Document how future matcher-heavy claims should use the wrapper | [`../benchmarks.md`](../benchmarks.md) documents `scripts/bench-screenfs-series.py` in the matcher-heavy gate section and states that its smoke artifact is tooling evidence only, not performance evidence. The same section still requires repeated/interleaved evidence and keeps matcher-heavy at `smoke`. | Satisfied |
| Add tests for wrapper behavior | `scripts/test_bench_screenfs_series.py` covers alternating pair order, arbitrary summary workload names, managed-arg rejection, child argv/output naming, mounted and ScreenFS-only ratio calculation, inferred common workload summaries, and custom manifest/summary parent directory creation. | Satisfied |
| Validate and review | Observed validation includes Python compile/unittests, `git diff --check`, `cargo fmt --check`, `cargo +nightly-2026-02-19 check`, `cargo +nightly-2026-02-19 clippy --all-targets --all-features`, `CARGO_INCREMENTAL=0 cargo +nightly-2026-02-19 test --all-targets --all-features`, and the required `/usr/bin/find` file-list checks. [`current-matcher-series-wrapper-review-closure.md`](current-matcher-series-wrapper-review-closure.md) records the exact commands, review findings, fixes, regression coverage, and final review state for the wrapper scope. | Satisfied |

## Current conclusion

The repeated/interleaved matcher-heavy measurement base is complete for the current objective:

- it provides a reusable wrapper for repeated/interleaved benchmark series;
- it preserves existing benchmark and ScreenFS semantics;
- it has unit tests and a same-binary tooling smoke artifact;
- it does not claim any matcher-heavy speedup.

Matcher-heavy policy path remains `smoke` / no kept speedup. Future implementation attempts should use this wrapper, plus the previously recorded same-binary control envelopes, before promoting any matcher-heavy claim.
