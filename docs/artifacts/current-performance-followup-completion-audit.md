# Current performance follow-up completion audit

This audit maps the current performance follow-up goal to checked-in evidence and current-worktree implementation. It is a completion readout, not a new blanket speedup claim.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Initial parallel doc review and findings loop landed in the final docs | `docs/benchmarks.md`, `docs/performance-roadmap.md`, `docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md`, and `docs/artifacts/current-next-performance-candidates.md` now agree on the five-row priority model, explicit `smoke`/`slice`/`claim` labels, fallback-only read/write claim scope, and the non-claim status of matcher-descendant and broad `readdirplus` wins. The doc changes are corrective, not promotional. | Satisfied |
| Docs were updated and left with no final doc-scope gap | `docs/benchmarks.md` now defines row-specific gates; `docs/performance-roadmap.md` now treats metadata/open-path as an umbrella, not the sole next task; `docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md` now states that the checked-in glob directory row is non-regression only; `docs/artifacts/current-next-performance-candidates.md` centralizes honest scope notes. | Satisfied |
| Code implementation landed for the subagent-delivered read/write small-I/O request-local resolver reuse slice | The kept bundle and source paths document request-local `RequestPathResolver` reuse across the read/write path guard and opened-fd revalidation path instead of constructing separate path-resolution work; `src/fs/tests/perf.rs` keeps counter assertions proving the reuse is request-local while both path and opened-fd revalidation still occur. | Satisfied |
| Test-focused pass exists for the kept follow-up slice | `docs/artifacts/read-write-small-io-guard-reuse/validation.log` records `python3 -m py_compile`, `python3 scripts/test_bench_screenfs.py`, `python3 scripts/test_bench_screenfs_workloads.py`, `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, and `cargo test --all-targets --all-features` passing. The log includes `187 passed` unit tests plus the `src/main.rs` mount-options test. | Satisfied |
| Code / security / docs review closed without a remaining blocker | `docs/artifacts/read-write-small-io-guard-reuse/review-closure.md` now records the current-scope subagent closure directly: code reviewer finished with no findings after reviewing code/tests/docs/artifacts, researcher found no blocking artifact-consistency issue, security-reviewer found no blocking issue, and the final docs/artifacts pass after adding the direct validation-log reference was re-reviewed with no blocking findings. Command execution evidence for the same scope remains in `docs/artifacts/read-write-small-io-guard-reuse/validation.log`. | Satisfied |
| Benchmark artifacts and claim scope are recorded clearly | `docs/artifacts/read-write-small-io-guard-reuse/summary.md` is the kept claim-grade bundle for the read/write slice and says the claim scope is the `fallback-unsafe-policy` row only; `docs/artifacts/readdirplus-deferred-attrs/summary.md` is a kept `slice`; `docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md` remains a separate directory/read-only-close claim; `docs/artifacts/current-next-performance-candidates.md` ties all current rows together. | Satisfied |
| Five active candidate statuses are preserved in one current snapshot | `docs/artifacts/current-next-performance-candidates.md` records: `readdirplus` page/scan=`slice`, read/write follow-up beyond fallback small-I/O baseline=`smoke`, mutation invalidation breadth=`smoke`, `open_confined`/`openat2` frequency=`claim`, matcher-heavy policy path=`smoke`. The kept fallback read/write small-I/O bundle remains a baseline claim inside that broader follow-up lane. The immediate next unfinished lane is now `readdirplus` page/scan. | Satisfied |
| Validation log path is explicit | Primary kept-bundle validation: `docs/artifacts/read-write-small-io-guard-reuse/validation.log`. Broader continuation/docs validation: `docs/artifacts/current-benchmark-continuation-validation.log`. | Satisfied |

## Five-candidate snapshot

| candidate | current status | evidence |
| --- | --- | --- |
| `readdirplus` page/scan | `slice` | `docs/artifacts/readdirplus-deferred-attrs/summary.md` + `docs/artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md` |
| read/write follow-up beyond fallback small-I/O baseline | `smoke` | `docs/artifacts/read-write-small-io-guard-reuse/summary.md` remains the kept fallback baseline claim; broader current follow-up artifacts are coverage-only |
| mutation invalidation breadth | `smoke` | `docs/artifacts/current-mutation-invalidation-set-smoke.md` |
| `open_confined` / `openat2` frequency | `claim` | `docs/artifacts/open-confined-stat-child-open-path/summary.md` + historical context in `docs/artifacts/current-open-confined-surface-smoke.md`, `docs/artifacts/current-metadata-opendir-smoke.md` |
| matcher-heavy policy path | `smoke` | `docs/artifacts/current-policy-heavy-matrix-smoke.md` |

## Remaining risks

- The kept read/write claim is still narrow: fallback policy only; no separate fast-policy, storage-backed, or concurrency claim exists.
- `readdirplus` evidence still does not prove a broad page/scan latency win across directory sizes; the 20k focused row remains a caveat.
- Mutation invalidation and matcher-heavy rows still need dedicated before/after claim-grade pairs; future `open_confined` changes also need fresh same-matrix evidence beyond the current direct stat-child opened-object claim.
- Matcher-descendant proof is still weak for directory-heavy non-empty visible descendant cases; current matcher32 smoke is not enough for that claim.
