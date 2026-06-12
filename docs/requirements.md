# ScreenFS 요구사항

## 1. 목적

`ScreenFS`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 존재하지 않는 것처럼 숨기고, 노출된 경로에는 별도 mutability policy를 적용할 수 있어야 한다. `pi-bash-sandbox`는 대표 통합 예시지만 프로젝트 목적을 그 통합 하나로 한정하지 않는다.

이 문서는 **visibility / mutability 두 축** 기준의 현재 계약을 정의한다. shared matcher는 bare slashless direct-child, anchored direct-child, subtree shorthand, recursive glob family를 함께 정규화하지만, current `visibility.visible` surface는 recursive bridge discovery가 필요 없는 범주만 허용한다. 따라서 recursive glob family(`**/*.pem`, `/**/*.pem`)와 recursive literal non-visible subtree canonical/shorthand family(`**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `~/**/aaa/hook/**`, `~/**/aaa/hook` 등)는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서는 계속 current일 수 있어도 `visibility.visible`에서는 현재 계약이 아니다. recursive literal directory shorthand는 non-visible surface에서만 지원되며 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`만 허용한다. `/**/`는 최대 한 번만 쓸 수 있고 tail component는 모두 literal이어야 하며, 내부적으로 `<prefix>/**/<literal-tail>/**`로 normalize되어 directory 자체와 descendants를 함께 포함한다. 예: `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`. 이는 subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이며 subtree shorthand 의미는 바뀌지 않는다. 세부 근거와 검증 요구사항은 `docs/operations.md`를 따른다.

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

`visibility.visible`은 hidden-by-default allowlist이거나 더 넓은 hidden 영역 내부의 carve-out이다. current contract는 recursive bridge discovery가 필요 없는 **discovery-free visible category**만 허용한다.

허용 범주:

1. **static subtree bridge**
   - exact path와 subtree shorthand(`/dir`, `/dir/**`)
   - visible target까지의 정적 ancestor chain만 bridge-visible이 된다.
2. **direct-child anchor bridge**
   - `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형(`*.pem`, `./dir/*`, `~/dir/*` 등)
   - normalized anchor의 ancestor들만 bridge-visible candidate가 된다.
   - anchor 바로 아래 immediate child match는 `lookup`/`readdir`/`readdirplus` 시점에 현재 directory/parent 기준으로만 평가하며, 그 directory와 무관한 matcher bucket은 건너뛴다. anchor subtree를 재귀 스캔해 bridge를 찾지 않는다.

비허용 범주:

- `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks` 같은 recursive descendant visible glob/shorthand
- `<normalized-prefix>/**/<literal>/**` canonical form과 그 recursive literal directory shorthand(`<normalized-prefix>/**/<literal>`)를 visible carve-out으로 쓰는 경우
- 거부 사유: visible descendant를 찾기 위한 recursive bridge discovery가 필요하기 때문
- 권장 대안: `/dir` 또는 `/dir/**` 같은 explicit subtree visible rule

중요: recursive glob family 자체는 제거된 것이 아니다. `visibility.hidden`, `mutability.readonly`, `mutability.writable`은 계속 recursive basename/suffix family와 recursive literal descendant-subtree canonical/shorthand family를 사용할 수 있다. 예를 들어 `**/.git/hooks`는 `**/.git/hooks/**`와, `/repo/**/.git/hooks`는 `/repo/**/.git/hooks/**`와 동일하게 normalize되어 매치된 directory 자체와 descendants를 모두 포함한다. current restriction은 `visibility.visible`에만 적용된다.

bridge-visible ancestor 규칙:

- bridge-visible은 ancestor directory에만 적용된다.
- subtree visible rule은 visible target까지의 existing ancestor directory만 bridge-visible로 합성한다.
- direct-child visible rule은 normalized anchor의 existing ancestor directory만 bridge-visible candidate가 되며, hidden sibling을 노출하지 않는다.
- bridge-visible directory는 `stat`/`lookup`/`getattr`/읽기 의도의 `access`/traverse/`opendir`/`readdir`/`readdirplus`만 허용한다.
- bridge-visible directory에 대한 mutation(`create`, `mkdir`, `rename`, `unlink`, metadata mutation 등)은 항상 `EROFS`다.
- bridge-visible directory listing은 visible entry와 visible descendant로 이어지는 entry만 반환한다. direct-child visible rule에서도 immediate child evaluation 결과에 없는 sibling은 숨겨진 채 남는다.
- bridge-visible은 traversal/listing 전용 상태다. symlink entry의 resolved virtual target이 fully visible이 아니고 hidden이거나 bridge-visible/non-fully-visible이면 `readlink`와 symlink dereference `open`은 `ENOENT`다.
- hidden path 자체는 계속 hidden이며 `ENOENT`다.
- symlink entry는 resolved virtual target이 fully visible일 때만 export된다. target이 hidden 또는 bridge-visible이면 listing에서 제외하고 `lookup`/`getattr`/`open`/`readlink`/dereference는 `ENOENT`다.
- symlink target visibility fast path는 compiled policy가 그 entry를 숨길 수 없음을 보일 때만 lexical target 재평가를 생략할 수 있다. 그렇지 않으면 listing/`lookup`/`getattr`/`readlink`/dereference/`open` 시점마다 lexical resolved target을 다시 확인해야 한다.
- symlink 판단은 cache contract가 아니다. prior listing success는 이후 point-of-use check 면제가 아니며, direct-path-only memoized result를 symlink-dependent check에 재사용하면 안 된다.

예시:

- `visibility.default=visible`, `visibility.hidden=/home`, `visibility.visible=/home/me/project`
  - `/home`, `/home/me`는 bridge-visible ancestor가 될 수 있다.
  - `/home/other`는 hidden이며 `ENOENT`다.
- `visibility.default=hidden`, `visibility.visible=/workspace`
  - `/`는 bridge-visible이 될 수 있고 `/workspace` subtree는 visible이다.
- `visibility.default=hidden`, `visibility.visible=/tmp/*`
  - `/`와 `/tmp`까지의 ancestor chain만 bridge-visible candidate다.
  - `/tmp` 아래 immediate child만 평가 대상이며 `/tmp` 전체를 재귀 스캔해 새 bridge를 찾지 않는다.
- `visibility.default=hidden`, `visibility.visible=**/.git/hooks/**` 또는 `visibility.visible=**/.git/hooks`
  - current contract에서는 둘 다 invalid configuration이다. recursive bridge discovery가 필요하므로 fail-fast 해야 하며 `/repo` 또는 `/repo/**` 같은 explicit subtree visible rule로 바꿔야 한다.

### 4.4 Listing 의미론

- hidden entry는 `readdir`, `readdirplus` 결과에서 제외된다.
- bridge-visible directory listing은 visible entry와 visible descendant로 이어지는 bridge-visible entry만 반환한다.
- directory-entry filtering fast path는 `readdir`/`readdirplus`가 현재 directory/parent 기준으로만 관련 matcher bucket을 보도록 제한할 수 있고, 그 directory와 무관한 bucket은 건너뛸 수 있다. 이 최적화는 hidden sibling 비노출과 listing 결과 의미론을 바꾸면 안 되며, recursive scan·background index·listing 결과 cache를 계약으로 요구하지 않는다.
- symlink child는 resolved virtual target이 fully visible일 때만 listing에 포함한다.
- `readdirplus`는 반환되는 visible/bridge-visible entry에 대해서만 metadata를 준다.
- listing에 보이는 symlink entry도 resolved virtual target이 fully visible일 때만 `readlink`/target dereference가 가능하다.
- 구현은 필요하면 per-handle iteration state를 둘 수 있지만, stable directory snapshot cache는 current contract가 아니다.
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

이 절은 visibility/mutability 모든 rule surface가 공유해야 하는 path-like rule normalization contract를 정의한다. shared matcher는 exact path와 제한된 glob family를 공통으로 정규화하지만, `visibility.visible`은 그중 discovery-free visible category만 현재 surface로 허용한다.

### 7.1 Shared matcher family

- absolute exact path는 virtual-root anchored semantics를 가진다.
- relative exact path와 relative prefixed glob prefix는 process cwd 기준 host path로 먼저 해석하고, 그 host path가 `source_root` 내부일 때만 source-root-relative virtual absolute path 또는 virtual glob prefix로 rebase한다.
- `~`/`~/...` 입력은 `HOME` 기준 host path로 expand한 뒤 같은 rebasing 규칙을 적용한다.
- shared matcher는 다음 family를 정규화한다.
  - **subtree / exact family**: exact path, `/dir`, `/dir/**`, `./dir`, `./dir/**`, `~/dir`, `~/dir/**`; same-anchor `/dir/**`는 `/dir`와 정확히 같은 normalized descriptor/specificity로 compile된다.
  - **direct-child glob family**: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `./dir/*`, `~/dir/*`, bare `*.pem`, `.env.*`, `id_*`; bare slashless form은 같은 normalized cwd anchor의 `./<pattern>` direct-child shorthand다.
  - **recursive non-visible glob family**: `**/.env`, `**/*.pem`, `**/.env.*`, `**/id_*`, `/**/.env`, `/**/*.pem`, `/**/.env.*`, `/**/id_*`, `./fixtures/**/*.pem`, `/a/**/*.txt`; prefixless form은 같은 normalized cwd anchor의 `./**/<pattern>` recursive shorthand다.
  - **recursive literal non-visible subtree family**: canonical form `<normalized-prefix>/**/<literal-component>(/<literal-component>)*/**`와 recursive literal directory shorthand `<normalized-prefix>/**/<literal-component>(/<literal-component>)*`; shorthand는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 지원된다. 허용 shorthand는 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`뿐이며 `/**/`는 최대 한 번만 쓸 수 있고 tail component는 모두 literal이어야 한다. 내부적으로는 `<prefix>/**/<literal-tail>/**` canonical form으로 compile된다. 이는 same-anchor subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이며 subtree shorthand 의미는 바뀌지 않는다. 예: `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`, `./repo/**/.git/hooks`=`./repo/**/.git/hooks/**`, `~/project/**/.git/hooks`=`~/project/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`, `**/target`=`**/target/**`, `**/dist`=`**/dist/**`, `**/build`=`**/build/**`

### 7.2 current `visibility.visible` 허용 범주

| category | 허용 syntax 예 | bridge 동작 | 비고 |
| --- | --- | --- | --- |
| static subtree bridge | `/dir`, `/dir/**`, `./dir`, `~/dir` | visible target까지의 정적 ancestor chain만 bridge-visible | `/dir/**`는 `/dir`와 동일 compile |
| direct-child anchor bridge | `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `*.pem`, `./dir/*`, `~/dir/*` | normalized anchor ancestor만 bridge-visible candidate, immediate child는 `lookup`/`readdir`에서 평가 | anchor subtree 재귀 스캔 금지, hidden sibling 비노출 |
| unsupported recursive visible glob/shorthand | `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks` | n/a | recursive bridge discovery가 필요하므로 shorthand와 canonical form 모두 fail-fast, `/dir` 또는 `/dir/**` 권장 |

중요:

- `visibility.hidden`, `mutability.readonly`, `mutability.writable`은 recursive non-visible glob family와 recursive literal non-visible subtree family를 계속 사용할 수 있다.
- current restriction은 `visibility.visible`에만 적용된다. recursive family가 visible surface에서 거부된다고 해서 shared matcher 전체에서 제거된 것은 아니다.

### 7.3 specificity / conflict / dedup / indexing contract

- 각 축에서는 **가장 구체적인 매치가 우선**한다.
- same-axis opposite rule이 같은 normalized anchor/specificity에서 충돌하면 fail-fast다.
- same-polarity identical descriptor는 deduplicate/idempotent다.
- matcher/indexing은 family와 normalized anchor를 기준으로 분리할 수 있어야 한다. 최소한 다음을 별도 descriptor 집합으로 유지한다.
  - exact/subtree descriptor
  - direct-child glob descriptor
  - recursive non-visible glob descriptor
  - recursive literal non-visible subtree descriptor
- same-anchor `/dir/**`는 `/dir` subtree rule과 동일 descriptor로 compile된다.
- same-anchor `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`는 `/dir/*` target set 안에 contained되는 더 구체적인 direct-child rule이다.
- same-anchor `/dir/*`는 `/dir` 및 `/dir/**` target set 안에 contained된다.
- recursive non-visible family에서는 기존 specificity/containment/conflict 규칙을 유지한다. 예를 들어 `**/*.pem`는 같은 normalized cwd anchor에서 `./**/*.pem`과 같은 descriptor/specificity로 compile되고, `*.pem`는 같은 anchor의 `./*.pem`과 같은 direct-child descriptor로 compile되며 same-anchor `**/*.pem` target set 안에 contained된다. recursive literal descendant-subtree shorthand도 canonical form과 동일 descriptor/specificity로 compile된다. 즉 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`이고 directory 자체와 descendants를 함께 매치하며 same-specificity conflict, containment, dedup 결과도 동일하다. descendant-subtree family끼리는 literal tail component 수가 더 많을수록, 그다음으로 normalized prefix가 더 길수록 더 구체적이다.
- `visibility.visible`에 recursive family가 들어오면 specificity 비교 이전에 fail-fast 한다. 즉 visible recursive rejection은 overlap/containment 문제로 늦게 처리하지 않는다.
- family/anchor bucket fast path는 current supported grammar에만 적용된다. unsupported visible recursive form이나 broader wildcard form을 부분 근사해 통과시키면 안 된다.

### 7.4 fail-fast 조건

- `HOME` 없음
- relative exact path / relative glob prefix / bare slashless glob shorthand / prefixless recursive shorthand를 정규화할 launch process cwd가 `source_root` 밖
- expanded host path가 `source_root` 밖
- `~user`
- wildcard가 prefix 내부에 섞이는 broader form(`foo/*/bar.pem`, `**/secret?.pem`)
- unanchored wildcard-all recursive form(`**/*`)
- one-sided basename-prefix/suffix subset 밖의 bare wildcard form(`*`, `a*b`, `*secret*`)
- descendant-subtree literal tail 내부 wildcard(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`)
- recursive literal directory shorthand subset 밖의 trailing `/**`-less 또는 multi-recursive descendant-subtree form(`~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]`, wildcard가 섞인 `foo/**/bar` 같은 broader/ambiguous input)
- `visibility.visible`에 recursive bridge discovery가 필요한 form(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks`)
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
- same-anchor `/dir/**`는 `/dir` subtree rule과 같은 descriptor/specificity로 compile되고, `/dir/*`는 same-anchor `/dir/*.pem` 같은 direct-child basename family보다 넓으며 `/dir`/`/dir/**` target set 안에 contained되는 direct-child wildcard-all rule이다. recursive literal descendant-subtree shorthand는 canonical trailing `/**` form과 같은 descriptor/specificity로 compile된다. 즉 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`이고 same-polarity dedup, opposite-polarity conflict, containment 결과도 동일하다. `visibility.visible`은 exact/subtree와 direct-child family만 current surface로 허용하고, recursive family(`**/*.pem`, `/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/dir/**/*.pem`)는 hidden/readonly/writable surface에만 남긴다. current verification 요구사항은 `docs/operations.md`를 따른다.
- 이 목표 계약에는 제거된 CLI/config surface를 위한 compatibility mapping이나 shim이 없다.

## 10. 성능 및 메모리 요구사항

전체 filesystem view를 제공하므로 metadata-heavy workload가 많을 수 있다. 중요한 최적화 방향:

- `readdirplus` 구현
- inode/path cache
- file handle 재사용
- visibility/mutability matcher 빠른 처리
- rule 증가 시 성능 저하 최소화
- visible file data path는 최대한 underlying filesystem으로 pass-through
- `visibility.visible` bridge-visible reachability는 discovery-free여야 한다. subtree visible rule은 정적 ancestor chain만 사용하고, direct-child visible rule은 normalized anchor ancestor와 immediate child evaluation만 사용해야 한다. startup recursive scan, lazy recursive discovery, dynamic bridge ancestor index 같은 discovery mechanism을 current contract로 추가하지 않는다.
- recursive literal directory shorthand 추가는 normalization/path-matcher-only delta여야 한다. `**/.git/hooks`, `~/**/aaa/hook` 같은 supported shorthand는 compile/match cost가 각각 `**/.git/hooks/**`, `~/**/aaa/hook/**`와 동일해야 하며, shorthand 때문에 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 추가하면 안 된다.
- `~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]` 같은 multi-recursive 또는 broader form은 discovery, ambiguous containment, broader glob compatibility를 피하기 위해 fail-fast 해야 한다.
- directory-entry filtering fast path는 `readdir`/`readdirplus`에서 현재 directory/parent 기준 관련 matcher bucket만 보고 unrelated bucket을 건너뛸 수 있어야 한다. 다만 hidden sibling 비노출과 결과 의미론은 그대로 유지해야 하며, recursive scan·background indexing·listing 결과 cache를 계약으로 요구하지 않는다.
- `/tmp/*` 같은 direct-child visible rule은 `/tmp` 전체를 재귀 순회하지 않아야 하며, 운영 검증에서 counter/trace/perf smoke 또는 동등한 계측으로 unrelated bucket skip과 결과 불변을 함께 증명해야 한다.
- recursive glob indexing은 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 family별로 유지할 수 있다. 이때도 exact/subtree, direct-child glob, recursive non-visible glob, recursive literal non-visible subtree descriptor를 분리하고 same-polarity identical descriptor dedup을 보장해야 한다. recursive literal directory shorthand는 canonical descriptor에 normalize될 뿐 별도 discovery/index family를 만들면 안 된다.
- symlink target visibility fast path는 policy가 target check를 생략해도 entry 비가시성이 생기지 않음을 증명할 때만 허용된다. 그 외에는 listing/lookup/getattr/readlink/dereference/open 시점마다 lexical resolved target을 다시 확인해야 하고, symlink decision cache나 direct-path-only memoized result 재사용은 current contract가 아니다.
- raw symlink target이 lexical virtual visibility 기준으로 fully visible하지만 host resolution에서 `source_root` 밖으로 escape하면 `readlink`는 raw target을 반환할 수 있고, dereference/open/access는 confinement 단계에서 `ENOENT`로 실패해야 함
- live mount smoke와 performance smoke는 visible recursive rejection, direct-child no-recursive-traversal, symlink point-of-use check 계약의 계속된 증거여야 함
- 위 fast path들은 current supported grammar에만 적용되며 unsupported visible recursive form이나 broader wildcard form을 근사하면 안 됨
- memory footprint는 장시간 실행에도 과도하게 증가하지 않아야 함

## 11. 현재 프로젝트 상태

프로젝트 경로:

```text
/home/spi-ca/Codebase/screenfs
```

현재 source of truth는 two-axis surface다. 문서와 recorded artifact는 current contract 기준으로 유지한다.

확인된 환경:

```text
kernel: 7.1.0-rc7-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
```

위 kernel config artifact는 현재 개발 환경이 `FUSE_OVER_IO_URING` 전제를 충족하도록 빌드되었음을 보여주는 정적 증거다. 이는 별도의 live mount smoke나 세션 협상 성공 주장과 동일하지 않다.
