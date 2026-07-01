# Current benchmark continuation audit

This audit maps the active benchmark/performance continuation work to concrete evidence in the current worktree. It is not a new speedup claim by itself.

## Benchmark surfaces added or refreshed

- Mutation invalidation smoke coverage:
  - `current-mutation-invalidation-smoke.{json,md,svg}`
  - `current-pinned-mutation-invalidation-smoke.{json,md,svg}`
  - `current-mutation-invalidation-set-smoke.{json,md,svg}`
  - Added `invalidations.scanned_entries` attribution and mutation-invalidation workload selection.
- Directory symlink visibility smoke coverage:
  - `current-directory-symlink-surface-smoke.{json,md,svg}`
  - Added `directory-symlink-surface`, `readdir_symlink_visibility`, and `readdirplus_symlink_visibility` with symlink fanout validation.
- Open-confined/opendir smoke coverage:
  - `current-open-confined-surface-smoke.{json,md,svg}`
  - `current-metadata-opendir-smoke.{json,md,svg}`
  - Added `open-confined-surface` and focused `metadata_opendir` coverage.
- Policy/matcher hot-path smoke coverage:
  - `current-policy-heavy-matrix-smoke.{json,md,svg}`
  - Refreshed `policy-heavy-matrix` with `--matcher-extra-rules 32` and current matcher family/order counters.
- Read/write data-path smoke coverage:
  - `current-read-write-surface-smoke.{json,md,svg}`
  - Added `read-write-surface` named set for sequential, small, and random read/write rows with current data-path split counters.
- Readdirplus attr-work experiment:
  - `readdirplus-deferred-attrs/summary.md` plus before/after JSON/Markdown/SVG artifacts.
  - Kept a scoped implementation that defers `readdirplus` child attr generation until after candidate page selection, reducing `readdirplus_attr_generation_entries` for non-returned entries.
  - This is attribution-backed attr-work reduction, not a broad latency speedup claim across directory sizes: 5k `directory-surface` improved, while 20k focused `readdirplus_basic` regressed slightly.

## Rejected experiments recorded

- `mutation-invalidation-range-scan/summary.md`: ordered-subtree invalidation scan experiment was not kept because mounted before/after evidence showed no latency win.
- `directory-symlink-resolver-reuse/summary.md`: request-local resolver reuse for directory symlink target visibility reduced one counter but worsened mounted latency, so the implementation was reverted and is not speedup evidence.

## Maintained speedup evidence scoped separately

- `post-metadata-follow-up-claim/worktree-3cb95ba/summary.md` remains scoped to directory/read-only-close follow-up evidence.
- `post-metadata-follow-up-claim/worktree-3cb95ba/validation-current.log` records validation for the maintained directory/read-only-close claim scope.
- `current-benchmark-continuation-validation.log` records fresh validation after the latest continuation artifacts, including the policy-heavy smoke.
- Metadata/open-path guard-context evidence remains a separate claim scope under `metadata-open-path-claim/`.

## Current validation evidence

Observed current benchmark refresh evidence is recorded in `current-benchmark-rerun.log`; validation is recorded in `current-benchmark-continuation-validation.log` and includes:

- Python harness tests: `scripts/test_bench_screenfs.py` and `scripts/test_bench_screenfs_workloads.py`.
- Rust checks: `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, and `cargo test --all-targets --all-features`.
- Artifact checks: new/current JSON artifacts parse successfully with `python3 -m json.tool`; `git diff --check` passes.
- Documentation/project file checks use `/usr/bin/find README.md AGENTS.md docs -maxdepth 2 -type f -print` and `/usr/bin/find .pi/agents .pi/skills .pi/prompts .pi/extensions -maxdepth 3 -type f -print | sort`.

## Completion status

The worktree now has broader benchmark coverage, corrected evidence scope for accepted/rejected performance work, and a kept `readdirplus` attr-work reduction with before/after attribution evidence. The kept `readdirplus` change should be read narrowly: it reduces unnecessary attr generation for non-returned entries, but mixed-size artifacts do not support a broad latency speedup claim. Do not mark the active goal complete unless a final completion audit maps the existing scoped speedup evidence, the new benchmark coverage, the kept attr-work reduction, and all validation requirements to current files and command output without unresolved review findings.
