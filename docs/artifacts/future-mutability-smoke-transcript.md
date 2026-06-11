# Legacy family-era mutability smoke transcript

Historical note: this transcript uses legacy family-era CLI/log vocabulary. Keep the command output as archival smoke evidence only; current policy contract is the visibility/mutability two-axis model and does not support these legacy options as aliases.

- Captured: 2026-06-10T23:15:44+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs

## Runtime environment
```text
$ ls -l /dev/fuse
crw-rw-rw- 1 root root 10, 229 Jun 10 23:10 /dev/fuse
[exit 0]
$ fusermount3 --version
fusermount3 version: 3.18.2
[exit 0]
```

## Build
```text
$ cargo build --offline
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
[exit 0]
```

## Selective-readonly smoke with relative and tilde normalization
```text
$ (cd /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src/workspace/app && HOME=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src/home/tester /home/spi-ca/Codebase/screenfs/target/debug/screenfs /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective --policy-family selective-readonly --hide ../secrets --readonly-rule ./fixtures/**/*.lock --readonly-rule '~/locks/**/*.lock')
mount active=1
$ ls /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective
allowed
allowed-config
free
hidden.txt
home
locked
workspace
[exit 0]
$ stat /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/workspace/secrets
stat: cannot statx '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/workspace/secrets': No such file or directory
[exit 1]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/workspace/app/fixtures/new.lock
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/workspace/app/fixtures/new.lock': Read-only file system
[exit 1]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/home/tester/locks/new.lock
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/home/tester/locks/new.lock': Read-only file system
[exit 1]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective/free/ok.txt
[exit 0]
$ fusermount3 -u /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective
[exit 0]
screenfs log:
$ cat /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/selective.log
mounting screenfs: source=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src mount=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective mutability-family=selective-readonly mutability-source=cli readonly-rules=2 allow-write-rules=0 hide-policy=compiled io_uring=required
[exit 0]
```

## Readonly-root-allowwrite smoke
```text
$ /home/spi-ca/Codebase/screenfs/target/debug/screenfs /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve --policy-family readonly-root-allowwrite --allow-write /allowed --hide /hidden.txt
mount active=1
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve/allowed/ok.txt
[exit 0]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve/free/blocked.txt
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve/free/blocked.txt': Read-only file system
[exit 1]
$ stat /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve/hidden.txt
stat: cannot statx '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve/hidden.txt': No such file or directory
[exit 1]
$ fusermount3 -u /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve
[exit 0]
screenfs log:
$ cat /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/carve.log
mounting screenfs: source=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src mount=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-carve mutability-family=readonly-root-allowwrite mutability-source=cli readonly-rules=0 allow-write-rules=1 hide-policy=compiled io_uring=required
[exit 0]
```

## Config-backed readonly-root-allowwrite smoke
```text
$ /home/spi-ca/Codebase/screenfs/target/debug/screenfs /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config --config /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/config.yaml --hide /hidden.txt
mount active=1
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config/allowed-config/from-config.txt
[exit 0]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config/free/blocked.txt
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config/free/blocked.txt': Read-only file system
[exit 1]
$ fusermount3 -u /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config
[exit 0]
screenfs log:
$ cat /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/config.log
mounting screenfs: source=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src mount=/home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-config mutability-family=readonly-root-allowwrite mutability-source=config readonly-rules=0 allow-write-rules=1 hide-policy=compiled io_uring=required
[exit 0]
```

## Fail-fast conflicts
```text
$ /home/spi-ca/Codebase/screenfs/target/debug/screenfs /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/src /home/spi-ca/Codebase/screenfs/.tmp/family-smoke-dUiz2W/mnt-selective --readonly-rule /locked --allow-write /allowed
--readonly-rule and --allow-write cannot be used together
[exit 2]
```
