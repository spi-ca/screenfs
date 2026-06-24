# Current matcher-heavy series wrapper review closure

This note records repo-local review closure for the matcher-heavy repeated/interleaved series wrapper.

## Review findings and resolution

Initial read-only implementation review found:

1. `--workload` summary filtering was restricted to a small hard-coded set of names.
   - Resolution: `scripts/bench-screenfs-series.py` no longer restricts `--workload` choices, so any child harness workload name can be summarized.
   - Regression coverage: `scripts/test_bench_screenfs_series.py` verifies an arbitrary workload name such as `metadata_open` is accepted.

2. Custom `--manifest-json` / `--summary-md` parent directories were not guaranteed to exist before child benchmark runs.
   - Resolution: the wrapper creates `output_dir`, `manifest_json.parent`, and `summary_md.parent` before running children.
   - Regression coverage: `scripts/test_bench_screenfs_series.py` verifies custom nested manifest/summary paths are written.

3. The first wrapper smoke used one pair and therefore did not demonstrate alternate ordering.
   - Resolution: [`matcher-heavy-series-wrapper-smoke/summary.md`](matcher-heavy-series-wrapper-smoke/summary.md) now records a two-pair smoke with `--order alternate`; [`matcher-heavy-series-wrapper-smoke/smoke-summary.md`](matcher-heavy-series-wrapper-smoke/smoke-summary.md) shows pair 1 before→after and pair 2 after→before.

4. Programmatic `main(argv)` invocations recorded `sys.argv` instead of the effective wrapper argv in the manifest.
   - Resolution: `scripts/bench-screenfs-series.py` now passes the effective command line into `build_manifest()`.
   - Regression coverage: `scripts/test_bench_screenfs_series.py` verifies the manifest command line contains the supplied `--before-bin` and path.

## Final review state

After these fixes, final read-only re-review reported no remaining findings for the checked wrapper scope.

Checked scope included:

- `scripts/bench-screenfs-series.py`
- `scripts/test_bench_screenfs_series.py`
- `docs/benchmarks.md` matcher-heavy wrapper documentation
- `docs/artifacts/matcher-heavy-series-wrapper-smoke/summary.md`
- `docs/artifacts/current-matcher-series-wrapper-completion-audit.md`

## Validation

Observed validation for the final wrapper state:

```bash
python3 -m py_compile scripts/bench-screenfs.py scripts/bench-screenfs-series.py scripts/test_bench_screenfs.py scripts/test_bench_screenfs_workloads.py scripts/test_bench_screenfs_series.py
python3 -m unittest scripts.test_bench_screenfs scripts.test_bench_screenfs_workloads scripts.test_bench_screenfs_series
git diff --check
cargo fmt --check
cargo +nightly-2026-02-19 check
cargo +nightly-2026-02-19 clippy --all-targets --all-features
CARGO_INCREMENTAL=0 cargo +nightly-2026-02-19 test --all-targets --all-features
/usr/bin/find README.md AGENTS.md docs -maxdepth 2 -type f -print
/usr/bin/find .pi/agents .pi/skills .pi/prompts .pi/extensions -maxdepth 3 -type f -print | sort
```

All passed with `nightly-2026-02-19` for Rust validation. The default stable `cargo check` path is known in this environment to hit the existing `compio-runtime 0.11.0` dependency `E0425` issue; the wrapper change itself is Python/documentation tooling and does not alter Rust semantics.

This review closure is for measurement tooling only and does not make a matcher-heavy performance claim.
