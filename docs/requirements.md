# ScreenFS 요구사항

## 1. 목적

`ScreenFS`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 존재하지 않는 것처럼 숨기고, visible path에는 별도 readonly policy를 적용할 수 있어야 한다. `pi-bash-sandbox`는 이 요구사항을 소비하는 대표 통합 예시지만, `ScreenFS`의 목적을 그 통합 하나로 한정하지 않는다.

## 2. 핵심 전제

- non-root에서 실행된다.
- `fractal-fuse = 0.4.0` 기반으로 구현한다.
- FUSE3 및 `FUSE_OVER_IO_URING` 기반 사용을 v1 필수 정책으로 삼는다.
- v1은 `FUSE_OVER_IO_URING` 협상 실패 시 fallback mount를 만들지 않고 명시적 오류로 fail-fast 한다.
- mount는 `fusermount3`로 수행한다.
- 대표 사용 시나리오에 chroot가 포함되므로 mount 결과는 전체 파일시스템 뷰를 제공해야 한다.
- `chroot` 실행 권한, same-host-uid 접근 모델, `/proc`·`/sys`·`/dev`·`/run` native semantics는 `ScreenFS` 단독 책임이 아니라 상위 supervisor/namespace layer 책임이다. `pi-bash-sandbox`는 그 책임을 지는 대표 예시다.

## 3. Non-root 실행

`ScreenFS`는 root 권한 없이 실행되어야 한다.

- 일반 사용자 권한으로 FUSE mount 가능해야 한다.
- `fusermount3`를 사용한다.
- root-only mount에 의존하지 않는다.
- privileged bind mount에 의존하지 않는다.
- system-wide mount namespace 조작에 의존하지 않는다.
- `/dev/null` bind overlay 같은 root/mount namespace 기반 masking에 의존하지 않는다.

## 4. 상위 consumer 요구 충족

`ScreenFS`는 상위 sandbox/chroot/orchestrator가 요구하는 filesystem view를 제공해야 한다. `pi-bash-sandbox`는 그 요구를 대표하는 통합 예시다.

- sandboxed bash가 필요한 시스템 경로를 볼 수 있어야 한다.
- 실행에 필요한 binary, shared library, config, runtime path가 보존되어야 한다.
- 권한 정책상 허용된 경로는 정상 접근 가능해야 한다.
- 차단/숨김 경로만 존재하지 않는 것처럼 처리해야 한다.
- 숨김 대상은 `Permission denied`로 존재를 드러내지 않고 가능한 한 `ENOENT`로 처리한다.

## 5. Whole-root consumer용 전체 파일시스템 뷰

대표 통합 시나리오에서 chroot를 사용하므로 `ScreenFS`는 단순 프로젝트 디렉터리 view가 아니라 전체 `/` filesystem view를 제공해야 한다. 이 whole-root view 요구는 `pi-bash-sandbox` 외의 다른 consumer에도 동일하게 적용된다.

```text
real / -> screenfs mount root -> chroot root
```

chroot 내부에서는 다음 같은 경로 구조가 보여야 한다.

```text
/
/bin
/usr
/lib
/lib64
/etc
/home
/tmp
/var
...
```

상정 사용법:

```bash
screenfs / /tmp/screenfs-root ...
chroot /tmp/screenfs-root /bin/bash
```

## 6. Hidden path 처리

지정한 숨김 path는 실제로 없는 것처럼 보여야 한다.

필수 동작:

| Operation | 기대 동작 |
| --- | --- |
| `readdir` | 결과에서 제외 |
| `readdirplus` | 결과에서 제외 |
| `lookup` | `ENOENT` |
| `getattr` | `ENOENT` |
| `open` | `ENOENT` |
| `access` | `ENOENT` |

일반 명령에서 기대 동작:

```bash
ls
find
rg
stat hidden-path
cat hidden-path
```

결과:

- 목록에 보이지 않는다.
- 직접 접근 시 `No such file or directory`로 실패한다.

금지 방식:

- `/dev/null` bind
- 빈 파일 overlay
- tmpfs로 덮어 이름만 남기는 방식
- 단순 `Permission denied`로 존재를 노출하는 방식

## 7. 전체 view + selective hiding

기본적으로 전체 파일시스템은 pass-through 하되, hide rule에 걸린 경로만 숨긴다.

예상 숨김 후보:

```text
/home/<user>/.ssh
/home/<user>/.aws
/home/<user>/.gnupg
/home/<user>/.pi/agent/auth.json
/home/<user>/.pi/agent/mcp-oauth
**/.env
**/.env.*
**/.git/hooks/**
**/*.pem
**/*.key
```

## 8. Selective readonly rule

`ScreenFS`는 hide rule과 별개로 visible path의 mutability policy를 제어해야 한다. 현재 요구사항의 기본 target contract는 특정 path/pattern에만 쓰기 금지를 적용하는 selective readonly rule이다.

### 8.1 Current target contract: path-scoped selective readonly

현재 목표 계약(path-scoped selective readonly)에서:

- readonly rule에 매치된 visible path는 read/stat/list 허용
- hidden path는 selective readonly 여부와 관계없이 `ENOENT`
- readonly rule에 매치된 path의 모든 쓰기성 operation은 `EROFS`
- readonly rule에 매치되지 않은 visible path는 이 요구사항만으로 자동 readonly가 되지 않음
- canonical CLI/config contract는 11절을 따른다. 현재 소스는 그 surface와 config `mutability` block을 구현했고, source/unit-test evidence가 있다. existing repo-local/whole-root FUSE smoke는 family baseline evidence로 분리해 읽으며, `docs/artifacts/whole-root-family-smoke-transcript.md`는 `readonly-root-allowwrite --allow-write /tmp` carve-out smoke다. selective-readonly whole-root/chroot live smoke와 option B nested live smoke는 아직 별도 fresh evidence가 필요하고, pre-removal historical transcript는 archival evidence로 분리해 읽는다.
- family 내부 nested override(option B)는 현재 canonical mutability contract에 반영됐다. 설계 배경과 detailed validation rationale은 `docs/nested-mutability-option-b.md`에 남긴다.

### 8.2 Shared invariants across policy families

다음 invariants는 현재 target contract와 future carve-out family 모두에 공통으로 유지돼야 한다.

- hidden path는 어떤 mutability policy family에서도 계속 `ENOENT`
- hidden `ENOENT` 우선순위는 readonly/allowWrite 같은 후행 policy 판정보다 앞선다.
- write-intent `open`, path-only mutation, multi-path mutation(`rename`, `link`, `symlink`, `copy_file_range`) 모두 operation별 affected coordinate 기준으로 판정돼야 한다. 단, `copy_file_range`는 source visibility와 destination writability를 분리해 본다.
- hide rule, current `--readonly-rule`, `--allow-write` surface는 같은 normalization contract를 재사용해야 한다.

### 8.3 Alternate family in current source: `readonly-root-allowwrite`

이 문서는 `readonly-root-allowwrite`를 현재 소스와 source/unit-test evidence에 반영된 alternate policy family로 정의한다.

- 이 alternate model은 `pi-bash-sandbox` 같은 integration use case에서 흔한 "기본은 readonly, 일부 path만 writable" 요구를 설명하는 대표 예시일 수 있다.
- 하지만 이는 현재 target selective readonly contract를 대체하지 않으며, live FUSE smoke evidence를 그런 정책까지 이미 완료된 것으로 확장해서 읽으면 안 된다.

이 family에 대해 이 문서에서 확정하는 semantics:

- 한 mount는 정확히 하나의 mutability policy family만 선택한다. current path-scoped selective readonly family와 carve-out family를 같은 mount에서 동시에 활성화하지 않는다.
- visible read/stat/list는 hidden이 아니면 pass-through다.
- visible mutation은 기본적으로 `readonly-root-allowwrite` family의 default readonly 정책에 의해 `EROFS`다.
- `allowWrite` rule들은 union semantics로 합쳐지며, 하나라도 매치되면 해당 mutation coordinate는 writable 후보가 된다.
- hidden path나 hidden target이 하나라도 관여하면 `allowWrite`와 무관하게 결과는 `ENOENT`다.
- mutation은 관련된 모든 write-requiring affected coordinate가 carve-out family 기준으로 writable이어야만 허용된다.
- write-intent `open`, path-only mutation, multi-path mutation, destination mutation이 있는 `copy_file_range` 모두 같은 affected-path evaluation family를 공유한다.
- `copy_file_range`에서는 source는 hidden/read visibility 대상이고, destination path와 destination parent는 writable이어야 한다.
- current `allow_write`/`--allow-write` surface도 hide/current `--readonly-rule`와 같은 exact path + supported prefixed glob normalization contract를 재사용한다.
- canonical contract는 legacy bool surface를 포함하지 않으며, config schema에도 별도 legacy bool을 두지 않는다.
- allowWrite는 ScreenFS 차원의 `EROFS`를 제거할 뿐이며, 최종 성공 여부는 계속 host filesystem 권한/소유권/LSM에 의존한다.

추가 경계 규칙:

- mutability family 집합은 현재 spec에서 `selective-readonly`와 `readonly-root-allowwrite` 두 개로 닫는다.
- future deny-like family와의 조합이나 제3 family 추가는 현재 spec 범위 밖이며, 별도 버전의 새 정책 family로만 도입할 수 있다.
- whole-mount readonly semantics가 필요하면 `readonly-root-allowwrite` family를 empty `allow_write`와 함께 사용한다.
- canonical contract는 제거된 bool shorthand를 복원하지 않는다.
- family-preserving nested override는 one-family-per-mount를 유지한 채 primary rule 아래 더 구체적인 opposite-polarity secondary rule만 허용한다.

쓰기성 operation 예:

```text
write
create
mkdir
mknod
unlink
rmdir
rename
link
symlink
setattr 중 mutation
fallocate
```

## 9. Rule input 정규화 계약

이 절은 hide rule과 현재 `--readonly-rule` surface 및 그 향후 확장이 공유해야 할 path-like rule normalization contract를 정의한다. exact path와 supported glob을 함께 다루되, current implementation snapshot과 target behavior를 분리해서 기록한다.

현재 구현 snapshot(코드 + mount-free unit test 기준):

- exact path와 supported prefixed glob 모두 같은 normalization contract를 공유한다.
- 이미 absolute인 exact path와 absolute prefixed glob은 virtual-root anchored semantics를 유지한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 host path가 `source_root` 내부일 때만 source-root-relative virtual absolute path 또는 virtual glob prefix로 rebase한다.
- `~`/`~/...` 입력은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- current code/evidence가 검증한 supported pattern 입력은 recursive `**/<basename>`, `**/*.<suffix>`, `**/<basename-prefix>*` tail, normalized-prefix direct-child basename-prefix/suffix form(`<normalized-prefix>/<basename-prefix>*`, `<normalized-prefix>/*.<suffix>`), 그리고 limited recursive literal descendant-subtree glob(`<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`)까지다. 대표 예시는 `**/.env`, `**/.env.*`, `**/*.pem`, `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, `/home/<user>/**/*.pem`, `/home/<user>/.env.*`, `~/.env.*`, `./fixtures/*.pem`, `~/*.pem`, `/home/<user>/*.pem`, `**/.git/**`, `**/.git/hooks/**`, `/home/spi-ca/Codebase/the-onion/palgong/**/.git/hooks/**`이다.
- descendant-subtree glob은 첫 literal component 앞에 recursive gap이 있고, literal tail subtree root 자체와 그 모든 descendants에 매치된다. 예를 들어 `**/.git/hooks/**`는 `.git/hooks`, `.git/hooks/pre-commit`, `.git/hooks/subdir/x`를 모두 포함한다.
- source/test evidence는 `src/matcher.rs`의 `matches_recursive_literal_descendant_subtree_globs`, `recursive_literal_descendant_subtree_rules_preserve_specificity_and_containment`, `recursive_literal_descendant_subtree_specificity_prefers_longer_tail_over_longer_prefix`, `matches_requested_absolute_descendant_subtree_glob_pattern`, `src/config.rs`의 `descendant_subtree_globs_share_runtime_config_grammar_across_surfaces`, `selective_readonly_descendant_subtree_nested_allow_write_uses_literal_tail_specificity`, `runtime_config_accepts_requested_absolute_descendant_subtree_pattern`, `src/fs.rs`의 `selective_readonly_descendant_subtree_rule_only_locks_git_hooks_subtree`, `hidden_descendant_subtree_rules_keep_enoent_precedence_over_readonly`, 그리고 direct-child subset/fail-fast coverage를 포함한다.
- fail-fast/unsupported 입력: `HOME` 없음, expanded path outside `source_root`, `~user`, prefix 내부 wildcard, broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`, bare suffix `*.pem`)은 모두 fail-fast다. descendant-subtree 관련 broader forms(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`, trailing `/**` 없는 `**/.git/hooks`)도 부분 해석 없이 fail-fast 대상으로 유지한다.

현재 문서화된 current-vs-target contract:

- exact path와 supported prefixed glob 모두 같은 normalization contract를 공유한다.
- 이미 absolute인 exact path와 absolute prefixed glob은 현재 virtual-root anchored 의미를 유지한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 host path가 `source_root` 내부일 때만 source-root-relative virtual absolute path 또는 virtual glob prefix로 rebase한다.
- `~`/`~/...` 입력은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- supported prefixed glob grammar는 recursive basename/suffix/basename-prefix tail(`**/<basename>`, `**/*.<suffix>`, `**/<basename-prefix>*`)과 normalized-prefix direct-child basename-prefix/suffix form(`<normalized-prefix>/<basename-prefix>*`, `<normalized-prefix>/*.<suffix>`)을 유지하면서, shared extension으로 limited recursive literal descendant-subtree glob(`<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`)도 허용한다. canonical examples: `**/.git/**`, `**/.git/hooks/**`, `/home/<user>/project/**/.git/hooks/**`, `./repo/**/.git/hooks/**`, `~/project/**/.git/hooks/**`.
- descendant-subtree glob은 첫 literal component 앞에 recursive gap이 있고, literal tail subtree root 자체와 그 모든 descendants에 매치된다. 예를 들어 `**/.git/hooks/**`는 `.git/hooks`, `.git/hooks/pre-commit`, `.git/hooks/subdir/x`를 모두 포함한다.
- nested override/ancestor 판단은 raw input이 아니라 normalization 이후의 target set 기준으로 수행한다. descendant-subtree glob끼리는 literal tail component 수가 더 많을수록, 그다음으로 normalized prefix가 더 길수록 더 구체적이다. 따라서 `**/.git/hooks/**`는 `**/.git/**` 내부의 더 구체적인 target set이다.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user`는 계속 unsupported이며 fail-fast 한다.
- wildcard가 prefix 내부에 섞이는 더 넓은 glob(`foo/*/bar.pem`, `**/secret?.pem`, unprefixed bare suffix `*.pem`, brace/env/command expansion`)과 descendant-subtree literal tail 내부 wildcard(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`) 또는 trailing `/**` 없는 form(`**/.git/hooks`)은 이번 범위에서도 unsupported/fail-fast로 남긴다.
- `--hide`, `--readonly-rule`, `--allow-write`는 같은 shared grammar를 사용해야 한다. hide/current mutability rules는 current source/unit-test 범위에서 exact, recursive, direct-child, descendant-subtree subset까지 같은 normalization contract와 hidden `ENOENT` precedence를 재사용한다.
- descendant-subtree extension의 남은 evidence gap은 already-absolute/relative/`~` live mount smoke, broader unsupported wildcard fail-fast live smoke, option B nested override live smoke다.

## 10. 성능 및 메모리 요구사항

전체 filesystem view를 제공하므로 metadata-heavy workload가 많을 수 있다. 중요한 최적화 방향:

- `readdirplus` 구현
- inode/path cache
- file handle 재사용
- hidden matcher 빠른 처리
- glob rule 증가 시 성능 저하 최소화
- visible file data path는 최대한 underlying filesystem으로 pass-through
- memory footprint는 장시간 실행에도 과도하게 증가하지 않아야 함

## 11. CLI 예시

기본 hide 예시:

```bash
screenfs / /tmp/screenfs-root \
  --hide /home/spi-ca/.ssh \
  --hide /home/spi-ca/.aws \
  --hide /home/spi-ca/.pi/agent/auth.json \
  --hide '/home/spi-ca/.pi/agent/mcp-oauth' \
  --hide '**/.env' \
  --hide '**/.env.*' \
  --hide '**/*.pem' \
  --hide '**/*.key'
```

Current canonical mutability CLI contract:

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--hide <rule> ...] \
  [--policy-family selective-readonly|readonly-root-allowwrite] \
  [--readonly-rule <rule> ...] \
  [--allow-write <rule> ...]
```

Current CLI semantics:

- `--config <path>`는 current config contract의 `mutability` block source를 제공한다.
- `--policy-family selective-readonly|readonly-root-allowwrite`는 mount당 하나의 mutability policy family를 명시한다.
- `--readonly-rule`는 `selective-readonly` family의 primary readonly rule이고, `readonly-root-allowwrite` family에서는 더 구체적인 secondary re-block rule이다.
- `--allow-write`는 `readonly-root-allowwrite` family의 primary writable carve-out rule이고, `selective-readonly` family에서는 더 구체적인 secondary carve-out rule이다.
- canonical contract는 explicit family/rule surface만 사용한다. whole-mount readonly shorthand는 제공하지 않는다.
- family 선택 추론 규칙:
  - CLI mutability option이 하나라도 있으면 CLI가 family를 결정하고 config `mutability` block 전체를 대체한다.
  - explicit `--policy-family`가 있으면 그 값을 사용한다.
  - explicit family가 없고 `--allow-write`만 있으면 family=`readonly-root-allowwrite`로 본다.
  - explicit family가 없고 `--readonly-rule`만 있거나 mutability rule이 없으면 family=`selective-readonly`로 본다.
  - CLI mutability option이 없고 config `mutability.family`가 있으면 그 값을 사용한다.
  - CLI mutability option도 없고 config `mutability.family`도 없으면 family=`selective-readonly`가 기본값이다.
- conflict/fail-fast 규칙:
  - `--readonly-rule`와 `--allow-write` 동시 사용은 explicit `--policy-family`가 없으면 fail-fast다.
  - `selective-readonly`에서 secondary `--allow-write`는 less-specific ancestor `--readonly-rule` 아래에 있어야 한다.
  - `readonly-root-allowwrite`에서 secondary `--readonly-rule`는 less-specific ancestor `--allow-write` 아래에 있어야 한다.
  - same-specificity opposite-polarity conflict와 primary ancestor 없는 secondary rule은 fail-fast다.

Current selective-readonly CLI example:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family selective-readonly \
  --readonly-rule /etc/ssh \
  --readonly-rule '~/.config/**/*.json' \
  --hide /home/spi-ca/.ssh \
  --hide '~/.env.*'
```

Current config contract:

```yaml
mutability:
  family: selective-readonly | readonly-root-allowwrite
  readonly_rules: []
  allow_write: []
```

- `readonly_rules`와 `allow_write`가 모두 non-empty이면 `mutability.family`는 필수다.
- `family=selective-readonly`이면 `allow_write`는 primary `readonly_rules` 아래 더 구체적인 carve-out rule일 때만 허용한다.
- `family=readonly-root-allowwrite`이면 `readonly_rules`는 primary `allow_write` 아래 더 구체적인 re-block rule일 때만 허용한다.
- config schema는 legacy `readonly: true|false` bool을 두지 않는다.
- current config contract는 제거된 bool surface를 별도 key로 보존하지 않는다.
- CLI mutability options가 하나라도 있으면 config의 mutability block 전체를 대체한다.
- CLI mutability option이 없으면 config `mutability` block이 canonical source of truth다.
- duplicate same-polarity rule은 허용하지만 semantics는 idempotent다; opposite-polarity same-specificity conflict는 fail-fast다.

Current readonly-root-allowwrite CLI example:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family readonly-root-allowwrite \
  --allow-write /tmp \
  --allow-write /home/spi-ca/workspace \
  --allow-write '~/.cache/**/*.lock' \
  --hide /home/spi-ca/.ssh
```

현재 구현/검증 기준 whole-mount readonly 예시:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family readonly-root-allowwrite \
  --hide /home/spi-ca/.ssh
```

이후:

```bash
chroot /tmp/screenfs-root /bin/bash
```

## 12. 현재 프로젝트 상태

프로젝트 경로:

```text
/home/spi-ca/Codebase/screenfs
```

현재 반영된 구현/검증 상태 요약:

- Rust 모듈 구현: `cli`, `config`, `errors`, `path`, `matcher`, `fs`
- CLI/config surface: `<source-root> <mount-root>`, 반복 `--hide`, 반복 `--readonly-rule`, `--policy-family`, 반복 `--allow-write`, `--config`, YAML `mutability` block이 구현돼 있다.
- mutability family resolution: `selective-readonly`/`readonly-root-allowwrite`, one-family-per-mount, CLI-over-config precedence, CLI 부재 시 config source-of-truth, 둘 다 없을 때 `selective-readonly` 기본값이 구현돼 있다.
- hide matcher: virtual root 기준 absolute exact rule, relative/`~` exact rule rebasing, directory prefix hiding, prefix 없는 limited glob, absolute/relative/`~` prefixed limited glob
- hidden/readonly guard: hidden path는 `ENOENT`, rule-matched visible mutation은 `EROFS`, hidden precedence는 selective/carve-out family 모두에서 유지된다.
- current `--readonly-rule`와 `--allow-write` surface는 hide와 같은 normalization contract를 재사용하고, nested override에서는 most-specific-match-wins와 primary/secondary ancestor validation을 적용한다. unit tests가 source-level option B evidence를 제공하며, existing repo-local family-aware live smoke transcript는 pre-option-B baseline으로 분리해 읽는다.
- 기본 FUSE 조회 경로: `lookup`, `getattr`, `open`, `read`, `readdir`, `readdirplus`, `readlink`, `access`, `statfs`
- mount-free unit test coverage includes relative/`~` exact·prefixed-glob normalization, direct-child basename-prefix/suffix glob normalization, descendant-subtree glob normalization/specificity regressions, hide/current mutability rule shared semantics, `selective_readonly_rules_are_scoped_to_matching_paths`, option B nested allow-write/re-block and fail-fast cases, `readonly_root_allowwrite_match_non_match_and_hidden_precedence`, `readonly_root_allowwrite_requires_writable_parent_for_path_only_and_multi_path_mutation`, `readonly_root_allowwrite_copy_file_range_requires_writable_destination_parent`, config mutability load/override를 포함한다. latest recorded full-suite cargo pass evidence는 current descendant-subtree regression tree 기준 `cargo fmt --check`, `cargo check`, `cargo clippy --all-targets --all-features`, `cargo test --all-targets --all-features`(88 tests 통과)다. repo-local family-aware live smoke transcript는 pre-option-B runtime baseline으로 분리한다.
- live FUSE smoke: `/dev/fuse`가 있는 환경에서 repo-local family-aware fixture mount, current `source-root=/` whole-root carve-out mount, current `source-root=/` whole-mount readonly mount가 성공한 documented evidence가 있다. pre-removal whole-root/chroot transcript는 archival artifact로 유지한다. 현재 세션의 fresh 상태 표기는 `docs/operations.md`를 source of truth로 따른다.
- user-namespace chroot smoke: current carve-out whole-root transcript와 current whole-mount readonly transcript에서 `unshare -UrR <mount> /bin/bash --noprofile --norc ...` allow/block smoke를 확인했다. pre-removal transcript의 `unshare -UrR` smoke는 archival evidence로 유지한다. 현재 세션의 fresh 상태 표기는 `docs/operations.md`를 source of truth로 따른다.
- 아직 미주장 범위: selective-readonly family의 whole-root/chroot live FUSE smoke, production-ready 전체 FUSE 완성, privileged supervisor end-to-end 운영 검증, `/dev/null` 등 device-node namespace 구성

확인된 환경:

```text
kernel: 7.1.0-rc6-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
```

위 kernel config artifact는 현재 개발 환경이 `FUSE_OVER_IO_URING` 전제를 충족하도록 빌드되었음을 보여주는 정적 증거다. 이는 별도의 live mount smoke나 세션 협상 성공 주장과 동일하지 않다.
