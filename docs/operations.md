# 운영 및 검증

이 문서는 현재 계약과 verified evidence를 구분해 적는다. 정책 surface는 `visibility.*` / `mutability.*` 두 축이며, bare slashless glob의 cwd-anchored `./<pattern>` direct-child semantics도 현재 source/test/live-smoke evidence가 갖춰진 contract다. 일부 transcript filename이나 본문 command/log에는 historical label이 남아 있을 수 있지만, CLI/config reference는 two-axis surface만 사용한다.

artifact 분류 quick map:

- current two-axis repo-local live smoke baseline: `docs/artifacts/current-two-axis-smoke-transcript.md`
- current two-axis whole-root/chroot live smoke baseline: `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`
- current default-writable live smoke baseline: `docs/artifacts/current-default-writable-smoke-transcript.md`
- current dynamic visible-glob whole-root startup smoke baseline: `docs/artifacts/current-dynamic-whole-root-smoke-transcript.md`
- current canonical 4-family glob repo-local live smoke baseline: `docs/artifacts/current-four-family-glob-smoke-transcript.md`
- current bare slashless cwd-anchor repo-local live smoke baseline: `docs/artifacts/current-bare-basename-glob-smoke-transcript.md`
- archival-only historical transcripts: `docs/artifacts/whole-root-family-smoke-transcript.md`, `docs/artifacts/whole-mount-readonly-smoke-transcript.md`, `docs/artifacts/future-mutability-smoke-transcript.md`, `docs/artifacts/fuse-smoke-transcript.md`

## 환경 전제

현재 확인된 환경:

```text
kernel: 7.1.0-rc7-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
/dev/fuse: present
```

위 kernel config artifact는 현재 커널 빌드 설정에 `FUSE_OVER_IO_URING` 관련 옵션이 활성화되어 있음을 보여주는 정적 근거다. 이는 mount 성공이나 실제 session 협상 성공을 새로 증명하는 smoke artifact는 아니다.

프로젝트 상태:

```text
/home/spi-ca/Codebase/screenfs
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

최신 recorded build/test evidence:

- recorded evidence는 visibility/mutability policy axis 기준으로 정리한다.
- `cargo fmt --check`, `cargo check`, `cargo test --all-targets --all-features`, `cargo clippy --all-targets --all-features` pass 기록이 남아 있다.
- latest recorded full-suite `cargo test --all-targets --all-features` 결과는 100 tests 통과이며, visibility/mutability axis regressions, bare slashless cwd-anchor/direct-child regressions, canonical 4-family matcher/runtime-config coverage를 포함한다.
- 현재 mount-free test coverage에는 다음 축별 확인이 포함된다.
  - `visibility.hidden`: hidden entry filtering, direct read/stat/access/open `ENOENT`, hidden symlink target `ENOENT`
  - `visibility.visible`: default-hidden carve-out, bridge-visible ancestor, nested listing
  - `mutability.default=writable`: rule-match mutation `EROFS`, non-match visible mutation 허용
  - `mutability.default=readonly`: writable carve-out success, carve-out 밖 visible mutation `EROFS`
  - `mutability.readonly` re-block: 더 구체적인 readonly override precedence
  - hidden-before-mutability: hidden path/target이 mutation 좌표에 섞이면 mutability 판정보다 먼저 `ENOENT`
  - affected-coordinate-wide writable requirement, `copy_file_range` source visibility / destination writability 분리
  - bare slashless cwd-anchor/direct-child delta: `cwd` rebasing, cwd-outside-`source_root` fail-fast, `*.pem`=`./*.pem` equivalence, `*.pem` vs `**/*.pem` containment contrast
  - canonical 4-family family-by-family source coverage: `**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`의 matching scope, representative non-match range, specificity/containment, `dynamic_bridge_scan_roots`
  - unsupported glob fail-fast, same-specificity conflict fail-fast, config load/axis override
- 문서 정합성 확인으로 변경된 문서 구간은 상호 일관성을 위해 다시 읽어 확인한다.

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

`docs/artifacts/current-two-axis-smoke-transcript.md`는 visibility/bridge-visible/mutability 동작에 대한 repo-local live baseline이다. 그 artifact 안의 `--hidden '*.pem'` invalid-glob line만 bare slashless 지원 이전 historical capture로 남아 있으며, bare slashless semantics는 refreshed current baseline인 `docs/artifacts/current-bare-basename-glob-smoke-transcript.md`를 따른다. `docs/artifacts/current-four-family-glob-smoke-transcript.md`는 `visibility.default=hidden` repo-local mount 네 세션(`**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`)을 current live smoke baseline으로 묶어 listing/access 결과와 `startup_ms`를 함께 남긴다. `startup_ms`는 startup smoke evidence일 뿐 formal benchmark가 아니다. `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`는 bare slashless shorthand에 의존하지 않는 `source_root=/` whole-root/chroot baseline으로 계속 current다. 과거 transcript는 historical 참고 자료로만 사용한다.

현재 bare slashless glob 상태:
  - `current-bare-basename-glob-smoke-transcript.md`는 cwd-anchored `./<pattern>` direct-child semantics의 current repo-local live baseline이다.
  - artifact는 `visibility.hidden`/`mutability.readonly`, `visibility.visible`/`mutability.writable`, cwd-outside-`source_root` fail-fast, unsupported bare wildcard fail-fast, `*.pem` vs `./*.pem` same-normalized-specificity conflict를 캡처한다.
  - `*.pem` vs `**/*.pem`는 current contract에서도 same-specificity conflict가 아니라 containment/nested-override 관계다. whole-root consumer에서 트리 전체 secret coverage가 필요하면 `**/*.pem`, `**/*.key`, `**/.env.*`, `/a/**/*.txt` 같은 explicit recursive/anchored form을 사용한다.

현재 canonical 4-family glob 상태:
  - `current-four-family-glob-smoke-transcript.md`는 `visibility.default=hidden` repo-local mount에서 `**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`를 family-by-family로 캡처한다.
  - artifact는 각 family의 listing 결과, 대표 read/stat success, representative non-match `ENOENT`, `startup_ms`를 같은 문서에 남긴다.
  - `startup_ms`는 family 간 대략적인 startup footprint smoke를 보여 주지만, 정식 성능 benchmark claim으로 읽지 않는다.
- startup log 문자열은 캡처 시각 기준으로 읽는다. transcript 본문에 남은 field명은 capture-time implementation detail이지 current contract 명명법이 아니다.

중요: 현재 운영/검증 문서의 해석 축은 다음뿐이다.

- `visibility.default`
- `visibility.hidden`
- `visibility.visible`
- bridge-visible ancestor
- `mutability.default`
- `mutability.writable`
- `mutability.readonly`
- hidden-before-mutability

## Smoke evidence recording rules

- smoke를 `완료` 또는 `성공`으로 표기하려면 실제 mount가 살아 있는 상태에서 후속 검증 명령까지 실행돼야 한다.
- kernel config artifact(`.../linux-spica-git/config.saved.x86_64`에서 `CONFIG_FUSE_IO_URING=y`, `CONFIG_IO_URING=y`)는 환경 전제 증거로만 기록하고, 단독으로 live smoke 성공으로 승격하지 않는다.
- 최소 기록 항목:
  - mount 명령 또는 동등한 실행 surface
  - source/mount 경로
  - `visibility.hidden` / `visibility.visible` 입력
  - `mutability.default`, `mutability.writable`, `mutability.readonly` 입력
  - `fusermount3 --version`
  - 후속 `ls`/`stat`/`cat`/mutation 명령
  - stderr/stdout 발췌
  - unmount 결과
- startup log 문자열이 바뀌면 transcript의 기존 log line은 historical artifact로 남기고, current startup-string evidence는 해당 세션의 fresh stderr/stdout 발췌나 현재 소스 기준으로 별도 구분해 적는다.
- repo-local fixture mount evidence는 normalization, `visibility.hidden`, `mutability.default` regression 확인에 유용하지만, whole-view와 chroot 관련 결론은 `source-root=/` 또는 동등한 전체 view 환경의 별도 증거로 보강해야 한다.
- `mutability.default=writable`를 주장하려면 rule-match path와 non-match path를 모두 포함한 별도 evidence가 필요하다.
- `mutability.default=readonly`를 주장하려면 writable carve-out 성공과 carve-out 밖 `EROFS`를 같은 세션에 남겨야 한다. whole-mount readonly baseline은 empty `mutability.writable`의 별도 케이스로 분리한다.
- `mutability.readonly` re-block를 주장하려면 더 넓은 `mutability.writable` 아래 더 구체적인 readonly override와 그 반대 polarity conflict/fail-fast를 함께 기록한다.
- `visibility.visible` carve-out을 주장하려면 hidden ancestor 아래 visible descendant에 도달하기 위한 bridge-visible listing/traversal 결과를 함께 기록한다.
- rule-input normalization 변화를 주장하려면 already-absolute, relative, `~` success case를 분리해 기록하고, bare slashless shorthand의 launch cwd / normalized anchor / cwd-outside-`source_root` fail-fast / `*.pem`=`./*.pem` equivalence / `*.pem` vs `**/*.pem` containment contrast, descendant-subtree success/fail-fast, broader unsupported wildcard fail-fast stderr도 별도로 남긴다.
- 문서상 current contract surface와 current verified evidence를 구분한다. transcript filename이 historical label을 포함하더라도 설명은 two-axis semantics로 적는다.
- mount 전에 실패했으면 상태는 `차단됨` 또는 `실패`로 적고, hidden/whole-view/mutability smoke를 `완료`로 승격하지 않는다.

## 운영 시나리오 번들

아래 항목은 운영 예시 번들이다. 최신 recorded verification evidence 자체로 읽지는 않는다. bare slashless glob은 현재 구현 계약에서 cwd-sensitive `./<pattern>` shorthand이므로 whole-root 예시에는 explicit recursive/anchored form을 우선 사용한다. `pi-bash-sandbox` + chroot는 대표 통합 시나리오지만, 동일한 whole-root mount는 다른 sandbox/chroot consumer에도 재사용될 수 있다.

- baseline hidden-only view
  - `source-root=/`
  - `visibility.hidden`: `/home/spi-ca/.ssh`, `/home/spi-ca/.aws`, `/home/spi-ca/.pi/agent/auth.json`, `/home/spi-ca/.pi/agent/mcp-oauth`, `**/.env`, `**/.env.*`, `**/*.pem`, `**/*.key`
  - `mutability.default=writable`
- whole-root writable-carve-out view
  - `source-root=/`
  - `visibility.hidden`: `/home/spi-ca/.ssh`
  - `mutability.default=readonly`
  - `mutability.writable`: `/tmp`
- whole-root readonly baseline view
  - `source-root=/`
  - `visibility.hidden`: `/home/spi-ca/.ssh`
  - `mutability.default=readonly`
  - `mutability.writable`: empty
- default-hidden carve-out view
  - `source-root=/`
  - `visibility.default=hidden`
  - `visibility.visible`: `/workspace`
  - `mutability.default=writable`
- chroot consumer baseline
  - live proof는 `unshare -UrR <mount> /bin/bash --noprofile --norc ...` transcript로 확인한다.
  - `ScreenFS`는 mount view를 제공할 뿐이며, `chroot` 권한 모델과 `/dev/null` 같은 device-node semantics는 상위 supervisor/namespace layer 책임이다.

`pi-bash-sandbox` style policy mapping check:

| Supervisor key | ScreenFS two-axis target |
| --- | --- |
| `denyRead` | `visibility.hidden` |
| `allowRead` | `visibility.visible` carve-out, or no rule when `visibility.default=visible` already exposes the path |
| `allowWrite` | `mutability.writable` |
| `denyWrite` | `mutability.readonly` |

Bare slashless pattern(`*.pem`, `*.key`, `.env.*`, `id_*`)을 supervisor가 그대로 전달하면 ScreenFS는 macOS sandbox-runtime matcher 전체를 복제하는 것이 아니라, 그런 supervisor가 쓰는 bare basename-oriented input shape를 보존하기 위해 launch cwd를 `source_root` 기준으로 rebase한 `./<pattern>` direct-child shorthand로 컴파일한다. macOS-style cwd-sensitive policy를 의도한 경우에만 bare form을 쓰고, whole-root secret coverage가 목적이면 `**/*.pem`, `**/*.key`, `**/.env.*`, `/workspace/**/*.lock` 같은 explicit recursive/anchored form을 생성한다.

The integration must emit ScreenFS YAML/CLI in the current two-axis model, not legacy ScreenFS CLI options.

## `visibility.hidden` smoke checks

최신 repo-local/whole-root smoke evidence는 아래 조건을 충족했다. 필요 시 동일 절차로 재검증한다.

```bash
ls /tmp/screenfs-root/home/spi-ca
find /tmp/screenfs-root/home/spi-ca -maxdepth 2 -name .ssh
stat /tmp/screenfs-root/home/spi-ca/.ssh
cat /tmp/screenfs-root/home/spi-ca/.ssh/id_rsa
```

기대 결과:

- `.ssh` 등 hidden entry가 listing에 나타나지 않는다.
- 직접 접근은 `No such file or directory`로 실패한다.
- `Permission denied`로 존재가 드러나지 않는다.

## bridge-visible traversal/listing checklist

`visibility.visible` carve-out을 검증할 때는 다음을 함께 남긴다.

- hidden ancestor 아래 visible descendant가 있으면, 그 descendant에 도달하는 데 필요한 bridge entry는 listing/traversal에서 살아 있어야 한다.
- bridge-visible parent는 존재를 감추면 안 되지만, carve-out 밖 hidden sibling은 계속 listing에서 제외돼야 한다.
- `stat`/`open`/`read`는 visible carve-out descendant에서 성공하고, carve-out 밖 sibling 직접 접근은 계속 `ENOENT`여야 한다.
- `readdir`와 `readdirplus` 모두 같은 bridge-visible 결과를 보여야 한다.
- artifact에는 parent listing, descendant `stat`, descendant read, hidden sibling `ENOENT`를 한 세션에 함께 남긴다.

## Whole-view baseline smoke checks

최신 `source-root=/` smoke evidence는 아래 조건을 충족했다. 필요 시 동일 절차로 재검증한다.

```bash
ls /tmp/screenfs-root
ls /tmp/screenfs-root/bin
ls /tmp/screenfs-root/usr
ls /tmp/screenfs-root/lib
ls /tmp/screenfs-root/lib64
ls /tmp/screenfs-root/etc
```

기대 결과:

- `/`, `/bin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/home`, `/tmp`, `/var` 같은 시스템 경로가 보인다.
- hidden rule에 걸리지 않은 binary, shared library, config, runtime path는 접근 가능하다.

## `mutability.default=readonly` baseline smoke checks

최신 repo-local/whole-root smoke evidence는 아래 조건을 충족했다. 이 절은 empty `mutability.writable`가 표현하는 whole-mount readonly baseline을 재검증하기 위한 checklist다.

```bash
stat /tmp/screenfs-root/bin/bash
ls /tmp/screenfs-root/etc
touch /tmp/screenfs-root/tmp/screenfs-write-check
mkdir /tmp/screenfs-root/tmp/screenfs-mkdir-check
```

기대 결과:

- `stat`, `ls`는 성공한다.
- `touch`, `mkdir` 등 mutation은 read-only filesystem 오류로 실패한다.
- hidden path는 default readonly 여부와 관계없이 `ENOENT`다.

## Rule-input normalization status and checklist

현재 계약 vs 현재 근거 상태:

- 현재 contract는 `visibility.hidden`/`visibility.visible`/`mutability.writable`/`mutability.readonly` shared normalization contract에 recursive prefixed glob, cwd-anchored bare slashless glob shorthand, direct-child basename-prefix/suffix subset, limited recursive literal descendant-subtree glob(`<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`)을 포함한다.
- 현재 recorded source/live evidence는 descendant-subtree matcher-level absolute/relative/`~` matching, runtime-config shared-surface matching, nested specificity regression, hidden `ENOENT` precedence, bridge-visible traversal/listing, bare slashless cwd-anchor/direct-child delta를 뒷받침한다.
- absolute exact path와 absolute prefixed glob은 virtual-root anchored semantics를 유지한다.
- bare slashless glob은 `/` component가 없는 glob pattern이며 launch process cwd를 먼저 host path로 정규화한 뒤 `source_root` relative normalized prefix로 rebase한 `./<pattern>` shorthand다. 따라서 `*.pem`은 같은 cwd anchor에서 `./*.pem`과 동등하고, 그 anchor 바로 아래 child basename과 matched child descendants에만 적용된다. 예를 들어 cwd rebase가 `/workspace`이면 `*.pem`은 `/workspace/root.pem`에는 매치하지만 `/workspace/nested/cert.pem`에는 매치하지 않는다.
- supported bare slashless glob `<pattern>`은 같은 normalized cwd anchor에서 `./<pattern>`과 동등하며 `**/<pattern>`과는 다르다. whole-tree recursive intent는 `**/*.pem`, `**/.env.*`, `/workspace/**/*.txt`처럼 explicit recursive form으로 표현한다.
- basename-prefix glob(`**/.env.*`, bare `.env.*`, `/home/<user>/.env.*`, literal example `~/.env.*`)은 recursive tail form인지 direct-child form인지 anchor 표현에 따라 갈린다. bare `.env.*`는 cwd-anchored direct-child shorthand이고, `**/.env.*`는 recursive basename-prefix tail이다.
- direct-child basename-prefix/suffix glob(`./fixtures/*.pem`, bare `*.pem`, `~/*.pem`, `/home/<user>/*.pem`, bare `id_*`, `~/.env.*`, `/home/<user>/id_*`)은 normalized prefix 바로 아래 child와 그 descendants에만 매치된다. 예를 들어 `/a/*.txt`는 `/a/file.txt`와 `/a/file.txt/child`에는 매치하지만 `/a/b/file.txt`에는 매치하지 않는다. 반대로 recursive prefixed glob `/a/**/*.txt`는 `/a/file.txt`, `/a/b/file.txt`, `/a/b/c/file.txt`처럼 `/a` 아래 임의 깊이의 basename에 매치한다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 rebase된다.
- `~`/`~/...` exact path와 prefixed glob은 `HOME` 기준으로 expand된 뒤 같은 rebasing 규칙을 따른다.
- focused currently recorded source evidence는 `src/matcher.rs`의 descendant-subtree tests와 bare slashless normalization/specificity tests, `src/config.rs`의 shared-surface runtime-config tests, `src/fs.rs`의 hidden-precedence 및 subtree-scope tests다.
- broader unsupported wildcard forms, prefix 내부 wildcard, one-sided basename-prefix/suffix subset 밖의 bare wildcard form(`*`, `a*b`, `*secret*`), `HOME` 없음, outside-`source_root`, `~user`는 계속 fail-fast다. descendant-subtree broader forms(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`, trailing `/**` 없는 `**/.git/hooks`)도 부분 해석 없이 fail-fast 대상으로 유지한다.
- existing repo-local transcript `docs/artifacts/future-mutability-smoke-transcript.md`는 relative exact path, relative prefixed-glob, `~/...` prefixed-glob archival evidence를 포함한다. Current live coverage는 `docs/artifacts/current-two-axis-smoke-transcript.md`, `docs/artifacts/current-bare-basename-glob-smoke-transcript.md`, `docs/artifacts/current-four-family-glob-smoke-transcript.md`, `docs/artifacts/current-default-writable-smoke-transcript.md`, `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`, `docs/artifacts/current-dynamic-whole-root-smoke-transcript.md`에 분리해 기록한다.

### Canonical 4-family glob table and current coverage note

아래 표는 current contract/design summary다. canonical 4-family 비교에 대한 dedicated source/live evidence 매핑은 바로 아래 note를 따른다.

| Syntax / family | Anchor normalization | Matching scope | Matched entry + descendants | Representative non-match | Specificity / conflict / containment | `visibility.visible` bridge-visible startup/performance impact |
| --- | --- | --- | --- | --- | --- | --- |
| `**/*.pem`<br>prefixless recursive suffix tail | prefix가 없고 cwd rebasing도 없다. prefixless whole-tree recursive family로 해석되며 별도 anchor가 없으면 `source_root`가 startup scan root가 될 수 있다. | 전체 virtual tree 아래 임의 깊이의 `*.pem` basename | 각 matched entry 자체에 적용되고, matched entry가 directory면 descendants도 함께 포함된다. | `/a/cert.txt` | 같은 whole-tree recursive `.pem` tail opposite-polarity rule과는 same-specificity conflict가 가능하다. `./fixtures/*.pem` 같은 anchored direct-child `.pem` family를 containment로 포함하는 더 넓은 rule이다. | 가장 broad하다. `visibility.visible` bridge index는 prefixless form 때문에 existing visible descendant를 찾기 위해 `source_root`부터 탐색할 수 있어 startup 비용이 가장 커질 수 있다. |
| `./fixtures/*.pem`<br>cwd-rebased anchored direct-child suffix | `./fixtures` prefix를 launch cwd host path에서 해석한 뒤 `source_root` 내부 virtual prefix로 rebase한다. | normalized `./fixtures` anchor 바로 아래 immediate child basename만 | 각 matched immediate child 자체와 그 descendants | `<normalized ./fixtures>/nested/cert.pem` | 같은 normalized anchor로 rebase되는 equivalent form과 same-specificity conflict/duplicate가 된다. `**/*.pem`보다 더 specific하며 target set은 그 안에 포함된다. | rule semantics는 direct-child anchored다. 다만 current bridge-index walk는 startup에서 anchor 아래를 재귀 탐색할 수 있으므로 whole-tree보다는 좁지만 anchor subtree 크기에 비례한다. |
| `/a/*.txt`<br>absolute anchored direct-child suffix | virtual-root anchored absolute prefix `/a`; cwd rebasing 없음 | `/a` 바로 아래 immediate child basename만 | 각 matched immediate child 자체와 그 descendants | `/a/b/file.txt` | 같은 `/a` direct-child `.txt` normalized anchor와 same-specificity conflict/duplicate가 된다. `/a/**/*.txt`보다 more-specific하고 target set은 그 안에 포함된다. | anchored direct-child라 scan root는 `/a`로 제한된다. 다만 current bridge-index walk는 startup에서 `/a` 아래를 재귀 탐색할 수 있어 direct-child matcher 자체보다 넓은 discovery cost가 남는다. |
| `/a/**/*.txt`<br>absolute anchored recursive suffix | virtual-root anchored absolute prefix `/a`; cwd rebasing 없음 | `/a` 아래 임의 깊이의 `*.txt` basename | 각 matched entry 자체와 그 descendants | `/b/file.txt` | `/a/*.txt` target set을 포함하는 broader recursive rule이다. 같은 `/a` recursive `.txt` form opposite-polarity rule과 same-specificity conflict가 가능하다. | scan root는 `/a`지만 recursive family라 `/a/*.txt`보다 훨씬 넓은 startup discovery가 가능하다. 그래도 prefixless `**/*.pem`처럼 `source_root` 전체로 퍼지지는 않는다. |

현재 recorded source/live evidence note:

- source tests는 canonical 4 families(`**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`)의 matching scope, representative non-match range, specificity/containment, `dynamic_bridge_scan_roots`를 family-by-family로 검증한다.
- `docs/artifacts/current-four-family-glob-smoke-transcript.md`는 `visibility.default=hidden` repo-local mount 네 세션을 current live smoke baseline으로 남기며, 각 family의 listing/access 결과와 `startup_ms`를 함께 기록한다.
- 이 artifact의 `startup_ms`는 mount startup smoke evidence일 뿐 formal benchmark가 아니다. whole-root coverage/latency 판단은 계속 `current-whole-root-chroot-smoke-transcript.md`, `current-dynamic-whole-root-smoke-transcript.md`, 또는 별도 benchmark task로 보강한다.
- `**/*.pem`은 prefixless whole-tree recursive family이고, `./fixtures/*.pem`와 `/a/*.txt`는 anchored direct-child family이며, `/a/**/*.txt`는 anchored recursive family라는 점을 source/live evidence가 함께 재확인한다.

문서화된 current contract checklist:

- relative exact path는 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual absolute path로 rebase된다.
- relative prefixed glob도 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual glob prefix로 rebase된다.
- `~`/`~/...` exact path와 prefixed glob은 `HOME` 기준으로 expand된 뒤 같은 rebasing 규칙을 따른다.
- implementation evidence가 검증해야 하는 glob 범위는 recursive basename/suffix/basename-prefix tail(`**/<basename>`, `**/*.<suffix>`, `**/<basename-prefix>*`), bare slashless glob shorthand(`*.pem`, `*.key`, `.env.*`, `id_*`, each equivalent to `./<pattern>` after cwd rebasing), normalized-prefix direct-child basename-prefix/suffix form(`<normalized-prefix>/<basename-prefix>*`, `<normalized-prefix>/*.<suffix>`), limited recursive literal descendant-subtree glob(`<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`)까지다.
- descendant-subtree glob은 첫 literal component 앞에 recursive gap이 있고, literal tail subtree root 자체와 그 descendants 전체에 매치된다.
- nested override와 specificity 검증은 normalized target set 기준으로 수행한다. `*.pem`는 같은 normalized cwd anchor에서 `./*.pem`과 같은 normalized anchor/specificity로 취급한다. 반면 `*.pem` vs `**/*.pem`는 containment가 있는 서로 다른 specificity rule이므로 same-specificity equivalent rule이 아니라 nested override territory다. descendant-subtree glob끼리는 literal tail component 수가 더 많을수록, 그다음으로 normalized prefix가 더 길수록 더 구체적이다.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user`는 unsupported fail-fast다.
- broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`, one-sided subset 밖의 bare wildcard `a*b`, brace/env/command expansion`)과 descendant-subtree literal tail 내부 wildcard(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`) 또는 trailing `/**` 없는 form(`**/.git/hooks`)은 부분 expansion 없이 fail-fast 한다.
- `visibility.hidden`, `visibility.visible`, `mutability.writable`, `mutability.readonly`는 같은 normalization contract를 재사용해야 한다.

## Mutability axis checklist

### `mutability.default=writable`

이 절은 path-scoped readonly 효과를 축 모델로 읽는 checklist다.

검증 시 확인할 점:

- exact path `mutability.readonly` match는 visible mutation을 `EROFS`로 만든다.
- pattern-based `mutability.readonly` match도 visible mutation을 `EROFS`로 만든다.
- `mutability.readonly`에 매치되지 않은 visible path는 host 정책이 허용하면 mutation 가능해야 한다.
- 더 구체적인 `mutability.writable` carve-out이 있으면 해당 descendant subtree에서는 `EROFS`가 해제된다.
- hidden path 또는 fully visible이 아닌 symlink target이 관여하면 결과는 계속 `ENOENT`다.
- 동일 연산에서 hidden-before-mutability 우선순위가 유지된다.
- fresh artifact에는 rule-match `EROFS`, carve-out success, non-match visible mutation success, hidden `ENOENT`, same-specificity conflict fail-fast, unsupported glob fail-fast를 함께 기록한다.

### `mutability.default=readonly`

이 절은 current whole-root baseline과 가장 직접적으로 연결된다.

검증 시 확인할 점:

- 기본 visible mutation은 `EROFS`다.
- `mutability.writable` carve-out에 매치된 visible path mutation은 host 정책이 허용하면 성공한다.
- 더 구체적인 `mutability.readonly` re-block이 있으면 carve-out 내부에서도 다시 `EROFS`다.
- hidden path나 hidden target은 writable carve-out 또는 readonly re-block과 겹쳐도 여전히 `ENOENT`다.
- hidden 우선순위가 carve-out/re-block 판정보다 앞선다.
- write-intent `open`, `setattr`, xattr mutation, `fallocate`도 같은 evaluator를 공유한다.
- `create`, `mkdir`, `mknod`, `unlink`, `rmdir`는 mutated path와 parent 둘 다 writable이어야 한다.
- `rename`, `link`, `symlink`는 source/target/parent 전체를 기준으로 판정하며, mutation에 관여하는 모든 write-requiring coordinate가 writable이어야 한다.
- destination mutation이 있는 `copy_file_range`는 source visibility와 destination writability를 분리한다. source는 hidden/read visibility 대상이고, destination path와 destination parent는 writable이어야 한다.
- fresh artifact에는 writable carve-out success, carve-out 밖 `EROFS`, nested readonly re-block `EROFS`, hidden `ENOENT`, same-specificity conflict fail-fast, unsupported glob fail-fast를 함께 기록한다.

## Unmount

FUSE mount 해제는 `fusermount3`를 사용한다.

```bash
fusermount3 -u /tmp/screenfs-root
```

## Requirement-linked verification matrix

현재 문서는 archival evidence와 latest recorded evidence를 구분해 적는다.

| 항목 | 현재 상태 | 근거 |
| --- | --- | --- |
| Rust formatting | 통과 | 최신 recorded evidence 기준 `cargo fmt --check` pass 기록 |
| Rust compile check | 통과 | 최신 recorded evidence 기준 `cargo check` pass 기록 |
| Rust lint | 통과 | 최신 recorded evidence 기준 `cargo clippy --all-targets --all-features` pass 기록 |
| Mount-free unit tests | 통과 | latest recorded `cargo test --all-targets --all-features`는 100 tests 통과이며, visibility/mutability axis regressions, bare slashless cwd-anchor/direct-child regressions, canonical 4-family matcher/runtime-config coverage를 포함한다. |
| `visibility.hidden` | source evidence 통과 / repo-local live smoke 통과 | mount-free tests는 direct access `ENOENT`와 listing exclusion을 검증; current smoke는 hidden `.git/config` `stat`/mutation `ENOENT`를 캡처 |
| `visibility.visible` carve-out | source evidence 통과 / repo-local live smoke 통과 | current smoke는 default-hidden에서 `/workspace/**/.git/hooks/**`, `/tmp`, `/allowed` visible carve-out을 캡처 |
| bridge-visible traversal/listing | source evidence 통과 / repo-local live smoke 통과 | current smoke는 `/workspace`, `/workspace/repo`, `/workspace/repo/.git` traversal/listing과 hidden sibling `ENOENT`를 캡처 |
| `mutability.default=writable` | source evidence 통과 / repo-local live smoke 통과 | `docs/artifacts/current-default-writable-smoke-transcript.md`가 default writable write success, readonly override `EROFS`, hidden-before-mutability `ENOENT`를 캡처 |
| `mutability.default=readonly` | source evidence 통과 / repo-local 및 whole-root/chroot live smoke 통과 | current smokes는 readonly default 아래 writable carve-out success, bridge-visible mutation `EROFS`, whole-root `/etc` write `EROFS`를 캡처 |
| `mutability.writable` carve-out | source evidence 통과 / repo-local live smoke 통과 | current smoke는 `/tmp/existing` 및 `/allowed/existing` write success를 캡처 |
| `mutability.readonly` re-block | source evidence 통과 / repo-local live smoke 통과 | current smoke는 `/allowed/reblock/existing` write가 `EROFS`로 막히는 것을 캡처 |
| hidden-before-mutability | source evidence 통과 / repo-local live smoke 통과 | current smoke는 hidden `.git/config` mutation이 readonly보다 먼저 `ENOENT`가 되는 것을 캡처 |
| symlink fully-visible gate | source evidence 통과 / repo-local live smoke 통과 | current smoke는 bridge-visible target symlink `/link-to-bridge`의 `stat`/`readlink` 실패와 exit status를 캡처 |
| bridge-visible reachability performance | source/design evidence 통과 / anchored whole-root dynamic startup smoke 통과 | code는 request-time recursive scan 없이 static subtree bridge query와 rule-anchor-bounded dynamic bridge index를 사용한다. `current-dynamic-whole-root-smoke-transcript.md`는 `/etc/**/*.conf` whole-root startup_ms/RSS/fd를 캡처한다. bare slashless rule은 cwd-scoped shorthand이므로 whole-root coverage와 성능 판단은 계속 explicit recursive/anchored rule 기준으로 읽는다. |
| canonical 4-family glob comparison (`**/*.pem`, `./fixtures/*.pem`, `/a/*.txt`, `/a/**/*.txt`) | source evidence 통과 / repo-local live smoke 통과 | current source tests는 matching scope, representative non-match range, specificity/containment, `dynamic_bridge_scan_roots`를 family-by-family로 검증한다. `current-four-family-glob-smoke-transcript.md`는 `visibility.default=hidden` mount 4개에서 listing/access 결과와 `startup_ms` smoke evidence를 캡처한다. `startup_ms`는 formal benchmark가 아니다. |
| bare slashless cwd-anchor/direct-child target semantics | source evidence 통과 / repo-local live smoke 통과 | `cargo test --all-targets --all-features`와 `current-bare-basename-glob-smoke-transcript.md`가 cwd rebasing, cwd-outside-`source_root` fail-fast, `*.pem`=`./*.pem` equivalence, `*.pem` vs `**/*.pem` containment contrast를 current contract로 검증한다. `current-two-axis-smoke-transcript.md`의 `--hidden '*.pem'` line만 historical pre-support capture다. |
| unsupported glob fail-fast | source evidence 통과 / repo-local live smoke 통과 | current source tests는 `*secret*` 또는 `**/secret?.pem` 같은 still-unsupported form을 fail-fast로 검증한다. `current-bare-basename-glob-smoke-transcript.md`는 bare slashless와 혼동될 수 있는 `*secret*` stderr를 current live evidence로 캡처한다. |
| same-specificity conflict fail-fast | source evidence 통과 / repo-local live smoke 통과 | current two-axis smoke는 `--hidden /same --visible /same` exact-path conflict stderr를 캡처하고, `current-bare-basename-glob-smoke-transcript.md`는 `*.pem` vs `./*.pem` same-normalized-specificity conflict를 캡처한다. `*.pem` vs `**/*.pem`는 same-specificity conflict 예시가 아니라 containment/nested-override 예시다. |
| unprovable overlapping glob fail-fast | source evidence pass / repo-local live smoke 통과 | current two-axis smoke는 hidden `/workspace/**/*.pem` vs visible `/workspace/**/.git/hooks/**` overlap stderr를 캡처 |
| user-namespace chroot smoke | current whole-root/chroot live smoke 통과 | `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`가 `unshare -r -R <mount>`에서 `/etc/passwd` read, `/root` `ENOENT`, `/tmp` writable carve-out을 캡처 |
| 성능/메모리 측정 | 제한적 live smoke 통과 | static whole-root/chroot, dynamic anchored-glob whole-root, canonical 4-family repo-local transcripts가 current startup/workload `startup_ms`를 캡처한다. 이 값들은 smoke evidence이며 formal benchmark가 아니다. whole-root coverage/성능 판단은 explicit recursive/anchored rule smoke와 별도 benchmark로 보강한다. 일부 기존 transcripts는 RSS/fd count도 포함한다. |

## 문서 변경 검증

문서만 변경했을 때 최소 검증:

```bash
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts -maxdepth 3 -type f -print | sort
cargo check
```

추가로 `git diff -- README.md AGENTS.md docs .pi` 또는 추적 전 파일의 실제 내용을 확인해 요구사항 누락, 문서 간 모순, 미승인 임시 문구가 없는지 검토한다.
