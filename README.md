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

ScreenFS의 기본 역할은 whole-root consumer가 읽을 수 있는 전체 `/` view를 만드는 것이다.

```text
real / -> screenfs mount root -> sandbox/chroot consumer
```

핵심 요약은 이 README를 기준으로 읽고, linked 문서는 세부 구현/검증 이력을 제공하는 참고 자료로 읽는다.

- 세부 요구사항: [`docs/requirements.md`](docs/requirements.md)
- 설계 계약: [`docs/design.md`](docs/design.md)
- 아키텍처 요약: [`docs/architecture.md`](docs/architecture.md)
- 운영/검증 evidence: [`docs/operations.md`](docs/operations.md)

문서가 설명하는 two-axis surface와 launch-cwd anchored glob normalization contract(`*.pem` direct-child, `/dir/*` anchored direct-child wildcard-all, `**/*.pem` recursive shorthand, `/dir/**` subtree shorthand 포함)은 현재 목표 계약이다. bare slashless `./<pattern>` direct-child 해석의 최신 source/live evidence는 [`docs/operations.md`](docs/operations.md)와 [`docs/artifacts/current-bare-basename-glob-smoke-transcript.md`](docs/artifacts/current-bare-basename-glob-smoke-transcript.md)에 정리되어 있다. prefixless recursive shorthand(`**/*.pem`, `**/.env.*`, `**/id_*`)의 cwd-anchor semantics와 explicit root-anchor distinction(`/**/*.pem`)을 포함한 current source/live evidence는 [`docs/operations.md`](docs/operations.md)와 [`docs/artifacts/current-four-family-glob-smoke-transcript.md`](docs/artifacts/current-four-family-glob-smoke-transcript.md)에 정리되어 있다. goal 612c03c6에서 추가된 anchored direct-child wildcard-all(`/dir/*`, `./dir/*`, `~/dir/*`)과 trailing subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)의 source/live evidence는 [`docs/operations.md`](docs/operations.md)와 [`docs/artifacts/current-compatibility-pattern-smoke-transcript.md`](docs/artifacts/current-compatibility-pattern-smoke-transcript.md)에 정리되어 있다.

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

CLI에서 한 축 옵션을 하나라도 주면 그 축의 config block 전체를 대체한다. visibility 축과 mutability 축은 서로 독립적으로 override된다. 제거된 family-era CLI option은 compatibility alias 없이 unknown option으로 거부한다.

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
- visible descendant에 도달시키기 위해 필요한 ancestor directory는 bridge-visible이 될 수 있으며, `**/.git/hooks/**` 같은 descendant-subtree carve-out도 경로상의 모든 existing ancestor directory를 bridge-visible로 합성해야 한다.
- bridge-visible ancestor는 `stat`/`lookup`/`getattr`/읽기 의도 `access`/`opendir`/`readdir`/`readdirplus`/traversal만 허용하고 mutation은 `EROFS`다.
- `mutability.default`는 visible path의 기본 쓰기 가능 여부를 정한다.
- `mutability.readonly`와 `mutability.writable`은 같은 축 안에서 more-specific override를 만든다.
- hidden path와 fully visible이 아닌 symlink target(숨겨졌거나 bridge-visible인 target)은 mutability보다 먼저 처리되며 항상 `ENOENT`가 우선한다.
- bridge-visible reachability는 bounded/cacheable하게 유지해야 하며 hot path에서 unbounded whole-root recursive scan에 의존하지 않아야 한다.
- 각 축에서는 가장 구체적인 매치가 이기고, 같은 축의 반대 rule이 같은 normalized anchor/specificity에서 충돌하면 invalid configuration이다.
- 이 목표 계약에는 제거된 예전 family/flag surface를 위한 compatibility alias나 shim이 없다.

### Integration mapping

외부 sandbox나 chroot supervisor는 자체 read/write allow/deny 모델을 ScreenFS의 두 축으로 변환할 수 있다. 읽기/존재 노출 정책은 `visibility.*`로, mutation 정책은 `mutability.*`로 내린 뒤 ScreenFS에는 current two-axis config만 전달한다.

`pi-bash-sandbox` 스타일 설정의 권장 매핑:

- `denyRead` → `visibility.hidden`
- `allowRead` → `visibility.visible` carve-out, 또는 `visibility.default=visible`에서 별도 rule 없음
- `allowWrite` → `mutability.writable`
- `denyWrite` → `mutability.readonly`

bare slashless pattern(`*.pem`, `*.key`, `.env.*`, `id_*`)은 ScreenFS가 macOS sandbox-runtime matcher 전체를 재현한다는 뜻이 아니라, 그런 supervisor가 내보내는 bare basename-oriented input shape를 보존하기 위해 launch process cwd를 `source_root` 기준으로 rebase한 `./<pattern>` direct-child rule로 컴파일하는 current contract다. recursive prefixless pattern(`**/*.pem`, `**/.env.*`, `**/id_*`)은 같은 cwd anchor를 재귀로 확장한 `./**/<pattern>` shorthand target이다. anchored direct-child wildcard-all(`/dir/*`, `./dir/*`, `~/dir/*`)은 normalized anchor directory의 모든 immediate child와 그 descendants에 적용되지만 anchor 자체에는 적용되지 않는다. trailing subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)은 같은 anchor의 subtree rule(`/dir`, `./dir`, `~/dir`)와 정확히 동등하며 별도 recursive-any family를 만들지 않는다. 따라서 cwd가 `source_root` 밖이면 bare/prefixless shorthand는 fail-fast 해야 하고, whole-root secret coverage가 목적이면 bare/prefixless cwd-sensitive form 대신 `/**/*.pem`, `/**/*.key`, `/**/.env.*`, `/**/id_*`, `/home/me/**/*.pem` 같은 explicit root-anchored/absolute recursive form을 내보내는 편이 안전하다. unanchored `*`와 `**/*`는 계속 unsupported/fail-fast다.

대표 4-family glob target 비교(`**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`)와 bare `*.pem` direct-child containment/specificity는 [`docs/requirements.md`](docs/requirements.md)의 canonical table과 [`docs/operations.md`](docs/operations.md)의 status note를 따른다. same-anchor containment는 `/dir/*.pem` ⊂ `/dir/*` ⊂ `/dir`=`/dir/**`로 읽고, `/dir/*`와 `/dir/**/*.pem`처럼 containment를 증명할 수 없는 opposite-polarity overlap은 계속 fail-fast다. [`docs/artifacts/current-four-family-glob-smoke-transcript.md`](docs/artifacts/current-four-family-glob-smoke-transcript.md)는 goal 38ca1993 기준 current live baseline이며, `**/*.pem`이 prefixless recursive cwd-anchor shorthand이고 `/**/*.pem`과는 다른 explicit root-anchor family임을 함께 보여준다. 새 anchored forms의 source/live evidence는 `docs/artifacts/current-compatibility-pattern-smoke-transcript.md`와 source tests가 제공한다.

이 매핑은 legacy ScreenFS CLI flag로 번역하지 않고, 아래 YAML 같은 two-axis config를 생성해야 한다.

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

recorded verification/evidence의 세부 상태는 [`docs/operations.md`](docs/operations.md)를 따른다. 문서상 목표 계약과 recorded artifact 상태를 구분해 읽는다.

Current baseline artifacts:

- repo-local two-axis smoke baseline: [`docs/artifacts/current-two-axis-smoke-transcript.md`](docs/artifacts/current-two-axis-smoke-transcript.md)
- repo-local bare slashless cwd-anchor direct-child smoke baseline: [`docs/artifacts/current-bare-basename-glob-smoke-transcript.md`](docs/artifacts/current-bare-basename-glob-smoke-transcript.md) (`*.pem`=`./*.pem` direct-child baseline)
- whole-root/chroot smoke baseline: [`docs/artifacts/current-whole-root-chroot-smoke-transcript.md`](docs/artifacts/current-whole-root-chroot-smoke-transcript.md)
- default-writable smoke baseline: [`docs/artifacts/current-default-writable-smoke-transcript.md`](docs/artifacts/current-default-writable-smoke-transcript.md)
- anchored dynamic visible-glob whole-root startup baseline: [`docs/artifacts/current-dynamic-whole-root-smoke-transcript.md`](docs/artifacts/current-dynamic-whole-root-smoke-transcript.md)
- canonical 4-family glob repo-local smoke baseline: [`docs/artifacts/current-four-family-glob-smoke-transcript.md`](docs/artifacts/current-four-family-glob-smoke-transcript.md) (`**/*.pem` cwd-anchor recursive shorthand, `/**/*.pem` explicit root-anchor distinction, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt` current live baseline)
- compatibility path-pattern smoke baseline: [`docs/artifacts/current-compatibility-pattern-smoke-transcript.md`](docs/artifacts/current-compatibility-pattern-smoke-transcript.md) (`/dir/*`, `./dir/*`, `~/dir/*`, `/dir/**`, `./dir/**`, `~/dir/**`, plus `*` and `**/*` fail-fast)

Historical legacy artifacts:

- whole-root smoke transcript: [`docs/artifacts/whole-root-family-smoke-transcript.md`](docs/artifacts/whole-root-family-smoke-transcript.md)
- whole-mount readonly smoke transcript: [`docs/artifacts/whole-mount-readonly-smoke-transcript.md`](docs/artifacts/whole-mount-readonly-smoke-transcript.md)
- repo-local smoke transcript: [`docs/artifacts/future-mutability-smoke-transcript.md`](docs/artifacts/future-mutability-smoke-transcript.md)
- pre-removal smoke transcript: [`docs/artifacts/fuse-smoke-transcript.md`](docs/artifacts/fuse-smoke-transcript.md)

## 프로젝트 가드레일

- non-root + `fusermount3` 기준을 유지한다.
- whole-root consumer를 위한 전체 `/` view 요구를 유지한다.
- hidden path는 `readdir`/`readdirplus`에서 제외되고 `lookup`/`getattr`/`open`/`access`는 `ENOENT`여야 한다.
- 이름을 남기는 overlay masking(`/dev/null` bind, 빈 파일, tmpfs 등)이나 privileged mount 의존 설계로 바꾸지 않는다.
