# All-five performance goal completion audit

Goal ID: `9016b393-41c3-42ad-8d73-260c0a0b9446`

This audit maps the active goal requirements to current evidence. Final review accepted the documented non-root read-side cold-cache approximation as coverage-only evidence and accepted that strict literal cold-cache is explicitly not claimed under the current non-root harness contract.

## Completed or evidence-backed parts

| goal requirement | current evidence | status |
| --- | --- | --- |
| Try all five remaining performance lanes | The five lanes are represented in `docs/artifacts/current-next-performance-candidates.md` and `docs/performance-roadmap.md`: mutation invalidation breadth, `open_confined`/`openat2`, `readdirplus` page/scan, matcher-heavy policy path, and read/write follow-up. | satisfied as documented lane coverage |
| Mutation invalidation breadth | `docs/artifacts/mutation-invalidation-breadth-index/summary.md` records fresh before/after set and focused subtree artifacts. The child-index attempt reduced `scanned_entries` (`15392 -> 572` set, `10764 -> 52` focused) but failed latency gates (`1.239x`–`1.439x` set p50; focused p50 `1.028x`), so Rust code was reverted/not kept. Post-`FORGET` remains limitation/state-test scope only, with no eviction-breadth claim. | satisfied as rejected/slice-context evidence |
| `open_confined`/`openat2` surrounding-cost separation | `src/fs.rs`, `src/fs/perf.rs`, `src/fs/guards.rs`, and `src/fs/tests/perf.rs` add behavior-preserving `open_like.pre_open_guard.*` and `open_like.post_open_revalidation.*` attribution. Regenerated `docs/artifacts/current-open-confined-surface-smoke.*` and `docs/artifacts/current-metadata-opendir-smoke.*` include these counters plus existing `open_confined_openat2`, `source_root_path`, `resolved_virtual_path_from_open_fd`, `fuse_op.open`, and `fuse_op.opendir`. Docs keep `open-confined-frequency/summary.md` as rejected/slice-context only. | satisfied as coverage/instrumentation improvement, not speedup claim |
| `readdirplus` 20k regression separation | `docs/artifacts/readdirplus-20k-policy-ablation/summary.md` and companion artifacts split 5k/20k and fallback/fast policy rows with `readdirplus_directory_scan`, `readdirplus_candidate_selection`, `readdirplus_page_commit`, p50/p95/p99. Docs mark it as dirty-worktree current-ablation coverage only, not a before/after or kept readdirplus optimization. | satisfied as coverage/current-ablation evidence |
| Matcher-heavy directory/descendant path | `scripts/bench-screenfs.py` and `scripts/test_bench_screenfs_workloads.py` add `matcher-descendant-directory` with `matcher_descendant_readdir` / `matcher_descendant_readdirplus` and `--matcher-extra-rules > 0` non-empty visible descendant fixture. `docs/artifacts/current-matcher-descendant-directory-smoke.*` confirms non-zero descendant/readdir/readdirplus counters. `docs/artifacts/matcher-descendant-combined-probe/summary.md` records a combined-probe Rust optimization attempt; it failed gate and was reverted/not kept. | satisfied as coverage + rejected experiment evidence |
| Read/write follow-up: fast-policy, larger-sequential, storage-backed, concurrency, and non-root cold-cache approximation coverage | `docs/artifacts/read-write-followup-current/summary.md` records fast/fallback current `read-write-surface` coverage, alternate same-64MiB mix rows, and actual `256MiB` larger-sequential rows. `docs/artifacts/read-write-storage-backed/summary.md` adds local `/home/...` btrfs storage-backed `read-write-surface` plus `read-write-concurrency` / `concurrent_rand_read_write_4k` coverage. `docs/artifacts/read-write-cold-cache-approx/summary.md` adds local btrfs `--cache-control posix-fadvise-read-fixture` coverage for fast/fallback surface + concurrency rows and documents method/applications/read-side-only limits. Docs explicitly scope all of these rows as coverage only, not before/after claims. | satisfied as coverage-only evidence |
| Required validation | `docs/artifacts/current-all-five-performance-validation.log` records exact command strings and successful validation results: Python compile/tests, `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`, JSON parse, required `/usr/bin/find` checks, and `git diff --check`. | satisfied |

## Completion scope notes

1. **No repo-local evidence blocker remains; strict literal cold-cache is an explicit scope boundary / non-claim.**
   - `docs/artifacts/read-write-storage-backed/summary.md` covers storage-backed and concurrency rows.
   - `docs/artifacts/read-write-cold-cache-approx/summary.md` covers the requested non-root read-side cold-cache approximation with documented `posix-fadvise-read-fixture` limits.
   - `docs/benchmarks.md`, `docs/performance-roadmap.md`, and `docs/artifacts/current-next-performance-candidates.md` now describe warm vs approximation semantics and explicitly say the approximation does not reset write-side, dentry, inode, or device caches.
   - What remains is a scope statement, not a missing artifact: strict literal cold-cache is still not claimed, and privileged `drop_caches` runs are still outside this harness contract.

2. **Review closure and validation evidence are repo-checkable.**
   - `docs/artifacts/current-all-five-performance-review-closure.md` records the final reviewer/researcher/security-reviewer closure state.
   - `docs/artifacts/current-all-five-performance-validation.log` includes exact command strings and successful results.

3. **Final review accepted this documented end-state.**
   - `docs/artifacts/current-all-five-performance-review-closure.md` records no blocking reviewer/researcher/security findings after the cold-cache approximation docs and artifacts were added.
   - Strict literal cold-cache remains an explicit non-claim, not an unverified hidden requirement of this goal.

## Kept changes so far

- Matcher-descendant benchmark coverage in `scripts/bench-screenfs.py` and `scripts/test_bench_screenfs_workloads.py`.
- Open-like perf attribution counters and tests in `src/fs.rs`, `src/fs/guards.rs`, `src/fs/perf.rs`, and `src/fs/tests/perf.rs`.
- Documentation and artifact updates under `docs/benchmarks.md`, `docs/performance-roadmap.md`, `docs/artifacts/**`.

## Reverted / not kept

- Matcher combined-probe Rust optimization: rejected by `docs/artifacts/matcher-descendant-combined-probe/summary.md` and reverted.
- Mutation child-index invalidation optimization: rejected by `docs/artifacts/mutation-invalidation-breadth-index/summary.md` and reverted.
- No kept readdirplus page/scan optimization was made in this goal.
- No kept read/write behavior optimization was made in this goal beyond the existing prior fallback claim; the new rows are coverage only.
- Strict literal cold-cache is intentionally not claimed under the current non-root harness contract.

## Completion decision

All explicit goal requirements are mapped to current evidence: every requested lane was tried, safe coverage/instrumentation changes were kept, failed optimization attempts were reverted and documented as rejected evidence, and read/write fast/larger-sequential/storage-backed/concurrency plus non-root cold-cache approximation coverage is present. Strict literal cold-cache is not claimed under the current non-root harness contract. No repo-local blocker remains for marking the goal complete.
