# Sync snapshot path-free candidate

This directory records a sync-surface performance attempt that removes unnecessary `VirtualPath` cloning from `flush()` and `fsync()` snapshots only. It does **not** repeat the rejected read/write fast snapshot-path candidate: read/write, `copy_file_range`, and path-sensitive mutation flows keep their existing path-bearing helpers.

## Result

Kept as a narrow **fast-policy sync-surface** speedup claim. Same-machine before/after evidence shows the targeted `fast-path-cache-eligible` `sync_flush_only` and `sync_fsync_only` rows both improved well past the usual `<= 0.90x` p50 and `<= 0.95x` p95/p99 slice bar. `sync_release_flush` improved only as mounted flush/release context: the run did not emit a non-zero `file_sync.release_flush` counter, so it is not release-helper claim evidence. The companion `fallback-unsafe-policy` row stayed within non-regression context but is not claimed as an independent speedup.

## Scope

- lane: `sync-surface`
- kept scope: `flush()` / `fsync()` snapshot helpers only
- claim scope: `fast-path-cache-eligible` `sync_flush_only` and `sync_fsync_only`
- context/non-regression: `sync_release_flush`, `fallback-unsafe-policy` `sync-surface`
- explicit non-goal: read/write and `copy_file_range` remain on their existing path-bearing helpers

## Code change

- `src/fs/state.rs`
  - added path-free `file_sync_snapshot()` returning only `Arc<File>` for `fsync()`
  - added path-free `file_flush_sync_snapshot()` returning `Arc<File>` plus `flush_needs_sync` for `flush()`
- `src/fs.rs`
  - `flush()` now uses `file_flush_sync_snapshot()`
  - `fsync()` now uses `file_sync_snapshot()`
  - `release()` remains unchanged
- `src/fs/tests/perf.rs`
  - added direct sync snapshot validation and negative `ENOENT` coverage
  - kept read/write and `copy_file_range` perf guardrails in validation plan

## Benchmark evidence

Artifact bundle: [`worktree-70a3105/`](worktree-70a3105/)

JSON source of truth:

- `worktree-70a3105/before-head-fast-path-cache-eligible-sync-surface.json`
- `worktree-70a3105/after-worktree-fast-path-cache-eligible-sync-surface.json`
- `worktree-70a3105/before-head-fallback-unsafe-policy-sync-surface.json`
- `worktree-70a3105/after-worktree-fallback-unsafe-policy-sync-surface.json`

All rows used `--perf-counters --workload-set sync-surface --cache-control warm --iterations 10 --warmups 3 --sync-bytes 4096 --sync-ops 128`.

After/before latency ratios:

| policy | workload | p50 | p90 | p95 | p99 | mean | stdev | interpretation |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| fast-path-cache-eligible | `sync_flush_only` | `0.739899x` | `0.705322x` | `0.671407x` | `0.646596x` | `0.725922x` | `0.320734x` | claimed improvement |
| fast-path-cache-eligible | `sync_fsync_only` | `0.833528x` | `0.765774x` | `0.769611x` | `0.772675x` | `0.804517x` | `0.506231x` | claimed improvement |
| fast-path-cache-eligible | `sync_release_flush` | `0.725385x` | `0.729039x` | `0.705202x` | `0.687285x` | `0.716642x` | `0.622507x` | flush/release context only; no `file_sync.release_flush` counter |
| fallback-unsafe-policy | `sync_flush_only` | `1.026981x` | `1.059238x` | `1.054225x` | `1.050250x` | `1.035384x` | `1.553902x` | non-regression context |
| fallback-unsafe-policy | `sync_fsync_only` | `0.987956x` | `0.973708x` | `0.975674x` | `0.977235x` | `0.976796x` | `0.870477x` | non-regression context |
| fallback-unsafe-policy | `sync_release_flush` | `1.033147x` | `1.011934x` | `1.018733x` | `1.024161x` | `1.025416x` | `0.949462x` | non-regression context; no release-helper claim |

The claimed fast-policy rows show non-overlapping improvement across p50/p90/p95/p99 and lower mean/stdev ratios. The fallback rows are near-neutral context: `sync_flush_only` shows a small tail regression but stays within the sync-slice non-regression threshold (`<= 1.05x` median and `<= 1.10x` p95/p99), and the other fallback rows remain neutral or slightly improved.

Selected fast-policy counters moved in the expected direction:

| counter | before avg | after avg | interpretation |
| --- | ---: | ---: | --- |
| `file_sync.flush` | `5127ns` | `2907ns` | lower sync workload body cost |
| `file_sync.fsync` | `3812ns` | `2551ns` | lower sync workload body cost |
| `state_read_lock_hold` | `77ns` | `66ns` | path-free snapshot reduces state-read hold work slightly |
| `fuse_op.flush` | `5440ns` | `3165ns` | end-to-end flush op improvement |
| `fuse_op.fsync` | `4093ns` | `2806ns` | end-to-end fsync op improvement |
| `fuse_op.release` | `629ns` | `525ns` | release context improved |

Fallback context stayed neutral overall: `sync_flush_only` p50/p95/p99 stayed within the documented non-regression gate, `sync_fsync_only` improved slightly, and `sync_release_flush` stayed within non-regression. Because `file_sync.release_flush` is absent/zero in these mounted artifacts, `sync_release_flush` is not used as release-helper speedup evidence.

## Safety constraints

The candidate preserves existing semantics:

- handle/inode validation still returns `ENOENT` on missing handle or inode mismatch
- `flush_needs_sync` still decides whether `flush()` skips backing sync work
- `release()` behavior is unchanged, including `flush=true` cleanup
- no visibility/policy/mutability guard semantics change
- no change to read/write, `copy_file_range`, or other path-sensitive request flows
- hidden `ENOENT` / readonly `EROFS` precedence and current FUSE contracts remain unchanged

## Local validation and review

Focused and full validation passed; see [`validation.log`](validation.log):

- `cargo fmt --check`
- `cargo check --features perf-counters`
- `cargo test --all-targets --all-features`
- `cargo clippy --all-targets --all-features`
- `cargo test --features perf-counters fs::tests::perf::perf_counters_record_file_sync_splits -- --exact`
- `cargo test --features perf-counters fs::tests::perf::sync_snapshots_reject_missing_or_mismatched_handles -- --exact`
- `cargo test --features perf-counters fs::tests::perf::readonly_open_uses_noflush_and_flush_skips_sync -- --exact`
- `cargo test --features perf-counters fs::tests::perf::perf_counters_record_data_path_splits_on_success -- --exact`
- `cargo test --features perf-counters fs::tests::perf::perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe -- --exact`
- `cargo test --features perf-counters fs::tests::perf::perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path -- --exact`
- `cargo test --features perf-counters fs::tests::data_mutations::flush_and_fsync_complete_under_compio_runtime -- --exact`
- `cargo test --features perf-counters fs::tests::data_mutations::release_flush_true_completes_under_compio_runtime_and_removes_handle -- --exact`

Subagent reviewer findings were resolved by adding negative `ENOENT` coverage and extending the validation plan to include read/write and `copy_file_range` guardrails. Follow-up reviewer reported no findings. Security review found no guardrail blocker.
