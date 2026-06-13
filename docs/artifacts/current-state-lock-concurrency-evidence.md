# Current state-lock concurrency evidence

This artifact records the current design decision for ScreenFS state-lock granularity work.

## Current consistency domain

`src/fs/state.rs` keeps these concerns in one consistency domain:

- inode allocation and inode records
- path-to-inode reverse map
- lookup and open reference counts
- open file handle table
- open directory handle and snapshot table
- parent/exact/tree invalidation after mutation

These values are coupled: inode eviction depends on both lookup and open refs, and mutation invalidation must update directory snapshots, path mappings, and inode records consistently.

## Selected low-risk direction

Use a single `RwLock<State>` consistency domain rather than per-table locks.

Rationale:

- read-only snapshots such as `path_for_inode`, file-handle snapshots, and directory snapshot reads can run concurrently
- cross-table writes still remain atomic under one write lock
- no multi-lock acquisition order is needed in the current design
- per-table locking would require a separate refcount/invalidation design before it is safe

## Lock rules

- Use a read lock for immutable snapshots only.
- Use a write lock for inode allocation, lookup ref increments, open ref increments/decrements, handle insertion/removal, directory snapshot insertion/removal, and mutation invalidation.
- Never perform host filesystem I/O or blocking syscalls while holding the state lock when a snapshot can be taken first.
- Already-open file `read`/`write` data operations use offset-based `FileExt::read_at`/`write_at` so state-lock-free data I/O does not race on the shared open file description offset.
- Do not upgrade a read lock to a write lock; drop the read lock and reacquire explicitly through a helper if a write is required.
- `readdirplus` must take the directory snapshot and increment lookup refs for returned child entries in one write-lock transaction to avoid returning inodes that can be invalidated before they are pinned.

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
- focused state tests for lookup/open ref eviction, directory handle lifecycle, readdir/readdirplus snapshot behavior, mutation invalidation, concurrent read-only snapshot behavior, lseek handle lifecycle, and offset-based read/write behavior
