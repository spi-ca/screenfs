# readdirplus page-scan visibility batch claim

## Scope

This bundle tests the hypothesis that the fast-policy/cache-eligible `readdirplus` row improves when scan-time child visibility uses the conservative `DirectoryChildVisibilityBatch` fast path.

The comparison isolates the batch fast path by using two temporary non-destructive worktrees from the same `HEAD` plus the current worktree patch:

- `before-no-batch-*`: current patch applied, then `RuntimeConfig::directory_child_visibility_batch()` forced to return `PerEntry` so scan visibility uses the existing `ScreenFs::entry_is_readable()` path.
- `after-batch-*`: current patch applied as-is, with the trivial safe all-visible batch fast path enabled.

This keeps the current open-confined/stat-child changes and the readdir/readdirplus scan counter surface in both sides, while toggling only the directory child visibility batch shortcut.

## Commands

All formal rows used non-root `scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3` and wrote JSON/Markdown/SVG artifacts in this directory.

Claim rows:

- `before-no-batch-fast-directory-surface.*` / `after-batch-fast-directory-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set directory-surface`
- `before-no-batch-fast-readdirplus-20k.*` / `after-batch-fast-readdirplus-20k.*`: `--policy-preset fast-path-cache-eligible --dir-entries 20000 --workload readdirplus_basic`

Rule-sensitive context rows:

- `before-no-batch-fallback-matcher32-directory-surface.*` / `after-batch-fallback-matcher32-directory-surface.*`: `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- `before-no-batch-fallback-matcher32-directory-surface-rerun.*` / `after-batch-fallback-matcher32-directory-surface-rerun.*`: same command, rerun after the first context pair had mixed p50/tail movement.

## Result summary

Mounted latency after/before ratios, lower is better:

| row | workload | p50 | p95 | p99 | gate read |
| --- | --- | ---: | ---: | ---: | --- |
| fast `directory-surface` | `readdir_basic` | 0.122x | 0.211x | 0.263x | non-regression pass |
| fast `directory-surface` | `readdirplus_basic` | 0.844x | 0.844x | 0.845x | pass |
| fast focused 20k | `readdirplus_basic` | 0.587x | 0.634x | 0.631x | pass |
| fallback matcher32 context | `readdir_basic` | 1.041x | 1.026x | 1.020x | non-regression pass |
| fallback matcher32 context | `readdirplus_basic` | 1.118x | 0.978x | 0.978x | context p50 regression; rerun below |
| fallback matcher32 context rerun | `readdir_basic` | 0.889x | 0.908x | 0.916x | context pass |
| fallback matcher32 context rerun | `readdirplus_basic` | 0.888x | 0.815x | 0.815x | context pass |

The fast-policy claim rows clear the documented `readdirplus` gate: claimed `readdirplus_basic` rows are `<=0.90x` median and `<=0.95x` at p95/p99, and the paired `readdir_basic` row stays within the non-regression budget while improving materially. The matcher32 row is context only for this fast-policy claim; a rerun showed no persistent rule-sensitive regression.

## Counter attribution

Fast `directory-surface` counters:

| counter | before | after |
| --- | ---: | ---: |
| `readdirplus_directory_scan.total_ns` | 492435898 | 68664987 |
| `readdirplus_scan.scan_visibility.total_ns` | 377733942 | 2060399 |
| `readdirplus_scan.returned_policy_recheck.total_ns` | 16953933 | 9493951 |
| `policy_decision.count` | 833917 | 266229 |
| `policy_decision.total_ns` | 520998580 | 98373864 |

Fast focused 20k counters:

| counter | before | after |
| --- | ---: | ---: |
| `readdirplus_directory_scan.total_ns` | 1335045479 | 282788095 |
| `readdirplus_scan.scan_visibility.total_ns` | 985986273 | 7765745 |
| `readdirplus_scan.returned_policy_recheck.total_ns` | 14804802 | 13125035 |
| `policy_decision.count` | 4691268 | 1046710 |
| `policy_decision.total_ns` | 2412122117 | 373482773 |

Interpretation: the kept speedup is attributable to removing per-entry scan-time policy/matcher work only for the trivial safe fast-policy shape. Returned-entry policy recheck remains present, and rule-sensitive shapes still use the fallback path.

## Scope boundaries

- This is a fast-policy/cache-eligible `readdirplus` page-scan claim, not a broad matcher-heavy or fallback-policy claim.
- It does not relax hidden `ENOENT`, bridge-visible behavior, symlink target visibility, stable cookies/shared cookie domain, or returned-page-only lookup-ref pinning.
- If future work broadens the batch API to bridge-visible/descendant matching, it needs `matcher-descendant-directory` evidence in addition to the rows above.
