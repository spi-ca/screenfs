# Post-metadata follow-up benchmark summary

Baseline: detached `HEAD` (`3cb95ba`) build from `/tmp/screenfs-before-postmeta-3cb95ba`.
Candidate: current worktree build with directory `d_type` fast path and read-only close sync bypass. The same worktree also contains the directory bridge-visible matcher descendant boolean path, but this bundle only provides directory/read-only-close workload evidence; it does not independently claim a rule-rich matcher-descendant speedup.

All runs used `--iterations 10 --warmups 3` with paired before/after `screenfs` binaries built in release mode. Perf-counter rows used release binaries built with `--features perf-counters`. Current validation for the maintained code and benchmark harness is recorded in [`validation-current.log`](validation-current.log).

## Latency ratios (after / before)

| policy/workload set | workload | p50 | p95 | p99 |
| --- | --- | ---: | ---: | ---: |
| fast-path-cache-eligible directory-surface | readdir_basic | 0.759 | 0.762 | 0.762 |
| fast-path-cache-eligible directory-surface | readdirplus_basic | 0.947 | 0.928 | 0.926 |
| glob-matcher-heavy directory-surface | readdir_basic | 0.781 | 0.793 | 0.793 |
| glob-matcher-heavy directory-surface | readdirplus_basic | 0.938 | 0.950 | 0.951 |
| fast-path-cache-eligible read-only-close-surface | read_only_open_close | 0.939 | 0.941 | 0.941 |
| fast-path-cache-eligible read-only-close-surface | read_only_open_read_close | 0.899 | 0.897 | 0.898 |
| fast-path-cache-eligible read-only-close-surface | write_open_write_close | 1.016 | 1.026 | 1.035 |
| fast-path-cache-eligible read-only-close-surface | write_open_fsync_close | 0.994 | 1.044 | 1.051 |

## Perf counter highlights

| policy/workload set | counter | before | after |
| --- | --- | ---: | ---: |
| fast directory-surface | readdir_attr_generation_entries | 427820 | 0 |
| fast directory-surface | readdir_directory_scan total ns | 1269078441 | 950588360 |
| fast directory-surface | readdir_candidate_selection total ns | 71402093 | 54702972 |
| fast directory-surface | readdir_page_commit total ns | 141389037 | 137904893 |
| fast directory-surface | readdirplus_directory_scan total ns | 387971398 | 393762564 |
| fast directory-surface | readdirplus_attr_generation_scan total ns | 129102073 | 129425816 |
| fast directory-surface | readdirplus_attr_generation_entries | 139868 | 139868 |
| fast directory-surface | readdirplus_candidate_selection total ns | 22929821 | 24439725 |
| fast directory-surface | readdirplus_page_commit total ns | 5217738 | 4690108 |
| glob directory-surface | readdir_attr_generation_entries | 427820 | 0 |
| glob directory-surface | readdir_directory_scan total ns | 1475848827 | 1273988328 |
| glob directory-surface | readdir_candidate_selection total ns | 70778453 | 62908701 |
| glob directory-surface | readdir_page_commit total ns | 140169060 | 141306317 |
| glob directory-surface | readdirplus_directory_scan total ns | 457180468 | 494871638 |
| glob directory-surface | readdirplus_attr_generation_scan total ns | 126184156 | 144752207 |
| glob directory-surface | readdirplus_attr_generation_entries | 139868 | 139868 |
| glob directory-surface | readdirplus_candidate_selection total ns | 23417338 | 25111892 |
| glob directory-surface | readdirplus_page_commit total ns | 5064312 | 5362262 |
| read-only-close-surface | fuse_op.flush count | 109824 | 3328 |
| read-only-close-surface | file_sync.flush count | 109824 | 3328 |
| read-only-close-surface | fuse_op.release count | 109824 | 109824 |
| read-only-close-surface | file_sync.fsync count | 1664 | 1664 |

## Files

Claim-scope artifacts:

- `before-fast-path-cache-eligible-directory-surface.*` / `after-fast-path-cache-eligible-directory-surface.*`
- `before-glob-matcher-heavy-directory-surface.*` / `after-glob-matcher-heavy-directory-surface.*`
- `before-fast-path-cache-eligible-read-only-close-surface.*` / `after-fast-path-cache-eligible-read-only-close-surface.*`
- `validation-current.log`

Context-only artifacts retained in this directory but not part of the directory/read-only-close claim:

- `before-fast-path-cache-eligible-metadata-open-path.*` / `after-fast-path-cache-eligible-metadata-open-path.*`

## Notes

- `readdir_basic` is the primary directory win: no no-follow attr generation is needed when host `d_type` is known, with safe `DT_UNKNOWN` fallback in code.
- The `readdirplus_basic` rows in this bundle keep the full directory page/scan/page-commit counters above, but they do not by themselves prove a separate page/scan latency win. Use [`../../readdirplus-deferred-attrs/summary.md`](../../readdirplus-deferred-attrs/summary.md) for the narrower attr-work reduction slice, and keep its 20k regression caveat attached.
- The checked-in `glob-matcher-heavy` directory row is a directory-policy non-regression row, not an independent rule-rich matcher-descendant claim. Current rule-rich descendant evidence still needs a directory workload that actually exercises non-empty visible descendant logic.
- A cheaper heap-based candidate selection experiment was also rejected because it did not improve the measured directory-surface artifact. The checked-in code keeps the stable name-ordered `BTreeMap` page selection.
- Read-only close workloads retain `release` cleanup while removing read-only `flush` sync work. Write-capable `flush` and explicit `fsync` counters remain present.
