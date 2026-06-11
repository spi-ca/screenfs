# Current whole-root/chroot two-axis ScreenFS smoke transcript

- Captured: 2026-06-12T02:19:05+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs
- Source root: /
- Mount root: /tmp/screenfs-whole-root-smoke-knNCV2/mnt

## Environment
```text
$ id
uid=1000(spi-ca) gid=1000(spi-ca) groups=1000(spi-ca),967(realtime),994(input),998(wheel)
$ uname -r
7.1.0-rc7-1-spica-git
$ ls -l /dev/fuse
crw-rw-rw- 1 root root 10, 229 Jun 11 13:44 /dev/fuse
$ fusermount3 --version
fusermount3 version: 3.18.2
$ unshare --version
unshare from util-linux 2.42.1
```

## Config
```yaml
visibility:
  default: visible
  hidden:
    - /root
    - /tmp/screenfs-whole-root-smoke-hidden
mutability:
  default: readonly
  writable:
    - /tmp
```

## Mount
```text
$ target/debug/screenfs / /tmp/screenfs-whole-root-smoke-knNCV2/mnt --config /tmp/screenfs-whole-root-smoke-knNCV2/config.yaml
mounted pid=168399 startup_ms=294
stderr:
mounting screenfs: source=/ mount=/tmp/screenfs-whole-root-smoke-knNCV2/mnt visibility-default=visible visibility-source=config hidden-rules=2 visible-rules=0 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=1 io_uring=required
```

## Whole-root visibility and mutability checks
```text
$ test -e /tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/passwd
$ head -n 1 /tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/passwd
root:x:0:0::/root:/usr/bin/fish
$ stat /tmp/screenfs-whole-root-smoke-knNCV2/mnt/root
stat: cannot statx '/tmp/screenfs-whole-root-smoke-knNCV2/mnt/root': No such file or directory
exit_status=1
$ sh -c printf no > '/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/screenfs-denied-probe'
sh: line 1: /tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/screenfs-denied-probe: Read-only file system
exit_status=1
$ sh -c printf 'ok\n' > '/tmp/screenfs-whole-root-smoke-knNCV2/mnt/tmp/screenfs-whole-root-probe' && cat '/tmp/screenfs-whole-root-smoke-knNCV2/mnt/tmp/screenfs-whole-root-probe'
ok
$ rm -f /tmp/screenfs-whole-root-smoke-knNCV2/mnt/tmp/screenfs-whole-root-probe
```

## Chroot/user-namespace smoke
```text
$ unshare -r -R /tmp/screenfs-whole-root-smoke-knNCV2/mnt -w / /bin/sh -c pwd; test -e /etc/passwd; head -n 1 /etc/passwd; stat /root; printf 'ok\n' > /tmp/screenfs-whole-root-probe && cat /tmp/screenfs-whole-root-probe; rm -f /tmp/screenfs-whole-root-probe
/
root:x:0:0::/root:/usr/bin/fish
stat: cannot statx '/root': No such file or directory
ok
```

## Lightweight whole-root performance snapshot
```text
$ sh -c find '/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc' -maxdepth 1 -print | sed -n '1,25p'
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/.pwd.lock
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/.updated
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/NetworkManager
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/OpenCL
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/PackageKit
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/UPower
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/X11
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/adjtime
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/alsa
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/arch-release
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/arptables.conf
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/audisp
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/audit
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/avahi
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/bash.bash_logout
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/bash.bashrc
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/bindresvport.blacklist
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/binfmt.d
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/bluetooth
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/ca-certificates
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/cifs-utils
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/colord
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/conf.d
/tmp/screenfs-whole-root-smoke-knNCV2/mnt/etc/containers
rss_kb=2113504
fd_count=22
```

## Unmount
```text
$ fusermount3 -u /tmp/screenfs-whole-root-smoke-knNCV2/mnt
unmounted
```
