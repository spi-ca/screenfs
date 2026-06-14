# AGENTS.md

이 저장소에서 작업하는 에이전트는 아래 공통 규칙을 따른다.

## 프로젝트 목적

`ScreenFS`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 전체 `/`를 pass-through 하면서 visibility axis로 민감 경로를 존재하지 않는 것처럼 숨기고, visible path에는 mutability axis로 readonly/writable policy를 적용한다. `pi-bash-sandbox`는 대표 통합 예시지만 프로젝트 목적을 그 용도 하나로 한정하지 않는다.

## 필수 가드레일

- 기본 실행 전제는 non-root다.
- FUSE mount는 `fusermount3`를 기준으로 문서화하고 구현한다.
- root-only mount, privileged bind mount, system-wide mount namespace 조작에 의존하지 않는다.
- `/dev/null` bind overlay, 빈 파일 overlay, tmpfs masking처럼 이름을 남기는 masking 방식은 사용하지 않는다.
- chroot 사용을 전제로 하므로 프로젝트 디렉터리 전용 view가 아니라 전체 `/` view 요구사항을 유지한다.
- `fractal-fuse = 0.4.0`, FUSE3, `FUSE_OVER_IO_URING` 목표를 임의로 바꾸지 않는다.
- hidden path는 가능한 한 `ENOENT`로 처리하고 `Permission denied`로 존재를 노출하지 않는다.
- current contract는 `visibility`/`mutability` 2축 모델만 취급하고, hidden `ENOENT` precedence와 axis별 most-specific rule wins 규칙을 유지한다.
- 현재 CLI/config surface와 historical pre-removal evidence를 혼동하지 않는다. pre-removal transcript는 archival evidence로만 읽는다.
- `docs/guidelines/**`는 명시적 요청 없이는 수정하지 않는다.
- 사용자의 기존 변경사항을 덮어쓰지 말고 변경 전후 diff를 확인한다.

## 필수 의미론

- hidden entry는 `readdir`, `readdirplus` 결과에서 제외된다.
- hidden path에 대한 `lookup`, `getattr`, `open`, `access`는 `ENOENT`다.
- `visibility.hidden` path는 `readdir`/`readdirplus`에서 제외되고 직접 접근은 `ENOENT`다.
- `visibility.visible` carve-out까지의 ancestor는 bridge-visible일 수 있으며 stat/traverse/list만 제한적으로 허용하고 mutation은 `EROFS`다.
- `mutability.default=readonly`에서 `mutability.writable` carve-out이 없는 visible mutation은 `EROFS`다.
- `mutability.default=writable`에서 `mutability.readonly` re-block에 매치된 visible mutation은 `EROFS`다.
- nested override를 포함한 양 축 모두에서 hidden `ENOENT`가 mutability `EROFS`보다 우선한다.

## 추가 문서

- Pi role workflow와 repo-local resource 설명: [`docs/pi-agents.md`](docs/pi-agents.md)
- 다이어그램 렌더링 계약: [`docs/diagrams/README.md`](docs/diagrams/README.md)
- 운영/검증 기준: [`docs/operations.md`](docs/operations.md)

## 검증 포인터

문서 변경 시 최소 확인:

```bash
cargo check
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts .pi/extensions -maxdepth 3 -type f -print | sort
```

코드 변경이 포함되면 추가 확인:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```

`docs/diagrams/*.mmd` 또는 Mermaid render config를 바꿨다면 `docs/diagrams/README.md` 규칙에 따라 대응 `*.svg`, `*.png`를 함께 재생성하고 PNG에는 Mermaid CLI `--scale 2`를 포함한다.
