# Writeback-cache / large-I/O preflight

This preflight artifact records the current planning state for the opt-in writeback-cache and large-I/O experiment tracks. It is **not** claim-grade performance evidence and does not mean `FUSE_WRITEBACK_CACHE` is part of the default ScreenFS contract. Some experimental code paths already exist; this artifact remains the pre-support/pre-claim provenance gate.

## Scope

Tracks covered before claim-grade support:

1. `O_WRONLY` writeback handle semantics
2. `FileHandle` user access mode separation
3. kernel page-cache invalidation unsupported scope
4. `max_write` / `max_readahead` experiment gate
5. `fractal-fuse` `max_pages` / page-size caveat
6. sync/flush/release surface gate

## Current baseline

- Default mount behavior keeps `write_back == false`; `--experimental-writeback-cache` is the only repo-local opt-in smoke/benchmark setter.
- ScreenFS has no repo-local `init()` override for `ReplyInit` tuning.
- ScreenFS has no checked-in `FuseNotifier` or kernel page-cache invalidation path.
- `FileHandle` stores requested user-visible access mode and experimental `writeback_internal_reads` separately from the backing `File`.
- `read`, `write`, `fallocate`, and `copy_file_range` apply policy/opened-target guards before requested-access `EBADF` checks.
- In opt-in `--experimental-writeback-cache` mode, write-only backing opens are widened for kernel read-before-write style internal reads only when the backing file also permits read access. If widening fails with `EACCES`/`EPERM`, ScreenFS falls back to the original write-only open and internal reads remain `EBADF`; default mode always keeps `O_WRONLY` read as `EBADF`. Because handler-level `read` requests are not currently distinguished by origin, the experimental mode is not a security/isolation boundary for workloads that depend on `O_WRONLY` preventing reads.

## Current evidence pointers

- `src/fs/tests/data_mutations.rs`: `write_only_open_allows_partial_overwrite_but_read_returns_ebadf`
- `src/fs/tests/data_mutations.rs`: `experimental_writeback_allows_internal_read_on_write_only_handle`
- `src/fs/tests/data_mutations.rs`: `experimental_writeback_falls_back_for_write_only_non_readable_file`
- `src/fs/tests/data_mutations.rs`: `experimental_writeback_preserves_truncate_and_append_flags`
- `src/fs/tests/data_mutations.rs`: `experimental_writeback_create_falls_back_for_existing_write_only_file`
- `src/fs/tests/data_mutations.rs`: `handle_mutators_use_requested_access_after_policy_precedence`
- `src/fs/tests/perf/sync.rs`: `readonly_open_uses_noflush_and_flush_skips_sync`, `write_capable_open_flushes_and_does_not_use_noflush`
- `src/fs/tests/perf/data_path.rs`: `perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures`

## Required experiment provenance

Any future run that enables writeback-cache or changes negotiated payload size must record:

- exact ScreenFS command line and experimental flags
- `--policy-label` or artifact label containing `writeback-exp` or `large-io-exp`
- same-machine baseline without the experimental flag
- targeted workload row (`read-write-surface`, larger sequential/storage-backed row, or both)
- companion `sync-surface` row
- companion `per-open-cache-minimum` context row
- observed/effective FUSE INIT payload values when relevant, including `max_write`, `max_readahead`, `max_pages`, page-size assumptions, or an explicit note that these values stayed at the baseline dependency/default behavior. Source these from the startup log emitted by pinned `fractal-fuse` during `FUSE_INIT` when available; otherwise record source-inspected dependency values from `fractal-fuse-0.4.0/src/session.rs` / `src/types.rs` and local page size via `getconf PAGESIZE`, and keep the row at planning/smoke grade.
- whether the row is smoke, planning, slice, or claim-grade evidence

## Mounted smoke/benchmark evidence

- [`mounted-bench-20260703/summary.md`](mounted-bench-20260703/summary.md) records a same-machine mounted baseline vs `--experimental-writeback-cache` run for `read-write-surface`, `sync-surface`, and `per-open-cache-minimum` using `--iterations 10 --warmups 3` on tmpfs with `fast-path-cache-eligible` policy.
- [`mounted-bench-rerun-20260703/summary.md`](mounted-bench-rerun-20260703/summary.md) repeats the same matrix. The rerun again shows `seq_write` and all `sync-surface` rows regressing, while `per-open-cache-minimum` improves in that run.
- Result interpretation remains preflight/smoke-grade rather than broad speedup claim-grade: read/write rows are mixed between runs, `seq_write` and sync rows regress consistently, while `per-open-cache-minimum` random read/write rows improve.

## Status

Experimental-only pre-support/pre-claim artifact. Mounted plumbing now has local preflight benchmark evidence, but no broad writeback-cache performance improvement claim is recorded here.
