# holefs 요구사항

## 1. 목적

`holefs`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 존재하지 않는 것처럼 숨기고, visible path에는 별도 readonly policy를 적용할 수 있어야 한다. `pi-bash-sandbox`는 이 요구사항을 소비하는 대표 통합 예시지만, `holefs`의 목적을 그 통합 하나로 한정하지 않는다.

## 2. 핵심 전제

- non-root에서 실행된다.
- `fractal-fuse = 0.4.0` 기반으로 구현한다.
- FUSE3 및 `FUSE_OVER_IO_URING` 기반 사용을 v1 필수 정책으로 삼는다.
- v1은 `FUSE_OVER_IO_URING` 협상 실패 시 fallback mount를 만들지 않고 명시적 오류로 fail-fast 한다.
- mount는 `fusermount3`로 수행한다.
- 대표 사용 시나리오에 chroot가 포함되므로 mount 결과는 전체 파일시스템 뷰를 제공해야 한다.
- `chroot` 실행 권한, same-host-uid 접근 모델, `/proc`·`/sys`·`/dev`·`/run` native semantics는 `holefs` 단독 책임이 아니라 상위 supervisor/namespace layer 책임이다. `pi-bash-sandbox`는 그 책임을 지는 대표 예시다.

## 3. Non-root 실행

`holefs`는 root 권한 없이 실행되어야 한다.

- 일반 사용자 권한으로 FUSE mount 가능해야 한다.
- `fusermount3`를 사용한다.
- root-only mount에 의존하지 않는다.
- privileged bind mount에 의존하지 않는다.
- system-wide mount namespace 조작에 의존하지 않는다.
- `/dev/null` bind overlay 같은 root/mount namespace 기반 masking에 의존하지 않는다.

## 4. 상위 consumer 요구 충족

`holefs`는 상위 sandbox/chroot/orchestrator가 요구하는 filesystem view를 제공해야 한다. `pi-bash-sandbox`는 그 요구를 대표하는 통합 예시다.

- sandboxed bash가 필요한 시스템 경로를 볼 수 있어야 한다.
- 실행에 필요한 binary, shared library, config, runtime path가 보존되어야 한다.
- 권한 정책상 허용된 경로는 정상 접근 가능해야 한다.
- 차단/숨김 경로만 존재하지 않는 것처럼 처리해야 한다.
- 숨김 대상은 `Permission denied`로 존재를 드러내지 않고 가능한 한 `ENOENT`로 처리한다.

## 5. Whole-root consumer용 전체 파일시스템 뷰

대표 통합 시나리오에서 chroot를 사용하므로 `holefs`는 단순 프로젝트 디렉터리 view가 아니라 전체 `/` filesystem view를 제공해야 한다. 이 whole-root view 요구는 `pi-bash-sandbox` 외의 다른 consumer에도 동일하게 적용된다.

```text
real / -> holefs mount root -> chroot root
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
holefs / /tmp/holefs-root ...
chroot /tmp/holefs-root /bin/bash
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
**/*.pem
**/*.key
```

## 8. Selective readonly rule

`holefs`는 hide rule과 별개로 특정 path/pattern에만 쓰기 금지를 적용하는 selective readonly rule을 기능 요구사항으로 지원해야 한다.

현재 목표 계약(path-scoped selective readonly)에서:

- readonly rule에 매치된 visible path는 read/stat/list 허용
- hidden path는 selective readonly 여부와 관계없이 `ENOENT`
- readonly rule에 매치된 path의 모든 쓰기성 operation은 `EROFS`
- readonly rule에 매치되지 않은 visible path는 이 요구사항만으로 자동 readonly가 되지 않음
- CLI syntax는 TBD이며, option name과 path/pattern grammar는 아직 확정되지 않음

고려 가능한 향후 정책 모델:

- `readonly-root` + `allowWrite` carve-out 같은 다른 기본값/override 조합은 향후 설계 대상으로는 열어둘 수 있다.
- 하지만 현재 요구사항, CLI, 검증 evidence를 그런 정책으로 확정해서 읽으면 안 된다.

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

현재 구현 snapshot:

- exact path 입력: exact hide rule은 이미 virtual root 기준 절대경로여야 한다.
- fail-fast/unsupported exact 입력: relative exact path, leading `~`, `~/...`, `~user`는 모두 unsupported다.
- supported pattern 입력: glob grammar는 현재 `**/<basename>` 또는 `**/*.<suffix>` 계열만 허용한다. 대표 예시는 `**/.env`, `**/*.pem`, `**/*.key`, `**/*.lock`이다.
- fail-fast/unsupported pattern 입력: relative/tilde prefixed glob 예시인 `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, 그리고 prefix 내부 wildcard가 섞인 더 넓은 glob은 현재 unsupported다.

고려 중인 target behavior(아직 미구현/미검증):

- exact path와 supported prefixed glob 모두 같은 normalization contract를 공유한다.
- 이미 absolute인 exact path와 absolute prefixed glob은 현재 virtual-root anchored 의미를 유지한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 host path가 `source_root` 내부일 때만 source-root-relative virtual absolute path 또는 virtual glob prefix로 rebase한다.
- `~`/`~/...` 입력은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- supported prefixed glob grammar는 현재의 recursive basename/suffix tail(`**/<basename>`, `**/*.<suffix>`)을 유지하되 optional normalized path prefix를 허용한다. 예: `**/*.pem`, `**/.env`, `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, `/home/<user>/**/*.pem`.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user`는 계속 unsupported이며 fail-fast 한다.
- wildcard가 prefix 내부에 섞이는 더 넓은 glob(`foo/*/bar.pem`, `**/secret?.pem`, brace/env/command expansion)은 이번 범위에서도 unsupported/fail-fast로 남긴다.
- 현재 소스에는 `--readonly-rule`과 readonly matcher surface가 이미 있으며, path-like normalization을 넓힐 때 hide와 readonly rule이 같은 normalization contract를 재사용해야 한다. 다만 현재 구현 검증/live evidence는 global `--readonly` bool 중심이다.

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
holefs / /tmp/holefs-root \
  --hide /home/spi-ca/.ssh \
  --hide /home/spi-ca/.aws \
  --hide /home/spi-ca/.pi/agent/auth.json \
  --hide '/home/spi-ca/.pi/agent/mcp-oauth' \
  --hide '**/.env' \
  --hide '**/*.pem' \
  --hide '**/*.key'
```

Selective readonly rule CLI 예시는 아직 TBD다. option name, path/pattern grammar, hide rule과의 조합 방식은 확정되지 않았다.

현재 구현/검증 기준 예시(global `--readonly` bool, 목표 selective readonly rule과는 별개):

```bash
holefs / /tmp/holefs-root \
  --readonly \
  --hide /home/spi-ca/.ssh
```

이후:

```bash
chroot /tmp/holefs-root /bin/bash
```

## 12. 현재 프로젝트 상태

프로젝트 경로:

```text
/home/spi-ca/Codebase/holefs
```

현재 반영된 구현/검증 상태 요약:

- Rust 모듈 구현: `cli`, `config`, `errors`, `path`, `matcher`, `fs`
- CLI 인자: `<source-root> <mount-root>`, 반복 `--hide`, global `--readonly` bool, 반복 `--readonly-rule`
- hide matcher: virtual root 기준 절대경로 exact rule, directory prefix hiding, `**/<basename>` / `**/*.<suffix>` glob
- hidden/readonly guard: hidden path는 `ENOENT`, global `--readonly` 활성 시 visible mutation은 `EROFS`
- relative exact path, leading `~`, `~/...`, `~user`, relative/tilde prefixed glob 입력 확장/정규화는 아직 구현되지 않았고 current tests/live evidence도 이미 절대화된 입력이나 prefix 없는 glob만 사용
- 기본 FUSE 조회 경로: `lookup`, `getattr`, `open`, `read`, `readdir`, `readdirplus`, `readlink`, `access`, `statfs`
- mount-free unit tests: `cargo test --all-targets --all-features` 기준 총 43 tests 통과, 현재는 global `--readonly` bool과 hidden semantics를 검증하며 selective readonly rule 전용 검증은 아직 없음
- live FUSE smoke: `/dev/fuse`가 있는 환경에서 repo-local fixture mount와 `source-root=/` whole-root mount 성공
- user-namespace chroot smoke: `unshare -UrR <mount> /bin/true` 및 `unshare -UrR <mount> /bin/bash --noprofile --norc ...` 실행 성공
- 아직 미주장 범위: selective readonly rule의 CLI/구현/검증 완료, production-ready 전체 FUSE 완성, privileged supervisor end-to-end 운영 검증, `/dev/null` 등 device-node namespace 구성

확인된 환경:

```text
kernel: 7.1.0-rc6-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
```

위 kernel config artifact는 현재 개발 환경이 `FUSE_OVER_IO_URING` 전제를 충족하도록 빌드되었음을 보여주는 정적 증거다. 이는 별도의 live mount smoke나 세션 협상 성공 주장과 동일하지 않다.
