# Current FUSE transport contract evidence

This artifact records the current source-level evidence for the ScreenFS v1 `FUSE_OVER_IO_URING` transport contract. It is not a live negative smoke on a kernel without `FUSE_OVER_IO_URING`; the current development kernel advertises the required support. Treat this as dependency-source plus local no-fallback evidence, and keep live mount smoke evidence separate.

## Environment premise

Recorded environment checks in this workspace:

```text
kernel: 7.1.0-rc7-1-spica-git
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
/dev/fuse: present
```

`docs/operations.md` continues to treat kernel config evidence as an environment premise rather than proof of a successful live mount or negotiation.

## Dependency-source enforcement

ScreenFS depends on `fractal-fuse = 0.4.0` (`Cargo.toml`). Local crate source inspected at:

```text
/home/spi-ca/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fractal-fuse-0.4.0/src/session.rs
```

Relevant `fractal-fuse` behavior:

- `parse_fuse_init` reconstructs the kernel `FUSE_INIT` flags and returns `io::ErrorKind::Unsupported` with message `kernel does not support FUSE_OVER_IO_URING (requires Linux 6.14+)` when the kernel does not advertise `FUSE_OVER_IO_URING`.
- `write_fuse_init_reply` starts `want_flags` with `FUSE_OVER_IO_URING`, intersects with kernel flags, and returns `io::ErrorKind::Unsupported` with message `FUSE_OVER_IO_URING not supported after negotiation` if the negotiated flags lose it.

These errors are the dependency-level fail-fast surface for unsupported or unsuccessfully negotiated FUSE transport.

## ScreenFS local no-fallback surface

`src/main.rs` keeps the startup path direct:

```text
screenfs_mount_options()
Session::new(mount_root, opts)?.run(ScreenFs::new(cfg))
```

There is no fallback mount path, retry with degraded options, or host filesystem `io_uring` conversion in ScreenFS. The local unit test `tests::screenfs_mount_options_preserve_current_mount_policy` fixes the mount option policy that ScreenFS passes into `fractal-fuse`:

- `fs_name = screenfs`
- `read_only = false`
- `force_readdir_plus = true`
- `default_permissions = false`
- `allow_other = false`
- `allow_root = false`
- `uid/gid/rootmode/custom_options = None`
- no lock, passthrough, writeback, killpriv, no-open, or masking capability opt-ins

The no-fallback behavior is intentionally code-structure evidence: the `Session::run` error is surfaced directly to `main`'s `io::Result<()>` without a degraded retry branch.

## Current verification commands

The focused mount-option regression test is:

```bash
cargo test tests::screenfs_mount_options_preserve_current_mount_policy --bin screenfs -- --exact --nocapture
```

Full validation for the change should still include:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```
