# ScreenFS 설계 문서

이 문서는 `ScreenFS`를 구현하기 위한 설계 기준이다. 현재 저장소에는 v1 핵심 모듈의 초기 구현이 포함되어 있지만, 이 문서는 여전히 구현 완료 선언이 아니라 구현자가 따라야 할 계약이다.

아키텍처 시각화 요약은 [docs/architecture.md](architecture.md)에 정리되어 있다. 다이어그램 원본과 공용 렌더링 규칙은 [docs/diagrams/README.md](diagrams/README.md)를 따른다. `docs/diagrams/*.png`는 `docs/diagrams/mermaid-config.json`, `docs/diagrams/puppeteer-config.json`을 함께 사용하고 Mermaid CLI `--scale 2`로 렌더링하는 것을 기준으로 읽는다.

> Implementation status note (2026-06): v1 core modules `cli`, `config`, `errors`, `path`, `matcher`, and `fs` now include the family-aware mutability surface. Current source implements `--policy-family`, `--readonly-rule`, `--allow-write`, `--config`, YAML `mutability` loading, one-family-per-mount validation, CLI-over-config precedence, shared hide/readonly/allow-write normalization, hidden-before-`EROFS` precedence, and affected-coordinate-wide writability checks for path-only and multi-path mutation. Fresh mount-free verification also covers visible-path `access` pass-through, symlink-target guards on `lookup`/`open`/`readlink`, whole-mount readonly via `readonly-root-allowwrite` empty carve-out, xattr/fallocate guards, source-root symlink escape rejection, mount-root recursion exclusion, and `copy_file_range` source/destination handling. Fresh current-session verification includes `cargo test --all-targets --all-features` passing 73 tests plus repo-local real-FUSE family-aware smoke for `selective-readonly`, `readonly-root-allowwrite`, config-backed family selection, relative hide exact path, relative/`~` prefixed-glob rules, and conflict fail-fast stderr (`docs/artifacts/future-mutability-smoke-transcript.md`). Fresh current-session whole-root/chroot smoke also exists for the current CLI surface via `readonly-root-allowwrite --allow-write /tmp` (`docs/artifacts/whole-root-family-smoke-transcript.md`) and `readonly-root-allowwrite` with empty `allow_write` (`docs/artifacts/whole-mount-readonly-smoke-transcript.md`). Pre-removal whole-root/chroot smoke remains a separate archival artifact (`docs/artifacts/fuse-smoke-transcript.md`). This remains partial implementation progress only; the design requirements below still define the remaining v1 contract and any remaining family-by-family whole-root/chroot matrix gap.

## 한눈에 보기

### 시스템 컨텍스트

![ScreenFS system context](diagrams/system-context.png)

### 요청 처리 흐름

![ScreenFS request decision flow](diagrams/request-decision-flow.png)

### 모듈 구조

![ScreenFS module architecture](diagrams/module-architecture.png)

## 빠른 읽기 가이드

- **무엇을 만드는가**: 1~4절
- **코드 경계와 데이터 흐름**: 5~8절
- **의미론과 errno 계약**: 9~11절
- **magic fs / device / integration boundary**: 12절
- **성능·검증·수용 기준**: 13절 이후

## 현재 구현 vs 목표 계약

| 관점 | 현재 구현에서 확인된 것 | 여전히 설계 목표로 읽어야 하는 것 |
| --- | --- | --- |
| hidden/readonly 기본 의미론 | hidden=`ENOENT`, readdir 필터링, family-aware evaluator 기준 visible mutation=`EROFS`/carve-out 허용, current whole-root/chroot smoke 존재 | family-by-family whole-root/chroot matrix 보강 |
| readonly 설정 표면 | current source에는 `--policy-family`, `--readonly-rule`, `--allow-write`, `--config`, YAML `mutability` block, one-family-per-mount/CLI-over-config/fail-fast가 구현돼 있다 | family-by-family whole-root/chroot live evidence 추가 보강 |
| rule 입력 정규화 | hide, current `--readonly-rule`, `--allow-write`가 shared normalization contract를 사용하며 absolute/relative/`~` exact rule, recursive prefixed limited glob, normalized-prefix direct-child basename-prefix/suffix form(`~/.env.*`, `~/*.pem` 등)을 mount-free test로 검증하고, repo-local live smoke는 relative hide exact + relative/`~` prefixed glob 일부를 보강한다 | already-absolute live cases, broader unsupported wildcard fail-fast, whole-root smoke까지 포함한 추가 live evidence 보강 |
| symlink 처리 | direct symlink-entry guard, hidden target 차단, source-root escape rejection | broader ancestor-symlink traversal hardening |
| whole-root view | `source-root=/` mount smoke, `/bin`/`/usr`/`/etc` 확인 | 상위 supervisor와의 production integration |
| chroot 실행 | `unshare -UrR` 기반 smoke 확인 | plain `chroot` 운영 모델 정리 |
| device semantics | `nodev` 제약 문서화 | `/dev/null` 등은 supervisor/namespace layer 제공 |

## 1. 설계 목표

이 절은 이 문서 전체가 만족해야 하는 상위 제품/시스템 목표를 고정한다.

`ScreenFS`는 sandbox/chroot 같은 whole-root consumer가 읽을 수 있는 non-root FUSE 기반 filesystem view layer다. `pi-bash-sandbox`는 대표 통합 시나리오지만, 이 설계는 특정 supervisor 하나에 고정되지 않는 일반화된 view layer 계약을 목표로 한다.

핵심 목표:

- non-root 사용자 권한으로 `fusermount3` 기반 FUSE3 mount를 생성한다.
- `fractal-fuse = 0.4.0` 기반으로 구현한다.
- v1은 `FUSE_OVER_IO_URING` 사용을 필수로 하며, 협상 실패 시 fallback 없이 명시적 오류로 fail-fast 한다.
- 전체 `/` view를 상위 whole-root consumer(대표 예시: chroot root)에게 제공한다.
- 기본은 underlying filesystem pass-through이며, hide rule에 걸린 경로만 없는 것처럼 숨긴다.
- hidden path는 가능한 한 `ENOENT`로 처리해 존재를 노출하지 않는다.
- hide와 별개인 selective readonly rule에 매칭된 visible mutation은 `EROFS`로 거부한다. current canonical CLI/config syntax는 16절에 정의한다.
- metadata-heavy workload를 고려해 `readdirplus`, matcher/cache, handle lifecycle을 설계한다.


구현 파일 바로가기: `README.md`, `docs/requirements.md`, `docs/architecture.md`

## 2. Threat model, integration assumptions, non-goals

이 절은 구현 범위를 넘는 책임을 미리 잘라서 설계 오해를 줄인다.

`ScreenFS`는 path-hiding filesystem view layer다. 단독 sandbox 또는 완전한 host isolation boundary가 아니다.

통합 전제:

- `ScreenFS` mount 생성은 non-root 사용자 권한으로 가능해야 한다.
- `chroot` 실행은 별도 권한 모델이 필요할 수 있다. `ScreenFS`는 `CAP_SYS_CHROOT`, privileged supervisor, user namespace 구성, 또는 동일 host uid 실행 정책을 직접 제공하지 않는다.
- 상위 supervisor/namespace layer(예: `pi-bash-sandbox`)는 `ScreenFS`가 제공한 whole-root view를 어떤 sandbox/chroot consumer에 연결할지 결정한다.
- chroot 내부 프로세스가 FUSE mount owner와 동일 host uid로 접근하는 것이 기본 전제다.
- 다른 host uid가 접근해야 하면 `allow_other`가 필요할 수 있으며, 이는 `/etc/fuse.conf`의 `user_allow_other` 및 mountpoint 권한 정책에 의존한다.
- project/sandbox 상위 레이어는 process, network, namespace, cgroup, seccomp 같은 isolation을 별도로 담당한다.

Non-goals:

- `/dev/null` bind overlay, tmpfs overlay, privileged bind mount 방식 masking은 사용하지 않는다.
- device node semantics, setuid behavior, procfs/sysfs caller-relative semantics를 `ScreenFS` 단독으로 완전히 재현하지 않는다.
- v1 hide guarantee는 virtual path 기반이다. hardlink alias는 별도 hide rule 없이는 자동 차단하지 않는다. symlink는 **목표 계약상** entry path와 resolved virtual target을 모두 검사해 hidden target으로의 symlink traversal을 `ENOENT`로 차단한다. 현재 구현에서 직접 확인된 범위는 status note와 `docs/architecture.md`에 정리된 direct symlink-entry guard다.


구현 파일 바로가기: `AGENTS.md`, `docs/requirements.md`, `docs/operations.md`

## 3. Filesystem view model

이 절은 사용자가 실제로 보게 될 virtual filesystem의 기본 모양을 정의한다.

요약:

- 입력: `source_root`, `mount_root`
- 출력: whole-root consumer(sandbox/chroot 등)가 읽는 virtual root
- 기본 정책: 전체 view pass-through + selective hiding

`ScreenFS`는 source root와 mount root를 받는다. chroot 사용을 위해 기본 source root는 `/`를 상정한다.

```text
source root: /
mount root:  /tmp/screenfs-root
chroot root: /tmp/screenfs-root
```

mount root 아래의 virtual path는 source root 아래의 같은 상대 경로로 해석한다.

```text
/tmp/screenfs-root/usr/bin/bash -> /usr/bin/bash
/tmp/screenfs-root/etc          -> /etc
```

hide rule에 매칭되지 않는 visible path는 underlying filesystem으로 pass-through 한다.


구현 파일 바로가기: `src/main.rs`, `src/config.rs`, `src/path.rs`

## 4. Mount-root recursion exclusion

이 절은 whole-root mount에서 빠지기 쉬운 자기참조 문제를 방지하는 핵심 안전장치다.

`source root = /`이고 `mount root = /tmp/screenfs-root`이면 mount root 자체가 source tree 안에 포함된다. 이 경로를 view 내부에 노출하면 다음과 같은 자기참조가 생긴다.

```text
/tmp/screenfs-root/tmp/screenfs-root/tmp/screenfs-root/...
```

따라서 mount root의 source-relative subtree는 내부 자동 hide rule로 취급한다.

필수 정책:

- mount root subtree는 `readdir`/`readdirplus` 결과에서 제외한다.
- mount root subtree에 대한 `lookup`, `getattr`, `statx`, `open`, `opendir`, `access`, `readlink`, xattr 조회는 `ENOENT`다.
- 사용자 지정 hide rule보다 낮은 우선순위가 아니라 항상 적용되는 internal rule이다.
- 검증 시 mount 후 mounted view 내부에서 mount root entry가 다시 나타나지 않아야 한다.


구현 파일 바로가기: `src/config.rs`, `src/matcher.rs`, `src/fs.rs`

## 5. Architecture overview and module boundaries

이 절은 실제 코드 탐색 순서와 책임 분리를 한 번에 잡기 위한 지도다.

이 절은 실제 코드 모듈을 읽을 때의 지도 역할을 한다.

- 진입점: `src/main.rs`
- 정책 조립: `src/cli.rs`, `src/config.rs`, `src/matcher.rs`
- 경로 의미론: `src/path.rs`
- errno / guard: `src/errors.rs`
- FUSE 요청 처리 중심: `src/fs.rs`, `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`

현재 모듈 구조:

```text
src/main.rs         CLI entry, config loading, mount lifecycle
src/cli.rs          argument parsing and validation
src/config.rs       runtime config, typed mutability resolution, MatcherScope-based PathRuleMatcher compilation
src/path.rs         virtual path normalization and source-root resolution
src/matcher.rs      PathRuleMatcher, MatcherScope, exact/prefix/limited-glob rule compilation
src/errors.rs       errno mapping and operation classification
src/fs.rs           ScreenFs module root, fractal-fuse Filesystem impl, submodule orchestration
src/fs/state.rs     inode/path table, lookup/open refcount, file/directory handle state
src/fs/guards.rs    hidden/readonly/symlink guards and request-level policy checks
src/fs/backing.rs   confined host filesystem access, open/statfs/xattr/setattr helpers
```

Data flow:

1. FUSE request arrives with inode/name or file handle.
2. Request is mapped to a virtual absolute path using inode/path state.
3. The virtual path is lexically normalized.
4. Internal mount-root exclusion and user hide `PathRuleMatcher` rules run first.
5. Hidden match returns `ENOENT` or excludes entry from listings.
6. Readonly-rule matching and mutation classification run next.
7. Visible reads/list/stat operations and visible mutations outside readonly scope are delegated through `src/fs/backing.rs`.
8. Results are converted to FUSE replies and caches are updated/invalidated by the `src/fs.rs` orchestrator plus `src/fs/state.rs`.


구현 파일 바로가기: `src/lib.rs`, `src/main.rs`, `src/fs.rs`, `src/fs/state.rs`, `src/fs/guards.rs`, `src/fs/backing.rs`

## 6. Virtual path model and normalization

이 절은 hide/readonly 판정의 기준 좌표계가 무엇인지 고정한다.

이 절은 hide rule이 **host path**가 아니라 **virtual path** 기준으로 동작한다는 점을 기억하고 읽으면 이해가 빠르다.

![ScreenFS path resolution](diagrams/path-resolution.png)

Hidden matching uses a **lexically normalized absolute virtual path**, not `std::fs::canonicalize()`.

Rules:

- The virtual root is `/` regardless of source root.
- Absolute hide rules such as `/home/<user>/.ssh` are anchored at the virtual root.
- Relative input components are joined from the parent virtual path and child name.
- `.` and duplicate `/` are removed.
- `..` pops one component but cannot escape above virtual `/`.
- Existing path, nonexistent path, broken symlink, permission-denied parent, and automount boundaries must still be classifiable without host canonicalization.
- Source path resolution must not escape source root. Prefer pre-opened source root fd plus dirfd/openat-style operations over string-only host path concatenation.

### 6.1 Rule input normalization contract

Current implementation snapshot:

- `src/path.rs`와 `src/matcher.rs`는 hide/current `--readonly-rule`용 shared normalization contract를 구현한다.
- Absolute exact rules keep the existing virtual-root-anchored semantics.
- Relative exact rules and relative prefixed-glob prefixes are interpreted from process cwd, then rebased only when the expanded host path stays inside `source_root`.
- Leading `~` / `~/...` exact or prefixed-glob inputs expand through `HOME` and follow the same containment/rebasing rules.
- The current glob parser permits recursive basename/suffix/basename-prefix tails plus normalized-prefix direct-child basename-prefix/suffix forms such as `/home/<user>/.env.*`, `~/.env.*`, `/home/<user>/*.pem`, and `~/*.pem`.
- Missing `HOME`, expanded paths outside `source_root`, `~user`, wildcard-in-prefix forms, and broader unsupported wildcard forms still fail fast.
- Current validation evidence for this normalization includes mount-free unit-test coverage in `src/matcher.rs` (`normalizes_direct_child_suffix_glob_rules`), `src/config.rs` (`hide_and_readonly_rules_accept_direct_child_suffix_forms_through_runtime_config`, `allow_write_rules_accept_direct_child_suffix_forms_through_runtime_config`, `bare_suffix_globs_stay_unsupported_on_hide_and_mutability_surfaces`), and `src/fs.rs` (`hidden_direct_child_suffix_rules_keep_enoent_precedence_over_readonly`), plus repo-local family-aware live smoke for relative hide exact path and relative/`~` prefixed-glob inputs.

Target contract for path-like rule inputs:

1. First classify whether the token is an exact-path candidate or one of the supported limited globs.
2. Supported globs keep the recursive basename/suffix/basename-prefix tail grammar, and may additionally use either an optional normalized path prefix before a recursive tail or a normalized-prefix direct-child basename-prefix/suffix tail that is normalized before matcher compilation.
3. Already-absolute exact paths like `/home/<user>/.ssh` preserve the current virtual-root-anchored semantics.
4. Already-absolute prefixed globs like `/home/<user>/**/*.pem`, `/home/<user>/.env.*`, and `/home/<user>/*.pem` preserve absolute-prefix semantics and compile to a virtual prefix plus the corresponding recursive or direct-child tail matcher. The `*.pem` form here is direct-child only.
5. Relative exact-path candidates and relative prefixed-glob prefixes are interpreted as host paths relative to the process cwd. Only when the expanded host path stays inside `source_root` may it be rebased to a source-root-relative virtual absolute path or virtual glob prefix. This applies to both recursive prefixed globs and direct-child prefix forms such as `./fixtures/*.pem` and `./app/.env.*`.
6. Leading `~` and `~/...` exact or prefixed-glob candidates are expanded through `HOME`, then subjected to the same `source_root` containment check and rebasing. Canonical examples include `~/.env.*` and `~/*.pem`.
7. Missing `HOME`, expanded paths outside `source_root`, and `~user` forms are all fail-fast errors.
8. Broader wildcard forms such as `foo/*/bar.pem`, `**/secret?.pem`, bare suffix `*.pem`, brace expansion, env-var expansion, and command substitution stay unsupported in this contract.
9. The existing `--readonly-rule` and current `--allow-write` surfaces should reuse the same rule-input normalization contract so hide and mutability classification share the same virtual path or glob prefix before policy matching.
10. Current implementation reaches this shared normalization contract for hide and the current mutability rule surfaces across exact paths, recursive prefixed globs, and direct-child basename-prefix/suffix forms. The direct-child suffix subset is covered by matcher/runtime-config/FUSE guard tests, while already-absolute live smoke and broader unsupported wildcard live fail-fast evidence remain separate follow-up work.


구현 파일 바로가기: `src/path.rs`, `src/fs.rs`, `src/matcher.rs`, `src/cli.rs`

## 7. Hide policy and matcher

이 절은 어떤 입력 rule이 어떤 경로를 숨기게 되는지의 정책 표면을 정의한다.

Target contract 기준으로 hide rules는 exact path와 제한된 glob pattern을 지원한다. 이 target contract의 glob은 recursive tail form과 normalized-prefix direct-child basename-prefix/suffix form을 포함한다. Current implementation snapshot에서도 `PathRuleMatcher`가 absolute/relative/`~` exact rule, recursive prefixed limited glob, direct-child basename-prefix/suffix form을 shared normalization contract 아래에서 compile하며, broader unsupported wildcard는 계속 제한한다.

Examples:

```text
/home/<user>/.ssh
/home/<user>/.aws
/home/<user>/.gnupg
/home/<user>/.pi/agent/auth.json
/home/<user>/.pi/agent/mcp-oauth
**/.env
**/.env.*
~/.env.*
~/*.pem
**/*.pem
**/*.key
```

Current code exposes the hide rule compiler/runtime through `PathRuleMatcher`.

Matcher structure:

- internal prefix rules: mount-root recursion exclusion
- exact absolute rule set: single path entries after current or future exact-path normalization
- hidden directory prefix set: exact directory hides its whole subtree
- compiled glob matcher: limited basename/suffix/basename-prefix forms such as `**/.env`, `**/.env.*`, `**/*.pem`, `**/*.key`, `**/*.lock`, `/home/<user>/.env.*`, `/home/<user>/*.pem`
- optional per-path hidden result cache keyed by normalized virtual path and rule version

Symlink target cache policy:

- The per-path hidden result cache may cache only direct rule matching for the normalized virtual path.
- Hidden decisions that depend on reading and resolving a symlink target must not be cached in the direct per-path hidden cache.
- v1 avoids reverse dependency tracking for symlink targets; symlink target checks are recomputed when `readlink`, traversal, or open-through-symlink classification needs them.
- Writable `rename`, `link`, `symlink`, `unlink`, and `setattr` mutations invalidate affected parent directories, alias indexes, and path/inode cache entries.
- If a later implementation adds symlink-target caching, it must add reverse dependency invalidation before enabling that cache.

Policy:

- Hidden decision is path-based in v1.
- A hidden directory hides the directory itself and all descendants.
- In directory listing, each child virtual path is checked before returning it.
- `readdirplus` must not return metadata for entries excluded by hidden rules.
- Symlink entries are matched by their own virtual path and by a lexically resolved virtual target when the link target can be interpreted inside the view.
- `readlink` on a symlink whose own path or resolved virtual target is hidden returns `ENOENT`.
- 목표 계약상 opening or traversing through a symlink must re-apply hidden matching to the resolved virtual target before host delegation. 현재 구현/증거 범위는 status note와 `docs/architecture.md`의 current-state 설명을 따른다.
- Broken symlinks whose own path is visible can be listed/readlinked if the target cannot be resolved to a hidden virtual path.
- Hardlink aliases remain visible unless their own virtual path matches a hide rule. This limitation is documented behavior; users must hide every sensitive hardlink path or avoid exposing hardlink aliases.


구현 파일 바로가기: `src/matcher.rs`, `src/config.rs`, `src/fs.rs`

## 8. Inode, path, and handle model

이 절은 path 기반 정책을 inode 기반 FUSE 요청에 어떻게 연결할지 설명한다.

FUSE is inode-centric, while hide rules are path-centric. `ScreenFS` maintains both views.

Inode table:

- FUSE root inode is fixed to `FUSE_ROOT_ID = 1`.
- Visible host objects receive synthetic FUSE inode ids.
- Reverse identity can use `(st_dev, st_ino)` for stable visible objects, with a path alias index for path-based hiding decisions.
- Hidden paths are never exported into the inode table.
- `lookup` increments lookup/refcount state; `forget` and `batch_forget` decrement and allow eviction.
- Stale inode/path associations must be invalidated after successful mutation in writable mode.

File handle table:

- `open`/`opendir` allocate file or directory handles.
- `read`/`readdir`/`readdirplus` use handles for the lifetime of the request stream.
- `release`/`releasedir` close underlying fd/backing resources and remove handle table entries.
- Baseline correctness may use ordinary `open`/`pread`/directory enumeration. `fractal-fuse` passthrough `backing_id` optimization is a later phase unless lifecycle is fully specified.

Directory iteration:

- `opendir` creates a per-handle visible entry snapshot after applying hidden filters.
- `readdir`/`readdirplus` use synthetic index-based cookies into that snapshot.
- Writable mutations invalidate affected directory snapshots and path/inode caches.
- This snapshot model is the v1 correctness baseline; streaming enumeration can be optimized later.


구현 파일 바로가기: `src/fs.rs`

## 9. FUSE operation matrix

이 절은 개별 FUSE operation이 hidden/readonly와 만나면 어떤 errno를 내야 하는지 빠르게 찾기 위한 표다.

이 표는 구현자가 가장 자주 참고해야 하는 의미론 요약이다.

아래 표의 **목표 계약**은 hide와 별개인 family-aware selective readonly semantics를 기준으로 읽는다. 현재 소스는 `selective-readonly`와 `readonly-root-allowwrite` evaluator를 mount-free code/tests로 구현했다. whole-mount readonly semantics가 필요하면 `readonly-root-allowwrite` family를 empty `allow_write`와 함께 사용한다. repo-local family-aware live smoke, current whole-root/chroot family-aware smoke, archival pre-removal transcript는 서로 다른 artifact로 관리한다.

### 9.1 조회 / 탐색 연산

| Operation class | Hidden path behavior | Visible path matched by readonly rule | Visible path not matched by readonly rule |
| --- | --- | --- | --- |
| `lookup`, `getattr`, `statx`, `access` without write mask | `ENOENT` | pass-through | pass-through |
| `access` with `W_OK` | `ENOENT` | `EROFS` | pass-through if allowed by host |
| `opendir` read-only intent | `ENOENT` | pass-through handle | pass-through handle |
| `opendir` write intent | `ENOENT` | `EROFS` | pass-through handle if host and flags allow |
| `readdir`, `readdirplus` | exclude entries | filtered listing | filtered listing |
| `open` read-only intent | `ENOENT` | pass-through handle | pass-through handle |
| `read`, `flush`, `fsync`, `release` | hidden handles must not exist | pass-through/close | pass-through/close |
| `readlink` | `ENOENT` | pass-through target | pass-through target |
| `statfs` | not path-hidden except mount-root policy | pass-through or synthesized source stats | pass-through or synthesized source stats |

### 9.2 쓰기 / 생성 / 삭제 연산

| Operation class | Hidden path behavior | Visible path matched by readonly rule | Visible path not matched by readonly rule |
| --- | --- | --- | --- |
| `open` write intent (`O_WRONLY`, `O_RDWR`, `O_TRUNC`, create-like flags) | `ENOENT` | `EROFS` | pass-through if allowed by host |
| `write` | `ENOENT` if handle should not exist because path is hidden | `EROFS` | pass-through if allowed by host |
| `create`, `mkdir`, `mknod` | `ENOENT` | `EROFS` if the created path or mutated parent matches a readonly rule | pass-through if allowed by host |
| `unlink`, `rmdir` | `ENOENT` | `EROFS` if the removed path or mutated parent matches a readonly rule | pass-through if allowed by host |
| `fallocate` | `ENOENT` | `EROFS` | pass-through if allowed by host |

### 9.3 메타데이터 / xattr 연산

| Operation class | Hidden path behavior | Visible path matched by readonly rule | Visible path not matched by readonly rule |
| --- | --- | --- | --- |
| `getxattr`, `listxattr` | `ENOENT` | pass-through or conservative unsupported error | pass-through or conservative unsupported error |
| `setxattr`, `removexattr` | `ENOENT` | `EROFS` | pass-through if allowed by host |
| `setattr` mutation (`chmod`, `chown`, `truncate`, `utimens`) | `ENOENT` | `EROFS` | pass-through if allowed by host |

### 9.4 멀티패스 연산

| Operation class | Hidden path behavior | Visible path matched by readonly rule | Visible path not matched by readonly rule |
| --- | --- | --- | --- |
| `rename` | `ENOENT` for any hidden source/target/parent involved | `EROFS` if any mutated visible source/target/parent matches a readonly rule | pass-through if allowed by host |
| `link`, `symlink` | `ENOENT` for any hidden source/target/parent involved | `EROFS` if any mutated visible source/target/parent matches a readonly rule | pass-through if allowed by host |
| `copy_file_range` with destination mutation | `ENOENT` for any hidden source/target/parent involved | `EROFS` if any mutated visible destination/parent matches a readonly rule | pass-through if allowed by host |

멀티패스 보충 규칙:

- `rename`, `link`, `symlink`, and copy-like mutation operations classify every involved source, target, and parent virtual path against hide rules and readonly rules.
- If any involved path is hidden, return `ENOENT` to avoid exposing hidden existence.
- If no involved path is hidden and any mutated visible path/parent is matched by a readonly rule, return `EROFS`.
- Current implementation snapshot: `readonly-root-allowwrite` with empty `allow_write` applies the same `EROFS` result to all visible mutations regardless of per-path match.
- Otherwise delegate to the underlying filesystem and preserve host errno where possible.


구현 파일 바로가기: `src/fs.rs`, `src/errors.rs`

## 10. Readonly implementation strategy

이 절은 readonly 판정이 요청 흐름 안에서 언제, 어떻게 적용되는지 설명한다.

핵심 질문: "이 요청이 hidden인가, readonly rule에 매칭된 visible mutation인가, 아니면 허용되는 visible access인가?"

**목표 계약**

1. Build normalized virtual path(s).
2. Apply mount-root exclusion and hide matcher.
3. If hidden, return `ENOENT` regardless of readonly rule state.
4. Determine every mutated path, parent, source, and target the operation can affect.
5. Apply selective readonly rules to those visible paths.
6. If any affected visible path is readonly-matched and the operation mutates state or opens with write intent, return `EROFS`.
7. Delegate visible read/stat/list operations and visible mutations outside readonly scope to the host filesystem.

**현재 목표 계약과 live evidence를 구분해서 읽을 점**

- 현재 목표 계약은 writable-by-default + path-scoped selective readonly deny rules다.
- `readonly-root-allowwrite`는 현재 소스/마운트-프리 테스트와 fresh repo-local/whole-root FUSE smoke transcript에 반영된 alternate family다. pre-removal historical transcript는 archival evidence로만 분리해 읽는다.
- 따라서 현재 문서의 selective readonly 표와 checklist를 family별 live smoke 완료로 확장 해석하면 안 된다.

### 10.1 Current family-aware implementation hotspots

현재 소스는 `readonly-root-allowwrite`를 단순 bool 뒤집기가 아니라 family-aware evaluator로 구현한다. 주요 구현 지점은 다음과 같다.

- `src/config.rs`
  - policy family selection, YAML `mutability` loading, CLI-over-config precedence, default family resolution
  - typed `MutabilitySurface`/`MutabilitySource` helper를 통해 surface conflict를 정리하고 `MatcherScope`별 `PathRuleMatcher`를 compile
- `src/fs.rs` + `src/fs/guards.rs`
  - `guard_mutation_path()`/`guard_multi_path_mutation()`가 source/target/parent 및 symlink resolution을 포함한 affected-coordinate evaluation 수행
  - hidden 우선 `ENOENT`, 그 다음 family별 `EROFS` 판정
- `src/errors.rs`
  - hidden/read-only/write-intent 분류 helper로 errno precedence 유지
- `src/cli.rs`
  - `--policy-family`, `--readonly-rule`, `--allow-write`, `--config` 파싱 및 family mismatch/conflict fail-fast
- `src/path.rs` / `src/matcher.rs`
  - hide/readonly/allow-write가 exact path + supported prefixed glob + fail-fast semantics를 공유

남은 설계 과제는 family surface 자체가 아니라 family별 live FUSE smoke 보강, broader ancestor-symlink hardening, production integration evidence 정리다.

### 10.2 Current carve-out family semantics in source

이 문서가 carve-out family에 대해 확정하는 자연스러운 적용 규칙은 다음과 같다.

- one-family-per-mount: 한 mount는 정확히 하나의 mutability policy family만 선택한다.
- allowWrite union semantics: allowWrite rule들은 합집합으로 평가하며, 하나라도 매치되면 해당 coordinate는 writable 후보다.
- hidden precedence: hidden path나 hidden target이 하나라도 관여하면 allowWrite보다 hidden `ENOENT`가 우선한다.
- host gate remains final: ScreenFS policy가 쓰기를 허용해도 최종 성공 여부는 host filesystem 권한/소유권/LSM이 결정한다.
- shared normalization contract: allowWrite도 hide/current readonly와 같은 exact path + supported prefixed glob + fail-fast semantics를 재사용한다.
- whole-mount readonly semantics는 `readonly-root-allowwrite` family와 empty `allow_write`로 표현한다.
- canonical CLI/config contract는 explicit family/rule/config surface만 유지한다.

### 10.3 Current carve-out operation classification

현재 소스는 carve-out 모델을 최소한 다음 operation class로 나눠 family-aware하게 다룬다.

- read/stat/list: hidden이 아니면 pass-through
- write-intent `open`: entry path와 resolved target을 hidden 검사한 뒤, 해당 entry coordinate가 allowWrite인지와 host permission을 함께 고려
- path-only mutation: `create`, `mkdir`, `mknod`, `unlink`, `rmdir`, mutation `setattr`, `setxattr`, `removexattr`, `fallocate`
- multi-path mutation: `rename`, `link`, `symlink`, destination mutation이 있는 `copy_file_range`
- parent mutation 포함 여부: directory entry set이 바뀌는 연산은 mutated parent까지 writable이어야 한다.

연산별 affected-path 규칙:

- `create` / `mkdir` / `mknod`: target path와 parent path가 모두 writable이어야 한다.
- `unlink` / `rmdir`: removed path와 parent path가 모두 writable이어야 한다.
- write-intent `open` / `write` / mutation `setattr` / xattr mutation / `fallocate`: entry path와 필요 시 resolved target 기준으로 판정한다.
- `rename`: source path, target path, source parent, target parent가 모두 writable이어야 한다.
- `link`: source path, target path, target parent가 writable이어야 하며 hidden 검사는 source/target/parent 전체에 적용된다.
- `symlink`: target path와 target parent가 writable이어야 하며, created link 자체와 target resolution 모두 hidden 검사를 통과해야 한다.
- destination mutation이 있는 `copy_file_range`: source는 hidden/read visibility 대상이고, destination path와 destination parent는 writable이어야 한다.

원칙: mutation에 관여하는 모든 write-requiring coordinate가 현재 family 기준으로 writable이어야 허용된다. `copy_file_range`에서는 source visibility와 destination writability를 분리한다.

### 10.4 Future carve-out family의 scope boundary

다음은 현재 spec이 의도적으로 범위 밖으로 두는 항목이다.

- 제3 mutability policy family 추가
- `selective-readonly`와 `readonly-root-allowwrite`의 한 mount 내 동시 활성
- future deny-like family와의 조합
- 제거된 whole-mount readonly bool shorthand를 제3 family나 config bool로 복원하는 해석

**현재 구현 상태**

- The handler enforces family-aware readonly decisions before host delegation.
- In current code/tests, `selective-readonly`는 rule-matched visible paths만 readonly-matched로 보고, `readonly-root-allowwrite`는 allow-write non-match coordinate를 readonly-matched로 본다.
- Whole-mount readonly가 필요하면 `readonly-root-allowwrite` with empty `allow_write`가 모든 visible path를 readonly-matched로 만든다.
- Handler-level guards are the source of truth. Mount-level read-only remains disabled and is not relied on for semantics.


구현 파일 바로가기: `src/fs.rs`, `src/errors.rs`, `src/main.rs`

## 11. Errno mapping

이 절은 사용자 공간에서 어떤 실패가 어떤 errno로 보일지 일관되게 맞추기 위한 규칙이다.

Error mapping rules:

- hidden path: `ENOENT`
- hidden entry in listing: omit entry
- readonly-matched mutation: `EROFS`
- host `io::Error::raw_os_error()`: preserve raw errno
- no raw errno: `EIO`
- wrong type: preserve host `ENOTDIR`, `EISDIR`, or equivalent errno
- host permission failure on visible path: preserve `EACCES`/`EPERM`
- unsupported optional op: return the least surprising FUSE-compatible errno, and document unsupported status if visible to callers


구현 파일 바로가기: `src/errors.rs`, `src/fs.rs`

## 12. Magic filesystem and device policy

이 절은 whole-root view가 곧 native kernel filesystem 재현을 의미하지 않는다는 점을 분명히 한다.

Whole `/` view does not imply transparent kernel filesystem reproduction.

Policy:

v1 policy is fixed as follows:

| Path family | ScreenFS v1 policy | Responsible layer |
| --- | --- | --- |
| `/proc`, `/proc/self`, `/proc/thread-self` | ordinary visible path traversal only; procfs caller-relative semantics are not guaranteed | 상위 supervisor/namespace layer (예: `pi-bash-sandbox`) |
| `/sys` | ordinary visible path traversal only; kernel sysfs semantics are not guaranteed | 상위 supervisor/namespace layer (예: `pi-bash-sandbox`) |
| `/dev`, `/dev/null`, `/dev/zero`, `/dev/urandom` | device node semantics are not guaranteed because non-root FUSE mounts commonly imply `nodev` | 상위 supervisor/namespace layer (예: `pi-bash-sandbox`) |
| `/dev/fd`, `/dev/stdin`, `/dev/stdout`, `/dev/stderr` | fd-relative semantics are not guaranteed | 상위 supervisor/namespace layer (예: `pi-bash-sandbox`) |
| `/run` | ordinary visible path traversal only; runtime socket/service availability is not guaranteed | 상위 supervisor/namespace layer (예: `pi-bash-sandbox`) |

`ScreenFS` v1 therefore treats these as ordinary visible paths unless hidden by rule, while explicitly not claiming native magic filesystem/device behavior. 특정 sandbox/chroot consumer가 이 경로들에 native behavior를 요구하면, 상위 supervisor가 `ScreenFS` 바깥에서 별도 namespace/mount 구성을 제공해야 한다.


구현 파일 바로가기: `README.md`, `docs/operations.md`, `docs/requirements.md`

## 13. FUSE3, io_uring, and mount options

이 절은 mount가 성공하기 위한 런타임 전제와 옵션 판단 기준을 정리한다.

Implementation uses `fractal-fuse = 0.4.0` and FUSE3.

Startup policy:

- Detect availability of `/dev/fuse`, FUSE support, `fusermount3`, and relevant kernel/session capabilities.
- The current development environment also has a documented kernel config artifact at `/home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64` with `CONFIG_FUSE_IO_URING=y` and `CONFIG_IO_URING=y`.
- v1 requires `FUSE_OVER_IO_URING` negotiation to succeed.
- Kernel config evidence is only a prerequisite signal; live session negotiation still has to succeed at runtime.
- If io_uring transport is unavailable or session negotiation fails, v1 fails fast with an explicit startup error and does not mount.
- A non-io_uring fallback can be added only as a separately documented compatibility mode in a later version.

Mount options to evaluate:

- `default_permissions`: use only if it preserves required `access` and hidden `ENOENT` behavior.
- `allow_other`: not part of v1 default behavior. Same-host-uid access is the v1 contract. A future explicit `--allow-other` option must require `/etc/fuse.conf` `user_allow_other`, restrictive mountpoint permissions, and a security warning.
- `ro`: mount-level read-only cannot express selective readonly rules, so it cannot be the source of truth for the target contract. It is only a possible whole-mount defense in depth for `readonly-root-allowwrite` empty-carve-out deployments or a future separately documented full-readonly mode.
- `force_readdir_plus`: consider enabling if compatible with `fractal-fuse` and workload.
- passthrough/backing fd optimization: phase 2 after correctness baseline.


구현 파일 바로가기: `src/main.rs`, `Cargo.toml`, `docs/operations.md`

## 14. Cache and memory model

이 절은 성능 최적화가 correctness와 메모리 상한을 깨지 않도록 제약을 건다.

Caches must improve metadata-heavy workload without unbounded memory growth.

Recommended caches:

- path hidden result cache: normalized virtual path -> hidden decision, keyed by rule version
- inode table: FUSE inode -> host identity/path state/refcount
- reverse map: host `(st_dev, st_ino)` -> FUSE inode for visible objects
- directory snapshot cache: per open directory handle or short-lived filtered listing
- file/dir handle table: bounded by open handle lifecycle

Policy:

- Set explicit cache size limits or LRU eviction.
- Use conservative `entry_timeout`, `attr_timeout`, and `negative_timeout` until correctness is proven.
- In writable mode, successful mutation invalidates affected parent directory, involved path entries, and related inode/path cache entries.
- For paths covered by readonly rules, longer TTLs can be considered but must not hide external host changes beyond documented limits.
- Track long-running RSS and fd counts during performance tests.


구현 파일 바로가기: `src/fs.rs`, `docs/operations.md`

## 15. Performance model

이 절은 v1이 느려지지 않았다고 말할 수 있는 최소 기준을 제시한다.

전체 `/` view는 `find`, shell startup, dynamic linker lookup 등 metadata-heavy workload가 많다.

Reference acceptance targets for v1 on the documented development environment:

- Repeated hidden matcher unit benchmark with 10,000 normalized paths and 100 hide rules completes within 2x the same loop with no hide rules plus 200 ms.
- FUSE mount idle RSS after startup remains below 128 MiB.
- After five repeated traversals of a 10,000-entry fixture tree, RSS growth remains below 64 MiB from post-startup baseline.
- After five repeated traversals, open fd count returns to within 16 fds of post-startup baseline.
- `readdirplus` of a 10,000-entry fixture directory completes without unbounded memory growth; the per-directory snapshot is released by `releasedir`.
- If the reference environment changes materially, new thresholds must be recorded with kernel, CPU, memory, and hide rule count.

Implementation priorities:

- implement `readdirplus`
- avoid per-entry host canonicalization
- compile hide rules once at startup
- use exact/prefix/glob matcher tiers
- maintain bounded inode/path caches
- reuse open handles during their lifetime
- keep visible file data path close to underlying filesystem; zero-copy/pass-through optimization is phase 2
- avoid unbounded directory snapshot retention


구현 파일 바로가기: `docs/operations.md`, `docs/requirements.md`

## 16. CLI shape

이 절은 사용자에게 노출되는 인터페이스를 문서와 구현 사이에서 일치시키기 위한 기준이다.

목표 계약에서 mutability policy는 hide와 별도의 surface여야 하며, current canonical family contract는 아래처럼 정리된다. family-preserving nested override를 검토하는 future option B proposal과 candidate canonical wording은 `docs/nested-mutability-option-b.md`에 따로 정리하고, 이 절의 current canonical contract와는 분리해서 읽는다.

Current canonical CLI shape:

```text
screenfs <source-root> <mount-root> \
  [--config <path>] \
  [--hide <rule> ...] \
  [--policy-family selective-readonly|readonly-root-allowwrite] \
  [--readonly-rule <rule> ...] \
  [--allow-write <rule> ...]
```

Current CLI contract:

- mount당 하나의 mutability policy family만 선택한다.
- `--readonly-rule`는 `selective-readonly` family 전용이다.
- `--allow-write`는 `readonly-root-allowwrite` family 전용이다.
- canonical contract는 explicit family/rule surface만 사용한다. whole-mount readonly shorthand는 제공하지 않는다.
- family inference rules:
  - CLI mutability option이 하나라도 있으면 CLI가 family를 결정한다.
  - explicit `--policy-family`가 있으면 그 값을 사용한다.
  - explicit family가 없고 `--allow-write`가 있으면 family=`readonly-root-allowwrite`로 본다.
  - explicit family가 없고 `--readonly-rule`가 있으면 family=`selective-readonly`로 본다.
  - CLI mutability option이 없고 config `mutability.family`가 있으면 그 값을 사용한다.
  - CLI mutability option도 없고 config `mutability.family`도 없으면 family=`selective-readonly`가 기본값이다.
- conflict/fail-fast rules:
  - `--readonly-rule`와 `--allow-write` 동시 사용 금지
  - `--policy-family selective-readonly`와 `--allow-write` 조합 금지
  - `--policy-family readonly-root-allowwrite`와 `--readonly-rule` 조합 금지
  - explicit family와 그 family 전용이 아닌 mutability rule 조합은 fail-fast다.
- exact path inputs now accept absolute paths plus relative paths and leading `~` / `~/...` under the documented source-root rebasing contract.
- current implementation/source-test evidence covers recursive basename/suffix/basename-prefix tails plus normalized-prefix direct-child basename-prefix/suffix forms such as `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, `/home/<user>/**/*.pem`, `/home/<user>/.env.*`, `~/.env.*`, `./fixtures/*.pem`, `~/*.pem`, and `/home/<user>/*.pem`.
- unprefixed bare suffix `*.pem` remains unsupported even after the direct-child suffix expansion.
- missing `HOME`, expanded paths outside `source_root`, and `~user` forms must remain fail-fast errors.
- broader wildcard forms outside that recursive-tail + direct-child-prefix contract remain unsupported.
- the existing `--readonly-rule` and current `--allow-write` parsing/matcher surfaces reuse the same rule-input normalization contract as hide rules, and the implemented direct-child suffix subset keeps that shared contract plus hidden-before-`EROFS` precedence.
- current implementation/source-test evidence for the direct-child suffix subset specifically includes `normalizes_direct_child_suffix_glob_rules`, the runtime-config shared-surface tests, `bare_suffix_globs_stay_unsupported_on_hide_and_mutability_surfaces`, and `hidden_direct_child_suffix_rules_keep_enoent_precedence_over_readonly`. live smoke evidence is still split between repo-local family-aware transcript, current whole-root/chroot family-aware transcript, and pre-removal archival transcript.

Current config contract:

```yaml
mutability:
  family: selective-readonly | readonly-root-allowwrite
  readonly_rules: []
  allow_write: []
```

- `family=selective-readonly`이면 `allow_write`는 비어 있어야 한다.
- `family=readonly-root-allowwrite`이면 `readonly_rules`는 비어 있어야 한다.
- config schema는 legacy `readonly: true|false` bool을 두지 않는다.
- CLI mutability options가 하나라도 있으면 config의 mutability block 전체를 대체한다.
- CLI mutability option이 없으면 config `mutability` block이 canonical source of truth다.
- duplicate rule은 허용하지만 semantics는 idempotent다.

Current implementation snapshot:

```text
screenfs <source-root> <mount-root> [--config <path>] [--hide <pattern> ...] [--policy-family <selective-readonly|readonly-root-allowwrite>] [--readonly-rule <rule> ...] [--allow-write <rule> ...]
```

Current example:

```bash
screenfs / /tmp/screenfs-root \
  --hide /home/spi-ca/.ssh \
  --hide /home/spi-ca/.aws \
  --hide /home/spi-ca/.pi/agent/auth.json \
  --hide /home/spi-ca/.pi/agent/mcp-oauth \
  --hide '**/.env' \
  --hide '**/.env.*' \
  --hide '**/*.pem' \
  --hide '**/*.key'
```

Current whole-mount readonly example:

```bash
screenfs / /tmp/screenfs-root \
  --policy-family readonly-root-allowwrite \
  --hide /home/spi-ca/.ssh
```


구현 파일 바로가기: `src/cli.rs`, `src/main.rs`, `README.md`

## 17. Validation strategy

이 절은 어떤 근거가 있으면 "동작한다"고 말할 수 있는지 검증 층위를 나눈다.

Validation is split into unit, integration, mount smoke, and system smoke levels. Readonly-related validation must be read in two tracks: the **target selective readonly contract** and the **whole-mount readonly coverage expressed through `readonly-root-allowwrite`**.

### Unit tests

- lexical virtual path normalization
- source-root escape prevention
- current implementation coverage: exact hide rule matching with virtual-root-anchored absolute paths
- current implementation coverage: limited glob hide rule matching, recursive absolute/relative/`~` prefixed glob normalization, normalized-prefix direct-child basename-prefix form, and rejection of broader unsupported wildcard inputs
- current implementation coverage: relative exact path rebasing from process cwd into a virtual absolute path when inside `source_root`
- current implementation coverage: relative prefixed-glob rebasing from process cwd into a virtual glob prefix when inside `source_root`
- current implementation coverage: leading `~` / `~/...` expansion through `HOME` for exact, recursive prefixed-glob, and direct-child basename-prefix/suffix inputs plus `source_root` containment checks.
- current implementation coverage: hide/current `--readonly-rule` shared normalization semantics and fail-fast propagation through `RuntimeConfig`
- target contract: fail-fast errors for missing `HOME`, `~user`, and expanded paths outside `source_root`
- hidden directory prefix matching
- mount-root recursion exclusion
- selective readonly rule matching for path/pattern-scoped rules (target contract)
- affected-path mutation classification, including write-intent `open` and multi-path operations (target contract)
- current implementation coverage: `readonly-root-allowwrite` empty-carve-out operation classifier, including write-intent `open`
- errno mapping
- inode table lookup/forget/refcount behavior
- readdir snapshot cookie behavior

### Integration tests without mount

- tempdir-backed host fixture
- visible lookup/getattr/open/read/list behavior
- hidden lookup/getattr/open/access/readlink/xattr behavior
- filtered `readdir` and `readdirplus`
- target rule-input normalization contract: relative cwd-based rebasing and `HOME`-based rebasing for exact and supported prefixed-glob inputs
- target rule-input normalization fail-fast cases: missing `HOME`, `~user`, expanded path outside `source_root`, wildcard-in-prefix forms, bare suffix `*.pem`, brace expansion, env-var expansion, command substitution, and broader unsupported wildcard forms such as `foo/*/bar.pem` and `**/secret?.pem`
- selective readonly rule application to matching and non-matching visible paths (target contract)
- multi-path op classification for hidden source/target/parent and readonly-matched source/target/parent (target contract)
- current implementation coverage: whole-mount readonly `EROFS` precedence after hidden `ENOENT`
- cache invalidation after writable mutation
- symlink target hidden checks are not served from stale direct-path hidden cache
- alias index updates after `link`, `rename`, and `unlink`

### FUSE mount smoke tests

- `ls`, `find`, `rg` do not list hidden entries
- `stat`, `cat`, `access` on hidden path fail with `ENOENT`
- hidden directory and descendant paths are hidden
- mount root subtree is not visible through the mounted view
- `/bin`, `/usr`, `/lib`, `/lib64`, `/etc`, `/home`, `/tmp`, `/var` are visible when not hidden
- `readlink` visible symlink behavior matches documented policy
- symlink whose resolved virtual target is hidden returns `ENOENT`
- hardlink alias limitation is documented and tested as path-based behavior
- smoke exact relative-path and `~` / `~/...` rule inputs separately from already-absolute inputs
- also smoke supported prefixed globs covering recursive suffix, direct-child suffix, and basename-prefix tails, such as `./fixtures/**/*.pem`, `~/fixtures/**/*.pem`, `/home/<user>/**/*.pem`, `/home/<user>/.env.*`, `~/.env.*`, `./fixtures/*.pem`, `~/*.pem`, and `/home/<user>/*.pem`
- keep unsupported/fail-fast smoke for wildcard-in-prefix forms, unprefixed bare suffix `*.pem`, brace/env/command expansion, and broader unsupported wildcard forms (for example `foo/*/bar.pem`, `**/secret?.pem`) rather than implicit shell expansion
- target contract: readonly-matched paths reject `touch`, `mkdir`, `rename`, `chmod`, `truncate`, `setxattr`, and write-intent `open` with `EROFS`, while visible non-matching paths remain writable if the host allows it
- current implementation smoke: the same mutation cases use `readonly-root-allowwrite` with empty `allow_write` when whole-mount readonly semantics are desired

### System smoke tests

- confirm `fusermount3` path and version
- confirm `/dev/fuse` and kernel FUSE support
- confirm the documented kernel config artifact still shows `CONFIG_FUSE_IO_URING=y` and `CONFIG_IO_URING=y` when using the referenced development kernel
- confirm same-uid access model or explicit `allow_other` policy
- `chroot` smoke only when the required privilege/user namespace model is available
- bash startup and dynamic linker/shared library access through mounted view
- `/proc`, `/sys`, `/dev`, `/run` policy smoke confirms ordinary path-only behavior and documents supervisor-owned native semantics
- once selective readonly CLI/config is defined, add exact path/pattern smoke cases and document current fallback coverage separately
- when exact-path normalization lands, add explicit system smoke for `HOME`-unset fail-fast, `source_root`-outside fail-fast, and `~user` unsupported errors
- large traversal with many hide rules while checking the v1 latency, RSS, and fd-count targets
- daemon shutdown and `fusermount3 -u` cleanup behavior


구현 파일 바로가기: `src/fs.rs`, `docs/operations.md`, `README.md`

## 18. Role review status

이 절은 이 문서가 어떤 리뷰 관점을 거쳐 정제되었는지 추적 가능하게 남긴다.

This design incorporates repeated parallel design-review findings from the listed review roles used when this design was written:

- `user-representative`
- `software-systems-engineer`
- `software-designer`
- `software-implementer`
- `software-qa`
- `software-reviewer`

Resolved review themes:

- mount-root recursion exclusion
- lexical path matching instead of host canonicalization
- hidden directory subtree semantics
- symlink target checks and symlink-target-dependent cache policy
- hardlink path-based limitation
- inode/path/file handle model
- readdir/readdirplus snapshot model
- expanded operation matrix
- readonly write-intent `open` handling
- errno mapping
- chroot permission and same-host-uid/`allow_other` assumptions
- magic filesystem/device policy
- FUSE3/io_uring v1 fail-fast policy
- cache bounds and invalidation
- requirement-linked validation strategy

Final parallel design-review result: all listed design-review role subagents reported no blocking findings or blockers for this design.


구현 파일 바로가기: `docs/pi-agents.md`, `.pi/agents/`, `.pi/skills/`

