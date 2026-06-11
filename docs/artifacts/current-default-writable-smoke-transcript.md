# Current default-writable two-axis ScreenFS smoke transcript

- Captured: 2026-06-12T02:08:03+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs
- Temp root: /tmp/screenfs-default-writable-smoke-5pYzs8

## Config
```yaml
visibility:
  default: visible
  hidden:
    - /hidden
mutability:
  default: writable
  readonly:
    - /locked
```

## Mount
```text
$ target/debug/screenfs /tmp/screenfs-default-writable-smoke-5pYzs8/src /tmp/screenfs-default-writable-smoke-5pYzs8/mnt --config /tmp/screenfs-default-writable-smoke-5pYzs8/config.yaml
mounted pid=163322
stderr:
mounting screenfs: source=/tmp/screenfs-default-writable-smoke-5pYzs8/src mount=/tmp/screenfs-default-writable-smoke-5pYzs8/mnt visibility-default=visible visibility-source=config hidden-rules=1 visible-rules=0 mutability-default=writable mutability-source=config readonly-rules=1 writable-rules=0 io_uring=required
```

## Checks
```text
$ sh -c printf 'ok\n' > '/tmp/screenfs-default-writable-smoke-5pYzs8/mnt/open/file' && cat '/tmp/screenfs-default-writable-smoke-5pYzs8/mnt/open/file'
ok
$ sh -c printf 'blocked\n' > '/tmp/screenfs-default-writable-smoke-5pYzs8/mnt/locked/file'
sh: line 1: /tmp/screenfs-default-writable-smoke-5pYzs8/mnt/locked/file: Read-only file system
exit_status=1
$ stat /tmp/screenfs-default-writable-smoke-5pYzs8/mnt/hidden/file
stat: cannot statx '/tmp/screenfs-default-writable-smoke-5pYzs8/mnt/hidden/file': No such file or directory
exit_status=1
$ sh -c printf hidden > '/tmp/screenfs-default-writable-smoke-5pYzs8/mnt/hidden/file'
sh: line 1: /tmp/screenfs-default-writable-smoke-5pYzs8/mnt/hidden/file: No such file or directory
exit_status=1
```

## Unmount
```text
$ fusermount3 -u /tmp/screenfs-default-writable-smoke-5pYzs8/mnt
unmounted
```
