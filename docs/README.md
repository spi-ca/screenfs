# Documentation

이 디렉터리는 ScreenFS의 계약, 설계, 운영 evidence를 분리해 관리한다. 현재 정책 모델은 `visibility`/`mutability` 두 축이다. 빠른 진입점은 아래 순서를 권장한다.

## Core documents

- [Requirements](requirements.md) — 제품 목적, whole-root 제약, two-axis contract
- [Design](design.md) — visibility/bridge-visible/mutability 의미론과 구현 계약
- [Architecture](architecture.md) — 코드 경계, 데이터 흐름, 다이어그램 요약
- [Operations](operations.md) — recorded build/test/live-smoke evidence와 검증 체크리스트

## Current policy model at a glance

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

- `visibility.hidden`은 hidden `ENOENT`를 만든다.
- `visibility.visible`은 hidden-by-default allowlist 또는 hidden 영역 내부 carve-out이다.
- `visibility.visible` descendant에 도달시키기 위해 필요한 ancestor directory는 bridge-visible이 될 수 있다.
- `mutability.readonly`와 `mutability.writable`은 visible path mutation만 조절한다.
- hidden 판정은 mutability보다 항상 먼저 적용된다.
- 현재 계약에는 예전 family/flag surface를 위한 compatibility alias나 shim이 없다.

## Integration mapping summary

외부 supervisor의 read/write policy는 ScreenFS current surface에 들어오기 전에 `visibility.*`와 `mutability.*` 두 축으로 정규화한다. ScreenFS 문서와 CLI/config는 이 두 축만 canonical contract로 다룬다.

## Supporting references

- [Pi role agents](pi-agents.md) — repo-local Pi workflow, role, resource 설명
- [Diagram artifacts README](diagrams/README.md) — Mermaid source of truth와 SVG/PNG 렌더링 계약

## Evidence artifacts

아래 transcript artifact는 모두 capture-time historical record다. current contract reference는 루트 [README](../README.md), [requirements.md](requirements.md), [design.md](design.md), [operations.md](operations.md)를 따른다. 새 two-axis surface의 live smoke transcript는 아직 별도 current baseline으로 승격하지 않는다.

Archival-only:

- [Whole-root smoke transcript](artifacts/whole-root-family-smoke-transcript.md) — historical whole-root carve-out baseline
- [Whole-mount readonly smoke transcript](artifacts/whole-mount-readonly-smoke-transcript.md) — historical whole-mount readonly baseline
- [Repo-local smoke transcript](artifacts/future-mutability-smoke-transcript.md) — repo-local historical baseline
- [Pre-removal smoke transcript](artifacts/fuse-smoke-transcript.md) — removed surface historical record
