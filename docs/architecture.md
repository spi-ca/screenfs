# ScreenFS 아키텍처 개요

이 문서는 현재 계획서(`README.md`, `docs/requirements.md`, `docs/design.md`, `docs/operations.md`)와 현재 코드(`src/*.rs`)를 함께 기준으로 정리한 아키텍처 요약이다. bare slashless glob의 cwd-anchored `./<pattern>` direct-child semantics는 현재 Rust 구현·테스트·fresh smoke evidence가 갖춰진 contract이며, 최신 baseline은 `docs/artifacts/current-bare-basename-glob-smoke-transcript.md`에 기록돼 있다.

- source of truth: 구현 코드는 `src/`, 계약과 목표는 `README.md`, `docs/requirements.md`, `docs/design.md`
- 이 문서는 승인된 목표 계약을 `visibility.*` / `mutability.*` 축으로 설명한다.
- transcript filename에 historical label이 남아 있더라도 current CLI/config reference는 두 축 문서를 따른다.
- 런타임 전제: non-root + `fusermount3`, `FUSE_OVER_IO_URING` 필수, 협상 실패 시 fallback 없이 fail-fast
- 통합 경계: mount owner와 동일 host uid 접근이 기본 전제이며, chroot 권한 모델과 `/proc`·`/sys`·`/dev`·`/run` native semantics는 상위 supervisor 책임이다
- 다이어그램 source of truth: `docs/diagrams/*.mmd`
- 다이어그램 렌더링 계약: `docs/diagrams/README.md`

## 1. 시스템 컨텍스트

ScreenFS는 `source_root`(대표 예시는 `/`)를 backing tree로 삼아 FUSE mount를 만들고, 그 mount를 sandbox/chroot 같은 상위 consumer가 읽는 구조다. `pi-bash-sandbox`는 대표 통합 예시지만 시스템 경계 자체는 특정 supervisor 하나에 고정되지 않는다. 아래 그림의 mount path는 `/tmp/screenfs-root` 예시일 뿐 고정 경로가 아니다.

정책 모델은 두 축이다.

- visibility axis
  - `visibility.default`
  - `visibility.hidden`
  - `visibility.visible`
  - bridge-visible ancestor
- mutability axis
  - `mutability.default`
  - `mutability.readonly`
  - `mutability.writable`
  - hidden-before-mutability precedence

현재 소스/증거를 이 축으로 읽으면 hidden `ENOENT`, bridge-visible ancestor, readonly/writable override, mount-root recursion exclusion이 핵심 축이다. mount-level `ro`만으로는 충분하지 않다.

![ScreenFS system context](diagrams/system-context.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

구현 파일 바로가기: `README.md`, `docs/design.md`, `docs/operations.md`

## 2. 모듈 아키텍처

현재 코드는 `main -> cli/config -> fs`를 중심으로 구성되어 있고, `src/fs.rs`는 module root/orchestrator로서 `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`에 inode/file-handle state, path guard, confined host access를 위임한다. `src/matcher.rs`의 `PathRuleMatcher`는 visibility/mutability rule compilation과 runtime policy checks에 공유된다.

![ScreenFS module architecture](diagrams/module-architecture.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

핵심 포인트:

- `src/main.rs`: mount option 구성과 `Session::run(ScreenFs::new(cfg))` 진입점
- `src/cli.rs`: launch input 파싱, default/override 관계 검증, help text와 fail-fast 진입점
- `src/config.rs`: `RuntimeConfig` 구성, mount-root recursion exclusion internal hidden rule 주입, visibility/mutability source 기록, matcher compilation, config/launch precedence 조립
- `src/path.rs`: lexical virtual path normalization, symlink target lexical resolution, source-root confinement 보조, relative/`~` exact·prefixed-glob rule input rebasing helper
- `src/matcher.rs`: `PathRuleMatcher`와 `MatcherScope`를 제공한다. exact/prefix/limited glob matcher를 compile하며 네 rule surface가 같은 normalization contract를 공유한다.
- `src/fs.rs`: `ScreenFs` FUSE 구현의 module root/orchestrator. request entrypoint를 `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`와 조합한다.
- `src/fs/state.rs`: inode/path map, lookup/open refcount, file/directory handle snapshot state를 관리한다.
- `src/fs/guards.rs`: hidden guard, bridge-visible guard, symlink target guard, mutation coordinate guard, directory filtering과 reply-building 전 검사를 담당한다.
- `src/fs/backing.rs`: `source_root` confinement 하의 host delegation, open/statfs/xattr/setattr helper를 담당한다.
- `src/errors.rs`: hidden 우선 `ENOENT`, mutation 차단 `EROFS`, host errno 보존 규칙을 담당한다.

구현 파일 바로가기: `src/main.rs`, `src/cli.rs`, `src/config.rs`, `src/path.rs`, `src/matcher.rs`, `src/errors.rs`, `src/fs.rs`, `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`

## 3. 요청 처리 결정 흐름

많은 조회·traversal 요청은 virtual path 계산 후 visibility 판정이 먼저, visible carve-out reachability에 따른 bridge-visible 합성과 symlink target fully-visible 검사, 그 뒤 mutability evaluator 판정이 이어지고, host filesystem delegation이 마지막 순서로 진행된다.

1. virtual path 계산
2. `visibility.hidden` / `visibility.visible` 평가
3. 필요 시 visible descendant reachability에 따라 bridge-visible ancestor 합성
4. 필요 시 symlink target이 fully visible인지 검사
5. mutation이면 `mutability.default` + 더 구체적인 readonly/writable override 평가
6. hidden이 아니고 mutation이 허용되면 host filesystem delegation

![ScreenFS request decision flow](diagrams/request-decision-flow.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

정책 축 세부 흐름:

![ScreenFS visibility axis](diagrams/visibility-axis.svg)

![ScreenFS mutability axis](diagrams/mutability-axis.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

핵심 포인트:

- hidden path는 가능한 한 존재하지 않는 것처럼 보여야 한다.
- hidden 판단은 mutability보다 먼저 적용된다.
- readonly/writable은 visible mutation에만 적용된다.
- symlink는 entry path뿐 아니라 resolved virtual target도 검사하며, target이 fully visible이 아니고 hidden 또는 bridge-visible이면 `ENOENT`다.
- `rename`/`link`/`symlink`/`copy_file_range` 같은 multi-path 연산은 source/target/parent 각각을 다시 검사한다.
- bridge-visible ancestor는 traversal/listing 전용이며 mutation에는 `EROFS`다.

구현 파일 바로가기: `src/fs.rs`, `src/errors.rs`, `docs/design.md`

## 4. 경로 정규화와 confinement

`PathRuleMatcher` 기반 matching은 host canonical path가 아니라 lexical virtual path 기준으로 동작한다. 이후 backing 접근은 `source_root` 밖으로 빠져나가지 않도록 제한한다. 아래 다이어그램은 특히 visible symlink entry를 직접 다루는 경로를 기준으로 읽는 것이 정확하다.

![ScreenFS path resolution and confinement](diagrams/path-resolution.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

핵심 포인트:

- `.` 제거, 중복 `/` 정리, `..`는 virtual `/` 위로 못 올라감
- 현재 구현의 shared rule grammar는 hidden/visible/readonly/writable 전체에서 같은 normalization contract를 재사용한다.
- current shared grammar의 supported glob은 prefix 없는 `**/<basename>` / `**/*.<suffix>` / `**/<basename-prefix>*`, 그 shorthand인 bare slashless glob(`*.pem`, `*.key`, `.env.*`, `id_*`), recursive optional normalized prefix가 붙은 limited glob, normalized-prefix direct-child basename-prefix/suffix form, recursive literal descendant-subtree form까지 포함한다. bare slashless glob `<pattern>`은 launch process cwd를 `source_root` relative normalized prefix로 rebase한 `./<pattern>` direct-child shorthand이며, 같은 cwd anchor 바로 아래 child basename과 matched child descendants에만 적용된다.
- canonical 4-family 비교(`**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`)는 `docs/requirements.md`의 table을 따른다. 아키텍처적으로 중요한 차이는 prefixless `**/*.pem`만 whole-tree recursive family라는 점이고, `./fixtures/*.pem`와 `/a/*.txt`는 direct-child anchored family지만 current bridge-index walk는 startup에서 anchor 아래를 재귀 탐색할 수 있다는 점이다. `/a/**/*.txt`는 같은 `/a` anchor라도 더 넓은 recursive discovery를 요구한다.
- relative path와 leading `~`, `~/...` prefix는 host path로 해석된 뒤 `source_root` 내부일 때만 virtual absolute path 또는 virtual glob prefix로 rebase된다.
- `HOME` 없음, relative/prefixless form의 launch cwd가 `source_root` 밖, `source_root` 밖으로 확장됨, `~user`, prefix 내부 wildcard, broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`, one-sided subset 밖의 bare wildcard `*`, `a*b`, `*secret*`)은 fail-fast/unsupported로 남는다.
- descendant-subtree broader forms(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`, trailing `/**` 없는 `**/.git/hooks`)도 부분 해석 없이 fail-fast다.
- descendant-subtree carve-out(`**/.git/hooks/**` 등)이 visible subtree를 열면 그 경로상의 모든 existing ancestor directory가 bridge-visible이어야 traversal/listing이 성립한다.
- symlink target은 lexical virtual target으로 재해석해 fully-visible 여부를 다시 검사하며, hidden 또는 bridge-visible target은 `readlink`/dereference에서 `ENOENT`다.
- host backing 접근은 `source_root` 밖 escape를 허용하지 않는다.

구현 파일 바로가기: `src/path.rs`, `src/matcher.rs`, `src/fs.rs`

## 5. 운영/통합 경계

이 아키텍처가 전제하는 운영 경계는 다음과 같다.

- mount 생성은 non-root 사용자 + `fusermount3` 기준이다.
- `FUSE_OVER_IO_URING` 협상 실패 시 mount를 degraded fallback으로 열지 않고 fail-fast 한다.
- 기본 접근 모델은 mount owner와 동일 host uid다.
- 다른 host uid 접근은 `allow_other`와 `/etc/fuse.conf` 정책이 별도로 필요하다.
- `chroot` 실행 권한, user namespace, supervisor 구성은 `ScreenFS` 바깥 책임이다 (`pi-bash-sandbox`는 대표 예시일 뿐 유일한 상위 레이어는 아님).
- `/proc`·`/sys`·`/dev`·`/run`의 native semantics 재현도 `ScreenFS` 단독 책임이 아니다.

구현 파일 바로가기: `README.md`, `docs/design.md`, `docs/operations.md`

## 6. 검증 경계

이 문서는 구조를 설명하는 artifact이고, live proof의 source of truth는 `docs/operations.md`다. 이 artifact를 읽고 바로 연결되어야 하는 최소 검증 질문은 다음이다.

- `visibility.hidden` entry가 `readdir`/`readdirplus`에서 실제로 빠지는가
- hidden 직접 접근이 `ENOENT`인가
- visibility axis diagram이 hidden/bridge-visible/visible 흐름, symlink fully-visible gate, listing 제한을 설명하는가
- mutability axis diagram이 hidden-before-EROFS와 affected-coordinate writable 요구를 설명하는가
- `visibility.visible` carve-out이 있을 때 bridge-visible traversal/listing이 유지되고 descendant-subtree pattern도 실제 ancestor bridge synthesis로 도달 가능한가
- `mutability.default=writable`에서 readonly match가 `EROFS`인가
- `mutability.default=readonly`에서 writable carve-out success와 carve-out 밖 `EROFS`가 갈리는가
- nested `mutability.readonly` re-block가 다시 `EROFS`를 만드는가
- hidden-before-mutability precedence가 유지되는가
- unsupported glob이 fail-fast 하는가
- `*.pem`가 current launch cwd anchor에서 `./*.pem`과 동등하게 normalize되고, `**/*.pem`과는 same-specificity equivalent가 아니라 containment/nested-override 관계로 처리되는가
- `source-root=/`에서 `/bin`, `/usr`, `/etc` 같은 whole-view 경로가 실제로 보이는가
- mount-root recursion exclusion이 listing/lookup에 다시 나타나지 않는가
- symlink entry가 resolved virtual target이 fully visible할 때만 읽히고 bridge-visible target이면 `ENOENT`인가
- live/performance smoke가 bridge-visible reachability를 startup bridge ancestor index 또는 동등한 bounded/cacheable 구조로 유지하고 request-time whole-root recursive scan 없이 동작함을 보여주는가
- `unshare -UrR` 기반 chroot smoke가 성공하는가, 그리고 `/dev/null` 같은 device-node semantics가 supervisor/namespace layer 책임으로 명확히 남는가
- archival transcript와 current baseline 설명을 혼동하지 않았는가

구현 파일 바로가기: `docs/operations.md`, `docs/artifacts/future-mutability-smoke-transcript.md`, `docs/artifacts/whole-root-family-smoke-transcript.md`, `docs/artifacts/whole-mount-readonly-smoke-transcript.md`, `docs/artifacts/fuse-smoke-transcript.md`, `README.md`

## 7. 현재 구현이 드러내는 아키텍처 요약

```text
Launch inputs
  -> RuntimeConfig
  -> PathRuleMatcher + internal mount-root prefix
  -> compiled visibility hidden/visible rule sets
  -> current mutability default + compiled readonly/writable rule sets
  -> src/fs.rs orchestrator + fs/{state,guards,backing}
  -> VirtualPath normalization + hidden-before-mutability evaluator
  -> confined host filesystem access
  -> FUSE replies to whole-root consumer (예: sandbox/chroot)
```

구현 파일 바로가기: `src/cli.rs`, `src/config.rs`, `src/fs.rs`, `src/path.rs`

## 8. 문서 간 역할 분담

- `README.md`: 프로젝트 목적, 운영 예시, 현재 상태 요약
- `docs/requirements.md`: v1 요구사항과 금지/비목표
- `docs/design.md`: 의미론과 모듈 경계에 대한 계약
- `docs/operations.md`: 검증 방식과 smoke evidence
- `docs/architecture.md`: 위 문서와 현재 코드를 함께 읽기 쉽게 시각화한 요약

구현 파일 바로가기: `README.md`, `docs/requirements.md`, `docs/design.md`, `docs/operations.md`
