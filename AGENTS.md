# AGENTS.md

이 저장소에서 작업하는 에이전트는 아래 지침을 따른다.

## 프로젝트 목적

`holefs`는 `pi-bash-sandbox`의 Linux sandbox/chroot 실행 요구사항을 만족하기 위한 FUSE 기반 filesystem view layer다. 전체 `/` 파일시스템을 chroot root로 사용할 수 있게 pass-through 하면서, 지정된 민감 경로는 존재하지 않는 것처럼 숨긴다.

## 작업 원칙

- 기본 실행 전제는 non-root다.
- FUSE mount는 `fusermount3`를 기준으로 문서화하고 구현한다.
- root-only mount, privileged bind mount, system-wide mount namespace 조작에 의존하는 설계로 바꾸지 않는다.
- `/dev/null` bind overlay, 빈 파일 overlay, tmpfs masking처럼 이름을 남기는 masking 방식은 사용하지 않는다.
- 숨김 대상은 가능한 한 `ENOENT`로 처리하고, `Permission denied`로 존재를 노출하지 않는다.
- chroot 사용을 전제로 하므로 프로젝트 디렉터리 전용 view가 아니라 전체 `/` view 요구사항을 유지한다.
- `fractal-fuse = 0.4.0` 기반 구현 전제를 임의로 변경하지 않는다.
- FUSE3 및 `FUSE_OVER_IO_URING` 사용 목표를 문서와 설계에서 유지한다.
- 사용자의 기존 변경사항을 덮어쓰지 말고, 변경 전후 diff를 확인한다.

## 필수 의미론

Hidden path는 다음 operation에서 존재하지 않는 것처럼 동작해야 한다.

- `readdir`, `readdirplus`: 결과에서 제외
- `lookup`, `getattr`, `open`, `access`: `ENOENT`

Selective readonly rule 목표는 다음을 지킨다.

- hidden path는 selective readonly 여부와 무관하게 `ENOENT`
- readonly rule에 매치된 visible path의 read/stat/list 허용
- `write`, `create`, `mkdir`, `mknod`, `unlink`, `rmdir`, `rename`, `link`, `symlink`, mutation `setattr`, `fallocate`는 readonly rule에 매치된 path에서 `EROFS`
- readonly rule에 매치되지 않은 visible path는 이 요구사항만으로 자동 readonly가 되지 않음
- CLI syntax는 TBD이며, 현재 구현/검증은 global `--readonly` bool 기준임

## Pi role agents

이 프로젝트는 software 작성 workflow를 위해 project-local Pi resource를 제공한다.

- Subagents: `.pi/agents/*.md`
- Skills: `.pi/skills/software-role-agents/SKILL.md`, `.pi/skills/software-developer-parallel/SKILL.md`, `.pi/skills/iterative-findings-loop/SKILL.md`
- Prompt templates: `.pi/prompts/software-role-workflow.md`, `.pi/prompts/software-developer-parallel.md`, `.pi/prompts/goal-impl.md`, `.pi/prompts/goal-change.md`
- Extensions: `.pi/extensions/guardrails.json`, `.pi/extensions/guardrails.v0.json`
- 설명 문서: `docs/pi-agents.md`

역할은 `user-representative`, `software-systems-engineer`, `software-designer`, `software-developer`, `software-implementer`, `software-qa`, `software-reviewer`를 기준으로 한다. `software-developer`는 독립 work package를 병렬 lane으로 구현할 때 사용한다. project-local subagent 실행 확인은 우회하지 않는다.

## 검증 지침

`docs/diagrams`를 수정했거나 `docs/diagrams/mermaid-config.json`, `docs/diagrams/puppeteer-config.json`이 바뀌면 다음도 함께 지킨다.

- `docs/diagrams/*.mmd`를 source of truth로 보고 대응 `*.svg`, `*.png`를 재생성한다.
- PNG 생성 명령은 공용 config 두 개를 함께 사용하고 Mermaid CLI `--scale 2`를 포함해야 한다.
- 상세 렌더 계약과 재생성 예시는 `docs/diagrams/README.md`를 따른다.

문서 변경 시 최소 확인:

```bash
cargo check
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts -maxdepth 3 -type f -print | sort
```

코드 변경이 포함되면 관련 Rust 검증을 추가한다.

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```

현재 단계의 문서가 구현 완료를 의미하지 않도록, 미구현 기능은 요구사항 또는 설계 목표로 표현한다.
