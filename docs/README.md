# Documentation

이 디렉터리는 ScreenFS의 계약, 설계, 운영 evidence를 분리해 관리한다. 빠른 진입점은 아래 순서다.

## Core documents

- [Requirements](requirements.md): 목적, 전제, non-goals, 사용자-facing 요구사항
- [Design](design.md): visibility/mutability 의미론, rule grammar, operation contract
- [Architecture](architecture.md): 모듈 경계, request flow, runtime boundary
- [Operations](operations.md): 검증 명령, smoke/evidence 기록 규칙, current evidence map
- [Benchmarks](benchmarks.md): formal performance benchmark workflow
- [Performance roadmap](performance-roadmap.md): measure-first 성능 backlog와 deferred 후보

## Policy model

ScreenFS의 canonical surface는 두 축뿐이다.

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

- hidden path는 listing에서 빠지고 직접 접근은 `ENOENT`다.
- bridge-visible ancestor는 visible descendant에 도달하기 위한 traverse/list 전용 상태다.
- visible path mutation은 mutability 축으로 `writable` 또는 `readonly`를 판정한다.
- hidden `ENOENT`는 mutability `EROFS`보다 먼저 적용된다.
- 제거된 CLI/config surface를 위한 compatibility mapping이나 shim은 없다.

## Supporting references

- [Pi role agents](pi-agents.md): repo-local Pi workflow와 resource
- [Diagram artifacts README](diagrams/README.md): Mermaid source of truth와 SVG/PNG 렌더링 계약
- [Guidelines](guidelines/): agent-facing docs와 coding-agent 행동 지침

## Evidence artifacts

Transcript artifact는 current contract baseline만 나열한다. 목표 계약 reference와 baseline 분류의 source of truth는 [operations.md](operations.md)다.

Current baseline artifacts:

- [Bare basename glob smoke](artifacts/current-bare-basename-glob-smoke-transcript.md)
- [Whole-root/chroot smoke](artifacts/current-whole-root-chroot-smoke-transcript.md)
- [Default-writable smoke](artifacts/current-default-writable-smoke-transcript.md)
- [Visible direct-child/subtree compatibility smoke](artifacts/current-compatibility-pattern-smoke-transcript.md)
- [FUSE transport contract evidence](artifacts/current-fuse-transport-contract-evidence.md)
- [File data-path async feasibility](artifacts/current-file-data-path-async-feasibility.md)
- [State-lock concurrency evidence](artifacts/current-state-lock-concurrency-evidence.md)
- [TOCTOU hardening evidence](artifacts/current-toctou-hardening-evidence.md)
- [Perf-counter baseline summary](artifacts/current-perf-counter-baseline-summary.md)
- [Perf-counter benchmark result](artifacts/current-perf-counter-benchmark-result.md)
