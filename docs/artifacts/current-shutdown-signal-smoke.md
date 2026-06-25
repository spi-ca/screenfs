# Current shutdown signal smoke

Date: 2026-06-25

Scope: repo-local live smoke for ScreenFS shutdown signal handling after adding the `src/main.rs` signal forwarder.

## Environment

- Binary: `./target/debug/screenfs` built with `cargo build --bin screenfs`
- `fusermount3 --version`: `fusermount3 version: 3.18.2`
- Source root: temporary directory from `mktemp -d`, containing `file`
- Mount root: temporary directory from `mktemp -d`
- Policy: defaults (`visibility.default=visible`, `mutability.default=writable`)
- Unmount tool: `fusermount3`; normal cleanup is performed by `fractal-fuse` `Session::run` via `fusermount3 -u`, and ScreenFS retries explicit `fusermount3 -u` if `/proc/self/mountinfo` still shows the ScreenFS mount after `Session::run` returns.

## Source evidence

- `src/main.rs:73-78` calls `block_shutdown_signals()`, starts a `sigwait` forwarding thread, creates the FUSE session, and installs `SessionShutdownHandle` before entering `Session::run`; `src/main.rs:96-101` shows the `SIGINT`/`SIGTERM` set and `SIG_BLOCK` call.
- `src/main.rs:126-164` records an early signal until the shutdown handle exists; once ready, the forwarding path logs the signal and calls `SessionShutdownHandle::shutdown()`.
- `src/main.rs:169-189` is the `sigwait` loop, so additional `SIGINT`/`SIGTERM` events remain observable while graceful teardown is in progress.
- `fractal-fuse = 0.4.0` `session.rs:141-154` is the FUSE serve-loop boundary; after `run_inner` returns, it performs normal unmount cleanup. `mount.rs:49-62` shows that cleanup command is `fusermount3 -u <mount-path>`.
- `src/main.rs:86-87` calls stale-mount inspection after `Session::run` returns; `src/main.rs:221-266` normalizes the mount root, retries explicit normal `fusermount3 -u` when inspection reports a remaining mount, and only prints lazy-unmount guidance if that normal retry fails. `src/main.rs:273-293` reads `/proc/self/mountinfo` and restricts matching to `fuse.screenfs` with source `screenfs`.
- `src/main.rs:278-290` restricts stale-mount detection to `fuse.screenfs` with source `screenfs`, avoiding path-only matches against unrelated mounts.

## SIGTERM path

Command shape:

```bash
cargo build --bin screenfs
./target/debug/screenfs "$work/src" "$work/mnt" >"$log" 2>&1 &
pid=$!
# wait until /proc/self/mountinfo contains "$work/mnt"
sleep 1
kill -TERM "$pid"
wait "$pid"
# verify /proc/self/mountinfo no longer contains "$work/mnt"
```

Observed output:

```text
SIGTERM status=0 still=0
mounting screenfs: source=/tmp/tmp.5CXdQiA3NK/src mount=/tmp/tmp.5CXdQiA3NK/mnt visibility-default=visible visibility-source=default hidden-rules=0 visible-rules=0 mutability-default=writable mutability-source=default readonly-rules=0 writable-rules=0 io_uring=required
received SIGTERM, requesting graceful ScreenFS shutdown
```

Evidence summary:

- `SIGTERM` was detected and logged.
- The signal path requested graceful shutdown through the session shutdown handle.
- The process exited with status `0`.
- `/proc/self/mountinfo` no longer contained the mount root after exit.

## SIGINT path

Command shape:

```bash
cargo build --bin screenfs
./target/debug/screenfs "$work/src" "$work/mnt" >"$log" 2>&1 &
pid=$!
# wait until /proc/self/mountinfo contains "$work/mnt"
sleep 1
kill -INT "$pid"
wait "$pid"
# verify /proc/self/mountinfo no longer contains "$work/mnt"
```

Observed output:

```text
SIGINT status=0 still=0
mounting screenfs: source=/tmp/tmp.TykH3SSy9j/src mount=/tmp/tmp.TykH3SSy9j/mnt visibility-default=visible visibility-source=default hidden-rules=0 visible-rules=0 mutability-default=writable mutability-source=default readonly-rules=0 writable-rules=0 io_uring=required
received SIGINT, requesting graceful ScreenFS shutdown
```

Evidence summary:

- `SIGINT` was detected and logged.
- The signal path requested graceful shutdown through the session shutdown handle.
- The process exited with status `0`.
- `/proc/self/mountinfo` no longer contained the mount root after exit.

## Explicit normal unmount retry path

A deliberately early SIGTERM probe exercised the stale-mount detection and explicit normal-unmount retry path.

Command shape:

```bash
cargo build --bin screenfs
./target/debug/screenfs "$work/src" "$work/mnt" >"$log" 2>&1 &
pid=$!
# wait until /proc/self/mountinfo contains "$work/mnt"
kill -TERM "$pid"
wait "$pid"
# verify /proc/self/mountinfo no longer contains "$work/mnt"
```

Observed output:

```text
early status=0 still=0
mounting screenfs: source=/tmp/tmp.lTXKaAfS8H/src mount=/tmp/tmp.lTXKaAfS8H/mnt visibility-default=visible visibility-source=default hidden-rules=0 visible-rules=0 mutability-default=writable mutability-source=default readonly-rules=0 writable-rules=0 io_uring=required
received SIGTERM, requesting graceful ScreenFS shutdown
/usr/bin/fusermount3: failed to unmount /tmp/tmp.lTXKaAfS8H/mnt: Device or resource busy
ScreenFS shutdown returned but /tmp/tmp.lTXKaAfS8H/mnt still appears mounted; retrying normal cleanup with: fusermount3 -u '/tmp/tmp.lTXKaAfS8H/mnt'
normal fusermount3 -u cleanup succeeded for /tmp/tmp.lTXKaAfS8H/mnt
```

Evidence summary:

- `fractal-fuse`'s first normal `fusermount3 -u` attempt failed with `Device or resource busy`.
- ScreenFS detected that the mount still appeared in `/proc/self/mountinfo`.
- ScreenFS retried explicit normal cleanup with `fusermount3 -u <mount-root>`.
- The retry succeeded and the mount was gone after process exit (`still=0`).

## Lazy unmount policy

Lazy unmount is not automatic. If both normal `fusermount3 -u` attempts fail and the mount still appears stale, ScreenFS prints operator guidance in this form instead of running lazy unmount itself:

```text
Lazy unmount is not automatic. If the mount is stale and no process should keep it busy, clean it up manually with: fusermount3 -uz '<mount-root>'
```
