# Mutation invalidation prefix-range attempt

This directory records a post-`70a3105` mutation invalidation breadth retry. The candidate replaced the full `path_inodes` subtree scan with an ordered `BTreeMap` lower-bound scan that starts at the root path and stops at the first key that no longer `starts_with(root)`. It did not add a mutation-maintained child index.

## Result

Rejected / not kept. The candidate reduced `invalidations.scanned_entries` substantially and improved the focused `subtree_rename_cached_unrelated` tails, but it failed the documented mutation-invalidation latency gate because all three workloads in the full `mutation-invalidation` set regressed. The focused row is supporting context: it improved tails but did not show a clear p50 win. The Rust implementation was reverted; this artifact remains rejected evidence only.

## Lane and rows

- candidate: `mutation invalidation breadth-first`
- previous label: `smoke`
- attempt shape: ordered `BTreeMap` lower-bound subtree scan for `path_inodes`, with no mutation-maintained child index

Rows collected with clean `HEAD` before and dirty candidate after, all using `--perf-counters --iterations 10 --warmups 3`:

- `--policy-preset fallback-unsafe-policy --workload-set mutation-invalidation --symlink-parent-mutations 20 --small-files 200`
- `--policy-preset fallback-unsafe-policy --workload subtree_rename_cached_unrelated --small-files 200`

## Benchmark evidence

JSON source of truth:

- `before-70a3105-mutation-invalidation.json`
- `after-worktree-70a3105-mutation-invalidation.json`
- `before-70a3105-subtree-rename-cached-unrelated.json`
- `after-worktree-70a3105-subtree-rename-cached-unrelated.json`

After/before latency ratios:

| row | workload | p50 | p95 | p99 | interpretation |
| --- | --- | ---: | ---: | ---: | --- |
| `mutation-invalidation` | `symlink_parent_mkdir_rmdir` | `1.170160x` | `1.305230x` | `1.324660x` | fails / regresses |
| `mutation-invalidation` | `pinned_symlink_parent_mkdir_rmdir` | `1.343853x` | `1.322512x` | `1.304703x` | fails / regresses |
| `mutation-invalidation` | `subtree_rename_cached_unrelated` | `1.222235x` | `1.203800x` | `1.199834x` | fails / regresses |
| focused | `subtree_rename_cached_unrelated` | `0.960478x` | `0.898640x` | `0.908639x` | supporting context: tails improve, p50 remains near-neutral |

Counter movement:

| artifact | `invalidations.scanned_entries` | `state_write_lock_hold.avg_ns` | interpretation |
| --- | ---: | ---: | --- |
| set before | `15392` | `376ns` | baseline |
| set after | `884` | `425ns` | scan breadth drops, write-lock avg regresses |
| focused before | `10764` | `502ns` | baseline |
| focused after | `104` | `388ns` | supporting context: scan breadth and write-lock avg improve, while p50 remains near-neutral |

The set row also showed pinned listing-path context counters (`readdirplus_directory_scan`, `readdirplus_page_commit`) remained present, but latency moved in the wrong direction. Therefore this is rejected evidence, not a kept breadth speedup.

## Safety constraints checked before rejection

The attempted implementation preserved these intended constraints while it was under test:

- exact invalidated path set for subtree invalidation
- sibling and same-prefix non-descendant mappings
- post-rename / post-unlink detach semantics and post-`FORGET` eviction behavior
- state write-lock consistency and no mutation-maintained child indexes
- invalidation-breadth scope only; no eviction-breadth claim

## Post-FORGET observability note

Mounted perf runs in this artifact observe invalidation breadth more directly than post-`FORGET` eviction breadth. No dedicated mounted post-`FORGET` driver was added, so `evicted_entries` remains context/limitation only and no eviction-breadth claim is made.

## Local validation and review

Local validation passed before benchmark rejection:

- `cargo fmt --check`
- `cargo check --features perf-counters`
- `cargo test fs::tests::state_cache::rename_invalidation_detaches_subtree_and_keeps_unrelated_sibling_mapping -- --exact --nocapture`
- `cargo test fs::tests::state_cache::mutation_invalidation_detaches_path_and_evicts_after_forget_without_touching_sibling -- --exact --nocapture`
- `cargo test fs::tests::state_cache::rename_overwrite_invalidates_source_and_target_but_not_sibling -- --exact --nocapture`
- `cargo test --features perf-counters fs::tests::state_cache::rename_invalidation_detaches_subtree_and_keeps_unrelated_sibling_mapping -- --exact --nocapture`
- `cargo test --features perf-counters perf_counters_record_ordered_subtree_invalidation_scan_count -- --nocapture`

Initial review requested an exact `invalidations.scanned_entries` regression test; `perf_counters_record_ordered_subtree_invalidation_scan_count` fixed that gap and follow-up review reported no findings. Security review found no guardrail blocker. Because the benchmark gate failed, the mutation invalidation lane remains `smoke` with this prefix-range attempt recorded as rejected evidence.
