# Pi role agents

이 프로젝트는 software 작성 workflow를 위해 project-local Pi resource를 제공한다. 현재 저장소 기준으로 subagent, skill, prompt template, extension guardrail 구성이 함께 관리된다.

## 현재 Pi 설정/리소스 요약

현재 확인된 repo-local Pi 구성:

- Subagents: `.pi/agents/*.md` 7개
- Skills: `.pi/skills/**/SKILL.md` 3개
- Prompt templates: `.pi/prompts/*.md` 4개
- Extensions: `.pi/extensions/*.json` 2개
- Project-local `settings.json`: 없음

Pi는 trusted project에서 `.pi/agents`, `.pi/skills`, `.pi/prompts`를 자동 발견한다. 이 저장소는 별도 `settings.json` 없이 repo-local resource discovery를 기본 전제로 사용한다.

## 제공 리소스

### 1. Subagents

위치: `.pi/agents/*.md`

| Agent | Model | Thinking | Tools | 역할 |
| --- | --- | --- | --- | --- |
| `user-representative` | `openai-codex/gpt-5.4-mini` | `low` | `read`, `grep`, `find`, `ls` | 사용자 의도, acceptance criteria, 사용자-facing scenario, blocker 정리 |
| `software-systems-engineer` | `openai-codex/gpt-5.4` | `high` | `read`, `grep`, `find`, `ls`, `bash` | OS/runtime/dependency/permission/deployment/운영 제약과 시스템 검증 검토 |
| `software-designer` | `openai-codex/gpt-5.4` | `high` | `read`, `grep`, `find`, `ls`, `bash` | 설계, 변경 범위, 구현 계획, 검증 전략 작성 |
| `software-developer` | `openai-codex/gpt-5.4` | `high` | `read`, `grep`, `find`, `ls`, `bash`, `edit`, `write` | 독립 work package 구현, 병렬 lane 안전성 확인, focused validation |
| `software-implementer` | `openai-codex/gpt-5.4` | `high` | `read`, `grep`, `find`, `ls`, `bash`, `edit`, `write` | 승인된 계획 또는 개발자 lane 결과에 따른 파일 수정·통합과 focused validation |
| `software-qa` | `openai-codex/gpt-5.4-mini` | `medium` | `read`, `grep`, `find`, `ls`, `bash` | acceptance criteria 기반 QA matrix, command/test/smoke evidence 검증 |
| `software-reviewer` | `openai-codex/gpt-5.4` | `high` | `read`, `grep`, `find`, `ls`, `bash` | diff, 품질, regression, completion evidence audit |

Project-local subagent는 repo-controlled prompt이므로 trusted project에서만 사용한다. 실행 시 확인이 요구되면 우회하지 않는다.

#### 실제 frontmatter 차이점

모든 subagent는 공통으로 `name`, `description`, `model`, `thinking`, `tools` frontmatter를 가진다. 차이는 주로 모델 등급, thinking 수준, tool 권한 범위에 있다.

| Agent | model 계열 | thinking | 편집 가능 | `bash` 사용 | description 핵심 문구 | 태그 | frontmatter상 특징 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `user-representative` | `gpt-5.4-mini` | `low` | 아니오 | 아니오 | User perspective subagent that turns requests into goals, acceptance criteria, blockers, and user-facing smoke scenarios | `read-only`, `analysis-focused` | 가장 가벼운 분석 전용 구성. 요구사항/acceptance criteria 정리에 집중 |
| `software-systems-engineer` | `gpt-5.4` | `high` | 아니오 | 예 | validates OS, runtime, dependency, deployment, performance, and operational constraints before implementation | `read-only`, `system-constraints` | 시스템/운영 제약 검토용 read-only + shell inspection 구성 |
| `software-designer` | `gpt-5.4` | `high` | 아니오 | 예 | converts accepted requirements into architecture, implementation plan, risk analysis, and verification strategy | `read-only`, `design-focused` | 설계/검증 전략 수립용 read-only + shell inspection 구성 |
| `software-developer` | `gpt-5.4` | `high` | 예 | 예 | Parallel-capable software developer subagent that implements one isolated work package with focused validation | `edit-capable`, `parallel-safe`, `package-scoped` | 유일하게 병렬 lane 전제를 description에 명시하고 `edit`, `write` 포함 |
| `software-implementer` | `gpt-5.4` | `high` | 예 | 예 | edits files according to an approved plan and runs focused validation | `edit-capable`, `integration-focused` | 승인된 계획 기반 통합/마무리 편집용. `software-developer`와 동일 tool 권한 |
| `software-qa` | `gpt-5.4-mini` | `medium` | 아니오 | 예 | validates acceptance criteria with tests, checks, smoke scenarios, and reproducible evidence | `read-only`, `verification-focused` | 검증 중심 mini 모델 구성. 테스트/스모크 실행은 가능하지만 편집 권한 없음 |
| `software-reviewer` | `gpt-5.4` | `high` | 아니오 | 예 | audits correctness, maintainability, regressions, security/performance risks, and completion evidence | `read-only`, `review-focused` | completion evidence audit 중심 read-only reviewer 구성 |

추가 관찰:

- mini 모델 사용 agent는 `user-representative`, `software-qa` 두 개뿐이다.
- `edit`, `write`가 있는 agent는 `software-developer`, `software-implementer` 두 개뿐이다.
- `bash`가 없는 agent는 `user-representative`뿐이다.
- `thinking: medium`은 `software-qa`만 사용하고, 나머지는 `low` 또는 `high`로 양분된다.
- `software-developer`와 `software-implementer`는 frontmatter 권한은 같지만, description과 본문 책임이 각각 병렬 독립 package 구현 vs 승인된 계획 통합으로 분리되어 있다.
- `parallel-safe` 태그는 description이나 본문에서 병렬 lane 안전성을 명시한 agent에만 붙인다. 현재는 `software-developer`만 해당한다.
- `read-only` 태그는 편집 도구가 없는 agent를 뜻하고, `edit-capable` 태그는 `edit`, `write`가 모두 있는 agent를 뜻한다.

## 역할별 입력/출력 계약

| 역할 | 주요 입력 | 주요 출력 |
| --- | --- | --- |
| 사용자 대표 | 사용자 원문 요청, 기존 문서, 현재 동작 증거 | 사용자 의도, acceptance criteria, 사용자-facing scenario, 질문/blocker |
| 시스템 엔지니어 | 사용자 대표 산출물, 저장소/환경 정보, dependency/build/runtime 정보 | 시스템 제약, feasibility, 운영 리스크, 시스템 수준 검증 |
| 설계자 | 사용자 대표 산출물, 시스템 엔지니어 산출물, 코드/문서 구조 | 설계 결정, 변경 범위, 구현 계획, 검증 전략 |
| 개발자 | 설계자 계획의 독립 work package, 허용 파일 범위, acceptance criteria, 프로젝트 지침 | package별 파일 변경, focused validation 결과, 병렬 안전성/충돌 보고, blocker |
| 구현자 | 설계자 계획, 개발자 lane 결과, 현재 파일 내용, 프로젝트 지침 | 파일 변경, 공유 파일 통합, 변경 요약, focused validation 결과, blocker |
| QA | acceptance criteria, 구현 diff, 검증 명령/환경 | QA matrix, command/test/smoke evidence, blocking/non-blocking findings |
| 리뷰어 | 사용자 요구사항, 설계, diff, QA 결과 | 품질 리뷰, blocking issue, completion audit, approve/request changes verdict |

### 2. Skills

위치:

- `.pi/skills/software-role-agents/SKILL.md`
- `.pi/skills/software-developer-parallel/SKILL.md`
- `.pi/skills/iterative-findings-loop/SKILL.md`

용도:

- `software-role-agents`
  - 역할별 책임과 품질 게이트 확인
  - subagent chain 예시 확인
  - 작은 작업에서 root agent가 같은 역할 순서를 fallback으로 수행할 때 기준 제공
- `software-developer-parallel`
  - 승인된 설계를 독립 work package로 나누고 `software-developer` lane을 병렬 실행할 때 기준 제공
- `iterative-findings-loop`
  - QA/리뷰 findings, blocker를 blocking/non-blocking으로 분류
  - 구현·문서 수정 후 focused validation → QA → review를 반복
  - fresh evidence가 확보될 때까지 반복 루프를 유지

### 3. Prompt template

위치:

- `.pi/prompts/software-role-workflow.md`
- `.pi/prompts/software-developer-parallel.md`
- `.pi/prompts/goal-impl.md`
- `.pi/prompts/goal-change.md`

사용 예:

```text
/software-role-workflow design selective readonly rules for ScreenFS
/software-developer-parallel split matcher and readonly-policy implementation work
/goal-impl readonly mode 구현 및 숨김 경로 ENOENT 보장
/goal-change subagent workflow에서 테스트 작성 책임을 더 명확히 반영해야 한다
```

`software-role-workflow` prompt는 다음 순서를 권장한다.

1. `user-representative`
2. `software-systems-engineer`
3. `software-designer`
4. 독립 package가 있으면 `software-developer` 병렬 lane
5. `software-implementer` 구현/통합
6. `software-qa`와 `software-reviewer` 병렬 검증

`software-developer-parallel` prompt는 승인된 구현 범위를 파일/모듈별 package로 나누고, 겹치지 않는 package만 `software-developer` agent들로 병렬 처리하도록 안내한다.

`goal-impl` prompt는 구현 요구사항으로 `create_goal`을 먼저 만들고, 역할 분석 → 병렬 구현/테스트 → 통합 → QA/review → findings 해소 반복까지 진행하도록 안내한다. 또한 `token_budget`을 명시해 budget 부족으로 workflow가 조기 중단되지 않게 하고, 현재 goal이 `budgetLimited`면 `replace_existing: true`로 새 goal을 만들도록 안내한다.

`goal-change` prompt는 요구사항 변경 이슈를 goal로 만들고, 영향 분석 → 문서/prompt/skill 수정 → QA/review → findings 해소 반복까지 진행하도록 안내한다. 이 prompt도 `token_budget`을 명시하고, 문서 중심 변경의 기본 예산을 넉넉히 잡으며, 기존 goal이 `budgetLimited`면 교체 후 계속 진행하도록 안내한다. 추가로 `docs/diagrams/*.mmd`나 Mermaid render config(`docs/diagrams/mermaid-config.json`, `docs/diagrams/puppeteer-config.json`)가 바뀌면 `docs/diagrams/README.md` 규칙, 대응 `*.svg`/`*.png` 산출물, PNG `--scale 2` 요구를 함께 확인하도록 안내한다. path-like rule grammar를 다루는 변경에서는 current implementation vs target behavior 분리, exact path normalization, supported prefixed-glob grammar, broader unsupported wildcard forms, `HOME`/`source_root` fail-fast 조건, future selective readonly 재사용 계약까지 함께 기록하도록 유도한다.

### 4. Extensions / Guardrails

위치:

- `.pi/extensions/guardrails.json`
- `.pi/extensions/guardrails.v0.json`

현재 확인된 내용:

- `guardrails.json`
  - schema: `@aliou/pi-guardrails@0.13.2`
  - version: `0.9.0-20260327`
  - allowedPaths: `~/.cargo/`, `/dev/fuse`, `/proc/filesystems`
- `guardrails.v0.json`
  - legacy/비교용 구 guardrail 파일
  - allowedPaths: `~/.cargo/`만 포함

운영 기준으로는 현재 더 최신이고 범위가 넓은 `guardrails.json`을 우선 참고한다. `guardrails.v0.json`은 과거 제약을 비교하거나 이력 확인이 필요할 때만 읽는다.

## 변경·추가·중복 반영 메모

- 추가됨
  - `iterative-findings-loop` skill
  - `goal-impl`, `goal-change` prompt template
- 현재 유지 중
  - 역할 기반 workflow용 subagent 7개
  - `software-role-agents`, `software-developer-parallel` skill
  - `software-role-workflow`, `software-developer-parallel` prompt
- 중복/병행 관리 주의
  - guardrail 파일이 `guardrails.json`, `guardrails.v0.json` 두 개 존재한다.
  - 문서에서는 현재값과 legacy 값을 구분해서 적고, 새 작업 기준은 `guardrails.json`을 우선 본다.
- 설정상 특이점
  - project-local `settings.json`은 없다.
  - repo-local discovery만으로 현재 Pi 리소스가 로드되도록 구성돼 있다.

## 완료 기준

- 모든 적용 대상 역할의 산출물이 있거나, 작은 작업에서는 root agent가 각 적용 대상 역할 관점 결과를 분리해 보고한다.
- `software-developer`는 독립 work package가 있을 때 적용 대상이다. 사용한 경우 각 lane의 allowed files, changed files, validation, conflict 여부가 fresh evidence로 남아 있고, 사용하지 않은 경우 안전하게 분리할 package가 없다는 근거가 남아 있다.
- 사용자 요구사항, 시스템 제약, 설계, 구현, QA, 리뷰가 fresh evidence로 연결된다.
- 목표 계약과 현재 구현 evidence를 섞지 않는다. 예를 들어 selective readonly rule 요구사항을 다룰 때는 current global `--readonly` 구현/검증을 historical 또는 current-state evidence로 분리 표기한다.
- mutability policy 문서를 갱신하면 one-family-per-mount, hidden `ENOENT` precedence, allowWrite union semantics, affected-coordinate-wide writable requirement, `copy_file_range` source/destination 분리, current `--readonly`의 legacy current-state 지위, standalone compatibility mode, future canonical contract 비포함, family/rule/config surface와의 fail-fast를 함께 점검한다.
- path/input grammar 변경이면 supported exact path forms, supported prefixed-glob grammar, broader unsupported wildcard forms, `HOME`/`source_root`/`~user` fail-fast semantics가 문서와 prompt/skill에 일관되게 반영된다.
- 현재 `--readonly-rule` surface와 그 향후 확장이 hide와 같은 normalization contract를 재사용해야 한다는 요구가 있으면 그 점도 current implementation evidence와 분리해서 기록한다.
- future CLI/config contract를 다루면 `--readonly-rule`/`--allow-write` exclusivity, current `--readonly`의 legacy current-state 분리와 standalone compatibility/fail-fast 규칙, config의 legacy bool 부재, CLI mutability option의 config block 대체 규칙과 no-CLI 시 config `mutability` block 우선 규칙까지 함께 반영한다.
- goal 기반 prompt를 사용할 때는 `create_goal`에 작업 규모에 맞는 `token_budget`을 명시하고, 이미 `budgetLimited`인 goal 위에서 그대로 실질 작업을 이어가지 않는다.
- 변경 유형에 맞는 저장소 검증 명령을 실행한다. 문서/Pi resource 전용 변경은 `AGENTS.md`와 `docs/operations.md`의 문서 변경 검증을 따르고, Rust 코드 변경이 포함되면 fmt/clippy/test 같은 코드 검증을 추가한다.
- 다이어그램 source/config 변경은 문서 검증 외에도 `docs/diagrams/README.md` 기준 render 명령과 PNG `--scale 2` 규칙이 인접 문서에 일관되게 반영됐는지 확인한다.
- 미승인 shortcut, 임시 미완성 표식, dead code, duplicated logic, 숨은 가정, 문서화되지 않은 behavior change를 남기지 않는다.
