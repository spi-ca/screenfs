# ScreenFS 설계 문서

이 문서는 `ScreenFS` 구현 계약을 정리한다. canonical policy surface는 `visibility`/`mutability` 두 축이며, legacy family/flag 모델은 archival note에서만 다룬다.

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

## 현재 구현 vs 계약

| 관점 | 현재 구현에서 읽히는 것 | 이 문서의 계약 |
| --- | --- | --- |
| visibility | `visibility.default`, hidden/visible matcher, bridge-visible ancestor 계산 | hidden/bridge-visible/visible 세 상태와 hidden-first 의미론 |
| mutability | `mutability.default`, readonly/writable matcher, same-axis conflict validation | visible path mutation에 대한 readonly/writable 판정 |
| rule grammar | hidden/visible/readonly/writable 모두 공유 matcher 사용 | 네 rule surface가 같은 normalization contract 공유 |
| precedence | 축별 best match 선택, hidden이 mutability보다 먼저 적용 | most-specific wins + hidden `ENOENT` precedence |

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
- hidden hardlink alias의 자동 전역 차단

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

`visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`는 같은 grammar를 사용한다.

지원 subset:

- absolute exact path
- relative exact path with cwd-based rebasing into `source_root`
- `~` / `~/...` exact path with `HOME` expansion and rebasing
- recursive basename/suffix/basename-prefix tail: `**/.env`, `**/*.pem`, `**/.env.*`
- normalized-prefix direct-child basename-prefix/suffix form: `./fixtures/*.pem`, `~/.env.*`, `/home/<user>/*.pem`
- limited recursive literal descendant-subtree glob: `<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`

예시:

```text
/home/<user>/.ssh
**/.env
**/.git/hooks/**
./repo/**/.git/hooks/**
~/.env.*
/home/<user>/*.pem
```

fail-fast subset:

- missing `HOME`
- expanded path outside `source_root`
- `~user`
- wildcard-in-prefix broader forms
- bare suffix `*.pem`
- descendant-subtree literal tail 내부 wildcard
- trailing `/**` 없는 descendant-subtree form
- brace/env/command expansion

### 6.2 Specificity and conflicts

각 축은 같은 override 규칙을 사용한다.

- most-specific match wins
- same-specificity same-polarity duplicate는 idempotent
- same-axis opposite rule이 같은 normalized anchor/specificity에서 충돌하면 fail-fast
- hidden 결과는 mutability보다 먼저 적용된다

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

### 7.3 Bridge-visible ancestor semantics

bridge-visible은 visible descendant를 향한 ancestor directory에서만 생긴다. descendant-subtree carve-out도 같은 규칙을 따르며, `**/.git/hooks/**` 같은 rule이 매치한 visible subtree에 도달하는 경로상의 모든 existing ancestor directory는 bridge-visible로 합성되어야 한다.

또한 bridge-visible은 traversal/listing 전용 상태다. bridge-visible 경로를 통해 도달 가능한 symlink entry라도 resolved virtual target이 fully visible하지 않으면 `readlink`나 symlink dereference `open`으로 노출하지 않는다.

허용:

- `lookup`, `getattr`, read/stat/traverse intent `access`
- `opendir`, `readdir`, `readdirplus`

거부:

- create/delete/rename/link/symlink
- write-intent `open`
- mutation `setattr`, xattr mutation, `fallocate`

반환 errno:

- bridge-visible ancestor mutation은 `EROFS`

### 7.4 Listing semantics

- hidden child는 listing에서 제외
- bridge-visible directory는 visible child 또는 다음 bridge-visible descendant로 이어지는 child만 보여준다
- `readdirplus`는 반환하는 visible/bridge-visible child에 대해서만 metadata를 준다
- listing에 보이는 symlink entry도 resolved virtual target이 fully visible할 때만 `readlink`/dereference가 가능하다
- hidden sibling count나 hidden subtree metadata를 새지 않아야 한다

## 8. Inode, path, and handle model

FUSE는 inode-centric이고 정책은 path-centric이므로 둘 다 유지해야 한다.

- root inode is fixed to `FUSE_ROOT_ID = 1`
- visible or bridge-visible exported objects receive synthetic FUSE inode ids
- hidden paths are never exported into the inode table
- directory handles keep filtered snapshots after visibility evaluation
- writable mutation invalidates affected parent directories, alias indexes, and path/inode caches

bridge-visible snapshot rule:

- snapshot은 hidden entry를 포함하지 않는다
- bridge-visible directory snapshot은 next-hop bridge/visible child만 포함한다

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

Recommended caches:

- path visibility result cache keyed by rule version
- inode table and reverse map
- directory snapshot cache
- file/dir handle table

Policy:

- bounded cache / LRU eviction
- conservative timeout defaults until correctness is proven
- writable mutation invalidates affected parent directory, involved path entries, and inode/path cache entries
- bridge-visible reachability는 mount startup에서 source tree를 스캔해 existing visible descendant의 ancestor index로 구축하거나 동등한 bounded/cacheable 구조를 사용해야 하며, request hot path에서 whole-root recursive scan을 유발하면 안 된다
- dynamic glob visible rule의 bridge-visible ancestor index는 mount-start snapshot이다. 외부 backing-tree 변경으로 새 dynamic-glob visible descendant가 생겨도 mid-mount에 previously unreachable hidden ancestor를 새로 열지 않으며, 그런 reachability 확장은 remount로 갱신한다. ScreenFS 내부 writable mutation은 affected inode/path/directory snapshot을 invalidate하지만 dynamic bridge index 자체를 확장하지 않는다.
- 현재 구현은 `ScreenFs::new`에서 dynamic bridge-visible ancestor index를 만들되 dynamic rule anchor를 scan root로 사용해 anchored glob의 startup 범위를 제한하고, request path에서는 static subtree bridge query 또는 index lookup만 수행한다
- bridge-visible and symlink-target-dependent visibility checks must not be served from stale direct-path-only cache

## 15. Performance model

전체 `/` view는 metadata-heavy workload가 많다.

Reference targets:

- visibility/mutability matcher benchmark는 no-rule baseline 대비 제한된 overhead 안에 있어야 한다
- idle RSS after startup < 128 MiB
- repeated traversal after five runs: RSS growth < 64 MiB from baseline
- repeated traversal after five runs: open fd count returns close to baseline
- `readdirplus` on large directory completes without unbounded memory growth
- 위 성능 기대치는 live mount smoke와 large-traversal performance smoke로 계속 재확인해야 한다

Implementation priorities:

- implement `readdirplus`
- avoid per-entry host canonicalization
- compile rules once at startup
- keep exact/prefix/glob matcher tiers bounded
- keep bridge-visible reachability derivation policy-bounded and cacheable by using a precomputed or equivalently bounded bridge ancestor index instead of request-time whole-root recursive scans
- keep visible file data path close to underlying filesystem

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
- removed family/flag surface를 위한 compatibility alias나 shim은 없다.

## 17. Validation strategy

Validation은 unit, integration, mount smoke, system smoke로 나눈다.

### Unit tests

- lexical virtual path normalization
- source-root escape prevention
- shared rule normalization for visibility/mutability
- specificity ordering and same-specificity conflict rejection
- hidden path matching
- bridge-visible ancestor synthesis
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
- symlink whose resolved target is hidden or bridge-visible/non-fully-visible returns `ENOENT`
- filtered `readdir` and `readdirplus`
- relative cwd-based rebasing and `HOME`-based rebasing
- missing `HOME`, `~user`, source-root-outside, broader unsupported wildcard fail-fast
- cache invalidation after writable mutation

### FUSE mount smoke tests

- `ls`, `find`, `rg` do not list hidden entries
- `stat`, `cat`, `access` on hidden path fail with `ENOENT`
- bridge-visible ancestor is traversable but not writable
- bridge-visible directory listing only exposes bridge/visible descendants
- descendant-subtree carve-out(`**/.git/hooks/**` 등)도 existing ancestor directory bridge synthesis로 실제 도달 가능하다
- symlink entry는 resolved virtual target이 fully visible할 때만 `readlink`/dereference로 읽힌다. A raw symlink target that is lexically fully visible but escapes `source_root` may still be returned by `readlink`; dereference then fails in source-root confinement with `ENOENT`.
- mount root subtree is not visible through the mounted view
- `/bin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/home`, `/tmp`, `/var` are visible when policy allows
- supported exact / recursive / direct-child / descendant-subtree inputs smoke separately
- unsupported wildcard forms fail fast without partial interpretation
- readonly visible mutation returns `EROFS`
- writable carve-out visible mutation succeeds when host allows it

### System smoke tests

- confirm `fusermount3` path and version
- confirm `/dev/fuse` and kernel FUSE support
- confirm documented kernel config artifact still shows `CONFIG_FUSE_IO_URING=y` and `CONFIG_IO_URING=y`
- confirm same-uid access model or explicit `allow_other` policy
- `chroot` smoke only when privilege/user namespace model is available
- large traversal with many rules while checking latency, RSS, and fd-count targets, and confirming bridge-visible reachability does not depend on unbounded whole-root recursive scans
- daemon shutdown and `fusermount3 -u` cleanup behavior

## 18. Archival note

일부 transcript filename과 historical note에는 과거 naming이 남아 있을 수 있다. current policy surface는 항상 `visibility`/`mutability` 두 축으로 읽는다.
