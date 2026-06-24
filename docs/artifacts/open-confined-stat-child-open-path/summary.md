# open-confined stat-child opened-object path

## Scope

This bundle records a claim-grade before/after pair for the `open_confined` / `openat2` frequency lane after changing `stat_child_no_follow()` to use a direct confined opened-object metadata path for non-root targets (`open_confined(path, O_PATH|O_NOFOLLOW)` + `fstat()`). The readlink preparation path remains parent-dirfd based because `readlinkat()` needs the parent dirfd.

## Commands

All runs used non-root `scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3` and wrote JSON/Markdown/SVG artifacts in this directory.

- `before-head-fallback-open-confined-surface.*`: `--policy-preset fallback-unsafe-policy --workload-set open-confined-surface`
- `after-worktree-fallback-open-confined-surface.*`: `--policy-preset fallback-unsafe-policy --workload-set open-confined-surface`
- `before-head-fast-open-confined-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set open-confined-surface`
- `after-worktree-fast-open-confined-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set open-confined-surface`
- `before-head-fallback-metadata-opendir.*`: `--policy-preset fallback-unsafe-policy --workload metadata_opendir`
- `after-worktree-fallback-metadata-opendir.*`: `--policy-preset fallback-unsafe-policy --workload metadata_opendir`
- `before-head-fallback-metadata-context.*`: `--policy-preset fallback-unsafe-policy --workload metadata_lookup --workload metadata_getattr --workload metadata_access`
- `after-worktree-fallback-metadata-context.*`: `--policy-preset fallback-unsafe-policy --workload metadata_lookup --workload metadata_getattr --workload metadata_access`

## Result summary

Mounted latency after/before ratios, lower is better:

| row | workload | p50 | p95 | p99 | gate |
| --- | --- | ---: | ---: | ---: | --- |
| fallback `open-confined-surface` | `metadata_open` | 0.559x | 0.532x | 0.516x | pass |
| fallback `open-confined-surface` | `metadata_opendir` | 0.597x | 0.567x | 0.553x | pass |
| fast `open-confined-surface` | `metadata_open` | 0.505x | 0.596x | 0.598x | pass |
| fast `open-confined-surface` | `metadata_opendir` | 0.646x | 0.558x | 0.562x | pass |
| fallback focused | `metadata_opendir` | 0.802x | 0.806x | 0.798x | context pass |
| fallback metadata context | `metadata_access` | 1.048x | 1.031x | 1.031x | non-regression pass |
| fallback metadata context | `metadata_getattr` | 1.027x | 1.026x | 1.017x | non-regression pass |
| fallback metadata context | `metadata_lookup` | 0.671x | 0.774x | 0.780x | context pass |

The primary `open-confined-surface` claim gate requires both `metadata_open` and `metadata_opendir` in the targeted row to be `<= 0.90x` median and `<= 0.95x` at p95/p99. Both fallback and fast rows pass. Companion metadata rows touched by the same change remain within `<= 1.05x` median and `<= 1.10x` at p95/p99.

## Counter attribution

Fallback `open-confined-surface` process-lifetime perf counters show the intended split movement:

| counter | before count / total ns | after count / total ns |
| --- | ---: | ---: |
| `stat_child_no_follow` | 53255 / 468613478 | 53255 / 105089579 |
| `stat_child_no_follow.directory_revalidation` | 53254 / 328881543 | absent |
| `stat_child_no_follow.host_fstatat` | 53254 / 23700284 | absent |
| `stat_child_no_follow.host_fstat` | 1 / 2248 | 53253 / 7703842 |
| `open_confined_openat2` | 66568 / 86140402 | 66568 / 87018046 |
| `resolved_virtual_path_from_open_fd` | 66567 / 78299532 | 13313 / 15447780 |

Interpretation: the kept speedup comes from removing the parent-dirfd directory revalidation/fstatat path from direct stat-child metadata work, not from replacing `openat2` confinement or adding a cross-request cache.

## Correctness and review evidence

Validation and review completed in the work session:

- `git diff --check`: pass
- `cargo fmt --check`: pass
- `cargo check --features perf-counters`: pass
- `cargo test --features perf-counters fs::tests::perf`: pass
- `cargo test --features perf-counters fs::tests::symlinks_access_create::symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent`: pass
- `cargo test --all-targets --all-features`: pass, 191 tests
- `cargo clippy --all-targets --all-features`: pass
- `cargo check`: pass
- Implementation review: no findings
- Security review: no findings
- Documentation review of the updated canonical performance docs: no findings

## Scope boundaries

- This is a workload-scoped `open-confined-surface` speedup claim, not a blanket metadata/open-path claim.
- The focused `metadata_opendir` row is supporting context; the claim is kept because the full fallback and fast `open-confined-surface` rows passed.
- The old `stat_child_no_follow.directory_revalidation` smoke totals are now pre-change attribution only for direct opened-object metadata paths.
- The change does not relax `openat2(RESOLVE_IN_ROOT|RESOLVE_NO_MAGICLINKS)`, hidden `ENOENT`, symlink target visibility, or no-cross-request-cache guardrails.
