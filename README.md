# ScreenFS

`ScreenFS`는 non-root FUSE 기반 whole-root filesystem view layer다. 실제 `/`를 그대로 통과시키되, 민감한 경로는 존재하지 않는 것처럼 숨기고(`ENOENT`), 보이는 경로에는 읽기/쓰기 정책을 적용한다.

대표 통합 예시는 `pi-bash-sandbox` 같은 sandbox/chroot consumer지만, 특정 supervisor 전용 컴포넌트는 아니다.

```text
real / -> screenfs mount root -> sandbox/chroot consumer
```

## 핵심 계약

- **Whole-root view**: 프로젝트 디렉터리만이 아니라 전체 `/` view를 제공한다.
- **Non-root FUSE3**: `fusermount3`와 `fractal-fuse = 0.4.0`의 `FUSE_OVER_IO_URING` 협상 경로를 기준으로 한다. 협상 실패 시 fallback 없이 startup error로 실패한다.
- **File-handle sync cleanup boundary**: `flush`/`fsync`/`release(flush)`의 runtime blocking-offload(`compio_runtime::spawn_blocking` 또는 승인된 동등 surface)는 이미 열린 file handle의 blocking sync syscall 실행 위치를 state lock 밖 blocking pool로 옮기는 low-risk concurrency cleanup이다. host-side `io_uring` 전환이나 benchmark-gated data-path redesign이 아니며, `read`/`write`는 `FileExt::read_at`/`write_at`를 유지한다.
- **Visibility axis**: 숨겨진 경로는 directory listing에서 제외되고, 직접 접근은 `ENOENT`다.
- **Mutability axis**: 보이는 경로에 대해 writable/readonly 정책을 적용한다. readonly mutation은 `EROFS`다.
- **Precedence**: hidden `ENOENT`가 mutability `EROFS`보다 항상 우선한다.
- **No masking overlay**: `/dev/null` bind, 빈 파일, tmpfs masking처럼 이름을 남기는 방식이나 privileged mount에 의존하지 않는다.

세부 계약은 아래 문서를 기준으로 한다.

- 요구사항: [`docs/requirements.md`](docs/requirements.md)
- 설계 계약: [`docs/design.md`](docs/design.md)
- 아키텍처 요약: [`docs/architecture.md`](docs/architecture.md)
- 운영/검증 evidence: [`docs/operations.md`](docs/operations.md)

## 구조 한눈에 보기

### 시스템 컨텍스트

![ScreenFS system context](docs/diagrams/system-context.svg)

### 모듈 구조

![ScreenFS module architecture](docs/diagrams/module-architecture.svg)

### 정책 축

![ScreenFS visibility axis](docs/diagrams/visibility-axis.svg)

![ScreenFS mutability axis](docs/diagrams/mutability-axis.svg)

다이어그램 원본과 렌더링 규칙: [`docs/diagrams/README.md`](docs/diagrams/README.md)

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
- `--visibility-default <visible|hidden>`: visibility 기본값을 설정한다.
- `--hidden <pattern>`: path/glob을 숨겨 `ENOENT`처럼 보이게 한다.
- `--visible <pattern>`: hidden/default-hidden 영역 안쪽 path/glob을 다시 노출한다.
- `--mutability-default <writable|readonly>`: visible path의 기본 mutability를 설정한다.
- `--readonly <pattern>`: matching visible path mutation을 `EROFS`로 막는다.
- `--writable <pattern>`: matching visible path mutation을 허용한다.

CLI에서 한 축 옵션을 하나라도 주면 그 축의 config block 전체를 대체한다. visibility와 mutability는 서로 독립적으로 override된다. 제거된 family CLI option은 compatibility mapping 없이 unknown option으로 거부한다.

## Policy config quick reference

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

## 정책 의미론 요약

- `visibility.default`는 rule 미매치 path의 기본 가시성을 정한다.
- `visibility.hidden`은 entry를 listing에서 제거하고 `lookup`/`getattr`/`open`/`access`/`readlink`를 `ENOENT`로 만든다.
- `visibility.visible`은 hidden-by-default allowlist 또는 hidden 영역 내부 carve-out이다.
- `visibility.visible`은 exact/subtree(`/dir`, `/dir/**`)와 direct-child anchor bridge(`/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형)만 지원한다.
- visible subtree carve-out은 target까지의 ancestor를 bridge-visible로 만들 수 있다.
- bridge-visible ancestor는 stat/traverse/list만 제한적으로 허용하고 mutation은 `EROFS`다.
- bridge-visible directory는 hidden sibling을 노출하지 않고, visible descendant로 이어지는 entry만 보여준다.
- 각 축은 **most-specific rule wins**를 따른다. 같은 축의 반대 rule이 같은 normalized anchor/specificity에서 충돌하면 invalid configuration이다.
- hidden path와 fully visible이 아닌 symlink target은 mutability보다 먼저 처리되어 `ENOENT`가 우선한다.
- listing 성공, cross-request direct-path memo, symlink decision cache는 point-of-use visibility check 면제 근거가 아니다.

패턴 문법과 fast-path 조건은 [`docs/operations.md`](docs/operations.md)의 current contract를 따른다. 특히 recursive descendant visible glob/shorthand는 recursive bridge discovery가 필요하므로 fail-fast이며, recursive family는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 지원된다. `*`, `**/*`, `a*b` 같은 broader wildcard도 unsupported/fail-fast다.

## Integration mapping

외부 sandbox나 chroot supervisor는 자체 read/write allow/deny 모델을 ScreenFS의 두 축으로 변환한다.

- `denyRead` → `visibility.hidden`
- `allowRead` → `visibility.visible` carve-out 또는 `visibility.default=visible`에서 rule 없음
- `allowWrite` → `mutability.writable`
- `denyWrite` → `mutability.readonly`

ScreenFS에는 제거된 legacy flag가 아니라 현재 two-axis YAML/CLI surface만 전달해야 한다.

## Examples

### YAML config 사용

```bash
screenfs / /tmp/screenfs-root --config screenfs.yaml
```

### 대부분은 보이게 두고 secret 숨기기

Whole-root secret pattern은 `/**/*.pem`처럼 root-anchored recursive rule을 사용한다. `**/*.pem`은 `/**/*.pem`과 다르며 launch cwd 기준 recursive shorthand다. bare slashless glob은 cwd-sensitive `./<pattern>` direct-child shorthand다.

```yaml
visibility:
  default: visible
  hidden:
    - /home/me/.ssh
    - '/**/*.pem'
mutability:
  default: writable
```

### 대부분 숨기고 한 subtree만 다시 열기

```yaml
visibility:
  default: hidden
  visible:
    - /workspace
mutability:
  default: writable
```

### 전체 readonly + 일부 writable carve-out + nested re-block

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

Recorded verification/evidence의 세부 상태는 [`docs/operations.md`](docs/operations.md)를 따른다. 문서상 목표 계약과 recorded artifact 상태를 구분하고, current contract evidence만 baseline으로 사용한다.

Current baseline artifacts:

- [`docs/artifacts/current-bare-basename-glob-smoke-transcript.md`](docs/artifacts/current-bare-basename-glob-smoke-transcript.md): `*.pem`=`./*.pem` direct-child baseline
- [`docs/artifacts/current-whole-root-chroot-smoke-transcript.md`](docs/artifacts/current-whole-root-chroot-smoke-transcript.md): whole-root/chroot smoke baseline
- [`docs/artifacts/current-default-writable-smoke-transcript.md`](docs/artifacts/current-default-writable-smoke-transcript.md): default-writable smoke baseline
- [`docs/artifacts/current-compatibility-pattern-smoke-transcript.md`](docs/artifacts/current-compatibility-pattern-smoke-transcript.md): visible direct-child/subtree compatibility baseline
- [`docs/artifacts/current-file-data-path-async-feasibility.md`](docs/artifacts/current-file-data-path-async-feasibility.md): already-open file-handle data-path async/io_uring scope와 `flush`/`fsync`/`release(flush)` sync cleanup boundary
- [`docs/artifacts/current-state-lock-concurrency-evidence.md`](docs/artifacts/current-state-lock-concurrency-evidence.md): state lock 바깥 sync syscall 실행 규칙과 low-risk executor-offload 범위
