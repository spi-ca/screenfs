# ScreenFS

`ScreenFS`는 non-root FUSE 기반 whole-filesystem view layer다. 실제 `/` 파일시스템을 pass-through 하되, 지정한 경로는 존재하지 않는 것처럼 숨기고 visible path에는 별도 mutability policy를 적용할 수 있게 설계한다. 대표 통합 시나리오는 `pi-bash-sandbox` 같은 sandbox/chroot consumer지만, 그 용도에만 한정된 컴포넌트로 문서화하지 않는다.

### 시스템 컨텍스트

![ScreenFS system context](docs/diagrams/system-context.png)

### 모듈 구조

![ScreenFS module architecture](docs/diagrams/module-architecture.png)

다이어그램 원본과 공용 렌더링 규칙은 `docs/diagrams/README.md`를 따른다. `docs/diagrams/*.png`는 `docs/diagrams/mermaid-config.json`, `docs/diagrams/puppeteer-config.json`을 함께 사용하고 Mermaid CLI `--scale 2`로 생성하는 것을 저장소 기준으로 삼는다.

## 목적

`ScreenFS`의 기본 역할은 전체 `/`를 입력으로 받아 상위 consumer가 읽을 수 있는 whole-root view를 만드는 것이다. 대표적인 통합 형태는 다음과 같다.

```text
real / -> screenfs mount root -> sandbox/chroot consumer
```

대표 사용 시나리오(`pi-bash-sandbox` + chroot)는 아래와 같다.

```bash
screenfs / /tmp/screenfs-root \
  --hide /home/spi-ca/.ssh \
  --hide /home/spi-ca/.aws \
  --hide /home/spi-ca/.pi/agent/auth.json \
  --hide '/home/spi-ca/.pi/agent/mcp-oauth' \
  --hide '**/.env' \
  --hide '**/*.pem' \
  --hide '**/*.key'

chroot /tmp/screenfs-root /bin/bash
```

이 예시는 대표 통합 시나리오일 뿐이며, `ScreenFS` 자체의 핵심 역할은 특정 supervisor 하나가 아니라 whole-root consumer 전반에 재사용 가능한 filesystem view layer를 제공하는 데 있다.

선택적 readonly rule은 목표 요구사항이며, 현재 소스에는 `--readonly-rule` surface가 존재한다. 다만 현재 검증/live evidence는 아래 global `--readonly` bool 예시에 한정된다.

현재 구현된 global readonly 예시:

```bash
screenfs / /tmp/screenfs-root \
  --readonly \
  --hide /home/spi-ca/.ssh

chroot /tmp/screenfs-root /bin/bash
```

future documented mutability policy family는 두 개로 닫는다.

- `selective-readonly`: 기본 writable, `--readonly-rule` 매치 path/pattern만 `EROFS`
- `readonly-root-allowwrite`: 기본 readonly, `--allow-write` 매치 coordinate만 ScreenFS 차원의 `EROFS` 해제
- 한 mount는 정확히 하나의 family만 선택한다.
- future canonical CLI/config contract에는 legacy bool surface를 포함하지 않는다. canonical 설명과 예시는 explicit family option만 사용한다.
- future config contract는 `mutability.family`, `mutability.readonly_rules`, `mutability.allow_write`를 사용한다. CLI mutability option이 하나라도 있으면 해당 config block 전체를 대체하고, CLI mutability option이 없으면 config `mutability` block이 canonical source of truth다.
- current `--readonly`는 현재 구현/검증 evidence를 설명하기 위한 legacy surface일 뿐이며, future documented contract의 일부로 승격하지 않는다.
- future implementation에서는 legacy `--readonly`를 standalone compatibility mode로만 잠정 유지하고, `--policy-family`/`--readonly-rule`/`--allow-write`/config `mutability` block과 병용되면 fail-fast하도록 정한다.

현재 구현/검증 evidence는 여전히 위 global `--readonly` placeholder 기준이며, future family surface는 아직 미구현/미검증이다.

## 핵심 전제

- 일반 사용자 권한으로 실행한다.
- FUSE mount는 `fusermount3`를 사용한다.
- root-only mount, privileged bind mount, system-wide mount namespace 조작에 의존하지 않는다.
- `/dev/null` bind overlay, 빈 파일 overlay, tmpfs masking처럼 이름을 남기는 masking 방식을 사용하지 않는다.
- `fractal-fuse = 0.4.0` 기반 구현을 전제로 한다.
- FUSE3 및 `FUSE_OVER_IO_URING` 기반 사용을 목표로 한다.
- chroot root로 사용할 수 있도록 단일 프로젝트 디렉터리가 아니라 전체 `/` filesystem view를 제공한다.

## 동작 요구사항

### 전체 view + selective hiding

기본 정책은 전체 파일시스템 pass-through다. 경로가 hide rule에 걸리지 않으면 실제 파일시스템의 binary, shared library, config, runtime path를 그대로 보여줘야 한다. 이를 통해 chroot 내부에서 다음과 같은 표준 경로 구조가 보존되어야 한다.

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

숨김 후보 예시는 다음과 같다.

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

### Hidden path 처리

숨김 대상은 권한 오류가 아니라 가능한 한 존재하지 않는 경로처럼 보여야 한다. 필수 동작:

- `readdir`: 결과에서 제외
- `readdirplus`: 결과에서 제외
- `lookup`: `ENOENT`
- `getattr`: `ENOENT`
- `open`: `ENOENT`
- `access`: `ENOENT`

일반 명령에서 기대되는 결과:

```bash
ls
find
rg
stat hidden-path
cat hidden-path
```

- 목록에는 숨김 대상이 나타나지 않는다.
- 직접 접근하면 `No such file or directory`로 실패한다.
- 단순 `Permission denied`로 존재를 노출하지 않는다.

### Selective readonly rule

hide rule과 별개로 visible path의 mutability policy를 제어하는 selective readonly rule을 지원해야 한다. 현재 소스에는 `--readonly-rule` 입력 surface가 있지만, 이 문서의 future user-facing 계약은 확정돼 있다. 다만 현재 구현/검증 범위는 아직 그 계약 완료를 주장하지 않는다.

현재 목표 계약(path-scoped selective readonly):

- hidden path는 selective readonly 여부와 관계없이 `ENOENT`다.
- readonly rule에 매치된 visible path의 read/stat/list는 허용한다.
- readonly rule에 매치된 path의 모든 쓰기성 operation은 `EROFS`로 실패한다.
- readonly rule에 매치되지 않은 visible path는 이 요구사항만으로 자동 readonly가 되지 않는다.

정책 family 공통 invariant:

- hidden path는 어떤 mutability policy family에서도 계속 `ENOENT`가 우선이다.
- hidden `ENOENT` 우선순위는 readonly/allowWrite 같은 후행 policy 판정보다 앞선다.
- hide rule, current `--readonly-rule` surface, 그리고 future carve-out allowWrite surface는 같은 rule-input normalization contract를 재사용해야 한다.

확정된 future alternate family(`readonly-root-allowwrite`)는 다음 계약을 따른다.

- 이 family는 `pi-bash-sandbox` 같은 integration use case가 기대하는 "기본은 readonly, 일부 path만 writable" 요구를 설명하는 대표 예시일 수 있지만, `ScreenFS`의 범용성을 제한하는 전용 모드는 아니다.
- `allowWrite` rule들은 union semantics로 합쳐지며, 하나라도 매치되면 해당 mutation coordinate는 carve-out 후보가 된다.
- hidden path나 hidden target이 하나라도 관여하면 `allowWrite`보다 hidden `ENOENT`가 우선한다.
- mutation은 관련된 모든 write-requiring affected coordinate가 현재 family 기준으로 writable이어야만 허용된다. 예를 들어 `copy_file_range`는 destination path/parent는 writable이어야 하지만 source는 hidden/read visibility 대상이다.
- future `allow_write`/`--allow-write` surface도 hide/current readonly와 같은 exact path + supported prefixed glob normalization contract를 재사용한다.
- future documented contract는 legacy bool surface를 포함하지 않으며, config schema에도 별도 legacy bool을 두지 않는다.
- carve-out 모델을 실제로 지원하려면 현재 bool+matcher 중심 readonly 판정만으로는 부족하고, 정책 family 선택, shared normalization, operation-aware affected-path evaluation 같은 함수/구조 변경이 추가로 필요하다.
- future mutability family 집합은 현재 spec에서 `selective-readonly`와 `readonly-root-allowwrite` 두 개로 닫고, future deny-like family와의 조합은 현재 spec 범위 밖으로 둔다.
- 하지만 현재 문서의 목표 계약, CLI/config syntax, 검증 evidence를 그런 정책이 이미 구현된 것으로 확정해서 읽으면 안 된다.

쓰기성 operation 범위:

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

### Rule input 정규화: 현재 구현 vs 확정된 target contract

이 저장소 문서는 hide rule과 future selective readonly / readonly-root-allowwrite rule surface의 path-like 입력을 설명할 때 현재 구현과 확정된 target contract를 분리해서 기록한다.

현재 구현(코드 + mount-free unit test 기준):

- exact path와 supported prefixed glob 모두 같은 rule-input normalization contract를 공유한다.
- 이미 absolute인 exact path와 absolute prefixed glob은 virtual-root anchored semantics를 유지한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 해석한 뒤 `source_root` 내부일 때만 virtual absolute path 또는 virtual glob prefix로 rebase한다.
- leading `~`, `~/...` exact path와 prefixed glob은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- supported prefixed glob grammar는 기존 recursive basename/suffix tail(`**/<basename>`, `**/*.<suffix>`)에 optional normalized prefix를 더한 범위까지만 허용한다.
- `HOME` 없음, expanded path outside `source_root`, `~user`, prefix 내부 wildcard, broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`)은 fail-fast다.
- live smoke/examples는 아직 주로 이미 절대화된 exact rule과 prefix 없는 glob 위주로 기록돼 있으므로, relative/`~` normalization의 fresh mount evidence는 별도 미보강 상태다.

확정된 target contract(현재 코드에는 반영됐지만 live smoke는 아직 미보강):

- exact path와 supported prefixed glob 모두 같은 rule-input normalization contract를 공유한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 결과가 `source_root` 내부일 때만 virtual absolute path 또는 virtual glob prefix로 rebase한다.
- leading `~`, `~/...` exact path와 prefixed glob은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- supported prefixed glob grammar는 현재의 recursive basename/suffix tail(`**/<basename>`, `**/*.<suffix>`)을 유지하되 optional normalized path prefix를 허용한다. 예: `**/*.pem`, `**/.env`, `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, `/home/spi-ca/**/*.pem`.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user` 형태는 unsupported이며 fail-fast 한다.
- wildcard가 prefix 내부에 섞이는 더 넓은 glob(`foo/*/bar.pem`, `**/secret?.pem`, brace/env/command expansion)은 이번 범위에서도 unsupported/fail-fast로 남긴다.
- 현재 소스에는 `--readonly-rule`과 readonly matcher surface가 이미 있으며, path-like normalization을 넓힐 때 hide와 readonly rule이 같은 normalization contract를 재사용해야 한다. 다만 현재 구현 검증/live evidence는 여전히 global `--readonly` bool 중심이다.

### 성능 및 메모리 방향

전체 filesystem view는 metadata-heavy workload가 많을 수 있으므로 다음 방향을 구현 기준으로 삼는다.

- `readdirplus` 구현으로 directory listing 중 metadata round-trip을 줄인다.
- inode/path cache를 둔다.
- file handle을 재사용한다.
- hidden matcher를 빠르게 처리한다.
- glob rule 수가 늘어도 성능 저하가 급격하지 않게 한다.
- visible file data path는 가능한 한 underlying filesystem으로 pass-through 한다.
- 장시간 실행해도 memory footprint가 과도하게 증가하지 않도록 cache eviction/상한을 설계한다.

## 현재 구현 상태

현재 저장소에는 Rust 기반 초기 구현이 포함되어 있다.

- CLI 인자 파싱이 구현되어 있다: `<source-root> <mount-root>`, 반복 `--hide`, global `--readonly` bool, 반복 `--readonly-rule`. selective readonly rule surface는 소스와 mount-free unit test에 일부 반영돼 있지만, 현재 문서의 live verification evidence는 아직 global `--readonly` 중심으로만 정리돼 있다.
- future carve-out support를 위해 필요한 문서상 change hotspot은 `src/config.rs`의 policy-family data model, `src/fs.rs`의 `guard_mutation_path`/`guard_multi_path_mutation` 같은 affected-path evaluator, `src/errors.rs`의 write-intent/errno precedence 분류, `src/cli.rs`의 policy-family 선택 surface다. 현재 구현에는 아직 반영되지 않았다.
- 문서상 후속 구현 우선순위는 다음 순서를 기준으로 고정한다: (1) `src/config.rs` family data model, (2) shared rule-input normalization, (3) `src/fs.rs` affected-coordinate evaluator, (4) `src/errors.rs` precedence helper, (5) `src/cli.rs` family surface. 이 중 (2)는 hide + current `--readonly-rule` 공용 contract 범위로 현재 코드에 반영됐다.
- lexical virtual path normalization과 symlink target 해석이 구현되어 있다.
- hide matcher가 구현되어 있다: virtual root 기준 absolute exact rule, relative/`~` exact rule rebasing, directory prefix hiding, prefix 없는 limited glob, absolute/relative/`~` prefixed limited glob
- hidden/readonly guard 분류가 구현되어 있다: hidden path는 `ENOENT`, global `--readonly` 활성 시 visible mutation은 `EROFS`
- current `--readonly-rule` surface도 hide와 같은 normalization contract를 재사용한다. 다만 live verification evidence는 여전히 global `--readonly` 중심이다.
- 기본 FUSE 조회 경로가 구현되어 있다: `lookup`, `getattr`, `open`, `read`, `readdir`, `readdirplus`, `readlink`, `access`, `statfs`
- 최근 mount-free blocker fix가 반영되어 있다: host permission 기반 `access` pass-through, hidden target symlink에 대한 `lookup`/`open`/`readlink` guard, global `--readonly` `create`의 `EROFS` 반환
- 현재 mount-free unit test는 `cargo test --all-targets --all-features` 기준 총 54개가 통과하며 parser/path/matcher/guard/FUSE baseline 동작, relative/`~` exact·prefixed-glob normalization, hide/current `--readonly-rule` shared semantics, access/symlink/create regression, global `--readonly`/hidden multi-path mutation·xattr guard, symlink target hiding, `copy_file_range` visible/hidden 경로를 검증한다. 여기에 `--readonly-rule` 매치 경로 범위와 symlink/hidden precedence를 확인하는 selective readonly 전용 테스트도 포함된다. 다만 이는 future selective-readonly family CLI/config contract나 live smoke 완료를 의미하지 않는다.

아직 완료로 주장하지 않는 범위:

- selective readonly rule 요구사항의 CLI/구현/검증 완료
- production-ready 전체 FUSE 동작 완성
- chroot 통합 완료
- privileged end-to-end 운영 검증 완료

최신 documented live FUSE smoke evidence(2026-06-10 기준): `/dev/fuse`가 존재하는 환경에서 repo-local fixture mount와 `source-root=/` whole-root mount가 성공한 기록이 있다. repo-local smoke는 hidden entry가 부모 listing에서 제외되고 hidden path 직접 접근이 `ENOENT`, global `--readonly` 기준 mutation이 `EROFS`로 처리됨을 확인했고, whole-root smoke는 `stat /bin/bash`, `ls /usr`, hidden `/home/spi-ca/.ssh`의 `ENOENT`, global `--readonly` 기준 `touch`의 `EROFS`를 확인했다. 또한 `unshare -UrR <mount> /bin/true`와 `unshare -UrR <mount> /bin/bash --noprofile --norc ...`가 성공해 user-namespace 기반 chroot 실행 smoke를 확인한 기록이 있다. 관련 transcript는 현재 `readonly-rule-policy=compiled`를 포함한 startup log를 담고 있으며, 현재 세션의 fresh 검증 상태와 세부 명령 출력은 `docs/operations.md`와 `docs/artifacts/fuse-smoke-transcript.md`를 따른다. FUSE mount는 `nodev`이므로 `/dev/null` 같은 device-node 동작은 상위 supervisor/namespace layer에서 별도 제공해야 한다 (`docs/operations.md` 참고).

추가로 확인된 환경:

- `fractal-fuse = 0.4.0`
- kernel: `7.1.0-rc6-1-spica-git`
- kernel config artifact: `/home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64`
- documented kernel config flags: `CONFIG_FUSE_IO_URING=y`, `CONFIG_IO_URING=y`
- `fusermount3`: `/usr/bin/fusermount3`
- `fusermount3 version`: `3.18.2`

위 kernel config artifact는 `FUSE_OVER_IO_URING` 요구사항의 환경 전제 증거이며, 그 자체로 새로운 live mount smoke 또는 협상 성공을 의미하지는 않는다.

## 추가 문서

- [아키텍처 개요](docs/architecture.md)
- [요구사항](docs/requirements.md)
- [설계 노트](docs/design.md)
- [운영 및 검증](docs/operations.md)
- [Pi role agents](docs/pi-agents.md)
