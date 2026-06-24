# Matcher empty-family lookup skip

Status: **rejected / not kept**

## Candidate

Skip repeated empty `direct_child_glob_by_anchor` lookup work in `src/matcher/index.rs` path-candidate traversal when the compiled matcher has no direct-child glob rules. The current matcher32 smoke shape is subtree-only (`matcher_family_candidates.direct_child_glob=0`, `recursive=0`), so this looked like a low-risk way to reduce ancestor-loop map probes without changing candidate ordering or public matcher APIs.

## Validation

Correctness smoke while the change was present:

```bash
CARGO_INCREMENTAL=0 cargo +nightly-2026-02-19 test matcher::index::tests
```

Result: 4 passed.

The default stable toolchain still failed in the existing `compio-runtime 0.11.0` dependency path; benchmark binaries were therefore built with the already-used `nightly-2026-02-19` toolchain and `--features perf-counters`.

## Benchmark evidence

All rows used non-root ScreenFS benchmark harness, warm cache, `--iterations 10 --warmups 3`, `--matcher-extra-rules 32`, and `--policy-preset fallback-unsafe-policy`.

### First policy-heavy pair looked promising

Artifacts:

- [`before-policy-heavy.json`](before-policy-heavy.json) / [`before-policy-heavy.md`](before-policy-heavy.md)
- [`after-policy-heavy.json`](after-policy-heavy.json) / [`after-policy-heavy.md`](after-policy-heavy.md)

After/before mounted p50 / p95 / p99:

| workload | p50 | p95 | p99 |
| --- | ---: | ---: | ---: |
| `metadata_lookup` | `0.680x` | `0.668x` | `0.668x` |
| `metadata_getattr` | `0.710x` | `0.774x` | `0.778x` |
| `metadata_access` | `0.744x` | `0.753x` | `0.743x` |
| `matcher_hidden_stat_miss` | `0.732x` | `0.713x` | `0.697x` |
| `matcher_readonly_access_wok` | `0.910x` | `0.917x` | `0.921x` |

### Rerun failed the policy-heavy gate

Artifacts:

- [`before-policy-heavy-rerun.json`](before-policy-heavy-rerun.json) / [`before-policy-heavy-rerun.md`](before-policy-heavy-rerun.md)
- [`after-policy-heavy-rerun.json`](after-policy-heavy-rerun.json) / [`after-policy-heavy-rerun.md`](after-policy-heavy-rerun.md)

After/before mounted p50 / p95 / p99:

| workload | p50 | p95 | p99 |
| --- | ---: | ---: | ---: |
| `metadata_lookup` | `0.936x` | `0.891x` | `0.891x` |
| `metadata_getattr` | `0.902x` | `0.898x` | `0.900x` |
| `metadata_access` | `1.361x` | `0.988x` | `0.988x` |
| `matcher_hidden_stat_miss` | `1.409x` | `1.451x` | `1.505x` |
| `matcher_readonly_access_wok` | `1.361x` | `1.236x` | `1.202x` |

This misses the matcher-heavy claim gate because `matcher_hidden_stat_miss` regressed materially and no stable primary workload improvement remained.

### Descendant companion was also unstable

The first descendant companion pair was noisy and had median regressions; those artifacts are preserved as [`before-descendant.json`](before-descendant.json) / [`after-descendant.json`](after-descendant.json). The immediate rerun improved and is preserved as:

- [`before-descendant-rerun.json`](before-descendant-rerun.json) / [`before-descendant-rerun.md`](before-descendant-rerun.md)
- [`after-descendant-rerun.json`](after-descendant-rerun.json) / [`after-descendant-rerun.md`](after-descendant-rerun.md)

Rerun after/before mounted p50 / p95 / p99:

| workload | p50 | p95 | p99 |
| --- | ---: | ---: | ---: |
| `matcher_descendant_readdir` | `0.809x` | `0.880x` | `0.834x` |
| `matcher_descendant_readdirplus` | `0.802x` | `0.812x` | `0.771x` |

Because the policy-heavy rerun failed, the descendant rerun is context only and does not rescue the candidate.

## Decision

Rejected and not kept. The `src/matcher/index.rs` change was reverted from the worktree after measurement.

Reason: initial policy-heavy numbers looked claim-grade, but an immediate same-shape rerun regressed `matcher_hidden_stat_miss` and `matcher_readonly_access_wok` badly. This candidate is too noisy / not robust enough to claim or keep.

## Follow-up guidance

- Do not retry this exact empty-family lookup skip as a kept optimization without stronger repeated-pair evidence.
- The signal still suggests matcher traversal overhead is measurable, but future work should use repeated or interleaved before/after pairs before treating a single good policy-heavy run as claim-grade.
