# ScreenFS 요구사항

## 1. 목적

`ScreenFS`는 non-root whole-root consumer를 위한 FUSE 기반 filesystem view layer다. 실제 `/`를 pass-through 하면서 민감 경로는 존재하지 않는 것처럼 숨기고, 노출된 경로에는 별도 mutability policy를 적용한다.

대표 통합 예시는 `pi-bash-sandbox` 같은 sandbox/chroot consumer지만, 프로젝트 목적을 그 통합 하나로 한정하지 않는다.

## 2. 핵심 전제

- non-root 사용자 권한으로 실행한다. Runtime은 effective uid 0 실행을 startup error로 거부한다.
- mount는 `fusermount3`와 FUSE3 기준이다.
- `fractal-fuse = 0.4.0`을 유지한다.
- v1은 `FUSE_OVER_IO_URING` 협상 성공을 요구한다. 실패하면 fallback mount 없이 startup error로 실패한다.
- 이 async 요구는 FUSE request/reply transport에 한정된다. backing filesystem metadata/data path 전체를 `io_uring`로 바꾸는 것은 v1 요구사항이 아니다.
- 대표 consumer가 chroot일 수 있으므로 mount 결과는 전체 `/` view여야 한다.
- `chroot` 권한 모델, user namespace, `/proc`·`/sys`·`/dev`·`/run` native semantics는 상위 supervisor/namespace layer 책임이다.

## 3. Non-goals

- root-only bind/overlay mount
- privileged mount namespace 조작
- `/dev/null` bind, 빈 파일, tmpfs masking처럼 이름을 남기는 masking
- device node, procfs, sysfs의 native/caller-relative semantics 재현
- hidden hardlink alternate path의 자동 전역 차단
- backing filesystem I/O 전체의 wholesale `io_uring` 전환
- 제거된 CLI/config surface를 위한 compatibility mapping 또는 shim

## 4. Canonical policy surface

정책 surface는 `visibility`와 `mutability` 두 축뿐이다.

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

### Visibility requirements

- `visibility.hidden` path는 `readdir`/`readdirplus` 결과에서 제외한다.
- hidden path의 `lookup`, `getattr`, `open`, `access`, `readlink`는 `ENOENT`다.
- `visibility.visible`은 hidden-by-default allowlist 또는 hidden 영역 내부 carve-out이다.
- visible carve-out까지 필요한 ancestor는 bridge-visible이 될 수 있다.
- bridge-visible ancestor는 stat/traverse/list만 허용하고 mutation은 `EROFS`다.
- bridge-visible listing은 visible child 또는 visible descendant로 이어지는 entry만 보여준다.
- symlink entry와 target은 point-of-use에서 fully visible이어야 한다. hidden 또는 bridge-visible/non-fully-visible target은 `ENOENT`다.
- Raw symlink target이 lexical visibility 기준으로 fully visible하지만 host resolution에서 `source_root` 밖으로 escape하면 `readlink`는 raw target을 반환할 수 있고, dereference/open/access는 confinement 단계에서 `ENOENT`로 실패한다. 이는 documented information-exposure boundary다.

Current `visibility.visible` 허용 범주:

| category | 예 | 요구사항 |
| --- | --- | --- |
| static subtree bridge | `/dir`, `/dir/**`, `./dir`, `~/dir` | target까지의 정적 ancestor chain만 bridge-visible |
| direct-child anchor bridge | `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `*.pem`, `./dir/*`, `~/dir/*` | normalized anchor ancestor만 bridge-visible candidate, immediate child만 평가 |
| recursive visible glob/shorthand | `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks`, `/repo/**/.git/hooks` | unsupported/fail-fast; recursive bridge discovery 금지 |

### Mutability requirements

- mutability는 fully visible path mutation에만 적용한다.
- `mutability.default=writable`: 기본 허용, `readonly`가 막고 더 구체적인 `writable`이 다시 허용할 수 있다.
- `mutability.default=readonly`: 기본 차단, `writable`이 허용하고 더 구체적인 `readonly`가 다시 막을 수 있다.
- readonly mutation은 `EROFS`다.
- bridge-visible ancestor mutation도 `EROFS`다.
- hidden 또는 non-fully-visible symlink target이 관여하면 mutability보다 먼저 `ENOENT`다.
- `rename`, `link`, `symlink`, `copy_file_range` 같은 multi-path operation은 source/target/parent coordinate별로 visibility와 mutability를 판정한다.

## 5. Shared rule grammar requirements

네 rule list(`visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`)는 같은 normalization contract를 공유한다. 단, `visibility.visible`은 discovery-free subset만 current surface로 허용한다.

지원 family:

- exact/subtree: `/dir`, `/dir/**`, `./dir`, `./dir/**`, `~/dir`, `~/dir/**`
- direct-child glob: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `*.pem`, `.env.*`, `id_*`
- recursive non-visible glob: `**/*.pem`, `/**/*.pem`, `./fixtures/**/*.pem`, `/a/**/*.txt`
- recursive literal non-visible subtree: `**/.git`, `**/.git/hooks`, `/repo/**/.git/hooks`, `~/**/aaa/hook`; internally canonical trailing `/**` form과 동등

Rules:

- 각 축은 most-specific rule wins를 따른다.
- same-polarity identical descriptor는 idempotent다.
- same-axis opposite-polarity rule이 같은 normalized anchor/specificity에서 충돌하면 fail-fast다.
- containment/overlap을 증명할 수 없는 반대 polarity glob 조합은 fail-fast다.
- `HOME` 없음, source-root 밖 cwd/expanded path, `~user`, prefix 내부 wildcard, `*`, `a*b`, `*secret*`, `**/*`, multi-recursive 또는 ambiguous descendant form은 fail-fast다.

## 6. Performance and memory requirements

- whole-root metadata workload를 고려해 matcher와 directory listing은 bounded해야 한다.
- `readdir`/`readdirplus`는 FUSE `size` budget에 맞는 bounded page를 반환하고, 같은 directory handle 안에서 stable resume cookie와 shared cookie domain을 유지하되, offset 이후 전체 directory snapshot을 계약으로 만들지 않는다.
- `readdirplus` lookup ref는 실제 반환 page의 child에만 증가시킨다.
- direct-child visible rule은 anchor subtree를 재귀 스캔하지 않는다.
- recursive literal directory shorthand는 normalization/path-matcher-only delta여야 하며 discovery, startup scan, background indexing, listing-result cache, symlink-decision cache를 추가하지 않는다.
- symlink visibility check는 cross-request cache로 생략하지 않는다. request-local reuse만 허용된다.
- selective file data-path async/io_uring 또는 sync-surface offload는 benchmark/API/semantic evidence가 있을 때 별도 follow-up으로 다룬다.

## 7. 현재 환경 evidence

현재 개발 환경 evidence는 운영 문서에서 관리한다. 요약:

```text
kernel: 7.1.0-rc7-1-spica-git
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
```

이 정적 환경 근거는 live mount smoke 성공 주장과 구분한다. 최신 검증 상태는 [`operations.md`](operations.md)를 따른다.
