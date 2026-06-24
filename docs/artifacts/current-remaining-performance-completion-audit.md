# Current remaining performance completion audit

This audit maps the current remaining-performance objective to checked-in evidence. It is a documentation-only completion readout, not a new performance claim.

> Note: this is a historical/current completion readout. For new rule-sensitive `DirectoryChildVisibilityBatch` / parent-local DP `scan_visibility` work, use `docs/benchmarks.md`, `docs/performance-roadmap.md`, `current-next-performance-candidates.md`, `current-matcher-descendant-directory-smoke.md`, matcher-heavy same-binary/series controls, and `readdirplus-visibility-{batch-proof,classification-feasibility}` as the source-of-truth evidence set.


## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Readdirplus was closed through a document-first review loop before calling the remaining scope complete | [`../benchmarks.md`](../benchmarks.md) now defines the five-row scope labels and records the kept fast-policy/cache-eligible `readdirplus` page-scan claim after a same-machine before/after pair cleared the p50/p95/p99 and counter requirements; [`../performance-roadmap.md`](../performance-roadmap.md) and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) keep the same scoped snapshot without promoting a broad all-policy `readdirplus` claim. | Satisfied |
| The p99 / counter doc fixes landed and no final doc findings remain | [`../benchmarks.md`](../benchmarks.md) explicitly requires `readdirplus` p50+p95+p99 tails plus non-zero `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, and `readdirplus_page_commit`; [`current-goal-completion-audit.md`](current-goal-completion-audit.md) records that reviewer findings were cleared and the latest result was no findings / no blocking findings; [`current-performance-followup-completion-audit.md`](current-performance-followup-completion-audit.md) records no remaining docs-scope blocker. | Satisfied |
| The attempted subagent-delivered readdirplus page/scan path-join implementation is preserved as an explicit experiment record with before/after artifacts | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) records the rejected page/scan path-join experiment. The bundle keeps paired 5k directory-surface artifacts (`before-directory-surface.{json,md,svg}`, `after-directory-surface.{json,md,svg}`) and paired focused 20k artifacts (`before-readdirplus-20k.{json,md,svg}`, `after-readdirplus-20k.{json,md,svg}`). | Satisfied |
| Rejection reason and code-revert evidence are explicit | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) records the rejection reason directly: 5k `readdir_basic` regressed badly (`1.557941x` p50, `1.584943x` p95, `1.594716x` p99) and the focused 20k `readdirplus_basic` row still regressed at p50 (`1.036241x`) even with better tails/counters. The same summary states that the code changes were reverted and are not part of the current implementation. Revert evidence is preserved in the experiment artifact metadata, including the dirty-file list captured by [`readdirplus-page-scan-path-join/after-readdirplus-20k.json`](readdirplus-page-scan-path-join/after-readdirplus-20k.json); current status should be read from the canonical candidate docs rather than a mutable worktree-diff file list. | Satisfied |
| Remaining candidate statuses stay preserved in one current snapshot | [`current-next-performance-candidates.md`](current-next-performance-candidates.md) and the current shortlist table in [`../performance-roadmap.md`](../performance-roadmap.md) agree on the active labels: `readdirplus` page/scan=`claim` for the fast-policy/cache-eligible batch fast path, read/write follow-up beyond fallback small-I/O baseline=`smoke`, mutation invalidation breadth=`smoke`, `open_confined` / `openat2` frequency=`claim`, matcher-heavy policy path=`smoke`. The kept fallback read/write small-I/O bundle remains a baseline claim inside that broader follow-up lane. The immediate next unfinished lane is now matcher-heavy policy path. | Satisfied |
| The kept read/write fallback baseline claim remains preserved while broader unfinished lanes stay scoped | [`../benchmarks.md`](../benchmarks.md) records the checked-in fallback read/write claim scope in [`read-write-small-io-guard-reuse/summary.md`](read-write-small-io-guard-reuse/summary.md), while [`current-next-performance-candidates.md`](current-next-performance-candidates.md) keeps the broader read/write follow-up lane at `smoke` and keeps `readdirplus` page/scan at scoped fast-policy `claim`; [`current-performance-followup-completion-audit.md`](current-performance-followup-completion-audit.md) repeats that the kept read/write baseline claim is narrow and separate from the unresolved follow-up lane. | Satisfied |
| Validation log path is explicit for the remaining-scope completion check | Current validation is recorded in [`current-remaining-performance-validation.log`](current-remaining-performance-validation.log), including Python compile/tests, `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`, JSON parse checks, `git diff --check`, and required file-list commands. | Satisfied |
| Review results are recorded and end with no blocking findings | [`current-goal-completion-audit.md`](current-goal-completion-audit.md) records that reviewer checks found and then cleared the deferred-`readdirplus` review issues and ended with no findings / no blocking findings. [`read-write-small-io-guard-reuse/review-closure.md`](read-write-small-io-guard-reuse/review-closure.md) records reviewer, researcher, security-reviewer, and final docs/artifacts re-review closure with no blocking findings. | Satisfied |

## Readdirplus kept vs. rejected evidence

| scope | evidence | current reading |
| --- | --- | --- |
| kept fast-policy `readdirplus` claim | [`readdirplus-page-scan-claim/summary.md`](readdirplus-page-scan-claim/summary.md) | kept as the fast-policy/cache-eligible page-scan visibility batch claim; not a broad all-policy, matcher-heavy, or symlink-visible claim |
| rejected `readdirplus` experiment | [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md) | rejected page/scan retuning; before/after artifacts are archival evidence only |
| current priority snapshot | [`current-next-performance-candidates.md`](current-next-performance-candidates.md) | fast-policy `readdirplus` page/scan is claim-grade, matcher-heavy no-writable mutability is rejected/not kept, the rejected path-join implementation remains archival, and matcher-heavy policy path is the next unfinished lane |

## Remaining candidate snapshot

| candidate | label | evidence |
| --- | --- | --- |
| `readdirplus` page/scan | `claim` | [`readdirplus-page-scan-claim/summary.md`](readdirplus-page-scan-claim/summary.md), with [`readdirplus-deferred-attrs/summary.md`](readdirplus-deferred-attrs/summary.md), [`readdirplus-page-scan-path-join/summary.md`](readdirplus-page-scan-path-join/summary.md), and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) as scoped/context evidence |
| read/write follow-up beyond fallback small-I/O baseline | `smoke` | [`read-write-small-io-guard-reuse/summary.md`](read-write-small-io-guard-reuse/summary.md) and [`read-write-small-io-guard-reuse/validation.log`](read-write-small-io-guard-reuse/validation.log) preserve the kept fallback baseline claim; broader current follow-up artifacts are coverage-only |
| mutation invalidation breadth | `smoke` | [`current-mutation-invalidation-set-smoke.md`](current-mutation-invalidation-set-smoke.md) |
| `open_confined` / `openat2` frequency | `claim` | [`open-confined-stat-child-open-path/summary.md`](open-confined-stat-child-open-path/summary.md), with [`current-open-confined-surface-smoke.md`](current-open-confined-surface-smoke.md) and [`current-metadata-opendir-smoke.md`](current-metadata-opendir-smoke.md) as historical attribution context |
| matcher-heavy policy path | `smoke` | [`matcher-no-writable-mutability-fast-path/summary.md`](matcher-no-writable-mutability-fast-path/summary.md), with [`current-policy-heavy-matrix-smoke.md`](current-policy-heavy-matrix-smoke.md) as attribution/context |

## Completion conclusion

The remaining-performance scope is documented to a verifiable stopping point:

- the readdirplus review/docs loop now records the scoped fast-policy/cache-eligible claim and still points future broader all-policy/matcher/symlink claims to p99 and split-counter gates;
- the subagent-delivered page/scan path-join attempt is preserved only as rejected evidence with before/after artifacts and explicit revert notes;
- the kept read/write fallback claim stays preserved separately;
- `docs/artifacts/current-remaining-performance-validation.log` records the current validation pass;
- recorded review results end with no blocking findings.

No `readdirplus` code from the rejected path-join experiment is kept as current implementation or counted as accepted performance evidence.
