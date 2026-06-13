# Current Path-Based TOCTOU Hardening Evidence

Date: 2026-06-12

## Scope

This artifact tracks the ScreenFS hardening work for the security-review finding that path-based syscalls after policy checks could race with same-UID path replacement. The current implementation direction is to keep policy decisions in virtual-path space, then pin the concrete parent or opened object before performing host filesystem operations.

## Current implementation evidence

- `ScreenFs::new()` pins `source_root` as an `O_PATH|O_DIRECTORY` fd and `open_confined()` uses that fd for `openat2(..., RESOLVE_IN_ROOT|RESOLVE_NO_MAGICLINKS)`, avoiding per-request reopening of the configured root path.
- `src/fs/backing.rs` adds pinned parent helpers:
  - `open_parent_dir()` opens the virtual parent through `open_confined()`.
  - `stat_child_no_follow()` uses parent-dirfd `fstatat(..., AT_SYMLINK_NOFOLLOW)`.
  - `readlink_child()` uses parent-dirfd `readlinkat()`.
  - `read_dir_entries()` enumerates an already-open directory with `fdopendir()`/`readdir()` and stats children by dirfd.
- `src/fs/guards.rs` revalidates opened fd targets:
  - `guard_opened_file_target()` still requires fully visible final file targets.
  - `guard_opened_directory_target()` allows bridge-visible directories for traversal/listing while still applying mutation restrictions when requested.
- `src/fs.rs` defers `O_TRUNC` for `open`/`create` until after opened-target visibility/mutability validation, so a raced final symlink cannot truncate a hidden/readonly target before the post-open guard runs.
- `src/fs.rs` replaces representative path mutations with dirfd syscalls:
  - `symlinkat()` for symlink creation and `unlinkat()` rollback.
  - `mknodat()` for node creation.
  - `unlinkat()` / `unlinkat(..., AT_REMOVEDIR)` for unlink/rmdir.
  - `mkdirat()` plus fd-pinned `fchmod()` for mkdir mode.
  - `renameat2()` for rename.
  - `linkat()` for hard links.
- `ReplyCreate.flags` remains `0`, because FUSE create replies use `FOPEN_*` reply flags rather than original open(2) request flags. This preserves the live `O_CREAT|O_EXCL` fix.
- `RuntimeConfig` uses zero attribute and entry TTLs. ScreenFS policy and backing state can change on every operation, and live FUSE rename/replace probes showed that a positive entry TTL let a just-renamed destination remain hidden behind a stale negative dentry until readdir refreshed it.

## Current validation evidence

Focused validation after the first hardening pass:

```bash
cargo fmt --all
cargo check
cargo test --all-targets --all-features
```

Observed result at this point:

- `cargo check`: passed.
- `cargo test --all-targets --all-features`: 124 library tests and 1 binary test passed before the failed-lookup regression was added; later serial full-suite validation observed 128 library tests and 1 binary test passing.
- Local live FUSE rename/replace probe with zero TTL observed immediate `exists=True` and successful same-process read after `os.rename()`.
- Focused source regressions include failed lookup state cleanup, visible symlink parent mutation through resolved parent, restrictive-mode `mkdir` success after creation, and write-intent `open(O_TRUNC)` through a hidden symlink returning `ENOENT` without truncating the target.

## Remaining work before goal completion

- Add or identify explicit focused tests for every hardened syscall family and race-sensitive metadata/listing behavior.
- Same-UID rename of an already-pinned parent directory after validation but before the final `*at` syscall is a Linux namespace/dirfd semantic limit, not a local ScreenFS implementation bug that can be fully removed with another `*at` call. A proof probe opened a directory fd, renamed that directory elsewhere, then created a child through the old fd; Linux created the child at the moved location (`created_path_exists=True`, `old_path_exists=False`). ScreenFS can still serialize its own requests or add best-effort pre/post validation, but that cannot prevent or safely roll back destructive operations raced by an external same-UID mutator.

## Parent-dirfd rename race option check

The remaining race requires one of the following threat-model decisions:

1. **Exclusive backing-tree control**: viable as an operational contract. If no external same-UID process mutates the backing tree behind ScreenFS, the pinned source-root and dirfd/`*at` implementation closes ScreenFS-internal path rewalk races. This is compatible with non-root `fusermount3` and with whole-root `source_root=/`, but it must be documented as an assumption because ScreenFS cannot enforce it for arbitrary host paths by itself.
2. **Namespace/permission isolation**: viable only as an upper-supervisor responsibility. A supervisor can mount/chroot/user-namespace or permission-isolate the backing tree so the consumer cannot concurrently rename backing parents outside ScreenFS. This fits existing architecture boundaries that leave chroot/user namespace setup to the supervisor, but it is not something the ScreenFS FUSE daemon can unilaterally guarantee for `source_root=/` without broader integration changes.
3. **Accept POSIX fd lifetime semantics**: viable as the minimal current contract. ScreenFS pins the source root and parent/object fds before operations; if an external same-UID actor moves an already-open directory or file, subsequent fd-relative operations follow the pinned inode, matching Linux/POSIX fd lifetime semantics. This should be documented as the remaining boundary, not treated as an implementation bug.
4. **Larger design change**: possible but scope-changing. Full current-virtual-path atomicity against external same-UID renames would require stronger coordination than plain Linux `*at` calls provide, such as ownership/lease protocols, backing-tree mediation, namespace isolation, or a different supervisor architecture. This is beyond the present dirfd/`*at` hardening pass and needs explicit design approval.

Selected current ScreenFS contract: **option 3, POSIX fd lifetime semantics with best-effort current-path validation**. ScreenFS pins `source_root` and operation parent/object fds and avoids path rewalk after policy checks where practical. Before fd-relative mutation syscalls, ScreenFS best-effort checks that the opened parent dirfd still resolves to the requested virtual parent path. This narrows the external rename window and catches already-moved parents, but it cannot make the check and subsequent Linux `*at` syscall atomic. If an external same-UID actor moves or unlinks an already-pinned directory/file after that validation, fd-relative operations follow the pinned inode according to Linux/POSIX semantics. Options 1 and 2 remain deployment hardening choices for supervisors that need stronger protection from external same-UID backing-tree mutation; option 4 remains a future architecture goal, not a hidden requirement of this hardening pass.
- Run `cargo fmt --check`, `cargo clippy --all-targets --all-features`, and the full test suite after final fixes.
- Run a local live FUSE smoke including `O_CREAT|O_EXCL`, listing, readlink, mkdir/unlink/rmdir/rename/link/symlink where practical.
- Obtain subagent code and security review with no unresolved blockers.
- Audit remaining path-based surfaces and either harden them or document why they are not in the requested risk class.
