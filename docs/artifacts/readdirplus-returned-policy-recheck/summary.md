# readdirplus returned-entry policy recheck experiment

## Scope

This bundle records a rejected `readdirplus` page/scan experiment. The attempted slice skipped the returned-entry `entry_is_readable()` recheck during `readdirplus` hydration when the scan-time entry type and final `stat_child_no_follow()` type had the same directory/non-directory readability class and the final type was not a symlink. Symlink final entries and type changes still rechecked policy and symlink target visibility.

The Rust change was reverted after the benchmark gate failed. The checked-in value of this bundle is evidence preservation only.

## Commands

All benchmark rows used non-root `scripts/bench-screenfs.py --build --perf-counters --iterations 10 --warmups 3` and wrote JSON/Markdown/SVG artifacts in this directory. The `before-*` rows were run from a temporary worktree at `HEAD` plus the already-kept direct opened-object `stat_child_no_follow()` change in `src/fs/backing.rs`; the `after-*` rows were run from the current worktree with the experimental readdirplus policy-recheck skip.

- `before-fast-directory-surface.*` / `after-fast-directory-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set directory-surface`
- `before-fallback-matcher32-directory-surface.*` / `after-fallback-matcher32-directory-surface.*`: `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`
- `before-fast-readdirplus-20k.*` / `after-fast-readdirplus-20k.*`: `--policy-preset fast-path-cache-eligible --dir-entries 20000 --workload readdirplus_basic`

## Result summary

Mounted latency after/before ratios, lower is better:

| row | workload | p50 | p95 | p99 | gate read |
| --- | --- | ---: | ---: | ---: | --- |
| fast `directory-surface` | `readdir_basic` | 1.121x | 1.112x | 1.127x | fail: paired `readdir` regression |
| fast `directory-surface` | `readdirplus_basic` | 0.951x | 0.994x | 0.994x | fail: misses `<=0.90x` / `<=0.95x` |
| fallback matcher32 `directory-surface` | `readdir_basic` | 0.598x | 0.605x | 0.608x | pass/context |
| fallback matcher32 `directory-surface` | `readdirplus_basic` | 1.242x | 1.007x | 0.998x | fail: p50 regression |
| fast focused 20k | `readdirplus_basic` | 1.010x | 0.962x | 0.962x | fail: p50 regression and p95/p99 miss `<=0.95x` |

The documented `readdirplus` gate requires every claimed `readdirplus_basic` size/policy row to show after/before `<=0.90x` median and `<=0.95x` at p95/p99, while paired `readdir_basic` rows must stay within `<=1.05x` median and `<=1.10x` at p95/p99. This experiment fails both the primary `readdirplus` gate and the paired fast `readdir_basic` non-regression gate.

## Revert / not-kept status

The experimental Rust changes in `src/fs.rs` and the focused perf test added to `src/fs/tests/perf.rs` were removed after the gate failed. Current source keeps the original returned-entry readability recheck behavior for `readdirplus` hydration.

## Validation and review evidence

During the experiment, focused verification passed before benchmarking:

- `git diff --check`: pass
- `cargo fmt --check`: pass
- `cargo check --features perf-counters`: pass
- `cargo test --features perf-counters perf_counters_record_readdirplus_regular_files_skip_returned_policy_rechecks`: pass while the experiment was present
- `cargo test --features perf-counters perf_counters_record_policy_state_open_and_readdirplus_attr_work`: pass
- `cargo test --features perf-counters readdirplus`: pass
- implementation review: no blocking findings; noted no dedicated host type-flip race test
- security review: no exploitable issue found for the narrow skip

After rejection, the code was reverted rather than kept.

## Scope boundaries

- This is rejected evidence only; do not read it as a kept readdirplus page/scan optimization.
- The benchmark results do not weaken hidden `ENOENT`, symlink target visibility, stable resume cookies/shared cookie domain, or returned-page-only `readdirplus` lookup-ref pinning.
- The next `readdirplus` attempt still needs a fresh same-matrix before/after pair plus focused large-directory `readdirplus_basic` evidence.
