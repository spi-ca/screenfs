---
description: 요구사항 변경 이슈로 goal을 만들고 문서·prompt·skill 변경과 리뷰를 병렬 subagent로 반복 처리
argument-hint: "<요구사항 변경>"
---
다음 요구사항 변경 요청을 처리한다.

요구사항 변경:
$ARGUMENTS

작업 규칙:
1. 요구사항 변경을 바탕으로 먼저 `create_goal`을 호출해 구체적인 objective를 만든다.
   - goal 예산 부족으로 workflow가 중단되지 않도록 `token_budget`을 명시한다.
   - 문서/프롬프트/skill 중심 변경은 보통 `token_budget: 30000` 이상을 기본값으로 잡고, 영향 범위가 넓으면 더 크게 잡는다.
   - 현재 goal이 이미 `budgetLimited`면 새 goal 생성 시 `replace_existing: true`를 사용해 교체한 뒤 진행한다.
2. 요구사항/문서 영향 분석을 우선한다. 기본 workflow 가이드는 기존 `software-role-agents`, `iterative-findings-loop` skill을 사용한다.
3. `user-representative`로 현재 이슈, 수정되어야 할 의도, acceptance criteria, 영향 받는 이해관계자, blocker를 정리한다.
4. 필요하면 `software-systems-engineer`와 `software-designer`로 시스템 제약, 구현 가정, 기존 문서, 검증 범위에 대한 영향을 분석한다.
5. `software-implementer`로 관련 문서, prompt template, skill, 프로젝트 문서를 갱신한다.
   - `docs/diagrams/*.mmd`, `docs/diagrams/mermaid-config.json`, `docs/diagrams/puppeteer-config.json`이 변경되면 `docs/diagrams/README.md`의 공용 규칙도 함께 맞춘다.
   - Mermaid PNG 산출물 규칙은 반드시 `docs/diagrams/mermaid-config.json` + `docs/diagrams/puppeteer-config.json` + Mermaid CLI `--scale 2` 조합으로 기록한다. 두 config 파일만으로 2x scale이 자동 보장된다고 가정하지 않는다.
   - 다이어그램 source/config 변경 시 대응 `*.svg`, `*.png` 재생성 여부와 근거 명령을 남긴다.
   - path-like CLI/rule grammar가 바뀌면 current implementation과 target contract를 분리해 문서화하고, exact path normalization / supported prefixed glob grammar / fail-fast error semantics를 각각 명시한다.
6. 문서 리뷰는 가능한 병렬로 진행하고, 특히 `software-reviewer`를 사용한다. 문서 검증 명령, prompt flow smoke check, 정합성 확인이 있으면 `software-qa`도 함께 사용한다.
7. 리뷰에서 findings 또는 blocker가 나오면 피드백을 반영해 문서를 다시 수정하고 검증/리뷰를 반복한다.
8. findings와 blocker가 없어질 때까지 반복한다.
9. 모든 명시 변경 요청이 수정된 파일, diff, 검증 명령 결과, 리뷰 결과 같은 fresh evidence에 매핑되기 전에는 종료하지 않는다.

실행 지침:
- 의존 단계는 `subagent` chain mode를 우선 사용하고, 독립 리뷰/검증은 parallel stage를 우선 사용한다.
- 특별히 현재 대화 맥락이 꼭 필요하지 않다면 `mode: "spawn"`을 사용한다.
- project-local subagent 확인이 필요하면 우회하지 말고 확인을 요청한다.
- 요구사항 분석, 영향 분석, 문서 수정, QA, 리뷰 결과를 분리해서 남긴다.
- 용어, workflow, 예시, acceptance criteria가 바뀌면 인접 문서도 함께 갱신한다.
- mutability policy 문서를 다루면 one-family-per-mount, hidden `ENOENT` precedence, allowWrite union semantics, affected-coordinate-wide writable requirement, `copy_file_range` source/destination 분리, current `--readonly`의 legacy current-state 지위, standalone compatibility mode, future canonical contract 비포함, family/rule/config surface와의 fail-fast를 함께 점검한다.
- path-like rule 입력을 다루면 exact path 입력과 glob grammar를 분리하고, supported prefixed glob 예시와 broader unsupported wildcard forms, `HOME`/`source_root`/`~user` fail-fast 조건을 문서 간 일관되게 맞춘다.
- 현재 `--readonly-rule` surface와 그 향후 확장이 같은 normalization contract를 재사용하는지, `--allow-write`와 config mutability block 규칙이 그 계약을 공유하는지, CLI mutability option이 없을 때 config `mutability` block이 우선하는지, current `--readonly`가 legacy current-state/standalone compatibility surface로만 남는지, 그리고 current verified evidence가 여전히 global `--readonly` 중심인지도 분리해서 적는다.
- 다이어그램 관련 변경이면 `README.md`, `docs/architecture.md`, `docs/design.md`, `docs/operations.md`, `docs/diagrams/README.md` 사이의 렌더링 규칙 문구가 일치하는지 확인한다.
- 문서, 리뷰, 검증이 모두 정리될 때까지 반복한다.

권장 chain 골격:

```json
{
  "chain": [
    {
      "label": "requirements-change-analysis",
      "agent": "user-representative",
      "task": "다음 요구사항 변경의 현재 이슈, 수정 의도, acceptance criteria, 영향 문서, 사용자 기대치, blocker를 정리하라: $ARGUMENTS"
    },
    {
      "type": "parallel",
      "label": "impact-analysis",
      "tasks": [
        {
          "agent": "software-systems-engineer",
          "task": "다음 요구사항 변경의 system/runtime/operational assumption 변경과 문서 영향을 분석하라: $ARGUMENTS"
        },
        {
          "agent": "software-designer",
          "task": "다음 요구사항 변경에 필요한 design/workflow/prompt/skill/documentation 변경을 분석하라: $ARGUMENTS"
        }
      ]
    },
    {
      "label": "documentation-update",
      "agent": "software-implementer",
      "task": "다음 요구사항 변경에 맞게 관련 문서, prompt, project docs를 수정하고 focused validation을 수행하라: $ARGUMENTS"
    },
    {
      "type": "parallel",
      "label": "documentation-review",
      "tasks": [
        {
          "agent": "software-reviewer",
          "task": "다음 요구사항 변경에 대해 수정된 문서, prompt, acceptance criteria 정합성을 리뷰하라: $ARGUMENTS"
        },
        {
          "agent": "software-qa",
          "task": "다음 요구사항 변경에 대해 문서 검증, 명령 확인, prompt-flow smoke check를 수행하라: $ARGUMENTS"
        }
      ]
    }
  ],
  "mode": "spawn"
}
```

findings 또는 blocker가 나오면 문서를 수정하고 필요한 리뷰/검증 단계를 반복해 모두 해소한다.
