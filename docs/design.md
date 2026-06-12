# ScreenFS 설계 문서

이 문서는 `ScreenFS`의 현재 목표 계약을 정리한다. canonical policy surface는 `visibility`/`mutability` 두 축이다. shared matcher는 bare slashless direct-child, anchored direct-child, subtree shorthand, recursive glob family를 함께 정규화한다. 다만 current `visibility.visible` surface는 recursive bridge discovery가 필요 없는 범주만 허용한다. 즉 exact path, subtree(`/dir`, `/dir/**`), direct-child anchor bridge(`/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형)만 현재 계약이고, `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks` 같은 recursive descendant visible glob/shorthand은 shorthand와 canonical form 모두 unsupported/fail-fast다. recursive family 자체는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서 계속 current이며, recursive literal directory shorthand는 `**/<literal-dir>` 또는 `<prefix>/**/<literal-tail>`만 허용되는 non-visible-surface-only normalization delta다. `/**/`는 최대 한 번만 허용되고 tail component는 모두 literal이어야 하며 내부적으로 `<prefix>/**/<literal-tail>/**`로 normalize된다. 예: `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`. 이는 subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이다. current source/live evidence는 `docs/operations.md`가 정리한다.

아키텍처 시각화 요약은 [docs/architecture.md](architecture.md)에 정리되어 있다. 다이어그램 source of truth는 `docs/diagrams/*.mmd`이고, 렌더링 계약은 [docs/diagrams/README.md](diagrams/README.md)를 따른다.

## 한눈에 보기

### 시스템 컨텍스트

![ScreenFS system context](diagrams/system-context.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### 요청 처리 흐름

![ScreenFS request decision flow](diagrams/request-decision-flow.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### Visibility axis

![ScreenFS visibility axis](diagrams/visibility-axis.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### Mutability axis

![ScreenFS mutability axis](diagrams/mutability-axis.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### 모듈 구조

![ScreenFS module architecture](diagrams/module-architecture.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

## 빠른 읽기 가이드

- **무엇을 만드는가**: 1~4절
- **코드 경계와 데이터 흐름**: 5~8절
- **visibility / mutability 의미론**: 9~11절
- **성능·검증·수용 기준**: 12절 이후

## 현재 구현 evidence와 문서 계약

| 관점 | 현재 구현에서 읽히는 것 | 이 문서의 계약 |
| --- | --- | --- |
| visibility | `visibility.default`, hidden/visible matcher, bridge-visible ancestor 계산 | hidden/bridge-visible/visible 세 상태와 hidden-first 의미론 |
| mutability | `mutability.default`, readonly/writable matcher, same-axis conflict validation | visible path mutation에 대한 readonly/writable 판정 |
| rule grammar | hidden/visible/readonly/writable 모두 공유 matcher 사용 | 네 rule surface가 같은 normalization contract 공유 |
| precedence | 축별 best match 선택, hidden이 mutability보다 먼저 적용 | most-specific wins + hidden `ENOENT` precedence |

참고: 위 표의 현재 구현 evidence에는 bare slashless cwd-anchor/direct-child delta도 포함된다. source regression coverage와 repo-local live smoke baseline은 `docs/operations.md`와 `docs/artifacts/current-bare-basename-glob-smoke-transcript.md`에서 확인할 수 있다.

## 1. 설계 목표

`ScreenFS`는 sandbox/chroot 같은 whole-root consumer가 읽을 수 있는 non-root FUSE 기반 filesystem view layer다.

핵심 목표:

- non-root 사용자 권한으로 `fusermount3` 기반 FUSE3 mount를 생성한다.
- `fractal-fuse = 0.4.0` 기반으로 구현한다.
- v1은 `FUSE_OVER_IO_URING` 사용을 필수로 하며, 협상 실패 시 fallback 없이 명시적 오류로 fail-fast 한다.
- 전체 `/` view를 상위 whole-root consumer에게 제공한다.
- 기본은 underlying filesystem pass-through다.
- visibility 축으로 hidden/bridge-visible/visible을 판정한다.
- mutability 축으로 writable/readonly를 판정한다.
- hidden `ENOENT`가 mutability `EROFS`보다 우선한다.

## 2. Threat model, integration assumptions, non-goals

`ScreenFS`는 path-hiding filesystem view layer다. 단독 sandbox 또는 완전한 host isolation boundary가 아니다.

통합 전제:

- mount 생성은 non-root 사용자 권한으로 가능해야 한다.
- `chroot` 실행 권한, privileged supervisor, user namespace 구성은 상위 레이어 책임이다.
- same-host-uid 접근 모델이 기본 전제다.
- `allow_other`가 필요하면 `/etc/fuse.conf`와 mountpoint 권한 정책이 별도로 필요하다.
- process, network, namespace, cgroup, seccomp isolation은 상위 레이어가 담당한다.

Non-goals:

- root-only bind/overlay masking
- device node native semantics 재현
- procfs/sysfs caller-relative semantics 재현
- hidden hardlink alternate path의 자동 전역 차단

## 3. Filesystem view model

입력은 `source_root`, `mount_root`이고 출력은 whole-root consumer가 읽는 virtual root다.

```text
source root: /
mount root:  /tmp/screenfs-root
chroot root: /tmp/screenfs-root
```

mount root 아래 virtual path는 source root 아래 같은 상대 경로로 해석한다.

```text
/tmp/screenfs-root/usr/bin/bash -> /usr/bin/bash
/tmp/screenfs-root/etc          -> /etc
```

정책 축은 서로 독립적이다.

- **visibility**: hidden / bridge-visible / visible
- **mutability**: readonly / writable

bridge-visible은 visibility 축의 중간 상태이며, visible descendant로 도달시키기 위한 ancestor directory에만 적용된다.

## 4. Mount-root recursion exclusion

`source root = /`이고 `mount root = /tmp/screenfs-root`이면 mount root subtree를 view 내부에 노출하면 자기참조가 생긴다.

```text
/tmp/screenfs-root/tmp/screenfs-root/tmp/screenfs-root/...
```

따라서 mount root의 source-relative subtree는 내부 강제 hidden rule로 취급한다.

필수 정책:

- mount root subtree는 `readdir`/`readdirplus` 결과에서 제외한다.
- mount root subtree에 대한 `lookup`, `getattr`, `statx`, `open`, `opendir`, `access`, `readlink`, xattr 조회는 `ENOENT`다.
- 이 internal rule은 user policy보다 먼저 적용된다.

## 5. Architecture overview and module boundaries

- 진입점: `src/main.rs`
- 정책 조립: `src/cli.rs`, `src/config.rs`, `src/matcher.rs`
- 경로 의미론: `src/path.rs`
- errno / guard: `src/errors.rs`
- FUSE 요청 처리 중심: `src/fs.rs`, `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`

Data flow:

1. FUSE request arrives with inode/name or file handle.
2. Request is mapped to a virtual absolute path using inode/path state.
3. The virtual path is lexically normalized.
4. Internal mount-root exclusion runs first.
5. Visibility evaluator computes `hidden`, `bridge-visible`, or `visible`, including bridge synthesis for reachable visible descendants.
6. Hidden returns `ENOENT` or omits listing entries.
7. Symlink read operations stay readable only when the resolved virtual target is fully visible; hidden or bridge-visible/non-fully-visible targets return `ENOENT`.
8. Bridge-visible directory mutation returns `EROFS`.
9. Visible path then goes through mutability evaluation.
10. Host delegation happens only after both axes allow it.

## 6. Virtual path model and shared rule normalization

hidden/visible/readonly/writable 판정 기준은 host canonical path가 아니라 **lexically normalized absolute virtual path**다.

Rules:

- virtual root is always `/`
- `.` and duplicate `/` are removed
- `..` pops one component but cannot escape above virtual `/`
- host canonicalization에 의존하지 않고 nonexistent path, broken symlink, permission-denied parent도 분류 가능해야 한다
- source path resolution must not escape `source_root`

### 6.1 Shared rule grammar

`visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`는 같은 lexical normalization을 재사용한다. 다만 `visibility.visible`은 그중 discovery-free visible category만 현재 surface로 허용한다.

지원 family:

- **exact / subtree family**: absolute path, relative/`~` rebased exact path, `/dir`, `/dir/**`, `./dir`, `./dir/**`, `~/dir`, `~/dir/**`
- **direct-child glob family**: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `./dir/*`, `~/dir/*`, bare `*.pem`, `.env.*`, `id_*`; bare slashless form은 같은 normalized cwd anchor의 `./<pattern>` direct-child shorthand다
- **recursive non-visible glob family**: `**/.env`, `**/*.pem`, `**/.env.*`, `**/id_*`, `/**/.env`, `/**/*.pem`, `/**/.env.*`, `/**/id_*`, `./fixtures/**/*.pem`, `/a/**/*.txt`; prefixless form은 같은 normalized cwd anchor의 `./**/<pattern>` shorthand다
- **recursive literal non-visible subtree family**: canonical form `<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`와 recursive literal directory shorthand `<normalized-prefix>/**/<literal-component>(/<literal-component>)*`; shorthand는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 지원된다. 허용 shorthand는 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`뿐이며 `/**/`는 최대 한 번만 쓸 수 있고 tail component는 모두 literal이어야 한다. 내부적으로는 `<prefix>/**/<literal-tail>/**` canonical form으로 compile된다. 이는 same-anchor subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이며 subtree shorthand 의미는 바뀌지 않는다. 예: `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`, `./repo/**/.git/hooks`=`./repo/**/.git/hooks/**`, `~/project/**/.git/hooks`=`~/project/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`, `**/target`=`**/target/**`, `**/dist`=`**/dist/**`, `**/build`=`**/build/**`

current `visibility.visible` subset:

- **static subtree bridge**: exact path와 subtree shorthand(`/dir`, `/dir/**`)
- **direct-child anchor bridge**: direct-child wildcard-all(`/dir/*`)과 direct-child basename-prefix/suffix form(`/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`), bare/cwd/HOME 동등형
- **unsupported recursive visible glob/shorthand**: `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks`

정규화/충돌 규칙:

- same-anchor `/dir/**`는 `/dir`와 같은 normalized descriptor/specificity로 compile된다.
- same-polarity identical descriptor는 deduplicate/idempotent다.
- `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`는 `/dir/*` 안에 contained되는 더 구체적인 direct-child rule이다.
- `/dir/*`는 `/dir`/`/dir/**` 안에 contained된다.
- recursive non-visible family에서는 기존 specificity/containment/conflict 규칙을 유지한다. 예를 들어 `**/*.pem`는 같은 normalized cwd anchor에서 `./**/*.pem`과 동등하고, `*.pem`는 same-anchor `**/*.pem` target set 안에 contained되는 direct-child rule이다. recursive literal descendant-subtree shorthand도 canonical form과 동일 descriptor/specificity로 compile된다. 즉 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`이고 directory 자체와 descendants를 함께 매치하며 same-specificity conflict, containment, dedup 결과도 동일하다.
- `visibility.visible`에 recursive family나 그 recursive literal directory shorthand가 들어오면 specificity 계산 전에 fail-fast 한다. 즉 overlap resolution이나 dynamic bridge discovery로 우회하지 않는다.

fail-fast subset:

- missing `HOME`
- launch cwd outside `source_root` for relative/prefixless forms
- expanded path outside `source_root`
- `~user`
- wildcard-in-prefix broader forms
- unanchored wildcard-all recursive form(`**/*`)
- one-sided basename-prefix/suffix subset 밖의 bare wildcard form(`*`, `a*b`, `*secret*`)
- descendant-subtree literal tail 내부 wildcard 또는 recursive literal directory shorthand subset 밖의 trailing `/**`-less/multi-recursive descendant-subtree form(`~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]` 포함)
- `visibility.visible`에서 recursive bridge discovery가 필요한 recursive descendant canonical/shorthand form
- brace/env/command expansion

## 7. Visibility policy and matcher

![ScreenFS visibility axis](diagrams/visibility-axis.svg)

visibility evaluator는 path마다 다음 셋 중 하나를 낸다.

- `hidden`
- `bridge-visible`
- `visible`

### 7.1 Default and overrides

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []
```

해석:

- `default=visible`: `hidden`이 기본 차단 rule이고 `visible`이 더 구체적인 carve-out이 될 수 있다.
- `default=hidden`: `visible`이 기본 allowlist이고 `hidden`이 더 구체적인 re-block이 될 수 있다.

### 7.2 Hidden semantics

hidden path semantics:

- `lookup`, `getattr`, `open`, `access`, `readlink`, `statx` → `ENOENT`
- `readdir`, `readdirplus` → omit entry
- symlink whose own path is hidden → `ENOENT`
- symlink readlink/dereference is allowed only when the resolved virtual target is fully visible; hidden target이거나 bridge-visible/non-fully-visible target이면 `ENOENT`
- symlink target visibility fast path는 compiled policy가 그 entry를 숨길 수 없음을 보일 때만 resolved final virtual target 재평가를 생략할 수 있다.
- 그렇지 않으면 listing/`lookup`/`getattr`/`readlink`/dereference/`open` 시점마다 multi-hop symlink와 ancestor symlink를 반영한 resolved final virtual target을 다시 확인해야 하며, prior listing success·symlink decision cache·direct-path-only memoized result는 면제 근거가 아니다.

### 7.3 `visibility.visible` current category

current `visibility.visible`은 세 범주로 읽는다.

1. **static subtree bridge**
   - exact path 또는 subtree shorthand(`/dir`, `/dir/**`)
   - visible target까지의 정적 ancestor chain만 bridge-visible이 된다.
2. **direct-child anchor bridge**
   - `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형
   - normalized anchor의 ancestor들만 bridge-visible candidate가 된다.
   - immediate child match는 `lookup`/`readdir`/`readdirplus` 시점에 현재 directory/parent 기준으로만 평가하며, 그 directory와 무관한 matcher bucket은 건너뛴다. anchor subtree를 재귀 스캔하지 않는다.
   - hidden sibling은 계속 숨겨지고, matched child 및 그 descendants만 visible로 열릴 수 있다.
3. **unsupported recursive visible glob/shorthand**
   - `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks`
   - recursive bridge discovery가 필요하므로 current contract에서는 canonical form과 shorthand form 모두 fail-fast다. 권장 대안은 `/dir` 또는 `/dir/**` 같은 explicit subtree visible rule이다.

### 7.4 Bridge-visible ancestor semantics

bridge-visible은 visible descendant를 향한 ancestor directory에서만 생긴다.

허용:

- `lookup`, `getattr`, read/stat/traverse intent `access`
- `opendir`, `readdir`, `readdirplus`

거부:

- create/delete/rename/link/symlink
- write-intent `open`
- mutation `setattr`, xattr mutation, `fallocate`

반환 errno:

- bridge-visible ancestor mutation은 `EROFS`

중요한 구현 제약:

- subtree visible rule은 visible target까지의 ancestor chain만 사용한다.
- direct-child visible rule은 normalized anchor ancestor와 immediate child evaluation만 사용한다.
- directory-entry filtering fast path는 `readdir`/`readdirplus`에서 현재 directory/parent와 관련된 matcher bucket만 보고 unrelated bucket을 건너뛰는 보수적 형태로만 허용된다.
- recursive descendant visible rule을 위한 eager/lazy bridge discovery, startup recursive bridge scan, dynamic bridge ancestor index는 current contract가 아니다.
- recursive literal directory shorthand는 normalization/path-matcher-only다. `**/.git/hooks`, `~/**/aaa/hook` 같은 supported shorthand는 각각 `**/.git/hooks/**`, `~/**/aaa/hook/**`와 같은 matcher cost를 유지해야 하고, shorthand 때문에 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 추가하면 안 된다.
- 위 fast path는 current supported grammar에만 적용되며 unsupported visible recursive form이나 broader wildcard form을 근사하면 안 된다.

### 7.5 Listing semantics

- hidden child는 listing에서 제외
- bridge-visible directory는 visible child 또는 다음 bridge-visible descendant로 이어지는 child만 보여준다
- direct-child visible rule에서도 immediate child evaluation 결과에 없는 sibling은 노출하지 않는다
- `readdir`/`readdirplus` filtering은 현재 directory/parent 기준 결과를 바꾸지 않는 범위에서만 관련 matcher bucket을 보고 unrelated bucket을 건너뛸 수 있다.
- `readdirplus`는 반환하는 visible/bridge-visible child에 대해서만 metadata를 준다
- listing에 보이는 symlink entry도 resolved virtual target이 fully visible할 때만 `readlink`/dereference가 가능하다
- stable directory listing cache나 snapshot은 current contract가 아니다. 구현은 필요하면 per-handle iteration state만 둘 수 있다.
- hidden sibling count나 hidden subtree metadata를 새지 않아야 한다

## 8. Inode, path, and handle model

FUSE는 inode-centric이고 정책은 path-centric이므로 둘 다 유지해야 한다.

- root inode is fixed to `FUSE_ROOT_ID = 1`
- visible or bridge-visible exported objects receive synthetic FUSE inode ids
- hidden paths are never exported into the inode table
- directory handles는 필요하면 visibility evaluation 이후 optional per-handle iteration state를 유지할 수 있다
- writable mutation invalidates affected parent directories, matcher indexes, and path/inode caches

bridge-visible iteration rule:

- per-handle state가 있다면 hidden entry를 포함하지 않는다
- per-handle state가 있다면 bridge-visible directory에서는 next-hop bridge/visible child만 다룬다

## 9. FUSE operation matrix

평가 순서는 항상 visibility → bridge-visible special case → mutability다.

### 9.1 조회 / 탐색 연산

| Operation class | Hidden | Bridge-visible directory | Visible |
| --- | --- | --- | --- |
| `lookup`, `getattr`, `statx` | `ENOENT` | pass-through or synthesized directory metadata | pass-through |
| `access` without write mask | `ENOENT` | pass-through | pass-through |
| `access` with `W_OK` | `ENOENT` | `EROFS` | mutability 기준 |
| `opendir` | `ENOENT` | pass-through handle | pass-through handle |
| `readdir`, `readdirplus` | omit | filtered bridge listing | filtered visible listing |
| `open` read-only intent | `ENOENT` | directory-only traversal semantics | pass-through (symlink dereference는 resolved virtual target fully-visible일 때만 허용) |
| `readlink` | `ENOENT` | n/a | resolved virtual target이 fully visible일 때만 pass-through; hidden 또는 bridge-visible target이면 `ENOENT` |
| `statfs` | not path-hidden except mount-root policy | pass-through or synthesized | pass-through or synthesized |

### 9.2 쓰기 / 생성 / 삭제 연산

| Operation class | Hidden | Bridge-visible directory | Visible |
| --- | --- | --- | --- |
| `open` write intent | `ENOENT` | `EROFS` | mutability 기준 |
| `write` | `ENOENT` if hidden handle should not exist | `EROFS` | mutability 기준 |
| `create`, `mkdir`, `mknod` | `ENOENT` | `EROFS` | mutability 기준 |
| `unlink`, `rmdir` | `ENOENT` | `EROFS` | mutability 기준 |
| `fallocate` | `ENOENT` | `EROFS` | mutability 기준 |
| metadata / xattr mutation | `ENOENT` | `EROFS` | mutability 기준 |

### 9.3 멀티패스 연산

| Operation class | 규칙 |
| --- | --- |
| `rename` | source/target/source parent/target parent 중 하나라도 hidden이면 `ENOENT`; bridge-visible mutation coordinate가 있거나 readonly coordinate가 있으면 `EROFS` |
| `link`, `symlink` | hidden involved path가 있으면 `ENOENT`; bridge-visible mutation coordinate나 readonly coordinate가 있으면 `EROFS` |
| `copy_file_range` | source는 visibility만, destination/destination parent는 mutability를 본다; hidden이면 `ENOENT`, readonly 또는 bridge-visible mutation이면 `EROFS` |

## 10. Mutability policy and evaluator

![ScreenFS mutability axis](diagrams/mutability-axis.svg)

```yaml
mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

해석:

- `default=writable`: `readonly`가 primary block, `writable`이 more-specific carve-out
- `default=readonly`: `writable`가 primary allow, `readonly`가 more-specific re-block

평가 원칙:

1. hidden이면 결과는 외부에서 `ENOENT`다.
2. bridge-visible ancestor mutation이면 `EROFS`다.
3. mutation에 관여하는 모든 write-requiring coordinate를 계산한다.
4. 각 coordinate에 대해 mutability specificity를 평가한다.
5. 하나라도 readonly면 `EROFS`다.
6. 전부 writable이면 host filesystem으로 위임한다.

affected-coordinate rule:

- `create` / `mkdir` / `mknod`: target path + parent path
- `unlink` / `rmdir`: removed path + parent path
- write-intent `open` / `write` / mutation `setattr` / xattr mutation / `fallocate`: entry path와 필요 시 resolved target
- `rename`: source path + target path + source parent + target parent
- `link`: source path + target path + target parent
- `symlink`: link path + link parent + target-visibility checks
- `copy_file_range`: source visibility + destination path + destination parent

## 11. Errno mapping

- hidden path: `ENOENT`
- hidden entry in listing: omit entry
- bridge-visible ancestor mutation: `EROFS`
- readonly-matched visible mutation: `EROFS`
- host `io::Error::raw_os_error()`: preserve raw errno
- no raw errno: `EIO`
- host permission failure on visible writable path: preserve `EACCES`/`EPERM`

핵심 우선순위:

```text
hidden ENOENT
  > bridge-visible mutation EROFS
  > mutability readonly EROFS
  > host errno
```

## 12. Magic filesystem and device policy

Whole `/` view는 native kernel filesystem 재현을 의미하지 않는다.

| Path family | ScreenFS v1 policy | Responsible layer |
| --- | --- | --- |
| `/proc`, `/proc/self`, `/proc/thread-self` | ordinary visible path traversal only | 상위 supervisor/namespace layer |
| `/sys` | ordinary visible path traversal only | 상위 supervisor/namespace layer |
| `/dev`, `/dev/null`, `/dev/zero`, `/dev/urandom` | device node semantics are not guaranteed | 상위 supervisor/namespace layer |
| `/dev/fd`, `/dev/stdin`, `/dev/stdout`, `/dev/stderr` | fd-relative semantics are not guaranteed | 상위 supervisor/namespace layer |
| `/run` | ordinary visible path traversal only | 상위 supervisor/namespace layer |

## 13. FUSE3, io_uring, and mount options

- detect `/dev/fuse`, FUSE support, `fusermount3`, and session capabilities
- v1 requires `FUSE_OVER_IO_URING` negotiation success
- negotiation failure is explicit startup error with no fallback mount
- `allow_other`는 기본 계약이 아니다
- mount-level `ro`는 selective policy의 source of truth가 될 수 없다
- `force_readdir_plus`와 passthrough optimization은 correctness 이후 단계에서 평가한다

## 14. Cache and memory model

Recommended caches/state:

- path visibility result cache keyed by rule version
- inode table and reverse map
- file/dir handle table
- optional per-handle directory iteration state

Policy:

- bounded cache / LRU eviction
- conservative timeout defaults until correctness is proven
- writable mutation invalidates affected parent directory, involved path entries, and inode/path cache entries
- matcher/indexing은 family와 normalized anchor를 기준으로 분리한다. 최소한 exact/subtree, direct-child glob, recursive non-visible glob, recursive literal non-visible subtree descriptor를 별도 집합으로 유지한다.
- same-polarity identical descriptor는 compile 시 dedup/idempotent 처리한다. `/dir/**`는 `/dir`와 동일 descriptor로 정규화하고, `**/.git/hooks` 같은 recursive literal directory shorthand는 `**/.git/hooks/**` canonical descriptor로 정규화한다.
- `visibility.visible` reachability는 discovery-free여야 한다. subtree visible rule은 정적 ancestor chain만 사용하고, direct-child visible rule은 normalized anchor ancestor와 immediate child evaluation만 사용한다.
- `readdir`/`readdirplus` fast path는 현재 directory/parent와 무관한 matcher bucket을 건너뛰는 보수적 형태로만 허용된다. recursive scan, background index, stable listing result cache는 current contract가 아니다.
- startup recursive bridge scan, lazy recursive bridge discovery, dynamic bridge ancestor index는 current contract가 아니다. recursive indexing이 필요하면 hidden/readonly/writable의 recursive family에만 국한한다. recursive literal directory shorthand는 canonical recursive literal descriptor에 normalize될 뿐 별도 discovery/index family를 만들면 안 된다.
- symlink target visibility check는 policy가 hide 가능성을 배제할 때만 생략할 수 있다. 그 외 point-of-use check는 symlink decision cache로 대체할 수 없고, bridge-visible 및 symlink-dependent visibility check에 stale direct-path-only cache를 재사용하면 안 된다. multi-hop symlink와 ancestor symlink를 실제 resolved virtual target 기준으로 다시 확인해 hidden 또는 bridge-visible final target은 `ENOENT`로 막아야 한다.
- recursive literal directory shorthand 추가는 normalization/path-matcher-only여야 하며 recursive bridge discovery, lazy discovery, startup scan, background indexing, stable listing result cache, symlink decision cache, 기타 새로운 filesystem discovery를 도입하면 안 된다. matcher cost는 기존 canonical `**/.../**` recursive literal subtree rule과 같아야 한다.
- 위 fast path들은 current supported grammar에만 적용되며 unsupported visible recursive form이나 broader wildcard form을 근사하면 안 된다

## 15. Performance model

전체 `/` view는 metadata-heavy workload가 많다.

Reference targets:

- visibility/mutability matcher benchmark는 no-rule baseline 대비 제한된 overhead 안에 있어야 한다
- idle RSS after startup < 128 MiB
- repeated traversal after five runs: RSS growth < 64 MiB from baseline
- repeated traversal after five runs: open fd count returns close to baseline
- `readdirplus` on large directory completes without unbounded memory growth
- visible direct-child rule(`/tmp/*` 또는 동등형)는 anchor subtree를 재귀 순회하지 않아야 하며, smoke/계측은 현재 directory/parent 기준 unrelated matcher bucket skip과 결과 불변을 함께 보여줘야 한다
- recursive literal directory shorthand는 canonical `**/.../**` recursive literal subtree rule과 같은 matcher cost를 유지해야 하며, `~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]` 같은 multi-recursive 또는 broader form은 discovery, ambiguous containment, broader glob compatibility를 피하기 위해 fail-fast 해야 한다
- symlink-heavy workload에서도 target visibility check는 policy가 hide 가능성을 배제하지 못하면 resolved final target 기준으로 point-of-use에서 다시 수행돼야 하며, prior listing success나 direct-path-only memoization에 기대면 안 된다

Implementation priorities:

- implement `readdirplus`
- avoid per-entry host canonicalization
- compile rules once at startup
- keep exact/prefix/glob matcher tiers bounded
- keep visible bridge derivation subtree/direct-child bounded and discovery-free
- keep visible file data path close to underlying filesystem
- keep metadata/xattr delegation fd-based or dirfd-relative where possible after openat2 confinement

## 16. CLI / config shape

Current contract exposes the two axes directly.

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--visibility-default visible|hidden] \
  [--hidden <rule> ...] \
  [--visible <rule> ...] \
  [--mutability-default writable|readonly] \
  [--readonly <rule> ...] \
  [--writable <rule> ...]
```

Config contract:

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []

mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

Contract notes:

- CLI axis option이 있으면 해당 축의 config block을 대체한다.
- visibility default is `visible` when omitted.
- mutability default is `writable` when omitted.
- duplicate same-polarity rule is idempotent.
- opposite-polarity same-specificity conflict on the same normalized anchor is fail-fast.
- 제거된 CLI/config surface를 위한 compatibility mapping이나 shim은 없다.

## 17. Validation strategy

Validation은 unit, integration, mount smoke, system smoke로 나눈다.

### Unit tests

- lexical virtual path normalization
- source-root escape prevention
- shared rule normalization for visibility/mutability
- specificity ordering and same-specificity conflict rejection
- same-anchor `/dir`=`/dir/**` descriptor dedup
- `/dir/*.pem` ⊂ `/dir/*` ⊂ `/dir` containment
- recursive literal descendant-subtree shorthand equivalence(`**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`)와 canonical descriptor dedup/conflict identity
- hidden path matching
- subtree/direct-child bridge-visible ancestor synthesis
- `visibility.visible` recursive descendant glob/shorthand rejection
- hidden-before-`EROFS` precedence
- mutability evaluation for matching and non-matching visible paths
- multi-path affected-coordinate classification
- errno mapping

### Integration tests without mount

- tempdir-backed fixture
- visible lookup/getattr/open/read/list behavior
- hidden lookup/getattr/open/access/readlink/xattr behavior
- bridge-visible ancestor stat/traverse/list behavior
- bridge-visible ancestor mutation `EROFS`
- direct-child visible rule이 immediate child만 평가하고 hidden sibling을 노출하지 않는지 확인
- `visibility.visible`이 `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, cwd/HOME-relative recursive form을 fail-fast로 거부하는지 확인
- non-visible surface에서 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**` shorthand equivalence와 hidden `ENOENT`/readonly `EROFS`/writable override 결과가 canonical form과 동일한지 확인
- unsupported trailing `/**`-less broader/ambiguous form이 부분 해석 없이 fail-fast 되는지 확인
- symlink whose resolved target is hidden or bridge-visible/non-fully-visible returns `ENOENT`
- symlink target visibility fast path가 point-of-use lexical check를 부당하게 생략하지 않는지 확인
- filtered `readdir` and `readdirplus`
- `readdir`/`readdirplus`가 현재 directory/parent와 무관한 matcher bucket을 건너뛰어도 결과가 바뀌지 않는지 counter/trace/perf evidence로 확인
- shorthand 추가 전후로 새로운 discovery/scanning code가 생기지 않았는지 source diff/trace/counter evidence로 확인
- relative cwd-based rebasing and `HOME`-based rebasing
- missing `HOME`, `~user`, source-root-outside, broader unsupported wildcard fail-fast
- cache invalidation after writable mutation

### FUSE mount smoke tests

- `ls`, `find`, `rg` do not list hidden entries
- `stat`, `cat`, `access` on hidden path fail with `ENOENT`
- bridge-visible ancestor is traversable but not writable
- bridge-visible directory listing only exposes bridge/visible descendants
- direct-child visible rule(`/tmp/*` 또는 동등형)가 immediate child만 여는지, hidden sibling을 계속 숨기는지 확인
- `visibility.visible` recursive descendant glob/shorthand rejection stderr를 별도 세션으로 남긴다
- non-visible surface에서 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`, `**/.git`=`**/.git/**`, `**/target`=`**/target/**`, `**/dist`=`**/dist/**`, `**/build`=`**/build/**` equivalence를 별도 세션으로 남긴다
- directory-entry filtering fast path가 현재 directory/parent와 무관한 matcher bucket을 건너뛰면서도 결과를 바꾸지 않는다는 counter/trace/perf evidence를 남긴다
- shorthand 추가 때문에 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery가 생기지 않았다는 source diff/trace/perf evidence를 남긴다
- symlink entry는 resolved virtual target이 fully visible할 때만 `readlink`/dereference로 읽힌다. A raw symlink target that is lexically fully visible but escapes `source_root` may still be returned by `readlink`; dereference then fails in source-root confinement with `ENOENT`.
- prior listing success나 direct-path-only memoized result를 symlink-dependent check 면제로 쓰지 않는 evidence를 남긴다.
- mount root subtree is not visible through the mounted view
- `/bin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/home`, `/tmp`, `/var` are visible when policy allows
- supported exact / recursive / direct-child inputs smoke separately
- unsupported wildcard forms과 shorthand subset 밖 trailing `/**`-less descendant form은 partial interpretation 없이 fail fast다
- hidden non-visible shorthand direct access는 `ENOENT`다
- readonly visible mutation returns `EROFS`
- writable carve-out visible mutation succeeds when host allows it
- shorthand/canonical same-specificity conflict와 same-polarity dedup 결과가 동일하다는 evidence를 남긴다

### System smoke tests

- confirm `fusermount3` path and version
- confirm `/dev/fuse` and kernel FUSE support
- confirm documented kernel config artifact still shows `CONFIG_FUSE_IO_URING=y` and `CONFIG_IO_URING=y`
- confirm same-uid access model or explicit `allow_other` policy
- `chroot` smoke only when privilege/user namespace model is available
- `/tmp/*` 같은 direct-child visible rule에 대해 recursive traversal이 일어나지 않고 현재 directory/parent와 무관한 matcher bucket을 건너뛴다는 성능-oriented smoke 또는 동등한 계측을 남긴다
- daemon shutdown and `fusermount3 -u` cleanup behavior

## 18. Archival note

current policy surface는 항상 `visibility`/`mutability` 두 축으로 읽는다.
