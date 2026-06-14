# Current file-data-path async/io_uring feasibility evidence

This artifact records current feasibility evidence for selective host-side async/io_uring work on already-open file handle data operations. It also records why `flush`/`fsync`/`release(flush)` `spawn_blocking` offload is a separate low-risk concurrency cleanup rather than host-side io_uring conversion. It is a planning and gating artifact, not proof that ScreenFS currently uses io_uring for backing file data I/O.

## Candidate scope

Only the following already-open file handle operations are candidates for a selective async/io_uring follow-up:

- `read`
- `write`
- `copy_file_range`
- `fallocate` only if benchmark/dependency evidence supports it

The candidate scope excludes lookup/getattr/readdir/readdirplus/readlink/xattr/setattr/rename/link/symlink/unlink/rmdir/mkdir/mknod, policy evaluation, path resolution, source-root confinement, symlink target visibility, directory filtering, recursive discovery, mount transport negotiation, and the separate sync-surface executor-offload cleanup described below.

## Separate low-risk sync cleanup surface

`flush`, `fsync`, and `release(flush)` already operate on previously opened file handles and already keep their blocking sync syscalls outside the `State` lock after handle snapshot or removal. Offloading those syscalls with the current FUSE runtime's blocking-offload surface (`compio_runtime::spawn_blocking`, or an explicitly approved equivalent if the runtime changes) changes executor placement only: it moves `sync_all`/`fdatasync`/`fsync` to a blocking pool outside the state lock and awaits the result before replying. That is not a host-side io_uring conversion, not a benchmark-gated wholesale async data-path redesign, and not a reason to change `read`/`write` away from `FileExt::read_at`/`write_at`.

## Current implementation baseline

Current ScreenFS data operations in `src/fs.rs` use synchronous host file descriptors after policy guards:

- `read`: `file_handle_snapshot` -> `guard_read_path` -> offset-based `FileExt::read_at`
- `write`: `file_handle_snapshot` -> `guard_mutation_path` -> offset-based `FileExt::write_at`
- `copy_file_range`: input/output handle validation -> input visibility guard -> output path and parent mutability guard -> `libc::copy_file_range`
- `fallocate`: `file_handle_snapshot` -> `guard_mutation_path` -> `libc::fallocate`

Current sync surfaces on already-open file handles are:

- `flush`: `file_handle_snapshot` -> runtime blocking-offload -> `std::fs::File::sync_all`
- `fsync`: `file_handle_snapshot` -> runtime blocking-offload -> `libc::fdatasync` or `libc::fsync`
- `release(flush=true)`: remove handle from state under write lock -> drop lock -> runtime blocking-offload -> `std::fs::File::sync_all` on the removed handle

Current handle state stores `Arc<std::fs::File>` and snapshots the shared file handle for each data operation. That model preserves short state-lock windows and keeps host I/O outside the `State` lock. Offset-based read/write avoid racing on the shared open file description offset, and the sync surfaces above keep the same no-blocking-host-I/O-under-state-lock rule while moving sync syscalls to the runtime blocking-offload path.

## Dependency/API evidence

Current direct dependencies in `Cargo.toml` do not expose a host-side io_uring API for ScreenFS. `compio-runtime = 0.11.0` is a direct dependency only for runtime blocking-offload of existing sync syscalls; `compio-driver` and `io-uring` remain transitive dependencies of `fractal-fuse = 0.4.0` and are not used as ScreenFS host-side data-path io_uring APIs.

Local source inspection found:

- `fractal-fuse = 0.4.0` uses io_uring for the FUSE transport/session layer, not for ScreenFS backing host file I/O.
- `compio-driver` has low-level `ReadAt`, `WriteAt`, and `Sync` style operations, but ScreenFS does not directly depend on that crate and would need an approved API/dependency design before use.
- `io-uring = 0.7.12` exposes read/write/fsync/fallocate opcodes, but direct usage would require explicit dependency, runtime/submission ownership, buffer lifetime design, and error/cancellation semantics.
- A direct `copy_file_range` opcode surface was not found in the local `io-uring = 0.7.12` source, so replacing `libc::copy_file_range` is not currently a low-risk direct translation.
- The runtime blocking-offload cleanup for `flush`/`fsync`/`release(flush)` reuses the existing already-open handle and no-state-lock-host-I/O structure; it does not adopt a host-side io_uring API, change path/policy semantics, or change the public FUSE surface. ScreenFS declares `compio-runtime` directly so `compio_runtime::spawn_blocking` is an explicit runtime API dependency instead of an undeclared transitive dependency.

## Benchmark gate

No formal file-data-path benchmark artifact currently proves a bottleneck in `read`, `write`, `copy_file_range`, or `fallocate`. Existing smoke evidence records startup, RSS, fd count, and policy behavior, and `docs/operations.md` explicitly treats those as smoke evidence rather than formal benchmark results.

Before implementing a selective data-path async/io_uring change, record at least:

- baseline command and workload for each candidate operation
- file sizes, offsets, byte counts, and concurrency level
- latency/throughput and CPU or syscall evidence
- chosen API/dependency and why it preserves current handle and errno semantics
- focused semantic tests for offsets, return byte counts, hidden `ENOENT`, mutability `EROFS`, source-root confinement, symlink target visibility, and handle lifecycle

For `flush`/`fsync`/`release(flush)` blocking-offload changes, record at least:

- which sync surface is being offloaded and which blocking syscall it uses
- the runtime blocking-offload API used, including any direct dependency added to make that API explicit
- source evidence that handle snapshot/removal finishes before the offload and that no state lock is held during the blocking syscall
- blocking-pool/thread-budget and queueing/backpressure expectations for close/fsync-heavy workloads. Current compio/fractal-fuse runtime evidence indicates per-runtime asyncify blocking-pool behavior with a default limit of 256, and fractal-fuse may create multiple worker runtimes; close/fsync-heavy workloads can therefore increase thread count and tail latency even though correctness is preserved. Ambient-runtime-free direct library calls use a fallback thread offload so they do not panic when no compio runtime is entered; that fallback is outside the mounted FUSE hot path and has thread-per-call resource behavior rather than compio pool backpressure.
- errno/result mapping for syscall failures and join/panic failures; syscall `errno` is preserved and non-OS join/panic failures map to deterministic `EIO`
- focused tests for success/error propagation, no-runtime fallback completion, and handle cleanup on `flush`, `fsync`, and `release(flush)`
- explicit confirmation that metadata/path policy operations, cache/discovery behavior, public API semantics, and `read`/`write` `FileExt::read_at`/`write_at` behavior did not change

## Current recommendation

Do not implement host-side file-data-path io_uring in the current state. The next safe step for that track is still benchmark and API design work, starting with `read`/`write` only and keeping `copy_file_range`/`fallocate` deferred unless direct API and benchmark evidence justify them. Separately, do not describe `flush`/`fsync`/`release(flush)` blocking-offload cleanup as io_uring work: if approved, keep it limited to executor placement for existing blocking sync syscalls outside the state lock, use the current FUSE runtime's blocking-offload API explicitly, await completion before replying, define join/panic-to-errno behavior, and preserve current `FileExt::read_at`/`write_at` read/write behavior.
