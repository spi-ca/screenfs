# ScreenFS 아키텍처 개요

이 문서는 `README.md`, [`requirements.md`](requirements.md), [`design.md`](design.md)의 목표 계약을 코드 구조와 연결해 읽기 위한 요약이다. Current evidence는 [`operations.md`](operations.md)를 따른다.

## 1. System context

ScreenFS는 `source_root`를 backing tree로 삼아 FUSE mount를 만들고, 그 mount를 sandbox/chroot 같은 whole-root consumer가 읽는다.

```text
host filesystem (/)
  -> ScreenFS FUSE mount (/tmp/screenfs-root 예시)
  -> whole-root consumer (sandbox/chroot 등)
```

![ScreenFS system context](diagrams/system-context.svg)

정책 모델은 두 축이다.

- **visibility**: hidden / bridge-visible / visible
- **mutability**: readonly / writable

hidden `ENOENT`가 mutability `EROFS`보다 먼저 적용된다. mount-level `ro`만으로는 이 selective policy를 대체할 수 없다.

## 2. Runtime boundaries

- 기본 실행 전제는 non-root이며, effective uid 0 실행은 mount 전에 거부한다.
- mount는 `fusermount3`, FUSE3, `FUSE_OVER_IO_URING` 협상 성공 기준이다.
- `FUSE_OVER_IO_URING`은 FUSE request/reply transport 요구이며 backing filesystem 전체를 host-side `io_uring`로 전환한다는 뜻이 아니다.
- 기본 접근 모델은 mount owner와 동일 host uid다. `allow_other`와 chroot/user namespace 구성은 상위 supervisor 책임이다.
- `/proc`, `/sys`, `/dev`, `/run`의 native semantics 재현도 ScreenFS 단독 책임이 아니다.

## 3. Module architecture

![ScreenFS module architecture](diagrams/module-architecture.svg)

Current module responsibilities:

| Module | Responsibility |
| --- | --- |
| `src/main.rs` | mount option 구성, `Session::run(ScreenFs::new(cfg))` 진입 |
| `src/cli.rs` | launch input parsing, CLI/config override validation, help/fail-fast surface |
| `src/config.rs` | `RuntimeConfig`, internal mount-root hidden rule, policy source/precedence, matcher compilation |
| `src/path.rs` | lexical virtual path normalization, source-root rebasing, symlink target lexical resolution |
| `src/matcher.rs` + `src/matcher/*` | shared rule grammar, descriptor/specificity/containment, candidate index |
| `src/errors.rs` | hidden `ENOENT`, readonly `EROFS`, host errno preservation |
| `src/fs.rs` | FUSE operation orchestrator |
| `src/fs/state.rs` | inode/path map, refs, file/dir handles, directory cookie state |
| `src/fs/guards.rs` | visibility/mutability guards, symlink target checks, mutation coordinate checks |
| `src/fs/backing.rs` | source-root confinement, fd/dirfd-relative host filesystem delegation |

## 4. Request decision flow

![ScreenFS request decision flow](diagrams/request-decision-flow.svg)

1. FUSE request에서 virtual path를 얻는다.
2. lexical normalization을 수행한다.
3. internal mount-root recursion exclusion을 적용한다.
4. visibility를 평가한다.
5. hidden이면 `ENOENT` 또는 listing omission으로 끝낸다.
6. bridge-visible이면 traverse/list만 허용하고 mutation은 `EROFS`다.
7. symlink가 관여하면 resolved virtual target이 fully visible인지 point-of-use에서 확인한다.
8. fully visible write coordinate에 mutability를 적용한다.
9. 허용되면 confined host operation으로 위임한다.

Visibility와 mutability 축 상세 흐름:

![ScreenFS visibility axis](diagrams/visibility-axis.svg)

![ScreenFS mutability axis](diagrams/mutability-axis.svg)

## 5. Path and rule architecture

![ScreenFS path resolution and confinement](diagrams/path-resolution.svg)

- Matching 기준은 host canonical path가 아니라 lexical virtual absolute path다.
- `.`와 중복 `/`는 제거하고, `..`는 virtual `/` 위로 올라가지 못한다.
- relative path, `./...`, `~/...`, bare slashless glob은 launch cwd/HOME을 host path로 해석한 뒤 `source_root` 내부일 때만 virtual prefix로 rebase한다.
- shared matcher family는 exact/subtree, direct-child glob, recursive non-visible glob, recursive literal non-visible subtree로 나뉜다.
- current `visibility.visible`은 exact/subtree와 direct-child anchor bridge만 허용한다. recursive visible glob/shorthand는 fail-fast다.
- `visibility.hidden`, `mutability.readonly`, `mutability.writable`은 recursive family와 recursive literal directory shorthand를 사용할 수 있다.

## 6. Directory and state model

- Directory listing은 FUSE `size` budget에 맞는 bounded page를 반환한다.
- `readdirplus` lookup ref는 실제 반환 page의 child에만 증가시킨다.
- full-directory child attr/inode snapshot cache나 stable listing result cache는 current contract가 아니다.
- State는 inode/path identity, refcount, handle table, directory cookie, mutation invalidation을 한 consistency domain으로 다룬다.
- current lock stance는 single `RwLock<State>`다. lock split은 contention evidence와 lock-order design 없이 하지 않는다.
- Host I/O와 blocking sync syscall은 state lock 밖에서 수행한다.

## 7. Confinement and TOCTOU stance

- Host access는 `source_root` 밖 escape를 허용하지 않는다.
- 가능한 operation은 fd 또는 dirfd-relative syscall로 위임한다.
- Mutation 직전 opened parent dirfd가 요청된 virtual parent path에 남아 있는지 best-effort로 재확인한다.
- 외부 same-UID actor가 validation 이후 pin된 inode를 이동/삭제하면 Linux/POSIX fd lifetime semantics를 따른다.
- 이 stance는 path 재해석 race를 줄이지만, validation과 syscall 사이 current virtual path membership을 원자적으로 보장하지는 않는다.

## 8. Verification questions

아키텍처 변경 시 최소 질문:

- hidden entry가 listing에서 빠지고 직접 접근은 `ENOENT`인가?
- hidden/non-fully-visible symlink target이 mutability보다 먼저 `ENOENT`인가?
- bridge-visible ancestor가 traverse/list 전용이고 mutation은 `EROFS`인가?
- current `visibility.visible` subset이 recursive discovery 없이 유지되는가?
- direct-child visible rule이 immediate child만 평가하고 hidden sibling을 노출하지 않는가?
- `readdir`/`readdirplus`가 bounded page와 stable resume cookie를 유지하는가?
- shared matcher가 exact/subtree, direct-child, recursive non-visible, recursive literal non-visible family를 분리해 다루는가?
- state lock이 host I/O 또는 blocking syscall 구간에 잡히지 않는가?
- `source_root` confinement와 fd/dirfd-relative delegation이 유지되는가?

## 9. Diagram contract

다이어그램 원본은 `docs/diagrams/*.mmd`다. `*.mmd` 또는 Mermaid config를 바꾸면 [`diagrams/README.md`](diagrams/README.md)에 따라 대응 `*.svg`, `*.png`를 함께 재생성하고 PNG에는 `--scale 2`를 적용한다.
