# Pi role agents

이 저장소는 software-writing workflow를 위해 repo-local Pi resource를 관리한다. 작은 작업은 root agent가 필요한 역할 관점을 직접 수행해도 되며, 큰 작업은 project-local skill과 subagent workflow를 사용한다.

## Resource inventory

Repo-local Pi discovery 기준 현재 구성:

- Subagents: `.pi/agents/*.md` 7개
- Skills: `.pi/skills/**/SKILL.md` 4개
- Prompt templates: `.pi/prompts/*.md` 2개
- Extensions/guardrails: `.pi/extensions/*.json` 2개
- Project-local `settings.json`: 없음

Trusted project에서만 repo-controlled Pi resources를 사용한다. 실행 시 확인이 요구되면 우회하지 않는다.

## Subagent roles

| Agent | 권한 | 역할 |
| --- | --- | --- |
| `user-representative` | read-only, no bash | 사용자 의도, acceptance criteria, user-facing scenario, blocker 정리 |
| `software-systems-engineer` | read-only + bash | OS/runtime/dependency/permission/deployment/운영 제약 검토 |
| `software-designer` | read-only + bash | 설계 결정, 변경 범위, 구현 계획, 검증 전략 작성 |
| `software-developer` | edit-capable + bash | 독립 work package 구현, focused validation, 병렬 lane 안전성 보고 |
| `software-implementer` | edit-capable + bash | 승인된 계획 또는 lane 결과 통합, 파일 수정, focused validation |
| `software-qa` | read-only + bash | acceptance criteria 기반 테스트/스모크/evidence 검증 |
| `software-reviewer` | read-only + bash | diff 품질, regression, completion evidence audit |

현재 `.pi/agents/*.md` frontmatter 확인 기준 상세 surface는 아래와 같다.

| File | model | thinking | tools |
| --- | --- | --- | --- |
| `.pi/agents/user-representative.md` | `openai-codex/gpt-5.6-luna` | `low` | `read`, `grep`, `find`, `ls` |
| `.pi/agents/software-systems-engineer.md` | `openai-codex/gpt-5.6-sol` | `high` | `read`, `grep`, `find`, `ls`, `bash` |
| `.pi/agents/software-designer.md` | `openai-codex/gpt-5.6-sol` | `high` | `read`, `grep`, `find`, `ls`, `bash` |
| `.pi/agents/software-developer.md` | `openai-codex/gpt-5.6-terra` | `high` | `read`, `grep`, `find`, `ls`, `bash`, `edit`, `write` |
| `.pi/agents/software-implementer.md` | `openai-codex/gpt-5.6-terra` | `high` | `read`, `grep`, `find`, `ls`, `bash`, `edit`, `write` |
| `.pi/agents/software-qa.md` | `openai-codex/gpt-5.6-luna` | `medium` | `read`, `grep`, `find`, `ls`, `bash` |
| `.pi/agents/software-reviewer.md` | `openai-codex/gpt-5.6-sol` | `high` | `read`, `grep`, `find`, `ls`, `bash` |

편집 가능 agent는 `software-developer`, `software-implementer`뿐이고 둘 다 `edit`와 `write`를 함께 가진다. `bash`가 없는 agent는 `user-representative`뿐이다. 병렬 lane 전제는 `software-developer`에만 둔다.

## Skills

- `.pi/skills/software-role-agents/SKILL.md`: 역할 기반 workflow와 품질 게이트
- `.pi/skills/software-developer-parallel/SKILL.md`: 승인된 work를 독립 package로 나누어 병렬 개발
- `.pi/skills/iterative-findings-loop/SKILL.md`: QA/review finding을 해소할 때까지 구현·검증 반복
- `.pi/skills/run-screenfs-benchmarks/SKILL.md`: ScreenFS formal benchmark harness 실행·기록

## Prompt templates

- `.pi/prompts/software-role-workflow.md`: 역할 순서 기반 workflow
- `.pi/prompts/software-developer-parallel.md`: 병렬 developer lane 구성

Recommended large-workflow order:

1. 사용자 대표 관점으로 요구사항과 acceptance criteria 정리
2. 시스템 엔지니어 관점으로 운영/환경 제약 확인
3. 설계자 관점으로 변경 범위와 검증 전략 작성
4. 독립 package가 안전하면 `software-developer` 병렬 lane 사용
5. `software-implementer`가 통합 수정
6. `software-qa`와 `software-reviewer`가 current evidence로 검증
7. blocking finding이 있으면 iterative findings loop로 반복

## Guardrails

- `.pi/extensions/guardrails.json`을 현재 기준으로 우선 본다.
- `.pi/extensions/guardrails.v0.json`은 과거 제약 비교가 필요할 때만 읽는다.
- Guardrail 설명을 바꾸면 `.pi/extensions/*.json`의 `$schema`, `version`, `allowedPaths`, current/legacy 역할을 실제 파일과 대조한다.
- 문서/Pi resource 변경도 `AGENTS.md`와 [`operations.md`](operations.md)의 검증 기준을 따른다.
- `.pi/agents/*.md`를 수정하거나 이 문서의 agent/frontmatter summary를 바꿨다면 summary와 실제 `.pi/agents/*.md` frontmatter(model/thinking/tools)를 대조해 검증한다.
- `docs/diagrams/*.mmd` 또는 Mermaid config를 바꾸면 [`diagrams/README.md`](diagrams/README.md)에 따라 SVG/PNG를 함께 재생성하고 PNG에는 `--scale 2`를 적용한다.

## Completion criteria

- 사용자 요구사항, 시스템 제약, 설계, 구현, QA, 리뷰가 current evidence로 연결된다.
- 작은 작업에서 subagent를 생략했다면 root agent가 해당 관점의 판단을 요약한다.
- `software-developer` 병렬 lane을 썼다면 lane별 allowed files, changed files, validation, conflict 여부가 남아 있다.
- mount lifecycle/shutdown 변경이라면 `SIGINT`/`SIGTERM` trigger, cancellation handoff, FUSE serve-loop graceful exit, explicit `fusermount3 -u <mountpoint>` cleanup, normal unmount failure 시 lazy-unmount option/manual guidance evidence가 current evidence로 남아 있다.
- `.pi/agents`를 편집했거나 이 문서의 frontmatter summary를 갱신했다면 summary/count가 실제 `.pi/agents/*.md`와 일치한다.
- Guardrail inventory를 바꾸면 실제 `.pi/extensions/*.json` 핵심 필드와 current/legacy 역할을 대조한 current evidence를 남긴다.
- current implementation evidence와 target contract를 섞지 않는다.
- 미승인 shortcut, 임시 미완성 표식, hidden assumption, duplicated logic, 문서화되지 않은 behavior change를 남기지 않는다.
