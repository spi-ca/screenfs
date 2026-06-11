# Current four-family glob smoke transcript

- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: /home/spi-ca/Codebase/screenfs/target/debug/screenfs
- Temp root: /tmp/screenfs-four-glob-smoke-1dxeLH
- Source root: /tmp/screenfs-four-glob-smoke-1dxeLH/src
- Launch cwd for relative direct-child session: /tmp/screenfs-four-glob-smoke-1dxeLH/src/workspace/app
- Captured: 2026-06-12T08:44:20+09:00
- Purpose: live repo-local evidence for canonical glob families `**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt` under `visibility.default=hidden` with readonly mutability.

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
/src/workspace/app/fixtures
/src/workspace/app/fixtures/bundle.pem
/src/workspace/app/fixtures/bundle.pem/child
/src/workspace/app/fixtures/local.pem
/src/workspace/app/fixtures/nested
/src/workspace/app/fixtures/nested/deep.pem
```

## **/*.pem (prefixless-recursive)

$ (cd /tmp/screenfs-four-glob-smoke-1dxeLH/src && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-1dxeLH/src /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive --config /tmp/screenfs-four-glob-smoke-1dxeLH/config-prefixless-recursive.yaml)
mounted pid=249344 startup_ms=393
$ cat /tmp/screenfs-four-glob-smoke-1dxeLH/logs/prefixless-recursive.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-1dxeLH/src mount=/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive' -maxdepth 5 -print | sed 's#/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive#/mnt#' | sort
/mnt
/mnt/root.pem
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/fixtures
/mnt/workspace/app/fixtures/bundle.pem
/mnt/workspace/app/fixtures/bundle.pem/child
/mnt/workspace/app/fixtures/local.pem
/mnt/workspace/app/fixtures/nested
/mnt/workspace/app/fixtures/nested/deep.pem
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive/root.pem'
root-pem
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive/workspace/app/fixtures/nested/deep.pem'
deep-pem
$ stat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive/a/cert.txt'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive/a/cert.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-prefixless-recursive

## ./fixtures/*.pem (cwd-direct-fixtures)

$ (cd /tmp/screenfs-four-glob-smoke-1dxeLH/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-1dxeLH/src /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures --config /tmp/screenfs-four-glob-smoke-1dxeLH/config-cwd-direct-fixtures.yaml)
mounted pid=249378 startup_ms=372
$ cat /tmp/screenfs-four-glob-smoke-1dxeLH/logs/cwd-direct-fixtures.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-1dxeLH/src mount=/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures' -maxdepth 7 -print | sed 's#/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures#/mnt#' | sort
/mnt
/mnt/workspace
/mnt/workspace/app
/mnt/workspace/app/fixtures
/mnt/workspace/app/fixtures/bundle.pem
/mnt/workspace/app/fixtures/bundle.pem/child
/mnt/workspace/app/fixtures/local.pem
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures/workspace/app/fixtures/local.pem'
workspace-pem
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures/workspace/app/fixtures/bundle.pem/child'
bundle-child
$ stat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures/workspace/app/fixtures/nested/deep.pem'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures/workspace/app/fixtures/nested/deep.pem': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-cwd-direct-fixtures

## /a/*.txt (absolute-direct-a)

$ (cd /tmp/screenfs-four-glob-smoke-1dxeLH/src && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-1dxeLH/src /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a --config /tmp/screenfs-four-glob-smoke-1dxeLH/config-absolute-direct-a.yaml)
mounted pid=249411 startup_ms=378
$ cat /tmp/screenfs-four-glob-smoke-1dxeLH/logs/absolute-direct-a.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-1dxeLH/src mount=/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a' -maxdepth 5 -print | sed 's#/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a#/mnt#' | sort
/mnt
/mnt/a
/mnt/a/cert.txt
/mnt/a/dir.txt
/mnt/a/dir.txt/child
/mnt/a/file.txt
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a/a/file.txt'
a-file
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a/a/dir.txt/child'
a-dir-child
$ stat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a/a/b/file.txt'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a/a/b/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-direct-a

## /a/**/*.txt (absolute-recursive-a)

$ (cd /tmp/screenfs-four-glob-smoke-1dxeLH/src && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-four-glob-smoke-1dxeLH/src /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a --config /tmp/screenfs-four-glob-smoke-1dxeLH/config-absolute-recursive-a.yaml)
mounted pid=249444 startup_ms=340
$ cat /tmp/screenfs-four-glob-smoke-1dxeLH/logs/absolute-recursive-a.log
mounting screenfs: source=/tmp/screenfs-four-glob-smoke-1dxeLH/src mount=/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=1 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=0 io_uring=required

$ find '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a' -maxdepth 5 -print | sed 's#/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a#/mnt#' | sort
/mnt
/mnt/a
/mnt/a/b
/mnt/a/b/file.txt
/mnt/a/cert.txt
/mnt/a/dir.txt
/mnt/a/dir.txt/child
/mnt/a/file.txt
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a/a/file.txt'
a-file
$ cat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a/a/b/file.txt'
nested-a-file
$ stat '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a/b/file.txt'
stat: cannot statx '/tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a/b/file.txt': No such file or directory
[exit 1]
$ fusermount3 -u /tmp/screenfs-four-glob-smoke-1dxeLH/mnt-absolute-recursive-a

