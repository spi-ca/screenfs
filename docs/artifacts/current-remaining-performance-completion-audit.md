# Current remaining performance completion audit

This audit maps the current remaining-performance objective to checked-in evidence. It is a documentation-only completion readout, not a new performance claim.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Readdirplus was closed through a document-first review loop before calling the remaining scope complete | [`../benchmarks.md`](../benchmarks.md) now defines the five-row scope labels and the `readdirplus` page/scan gate as `slice` until a same-machine before/after pair clears the p95/p99 and counter requirements; [`../performance-roadmap.md`](../performance-roadmap.md) and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) keep the same remaining-row snapshot instead of promoting a broad `readdirplus` claim. | Satisfied |
| The p99 / counter doc fixes landed and no final doc findings remain | [`../benchmarks.md`](../benchmarks.md) explicitly requires `readdirplus` p50+p95+p99 tails plus non-zero `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, and `readdirplus_page_commit`; [`current-goal-completion-audit.md`](current-goal-completion-audit.md) records that reviewer findings were cleared and the latest result was no findings / no blocking findings; [`current-performance-followup-completion-audit.md`](current-performance-followup-completion-audit.md) records no remaining docs-scope blocker. | Satisfied |
| The attempted subagent-delivered readdirplus page/scan path-join implementation is preserved as an explicit experiment record with before/after artifacts | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) records the rejected page/scan path-join experiment. The bundle keeps paired 5k directory-surface artifacts (`before-directory-surface.{json,md,svg}`, `after-directory-surface.{json,md,svg}`) and paired focused 20k artifacts (`before-readdirplus-20k.{json,md,svg}`, `after-readdirplus-20k.{json,md,svg}`). | Satisfied |
| Rejection reason and code-revert evidence are explicit | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) records the rejection reason directly: 5k `readdir_basic` regressed badly (`1.557941x` p50, `1.584943x` p95, `1.594716x` p99) and the focused 20k `readdirplus_basic` row still regressed at p50 (`1.036241x`) even with better tails/counters. The same summary states that the code changes were reverted and are not part of the current implementation. Revert evidence is also visible in the experiment artifact metadata: [`readdirplus-page-scan-path-join/after-readdirplus-20k.json`](readdirplus-page-scan-path-join/after-readdirplus-20k.json) captured dirty files including `src/fs/backing.rs`, `src/path.rs`, and `src/path_tests.rs`, while the current worktree source diff now shows only `src/fs.rs`, `src/fs/guards.rs`, and `src/fs/tests/perf.rs` as modified, supporting that the path-join experiment code was not kept. | Satisfied |
| Remaining candidate statuses stay preserved in one current snapshot | [`current-next-performance-candidates.md`](current-next-performance-candidates.md) and the current shortlist table in [`../performance-roadmap.md`](../performance-roadmap.md) agree on the active labels: `readdirplus` page/scan=`slice`, read/write small-I/O guard cost=`claim`, mutation invalidation breadth=`smoke`, `open_confined` / `openat2` frequency=`smoke`, matcher-heavy policy path=`smoke`. | Satisfied |
| The kept read/write fallback claim remains preserved while readdirplus stays unresolved | [`../benchmarks.md`](../benchmarks.md) records the checked-in read/write claim scope as the `fallback-unsafe-policy` row in [`read-write-small-io-guard-reuse/summary.md`](read-write-small-io-guard-reuse/summary.md); [`current-next-performance-candidates.md`](current-next-performance-candidates.md) keeps that row at `claim` and keeps `readdirplus` page/scan at `slice`; [`current-performance-followup-completion-audit.md`](current-performance-followup-completion-audit.md) repeats that the kept read/write claim is narrow and separate from the unresolved `readdirplus` gap. | Satisfied |
| Validation log path is explicit for the remaining-scope completion check | Current validation is recorded in [`current-remaining-performance-validation.log`](current-remaining-performance-validation.log), including Python compile/tests, `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`, JSON parse checks, `git diff --check`, and required file-list commands. | Satisfied |
| Review results are recorded and end with no blocking findings | [`current-goal-completion-audit.md`](current-goal-completion-audit.md) records that reviewer checks found and then cleared the deferred-`readdirplus` review issues and ended with no findings / no blocking findings. [`read-write-small-io-guard-reuse/review-closure.md`](read-write-small-io-guard-reuse/review-closure.md) records reviewer, researcher, security-reviewer, and final docs/artifacts re-review closure with no blocking findings. | Satisfied |

## Readdirplus kept vs. rejected evidence

| scope | evidence | current reading |
| --- | --- | --- |
| kept `readdirplus` slice | [`readdirplus-deferred-attrs/summary.md`](readdirplus-deferred-attrs/summary.md) | kept only as attr-work reduction for non-returned entries; 20k focused row still regresses slightly, so this stays `slice`, not a broad latency claim |
| rejected `readdirplus` experiment | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) | rejected page/scan retuning; before/after artifacts are archival evidence only |
| current priority snapshot | [`current-next-performance-candidates.md`](current-next-performance-candidates.md) | `readdirplus` page/scan remains open, and no rejected path-join implementation is counted as kept evidence |

## Remaining candidate snapshot

| candidate | label | evidence |
| --- | --- | --- |
| `readdirplus` page/scan | `slice` | [`readdirplus-deferred-attrs/summary.md`](readdirplus-deferred-attrs/summary.md), [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md), [`current-next-performance-candidates.md`](current-next-performance-candidates.md) |
| read/write small-I/O guard cost | `claim` | [`read-write-small-io-guard-reuse/summary.md`](read-write-small-io-guard-reuse/summary.md), [`read-write-small-io-guard-reuse/validation.log`](read-write-small-io-guard-reuse/validation.log) |
| mutation invalidation breadth | `smoke` | [`current-mutation-invalidation-set-smoke.md`](current-mutation-invalidation-set-smoke.md) |
| `open_confined` / `openat2` frequency | `smoke` | [`current-open-confined-surface-smoke.md`](current-open-confined-surface-smoke.md), [`current-metadata-opendir-smoke.md`](current-metadata-opendir-smoke.md) |
| matcher-heavy policy path | `smoke` | [`current-policy-heavy-matrix-smoke.md`](current-policy-heavy-matrix-smoke.md) |

## Completion conclusion

The remaining-performance scope is documented to a verifiable stopping point:

- the readdirplus review/docs loop now points to p99 and split-counter gates instead of a premature completion claim;
- the subagent-delivered page/scan path-join attempt is preserved only as rejected evidence with before/after artifacts and explicit revert notes;
- the kept read/write fallback claim stays preserved separately;
- `docs/artifacts/current-remaining-performance-validation.log` records the current validation pass;
- recorded review results end with no blocking findings.

No `readdirplus` code from the rejected path-join experiment is kept as current implementation or counted as accepted performance evidence.
