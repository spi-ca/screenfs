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

The pre-proof hot call was per-entry scan-time visibility. Current code now routes scan visibility through `directory_child_visibility_batch()`:

- `src/fs.rs` `collect_child_directory_page()` builds a directory-local batch before scanning children.
- For the trivial `visibility.default=visible` / no user hidden rule / no internal hidden rule shape, the batch can assume all direct children are visible and skip the per-entry scan-time policy call.
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

The safer implementation direction is a request-local/directory-local batch API that preserves per-entry semantics while reducing repeated matcher setup/candidate discovery where equivalence is proven. The current proof implements only the narrow all-visible shape and keeps fallback for rule-sensitive cases.

Current/recommended shape:

```rust
struct DirectoryChildVisibilityBatch<'a> {
    cfg: &'a RuntimeConfig,
    mode: DirectoryChildVisibilityMode,
}

impl DirectoryChildVisibilityBatch<'_> {
    fn can_assume_all_visible(&self) -> bool {
        matches!(self.mode, DirectoryChildVisibilityMode::AllVisible)
    }
}

// collect_child_directory_page():
let readable = if child_visibility.can_assume_all_visible() {
    true
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
- Directory-local prefix classification is too easy to overgeneralize; use it only for proven trivial rule shapes.
- A conservative directory child visibility batch API is the better implementation direction, but it should remain an equivalence-tested wrapper with fallback rather than a broad prefix classifier.
- [`readdirplus-visibility-batch-proof/summary.md`](readdirplus-visibility-batch-proof/summary.md) records the first conservative proof: only the trivial `visibility.default=visible` / no hidden rule / no internal hidden rule shape uses an all-visible batch fast path, while rule-sensitive shapes still fall back to per-entry visibility.
