# Current matcher-heavy continuation completion audit

Goal ID: `83f194f7-2da6-4fcc-9b28-8a723e165942`

This audit maps the active matcher-heavy continuation objective to current checked-in evidence. It is a completion/readout document, not a new performance claim.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Continue matcher-heavy performance work without repeating prior rejected attempts | The current docs keep prior rejected attempts as rejected-only evidence: [`matcher-no-writable-mutability-fast-path/summary.md`](matcher-no-writable-mutability-fast-path/summary.md), [`matcher-visible-default-short-circuit/summary.md`](matcher-visible-default-short-circuit/summary.md), [`matcher-subtree-only-best-candidate/summary.md`](matcher-subtree-only-best-candidate/summary.md), and [`matcher-rank-zero-early-stop/summary.md`](matcher-rank-zero-early-stop/summary.md). The new attempted candidate is separate: [`matcher-empty-family-lookup-skip/summary.md`](matcher-empty-family-lookup-skip/summary.md). | Satisfied |
| Use measure-first evidence before keeping a matcher-heavy optimization | [`matcher-empty-family-lookup-skip/summary.md`](matcher-empty-family-lookup-skip/summary.md) records the measured policy-heavy and descendant pairs before deciding. The first policy-heavy pair looked promising, but the immediate same-shape rerun failed the gate. | Satisfied |
| Preserve ScreenFS visibility/mutability guardrails | The empty-family candidate was reverted/not kept, so no matcher code change remains. Review confirmed `src/matcher/index.rs` has no remaining empty-family skip diff and no residual guardrail risk from this attempt. Existing code changes are limited to matcher attribution/probe surfaces and documented rejected/coverage artifacts; no policy semantics relaxation is claimed. | Satisfied |
| Claim only if repeated evidence clears the gate | No matcher-heavy speedup is claimed. [`matcher-empty-family-lookup-skip/summary.md`](matcher-empty-family-lookup-skip/summary.md) is explicitly `rejected / not kept`. [`current-next-performance-candidates.md`](current-next-performance-candidates.md), [`../performance-roadmap.md`](../performance-roadmap.md), and [`../benchmarks.md`](../benchmarks.md) keep matcher-heavy policy path at `smoke`. | Satisfied |
| Preserve failed experiments as rejected evidence | [`matcher-empty-family-lookup-skip/summary.md`](matcher-empty-family-lookup-skip/summary.md) preserves before/after and rerun artifacts for `policy-heavy-matrix` plus descendant companion artifacts. It records why the candidate was rejected: the policy-heavy rerun regressed `matcher_hidden_stat_miss` and `matcher_readonly_access_wok` materially. | Satisfied |
| Account for benchmark noise before future matcher-heavy claims | [`matcher-heavy-no-code-control/summary.md`](matcher-heavy-no-code-control/summary.md) records same-binary `policy-heavy-matrix` control pairs showing large apparent no-candidate movement, including `matcher_hidden_stat_miss` p50 `0.696x`-`1.089x` and p99 `0.785x`-`1.232x`. [`matcher-heavy-descendant-no-code-control/summary.md`](matcher-heavy-descendant-no-code-control/summary.md) records same-binary `matcher-descendant-directory` control pairs, including `matcher_descendant_readdirplus` p50 `0.895x`-`1.225x`. | Satisfied |
| Update source-of-truth docs so future work uses stronger gates | [`../benchmarks.md`](../benchmarks.md) now requires repeated or interleaved same-shape matcher-heavy pairs rather than a single pair. [`../performance-roadmap.md`](../performance-roadmap.md) and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) reference both same-binary controls and keep matcher-heavy at `smoke` / no kept speedup. | Satisfied |
| Complete validation and review | [`current-matcher-heavy-continuation-validation.log`](current-matcher-heavy-continuation-validation.log) records the observed validation commands and passing results: Python compile, Python unittests, `git diff --check`, `cargo fmt --check`, and targeted matcher index Rust tests with `CARGO_INCREMENTAL=0 cargo +nightly-2026-02-19 test matcher::index::tests`. [`current-matcher-heavy-continuation-review-closure.md`](current-matcher-heavy-continuation-review-closure.md) records read-only implementation/doc review closure and the audit-provenance fixes. | Satisfied |

## Current conclusion

The matcher-heavy continuation reached a verifiable stopping point:

- no matcher-heavy optimization is kept;
- the only new implementation candidate, empty-family lookup skip, is rejected and reverted;
- same-binary controls explain why single-pair matcher-heavy evidence is insufficient;
- source-of-truth docs now require repeated/interleaved evidence for future matcher-heavy claims;
- review closure and validation provenance are recorded in repo-local artifacts.

Therefore the active continuation can be closed as completed-without-claim evidence: the objective allowed failed experiments to be preserved as rejected evidence when no safe claimable improvement survived validation.
