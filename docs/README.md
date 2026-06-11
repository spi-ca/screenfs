# Documentation

이 디렉터리는 ScreenFS의 계약, 설계, 운영 evidence를 분리해 관리한다. 빠른 진입점은 아래 순서를 권장한다.

## Core documents

- [Requirements](requirements.md) — 제품 목적, hidden `ENOENT`, whole-root view, mutability family 요구사항
- [Design](design.md) — current implementation snapshot과 남은 설계 계약
- [Architecture](architecture.md) — 코드 경계, 데이터 흐름, 다이어그램 요약
- [Operations](operations.md) — 최신 recorded build/test/live-smoke evidence와 운영 checklist
- [Nested mutability option B](nested-mutability-option-b.md) — nested override 채택 배경과 canonical contract

## Supporting references

- [Pi role agents](pi-agents.md) — repo-local Pi workflow, role, resource 설명
- [Diagram artifacts README](diagrams/README.md) — Mermaid source of truth와 SVG/PNG 렌더링 계약

## Evidence artifacts

- [Repo-local family-aware smoke transcript](artifacts/future-mutability-smoke-transcript.md)
- [Whole-root family smoke transcript](artifacts/whole-root-family-smoke-transcript.md)
- [Whole-mount readonly smoke transcript](artifacts/whole-mount-readonly-smoke-transcript.md)
- [Pre-removal archival smoke transcript](artifacts/fuse-smoke-transcript.md)
