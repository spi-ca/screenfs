# Current two-axis ScreenFS smoke transcript

- Captured: 2026-06-12T02:01:09+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs
- Temp root: /tmp/screenfs-axis-smoke-lAVClR

## Environment
```text
$ ls -l /dev/fuse
crw-rw-rw- 1 root root 10, 229 Jun 11 13:44 /dev/fuse
$ fusermount3 --version
fusermount3 version: 3.18.2
```

## Config
```yaml
visibility:
  default: hidden
  visible:
    - /workspace/**/.git/hooks/**
    - /tmp
    - /allowed
    - /link-to-bridge
mutability:
  default: readonly
  writable:
    - /tmp
    - /allowed
  readonly:
    - /allowed/reblock
```

## Mount
```text
$ target/debug/screenfs /tmp/screenfs-axis-smoke-lAVClR/src /tmp/screenfs-axis-smoke-lAVClR/mnt --config /tmp/screenfs-axis-smoke-lAVClR/config.yaml
mounted pid=160318
stderr:
mounting screenfs: source=/tmp/screenfs-axis-smoke-lAVClR/src mount=/tmp/screenfs-axis-smoke-lAVClR/mnt visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=4 mutability-default=readonly mutability-source=config readonly-rules=1 writable-rules=2 io_uring=required
```

## Visibility and bridge-visible checks
```text
$ find /tmp/screenfs-axis-smoke-lAVClR/mnt -maxdepth 5 -print
/tmp/screenfs-axis-smoke-lAVClR/mnt
/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed
/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/existing
/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/reblock
/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/reblock/existing
/tmp/screenfs-axis-smoke-lAVClR/mnt/tmp
/tmp/screenfs-axis-smoke-lAVClR/mnt/tmp/existing
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/hooks
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/hooks/pre-commit
$ find /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace -maxdepth 1 -print
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo
$ find /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo -maxdepth 1 -print
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git
$ find /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git -maxdepth 1 -print
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git
/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/hooks
$ cat /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/hooks/pre-commit
hook ok
$ stat /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/config
stat: cannot statx '/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/config': No such file or directory
exit_status=1
$ stat /tmp/screenfs-axis-smoke-lAVClR/mnt/link-to-bridge
stat: cannot statx '/tmp/screenfs-axis-smoke-lAVClR/mnt/link-to-bridge': No such file or directory
exit_status=1
$ readlink /tmp/screenfs-axis-smoke-lAVClR/mnt/link-to-bridge
exit_status=1
```

## Mutability checks
```text
$ chmod 755 /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace
chmod: changing permissions of '/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace': Read-only file system
exit_status=1
$ sh -c printf hidden > '/tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/config'
sh: line 1: /tmp/screenfs-axis-smoke-lAVClR/mnt/workspace/repo/.git/config: No such file or directory
exit_status=1
$ sh -c printf 'ok\n' > '/tmp/screenfs-axis-smoke-lAVClR/mnt/tmp/existing' && cat '/tmp/screenfs-axis-smoke-lAVClR/mnt/tmp/existing'
ok
$ sh -c printf 'ok\n' > '/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/existing' && cat '/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/existing'
ok
$ sh -c printf 'no\n' > '/tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/reblock/existing'
sh: line 1: /tmp/screenfs-axis-smoke-lAVClR/mnt/allowed/reblock/existing: Read-only file system
exit_status=1
```

## Fail-fast checks
```text
$ target/debug/screenfs /tmp/screenfs-axis-smoke-lAVClR/src /tmp/screenfs-axis-smoke-lAVClR/bad-mnt --hidden '*.pem'
Error: Custom { kind: InvalidInput, error: "invalid hidden pattern: unsupported glob: *.pem" }
$ target/debug/screenfs /tmp/screenfs-axis-smoke-lAVClR/src /tmp/screenfs-axis-smoke-lAVClR/bad-mnt --hidden /same --visible /same
Error: Custom { kind: InvalidInput, error: "hidden and visible rules conflict at the same normalized specificity" }
$ target/debug/screenfs /tmp/screenfs-axis-smoke-lAVClR/src /tmp/screenfs-axis-smoke-lAVClR/bad-mnt --hidden '/workspace/**/*.pem' --visible '/workspace/**/.git/hooks/**'
Error: Custom { kind: InvalidInput, error: "hidden and visible rules have overlapping glob targets without provable containment" }
```

## Lightweight performance/concurrency smoke
```text
$ for i in 1 2 3; do find /tmp/screenfs-axis-smoke-lAVClR/mnt -maxdepth 6 -print >/dev/null; done
elapsed_ms=6
rss_kb=2113364
fd_count=22
$ rg -n has_visible_descendant src/fs src/fs.rs
```

## Unmount
```text
$ fusermount3 -u /tmp/screenfs-axis-smoke-lAVClR/mnt
unmounted
```
