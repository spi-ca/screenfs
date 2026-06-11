# ScreenFS

`ScreenFS`는 non-root FUSE 기반 whole-filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 `ENOENT`처럼 숨기고, visible path에는 family-aware mutability policy를 적용한다. 대표 통합 예시는 `pi-bash-sandbox` 같은 sandbox/chroot consumer지만, 특정 supervisor 전용 컴포넌트로 제한하지 않는다.

### 시스템 컨텍스트

![ScreenFS system context](docs/diagrams/system-context.svg)

다이어그램 원본: [docs/diagrams/README.md](docs/diagrams/README.md)

### 모듈 구조

![ScreenFS module architecture](docs/diagrams/module-architecture.svg)

다이어그램 원본: [docs/diagrams/README.md](docs/diagrams/README.md)

## 목적

ScreenFS의 기본 역할은 whole-root consumer가 읽을 수 있는 전체 `/` view를 만드는 것이다.

```text
real / -> screenfs mount root -> sandbox/chroot consumer
```

핵심 요구와 구현/검증 문서는 분리해서 읽는다.

- 제품/의미론 계약: [`docs/requirements.md`](docs/requirements.md)
- 설계와 current implementation snapshot: [`docs/design.md`](docs/design.md)
- 운영/검증 evidence: [`docs/operations.md`](docs/operations.md)
- nested override 배경: [`docs/nested-mutability-option-b.md`](docs/nested-mutability-option-b.md)

## CLI quick reference

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--hide <pattern> ...] \
  [--policy-family <selective-readonly|readonly-root-allowwrite>] \
  [--readonly-rule <pattern> ...] \
  [--allow-write <pattern> ...]
```

### Arguments

- `<source-root>`: backing filesystem root. whole-root view에는 보통 `/`를 사용한다.
- `<mount-root>`: ScreenFS view를 mount할 기존 디렉터리.

### Options

- `--config <path>`: YAML config를 로드한다. CLI mutability option을 하나라도 주면 config의 `mutability` block 전체를 대체한다.
- `--hide <pattern>`: exact path 또는 shared rule-input grammar의 지원 glob을 숨긴다. hidden path는 `ENOENT`처럼 보인다.
- `--policy-family <selective-readonly|readonly-root-allowwrite>`: mount당 하나의 mutability family를 선택한다.
- `--readonly-rule <pattern>`: `selective-readonly`의 primary readonly rule, 또는 `readonly-root-allowwrite`의 nested re-block rule.
- `--allow-write <pattern>`: `readonly-root-allowwrite`의 primary carve-out rule, 또는 `selective-readonly`의 nested writable carve-out rule.

`--hide`, `--readonly-rule`, `--allow-write`는 같은 rule-input grammar를 공유한다. 현재 소스와 mount-free tests는 exact path, 기존 recursive/direct-child limited glob, 그리고 one-or-more literal path component를 tail로 갖는 limited recursive literal descendant-subtree glob(`<normalized-prefix>/**/<literal-tail>/**`)까지 검증한다. 현재 source-test example: `/home/<user>/project/**/.git/hooks/**`.

current 구현 snapshot과 남아 있는 live-smoke evidence gap, fail-fast 범위, nested override specificity/ancestor 규칙은 [`docs/requirements.md`](docs/requirements.md), [`docs/design.md`](docs/design.md), [`docs/nested-mutability-option-b.md`](docs/nested-mutability-option-b.md), [`docs/operations.md`](docs/operations.md)를 함께 읽는다. descendant-subtree glob의 current source/unit-test support와 live smoke pending 범위를 구분해서 읽는다.

### Mutability summary

- hidden path는 mutability rule보다 먼저 판정되며 항상 `ENOENT`가 우선한다.
- `selective-readonly`: 기본 writable, `--readonly-rule` 매치 path만 `EROFS`.
- `readonly-root-allowwrite`: 기본 readonly, `--allow-write` 매치 path만 writable 후보.
- opposite-polarity rule을 함께 쓰는 nested override는 explicit `--policy-family`와 valid ancestor 관계가 필요하고, 판정은 most-specific-match-wins를 따른다.
- whole-mount readonly semantics는 `readonly-root-allowwrite` + empty `allow_write`로 표현한다.

세부 contract와 normalization/fail-fast 규칙은 [`docs/requirements.md`](docs/requirements.md), [`docs/design.md`](docs/design.md), [`docs/nested-mutability-option-b.md`](docs/nested-mutability-option-b.md)에 정리돼 있다.

### Examples

Hide secrets in a whole-root view with current supported forms:

```bash
screenfs / /tmp/screenfs-root \
  --hide /home/me/.ssh \
  --hide '**/*.pem'
```

Current source/unit tests also cover descendant-subtree forms such as `--readonly-rule '/home/me/project/**/.git/hooks/**'`. Live mount smoke for those forms is still tracked separately in the docs.

Make selected paths read-only:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family selective-readonly \
  --readonly-rule /etc/ssh
```

Make the whole view read-only except selected paths:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family readonly-root-allowwrite \
  --allow-write /tmp
```

Use a YAML config file:

```bash
screenfs / /tmp/screenfs-root --config screenfs.yaml
```

## 문서 안내

- 문서 인덱스: [`docs/README.md`](docs/README.md)
- 요구사항: [`docs/requirements.md`](docs/requirements.md)
- 설계: [`docs/design.md`](docs/design.md)
- 아키텍처 요약: [`docs/architecture.md`](docs/architecture.md)
- 운영/검증: [`docs/operations.md`](docs/operations.md)
- nested mutability option B: [`docs/nested-mutability-option-b.md`](docs/nested-mutability-option-b.md)
- Pi workflow 문서: [`docs/pi-agents.md`](docs/pi-agents.md)
- 다이어그램 렌더링 계약: [`docs/diagrams/README.md`](docs/diagrams/README.md)

## Evidence

최신 recorded verification/evidence는 [`docs/operations.md`](docs/operations.md)와 아래 artifact를 기준으로 읽는다.

- repo-local family-aware baseline: [`docs/artifacts/future-mutability-smoke-transcript.md`](docs/artifacts/future-mutability-smoke-transcript.md)
- whole-root carve-out baseline: [`docs/artifacts/whole-root-family-smoke-transcript.md`](docs/artifacts/whole-root-family-smoke-transcript.md)
- whole-mount readonly baseline: [`docs/artifacts/whole-mount-readonly-smoke-transcript.md`](docs/artifacts/whole-mount-readonly-smoke-transcript.md)
- pre-removal archival transcript: [`docs/artifacts/fuse-smoke-transcript.md`](docs/artifacts/fuse-smoke-transcript.md)

## 프로젝트 가드레일

- non-root + `fusermount3` 기준을 유지한다.
- whole-root consumer를 위한 전체 `/` view 요구를 유지한다.
- hidden path는 `readdir`/`readdirplus`에서 제외되고 `lookup`/`getattr`/`open`/`access`는 `ENOENT`여야 한다.
- 이름을 남기는 overlay masking(`/dev/null` bind, 빈 파일, tmpfs 등)이나 privileged mount 의존 설계로 바꾸지 않는다.
