# readdirplus directory child visibility batch proof

## Scope

This bundle records the first conservative implementation proof for a request-local/directory-local `DirectoryChildVisibilityBatch` used by the `readdirplus` directory scan path. It is a proof/smoke artifact, not a claim-grade before/after performance bundle.

The original proof scope kept broad prefix classification out of the fast path and only skipped per-entry scan-time policy evaluation for the trivial safe shape: `visibility.default=visible`, no user hidden rules, and no internal hidden mount-root rule. The current implementation also has a later parent-local slice for `visibility.default=hidden`, no user hidden rules, no internal hidden mount-root rule, and visible descriptors that are all subtree anchors; that later slice is `readdirplus`-only and has no kept speed claim. Broader rule-sensitive shapes still fall back to the existing `entry_is_readable()` path and need matcher-descendant plus repeated/interleaved evidence before any speed claim.

The following guardrails remain unchanged:

- returned-entry `readdirplus` policy recheck;
- symlink target visibility checks;
- stable resume cookies/shared cookie domain;
- returned-page-only `readdirplus` lookup-ref pinning;
- hidden `ENOENT` semantics.

## Validation commands

Focused proof validation:

```bash
cargo test --features perf-counters directory_child_visibility_batch
cargo test --features perf-counters perf_counters_record_scan_visibility_batch_skips_trivial_policy_shape
```

Both commands passed.

## Smoke commands

All rows used non-root `scripts/bench-screenfs.py --build --perf-counters --iterations 3 --warmups 1`.

- `fast-directory-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set directory-surface`
- `fallback-matcher32-directory-surface.*`: `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`

## Observed smoke attribution

| row | workload | mounted p50 | mounted/native p50 | notable counters |
| --- | --- | ---: | ---: | --- |
| fast directory-surface | `readdir_basic` | 0.025628s | 19.193x | scan fast path active |
| fast directory-surface | `readdirplus_basic` | 0.198407s | 25.040x | `readdirplus_directory_scan=32469063ns`, `readdirplus_scan.scan_visibility=956758ns`, `returned_policy_recheck=4241504ns` |
| fallback matcher32 directory-surface | `readdir_basic` | 0.174720s | 91.913x | fallback path active |
| fallback matcher32 directory-surface | `readdirplus_basic` | 0.339186s | 33.912x | `readdirplus_directory_scan=238047997ns`, `readdirplus_scan.scan_visibility=198734888ns`, `returned_policy_recheck=9679781ns` |

The fast-policy row shows scan-time visibility cost dropping to a small residual counter bucket because the batch returns `AllVisible` without calling `entry_is_readable()` per child. The matcher32 row still records large `scan_visibility` cost, demonstrating fallback for rule-sensitive policy shapes.

## Scope boundary

This is not a full `readdirplus` page/scan claim. Claim-grade evidence still requires the documented before/after matrix in `docs/benchmarks.md`: fast `directory-surface`, matcher32 `directory-surface` where relevant, and focused large-directory `readdirplus_basic` rows with `--iterations 10 --warmups 3`.
