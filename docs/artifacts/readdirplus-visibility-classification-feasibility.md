# readdirplus visibility classification feasibility

## Scope

This note evaluates two ideas discussed for reducing `readdirplus_scan.scan_visibility` cost:

1. directory-local prefix classification;
2. a directory child visibility batch API.

It is analysis/evidence only. It does not claim a performance win.

## Current evidence

The current page-scan attribution smoke shows scan-time visibility dominates the visible `readdirplus_directory_scan` sub-buckets in sampled rows:

- fast 5k directory-surface: `readdirplus_scan.scan_visibility=91984803ns` of `readdirplus_directory_scan=127695685ns`
- fast 20k focused: `readdirplus_scan.scan_visibility=528955527ns` of `readdirplus_directory_scan=736738341ns`
- fallback matcher32 directory-surface: `readdirplus_scan.scan_visibility=214514591ns` of `readdirplus_directory_scan=259412653ns`

Source: [`current-readdirplus-page-scan-attribution-smoke/summary.md`](current-readdirplus-page-scan-attribution-smoke/summary.md).

The pre-proof hot call was per-entry scan-time visibility. Current code now routes `readdirplus` scan visibility through `directory_child_visibility_batch()`:

- `src/fs.rs` `collect_child_directory_page()` builds a directory-local batch only when `with_plus` is true; plain `readdir` continues to use the per-entry path.
- For the trivial `visibility.default=visible` / no user hidden rule / no internal hidden rule shape, the batch can assume all direct children are visible and skip the per-entry scan-time policy call.
- For `visibility.default=hidden`, no user hidden rule, no internal hidden rule, and visible descriptors that are all subtree anchors, the batch computes parent-local direct visible and bridge-visible child frontiers for the current `readdirplus` request.
- For rule-sensitive shapes, `src/fs.rs` still falls back to `ScreenFs::entry_is_readable(&entry.child, entry.is_dir)` so perf/matcher counters and existing visibility semantics remain intact.
- `src/config.rs` `entry_is_readable()` delegates to `visibility_decision()` and maps `Visible => true`, `BridgeVisible => is_directory`, `Hidden => false`.
- `visibility_decision()` checks internal hidden rules, visible rules, then `visible_matcher.may_match_descendant_of(path)` for bridge-visible ancestors.

## Directory-local prefix classification

A broad classification such as `AllVisible`, `AllHidden`, or `DirectoryOnly` is only safe when it can prove every direct child has the same result without looking at the child name.

That proof is rarely available from prefix alone because current visibility semantics depend on:

- visible carve-outs under a hidden/default-hidden parent, where ancestors are `BridgeVisible` and only the branch toward the carve-out is readable;
- subtree/direct-child glob/recursive literal rules;
- hidden vs visible overlap/most-specific precedence;
- the `BridgeVisible => is_directory` rule, which depends on the actual child type;
- symlink target visibility, which must remain a separate point-of-use check.

Existing tests demonstrating the risky cases include:

- `config_tests::visibility_hidden_and_visible_use_most_specific_match_with_bridge`
- `config_tests::default_hidden_supports_visible_carve_outs`
- `config_tests::direct_child_visible_rule_bridges_only_normalized_anchor_ancestors` for direct-child glob bridge scope
- `config_tests::canonical_glob_families_share_visibility_and_mutability_axis_semantics` for direct-child vs recursive glob family behavior
- `config_tests::config_file_accepts_recursive_literal_directory_shorthand_for_hidden_readonly_and_writable` for recursive literal directory shorthand behavior
- `visibility::readdir_page_boundaries_keep_bridge_visible_filtering`
- readdirplus lookup-ref/cookie tests in `src/fs/tests/state_cache.rs`

Conclusion: directory-local prefix classification is safe only for very narrow rule shapes, for example no relevant hidden/visible rules below the parent. It is not enough as the general next optimization because the high-cost rows include policy-sensitive matcher cases.

## Batch API direction

The safer implementation direction is a request-local/directory-local batch API that preserves per-entry semantics while reducing repeated matcher setup/candidate discovery where equivalence is proven. The first proof implemented only the narrow all-visible shape. The current implementation additionally keeps a `readdirplus`-only parent-scoped visible-subtree mode for default-hidden/no-hidden/no-internal-hidden policy shapes and falls back for broader rule-sensitive cases.

Current/recommended shape:

```rust
struct DirectoryChildVisibilityBatch<'a> {
    cfg: &'a RuntimeConfig,
    mode: DirectoryChildVisibilityMode,
}

enum DirectoryChildVisibilityMode {
    AllVisible,
    ParentScopedVisibleSubtrees { /* request-local frontier */ },
    PerEntry,
}

impl DirectoryChildVisibilityBatch<'_> {
    fn can_assume_all_visible(&self) -> bool {
        matches!(
            self.mode,
            DirectoryChildVisibilityMode::AllVisible
                | DirectoryChildVisibilityMode::ParentScopedVisibleSubtrees {
                    fully_visible: true,
                    ..
                }
        )
    }
}

// collect_child_directory_page(): batch is built only when with_plus/readdirplus is true.
let readable = if let Some(child_visibility) = child_visibility.as_ref() {
    if child_visibility.can_assume_all_visible() {
        true
    } else if let Some(readable) =
        child_visibility.parent_scoped_entry_is_readable(&entry.child, entry.is_dir)
    {
        readable
    } else {
        self.entry_is_readable(&entry.child, entry.is_dir)
    }
} else {
    self.entry_is_readable(&entry.child, entry.is_dir)
};
```

Implementation must remain conservative:

- keep returned-entry policy recheck unchanged;
- keep symlink target visibility checks unchanged;
- keep candidate-order debug/metrics semantics unchanged unless separately tested;
- use fallback for direct-child glob, recursive glob/literal, hidden/visible overlaps, and bridge-visible carve-out ambiguity until an equivalence test covers the case;
- do not persist the batch beyond the current directory request.

### Parent-local DP expansion guardrails

The current parent-local DP / memoized `DirectoryChildVisibilityBatch` mode is a request-local helper for the current `readdirplus` directory scan. It covers only `visibility.default=hidden`, no hidden/internal-hidden rules, and visible rules whose descriptors are all subtree anchors; plain `readdir` and uncertain policy shapes still use the per-entry path. It must not become a cross-request visibility, path, or symlink cache, and it must not replace point-of-use symlink target checks or the returned-entry `readdirplus` policy recheck.

Before any broader rule-sensitive mode is kept, it must prove equivalence against `entry_is_readable(child_path, is_directory)` for the exact child entries it classifies. Unknown or unproven shapes must fall back to `PerEntry`. In particular, fallback remains required until tests cover hidden subtree plus visible carve-out interactions, direct-child glob anchors, recursive literal/glob rules, broader `BridgeVisible => is_directory` boundaries, and hidden/visible overlap precedence.

## Verification plan for future expansions

Before claiming a speedup, add tests that compare batch/classified results against existing `entry_is_readable()` for:

- default visible with no child-affecting hidden rules;
- default hidden with a visible carve-out under the scanned parent;
- hidden subtree plus nested visible carve-out;
- direct-child glob and recursive literal rules, initially requiring fallback;
- bridge-visible directory page boundaries;
- symlink entries whose targets are hidden or bridge-visible.

Run at minimum:

```bash
cargo fmt --check
cargo test --features perf-counters perf_counters_record_policy_state_open_and_readdirplus_attr_work
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features
```

Performance evidence must still use the `readdirplus` gate in `docs/benchmarks.md`: fast `directory-surface`, matcher32 `directory-surface` where relevant, and focused large-directory `readdirplus_basic` before/after rows. If a batch/classification change touches descendant or bridge-visible matching, also add `--workload-set matcher-descendant-directory --matcher-extra-rules 32` (or equivalent non-empty visible descendant evidence) so the bridge-visible path is exercised directly.

## Conclusion

- A scan visibility fast path was not previously applied to `collect_child_directory_page()`; only symlink-target and other separate fast paths existed.
- Directory-local prefix classification is too easy to overgeneralize; use it only for proven rule shapes with explicit equivalence coverage.
- A conservative directory child visibility batch API is the better implementation direction, but it should remain an equivalence-tested wrapper with fallback rather than a broad prefix classifier.
- [`readdirplus-visibility-batch-proof/summary.md`](readdirplus-visibility-batch-proof/summary.md) records the first conservative proof: the trivial `visibility.default=visible` / no hidden rule / no internal hidden rule shape uses an all-visible batch fast path. The current follow-up implementation also adds a `readdirplus`-only default-hidden/no-hidden/no-internal-hidden visible-subtree-anchor parent-local slice; broader rule-sensitive shapes still fall back to per-entry visibility and broader speed claims require matcher-descendant plus repeated/interleaved evidence.
