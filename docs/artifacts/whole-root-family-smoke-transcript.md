# Whole-root legacy-family smoke transcript

Historical note: this transcript uses legacy family-era CLI/log vocabulary. Keep the command output as archival smoke evidence only; current policy contract is the visibility/mutability two-axis model documented in `docs/requirements.md` and `docs/operations.md`.

- Captured: 2026-06-11T00:25:00+09:00
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

## Whole-root readonly-root-allowwrite smoke
```text
$ /home/spi-ca/Codebase/screenfs/target/debug/screenfs / /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt --policy-family readonly-root-allowwrite --allow-write /tmp --hide /home/spi-ca/.ssh
mount active=1
$ stat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/bin/bash
  File: /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,77	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 2026-06-10 04:59:30.407418592 +0900
[exit 0]
$ ls /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/usr | head
bin
include
lib
lib64
libexec
local
sbin
share
src
var
[exit 0]
$ stat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/home/spi-ca/.ssh
stat: cannot statx '/home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/home/spi-ca/.ssh': No such file or directory
[exit 1]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/tmp/whole-root-allow-ok
[exit 0]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/var/tmp/whole-root-blocked
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt/var/tmp/whole-root-blocked': Read-only file system
[exit 1]
$ unshare -UrR /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt /bin/bash --noprofile --norc -lc 'echo chroot-bash-ok; test -x /bin/bash; stat /bin/bash; touch /tmp/chroot-allow-ok'
chroot-bash-ok
  File: /bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,77	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (65534/  nobody)   Gid: (65534/  nobody)
Access: 2026-06-10 04:59:30.407418592 +0900
Modify: 2025-12-11 07:02:55.000000000 +0900
Change: 2026-03-07 13:25:52.295336313 +0900
 Birth: -
[exit 0]
$ unshare -UrR /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt /bin/bash --noprofile --norc -lc 'touch /var/tmp/chroot-blocked'
touch: cannot touch '/var/tmp/chroot-blocked': Read-only file system
[exit 1]
$ fusermount3 -u /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt
[exit 0]
screenfs log:
$ cat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/root.log
mounting screenfs: source=/ mount=/home/spi-ca/Codebase/screenfs/.tmp/whole-root-family-HMMRwt/mnt mutability-family=readonly-root-allowwrite mutability-source=cli readonly-rules=0 allow-write-rules=1 hide-policy=compiled io_uring=required
[exit 0]
```
