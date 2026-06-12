# Current four-family glob smoke transcript

- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: /home/spi-ca/Codebase/screenfs/target/debug/screenfs
- `fusermount3 --version`: `fusermount3 version: 3.18.2`
- Temp root: /tmp/screenfs-four-glob-smoke-3XXVXk
- Source root: /tmp/screenfs-four-glob-smoke-3XXVXk/src
- Launch cwd for prefixless/relative sessions: /tmp/screenfs-four-glob-smoke-3XXVXk/src/workspace/app
- Captured: 2026-06-12T09:45:09+09:00
- Purpose: live repo-local evidence for goal 38ca1993 canonical glob families `**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt` under `visibility.default=hidden` with readonly mutability.
- Coverage note: this transcript, together with the 103-test full source suite, shows that prefixless recursive `**/*.pem` is a cwd-anchored shorthand distinct from explicit root-anchor `/**/*.pem`, while `./fixtures/**/*.pem`, `/a/*.txt`, and `/a/**/*.txt` keep their documented families.
- Performance note: per-session `startup_ms`, `VmRSS`, and fd count are smoke evidence only, not a formal benchmark. They demonstrate the configured rule family starts from the intended anchor on this fixture.

## Source fixture

```text
/src
/src/a
/src/a/b
/src/a/b/file.txt
/src/a/cert.txt
/src/a/dir.txt
/src/a/dir.txt/child
/src/a/file.txt
/src/b
/src/b/file.txt
/src/root.pem
/src/workspace
/src/workspace/app
/src/workspace/app/cwd.pem
/src/workspace/app/fixtures
/src/workspace/app/fixtures/bundle.pem
/src/workspace/app/fixtures/bundle.pem/child
/src/workspace/app/fixtures/local.pem
/src/workspace/app/fixtures/nested
/src/workspace/app/fixtures/nested/deep.pem
/src/workspace/peer
/src/workspace/peer/peer.pem
```

## **/*.pem (prefixless-recursive-cwd)

$ (cd /tmp/screenfs-four-glob-smoke-3XXVXk/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-3XXVXk/src /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd --config /tmp/screenfs-four-glob-smoke-3XXVXk/config-prefixless-recursive-cwd.yaml)
mounted pid=270049 startup_ms=3 rss=2292 kB fd_count=3
$ cat /tmp/screenfs-four-glob-smoke-3XXVXk/logs/prefixless-recursive-cwd.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-3XXVXk/src mount=/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd' -maxdepth 6 -print | sed 's#/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd#/mnt#' | sort
/mnt
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/cwd.pem
/mnt/workspace/app/fixtures
/mnt/workspace/app/fixtures/bundle.pem
/mnt/workspace/app/fixtures/bundle.pem/child
/mnt/workspace/app/fixtures/local.pem
/mnt/workspace/app/fixtures/nested
/mnt/workspace/app/fixtures/nested/deep.pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/workspace/app/cwd.pem'
cwd-pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/workspace/app/fixtures/nested/deep.pem'
deep-pem
$ stat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/root.pem'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/root.pem': No such file or directory
[exit 1]
$ stat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/workspace/peer/peer.pem'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd/workspace/peer/peer.pem': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-prefixless-recursive-cwd

## /**/*.pem (explicit-root-recursive)

$ (cd /tmp/screenfs-four-glob-smoke-3XXVXk/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-3XXVXk/src /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive --config /tmp/screenfs-four-glob-smoke-3XXVXk/config-explicit-root-recursive.yaml)
mounted pid=270090 startup_ms=341 rss=2324 kB fd_count=3
$ cat /tmp/screenfs-four-glob-smoke-3XXVXk/logs/explicit-root-recursive.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-3XXVXk/src mount=/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive' -maxdepth 4 -print | sed 's#/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive#/mnt#' | sort
/mnt
/mnt/root.pem
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/cwd.pem
/mnt/workspace/app/fixtures
/mnt/workspace/app/fixtures/bundle.pem
/mnt/workspace/app/fixtures/local.pem
/mnt/workspace/app/fixtures/nested
/mnt/workspace/peer
/mnt/workspace/peer/peer.pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive/root.pem'
root-pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive/workspace/peer/peer.pem'
peer-pem
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-explicit-root-recursive

## ./fixtures/**/*.pem (cwd-recursive-fixtures)

$ (cd /tmp/screenfs-four-glob-smoke-3XXVXk/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-3XXVXk/src /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures --config /tmp/screenfs-four-glob-smoke-3XXVXk/config-cwd-recursive-fixtures.yaml)
mounted pid=270137 startup_ms=337 rss=2324 kB fd_count=3
$ cat /tmp/screenfs-four-glob-smoke-3XXVXk/logs/cwd-recursive-fixtures.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-3XXVXk/src mount=/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures' -maxdepth 7 -print | sed 's#/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures#/mnt#' | sort
/mnt
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/fixtures
/mnt/workspace/app/fixtures/bundle.pem
/mnt/workspace/app/fixtures/bundle.pem/child
/mnt/workspace/app/fixtures/local.pem
/mnt/workspace/app/fixtures/nested
/mnt/workspace/app/fixtures/nested/deep.pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures/workspace/app/fixtures/local.pem'
workspace-pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures/workspace/app/fixtures/nested/deep.pem'
deep-pem
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures/workspace/app/fixtures/bundle.pem/child'
bundle-child
$ stat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures/workspace/app/cwd.pem'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures/workspace/app/cwd.pem': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-cwd-recursive-fixtures

## /a/*.txt (absolute-direct-a)

$ (cd /tmp/screenfs-four-glob-smoke-3XXVXk/src && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-3XXVXk/src /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a --config /tmp/screenfs-four-glob-smoke-3XXVXk/config-absolute-direct-a.yaml)
mounted pid=270177 startup_ms=336 rss=2328 kB fd_count=3
$ cat /tmp/screenfs-four-glob-smoke-3XXVXk/logs/absolute-direct-a.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-3XXVXk/src mount=/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a' -maxdepth 5 -print | sed 's#/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a#/mnt#' | sort
/mnt
/mnt/a
/mnt/a/cert.txt
/mnt/a/dir.txt
/mnt/a/dir.txt/child
/mnt/a/file.txt
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a/a/file.txt'
a-file
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a/a/dir.txt/child'
a-dir-child
$ stat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a/a/b/file.txt'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a/a/b/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-direct-a

## /a/**/*.txt (absolute-recursive-a)

$ (cd /tmp/screenfs-four-glob-smoke-3XXVXk/src && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-3XXVXk/src /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a --config /tmp/screenfs-four-glob-smoke-3XXVXk/config-absolute-recursive-a.yaml)
mounted pid=270219 startup_ms=333 rss=2332 kB fd_count=3
$ cat /tmp/screenfs-four-glob-smoke-3XXVXk/logs/absolute-recursive-a.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-3XXVXk/src mount=/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a' -maxdepth 5 -print | sed 's#/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a#/mnt#' | sort
/mnt
/mnt/a
/mnt/a/b
/mnt/a/b/file.txt
/mnt/a/cert.txt
/mnt/a/dir.txt
/mnt/a/dir.txt/child
/mnt/a/file.txt
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a/a/file.txt'
a-file
$ cat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a/a/b/file.txt'
nested-a-file
$ stat '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a/b/file.txt'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a/b/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-3XXVXk/mnt-absolute-recursive-a

