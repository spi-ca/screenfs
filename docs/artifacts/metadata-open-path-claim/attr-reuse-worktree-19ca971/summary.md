# Attr reuse slice metadata/open-path benchmark evidence

This supplemental bundle records same-machine before/after evidence for the lookup/getattr same-path no-follow attr reuse slice. It is slice-scoped evidence, not a full metadata/open-path candidate claim. The before binary is detached `HEAD` and the after binary is the current dirty worktree; both use the same harness invocation, machine, policy, workload sizing, and warm-cache assumptions.

## Files

- `after-worktree-fast-path-metadata-lookup-getattr-open-access.json`
- `after-worktree-fast-path-metadata-lookup-getattr-open-access.md`
- `after-worktree-fast-path-metadata-lookup-getattr-open-access.svg`
- `before-head-fast-path-metadata-lookup-getattr-open-access.json`
- `before-head-fast-path-metadata-lookup-getattr-open-access.md`
- `before-head-fast-path-metadata-lookup-getattr-open-access.svg`

## Workload medians

| workload | before mounted p50 s | after mounted p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `metadata_lookup` | 0.095657 | 0.098624 | 1.031x | 0.100947 | 0.100217 | 0.993x | 0.101257 | 0.101139 | 0.999x |
| `metadata_getattr` | 0.124985 | 0.107563 | 0.861x | 0.130707 | 0.110236 | 0.843x | 0.131746 | 0.111759 | 0.848x |
| `metadata_open` | 0.127302 | 0.116219 | 0.913x | 0.131488 | 0.121142 | 0.921x | 0.132022 | 0.121505 | 0.920x |
| `metadata_access` | 0.124114 | 0.116529 | 0.939x | 0.130635 | 0.117902 | 0.903x | 0.131503 | 0.118047 | 0.898x |

## Selected perf counters

| counter | before count | before avg ns | after count | after avg ns |
| --- | ---: | ---: | ---: | ---: |
| `fuse_op.lookup` | 159749 | 20244 | 159749 | 16469 |
| `fuse_op.getattr` | 13313 | 19806 | 13313 | 17255 |
| `fuse_op.open` | 13312 | 20189 | 13312 | 26008 |
| `fuse_op.access` | 13313 | 20241 | 13313 | 26573 |
| `stat_child_no_follow` | 0 | 0 | 213001 | 13411 |
| `open_confined_openat2` | 399374 | 1146 | 239626 | 1306 |
| `source_root_path` | 399373 | 3352 | 239625 | 3490 |
| `resolved_virtual_path_from_open_fd` | 399373 | 1811 | 239625 | 1869 |
| `resolved_virtual_path_from_path` | 0 | 0 | 213000 | 3737 |

## Caveats

- This compares detached `HEAD` to a dirty worktree that includes this slice and earlier uncommitted changes; use it as current worktree evidence, not as a final release claim.
- The direct slice win is clearest on `metadata_getattr`; `metadata_lookup` p50 is neutral/noisy while its whole-run `fuse_op.lookup` average and open-confined call count improve.
- Before artifacts do not contain `stat_child_no_follow` because that counter is added by the after worktree.
