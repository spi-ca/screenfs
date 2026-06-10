# Whole-root whole-mount readonly smoke transcript

- Captured: 2026-06-11T00:38:31+09:00
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

## Whole-root readonly-root-allowwrite with empty allow_write
```text
$ /home/spi-ca/Codebase/screenfs/target/debug/screenfs / /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt --policy-family readonly-root-allowwrite --hide /home/spi-ca/.ssh
mount active=1
$ stat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/bin/bash
  File: /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,77	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (    0/    root)   Gid: (    0/    root)
Access: 2026-06-10 04:59:30.407418592 +0900
[exit 0]
$ ls /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/etc | head
adjtime
alsa
arch-release
arptables.conf
audisp
audit
avahi
bash.bash_logout
bash.bashrc
bindresvport.blacklist
[exit 0]
$ stat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/home/spi-ca/.ssh
stat: cannot statx '/home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/home/spi-ca/.ssh': No such file or directory
[exit 1]
$ touch /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/tmp/whole-root-readonly-blocked
touch: cannot touch '/home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/tmp/whole-root-readonly-blocked': Read-only file system
[exit 1]
$ mkdir /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/tmp/whole-root-readonly-dir
mkdir: cannot create directory ‘/home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt/tmp/whole-root-readonly-dir’: Read-only file system
[exit 1]
$ unshare -UrR /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt /bin/bash --noprofile --norc -lc 'echo chroot-bash-ok; stat /bin/bash; touch /tmp/chroot-blocked'
chroot-bash-ok
  File: /bin/bash
  Size: 1162312   	Blocks: 2272       IO Block: 4096   regular file
Device: 0,77	Inode: 5           Links: 1
Access: (0755/-rwxr-xr-x)  Uid: (65534/  nobody)   Gid: (65534/  nobody)
Access: 2026-06-10 04:59:30.407418592 +0900
Modify: 2025-12-11 07:02:55.000000000 +0900
Change: 2026-03-07 13:25:52.295336313 +0900
 Birth: -
touch: cannot touch '/tmp/chroot-blocked': Read-only file system
[exit 1]
$ fusermount3 -u /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt
[exit 0]
screenfs log:
$ cat /home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/root.log
mounting screenfs: source=/ mount=/home/spi-ca/Codebase/screenfs/.tmp/whole-root-readonly-4cf54F/mnt mutability-family=readonly-root-allowwrite mutability-source=cli readonly-rules=0 allow-write-rules=0 hide-policy=compiled io_uring=required
[exit 0]
```
