# 운영 및 검증

## 환경 전제

현재 확인된 환경:

```text
kernel: 7.1.0-rc6-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
/dev/fuse: present
```

위 kernel config artifact는 현재 커널 빌드 설정에 `FUSE_OVER_IO_URING` 관련 옵션이 활성화되어 있음을 보여주는 정적 근거다. 이는 mount 성공이나 세션 협상 성공을 새로 증명하는 smoke artifact는 아니다.

프로젝트 상태:

```text
/home/spi-ca/Codebase/holefs
Rust v1 core modules present: cli, config, errors, path, matcher, fs
fractal-fuse = 0.4.0
```

## 빌드 및 테스트 확인

이 문서는 현재 저장소의 최신 자동 검증 결과를 직접 재확인했을 때만 pass evidence로 갱신해야 한다.

표준 Rust 검증 명령:

```bash
cargo fmt --check
cargo check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features
```

현재 문서 갱신 세션의 실제 상태:

- 이번 세션에서는 문서 변경만 수행했고 코드는 수정하지 않았다.
- `cargo check`는 현재 저장소의 기존 compile blocker 때문에 실패 중이다 (`src/fs.rs`의 guard API 불일치 등).
- 따라서 이 절은 이전 코드 검증 이력을 보존하더라도, 이번 문서 갱신의 fresh pass evidence로 해석하면 안 된다.
- 문서 전용 검증으로는 파일 목록 확인 명령(`find README.md AGENTS.md docs ...`, `find .pi/agents .pi/skills .pi/prompts ...`)만 이번 세션에서 다시 실행했다.

## Mermaid 다이어그램 산출물 재생성

문서 다이어그램 source of truth는 `docs/diagrams/*.mmd`이며, 공용 렌더링 규칙은 `docs/diagrams/README.md`를 따른다.

- `docs/diagrams/mermaid-config.json`: Mermaid theme/font/color 설정
- `docs/diagrams/puppeteer-config.json`: Puppeteer/Chromium launch args 설정
- `docs/diagrams/*.png`: 위 두 config를 함께 사용하고 Mermaid CLI `--scale 2`로 렌더링해야 한다.
- `docs/diagrams/*.svg`: 같은 config를 사용하되 PNG 전용 `--scale 2` 규칙은 적용하지 않는다.

재생성 예시:

```bash
for input in docs/diagrams/*.mmd; do
  base="${input%.mmd}"

  npx -y @mermaid-js/mermaid-cli \
    -i "$input" \
    -o "${base}.svg" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json

  npx -y @mermaid-js/mermaid-cli \
    -i "$input" \
    -o "${base}.png" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json \
    --scale 2

done

file docs/diagrams/*.png
```

중요: 현재 추가된 두 JSON만으로는 PNG 2x scale이 자동 보장되지 않는다. PNG 품질 규칙은 명시적인 `--scale 2` 명령 인자로 유지한다.

## 최신 live FUSE smoke evidence

최신 확보 증거 기준으로 live mount 관련 상태는 다음과 같다.

- `/dev/fuse`가 존재하는 환경에서 FUSE mount가 실제로 성공했다.
- repo-local fixture mount smoke 성공:
  - hidden entry는 부모 listing에서 제외됐다.
  - hidden path에 대한 직접 `ls`/`stat`/`cat`류 접근은 `ENOENT`로 실패했다.
  - 현재 구현의 global `--readonly` mutation은 `EROFS`로 거부됐다.
- `source-root=/` whole-root mount smoke 성공:
  - `stat <mount>/bin/bash` 성공
  - `ls <mount>/usr` 성공
  - hidden `/home/spi-ca/.ssh` 접근은 `ENOENT`
  - current global `--readonly` mount에서 `touch`는 `EROFS`
- plain `chroot <mount> /bin/bash`는 권한 모델상 사용할 수 없지만, `unshare -UrR <mount> /bin/true`와 `unshare -UrR <mount> /bin/bash --noprofile --norc ...`는 성공했다. 단, FUSE mount는 `nodev`이므로 `/dev/null` 같은 device-node 동작은 상위 supervisor/namespace layer에서 별도 제공해야 한다. 관련 transcript artifact는 `docs/artifacts/fuse-smoke-transcript.md`에 있다.

중요: 요구사항/설계 목표로 읽어야 하는 readonly contract는 이제 "전체 mount readonly"가 아니라 "특정 path/pattern에만 적용되는 selective readonly rule"이다. 하지만 현재 코드와 live smoke evidence는 아직 global `--readonly` bool 동작만 증명한다. 아래 checklist도 이 둘을 분리해 기록한다.

## Smoke evidence recording rules

- smoke를 `완료` 또는 `성공`으로 표기하려면 실제 mount가 살아 있는 상태에서 후속 검증 명령까지 실행돼야 한다.
- kernel config artifact(`.../linux-spica-git/config.saved.x86_64`에서 `CONFIG_FUSE_IO_URING=y`, `CONFIG_IO_URING=y`)는 환경 전제 증거로만 기록하고, 단독으로 live smoke 성공으로 승격하지 않는다.
- 최소 기록 항목: mount 명령, source/mount 경로, hide 플래그, readonly 관련 플래그/정책 표현, `fusermount3 --version`, 후속 `ls`/`stat`/`cat`/mutation 명령, stderr/stdout 발췌, unmount 결과.
- repo-local fixture mount evidence는 hidden semantics와 현재 global readonly 구현 재확인에 유용하지만, whole-view와 chroot 관련 결론은 `source-root=/` 또는 동등한 전체 view 환경의 별도 증거로 보강해야 한다.
- selective readonly rule을 주장하려면 rule-match path와 non-match path를 모두 포함한 별도 smoke evidence가 필요하다. current global `--readonly` smoke만으로는 selective rule 검증으로 승격하지 않는다.
- rule-input normalization 변화를 주장하려면 already-absolute input과 relative/`~` exact input, relative/`~` prefixed glob input을 분리해 기록하고, broader unsupported wildcard forms와 fail-fast stderr도 함께 남긴다.
- current source surface와 current verified evidence를 구분한다. 예를 들어 `--readonly-rule` 파싱/매처가 소스에 있어도, live smoke나 test evidence가 global `--readonly` 중심이면 그렇게 분리 기록한다.
- mount 전에 실패했으면 상태는 `차단됨` 또는 `실패`로 적고, hidden/whole-view/read-only smoke를 `완료`로 승격하지 않는다.

## Mount 예시

아래 명령은 운영 예시다. 이번 문서 갱신의 검증 증거에는 포함되지 않는다.

기본 mount:

```bash
holefs / /tmp/holefs-root \
  --hide /home/spi-ca/.ssh \
  --hide /home/spi-ca/.aws \
  --hide /home/spi-ca/.pi/agent/auth.json \
  --hide '/home/spi-ca/.pi/agent/mcp-oauth' \
  --hide '**/.env' \
  --hide '**/*.pem' \
  --hide '**/*.key'
```

현재 구현의 global readonly mount:

```bash
holefs / /tmp/holefs-root \
  --readonly \
  --hide /home/spi-ca/.ssh
```

chroot 실행 예시:

```bash
chroot /tmp/holefs-root /bin/bash
```

`chroot` 자체는 `holefs`가 제공하는 기능이 아니며, `CAP_SYS_CHROOT`, privileged supervisor, user namespace 등 별도 권한 모델이 필요하다. 최신 live smoke에서는 plain `chroot` 대신 `unshare -UrR <mount> ...` 방식으로 `/bin/true`와 `/bin/bash` 실행을 확인했다. `holefs` v1의 기본 접근 모델은 mount owner와 동일 host uid다.

## Hidden path smoke checks

최신 repo-local/whole-root smoke evidence는 아래 조건을 충족했다. 필요 시 동일 절차로 재검증한다.

```bash
ls /tmp/holefs-root/home/spi-ca
find /tmp/holefs-root/home/spi-ca -maxdepth 2 -name .ssh
stat /tmp/holefs-root/home/spi-ca/.ssh
cat /tmp/holefs-root/home/spi-ca/.ssh/id_rsa
```

기대 결과:

- `.ssh` 등 hidden entry가 listing에 나타나지 않는다.
- 직접 접근은 `No such file or directory`로 실패한다.
- `Permission denied`로 존재가 드러나지 않는다.

## Whole-view smoke checks

최신 `source-root=/` smoke evidence는 아래 조건을 충족했다. 필요 시 동일 절차로 재검증한다.

```bash
ls /tmp/holefs-root
ls /tmp/holefs-root/bin
ls /tmp/holefs-root/usr
ls /tmp/holefs-root/lib
ls /tmp/holefs-root/lib64
ls /tmp/holefs-root/etc
```

기대 결과:

- `/`, `/bin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/home`, `/tmp`, `/var` 같은 시스템 경로가 보인다.
- hide rule에 걸리지 않은 binary, shared library, config, runtime path는 접근 가능하다.

## Current global `--readonly` smoke checks

최신 repo-local/whole-root smoke evidence는 아래 조건을 충족했다. 이 절은 현재 구현의 global `--readonly` bool 동작만 재검증한다.

```bash
stat /tmp/holefs-root/bin/bash
ls /tmp/holefs-root/etc
touch /tmp/holefs-root/tmp/holefs-write-check
mkdir /tmp/holefs-root/tmp/holefs-mkdir-check
```

기대 결과:

- `stat`, `ls`는 성공한다.
- `touch`, `mkdir` 등 mutation은 read-only filesystem 오류로 실패한다.
- hidden path는 current global `--readonly` 여부와 관계없이 `ENOENT`다.

## Rule-input normalization status and target checklist

현재 구현/증거 상태:

- 현재 live examples와 smoke transcript는 이미 절대화된 exact hide rule과 prefix 없는 limited glob만 사용한다.
- current parser/matcher는 relative exact path, leading `~`, `~/...`, `~user`를 지원하지 않는다.
- current glob grammar는 `**/<basename>` 또는 `**/*.<suffix>` 계열을 허용하며 relative/tilde prefixed glob(`./fixtures/**/*.pem`, `~/fixtures/**/*.pem`)은 아직 unsupported다.

고려 중인 target checklist(아직 미구현/미검증):

- relative exact path는 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual absolute path로 rebase된다.
- relative prefixed glob도 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual glob prefix로 rebase된다.
- `~`/`~/...` exact path와 prefixed glob은 `HOME` 기준으로 expand된 뒤 같은 rebasing 규칙을 따른다.
- supported glob 확장 범위는 optional normalized prefix + 기존 recursive basename/suffix tail(`**/<basename>`, `**/*.<suffix>`)까지다.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user`는 unsupported fail-fast다.
- broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`)은 부분 expansion 없이 fail-fast 한다.
- 현재 소스의 `--readonly-rule` surface와 향후 그 확장도 같은 normalization contract를 재사용해야 한다.

## Selective readonly target checklist

이 절은 목표 계약을 위한 checklist이며, 현재 저장소의 구현/증거 상태로는 아직 미완료다.

검증 시 확인할 점:

- exact path readonly rule에 매칭된 visible mutation은 `EROFS`다.
- pattern-based readonly rule에 매칭된 visible mutation은 `EROFS`다.
- readonly rule에 매칭되지 않은 visible path는 writable mode에서 host 정책에 따라 mutation 가능해야 한다.
- hidden path는 selective readonly rule과 무관하게 계속 `ENOENT`다.
- 동일 연산에서 hidden `ENOENT` 우선순위가 selective readonly `EROFS`보다 앞선다.
- future exact path readonly rule smoke는 hide rule과 같은 normalization contract를 사용했음을 함께 기록한다.

## Unmount

FUSE mount 해제는 `fusermount3`를 사용한다.

```bash
fusermount3 -u /tmp/holefs-root
```

## Requirement-linked verification matrix

현재 문서에는 historical evidence와 current-session evidence를 구분해 적는다.

| 항목 | 현재 상태 | 근거 |
| --- | --- | --- |
| Rust formatting | 이번 세션 미재확인 | 이번 세션은 문서만 수정했고 `cargo fmt --check`는 다시 실행하지 않음 |
| Rust compile check | 현재 차단됨 | 이번 세션에서 `cargo check` 재실행 시 `src/fs.rs` guard API 불일치 등 기존 compile blocker로 실패 |
| Rust lint | 이번 세션 미재확인 | `cargo clippy --all-targets --all-features`는 이번 세션에서 다시 실행하지 않음 |
| Mount-free unit tests | 이번 세션 미재확인 | 과거 문서에는 43 tests evidence가 있으나 이번 세션에서 fresh 재실행하지 않음 |
| FUSE mount smoke | historical evidence only | `/dev/fuse`가 존재하는 환경에서 repo-local fixture mount와 `source-root=/` mount 성공 기록이 있으나 이번 세션 재실행은 없음 |
| hidden path mount smoke | historical evidence only | hidden entry listing 제외, hidden path 직접 접근 `ENOENT` 확인 기록이 있으나 이번 세션 재실행은 없음 |
| whole-view smoke | historical evidence only | `source-root=/` mount에서 `stat /bin/bash`, `ls /usr` 성공 기록이 있으나 이번 세션 재실행은 없음 |
| current global `--readonly` mount smoke | historical evidence only | repo-local/whole-root smoke에서 visible mutation `EROFS` 확인 기록이 있으나 이번 세션 재실행은 없음 |
| rule-input normalization smoke | 미구현/미검증 | current code/live evidence는 이미 절대화된 exact hide rule과 prefix 없는 glob만 사용하며 relative/`~` exact·prefixed-glob normalization contract는 아직 문서상의 target behavior |
| selective readonly rule smoke | 미구현/미검증 | 목표 계약은 path/pattern scoped readonly지만 current verified evidence는 아직 global `--readonly` 중심 |
| user-namespace chroot smoke | historical evidence only | `unshare -UrR <mount> /bin/true` 및 `/bin/bash --noprofile --norc ...` 실행 성공 기록이 있으나 이번 세션 재실행은 없음; device-node 동작은 supervisor/namespace layer 책임 |
| 성능/메모리 측정 | 미실행 | 측정 절차/기준만 남아 있음 |

## 문서 변경 검증

문서만 변경했을 때 최소 검증:

```bash
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts -maxdepth 3 -type f -print | sort
cargo check
```

추가로 `git diff -- README.md AGENTS.md docs .pi` 또는 추적 전 파일의 실제 내용을 확인해 요구사항 누락, 문서 간 모순, 미승인 임시 문구가 없는지 검토한다.
