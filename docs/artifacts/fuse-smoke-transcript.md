# FUSE smoke transcript

This artifact is a command transcript captured from the root pi session, not from isolated subagents. It records runtime FUSE state, repo-local smoke, whole-root smoke, and `unshare -UrR` chroot execution.

Startup log lines shown below are captured historical output from that run. Current binaries now print `readonly-rule-policy=compiled` in the startup line, so the transcript log wording should not be reused as fresh evidence for the current startup-string contract.

- Captured: 2026-06-10T09:57:06+09:00
- Working directory: /home/spi-ca/Codebase/holefs
- Binary: target/debug/holefs
- Kernel config evidence previously verified from /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64: CONFIG_IO_URING=y, CONFIG_FUSE_FS=m, CONFIG_FUSE_PASSTHROUGH=y, CONFIG_FUSE_IO_URING=y.

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
   Compiling holefs v0.1.0 (/home/spi-ca/Codebase/holefs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
[exit 0]
```

## Repo-local readonly hidden smoke
```text
$ target/debug/holefs .tmp/fuse-smoke-current/repo-src .tmp/fuse-smoke-current/repo-mnt --readonly --hide /secret
mount active=1
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
holefs log:
$ cat .tmp/fuse-smoke-current/repo-holefs.log
mounting holefs: source=.tmp/fuse-smoke-current/repo-src mount=.tmp/fuse-smoke-current/repo-mnt readonly=true hide-policy=compiled io_uring=required
FUSE_OVER_IO_URING negotiation is mandatory; session startup fails without it.
[exit 0]
```

## Whole-root readonly hidden smoke and chroot execution
```text
$ target/debug/holefs / .tmp/fuse-smoke-current/root-mnt --readonly --hide /home/spi-ca/.ssh
mount active=1
$ stat .tmp/fuse-smoke-current/root-mnt/bin/bash
  File: .tmp/fuse-smoke-current/root-mnt/bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,77	Inode: 5           Links: 1
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
$ touch .tmp/fuse-smoke-current/root-mnt/tmp/holefs-write-check
touch: cannot touch '.tmp/fuse-smoke-current/root-mnt/tmp/holefs-write-check': Read-only file system
[exit 1]
$ .tmp/fuse-smoke-current/root-mnt/bin/true
[exit 0]
$ .tmp/fuse-smoke-current/root-mnt/bin/bash --noprofile --norc -lc echo direct-bash-ok
direct-bash-ok
[exit 0]
$ unshare -UrR .tmp/fuse-smoke-current/root-mnt /bin/true
[exit 0]
$ unshare -UrR .tmp/fuse-smoke-current/root-mnt /bin/bash --noprofile --norc -lc echo chroot-bash-ok; test -x /bin/bash; stat /bin/bash >/dev/null
chroot-bash-ok
/bin/bash: line 1: /dev/null: Permission denied
[exit 1]
$ fusermount3 -u .tmp/fuse-smoke-current/root-mnt
[exit 0]
holefs log:
$ cat .tmp/fuse-smoke-current/root-holefs.log
mounting holefs: source=/ mount=.tmp/fuse-smoke-current/root-mnt readonly=true hide-policy=compiled io_uring=required
FUSE_OVER_IO_URING negotiation is mandatory; session startup fails without it.
[exit 0]
```

## Corrected chroot bash smoke without /dev/null redirection

The previous bash smoke proved command entry but failed because the check redirected to `/dev/null`, and the FUSE mount is mounted with `nodev`. Re-running on a fresh live whole-root mount without `/dev/null` redirection succeeded:

```text
$ target/debug/holefs / .tmp/chroot-bash-ok-1781053049830227035-mnt --readonly --hide /home/spi-ca/.ssh
mount active=1
$ unshare -UrR .tmp/chroot-bash-ok-1781053049830227035-mnt /bin/bash --noprofile --norc -lc 'echo chroot-bash-ok; test -x /bin/bash; stat /bin/bash'
chroot-bash-ok
  File: /bin/bash
[exit 0]
$ fusermount3 -u .tmp/chroot-bash-ok-1781053049830227035-mnt
[exit 0]
```

Conclusion: `unshare -UrR` can enter the holefs whole-root mount and execute `/bin/true` and `/bin/bash`. Device-node behavior such as `/dev/null` remains constrained by the FUSE mount's `nodev` option and should be provided by the supervisor/namespace layer when needed.
