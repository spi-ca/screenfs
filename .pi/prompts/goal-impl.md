---
description: 구현 요구사항으로 goal을 만들고 코드·테스트·문서·QA·리뷰를 병렬 subagent로 반복 처리
argument-hint: "<구현 요구사항>"
---
다음 구현 요청을 처리한다.

구현 요구사항:
$ARGUMENTS

작업 규칙:
1. 요구사항을 바탕으로 먼저 `create_goal`을 호출해 구체적인 objective를 만든다.
   - goal 예산 부족으로 구현 workflow가 중단되지 않도록 `token_budget`을 명시한다.
   - 구현/테스트/리뷰를 포함하는 작업은 보통 `token_budget: 50000` 이상을 기본값으로 잡고, 병렬 lane·재시도가 예상되면 더 크게 잡는다.
   - 현재 goal이 이미 `budgetLimited`면 새 goal 생성 시 `replace_existing: true`를 사용해 교체한 뒤 진행한다.
2. 기본 workflow 가이드는 기존 `software-role-agents`, `software-developer-parallel`, `iterative-findings-loop` skill을 사용한다.
3. `user-representative`로 사용자 의도, acceptance criteria, 사용자 시나리오, blocker를 먼저 정리한다.
4. 필요하면 `software-systems-engineer`로 시스템/runtime/repository 제약과 검증 필요사항을 정리한다.
5. `software-designer`로 구현 계획, work package, 테스트 전략, 문서 변경 범위, 검증 전략을 만든다.
6. 안전하게 분리 가능하면 구현과 테스트 작업을 독립 work package로 나누고 `software-developer`를 병렬 실행한다. 겹치는 파일은 병렬 lane에 넣지 않는다.
7. `software-implementer`로 공유 파일 통합, 남은 구현, 테스트 보강, 필요한 문서 업데이트를 수행한다.
8. 생성된 코드, 테스트, 문서에 대해 `software-qa`와 `software-reviewer`를 병렬 실행한다.
9. QA/리뷰에서 findings 또는 blocker가 나오면 피드백을 계획에 반영하고 필요한 구현·테스트·문서 단계를 다시 실행한다.
10. findings와 blocker가 없어질 때까지 반복한다.
11. 모든 명시 요구사항이 파일, diff, 명령 결과, 테스트, 리뷰 결과 같은 current evidence에 매핑되기 전에는 종료하지 않는다.

실행 지침:
- 의존 단계는 `subagent` chain mode를 우선 사용하고, 독립 단계는 parallel stage를 우선 사용한다.
- 특별히 현재 대화 맥락이 꼭 필요하지 않다면 `mode: "spawn"`을 사용한다.
- project-local subagent 확인이 필요하면 우회하지 말고 확인을 요청한다.
- 요구사항 분석, 제약 검토, 설계, 병렬 구현 lane, 통합, QA, 리뷰 결과를 분리해서 남긴다.
- 동작, 워크플로우, 테스트, 운영 기대치가 바뀌면 문서를 함께 갱신한다.
- 코드, 테스트, 문서, QA, 리뷰가 모두 정리될 때까지 반복한다.

권장 chain 골격:

```json
{
  "chain": [
    {
      "label": "requirements",
      "agent": "user-representative",
      "task": "다음 요구사항의 의도, acceptance criteria, 사용자 시나리오, 제약, blocker를 정리하라: $ARGUMENTS"
    },
    {
      "label": "system-constraints",
      "agent": "software-systems-engineer",
      "task": "이전 결과를 바탕으로 다음 요구사항의 repository/environment 제약, feasibility, verification needs를 정리하라: $ARGUMENTS"
    },
    {
      "label": "design",
      "agent": "software-designer",
      "task": "이전 결과를 바탕으로 다음 요구사항의 구현 계획, work package, 테스트 전략, 문서 업데이트, 검증 전략을 설계하라: $ARGUMENTS"
    },
    {
      "type": "parallel",
      "label": "parallel-development",
      "tasks": [
        {
          "agent": "software-developer",
          "task": "다음 요구사항의 독립 package A를 구현하라: $ARGUMENTS. Allowed files, acceptance criteria, preserved behavior, focused validation을 포함하라."
        },
        {
          "agent": "software-developer",
          "task": "다음 요구사항의 독립 package B를 구현하라: $ARGUMENTS. Allowed files, acceptance criteria, preserved behavior, focused validation을 포함하라."
        }
      ]
    },
    {
      "label": "implementation-merge",
      "agent": "software-implementer",
      "task": "developer lane 결과를 통합하고, 공유 파일 구현·테스트·문서 업데이트와 focused validation을 수행하라: $ARGUMENTS"
    },
    {
      "type": "parallel",
      "label": "verification-review",
      "tasks": [
        {
          "agent": "software-qa",
          "task": "다음 요구사항에 대해 코드, 테스트, 문서를 acceptance criteria 기준으로 검증하라: $ARGUMENTS"
        },
        {
          "agent": "software-reviewer",
          "task": "다음 요구사항에 대해 correctness, maintainability, regression, 문서 정합성, completion readiness를 리뷰하라: $ARGUMENTS"
        }
      ]
    }
  ],
  "mode": "spawn"
}
```

findings 또는 blocker가 나오면 계획을 갱신하고 필요한 구현·문서·QA·리뷰 단계를 반복해 모두 해소한다.
