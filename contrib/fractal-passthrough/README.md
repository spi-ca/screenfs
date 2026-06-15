# fractal-passthrough

Minimal `fractal-fuse = 0.4.0` passthrough filesystem used as a managed benchmark baseline for ScreenFS fio attribution.

This helper exists to measure a simple FUSE/userspace passthrough floor with the same Rust FUSE transport family as ScreenFS. It is not a ScreenFS feature and does not implement ScreenFS visibility or mutability policy.

Security boundary warning: this helper is benchmark-only and does not implement ScreenFS source-root confinement. Use it only with a trusted private scratch `SOURCE` tree that does not contain symlinks to external paths. The binary refuses to run as root and rejects a `MOUNT` directory inside `SOURCE`, but it is still not a sandbox.

## Build

```bash
cargo build --release --manifest-path contrib/fractal-passthrough/Cargo.toml
```

## Run

Run as a regular non-root user with an existing mount directory:

```bash
contrib/fractal-passthrough/target/release/fractal_passthrough <SOURCE> <MOUNT>
```

Unmount with:

```bash
fusermount3 -u <MOUNT>
```

## Benchmark use

Use the same fio job against native, this passthrough mount, and ScreenFS. Record the binary path, binary SHA256, command line, mount stderr, fstype, and fio JSON outputs in `docs/artifacts/`.

The baseline is intentionally minimal and attribution-oriented. Do not treat a single warm-cache passthrough comparison as claim-grade ScreenFS performance evidence.
