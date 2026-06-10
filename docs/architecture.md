# ScreenFS 아키텍처 개요

이 문서는 현재 계획서(`README.md`, `docs/requirements.md`, `docs/design.md`, `docs/operations.md`)와 현재 코드(`src/*.rs`)를 함께 기준으로 정리한 아키텍처 요약이다.

- source of truth: 구현 코드는 `src/`, 계약과 목표는 `README.md`, `docs/requirements.md`, `docs/design.md`
- 현재 상태: v1 핵심 hidden 경로 의미론과 현재 구현의 global `--readonly` bool guard가 구현되어 있고, repo-local/whole-root FUSE smoke 및 `unshare -UrR` 기반 chroot 실행 smoke가 통과했다 (`docs/artifacts/fuse-smoke-transcript.md` 참고). selective readonly rule은 목표 계약으로 분리해 읽어야 하며, 현재 live evidence는 global `--readonly` 기준까지만 확인됐다. `/dev/null` 등 device-node semantics는 FUSE mount의 `nodev` 제약 때문에 상위 supervisor/namespace layer 책임으로 남는다.
- 런타임 전제: non-root + `fusermount3`, `FUSE_OVER_IO_URING` 필수, 협상 실패 시 fallback 없이 fail-fast
- 통합 경계: mount owner와 동일 host uid 접근이 기본 전제이며, chroot 권한 모델과 `/proc`·`/sys`·`/dev`·`/run` native semantics는 상위 supervisor 책임이다
- 다이어그램 원본: `docs/diagrams/*.mmd`
- 렌더링 산출물: `docs/diagrams/*.png`, `docs/diagrams/*.svg`
- 공용 렌더링 규칙: `docs/diagrams/README.md`를 따른다. `docs/diagrams/mermaid-config.json`은 Mermaid theme/font source of truth이고, `docs/diagrams/puppeteer-config.json`은 Puppeteer launch option source of truth다. PNG는 이 두 config를 함께 사용해 Mermaid CLI `--scale 2`로 렌더링한다.

## 1. 시스템 컨텍스트

이 절은 ScreenFS가 host filesystem과 whole-root consumer 사이에서 어떤 경계 역할을 하는지 빠르게 보여준다.

ScreenFS는 `source_root`(대표 예시는 `/`)를 backing tree로 삼아 FUSE mount를 만들고, 그 mount를 sandbox/chroot 같은 상위 consumer가 읽는 구조다. `pi-bash-sandbox`는 대표 통합 예시지만 시스템 경계 자체는 특정 supervisor 하나에 고정되지 않는다. 아래 그림의 mount path는 `/tmp/screenfs-root` 예시일 뿐 고정 경로가 아니다. 목표 계약상 숨김 경로는 `ENOENT`, selective readonly rule에 매칭된 visible mutation은 `EROFS`로 분기한다. 현재 구현/검증 evidence는 이 readonly 판정을 아직 global `--readonly` bool로만 수행한다. future documented alternate policy family는 `readonly-root-allowwrite`로 닫혀 있으며, canonical future contract는 explicit family/rule surface만 사용한다. legacy `--readonly`는 current-state compatibility mode로만 남고 future family/rule/config surface와 병용 시 fail-fast 대상으로 본다. 그 경우에도 handler-level policy evaluator가 source/target/parent와 hidden 우선순위를 함께 판정해야 하며 mount-level `ro`만으로는 충분하지 않다.

![ScreenFS system context](diagrams/system-context.png)

구현 파일 바로가기: `README.md`, `docs/design.md`, `docs/operations.md`

## 2. 모듈 아키텍처

이 절은 코드를 어디서부터 읽어야 하는지와 책임이 어떻게 나뉘는지를 보여준다.

현재 코드는 `main -> cli/config -> fs`를 중심으로 구성되어 있고, `fs`가 `path`, `matcher`, `errors`와 내부 inode/file-handle state를 조합해 대부분의 FUSE 의미론을 수행한다. 별도 backing adapter 모듈은 아직 없고, host 접근 helper는 주로 `src/fs.rs` 안에 있다.

![ScreenFS module architecture](diagrams/module-architecture.png)

핵심 포인트:

- `src/main.rs`: mount option 구성과 `Session::run(ScreenFs::new(cfg))` 진입점
- `src/cli.rs`: `<source-root> <mount-root>`, 반복 `--hide`, global `--readonly`, 반복 `--readonly-rule` 파싱. future family-aware surface(`--policy-family`, `--allow-write`)는 아직 없다.
- `src/config.rs`: `RuntimeConfig` 구성과 mount-root recursion exclusion internal rule 주입, global readonly flag와 readonly matcher를 함께 보관/조립한다. hide/current `--readonly-rule` 입력은 shared normalization contract를 거쳐 matcher로 들어간다. future documented contract에서는 `MutabilityPolicy` family + compiled readonly/`allow_write` rule set을 담는 evaluator-oriented data model이 추가로 필요하다.
- `src/path.rs`: lexical virtual path normalization, symlink target lexical resolution, source-root confinement 보조, relative/`~` exact·prefixed-glob rule input rebasing helper
- `src/matcher.rs`: exact/prefix/limited glob matcher. 현재는 absolute exact rule과 optional normalized prefix가 붙은 limited glob(`**/<basename>`, `**/*.<suffix>`)까지 처리한다. broader unsupported wildcard forms는 계속 fail-fast다.
- `src/fs.rs`: hidden guard, 현재 global readonly guard, symlink target guard, directory filtering, host delegation. future carve-out에서는 `guard_mutation_path`/`guard_multi_path_mutation`가 affected-path set 전체를 보는 policy evaluator 방향으로 확장돼야 한다.
- `src/errors.rs`: hidden 우선 `ENOENT`, readonly-target mutation `EROFS` 분류. future family에서도 errno taxonomy는 유지하되 evaluator reason model은 더 풍부해질 수 있다.

구현 파일 바로가기: `src/main.rs`, `src/cli.rs`, `src/config.rs`, `src/path.rs`, `src/matcher.rs`, `src/errors.rs`, `src/fs.rs`

## 3. 요청 처리 결정 흐름

이 절은 한 요청이 `ENOENT`, `EROFS`, pass-through 중 어디로 분기되는지 이해하기 위한 운영 중심 요약이다.

많은 조회·traversal 요청은 virtual path 계산 후 hidden 판정이 먼저, symlink target hidden 검사와 mutability policy evaluator 판정이 뒤따르고, host filesystem delegation이 마지막 순서로 진행된다. 아래 다이어그램은 current target selective-readonly family를 기준으로 읽되, same evaluator shape가 future carve-out family에도 재사용된다고 이해하면 된다. callout처럼 현재 구현은 이 evaluator를 아직 global `--readonly` bool 단순화로만 수행한다.

![ScreenFS request decision flow](diagrams/request-decision-flow.png)

핵심 포인트:

- hidden path는 가능한 한 존재하지 않는 것처럼 보여야 한다
- readonly는 rule에 매칭된 visible mutation에만 적용된다. 현재 구현 evidence는 global `--readonly`가 모든 visible mutation에 적용되는 형태다.
- future alternate policy family로 `readonly-root-allowwrite`를 도입하더라도 hidden `ENOENT` 우선과 handler-level 판정은 유지돼야 한다.
- symlink는 entry path뿐 아니라 resolved virtual target도 검사한다
- `rename`/`link`/`symlink`/copy-like mutation 같은 multi-path 연산은 source/target/parent 각각을 다시 검사한다

현재 구현과 계획서가 공통으로 강조하는 포인트는 다음과 같다.

- hidden 관련 판단은 가능한 한 `ENOENT`를 우선시한다.
- `lookup`/`getattr`/`open`/`opendir`/`access`/`readlink` 같은 existing-entry 요청은 visible symlink entry의 resolved virtual target도 검사해 hidden이면 `ENOENT`로 차단한다.
- `create`/`mkdir`/`mknod`/`unlink`/`rmdir` 같은 path-only mutation은 기본적으로 path hidden/readonly rule guard를 거친다.
- multi-path mutation은 관련된 source/target/parent를 각각 다시 검사해 selective readonly 대상 여부도 판정한다. 현재 구현 evidence는 이 판정을 global `--readonly` bool로 단순화한다.
- broader ancestor-symlink traversal hardening은 설계 문서의 더 넓은 목표이며, 이 문서의 흐름도는 현재 코드에서 확인되는 direct symlink-entry guard 범위를 우선 보여준다.

구현 파일 바로가기: `src/fs.rs`, `src/errors.rs`, `docs/design.md`

## 4. 경로 정규화와 confinement

이 절은 왜 path 판단을 host canonical path가 아니라 virtual path 기준으로 하는지 설명한다.

hide matcher는 host canonical path가 아니라 lexical virtual path 기준으로 동작한다. 이후 backing 접근은 `source_root` 밖으로 빠져나가지 않도록 제한한다. 아래 다이어그램은 특히 visible symlink entry를 직접 다루는 경로를 기준으로 읽는 것이 정확하다.

![ScreenFS path resolution and confinement](diagrams/path-resolution.png)

핵심 포인트:

- `.` 제거, 중복 `/` 정리, `..`는 virtual `/` 위로 못 올라감
- 현재 구현의 exact hide/current readonly rule은 absolute virtual path를 직접 받거나, relative/`~` exact 입력을 `source_root` 내부 virtual absolute path로 rebase해 사용한다.
- 현재 구현의 supported glob은 prefix 없는 `**/<basename>` / `**/*.<suffix>`뿐 아니라 optional normalized prefix가 붙은 limited glob까지 포함한다.
- relative path와 leading `~`, `~/...` prefix는 host path로 해석된 뒤 `source_root` 내부일 때만 virtual absolute path 또는 virtual glob prefix로 rebase된다.
- `HOME` 없음, `source_root` 밖으로 확장됨, `~user`, prefix 내부 wildcard, broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`)은 fail-fast/unsupported로 남는다.
- symlink target은 lexical virtual target으로 재해석해 hidden 여부를 다시 검사
- host backing 접근은 `source_root` 밖 escape를 허용하지 않음

구현 파일 바로가기: `src/path.rs`, `src/matcher.rs`, `src/fs.rs`

## 5. 운영/통합 경계

이 절은 ScreenFS가 책임지는 것과 상위 supervisor가 책임지는 것을 분리해 준다.

이 아키텍처가 전제하는 운영 경계는 다음과 같다.

- mount 생성은 non-root 사용자 + `fusermount3` 기준이다
- `FUSE_OVER_IO_URING` 협상 실패 시 mount를 degraded fallback으로 열지 않고 fail-fast 한다
- 기본 접근 모델은 mount owner와 동일 host uid다
- 다른 host uid 접근은 `allow_other`와 `/etc/fuse.conf` 정책이 별도로 필요하다
- `chroot` 실행 권한, user namespace, supervisor 구성은 `ScreenFS` 바깥 책임이다 (`pi-bash-sandbox`는 대표 예시일 뿐 유일한 상위 레이어는 아님)
- `/proc`·`/sys`·`/dev`·`/run`의 native semantics 재현도 `ScreenFS` 단독 책임이 아니다

구현 파일 바로가기: `README.md`, `docs/design.md`, `docs/operations.md`

## 6. 검증 경계

이 절은 무엇이 설계/구조 설명이고 무엇이 실제 검증 evidence인지 구분하게 해 준다.

이 문서는 구조를 설명하는 artifact이고, live proof의 source of truth는 `docs/operations.md`다. 이 artifact를 읽고 바로 연결되어야 하는 최소 검증 질문은 다음이다.

- hidden entry가 `readdir`/`readdirplus`에서 실제로 빠지는가
- hidden 직접 접근이 `ENOENT`인가
- selective readonly rule에 매칭된 visible mutation이 `EROFS`인가
- rule-input normalization contract를 current implementation snapshot과 target behavior로 분리해 읽었는가
- 현재 구현의 global `--readonly` smoke가 모든 visible mutation을 `EROFS`로 막는다는 사실과 목표 계약을 혼동하지 않았는가
- `source-root=/`에서 `/bin`, `/usr`, `/etc` 같은 whole-view 경로가 실제로 보이는가
- mount-root recursion exclusion이 listing/lookup에 다시 나타나지 않는가
- `unshare -UrR` 기반 chroot smoke가 성공하는가, 그리고 `/dev/null` 같은 device-node semantics가 supervisor/namespace layer 책임으로 명확히 남는가

구현 파일 바로가기: `docs/operations.md`, `docs/artifacts/fuse-smoke-transcript.md`, `README.md`

## 7. 현재 구현이 드러내는 아키텍처 요약

이 절은 위 내용을 한 줄 흐름으로 다시 압축해서 기억하기 쉽게 만든다.

```text
CLI args
  -> RuntimeConfig
  -> HideMatcher + internal mount-root prefix + current global readonly flag
  -> future documented contract: MutabilityPolicy family + compiled readonly/allow_write rule sets
  -> current shared rule normalization for hide/current readonly rules (absolute/relative/`~` exact + supported prefixed glob)
  -> ScreenFs Filesystem implementation
  -> VirtualPath normalization + hidden/affected-coordinate policy evaluator
  -> confined host filesystem access
  -> FUSE replies to whole-root consumer (예: sandbox/chroot)
```

구현 파일 바로가기: `src/cli.rs`, `src/config.rs`, `src/fs.rs`, `src/path.rs`

## 8. 문서 간 역할 분담

이 절은 어떤 질문이 생겼을 때 어느 문서를 source of truth로 봐야 하는지 정리한다.

- `README.md`: 프로젝트 목적, 운영 예시, 현재 상태 요약
- `docs/requirements.md`: v1 요구사항과 금지/비목표
- `docs/design.md`: 의미론과 모듈 경계에 대한 계약
- `docs/operations.md`: 검증 방식과 smoke evidence
- `docs/architecture.md`: 위 문서와 현재 코드를 함께 읽기 쉽게 시각화한 요약

구현 파일 바로가기: `README.md`, `docs/requirements.md`, `docs/design.md`, `docs/operations.md`
