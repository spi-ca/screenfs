# FUSE smoke transcript

This artifact was regenerated from the root pi session, not from isolated subagents. It records runtime FUSE state, repo-local smoke, whole-root smoke, and `unshare -UrR` chroot execution using the renamed `screenfs` binary.

Historical note: this transcript predates the visibility/mutability two-axis policy model and the removal of legacy CLI surfaces. Keep all legacy commands and logs below as archival whole-root/chroot evidence only; do not use them as current ScreenFS policy contract or current invocation guidance.

- Captured: 2026-06-10T16:22:02+09:00
- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs

## Runtime environment
```text
$ ls -l /dev/fuse
crw-rw-rw- 1 root root 10, 229 Jun 10 08:46 /dev/fuse
[exit 0]
$ grep -w fuse /proc/filesystems
nodev	fuse
[exit 0]
$ fusermount3 --version
fusermount3 version: 3.18.2
[exit 0]
$ unshare -Ur true
[exit 0]
```

## Build
```text
$ cargo build --offline
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
[exit 0]
```

## Repo-local readonly hidden smoke
```text
$ target/debug/screenfs .tmp/fuse-smoke-current/repo-src .tmp/fuse-smoke-current/repo-mnt --readonly --hide /secret
mount active=1
[exit 0]
$ cat .tmp/fuse-smoke-current/repo-mnt/public/file.txt
hello from repo fixture
[exit 0]
$ ls .tmp/fuse-smoke-current/repo-mnt
public
[exit 0]
$ stat .tmp/fuse-smoke-current/repo-mnt/secret
stat: cannot statx '.tmp/fuse-smoke-current/repo-mnt/secret': No such file or directory
[exit 1]
$ touch .tmp/fuse-smoke-current/repo-mnt/public/new-file
touch: cannot touch '.tmp/fuse-smoke-current/repo-mnt/public/new-file': Read-only file system
[exit 1]
$ fusermount3 -u .tmp/fuse-smoke-current/repo-mnt
[exit 0]
screenfs log:
$ cat .tmp/fuse-smoke-current/repo-screenfs.log
mounting screenfs: source=.tmp/fuse-smoke-current/repo-src mount=.tmp/fuse-smoke-current/repo-mnt readonly=true readonly-rule-policy=compiled hide-policy=compiled io_uring=required
FUSE_OVER_IO_URING negotiation is mandatory; session startup fails without it.
[exit 0]
```

## Whole-root readonly hidden smoke and chroot execution
```text
$ target/debug/screenfs / .tmp/fuse-smoke-current/root-mnt --readonly --hide /home/spi-ca/.ssh
mount active=1
[exit 0]
$ stat .tmp/fuse-smoke-current/root-mnt/bin/bash
  File: .tmp/fuse-smoke-current/root-mnt/bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,139	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 2026-06-10 04:59:30.407418592 +0900
Modify: 2025-12-11 07:02:55.000000000 +0900
Change: 2026-03-07 13:25:52.295336313 +0900
 Birth: -
[exit 0]
$ ls .tmp/fuse-smoke-current/root-mnt/usr
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
$ stat .tmp/fuse-smoke-current/root-mnt/home/spi-ca/.ssh
stat: cannot statx '.tmp/fuse-smoke-current/root-mnt/home/spi-ca/.ssh': No such file or directory
[exit 1]
$ touch .tmp/fuse-smoke-current/root-mnt/tmp/screenfs-write-check
touch: cannot touch '.tmp/fuse-smoke-current/root-mnt/tmp/screenfs-write-check': Read-only file system
[exit 1]
$ .tmp/fuse-smoke-current/root-mnt/bin/true
[exit 0]
$ .tmp/fuse-smoke-current/root-mnt/bin/bash --noprofile --norc -lc 'echo direct-bash-ok'
direct-bash-ok
[exit 0]
$ unshare -UrR .tmp/fuse-smoke-current/root-mnt /bin/true
[exit 0]
$ unshare -UrR .tmp/fuse-smoke-current/root-mnt /bin/bash --noprofile --norc -lc 'echo chroot-bash-ok; test -x /bin/bash; stat /bin/bash'
chroot-bash-ok
  File: /bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,139	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (65534/  nobody)   Gid: (65534/  nobody)
Access: 2026-06-10 04:59:30.407418592 +0900
Modify: 2025-12-11 07:02:55.000000000 +0900
Change: 2026-03-07 13:25:52.295336313 +0900
 Birth: -
[exit 0]
$ fusermount3 -u .tmp/fuse-smoke-current/root-mnt
[exit 0]
screenfs log:
$ cat .tmp/fuse-smoke-current/root-screenfs.log
mounting screenfs: source=/ mount=.tmp/fuse-smoke-current/root-mnt readonly=true readonly-rule-policy=compiled hide-policy=compiled io_uring=required
FUSE_OVER_IO_URING negotiation is mandatory; session startup fails without it.
[exit 0]
```

## Current conclusions

- Repo-local fixture mount preserved hidden `ENOENT` and historical whole-mount readonly `EROFS` behavior.
- Whole-root mount exposed `/bin`, `/usr`, and executable entrypoints while hidden `/home/spi-ca/.ssh` stayed `ENOENT`.
- `unshare -UrR` entered the whole-root mount and executed `/bin/true` and `/bin/bash` successfully. Device-node behavior such as `/dev/null` still depends on the FUSE mount's `nodev` option and any supervisor/namespace layer that augments it.
