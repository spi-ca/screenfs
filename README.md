# ScreenFS

`ScreenFS`는 non-root FUSE 기반 whole-filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 `ENOENT`처럼 숨기고, visible path에는 `visibility`/`mutability` 두 축 정책을 적용한다. 대표 통합 예시는 `pi-bash-sandbox` 같은 sandbox/chroot consumer지만, 특정 supervisor 전용 컴포넌트로 제한하지 않는다.

### 시스템 컨텍스트

![ScreenFS system context](docs/diagrams/system-context.svg)

다이어그램 원본: [docs/diagrams/README.md](docs/diagrams/README.md)

### 모듈 구조

![ScreenFS module architecture](docs/diagrams/module-architecture.svg)

다이어그램 원본: [docs/diagrams/README.md](docs/diagrams/README.md)

### 정책 축

![ScreenFS visibility axis](docs/diagrams/visibility-axis.svg)

![ScreenFS mutability axis](docs/diagrams/mutability-axis.svg)

다이어그램 원본: [docs/diagrams/README.md](docs/diagrams/README.md)

## 목적

ScreenFS의 기본 역할은 whole-root consumer가 읽을 수 있는 전체 `/` view를 만드는 것이다. FUSE request/reply transport는 `fractal-fuse = 0.4.0`의 `FUSE_OVER_IO_URING` 협상 경로를 필수로 사용하며, 협상 실패 시 fallback mount 없이 startup error로 실패해야 한다. 이 async 요구사항은 FUSE transport 경계에 한정되고, backing filesystem metadata/data path를 wholesale `io_uring`로 전환하는 것은 current scope가 아니다. 이미 열린 file handle의 read/write/copy_file_range/fallocate/fsync data path를 선택적으로 바꾸는 작업은 별도 benchmark-gated follow-up이며, metadata/path policy operation을 함께 전환하거나 recursive discovery를 추가하는 근거가 될 수 없다.

```text
real / -> screenfs mount root -> sandbox/chroot consumer
```

핵심 요약은 이 README를 기준으로 읽고, linked 문서는 세부 구현/검증 이력을 제공하는 참고 자료로 읽는다.

- 세부 요구사항: [`docs/requirements.md`](docs/requirements.md)
- 설계 계약: [`docs/design.md`](docs/design.md)
- 아키텍처 요약: [`docs/architecture.md`](docs/architecture.md)
- 운영/검증 evidence: [`docs/operations.md`](docs/operations.md)

문서가 설명하는 current contract는 two-axis surface와 shared rule normalization이다. shared matcher는 `visibility.hidden`/`mutability.readonly`/`mutability.writable`에서 prefixless recursive shorthand(`**/*.pem`), explicit root-anchor recursive form(`/**/*.pem`), recursive literal non-visible subtree canonical form(`**/.git/hooks/**`)과 recursive literal directory shorthand(`**/.git`, `**/.git/hooks`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks`, `~/repo/**/.git/hooks`, `~/**/aaa/hook`, `**/node_modules`, `**/target`, `**/dist`, `**/build`)를 계속 허용한다. 이 shorthand는 non-visible surface에서만 지원되며 `**/<literal-dir>` 또는 `<prefix>/**/<literal-tail>` 꼴만 허용한다. `/**/`는 최대 한 번만 쓸 수 있고 tail component는 모두 literal이어야 하며, 내부적으로 `<prefix>/**/<literal-tail>/**`로 normalize되어 matched directory 자체와 descendants를 함께 포함한다. 예를 들어 `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`다. 이것은 same-anchor subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이며 subtree shorthand 의미는 바뀌지 않는다. 반면 `visibility.visible`의 current contract는 discovery-free bridge만 허용하며 exact path, subtree(`/dir`, `/dir/**`), direct-child anchor bridge(`/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형)만 지원한다. `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks` 같은 recursive descendant visible glob/shorthand는 recursive bridge discovery가 필요하므로 shorthand와 canonical form 모두 unsupported/fail-fast다. `~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]`, `a*b`, `*secret*` 같은 multi-recursive 또는 broader wildcard form도 계속 fail-fast다. recursive literal directory shorthand 지원은 normalization/path-matcher-only delta여야 하며 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 추가하면 안 된다. visible listing/filtering fast path는 current supported grammar에만 적용되며, unsupported recursive visible form이나 broader wildcard를 추정해 근사하면 안 된다. bare slashless direct-child semantics와 shared recursive family, recursive literal directory shorthand, visible direct-child/subtree current baseline은 [`docs/operations.md`](docs/operations.md)가 정리한다.

## CLI quick reference

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--visibility-default <visible|hidden>] \
  [--hidden <pattern> ...] \
  [--visible <pattern> ...] \
  [--mutability-default <writable|readonly>] \
  [--readonly <pattern> ...] \
  [--writable <pattern> ...]
```

### Arguments

- `<source-root>`: backing filesystem root. whole-root view에는 보통 `/`를 사용한다.
- `<mount-root>`: ScreenFS view를 mount할 기존 디렉터리.

### Options

- `--config <path>`: YAML config를 로드한다.
- `--visibility-default <visible|hidden>`: visibility 축 기본값을 설정한다.
- `--hidden <pattern>`: path/glob를 숨겨 `ENOENT`처럼 보이게 한다.
- `--visible <pattern>`: hidden/default-hidden 영역 안쪽 path/glob를 다시 노출한다.
- `--mutability-default <writable|readonly>`: visible path의 기본 mutability를 설정한다.
- `--readonly <pattern>`: matching visible path mutation을 `EROFS`로 막는다.
- `--writable <pattern>`: matching visible path mutation을 허용한다.

CLI에서 한 축 옵션을 하나라도 주면 그 축의 config block 전체를 대체한다. visibility 축과 mutability 축은 서로 독립적으로 override된다. 제거된 family CLI option은 compatibility mapping 없이 unknown option으로 거부한다.

### Policy config quick reference

```yaml
visibility:
  default: visible | hidden
  hidden:
    - <pattern>
  visible:
    - <pattern>

mutability:
  default: writable | readonly
  readonly:
    - <pattern>
  writable:
    - <pattern>
```

### Two-axis policy summary

정책 축의 상세 흐름은 [visibility axis diagram](docs/diagrams/visibility-axis.svg)과 [mutability axis diagram](docs/diagrams/mutability-axis.svg)을 함께 읽는다.

- `visibility.default`는 rule 미매치 path의 기본 가시성을 정한다.
- `visibility.hidden`은 entry를 listing에서 제거하고 `lookup`/`getattr`/`open`/`access`/`readlink`를 `ENOENT`로 만든다.
- `visibility.visible`은 hidden-by-default allowlist이거나 hidden 영역 내부 carve-out이다.
- current `visibility.visible` 범주는 exact/subtree(`/dir`, `/dir/**`)와 direct-child anchor bridge(`/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형)뿐이다.
- subtree visible rule은 visible target까지의 정적 ancestor chain만 bridge-visible로 만든다.
- direct-child visible rule은 normalized anchor의 ancestor들만 bridge-visible candidate가 되며, anchor 바로 아래 immediate child match는 `lookup`/`readdir`/`readdirplus` 시점에 현재 directory/parent 기준으로만 평가한다. 그 directory와 무관한 matcher bucket은 건너뛰고, anchor subtree를 재귀 스캔하거나 background index·listing 결과 cache를 계약으로 요구하지 않는다.
- recursive descendant visible glob/shorthand(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks`)은 recursive bridge discovery가 필요하므로 current contract에서 shorthand와 canonical recursive literal form 모두 unsupported/fail-fast다. 같은 recursive family와 recursive literal directory shorthand는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 계속 사용할 수 있다.
- visibility fast path는 compiled visibility policy가 해당 entry와 resolved target을 숨기거나 bridge-visible/non-fully-visible로 만들 수 없음을 증명할 때만 target 재검사를 생략할 수 있다. 그렇지 않으면 listing/`lookup`/`getattr`/`readlink`/dereference/`open` 시점마다 multi-hop symlink와 ancestor symlink를 반영한 resolved final virtual target visibility를 다시 확인해야 한다.
- bridge-visible ancestor는 `stat`/`lookup`/`getattr`/읽기 의도 `access`/`opendir`/`readdir`/`readdirplus`/traversal만 허용하고 mutation은 `EROFS`다.
- hidden path와 fully visible이 아닌 symlink target(숨겨졌거나 bridge-visible인 target)은 mutability보다 먼저 처리되며 항상 `ENOENT`가 우선한다.
- mutability fast path는 visibility 증명 수단이 아니다. `mutability.default=writable`에서는 readonly match 불가가 보여도, `mutability.default=readonly`에서는 writable carve-out 불가로 즉시 `EROFS`를 내릴 수 있어도, hidden/non-fully-visible 가능성이 남아 있으면 resolved target visibility를 먼저 다시 확인해야 한다.
- single-request 안에서는 이미 구한 resolved final target을 visibility/mutability 단계가 재사용할 수 있지만, prior listing success·cross-request direct-path memo·symlink decision cache는 point-of-use 면제 근거가 아니다.
- bridge-visible directory는 hidden sibling을 노출하지 않으며 visible descendant로 이어지는 entry만 보여준다.
- 각 축에서는 가장 구체적인 매치가 이기고, 같은 축의 반대 rule이 같은 normalized anchor/specificity에서 충돌하면 invalid configuration이다.
- 이 목표 계약에는 제거된 CLI/config surface를 위한 compatibility mapping이나 shim이 없다.

### Integration mapping

외부 sandbox나 chroot supervisor는 자체 read/write allow/deny 모델을 ScreenFS의 두 축으로 변환할 수 있다. 읽기/존재 노출 정책은 `visibility.*`로, mutation 정책은 `mutability.*`로 내린 뒤 ScreenFS에는 current policy config만 전달한다.

`pi-bash-sandbox` 스타일 설정의 권장 매핑:

- `denyRead` → `visibility.hidden`
- `allowRead` → `visibility.visible` carve-out, 또는 `visibility.default=visible`에서 별도 rule 없음
- `allowWrite` → `mutability.writable`
- `denyWrite` → `mutability.readonly`

bare slashless pattern(`*.pem`, `*.key`, `.env.*`, `id_*`)은 ScreenFS가 macOS sandbox-runtime matcher 전체를 재현한다는 뜻이 아니라, 그런 supervisor가 내보내는 bare basename-oriented input shape를 보존하기 위해 launch process cwd를 `source_root` 기준으로 rebase한 `./<pattern>` direct-child rule로 컴파일하는 current contract다. anchored direct-child wildcard-all(`/dir/*`, `./dir/*`, `~/dir/*`)과 direct-child basename-prefix/suffix form(`/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`)도 같은 direct-child family다. same-anchor subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)은 `/dir`, `./dir`, `~/dir`와 정확히 동등하고, `~/aa/**`=`~/aa` 의미도 그대로다. 반면 recursive literal directory shorthand는 non-visible surface에서만 지원되는 별도 문법이며 `**/<literal-dir>` 또는 `<prefix>/**/<literal-tail>` 꼴만 허용한다. `/**/`는 최대 한 번만 쓸 수 있고 tail component는 모두 literal이어야 하며, 내부적으로 `<prefix>/**/<literal-tail>/**`로 정규화된다. 예를 들어 `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`다. 이 shorthand는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 지원되며 matched directory 자체와 descendants를 함께 포함하고 same-specificity conflict, containment, dedup 결과도 canonical form과 완전히 같다. 반면 `visibility.visible`은 `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks` 같은 recursive descendant form을 shorthand와 canonical form 모두 허용하지 않는다. 이런 rule은 recursive bridge discovery가 필요하므로 fail-fast 해야 하며, visible carve-out이 목적이면 `/dir` 또는 `/dir/**` 같은 explicit subtree rule을 생성해야 한다. `~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]`, `a*b`, `*secret*` 같은 multi-recursive 또는 broader wildcard form도 fail-fast다. 이 shorthand 추가는 normalization/path-matcher-only여야 하고 기존 `**/.../**` recursive literal subtree rule과 같은 matcher cost를 유지해야 하며 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 도입하면 안 된다. 이 fast path는 current supported grammar에만 적용되며 unsupported recursive visible form이나 broader wildcard를 근사하기 위해 확장되면 안 된다. recursive family 자체는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서 계속 current다. 따라서 whole-root secret coverage가 목적이면 hidden/mutability 쪽에 `/**/*.pem`, `/**/*.key`, `/**/.env.*`, `/**/id_*`, `/home/me/**/*.pem`, `**/.git/hooks`, `**/node_modules` 같은 recursive non-visible form을 내보내고, visible carve-out은 subtree/direct-child 범주로 제한하는 편이 안전하다. unanchored `*`와 `**/*`는 계속 unsupported/fail-fast다.

대표 4-family glob target 비교(`**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`)와 bare `*.pem` direct-child containment/specificity는 shared matcher와 non-visible recursive family의 current baseline으로 [`docs/requirements.md`](docs/requirements.md) 및 [`docs/operations.md`](docs/operations.md)에서 다룬다. same-anchor containment는 `/dir/*.pem` ⊂ `/dir/*` ⊂ `/dir`=`/dir/**`로 읽는다. `**/*.pem`, `/dir/**/*.pem` 같은 recursive descendant glob은 `visibility.visible`에서는 current contract상 unsupported이고, hidden/readonly/writable surface에서만 recursive family로 남는다.

이 매핑은 제거된 ScreenFS CLI flag로 번역하지 않고, 아래 YAML 같은 two-axis config를 생성해야 한다.

### Examples

Use a YAML config file:

```bash
screenfs / /tmp/screenfs-root --config screenfs.yaml
```

Hide secrets while keeping the rest of `/` visible. Whole-root secret patterns should use explicit root-anchored or absolute recursive rules such as `/**/*.pem`; bare slashless supported globs are cwd-sensitive `./<pattern>` shorthands, and prefixless recursive `**/*.pem` is only a cwd-anchored recursive shorthand:

```yaml
visibility:
  default: visible
  hidden:
    - /home/me/.ssh
    - '/**/*.pem'
mutability:
  default: writable
```

Hide most paths, then reopen one subtree through bridge-visible ancestors:

```yaml
visibility:
  default: hidden
  visible:
    - /workspace
mutability:
  default: writable
```

Make the whole view read-only except selected writable carve-outs, then re-block a nested path:

```yaml
visibility:
  default: visible
mutability:
  default: readonly
  writable:
    - /tmp
  readonly:
    - /tmp/locked
```

## 문서 안내

- 문서 인덱스: [`docs/README.md`](docs/README.md)
- 요구사항: [`docs/requirements.md`](docs/requirements.md)
- 설계: [`docs/design.md`](docs/design.md)
- 아키텍처 요약: [`docs/architecture.md`](docs/architecture.md)
- 운영/검증: [`docs/operations.md`](docs/operations.md)
- Pi workflow 문서: [`docs/pi-agents.md`](docs/pi-agents.md)
- 다이어그램 렌더링 계약: [`docs/diagrams/README.md`](docs/diagrams/README.md)

## Evidence

recorded verification/evidence의 세부 상태는 [`docs/operations.md`](docs/operations.md)를 따른다. 문서상 목표 계약과 recorded artifact 상태를 구분해 읽고, current contract evidence만 baseline으로 사용한다.

Current baseline artifacts:

- repo-local bare slashless cwd-anchor direct-child smoke baseline: [`docs/artifacts/current-bare-basename-glob-smoke-transcript.md`](docs/artifacts/current-bare-basename-glob-smoke-transcript.md) (`*.pem`=`./*.pem` direct-child baseline)
- whole-root/chroot smoke baseline: [`docs/artifacts/current-whole-root-chroot-smoke-transcript.md`](docs/artifacts/current-whole-root-chroot-smoke-transcript.md)
- default-writable smoke baseline: [`docs/artifacts/current-default-writable-smoke-transcript.md`](docs/artifacts/current-default-writable-smoke-transcript.md)
- visible direct-child/subtree compatibility smoke baseline: [`docs/artifacts/current-compatibility-pattern-smoke-transcript.md`](docs/artifacts/current-compatibility-pattern-smoke-transcript.md) (`/dir/*`, `./dir/*`, `~/dir/*`, `/dir/**`, `./dir/**`, `~/dir/**`, plus `*` and `**/*` fail-fast)


## 프로젝트 가드레일

- non-root + `fusermount3` 기준을 유지한다.
- whole-root consumer를 위한 전체 `/` view 요구를 유지한다.
- hidden path는 `readdir`/`readdirplus`에서 제외되고 `lookup`/`getattr`/`open`/`access`는 `ENOENT`여야 한다.
- 이름을 남기는 overlay masking(`/dev/null` bind, 빈 파일, tmpfs 등)이나 privileged mount 의존 설계로 바꾸지 않는다.
