# Documentation

이 디렉터리는 ScreenFS의 계약, 설계, 운영 evidence를 분리해 관리한다. 현재 정책 모델은 `visibility`/`mutability` 두 축이며, launch-cwd anchored glob normalization contract(`*.pem` direct-child, `**/*.pem` recursive shorthand 포함)을 다룬다. bare slashless direct-child semantics의 최신 source/live evidence는 `operations.md`와 `artifacts/current-bare-basename-glob-smoke-transcript.md`에 정리되어 있고, prefixless recursive cwd-anchor semantics와 explicit root-anchor distinction(`/**/*.pem`)의 current source evidence는 `operations.md`가 다룬다. 빠른 진입점은 아래 순서를 권장한다.

## Core documents

- [Requirements](requirements.md) — 제품 목적, whole-root 제약, two-axis contract
- [Design](design.md) — visibility/bridge-visible/mutability 의미론과 구현 계약
- [Architecture](architecture.md) — 코드 경계, 데이터 흐름, 다이어그램 요약
- [Operations](operations.md) — recorded build/test/live-smoke evidence와 검증 체크리스트
- [Benchmarks](benchmarks.md) — formal performance benchmark workflow and result recording rules

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
- 이 목표 계약에는 제거된 CLI/config surface를 위한 compatibility mapping이나 shim이 없다.

## Integration mapping summary

외부 supervisor의 read/write policy는 ScreenFS current surface에 들어오기 전에 `visibility.*`와 `mutability.*` 두 축으로 정규화한다. ScreenFS 문서와 CLI/config는 이 두 축만 canonical contract로 다룬다.

## Supporting references

- [Pi role agents](pi-agents.md) — repo-local Pi workflow, role, resource 설명
- [Diagram artifacts README](diagrams/README.md) — Mermaid source of truth와 SVG/PNG 렌더링 계약
- [Benchmarks](benchmarks.md) — ScreenFS/native comparative performance benchmark harness

## Evidence artifacts

Transcript artifact는 current contract baseline만 나열한다. 목표 계약 reference와 baseline 분류의 source of truth는 루트 [README](../README.md)와 [operations.md](operations.md)다.

Current baseline:

- [Current bare basename glob smoke](artifacts/current-bare-basename-glob-smoke-transcript.md) — bare direct-child baseline (`*.pem`=`./*.pem`)
- [Current whole-root/chroot smoke](artifacts/current-whole-root-chroot-smoke-transcript.md)
- [Current default-writable smoke](artifacts/current-default-writable-smoke-transcript.md)
- [Current visible direct-child/subtree compatibility smoke](artifacts/current-compatibility-pattern-smoke-transcript.md) — `/dir/*`, `./dir/*`, `~/dir/*`, `/dir/**`, `./dir/**`, `~/dir/**`, `*`, `**/*` fail-fast current baseline
