# Current all-five retry completion audit

Goal: `3bac07ab-4d99-4a66-93d2-25a446b28134`

This audit maps the `3bac07ab-4d99-4a66-93d2-25a446b28134` goal requirements to the evidence that existed when that goal was completed, before the later `sync-snapshot-path` follow-up work began. At that audit point, no rejected Rust implementation remained in the worktree and the all-five retry diff was documentation and benchmark artifacts only. Safe speedups were not kept for that goal because every fresh candidate in the retry failed its documented gate, so each implementation was reverted and recorded as rejected evidence.

## Requirement mapping

| requirement | evidence | status |
| --- | --- | --- |
| Document-first review and update before code/test changes | Parallel document review found read/write gate/checkpoint gaps; updates were made in `docs/benchmarks.md`, `docs/performance-roadmap.md`, and `docs/artifacts/current-next-performance-candidates.md`; follow-up reviewer/security-reviewer reported no blockers. | verified |
| `open_confined` / `openat2` retry with fallback/fast `open-confined-surface`, opendir/open counters, and rejected/claim evidence | `docs/artifacts/open-confined-fd-path/summary.md` records a fresh `/proc/self/fd/<fd>` path-builder micro-candidate, local validation, fallback/fast `open-confined-surface` before/after rows, `metadata_opendir` and `metadata_access` context, counters including `open_confined_openat2`, `open_like.*`, `source_root_path`, `resolved_virtual_path_from_open_fd`, `fuse_op.open`, `fuse_op.opendir`, and rejection. The Rust change was reverted. | verified rejected |
| `readdirplus` page/scan 20k retry with directory-size/focused before/after and page/scan counters | `docs/artifacts/readdirplus-bounded-candidate-heap/summary.md` records a bounded candidate-set retry, fast and matcher-heavy `directory-surface` rows, focused 20k `readdirplus_basic`, p50/p95/p99 ratios, and counters including `readdirplus_directory_scan`, `readdirplus_candidate_selection`, `readdirplus_page_commit`. Gate failed; Rust change reverted. | verified rejected |
| mutation invalidation breadth retry with `invalidations.scanned_entries` reduction plus latency gate and focused subtree evidence | `docs/artifacts/mutation-invalidation-prefix-range/summary.md` records the no-index ordered prefix-range retry, set row and focused `subtree_rename_cached_unrelated`, `invalidations.*`, `state_write_lock_hold`, post-`FORGET` limitation, and before/after ratios. Scan breadth dropped but full-set latency gate failed; Rust change reverted. | verified rejected |
| matcher-heavy policy path retry with matcher32 policy row, descendant/directory evidence, matcher counters, and rejected/claim evidence | `docs/artifacts/matcher-rank-zero-early-stop/summary.md` records the rank-0 early-stop retry, `policy-heavy-matrix`, `matcher-descendant-directory`, matcher counters (`policy_decision`, `matcher_candidate_order.*`, `matcher_family_candidates.*`, etc.), p50/p95/p99 ratios, and rejection. Rust change reverted. | verified rejected |
| read/write follow-up retry preserving fallback small-I/O claim and trying a separate fast/storage/concurrency/cold-cache-related claim or rejection | `docs/artifacts/read-write-fast-snapshot-path/summary.md` records the selected fast-policy small-I/O sub-axis, before/after `read-write-surface` rows, fallback context, p50/p95/p99 ratios, and rejection. Existing fallback small-I/O claim remains context; Rust change reverted. | verified rejected |
| Preserve ScreenFS behavior and guardrails | All rejected code for this all-five retry was reverted. Final reviewer/security-reviewer checks reported no blockers. The all-five validation at that point (`docs/artifacts/current-all-five-retry-validation.log`) passed `cargo test --all-targets --all-features` including hidden `ENOENT`, mutability, symlink, fd revalidation, state/cache, matcher, and perf tests. Later `sync-snapshot-path` code changes have their own validation log under `docs/artifacts/sync-snapshot-path/validation.log`. | verified |
| Required validation commands and artifact checks | `docs/artifacts/current-all-five-retry-validation.log` records the all-five retry's `python3 -m py_compile`, 11+32 Python tests, `cargo fmt --check`, `cargo check`, `cargo check --features perf-counters`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features` (188 lib + 1 main), JSON parse (`json-ok 28 files`), required `/usr/bin/find` checks, and `git diff --check`. Later sync follow-up validation is intentionally separate. | verified |
| Subagent review/validation with blockers cleared | Final parallel reviewer/security-reviewer/researcher reported no blockers for the all-five retry evidence and validation. Per-lane follow-up reviews also reported no blockers after fixes. | verified |

## Current lane labels after this retry

- `open_confined` / `openat2`: remains `smoke`; fresh fd-path candidate rejected.
- `readdirplus` page/scan: remains `slice`; fresh bounded candidate-set candidate rejected.
- mutation invalidation breadth: remains `smoke`; fresh prefix-range candidate rejected.
- matcher-heavy policy path: remains `smoke`; fresh rank-0 early-stop candidate rejected.
- read/write follow-up: remains `smoke` beyond the prior fallback small-I/O claim; fresh fast snapshot-path candidate rejected.

## Completion conclusion

Every explicit lane in the completed all-five goal was retried after `70a3105` with before/after evidence or focused context, no unsafe or gate-failing implementation was kept for that goal, rejected attempts are documented with ratios/counters/provenance, all-five final validation passed, and final subagent reviews reported no blockers. The all-five goal requirements were fully verified for completion at that audit point; later sync follow-up work is tracked in `docs/artifacts/sync-snapshot-path/summary.md` and `validation.log`.
