# ScreenFS 설계 문서

이 문서는 ScreenFS의 현재 목표 계약을 구현 관점에서 정리한다. 제품 요구사항은 [`requirements.md`](requirements.md), 코드 경계 요약은 [`architecture.md`](architecture.md), current evidence는 [`operations.md`](operations.md)를 따른다.

## 1. Design goals

- non-root 사용자 권한으로 whole-root FUSE3 view를 제공하고, effective uid 0 실행은 startup error로 거부한다.
- `fusermount3`, `fractal-fuse = 0.4.0`, `FUSE_OVER_IO_URING` 협상 성공을 기준으로 한다.
- 정책 surface는 `visibility`와 `mutability` 두 축만 사용한다.
- hidden `ENOENT`가 mutability `EROFS`보다 항상 먼저 적용된다.
- root-only mount, privileged bind mount, 이름이 남는 masking overlay는 사용하지 않는다.

## 2. Filesystem view model

입력은 `source_root`와 `mount_root`이고 출력은 whole-root consumer가 읽는 virtual root다.

```text
source root: /
mount root:  /tmp/screenfs-root
virtual path /usr/bin/bash -> source path /usr/bin/bash
```

`source_root=/`에서 `mount_root` subtree가 다시 view 안에 보이면 자기참조가 생긴다. 따라서 mount-root source-relative subtree는 internal hidden rule로 취급하고 user policy보다 먼저 적용한다.

## 3. Policy evaluation order

모든 request는 다음 순서를 따른다.

1. inode/name/file handle에서 virtual path를 계산한다.
2. virtual path를 lexical absolute path로 normalize한다.
3. internal mount-root exclusion을 먼저 적용한다.
4. visibility를 `hidden`, `bridge-visible`, `visible` 중 하나로 판정한다.
5. hidden이면 `ENOENT` 또는 listing omission으로 끝낸다.
6. symlink가 관여하면 resolved virtual target이 fully visible인지 point-of-use에서 확인한다.
7. bridge-visible ancestor mutation이면 `EROFS`다.
8. fully visible write-requiring coordinate에만 mutability를 적용한다.
9. 허용되면 confined host filesystem operation으로 위임한다.

핵심 우선순위:

```text
hidden/non-fully-visible ENOENT
  > bridge-visible mutation EROFS
  > mutability readonly EROFS
  > host errno
```

## 4. Visibility design

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []
```

- `default=visible`: hidden rule이 기본 차단이고, 더 구체적인 visible rule이 carve-out이 될 수 있다.
- `default=hidden`: visible rule이 allowlist이고, 더 구체적인 hidden rule이 re-block이 될 수 있다.
- hidden path는 listing에서 빠지고 direct operation은 `ENOENT`다.
- listing 성공, cross-request direct-path memo, symlink decision cache는 later target check 면제 근거가 아니다.

### Bridge-visible ancestor

Bridge-visible은 visible descendant로 도달시키기 위한 ancestor directory 상태다.

허용:

- `lookup`, `getattr`, read/traverse intent `access`
- `opendir`, `readdir`, `readdirplus`

거부:

- create/delete/rename/link/symlink
- write-intent `open`
- `setattr`, xattr mutation, `fallocate`

Bridge-visible listing은 visible child 또는 visible descendant로 이어지는 child만 반환한다. hidden sibling은 노출하지 않는다.

### Current `visibility.visible` subset

| category | 지원 | 구현 제약 |
| --- | --- | --- |
| static subtree bridge | `/dir`, `/dir/**`, `./dir`, `~/dir` | visible target까지 정적 ancestor chain만 사용 |
| direct-child anchor bridge | `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형 | normalized anchor ancestor만 bridge-visible candidate; current directory/parent 기준 immediate child만 평가 |
| recursive visible glob/shorthand | `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks`, `/repo/**/.git/hooks` 등 | unsupported/fail-fast; recursive bridge discovery, startup scan, lazy discovery 금지 |

## 5. Mutability design

```yaml
mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

- mutability는 visibility가 fully visible로 증명된 coordinate에만 적용한다.
- `default=writable`: readonly match가 mutation을 `EROFS`로 막고, 더 구체적인 writable이 다시 허용할 수 있다.
- `default=readonly`: writable match가 mutation을 허용하고, 더 구체적인 readonly가 다시 막을 수 있다.
- fast allow/block은 visibility proof를 대체하지 못한다.

Write-requiring coordinate 예:

| Operation | Coordinates |
| --- | --- |
| `create`, `mkdir`, `mknod` | target + parent |
| `unlink`, `rmdir` | removed path + parent |
| write-intent `open`, `write`, `setattr`, xattr mutation, `fallocate` | entry path와 필요 시 resolved target |
| `rename` | source + target + source parent + target parent |
| `link` | source visibility + target + target parent |
| `symlink` | link path + link parent + target visibility checks |
| `copy_file_range` | source visibility + destination path + destination parent |

## 6. Shared rule normalization

Matching은 host canonical path가 아니라 lexical virtual path 기준이다. `visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`는 같은 parser/normalization을 공유한다. 단, `visibility.visible`은 discovery-free subset만 current surface로 허용한다.

Supported family:

- **exact/subtree**: `/dir`, `/dir/**`, `./dir`, `~/dir`; same-anchor `/dir/**`는 `/dir`와 같은 descriptor다.
- **direct-child glob**: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare `*.pem`, `.env.*`, `id_*`; bare slashless form은 launch cwd 기준 `./<pattern>` direct-child shorthand다.
- **recursive non-visible glob**: `**/*.pem`, `/**/*.pem`, `./fixtures/**/*.pem`, `/a/**/*.txt`; prefixless form은 cwd anchor의 recursive shorthand다.
- **recursive literal non-visible subtree**: `**/.git`, `**/.git/hooks`, `/repo/**/.git/hooks`, `~/**/aaa/hook`; `**/<literal-dir>` 또는 `<prefix>/**/<literal-tail>`만 허용하고 canonical trailing `/**` descriptor와 동등하게 compile한다.

Conflict and fail-fast:

- same-polarity duplicate는 idempotent다.
- 같은 축의 opposite-polarity same-specificity conflict는 invalid configuration이다.
- overlap/containment를 증명할 수 없는 broader glob 조합은 fail-fast다.
- `HOME` 없음, source-root 밖 cwd/expanded path, `~user`, prefix 내부 wildcard, unanchored `**/*`, `*`, `a*b`, `*secret*`, multi-recursive descendant form은 fail-fast다.

## 7. State, cache, and concurrency

- inode/path map, lookup/open refcount, file/dir handle table, directory cookie state는 하나의 consistency domain이다.
- current stance는 `RwLock<State>` 단일 domain이다. per-table lock split은 별도 design/evidence 전까지 하지 않는다.
- state lock을 host filesystem I/O나 blocking syscall 구간에 잡고 있지 않는다.
- `readdirplus` lookup ref 증가는 실제 반환 page child에만, returned-page commit과 같은 write-lock transaction에서 수행한다.
- request-local resolved-target reuse는 허용하지만 cross-request symlink decision cache나 stable listing snapshot cache는 current contract가 아니다.

## 8. FUSE and host delegation boundary

- `FUSE_OVER_IO_URING` 요구는 FUSE transport에 한정한다.
- backing filesystem access는 `source_root` confinement와 fd/dirfd-relative syscall을 우선한다.
- Raw symlink target이 lexical visibility 기준으로 fully visible하지만 host resolution에서 `source_root` 밖으로 escape하면 `readlink`는 raw target을 반환할 수 있고, dereference/open/access는 confinement 단계에서 `ENOENT`로 실패한다. 이 정보노출 경계는 current contract로 문서화한다.
- open target과 mutation parent는 fd로 pin하고, mutation 직전 opened parent dirfd가 요청된 virtual parent path에 남아 있는지 best-effort로 재확인한다.
- 외부 same-UID actor가 validation 이후 이미 pin된 object를 rename/unlink하면 POSIX fd lifetime semantics를 따른다. ScreenFS는 current virtual path membership을 원자적으로 보장하지 않는다.
- `flush`/`fsync`/`release(flush)` offload는 state lock 밖 blocking pool로 sync syscall 실행 위치만 옮기는 low-risk cleanup이다. `read`/`write`는 `FileExt::read_at`/`write_at` 경로를 유지한다.

## 9. Validation strategy

변경은 다음 층에서 검증한다.

- Unit: path normalization, matcher conflict/dedup, hidden-before-EROFS, affected-coordinate classification
- Integration without mount: hidden/listing, bridge-visible traversal, symlink target visibility, mutability overrides, config/CLI precedence
- FUSE smoke: real listing/direct access/mutation behavior, mount-root exclusion, whole-root/chroot baseline
- System smoke: `/dev/fuse`, `fusermount3`, FUSE/io_uring environment, unmount cleanup
- Performance: [`benchmarks.md`](benchmarks.md)의 harness와 claim bar

다이어그램 source of truth는 `docs/diagrams/*.mmd`이고 렌더링 계약은 [`diagrams/README.md`](diagrams/README.md)를 따른다.
