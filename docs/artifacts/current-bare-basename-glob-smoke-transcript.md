# Current cwd-anchored bare slashless glob ScreenFS smoke transcript

- Working directory: /home/spi-ca/Codebase/screenfs
- Binary: target/debug/screenfs
- Temp root: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx
- Launch cwd for successful bare-slashless sessions: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src/workspace/app
- Date: 2026-06-12T04:23:21+09:00

Acceptance summary:
- Bare slashless `*.pem`, `.env.*`, and `id_*` are launch-cwd anchored `./<pattern>` direct-child shorthands.
- Session 1 verifies `visibility.hidden` and `mutability.readonly`: cwd immediate children and matched-child descendants are affected, while nested non-child matches are not.
- Session 2 verifies `visibility.visible` and `mutability.writable`: cwd immediate children are exposed/writable as configured under default-hidden/default-readonly, while nested non-child matches stay hidden.
- Fail-fast checks verify cwd outside `source_root`, still-unsupported wildcard, and `*.pem` vs `./*.pem` same-normalized-specificity conflict. `*.pem` vs `**/*.pem` is not same-specificity conflict; source tests cover containment/override semantics.
- Sandbox relevance: macOS-style cwd-sensitive bare patterns can be passed as bare forms only when launch cwd is the intended anchor; whole-root secret coverage should use explicit recursive/anchored rules such as `**/*.pem`.

```console
$ fusermount3 --version
fusermount3 version: 3.18.2
```

## Session 1: default-visible hidden bare glob and readonly bare glob

Config:
```yaml
visibility:
  default: visible
  hidden:
    - '*.pem'
mutability:
  default: writable
  readonly:
    - '.env.*'
    - 'id_*'
```

```console
$ (cd /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly --config /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/config-hidden-readonly.yaml)
mounted pid=209485
mounting screenfs: source=/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src mount=/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly visibility-default=visible visibility-source=config hidden-rules=1 visible-rules=0 mutability-default=writable mutability-source=config readonly-rules=2 writable-rules=0 io_uring=required
$ stat mnt-hidden-readonly/workspace/app/root.pem
stat: cannot statx '/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly/workspace/app/root.pem': No such file or directory
$ stat mnt-hidden-readonly/workspace/app/nested/cert.pem
  File: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly/workspace/app/nested/cert.pem
  Size: 12        	Blocks: 8          IO Block: 4096   regular file
Device: 0,219	Inode: 5           Links: 1
Access: (0640/-rw-r-----)  Uid: ( 1000/  spi-ca)   Gid: ( 1000/  spi-ca)
Access: 2026-06-12 04:23:21.468282816 +0900
Modify: 2026-06-12 04:23:21.468282816 +0900
Change: 2026-06-12 04:23:21.468282816 +0900
 Birth: -
$ cat mnt-hidden-readonly/other/root.pem
other-cert
$ sh -c printf no > mnt-hidden-readonly/workspace/app/.env.production
sh: line 1: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly/workspace/app/.env.production: Read-only file system
$ sh -c printf no > mnt-hidden-readonly/workspace/app/id_ed25519
sh: line 1: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-hidden-readonly/workspace/app/id_ed25519: Read-only file system
$ sh -c printf ok > mnt-hidden-readonly/workspace/app/nested/.env.production && cat mnt-hidden-readonly/workspace/app/nested/.env.production
ok$ sh -c printf ok > mnt-hidden-readonly/workspace/app/public.txt && cat mnt-hidden-readonly/workspace/app/public.txt
ok$ fusermount3 -u mnt-hidden-readonly
```

## Session 2: default-hidden visible bare glob and writable bare glob carve-out

Config:
```yaml
visibility:
  default: hidden
  visible:
    - '*.pem'
    - '.env.*'
    - 'id_*'
mutability:
  default: readonly
  writable:
    - 'id_*'
```

```console
$ (cd /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-visible-writable --config /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/config-visible-writable.yaml)
mounted pid=209512
mounting screenfs: source=/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src mount=/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-visible-writable visibility-default=hidden visibility-source=config hidden-rules=0 visible-rules=3 mutability-default=readonly mutability-source=config readonly-rules=0 writable-rules=1 io_uring=required
$ find mnt-visible-writable -maxdepth 5 -print | sort
mnt-visible-writable
mnt-visible-writable/workspace
mnt-visible-writable/workspace/app
mnt-visible-writable/workspace/app/.env.production
mnt-visible-writable/workspace/app/id_ed25519
mnt-visible-writable/workspace/app/root.pem
$ cat mnt-visible-writable/workspace/app/root.pem
cwd-cert
$ cat mnt-visible-writable/workspace/app/.env.production
env
$ stat mnt-visible-writable/workspace/app/nested/cert.pem
stat: cannot statx '/tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-visible-writable/workspace/app/nested/cert.pem': No such file or directory
$ sh -c printf no > mnt-visible-writable/workspace/app/root.pem
sh: line 1: /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/mnt-visible-writable/workspace/app/root.pem: Read-only file system
$ sh -c printf ok > mnt-visible-writable/workspace/app/id_ed25519 && cat mnt-visible-writable/workspace/app/id_ed25519
ok$ fusermount3 -u mnt-visible-writable
```

## Fail-fast checks

```console
$ (cd /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/bad-mnt --hidden '*.pem')
Error: Custom { kind: InvalidInput, error: "invalid hidden pattern: rule path resolves outside source_root: ." }
$ (cd /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/bad-mnt --hidden '*secret*')
Error: Custom { kind: InvalidInput, error: "invalid hidden pattern: unsupported glob: *secret*" }
$ (cd /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src/workspace/app && /home/spi-ca/Codebase/screenfs/target/debug/screenfs /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/src /tmp/screenfs-cwd-bare-glob-smoke-wsBxCx/bad-mnt --hidden '*.pem' --visible './*.pem')
Error: Custom { kind: InvalidInput, error: "hidden and visible rules conflict at the same normalized specificity" }
```
