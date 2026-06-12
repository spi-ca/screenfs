# ScreenFS 요구사항

## 1. 목적

`ScreenFS`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 존재하지 않는 것처럼 숨기고, 노출된 경로에는 별도 mutability policy를 적용할 수 있어야 한다. `pi-bash-sandbox`는 대표 통합 예시지만 프로젝트 목적을 그 통합 하나로 한정하지 않는다.

이 문서는 **visibility / mutability 두 축** 기준의 현재 계약을 정의한다. bare slashless glob의 cwd-anchored `./<pattern>` direct-child semantics와 prefixless recursive shorthand(`**/*.pem`, `**/.env.*`, `**/id_*`)의 cwd-anchored recursive semantics는 최신 source/test/smoke evidence로 확인돼 있으며, 세부 근거는 `docs/operations.md`를 따른다.

- **visibility 축**: 무엇이 보이는가
- **mutability 축**: 보이는 것 중 무엇이 쓰기 가능한가

## 2. 핵심 전제

- non-root에서 실행된다.
- `fractal-fuse = 0.4.0` 기반으로 구현한다.
- FUSE3 및 `FUSE_OVER_IO_URING` 기반 사용을 v1 필수 정책으로 삼는다.
- v1은 `FUSE_OVER_IO_URING` 협상 실패 시 fallback mount를 만들지 않고 명시적 오류로 fail-fast 한다.
- mount는 `fusermount3`로 수행한다.
- 대표 사용 시나리오에 chroot가 포함되므로 mount 결과는 전체 파일시스템 뷰를 제공해야 한다.
- `chroot` 실행 권한, same-host-uid 접근 모델, `/proc`·`/sys`·`/dev`·`/run` native semantics는 `ScreenFS` 단독 책임이 아니라 상위 supervisor/namespace layer 책임이다.

## 3. Non-root whole-root consumer 요구

`ScreenFS`는 root 권한 없이 전체 `/` view를 제공해야 한다.

- 일반 사용자 권한으로 FUSE mount 가능해야 한다.
- root-only mount, privileged bind mount, system-wide mount namespace 조작에 의존하지 않는다.
- `/dev/null` bind overlay, 빈 파일 overlay, tmpfs masking처럼 이름을 남기는 masking 방식은 사용하지 않는다.
- sandboxed consumer가 필요한 binary, shared library, config, runtime path를 정상적으로 볼 수 있어야 한다.
- 차단 대상만 존재하지 않는 것처럼 처리해야 하며, hidden 대상은 가능한 한 `Permission denied` 대신 `ENOENT`를 사용한다.

## 4. Visibility 계약

![ScreenFS visibility axis](diagrams/visibility-axis.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### 4.1 축 정의

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []
```

- `visibility.default=visible`: 기본은 보이고, `visibility.hidden`이 숨기며, 더 구체적인 `visibility.visible`이 carve-out으로 다시 노출할 수 있다.
- `visibility.default=hidden`: 기본은 숨겨지고, `visibility.visible`이 노출하며, 더 구체적인 `visibility.hidden`이 다시 re-block 할 수 있다.
- 이 축은 다른 축과 독립적으로 평가되지만, mutability보다 먼저 적용된다.

### 4.2 Hidden 의미론

hidden으로 판정된 path는 실제로 없는 것처럼 보여야 한다.

| Operation | 기대 동작 |
| --- | --- |
| `readdir` | 결과에서 제외 |
| `readdirplus` | 결과에서 제외 |
| `lookup` | `ENOENT` |
| `getattr` | `ENOENT` |
| `open` | `ENOENT` |
| `access` | `ENOENT` |
| `readlink` | `ENOENT` |

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

### 4.3 `visibility.visible`과 bridge-visible ancestor

`visibility.visible`은 hidden-by-default allowlist이거나 더 넓은 hidden 영역 내부의 carve-out이다. 이때 visible descendant에 도달시키기 위해 필요한 ancestor directory는 **bridge-visible** 상태로 취급할 수 있다. descendant-subtree carve-out도 예외가 아니며, `**/.git/hooks/**` 같은 rule이 매치한 visible subtree에 도달하려면 그 경로상의 모든 existing ancestor directory가 bridge-visible로 합성되어 traversal/listing이 가능해야 한다.

bridge-visible ancestor 규칙:

- bridge-visible은 ancestor directory에만 적용된다.
- descendant-subtree carve-out(`**/.git/hooks/**` 등)은 실제 visible target까지의 모든 existing ancestor directory를 bridge-visible로 합성해야 한다.
- bridge-visible directory는 `stat`/`lookup`/`getattr`/읽기 의도의 `access`/traverse/`opendir`/`readdir`/`readdirplus`만 허용한다.
- bridge-visible directory에 대한 mutation(`create`, `mkdir`, `rename`, `unlink`, metadata mutation 등)은 항상 `EROFS`다.
- bridge-visible directory는 hidden sibling을 노출하지 않으며, visible descendant로 이어지는 entry만 보여준다.
- bridge-visible은 traversal/listing 전용 상태다. symlink entry의 resolved virtual target이 fully visible이 아니고 hidden이거나 bridge-visible/non-fully-visible이면 `readlink`와 symlink dereference `open`은 `ENOENT`다.
- hidden path 자체는 계속 hidden이며 `ENOENT`다.
- symlink entry는 resolved virtual target이 fully visible일 때만 export된다. target이 hidden 또는 bridge-visible이면 listing에서 제외하고 `lookup`/`getattr`/`open`/`readlink`/dereference는 `ENOENT`다.

예시:

- `visibility.default=visible`, `visibility.hidden=/home`, `visibility.visible=/home/me/project`
  - `/home`, `/home/me`는 bridge-visible ancestor가 될 수 있다.
  - `/home/other`는 hidden이며 `ENOENT`다.
- `visibility.default=hidden`, `visibility.visible=/workspace`
  - `/`는 bridge-visible이 될 수 있고 `/workspace` subtree는 visible이다.
- `visibility.default=hidden`, `visibility.visible=**/.git/hooks/**`
  - 각 매치 인스턴스마다 visible hook subtree로 내려가는 기존 ancestor directory들이 bridge-visible이 된다.
  - bridge-visible ancestor를 거쳐 도달한 symlink라도 resolved virtual target이 fully visible이 아니면 `readlink`/`open`은 `ENOENT`다.

### 4.4 Listing 의미론

- hidden entry는 `readdir`, `readdirplus` 결과에서 제외된다.
- bridge-visible directory listing은 visible entry와 visible descendant로 이어지는 bridge-visible entry만 반환한다.
- symlink child는 resolved virtual target이 fully visible일 때만 listing에 포함한다.
- `readdirplus`는 반환되는 visible/bridge-visible entry에 대해서만 metadata를 준다.
- listing에 보이는 symlink entry도 resolved virtual target이 fully visible일 때만 `readlink`/target dereference가 가능하다.
- hidden directory 전체가 listing에서 빠질 때도 이름만 남기는 masking을 하지 않는다.

## 5. Mutability 계약

![ScreenFS mutability axis](diagrams/mutability-axis.svg)

다이어그램 원본: [diagrams/README.md](diagrams/README.md)

### 5.1 축 정의

```yaml
mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

- `mutability.default=writable`: 기본은 쓰기 가능하고, `mutability.readonly`가 쓰기를 막으며, 더 구체적인 `mutability.writable`이 carve-out으로 다시 허용할 수 있다.
- `mutability.default=readonly`: 기본은 읽기 전용이고, `mutability.writable`이 쓰기를 허용하며, 더 구체적인 `mutability.readonly`가 다시 re-block 할 수 있다.

### 5.2 Visible path에 대한 의미론

visible path의 읽기/탐색은 visibility 축에서 허용되면 pass-through다. 쓰기성 operation은 mutability 축으로 별도 판정한다.

- readonly로 판정된 visible path의 mutation은 `EROFS`
- writable로 판정된 visible path의 mutation은 host filesystem 권한이 허용하면 pass-through
- bridge-visible ancestor는 mutability 축과 무관하게 mutation 시 항상 `EROFS`

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
setxattr
removexattr
fallocate
```

### 5.3 Hidden precedence

- hidden path와 fully visible이 아닌 symlink target(숨겨졌거나 bridge-visible에만 도달하는 target)은 mutability보다 먼저 처리된다.
- 따라서 hidden 또는 non-fully-visible symlink target이 관여하면 결과는 항상 `ENOENT`다.
- readonly/writable rule은 hidden 결과를 뒤집지 못한다.

## 6. Shared rule semantics

visibility와 mutability 축은 같은 rule semantics를 공유해야 한다.

- exact path와 supported glob은 같은 normalization contract를 재사용한다.
- `visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`는 동일한 rule grammar를 사용한다.
- 각 축에서는 **가장 구체적인 매치가 우선**한다.
- 같은 축에서 반대 polarity rule이 같은 normalized anchor/specificity에서 충돌하면 fail-fast다.
- 같은 축의 반대 polarity glob rule 조합이 overlap 가능하지만 normalized target-set containment를 증명할 수 없으면 fail-fast다.
- duplicate same-polarity rule은 허용하지만 의미는 idempotent다.
- write-intent `open`, path-only mutation, multi-path mutation(`rename`, `link`, `symlink`, `copy_file_range`)은 affected coordinate별로 visibility와 mutability를 판정한다.
- `copy_file_range`는 source visibility와 destination mutability를 분리해 본다.

## 7. Rule input 정규화 계약

이 절은 visibility/mutability 모든 rule surface가 공유해야 하는 path-like rule normalization contract를 정의한다.

현재 shared normalization contract는 아래와 같다. bare slashless glob의 cwd-anchored `./<pattern>` direct-child semantics와 prefixless recursive shorthand(`**/*.pem`, `**/.env.*`, `**/id_*`)의 cwd-anchored recursive retargeting은 current source/test/smoke evidence가 있으며, source/live evidence 상태는 `docs/operations.md`에서 추적한다.

- absolute exact path는 virtual-root anchored semantics를 가진다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 host path가 `source_root` 내부일 때만 source-root-relative virtual absolute path 또는 virtual glob prefix로 rebase한다.
- `~`/`~/...` 입력은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- exact path와 함께 다음 limited glob subset을 지원한다.
  - recursive basename/suffix/basename-prefix tail: prefixless form(`**/.env`, `**/*.pem`, `**/.env.*`, `**/id_*`)은 launch process cwd를 먼저 host path로 정규화한 뒤 `source_root` relative normalized prefix로 rebase한 recursive shorthand다. 같은 normalized cwd anchor에서 `./**/.env`, `./**/*.pem`, `./**/.env.*`, `./**/id_*`와 동등하며, cwd가 `source_root` 밖이면 fail-fast 한다.
  - explicit-root 또는 normalized-prefix recursive form: `/**/.env`, `/**/*.pem`, `/**/.env.*`, `/**/id_*`, `./fixtures/**/*.pem`, `/a/**/*.txt`; `/**/*.pem` 같은 explicit root-anchor form은 whole-tree recursive semantics를 가지며, anchored recursive form은 해당 normalized prefix 아래 임의 깊이의 matching basename에 계속 매치한다.
  - bare slashless glob shorthand: `/` component가 없는 `*.pem`, `*.key`, `.env.*`, `id_*` 같은 supported basename-prefix/suffix pattern은 `./<pattern>` shorthand다. launch process cwd를 먼저 host path로 정규화해 `source_root` relative normalized prefix로 rebase하고, 그 cwd가 `source_root` 밖이면 fail-fast 한다. 결과 rule은 그 cwd anchor 바로 아래의 immediate child basename만 매치하며, matched child 자체와 그 descendants에 적용된다. 즉 `*.pem`은 같은 normalized cwd anchor에서 `./*.pem`과 동등하고, 같은 anchor의 `**/*.pem`보다 더 구체적인 direct-child rule이다.
  - normalized-prefix direct-child basename-prefix/suffix form: `./fixtures/*.pem`, `~/.env.*`, `/home/<user>/*.pem`; `/a/*.txt`는 `/a/file.txt`와 그 descendants에만 매치하고 `/a/b/file.txt`에는 매치하지 않는 반면, same-anchor recursive form `./fixtures/**/*.pem`, `/a/**/*.txt`, `**/*.pem`은 각각 해당 anchor 아래 임의 깊이의 matching basename에 계속 매치한다.
  - limited recursive literal descendant-subtree glob: `<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`
- descendant-subtree glob 예: `**/.git/**`, `**/.git/hooks/**`, `./repo/**/.git/hooks/**`, `~/project/**/.git/hooks/**`
- descendant-subtree glob은 literal tail subtree root 자체와 그 모든 descendants에 매치된다.

### 7.1 Canonical 4-family glob table

| Syntax / family | Anchor normalization | Matching scope | Matched entry + descendants | Representative non-match | Specificity / conflict / containment | `visibility.visible` bridge-visible startup/performance impact |
| --- | --- | --- | --- | --- | --- | --- |
| `**/*.pem`<br>prefixless recursive suffix shorthand | prefix에 explicit anchor가 없다. launch cwd host path를 `source_root` 내부 virtual prefix로 rebase해 recursive anchor로 사용하며, 같은 normalized cwd anchor에서 `./**/*.pem`과 동등하다. cwd가 `source_root` 밖이면 fail-fast다. | normalized cwd anchor 아래 임의 깊이의 `*.pem` basename | 각 matched entry 자체에 적용되고, matched entry가 directory면 descendants도 함께 포함된다. | normalized cwd anchor 밖의 `cert.pem` | 같은 normalized cwd anchor의 opposite-polarity `**/*.pem`/`./**/*.pem`과는 same-specificity conflict가 가능하다. 같은 anchor의 `*.pem`/`./*.pem` direct-child target set을 포함하는 더 넓은 recursive rule이고, explicit root-anchor `/**/*.pem`과는 다른 anchor family다. | scan root는 normalized cwd anchor다. `visibility.visible` bridge index도 그 anchor subtree만 재귀 탐색하면 되고, whole-root scan은 explicit `/**/*.pem`에서만 필요하다. |
| `./fixtures/**/*.pem`<br>cwd-rebased anchored recursive suffix | `./fixtures` prefix를 launch cwd host path에서 해석한 뒤 `source_root` 내부 virtual prefix로 rebase한다. | normalized `./fixtures` anchor 아래 임의 깊이의 `*.pem` basename | 각 matched entry 자체에 적용되고, matched entry가 directory면 descendants도 함께 포함된다. | `<normalized ./fixtures anchor 밖>/local.pem` | 같은 normalized anchor의 opposite-polarity recursive `.pem` rule과는 same-specificity conflict가 가능하다. 같은 anchor의 `./fixtures/*.pem` direct-child family는 더 specific하며 target set이 그 안에 포함된다. | scan root는 normalized `./fixtures` anchor다. recursive family라 direct-child `./fixtures/*.pem`보다 discovery 범위는 넓지만 whole-root로 퍼지지는 않는다. |
| `/a/*.txt`<br>absolute anchored direct-child suffix | virtual-root anchored absolute prefix `/a`; cwd rebasing 없음 | `/a` 바로 아래 immediate child basename만 | 각 matched immediate child 자체와 그 descendants | `/a/b/file.txt` | 같은 `/a` direct-child `.txt` normalized anchor와 same-specificity conflict/duplicate가 된다. `/a/**/*.txt`보다 more-specific하고 target set은 그 안에 포함된다. | anchored direct-child라 scan root는 `/a`로 제한된다. 다만 current bridge-index walk는 startup에서 `/a` 아래를 재귀 탐색할 수 있어 direct-child matcher 자체보다 넓은 discovery cost가 남는다. |
| `/a/**/*.txt`<br>absolute anchored recursive suffix | virtual-root anchored absolute prefix `/a`; cwd rebasing 없음 | `/a` 아래 임의 깊이의 `*.txt` basename | 각 matched entry 자체와 그 descendants | `/b/file.txt` | `/a/*.txt` target set을 포함하는 broader recursive rule이다. 같은 `/a` recursive `.txt` form opposite-polarity rule과 same-specificity conflict가 가능하다. | scan root는 `/a`지만 recursive family라 `/a/*.txt`보다 훨씬 넓은 startup discovery가 가능하다. 그래도 explicit root-anchor `/**/*.pem`처럼 root 전체로 퍼지는 형태와는 다르다. |

중요: `**/*.pem`은 더 이상 whole-tree recursive family가 아니다. 같은 normalized cwd anchor에서는 `./**/*.pem`과 동등한 recursive shorthand이고, whole-tree recursive intent는 `/**/*.pem` 같은 explicit root-anchor form으로 표현한다. bare slashless `*.pem`는 같은 anchor의 direct-child shorthand로 남으며 same-anchor `**/*.pem` target set에 contained된다.

fail-fast 조건:

- `HOME` 없음
- relative exact path / relative glob prefix / bare slashless glob shorthand / prefixless recursive shorthand를 정규화할 launch process cwd가 `source_root` 밖
- expanded host path가 `source_root` 밖
- `~user`
- wildcard가 prefix 내부에 섞이는 broader form(`foo/*/bar.pem`, `**/secret?.pem`)
- one-sided basename-prefix/suffix subset 밖의 bare wildcard form(`*`, `a*b`, `*secret*`)
- descendant-subtree literal tail 내부 wildcard(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`)
- trailing `/**` 없는 descendant-subtree form(`**/.git/hooks`)
- brace/env/command expansion

## 8. Operation 의미론 요약

평가 순서:

1. normalized virtual path를 만든다.
2. mount-root exclusion과 visibility를 평가한다.
3. hidden이면 즉시 `ENOENT`다.
4. bridge-visible ancestor mutation이면 즉시 `EROFS`다.
5. 그 외 visible path에 대해 mutability를 평가한다.
6. readonly이면 `EROFS`, writable이면 host filesystem으로 위임한다.

멀티패스 보충 규칙:

- `rename`, `link`, `symlink`, `copy_file_range`는 관련 source/target/parent 전체를 visibility와 mutability 기준으로 본다.
- 하나라도 hidden이면 결과는 `ENOENT`다.
- hidden이 없고 write-requiring coordinate 중 하나라도 readonly 또는 bridge-visible ancestor mutation이면 `EROFS`다.
- 그 외에는 host errno를 최대한 보존한다.

## 9. Canonical CLI / config 계약

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--visibility-default visible|hidden] \
  [--hidden <rule> ...] \
  [--visible <rule> ...] \
  [--mutability-default writable|readonly] \
  [--readonly <rule> ...] \
  [--writable <rule> ...]
```

Canonical config shape:

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []

mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

CLI/config semantics:

- CLI axis option이 하나라도 있으면 해당 축의 config block 전체를 대체한다.
- explicit default가 없으면 visibility 기본값은 `visible`, mutability 기본값은 `writable`로 읽는다.
- 같은 축의 more-specific override가 덜 구체적인 rule보다 우선한다.
- `**/*.pem`는 같은 normalized cwd anchor에서 `./**/*.pem`과 같은 specificity/equivalence로 compile된다. `*.pem`는 같은 normalized cwd anchor에서 `./*.pem`과 같은 direct-child specificity/equivalence로 compile되며, same-anchor `**/*.pem` target set 안에 contained되는 더 구체적인 rule이다. current fresh evidence는 두 관계를 모두 검증하며, 상세 상태는 `docs/operations.md`를 따른다.
- 이 목표 계약에는 제거된 예전 family/flag surface를 위한 compatibility alias나 legacy shim이 없다.

## 10. 성능 및 메모리 요구사항

전체 filesystem view를 제공하므로 metadata-heavy workload가 많을 수 있다. 중요한 최적화 방향:

- `readdirplus` 구현
- inode/path cache
- file handle 재사용
- visibility/mutability matcher 빠른 처리
- rule 증가 시 성능 저하 최소화
- visible file data path는 최대한 underlying filesystem으로 pass-through
- bridge-visible reachability 판정은 startup에 구축한 bridge ancestor index 또는 동등한 bounded/cacheable 구조를 사용해야 하며 hot path에서 unbounded whole-root recursive scan을 요구해서는 안 됨
- `visibility.visible` glob startup 범위는 family마다 다르다. same-cwd-anchor recursive shorthand `**/*.pem`은 scan root가 normalized cwd anchor다. `./fixtures/**/*.pem`, `/a/**/*.txt`는 각 anchor subtree 전체로 더 넓어질 수 있고, `/a/*.txt`, `*.pem` 같은 direct-child family도 current bridge-index walk는 anchor 아래를 재귀 탐색할 수 있지만 scan root는 anchor로 제한된다. explicit root-anchor `/**/*.pem`은 `source_root`가 scan root다.
- dynamic glob visible rule의 bridge-visible ancestor index는 mount-start snapshot이며, 외부 backing-tree 변경으로 새 visible descendant가 생겨도 remount 전에는 previously unreachable hidden ancestor를 새로 노출하지 않음
- raw symlink target이 lexical virtual visibility 기준으로 fully visible하지만 host resolution에서 `source_root` 밖으로 escape하면 `readlink`는 raw target을 반환할 수 있고, dereference/open/access는 confinement 단계에서 `ENOENT`로 실패해야 함
- live mount smoke와 performance smoke는 bridge-visible reachability/latency 계약의 계속된 증거여야 함
- memory footprint는 장시간 실행에도 과도하게 증가하지 않아야 함

## 11. 현재 프로젝트 상태와 archival note

프로젝트 경로:

```text
/home/spi-ca/Codebase/screenfs
```

현재 source of truth는 two-axis surface다. 일부 transcript filename과 historical note에는 예전 naming이 남아 있을 수 있지만, 그것들은 archival context로만 읽는다.

확인된 환경:

```text
kernel: 7.1.0-rc7-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
```

위 kernel config artifact는 현재 개발 환경이 `FUSE_OVER_IO_URING` 전제를 충족하도록 빌드되었음을 보여주는 정적 증거다. 이는 별도의 live mount smoke나 세션 협상 성공 주장과 동일하지 않다.
