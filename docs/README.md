# Documentation

이 디렉터리는 ScreenFS의 계약, 설계, 운영 evidence를 분리해 관리한다. 이 파일은 처음 읽는 사람이 문서, 저장소 구조, 코드 책임, visibility/mutability 흐름을 한 번에 찾기 위한 출발점이다.

## Project metadata

- Repository: <https://github.com/spi-ca/screenfs>
- License: BSD 3-Clause License. See [`../LICENSE`](../LICENSE).

## Reading path

처음 읽을 때는 아래 순서가 가장 빠르다.

1. [Project README](../README.md): 프로젝트 요약, CLI/config quick reference, examples
2. [Requirements](requirements.md): 목적, 전제, non-goals, 사용자-facing 요구사항
3. [Design](design.md): visibility/mutability 의미론, rule grammar, operation contract
4. [Architecture](architecture.md): 모듈 경계, request flow, runtime boundary
5. [Operations](operations.md): 검증 명령, smoke/evidence 기록 규칙, current evidence map
6. [Benchmarks](benchmarks.md): formal performance benchmark workflow
7. [Performance roadmap](performance-roadmap.md): measure-first 성능 backlog와 deferred 후보

## Repository map

```text
screenfs/
├── README.md        # project entrypoint, CLI/config quick reference
├── Cargo.toml       # Rust package, features, dependencies
├── AGENTS.md        # repo-local agent guardrails
├── src/             # ScreenFS library and binary source
├── docs/            # contract, architecture, operations, evidence docs
│   ├── artifacts/   # recorded smoke/benchmark/evidence outputs
│   └── diagrams/    # Mermaid sources and rendered SVG/PNG
├── scripts/         # benchmark/test helper scripts
├── contrib/         # supporting/comparison code
└── .pi/             # repo-local Pi agents, prompts, skills, settings
```

## Code map

| Area | Responsibility |
| --- | --- |
| `src/main.rs` | binary entrypoint, mount option 구성, `Session::run(ScreenFs::new(cfg))` 진입 |
| `src/lib.rs` | public module export, non-root 실행 guard |
| `src/cli.rs` | CLI parsing, config override flags, help/fail-fast surface |
| `src/config.rs` | runtime config, policy source/precedence, internal mount-root hidden rule, matcher compilation |
| `src/path.rs` | lexical virtual path normalization, source-root rebasing, symlink target lexical resolution |
| `src/matcher.rs`, `src/matcher/*` | shared rule grammar, descriptor/specificity/containment, candidate index |
| `src/errors.rs` | host errno 변환, write-intent 판정 helper |
| `src/fs.rs` | FUSE operation orchestrator |
| `src/fs/guards.rs` | visibility/mutability guards, symlink target checks, mutation coordinate checks |
| `src/fs/backing.rs` | source-root confinement, fd/dirfd-relative host filesystem delegation |
| `src/fs/state.rs` | inode/path map, refs, file/dir handles, directory cookie state |
| `src/fs/perf.rs` | optional `perf-counters` feature metrics |
| `src/*_tests.rs` | CLI/config/path/matcher/errors standalone regression coverage |
| `src/fs/tests/**` | mount 없는 filesystem behavior regression coverage |

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
