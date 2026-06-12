# Current compatibility path-pattern smoke transcript

- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: /home/spi-ca/Codebase/screenfs/target/debug/screenfs
- fusermount3 --version: fusermount3 version: 3.18.2
- Temp root: /tmp/screenfs-compat-glob-smoke-Sov6Zk
- Source root: /tmp/screenfs-compat-glob-smoke-Sov6Zk/src
- Launch cwd for relative sessions: /tmp/screenfs-compat-glob-smoke-Sov6Zk/src/workspace/app
- HOME for tilde sessions: /tmp/screenfs-compat-glob-smoke-Sov6Zk/src/home/tester
- Captured: 2026-06-12T10:49:22+09:00
- Purpose: live repo-local evidence for goal 612c03c6 compatibility forms: anchored direct-child wildcard-all (`/dir/*`, `./rel/*`, `~/homeglob/*`) and trailing subtree shorthand (`/dir/**`, `./rel/**`, `~/homesub/**`).
- Performance note: per-session `startup_ms`, `VmRSS`, and fd count are smoke evidence only, not a formal benchmark.

## Source fixture

```text
/src
/src/dir
/src/dir2
/src/dir2/file.txt
/src/dir/file.txt
/src/dir/subdir
/src/dir/subdir/deep.txt
/src/home
/src/home/tester
/src/home/tester/homeglob
/src/home/tester/homeglob/child
/src/home/tester/homeglob/child/deep.txt
/src/home/tester/homeglob/file.txt
/src/home/tester/homesub
/src/home/tester/homesub/deep
/src/home/tester/homesub/deep/file.txt
/src/workspace
/src/workspace/app
/src/workspace/app/rel
/src/workspace/app/rel/child
/src/workspace/app/rel/file.txt
/src/workspace/app/rel/subdir
/src/workspace/app/rel/subdir/deep.txt
```

## Anchored direct-child wildcard-all

$ (cd /tmp/screenfs-compat-glob-smoke-Sov6Zk/src/workspace/app && HOME=/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/home/tester /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-compat-glob-smoke-Sov6Zk/src /tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all --config /tmp/screenfs-compat-glob-smoke-Sov6Zk/config-wildcard-all.yaml)
mounted pid=291355 startup_ms=335 VmRSS=2268 kB fd_count=3
$ cat /tmp/screenfs-compat-glob-smoke-Sov6Zk/logs/wildcard-all.log
mounting screenfs: source=/tmp/screenfs-compat-glob-smoke-Sov6Zk/src mount=/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=3 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all' -maxdepth 6 -print | sed 's#/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all#/mnt#' | sort
/mnt
/mnt/dir
/mnt/dir/file.txt
/mnt/dir/subdir
/mnt/dir/subdir/deep.txt
/mnt/home
/mnt/home/tester
/mnt/home/tester/homeglob
/mnt/home/tester/homeglob/child
/mnt/home/tester/homeglob/child/deep.txt
/mnt/home/tester/homeglob/file.txt
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/rel
/mnt/workspace/app/rel/child
/mnt/workspace/app/rel/file.txt
/mnt/workspace/app/rel/subdir
/mnt/workspace/app/rel/subdir/deep.txt
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/dir/file.txt'
root-dir-child
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/dir/subdir/deep.txt'
root-dir-deep
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/workspace/app/rel/file.txt'
rel-child
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/workspace/app/rel/subdir/deep.txt'
rel-deep
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/home/tester/homeglob/file.txt'
home-child
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/home/tester/homeglob/child/deep.txt'
home-deep
$ stat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/dir2/file.txt'
stat: cannot statx '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all/dir2/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-wildcard-all

## Trailing subtree shorthand

$ (cd /tmp/screenfs-compat-glob-smoke-Sov6Zk/src/workspace/app && HOME=/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/home/tester /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-compat-glob-smoke-Sov6Zk/src /tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand --config /tmp/screenfs-compat-glob-smoke-Sov6Zk/config-subtree-shorthand.yaml)
mounted pid=291398 startup_ms=331 VmRSS=2292 kB fd_count=3
$ cat /tmp/screenfs-compat-glob-smoke-Sov6Zk/logs/subtree-shorthand.log
mounting screenfs: source=/tmp/screenfs-compat-glob-smoke-Sov6Zk/src mount=/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=3 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand' -maxdepth 6 -print | sed 's#/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand#/mnt#' | sort
/mnt
/mnt/dir
/mnt/dir/file.txt
/mnt/dir/subdir
/mnt/dir/subdir/deep.txt
/mnt/home
/mnt/home/tester
/mnt/home/tester/homesub
/mnt/home/tester/homesub/deep
/mnt/home/tester/homesub/deep/file.txt
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/rel
/mnt/workspace/app/rel/child
/mnt/workspace/app/rel/file.txt
/mnt/workspace/app/rel/subdir
/mnt/workspace/app/rel/subdir/deep.txt
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/dir/file.txt'
root-dir-child
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/dir/subdir/deep.txt'
root-dir-deep
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/workspace/app/rel/file.txt'
rel-child
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/workspace/app/rel/subdir/deep.txt'
rel-deep
$ cat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/home/tester/homesub/deep/file.txt'
homesub
$ stat '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/dir2/file.txt'
stat: cannot statx '/tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand/dir2/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-compat-glob-smoke-Sov6Zk/mnt-subtree-shorthand

## Fail-fast checks

$ (cd '/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/workspace/app' && HOME='/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/home/tester' '/home/spi-ca/Codebase/screenfs/target/debug/screenfs' '/tmp/screenfs-compat-glob-smoke-Sov6Zk/src' '/tmp/screenfs-compat-glob-smoke-Sov6Zk/bad-mnt-star' --hidden '*')
Error: Custom { kind: InvalidInput, error: "invalid hidden pattern: unsupported glob: *" }
[exit 1]
$ (cd '/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/workspace/app' && HOME='/tmp/screenfs-compat-glob-smoke-Sov6Zk/src/home/tester' '/home/spi-ca/Codebase/screenfs/target/debug/screenfs' '/tmp/screenfs-compat-glob-smoke-Sov6Zk/src' '/tmp/screenfs-compat-glob-smoke-Sov6Zk/bad-mnt-recursive-star' --hidden '**/*')
Error: Custom { kind: InvalidInput, error: "invalid hidden pattern: unsupported glob: **/*" }
[exit 1]
