# Current open-confined frequency completion audit

This audit maps the current `open_confined` / `openat2` frequency follow-up to checked-in evidence. It supersedes earlier smoke-only readings for this row.

## Requirement mapping

| requirement | current evidence | status |
| --- | --- | --- |
| Candidate selection and status are explicit | [`../benchmarks.md`](../benchmarks.md), [`../performance-roadmap.md`](../performance-roadmap.md), and [`current-next-performance-candidates.md`](current-next-performance-candidates.md) now mark the direct opened-object `stat_child_no_follow()` path under `open_confined` as a kept claim and move the immediate next unfinished lane to `readdirplus` page/scan. | Satisfied |
| Claim-grade before/after artifacts exist | [`open-confined-stat-child-open-path/summary.md`](open-confined-stat-child-open-path/summary.md) and its companion JSON/Markdown/SVG files record fallback and fast `open-confined-surface` before/after rows, plus focused fallback `metadata_opendir` and fallback metadata context rows. | Satisfied |
| Benchmark result summary clears the documented gate | The kept bundle reports fallback `metadata_open` p50/p95/p99 `0.559x`/`0.532x`/`0.516x`, fallback `metadata_opendir` `0.597x`/`0.567x`/`0.553x`, fast `metadata_open` `0.505x`/`0.596x`/`0.598x`, and fast `metadata_opendir` `0.646x`/`0.558x`/`0.562x`. Companion fallback metadata rows stay within the `<=1.05x` median and `<=1.10x` p95/p99 non-regression gate. | Satisfied |
| Counter attribution matches the implementation | The summary records `stat_child_no_follow.directory_revalidation` and `host_fstatat` disappearing from the direct metadata path while `host_fstat` becomes the direct opened-object stat path. `open_confined_openat2` confinement remains in use, and readlink preparation remains parent-dirfd based. | Satisfied |
| Rejected prior attempts remain separated | [`open-confined-frequency/summary.md`](open-confined-frequency/summary.md) and [`open-confined-fd-path/summary.md`](open-confined-fd-path/summary.md) remain rejected/slice-context evidence only and are not promoted into the kept claim. | Satisfied |
| Validation and reviews are recorded | [`open-confined-stat-child-open-path/summary.md`](open-confined-stat-child-open-path/summary.md) records passing focused and full validation, plus implementation/security/documentation review closure. | Satisfied |

## Current open-confined read

| scope | evidence | current reading |
| --- | --- | --- |
| kept claim | [`open-confined-stat-child-open-path/summary.md`](open-confined-stat-child-open-path/summary.md) | kept workload-scoped `open-confined-surface` speedup for the direct opened-object `stat_child_no_follow()` path |
| baseline smokes | [`current-open-confined-surface-smoke.md`](current-open-confined-surface-smoke.md), [`current-metadata-opendir-smoke.md`](current-metadata-opendir-smoke.md) | historical attribution baselines; useful context, not the current claim artifact |
| rejected experiments | [`open-confined-frequency/summary.md`](open-confined-frequency/summary.md), [`open-confined-fd-path/summary.md`](open-confined-fd-path/summary.md) | rejected alternatives; do not count as kept implementation evidence |

## Remaining candidate snapshot

| candidate | label | evidence |
| --- | --- | --- |
| `readdirplus` page/scan | `slice` | [`current-next-performance-candidates.md`](current-next-performance-candidates.md), [`readdirplus-deferred-attrs/summary.md`](readdirplus-deferred-attrs/summary.md) |
| read/write follow-up beyond fallback small-I/O baseline | `smoke` | [`read-write-small-io-guard-reuse/summary.md`](read-write-small-io-guard-reuse/summary.md) remains a kept fallback baseline claim, while the broader follow-up lane stays coverage-only in [`current-next-performance-candidates.md`](current-next-performance-candidates.md) |
| mutation invalidation breadth | `smoke` | [`current-mutation-invalidation-set-smoke.md`](current-mutation-invalidation-set-smoke.md) |
| `open_confined` / `openat2` frequency | `claim` | [`open-confined-stat-child-open-path/summary.md`](open-confined-stat-child-open-path/summary.md) |
| matcher-heavy policy path | `smoke` | [`current-policy-heavy-matrix-smoke.md`](current-policy-heavy-matrix-smoke.md), [`current-matcher-descendant-directory-smoke.md`](current-matcher-descendant-directory-smoke.md) |

## Completion conclusion

The current `open_confined` frequency row is no longer smoke-only. The direct opened-object `stat_child_no_follow()` path has kept claim-grade evidence, while broader metadata/open-path claims and future `open_confined` changes still require their own same-matrix before/after evidence.
