# Current state-lock concurrency evidence

This artifact records the current design decision for ScreenFS state-lock granularity work.

## Current consistency domain

`src/fs/state.rs` keeps these concerns in one consistency domain:

- inode allocation and inode records
- path-to-inode reverse map
- lookup and open reference counts
- open file handle table
- open directory handle and lightweight page/cookie state table; full-directory child attr/inode snapshots are not kept at `opendir` time
- parent/exact/tree invalidation after mutation

These values are coupled: inode eviction depends on both lookup and open refs, and mutation invalidation must update directory page/cookie state, path mappings, and inode records consistently.

## Selected low-risk direction

Use a single `RwLock<State>` consistency domain rather than per-table locks.

Rationale:

- read-only snapshots such as `path_for_inode`, file-handle snapshots, and directory resume snapshots can run concurrently when they do not update lookup refs
- cross-table writes still remain atomic under one write lock
- no multi-lock acquisition order is needed in the current design
- per-table locking would require a separate refcount/invalidation design before it is safe

## Lock rules

- Use a read lock for immutable snapshots only.
- Use a write lock for inode allocation, lookup ref increments, open ref increments/decrements, handle insertion/removal, directory page/cookie state insertion/removal/progress commits, and mutation invalidation.
- Never perform host filesystem I/O or blocking syscalls while holding the state lock when a snapshot can be taken first.
- Already-open file `read`/`write` data operations use offset-based `FileExt::read_at`/`write_at` so state-lock-free data I/O does not race on the shared open file description offset.
- `flush`, `fsync`, and `release(flush)` must snapshot or remove the already-open file handle before calling `sync_all`/`fdatasync`/`fsync`; any runtime blocking-offload (`compio_runtime::spawn_blocking` in the current FUSE runtime, or an explicitly approved equivalent) must keep that blocking sync syscall outside the state lock, await completion before replying, and must not change policy/public API semantics.
- Do not upgrade a read lock to a write lock; drop the read lock and reacquire explicitly through a helper if a write is required.
- `readdirplus` may collect page candidates outside the state lock, but committing the returned page's inode/cookie state and incrementing lookup refs for that page's returned child entries must happen in one write-lock transaction after revalidating the directory handle. It must not pin offset-after entries that are outside the page or entries that the FUSE `size` budget will prevent from reaching the kernel.

## Low-risk executor-offload boundary

Current source keeps `flush`, `fsync`, and `release(flush)` sync syscalls outside the state lock after file-handle snapshot or removal, then runs those syscalls through the current runtime's blocking-offload surface. Ambient-runtime-free direct library calls use a fallback thread offload so they do not panic solely because no compio runtime is entered. This is a low-risk concurrency cleanup: it changes executor placement only, not handle lifetime ordering, not metadata/path policy operations, not cache/discovery behavior, not public API semantics, and not `read`/`write` offset semantics. `read`/`write` continue to use `FileExt::read_at`/`write_at`. The implementation declares the runtime API dependency explicitly, preserves raw syscall errno, maps non-OS join/panic failures deterministically to `EIO`, and records blocking-pool/thread-budget expectations for close/fsync-heavy workloads in `current-file-data-path-async-feasibility.md`.

## Known non-goals

- Do not split inode, file-handle, and directory-handle maps into independent locks in the current change.
- Do not change visibility/mutability policy semantics, symlink target checks, source-root confinement, or discovery-free visible rules.
- Do not introduce recursive discovery, background indexing, or stable global directory listing caches.

## Validation expectations

Required checks for this change family:

- `cargo fmt --check`
- `cargo check`
- `cargo clippy --all-targets --all-features`
- `cargo test --all-targets --all-features`
- focused state tests for lookup/open ref eviction, directory handle lifecycle, bounded readdir/readdirplus page behavior, page-local readdirplus lookup-ref accounting, mutation invalidation, concurrent read-only snapshot behavior, lseek handle lifecycle, offset-based read/write behavior, and `flush`/`fsync`/`release(flush)` success/error propagation with no blocking sync syscall under the state lock
