# Current file-data-path async/io_uring feasibility evidence

This artifact records current feasibility evidence for selective host-side async/io_uring work on already-open file handle data operations. It is a planning and gating artifact, not proof that ScreenFS currently uses io_uring for backing file data I/O.

## Candidate scope

Only the following already-open file handle operations are candidates for a selective follow-up:

- `read`
- `write`
- `copy_file_range`
- `fallocate` only if benchmark/dependency evidence supports it
- `fsync`/`fdatasync` only if benchmark/dependency evidence supports it

The candidate scope excludes lookup/getattr/readdir/readdirplus/readlink/xattr/setattr/rename/link/symlink/unlink/rmdir/mkdir/mknod, policy evaluation, path resolution, source-root confinement, symlink target visibility, directory filtering, recursive discovery, and mount transport negotiation.

## Current implementation baseline

Current ScreenFS data operations in `src/fs.rs` use synchronous host file descriptors after policy guards:

- `read`: `file_handle_snapshot` -> `guard_read_path` -> offset-based `FileExt::read_at`
- `write`: `file_handle_snapshot` -> `guard_mutation_path` -> offset-based `FileExt::write_at`
- `copy_file_range`: input/output handle validation -> input visibility guard -> output path and parent mutability guard -> `libc::copy_file_range`
- `fallocate`: `file_handle_snapshot` -> `guard_mutation_path` -> `libc::fallocate`
- `fsync`: `file_handle_snapshot` -> `libc::fdatasync` or `libc::fsync`

Current handle state stores `Arc<std::fs::File>` and snapshots the shared file handle for each data operation. That model preserves short state-lock windows and keeps host I/O outside the `State` lock. Offset-based read/write avoid racing on the shared open file description offset.

## Dependency/API evidence

Current direct dependencies in `Cargo.toml` do not expose a host-side io_uring API for ScreenFS. `compio-runtime`, `compio-driver`, and `io-uring` are present only as transitive dependencies of `fractal-fuse = 0.4.0`.

Local source inspection found:

- `fractal-fuse = 0.4.0` uses io_uring for the FUSE transport/session layer, not for ScreenFS backing host file I/O.
- `compio-driver` has low-level `ReadAt`, `WriteAt`, and `Sync` style operations, but ScreenFS does not directly depend on that crate and would need an approved API/dependency design before use.
- `io-uring = 0.7.12` exposes read/write/fsync/fallocate opcodes, but direct usage would require explicit dependency, runtime/submission ownership, buffer lifetime design, and error/cancellation semantics.
- A direct `copy_file_range` opcode surface was not found in the local `io-uring = 0.7.12` source, so replacing `libc::copy_file_range` is not currently a low-risk direct translation.

## Benchmark gate

No formal file-data-path benchmark artifact currently proves a bottleneck in `read`, `write`, `copy_file_range`, `fallocate`, or `fsync`. Existing smoke evidence records startup, RSS, fd count, and policy behavior, and `docs/operations.md` explicitly treats those as smoke evidence rather than formal benchmark results.

Before implementing a selective data-path async/io_uring change, record at least:

- baseline command and workload for each candidate operation
- file sizes, offsets, byte counts, and concurrency level
- latency/throughput and CPU or syscall evidence
- chosen API/dependency and why it preserves current handle and errno semantics
- focused semantic tests for offsets, return byte counts, hidden `ENOENT`, mutability `EROFS`, source-root confinement, symlink target visibility, and handle lifecycle

## Current recommendation

Do not implement host-side file-data-path io_uring in the current state. The next safe step is benchmark and API design work. If implementation is later approved by evidence, start with `read`/`write` only; treat `fsync` as a separate candidate; keep `copy_file_range` and `fallocate` deferred unless direct API and benchmark evidence justify them.
