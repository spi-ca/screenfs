# Mutation invalidation range-scan before/after

This bundle records an ordered-subtree scan experiment for `invalidate_path_tree()`. The experiment compared detached clean `HEAD` (`41cb2d2`) against a dirty worktree that scanned the ordered `path_inodes` map from the subtree root and stopped at the first non-descendant.

The experiment is **not kept as the current implementation** because mounted evidence did not show a latency win. Treat this bundle as rejected-experiment evidence, not as a performance claim.

## Files

Original mutation-invalidation set run:

- `before-41cb2d2-mutation-invalidation.json`
- `before-41cb2d2-mutation-invalidation.md`
- `before-41cb2d2-mutation-invalidation.svg`
- `after-worktree-41cb2d2-mutation-invalidation.json`
- `after-worktree-41cb2d2-mutation-invalidation.md`
- `after-worktree-41cb2d2-mutation-invalidation.svg`

Subtree-heavy cached-unrelated run:

- `before-41cb2d2-subtree-rename-cached-unrelated.json`
- `before-41cb2d2-subtree-rename-cached-unrelated.md`
- `before-41cb2d2-subtree-rename-cached-unrelated.svg`
- `after-worktree-41cb2d2-subtree-rename-cached-unrelated.json`
- `after-worktree-41cb2d2-subtree-rename-cached-unrelated.md`
- `after-worktree-41cb2d2-subtree-rename-cached-unrelated.svg`

## Original mounted workload ratios

Command shape: `--workload-set mutation-invalidation --iterations 10 --warmups 3 --symlink-parent-mutations 200`.

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `symlink_parent_mkdir_rmdir` | 0.141383 | 0.145604 | 1.030x | 0.144126 | 0.192482 | 1.336x | 0.144522 | 0.200001 | 1.384x |
| `pinned_symlink_parent_mkdir_rmdir` | 0.207482 | 0.230316 | 1.110x | 0.213492 | 0.296427 | 1.389x | 0.215659 | 0.298073 | 1.382x |

Selected counters:

| counter | before | after |
| --- | ---: | ---: |
| `invalidations.count` | 10426 | 10426 |
| `invalidations.invalidated_entries` | 5213 | 5213 |
| `invalidations.evicted_entries` | 0 | 0 |
| `state_write_lock_hold.total_ns` | 50821648 | 62040493 |
| `fuse_op.mkdir.total_ns` | 443612477 | 514259749 |
| `fuse_op.rmdir.total_ns` | 349728458 | 396159247 |
| `readdirplus_directory_scan.total_ns` | 19030749 | 24543325 |

## Subtree-heavy cached-unrelated run

Command shape: `--workload subtree_rename_cached_unrelated --iterations 5 --warmups 1 --small-files 500`.

Historical caveat: this rejected-experiment artifact was captured before `subtree_rename_cached_unrelated` moved its restore rename into post-sample cleanup, so the recorded sample includes a rename out and a restore rename. Use current `current-mutation-invalidation-set-smoke.*` artifacts for the fixed workload shape.

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `subtree_rename_cached_unrelated` | 0.049884 | 0.060480 | 1.212x | 0.050548 | 0.064573 | 1.277x | 0.050559 | 0.065381 | 1.293x |

Selected counters:

| counter | before | after |
| --- | ---: | ---: |
| `invalidations.count` | 12 | 12 |
| `invalidations.invalidated_entries` | 24 | 24 |
| `invalidations.evicted_entries` | 0 | 0 |
| `invalidations.scanned_entries` | n/a | 48 |
| `state_write_lock_hold.total_ns` | 6974795 | 7119838 |

## Interpretation

Neither mounted run showed a latency win for the ordered-subtree scan experiment; the subtree-heavy cached-unrelated run was also slower after the change. Do not use this bundle as speedup evidence. The current code keeps the scanned-entry counter and mutation-invalidation workloads so future optimization attempts can first prove a reduction in `invalidations.scanned_entries` and then check latency separately.
