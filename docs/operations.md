# 운영 및 검증

이 문서는 현재 목표 계약과 verified evidence를 구분해 적는다. 정책 surface는 `visibility.*` / `mutability.*` 두 축이며, shared matcher는 bare slashless direct-child, anchored direct-child, subtree shorthand, recursive glob family를 함께 정규화한다. 다만 current `visibility.visible` surface는 recursive bridge discovery가 필요 없는 subtree/direct-child category만 허용한다. 따라서 `**/*.pem`, `/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `~/**/aaa/hook/**`, `~/**/aaa/hook` 같은 recursive canonical/shorthand family는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서만 current일 수 있고 `visibility.visible`에서는 current contract가 아니다. recursive literal directory shorthand는 canonical trailing `/**` form과 동일 descriptor/semantics를 가져야 하지만 normalization/path-matcher-only delta여야 하며 새로운 discovery/scanning contract를 만들면 안 된다. 이는 subtree shorthand(`/dir/**`=`/dir`, `~/aa/**`=`~/aa`)와 다른 문법이다. CLI/config reference는 two-axis surface만 사용하며, recorded artifact도 current contract baseline만 유지한다.

artifact 분류 quick map:

- current policy whole-root/chroot live smoke baseline: `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`
- current default-writable live smoke baseline: `docs/artifacts/current-default-writable-smoke-transcript.md`
- current bare slashless cwd-anchor repo-local live smoke baseline: `docs/artifacts/current-bare-basename-glob-smoke-transcript.md` (`*.pem`=`./*.pem` direct-child baseline)
- current visible direct-child/subtree compatibility smoke baseline: `docs/artifacts/current-compatibility-pattern-smoke-transcript.md` (`/dir/*`, `./dir/*`, `~/dir/*`, `/dir/**`, `./dir/**`, `~/dir/**`, and `*`/`**/*` fail-fast)
- current FUSE transport contract source evidence: `docs/artifacts/current-fuse-transport-contract-evidence.md` (`fractal-fuse = 0.4.0` `FUSE_OVER_IO_URING` negotiation fail-fast source evidence plus ScreenFS no-fallback mount path evidence)
- current file-data-path async/io_uring feasibility evidence: `docs/artifacts/current-file-data-path-async-feasibility.md` (already-open file handle data-path async/io_uring candidate scope plus separate `flush`/`fsync`/`release(flush)` sync cleanup boundary)
- current state-lock concurrency evidence: `docs/artifacts/current-state-lock-concurrency-evidence.md` (single consistency-domain `RwLock<State>` rationale, lock rules, `flush`/`fsync`/`release(flush)` state-lock-outside-sync-syscall evidence, and validation expectations)

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

위 kernel config artifact는 현재 커널 빌드 설정에 `FUSE_OVER_IO_URING` 관련 옵션이 활성화되어 있음을 보여주는 정적 근거다. 이는 mount 성공이나 실제 session 협상 성공을 새로 증명하는 smoke artifact는 아니다. current v1 async scope는 FUSE request/reply transport 협상에 한정되며, backing filesystem metadata/data path의 host syscall/openat2-confined delegation을 wholesale `io_uring`로 바꾸는 것을 요구하지 않는다. 이미 열린 file handle의 data path를 선택적으로 바꾸려면 `read`/`write`/`copy_file_range`/`fallocate`별 baseline benchmark, dependency/API support, and policy-preservation evidence를 별도로 남긴다. 반면 `flush`/`fsync`/`release(flush)`의 runtime blocking-offload는 이미 열린 file handle의 blocking sync syscall 실행 위치를 state lock 밖 blocking pool로 옮기는 별도 low-risk concurrency cleanup으로 기록한다.

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
- latest recorded full-suite `cargo test --all-targets --all-features` 결과는 142 library tests + 1 binary test 통과이며, visibility/mutability axis regressions, bare slashless cwd-anchor/direct-child regressions, shared recursive-family coverage, direct-child/subtree compatibility coverage, recursive literal directory shorthand coverage, recursive `visibility.visible` rejection, matcher bucket/index module split, symlink final-target visibility safety, fd-based xattr/setattr hardening, ScreenFS mount option policy regression, state-lock concurrency regressions, `flush`/`fsync`/`release(flush)` executor-offload regressions, create/open target revalidation regressions, TOCTOU hardening regressions를 포함한다.
- current mount-free/source coverage는 최소한 다음을 포함한다.
  - `visibility.hidden`: hidden entry filtering, direct read/stat/access/open `ENOENT`, single-hop/multi-hop/ancestor symlink hidden target `ENOENT`
  - `visibility.visible`: default-hidden carve-out, subtree/direct-child bridge-visible ancestor, hidden sibling 비노출, recursive visible glob rejection
  - visibility fast-path guardrail: hide 불가 증명이 있을 때만 resolved-target recheck를 생략하고 prior listing success·cross-request direct-path memo·symlink decision cache를 면제 근거로 쓰지 않는지 확인
  - directory-entry filtering fast path: `readdir`/`readdirplus`가 현재 directory/parent 기준 관련 matcher bucket만 보고 unrelated bucket을 건너뛰어도 결과가 바뀌지 않는지 확인
  - `mutability.default=writable`: rule-match mutation `EROFS`, non-match visible mutation 허용, fast allow가 visibility proof를 대체하지 않는지 확인
  - `mutability.default=readonly`: writable carve-out success, carve-out 밖 visible mutation `EROFS`, fast `EROFS`가 hidden-before-`EROFS`를 뒤집지 않는지 확인
  - `mutability.readonly` re-block: 더 구체적인 readonly override precedence
  - hidden-before-mutability: hidden path/target이 mutation 좌표에 섞이면 mutability 판정보다 먼저 `ENOENT`; symlink-resolved hidden target도 같은 precedence를 유지
  - affected-coordinate-wide writable requirement, `copy_file_range` source visibility / destination writability 분리
  - bare slashless cwd-anchor/direct-child delta: `cwd` rebasing, cwd-outside-`source_root` fail-fast, `*.pem`=`./*.pem` equivalence
  - shared recursive-family coverage: `**/*.pem`=`./**/*.pem` same-anchor equivalence, `**/*.pem` vs `/**/*.pem` explicit root-anchor distinction, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`
  - goal 612c03c6 compatibility patterns: `/dir/*`, `./dir/*`, `~/dir/*` anchored direct-child wildcard-all, `/dir/**`, `./dir/**`, `~/dir/**` same-anchor subtree shorthand, unanchored `*`/`**/*` fail-fast
  - current visible rejection set: `visibility.visible`에서 `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, cwd/HOME-relative recursive descendant canonical/shorthand form이 fail-fast 되는지 확인
  - unsupported glob fail-fast, same-specificity conflict fail-fast, config load/axis override
  - xattr/setattr hardening: xattr and setattr paths use confined fd-based operations after policy checks and preserve hidden symlink `ENOENT`
- state-lock concurrency changes are tracked in `docs/artifacts/current-state-lock-concurrency-evidence.md`; current source inspection evidence includes read/write lock snapshot paths and `flush`/`fsync`/`release(flush)` state-lock-outside-sync-syscall paths, while source test coverage includes readdirplus atomic lookup-ref pinning, lseek handle lifecycle, offset-based `FileExt::read_at`/`write_at` behavior, focused sync-surface executor-offload tests, and focused state-cache concurrency regression tests.
- Path-based TOCTOU hardening is tracked in `docs/artifacts/current-toctou-hardening-evidence.md`; current direction is to pin `source_root` and parent/opened objects with `open_confined()` and use dirfd/`*at` syscalls for metadata, listing, readlink, and mutation surfaces instead of reusing raceable host path strings after policy checks. Mutation paths also best-effort revalidate that opened parent dirfds still map to the requested virtual parent before the final fd-relative syscall, and write-intent open/create defers `O_TRUNC` until after opened-target validation. Remaining external same-UID rename/unlink after that validation follows Linux/POSIX fd lifetime semantics rather than current-virtual-path atomicity.
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

  bunx @mermaid-js/mermaid-cli \
    -i "$input" \
    -o "${base}.svg" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json

  bunx @mermaid-js/mermaid-cli \
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

`docs/artifacts/current-whole-root-chroot-smoke-transcript.md`는 bare slashless shorthand나 prefixless recursive shorthand에 의존하지 않는 `source_root=/` whole-root/chroot baseline이다.

현재 bare slashless glob 상태:

- `current-bare-basename-glob-smoke-transcript.md`는 cwd-anchored `./<pattern>` direct-child semantics의 current repo-local live baseline이다.
- artifact는 `visibility.hidden`/`mutability.readonly`, `visibility.visible`/`mutability.writable`, cwd-outside-`source_root` fail-fast, unsupported bare wildcard fail-fast, `*.pem` vs `./*.pem` same-normalized-specificity conflict를 캡처한다.
- same-anchor recursive shorthand(`**/*.pem`=`./**/*.pem`)와 explicit root-anchor `/**/*.pem` distinction은 source tests가 기록한다.

현재 shared recursive-family 상태:

- target contract의 canonical 4-family 비교는 `**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`다.
- Source tests는 current canonical set(`**/*.pem`, `/**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`)의 prefixless recursive cwd-anchor와 explicit root-anchor distinction을 직접 검증한다.
- transcript에 포함된 `startup_ms`, `VmRSS`, `fd_count`는 shared matcher family가 어떤 anchor를 쓰는지 보여주는 smoke evidence이며 formal benchmark는 아니다.

현재 visible direct-child/subtree 상태:

- target contract는 `/dir/*`, `./dir/*`, `~/dir/*`를 anchored direct-child wildcard-all family로, `/dir/**`, `./dir/**`, `~/dir/**`를 same-anchor subtree shorthand로 정의한다.
- source tests는 matcher/config/fs 계층에서 absolute/relative/home anchored forms, subtree shorthand equivalence, unsupported `*`/`**/*` fail-fast를 검증해야 한다.
- `current-compatibility-pattern-smoke-transcript.md`는 repo-local live session에서 `/dir/*`, `./rel/*`, `~/homeglob/*`, `/dir/**`, `./rel/**`, `~/homesub/**`, and `*`/`**/*` fail-fast를 캡처한다.
- `visibility.visible` recursive descendant glob rejection(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, cwd/HOME-relative recursive canonical/shorthand form)은 current contract의 필수 검증 항목이며, compatibility baseline과 별도로 다뤄야 한다.

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
- `FUSE_OVER_IO_URING` transport 계약을 증거로 남길 때는 `fractal-fuse = 0.4.0`의 session negotiation이 unsupported transport에서 startup error를 반환한다는 dependency-source 근거와, ScreenFS가 해당 error를 fallback 없이 노출한다는 local code/test 또는 live smoke 근거를 분리해 기록한다.
- selective file-data-path async/io_uring evidence를 남길 때는 metadata/path policy operations와 분리한다. 최소 기록 항목은 baseline workload, measurement command, input file sizes/counts, operation (`read`, `write`, `copy_file_range`, optional `fallocate`), latency/throughput/CPU or syscall evidence, dependency/API used, focused semantic tests, and explicit confirmation that lookup/getattr/readdir/readlink/xattr/setattr/rename/link/symlink/unlink/mkdir and recursive discovery were not changed.
- `flush`/`fsync`/`release(flush)` executor-offload evidence를 남길 때는 async/io_uring benchmark artifact와 분리한다. 최소 기록 항목은 operation surface, handle snapshot/removal ordering, state lock이 offload 구간 전에 해제되었는지 여부, blocking syscall(`sync_all`/`fdatasync`/`fsync`) 종류, 현재 FUSE runtime에 맞는 blocking-offload API(예: `compio_runtime::spawn_blocking`)와 필요한 직접 dependency, blocking-pool/thread-budget 및 queueing/backpressure 기대치, syscall errno 보존과 join/panic 실패의 deterministic errno 매핑, 그리고 metadata/path policy ops·cache/discovery·public API·`read`/`write` `FileExt::read_at`/`write_at` 경로가 바뀌지 않았다는 확인이다.
- 최소 기록 항목:
  - mount 명령 또는 동등한 실행 surface
  - source/mount 경로
  - `visibility.hidden` / `visibility.visible` 입력
  - `mutability.default`, `mutability.writable`, `mutability.readonly` 입력
  - `fusermount3 --version`
  - 후속 `ls`/`stat`/`cat`/mutation 명령
  - stderr/stdout 발췌
  - unmount 결과
- startup log 문자열이 바뀌면 current startup-string evidence는 해당 세션의 current-session stderr/stdout 발췌나 현재 소스 기준으로 갱신한다.
- repo-local fixture mount evidence는 normalization, `visibility.hidden`, `mutability.default` regression 확인에 유용하지만, whole-view와 chroot 관련 결론은 `source-root=/` 또는 동등한 전체 view 환경의 별도 증거로 보강해야 한다.
- `mutability.default=writable`를 주장하려면 rule-match path와 non-match path를 모두 포함한 별도 evidence가 필요하다.
- `mutability.default=readonly`를 주장하려면 writable carve-out 성공과 carve-out 밖 `EROFS`를 같은 세션에 남겨야 한다. whole-mount readonly baseline은 empty `mutability.writable`의 별도 케이스로 분리한다.
- `mutability.readonly` re-block를 주장하려면 더 넓은 `mutability.writable` 아래 더 구체적인 readonly override와 그 반대 polarity conflict/fail-fast를 함께 기록한다.
- `visibility.visible` carve-out을 주장하려면 hidden ancestor 아래 visible descendant에 도달하기 위한 bridge-visible listing/traversal 결과를 함께 기록한다.
- rule-input normalization 변화를 주장하려면 already-absolute, relative, `~` success case를 분리해 기록하고, bare slashless shorthand의 launch cwd / normalized anchor / cwd-outside-`source_root` fail-fast / `*.pem`=`./*.pem` equivalence, shared recursive family의 same-anchor `**/*.pem`=`./**/*.pem` equivalence / same-anchor `*.pem` containment / explicit root-anchor `/**/*.pem` distinction, anchored direct-child wildcard-all(`/dir/*`, `./dir/*`, `~/dir/*`)의 immediate-child-only scope와 `/dir/*.pem` containment, same-anchor subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)의 `/dir` equivalence, recursive literal directory shorthand(`**/.git`, `**/.git/hooks`, `/repo/**/.git/hooks`, `~/**/aaa/hook`, `**/node_modules`, `**/target`, `**/dist`, `**/build`)의 canonical trailing `/**` form equivalence를 별도로 남긴다. 여기서 shorthand는 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`만 허용되고 `/**/`는 최대 한 번만 쓸 수 있으며 tail component는 모두 literal이어야 한다.
- current `visibility.visible` contract를 주장하려면 recursive visible glob rejection(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, cwd/HOME-relative recursive form) stderr와 권장 대안(`/dir`, `/dir/**`)을 함께 남긴다.
- non-visible recursive literal directory shorthand contract를 주장하려면 shorthand와 canonical form이 hidden `ENOENT`, readonly `EROFS`, writable override success, same-polarity dedup, opposite-polarity conflict, containment 결과까지 동일하다는 evidence를 함께 남긴다.
- recursive literal directory shorthand를 current contract evidence로 삼으려면 source diff/trace/counter/perf smoke로 normalization/path-matcher-only임을 보여야 한다. 즉 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery가 추가되지 않았고 matcher cost도 기존 canonical `**/.../**` recursive literal subtree rule과 같아야 한다. `**/.git/hooks`, `~/**/aaa/hook` 같은 supported shorthand는 각각 canonical `**/.git/hooks/**`, `~/**/aaa/hook/**`와 같은 비용이어야 한다.
- direct-child visible rule(`/tmp/*` 또는 동등형)을 current contract evidence로 삼으려면 startup/lookup/readdir 단계에서 anchor subtree 재귀 순회가 없고 현재 directory/parent와 무관한 matcher bucket을 건너뛴다는 성능-oriented smoke, counter, trace, 또는 동등한 계측을 함께 남긴다. recursive literal directory shorthand 목표를 함께 건드렸다면 shorthand 추가가 이 경로에 새로운 recursive discovery/scanning 코드를 끼워 넣지 않았다는 근거도 같은 세션 또는 source diff로 남긴다.
- symlink-dependent visibility contract를 주장하려면 listing/lookup/getattr/readlink/dereference/open 시점 check가 prior listing success, symlink decision cache, cross-request direct-path memoized result로 대체되지 않고 single-request resolved-target reuse만 허용된다는 evidence를 함께 남긴다.
- state-lock concurrency 변경을 주장하려면 lock domain, read/write lock usage, no-upgrade rule, host-I/O-outside-lock rule, readdirplus lookup-ref pinning, mutation invalidation atomicity, and focused state-cache/concurrency tests를 함께 남긴다. Per-table lock split은 별도 design artifact 없이 current evidence로 승격하지 않는다.
- 문서상 current contract surface와 current verified evidence를 구분하고, current baseline artifact만 유지한다.
- mount 전에 실패했으면 상태는 `차단됨` 또는 `실패`로 적고, hidden/whole-view/mutability smoke를 `완료`로 승격하지 않는다.

## 운영 시나리오 번들

아래 항목은 운영 예시 번들이다. 최신 recorded verification evidence 자체로 읽지는 않는다. bare slashless glob과 prefixless recursive shorthand는 모두 cwd-sensitive이므로 whole-root 예시에는 explicit root-anchored/absolute recursive form을 우선 사용한다. `pi-bash-sandbox` + chroot는 대표 통합 시나리오지만, 동일한 whole-root mount는 다른 sandbox/chroot consumer에도 재사용될 수 있다.

- baseline hidden-only view
  - `source-root=/`
  - `visibility.hidden`: `/home/spi-ca/.ssh`, `/home/spi-ca/.aws`, `/home/spi-ca/.pi/agent/auth.json`, `/home/spi-ca/.pi/agent/mcp-oauth`, `/**/.env`, `/**/.env.*`, `/**/*.pem`, `/**/*.key`
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

Bare slashless pattern(`*.pem`, `*.key`, `.env.*`, `id_*`)을 supervisor가 그대로 전달하면 ScreenFS는 macOS sandbox-runtime matcher 전체를 복제하는 것이 아니라, 그런 supervisor가 쓰는 bare basename-oriented input shape를 보존하기 위해 launch cwd를 `source_root` 기준으로 rebase한 `./<pattern>` direct-child shorthand로 컴파일한다. recursive prefixless pattern(`**/*.pem`, `**/.env.*`, `**/id_*`)은 같은 cwd anchor의 `./**/<pattern>` recursive shorthand target이다. anchored direct-child wildcard-all(`/dir/*`, `./dir/*`, `~/dir/*`)은 normalized anchor directory의 모든 immediate child와 그 descendants를 대상으로 하지만 anchor 자체에는 적용되지 않는다. same-anchor subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)은 `/dir`, `./dir`, `~/dir`와 정확히 동등하며 `~/aa/**`=`~/aa` 의미도 그대로다. recursive literal directory shorthand는 non-visible surface에서만 지원되는 별도 문법이며 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`만 허용한다. `/**/`는 최대 한 번만 허용되고 tail component는 모두 literal이어야 하며 내부적으로 `<prefix>/**/<literal-tail>/**`로 normalize된다. 예: `**/.git`=`**/.git/**`, `**/.git/hooks`=`**/.git/hooks/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**`. 다만 `visibility.visible`은 `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, cwd/HOME-relative recursive descendant form 같은 recursive visible rule을 shorthand와 canonical form 모두 허용하지 않는다. visible carve-out이 필요하면 subtree(`/dir`, `/dir/**`)나 direct-child family만 사용해야 한다. shorthand 추가는 normalization/path-matcher-only여야 하고 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 만들면 안 된다. `~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]` 같은 multi-recursive 또는 broader form은 discovery, ambiguous containment, broader glob compatibility를 피하기 위해 fail-fast다. macOS-style cwd-sensitive policy를 의도한 경우에만 bare/prefixless form을 쓰고, whole-root secret coverage가 목적이면 `/**/*.pem`, `/**/*.key`, `/**/.env.*`, `/workspace/**/*.lock`, `**/.git/hooks`, `**/node_modules` 같은 non-visible recursive form을 생성한다. unanchored `*`와 `**/*`는 계속 fail-fast다.

The integration must emit ScreenFS YAML/CLI in the current policy model.

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
- direct-child visible rule에서는 immediate child evaluation 결과에 없는 sibling을 노출하지 않아야 한다.
- artifact에는 parent listing, descendant `stat`, descendant read, hidden sibling `ENOENT`를 한 세션에 함께 남긴다.

## directory-entry filtering fast-path checklist

`readdir`/`readdirplus` fast path를 current contract evidence로 삼으려면 다음을 함께 남긴다.

- filtering은 현재 directory/parent 기준으로만 평가되고, 그 directory와 무관한 matcher bucket은 건너뛴다는 counter/trace/perf evidence가 있어야 한다.
- unrelated bucket skip이 listing 결과를 바꾸지 않아야 한다. hidden sibling 비노출, bridge-visible next-hop only, `readdir`=`readdirplus` 의미론은 그대로 유지돼야 한다.
- recursive scan, background index, stable listing result cache를 요구하지 않는다는 점이 드러나야 한다.
- 필요하면 per-handle iteration state를 둘 수 있지만, prior listing 결과 snapshot을 다음 symlink/path 판정 면제로 재사용하면 안 된다.

## visible recursive rejection checklist

current `visibility.visible` contract는 recursive bridge discovery가 필요한 form을 거부해야 한다. 최소한 아래 예시를 별도 검증으로 남긴다.

```bash
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/dir/**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/.git/hooks'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/repo/**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/repo/**/.git/hooks'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible './repo/**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible './repo/**/.git/hooks'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible './repo/**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '~/repo/**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '~/repo/**/.git/hooks'
```

기대 결과:

- mount 전에 fail-fast 한다.
- stderr는 unsupported reason이 `visibility.visible`용 recursive bridge discovery 필요성임을 드러내야 한다.
- 권장 대안(`/dir`, `/dir/**`)이 함께 안내되면 가장 좋다.
- canonical form과 shorthand form이 모두 같은 unsupported reason으로 거부되는지 확인한다.

## non-visible recursive literal directory shorthand checklist

recursive literal directory shorthand는 canonical trailing `/**` form과 완전히 같은 의미론이어야 한다. shorthand는 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`만 허용되고 `/**/`는 최대 한 번만 쓸 수 있으며 tail component는 모두 literal이어야 한다. 최소한 아래 예시를 별도 검증으로 남긴다.

```bash
screenfs / /tmp/screenfs-root --hidden '**/.git/hooks'
screenfs / /tmp/screenfs-root --hidden '**/.git/hooks/**'
screenfs / /tmp/screenfs-root --hidden '/repo/**/.git/hooks'
screenfs / /tmp/screenfs-root --readonly '**/node_modules'
screenfs / /tmp/screenfs-root --readonly '**/node_modules/**'
screenfs / /tmp/screenfs-root --writable '**/dist'
screenfs / /tmp/screenfs-root --writable '**/dist/**'
```

기대 결과:

- `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`, `**/.git`=`**/.git/**`, `**/target`=`**/target/**`, `**/dist`=`**/dist/**`, `**/build`=`**/build/**`로 normalize되어 directory 자체와 descendants를 함께 매치한다.
- hidden surface에서는 shorthand/canonical direct access 결과가 모두 `ENOENT`이고 listing exclusion도 동일하다.
- readonly surface에서는 shorthand/canonical mutation 결과가 모두 `EROFS`다.
- writable carve-out에서는 shorthand/canonical override 결과가 모두 동일하다.
- same-polarity shorthand/canonical 조합은 dedup/idempotent이고, opposite-polarity shorthand/canonical same-specificity 조합은 동일 conflict로 fail-fast다.
- unsupported trailing `/**`-less broader/ambiguous form과 multi-recursive/broader form(`~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]`, `a*b`, `*secret*`)은 부분 해석 없이 fail-fast다.
- source diff/trace/counter/perf smoke로 shorthand 추가가 normalization/path-matcher-only이며 recursive bridge discovery, lazy discovery, startup scan, background indexing, listing 결과 cache, symlink decision cache, 기타 새로운 filesystem discovery를 추가하지 않았음을 보여야 한다.
- matcher cost는 기존 canonical `**/.../**` recursive literal subtree rule과 동일해야 한다.

## direct-child no-recursive-traversal checklist

`/tmp/*` 같은 direct-child visible rule은 normalized anchor의 immediate child만 평가해야 한다. 아래와 같은 성능-oriented smoke 또는 동등한 계측을 남긴다.

```bash
strace -f -e getdents64,newfstatat \
  -o /tmp/screenfs-visible-tmp-star.strace \
  screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/tmp/*'
```

기대 결과:

- startup 또는 첫 `readdir` 전에 `/tmp` 아래 deeper descendant를 재귀 순회하는 흔적이 없어야 한다.
- `/tmp`의 immediate child를 늘려도 deep subtree 크기에 비례한 recursive traversal이 나타나지 않아야 한다.
- 현재 directory/parent와 무관한 matcher bucket이 건너뛰어졌다는 counter/trace/perf evidence가 있어야 한다.
- 같은 결론을 다른 계측(내부 counter, perf trace, bounded startup log)으로 증명해도 된다.

## symlink point-of-use fast-path checklist

symlink target visibility fast path를 주장하려면 다음을 함께 남긴다.

- policy가 target check 생략을 증명하지 못하는 경우 listing/lookup/getattr/readlink/dereference/open 시점마다 multi-hop symlink와 ancestor symlink를 반영한 resolved final virtual target을 다시 확인해야 한다.
- prior listing success가 이후 symlink dereference 또는 `readlink`의 cached exemption이 아니어야 한다.
- cross-request direct-path-only memoized result를 symlink-dependent check에 재사용하지 않는다는 trace/log/test evidence가 있어야 한다.
- single-request 안에서만 이미 계산한 resolved final target 재사용이 허용된다는 evidence가 있으면 더 좋다.
- symlink decision cache를 current contract로 문서화하지 않는다.

## Fast-path QA matrix

### Visibility fast-path QA

| case | 확인할 점 | 기대 결과 |
| --- | --- | --- |
| directory-entry filtering | 현재 directory/parent 기준 matcher bucket만 보며 unrelated bucket을 건너뜀 | 결과 의미론 불변, hidden sibling 비노출 |
| symlink target recheck elision | compiled visibility policy가 hide 불가를 증명할 때만 생략 | 증명 실패 시 point-of-use resolved-target visibility 재검사 |
| cache guardrail | prior listing success, cross-request direct-path memo, symlink decision cache 금지 | point-of-use exemption 없음 |
| request-local reuse | 단일 FUSE request 안의 resolved final target 재사용 | 같은 request 내부에서만 허용 |

### Mutability fast-path QA

| policy default | 확인할 점 | 기대 결과 |
| --- | --- | --- |
| `mutability.default=writable` | readonly match 불가 fast allow | visibility가 fully visible을 먼저 증명한 경우에만 host 위임 |
| `mutability.default=readonly` | writable carve-out 불가 fast `EROFS` | hidden/non-fully-visible 가능성이 없을 때만 `EROFS` |
| carve-out / re-block | nested override와 affected coordinate 평가 | most-specific wins, visibility proof 대체 금지 |

### Combined fast-path QA

| case | 확인할 점 | 기대 결과 |
| --- | --- | --- |
| hidden + readonly/writable overlap | visibility block과 mutability block 조합 | hidden `ENOENT`가 항상 `EROFS`보다 우선 |
| bridge-visible mutation | visibility block에서 bridge-visible 식별 후 mutability block 진입 전 차단 | `EROFS`, hidden sibling 비노출 |
| symlinked mutation target | visibility point-of-use 재검사 후 mutability 평가 | hidden/non-fully-visible이면 `ENOENT`, fully visible일 때만 mutability 적용 |
| multi-path mutation | source/target/parent coordinate별 visibility 후 mutability | one hidden coordinate => `ENOENT`, else readonly/bridge-visible => `EROFS` |

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

- shared matcher는 `visibility.hidden`/`visibility.visible`/`mutability.writable`/`mutability.readonly`에 exact/subtree family, direct-child glob family, recursive non-visible glob family, recursive literal non-visible subtree family를 제공한다. recursive literal family는 canonical trailing `/**` form과 recursive literal directory shorthand를 함께 받되 shorthand를 canonical descriptor로 normalize한다. 허용 shorthand는 `**/<literal-dir>`와 `<prefix>/**/<literal-tail>`뿐이고 `/**/`는 최대 한 번만 허용되며 tail component는 모두 literal이다.
- current `visibility.visible` contract는 subtree(`/dir`, `/dir/**`)와 direct-child anchor bridge(`/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형)만 허용한다.
- recursive family(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, cwd/HOME-relative recursive descendant canonical/shorthand form)는 `visibility.hidden`, `mutability.readonly`, `mutability.writable`에서는 계속 current일 수 있지만 `visibility.visible`에서는 fail-fast 대상이다.
- absolute exact path와 absolute prefixed glob은 virtual-root anchored semantics를 유지한다.
- prefixless recursive glob `**/*.pem`, `**/.env.*`, `**/id_*`는 launch process cwd를 먼저 host path로 정규화한 뒤 `source_root` relative normalized prefix로 rebase한 recursive shorthand다. 따라서 `**/*.pem`은 같은 cwd anchor에서 `./**/*.pem`과 동등하고, whole-tree recursive intent는 `/**/*.pem`, `/**/.env.*`, `/**/id_*`처럼 explicit root-anchor form으로 표현한다.
- bare slashless glob은 `/` component가 없는 glob pattern이며 launch process cwd를 먼저 host path로 정규화한 뒤 `source_root` relative normalized prefix로 rebase한 `./<pattern>` direct-child shorthand다. 따라서 `*.pem`은 같은 cwd anchor에서 `./*.pem`과 동등하고, 그 anchor 바로 아래 child basename과 matched child descendants에만 적용된다.
- direct-child wildcard-all glob(`/dir/*`, `./dir/*`, `~/dir/*`)은 normalized anchor directory의 모든 immediate child와 그 descendants에만 매치된다. anchor 자체에는 적용되지 않으며, immediate child를 먼저 매치하지 않고는 deeper non-child basename에 직접 매치하지 않는다.
- same-anchor subtree shorthand(`/dir/**`, `./dir/**`, `~/dir/**`)은 `/dir`, `./dir`, `~/dir`와 정확히 동등한 subtree rule이다.
- matcher/indexing은 family와 normalized anchor를 기준으로 분리하고, same-polarity identical descriptor는 dedup/idempotent 처리해야 한다. `/dir/**`는 `/dir`와 같은 descriptor로 compile된다.
- broader unsupported wildcard forms, prefix 내부 wildcard, unanchored `**/*`, one-sided basename-prefix/suffix subset 밖의 bare wildcard form(`*`, `a*b`, `*secret*`), `HOME` 없음, outside-`source_root`, `~user`는 계속 fail-fast다. descendant-subtree broader forms(`~/**/bbb/**/ccc`, `**/.git/**/hooks`, `**/.git/*/hooks`, `**/foo?`, `**/[abc]`, shorthand subset 밖 trailing `/**`-less broader/ambiguous form)도 부분 해석 없이 fail-fast 대상으로 유지한다.

### current `visibility.visible` category table

| category | syntax 예 | current status | bridge / indexing note |
| --- | --- | --- | --- |
| static subtree bridge | `/dir`, `/dir/**`, `./dir`, `~/dir` | 지원 | 정적 ancestor chain만 bridge-visible, `/dir/**`=`/dir` descriptor dedup |
| direct-child anchor bridge | `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `*.pem`, `./dir/*`, `~/dir/*` | 지원 | normalized anchor ancestor만 bridge-visible candidate, immediate child만 평가, recursive traversal 금지 |
| recursive visible glob | `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, `./repo/**/*.pem`, `~/repo/**/.git/hooks/**`, `~/repo/**/.git/hooks` | 거부 | recursive bridge discovery 필요, `/dir` 또는 `/dir/**` 권장 |

문서화된 current contract checklist:

- relative exact path는 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual absolute path로 rebase된다.
- relative prefixed glob도 process cwd 기준 host path로 해석된 뒤 `source_root` 내부일 때만 virtual glob prefix로 rebase된다.
- `~`/`~/...` exact path와 prefixed glob은 `HOME` 기준으로 expand된 뒤 같은 rebasing 규칙을 따른다.
- implementation evidence가 검증해야 하는 family는 exact/subtree, direct-child glob, recursive non-visible glob, recursive literal non-visible subtree다.
- nested override와 specificity 검증은 normalized target set 기준으로 수행한다. `**/*.pem`는 같은 normalized cwd anchor에서 `./**/*.pem`과 같은 descriptor/specificity로 취급한다. `*.pem`는 같은 anchor에서 `./*.pem`과 같은 direct-child descriptor/specificity로 취급되며 same-anchor `**/*.pem` target set 안에 contained된다. same-anchor `/dir/**`는 `/dir`와 같은 descriptor/specificity로 취급한다. `/dir/*`는 same-anchor `/dir/*.pem` 같은 direct-child basename family보다 넓고 `/dir`/`/dir/**` target set 안에 contained된다. recursive literal descendant-subtree shorthand도 canonical form과 같은 descriptor/specificity로 취급한다. 즉 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`이고 dedup/conflict/containment 결과가 동일하다.
- `visibility.visible`에 recursive family나 그 shorthand가 들어오면 overlap resolution 전에 fail-fast 한다.
- family/anchor bucket fast path는 current supported grammar에만 적용되며 unsupported visible recursive form이나 broader wildcard form을 근사하면 안 된다. shorthand 추가는 normalization/path-matcher-only여야 하고 새로운 discovery/scanning contract를 만들면 안 된다.
- `HOME`이 없으면 fail-fast 한다.
- expanded host path가 `source_root` 밖이면 fail-fast 한다.
- `~user`는 unsupported fail-fast다.
- broader unsupported wildcard forms(`foo/*/bar.pem`, `**/secret?.pem`, unanchored `**/*`, one-sided subset 밖의 bare wildcard `a*b`, brace/env/command expansion`)과 descendant-subtree literal tail 내부 wildcard(`**/.git/*/hooks/**`, `**/.git/**/hooks/**`) 또는 shorthand subset 밖 trailing `/**`-less broader/ambiguous form은 부분 expansion 없이 fail-fast 한다.
- `visibility.hidden`, `visibility.visible`, `mutability.writable`, `mutability.readonly`는 같은 normalization contract를 재사용해야 한다.

## Mutability axis checklist

### `mutability.default=writable`

이 절은 path-scoped readonly 효과를 축 모델로 읽는 checklist다.

검증 시 확인할 점:

- exact path `mutability.readonly` match는 visible mutation을 `EROFS`로 만든다.
- pattern-based `mutability.readonly` match도 visible mutation을 `EROFS`로 만든다.
- `mutability.readonly`에 매치되지 않은 visible path는 host 정책이 허용하면 mutation 가능해야 한다.
- 더 구체적인 `mutability.writable` carve-out이 있으면 해당 descendant subtree에서는 `EROFS`가 해제된다.
- default-writable fast allow는 readonly match 불가를 줄이는 최적화일 뿐이며, hidden path 또는 fully visible이 아닌 symlink target 가능성이 남아 있으면 먼저 resolved-target visibility를 다시 확인해야 한다.
- hidden path 또는 fully visible이 아닌 symlink target이 관여하면 결과는 계속 `ENOENT`다.
- 동일 연산에서 hidden-before-mutability 우선순위가 유지된다.
- current-session artifact에는 rule-match `EROFS`, carve-out success, non-match visible mutation success, hidden `ENOENT`, same-specificity conflict fail-fast, unsupported glob fail-fast를 함께 기록한다.

### `mutability.default=readonly`

이 절은 current whole-root baseline과 가장 직접적으로 연결된다.

검증 시 확인할 점:

- 기본 visible mutation은 `EROFS`다.
- `mutability.writable` carve-out에 매치된 visible path mutation은 host 정책이 허용하면 성공한다.
- 더 구체적인 `mutability.readonly` re-block이 있으면 carve-out 내부에서도 다시 `EROFS`다.
- default-readonly fast `EROFS`는 writable carve-out 불가를 줄이는 최적화일 뿐이며, hidden path나 hidden target 가능성이 남아 있으면 먼저 `ENOENT`가 우선해야 한다.
- hidden path나 hidden target은 writable carve-out 또는 readonly re-block과 겹쳐도 여전히 `ENOENT`다.
- hidden 우선순위가 carve-out/re-block 판정보다 앞선다.
- write-intent `open`, `setattr`, xattr mutation, `fallocate`도 같은 evaluator를 공유한다.
- `create`, `mkdir`, `mknod`, `unlink`, `rmdir`는 mutated path와 parent 둘 다 writable이어야 한다.
- `rename`, `link`, `symlink`는 source/target/parent 전체를 기준으로 판정하며, mutation에 관여하는 모든 write-requiring coordinate가 writable이어야 한다.
- destination mutation이 있는 `copy_file_range`는 source visibility와 destination writability를 분리한다. source는 hidden/read visibility 대상이고, destination path와 destination parent는 writable이어야 한다.
- current-session artifact에는 writable carve-out success, carve-out 밖 `EROFS`, nested readonly re-block `EROFS`, hidden `ENOENT`, same-specificity conflict fail-fast, unsupported glob fail-fast를 함께 기록한다.

## Unmount

FUSE mount 해제는 `fusermount3`를 사용한다.

```bash
fusermount3 -u /tmp/screenfs-root
```

## Requirement-linked verification matrix

현재 문서는 latest recorded evidence를 current contract 기준으로 적는다.

| 항목 | 현재 상태 | 근거 |
| --- | --- | --- |
| Rust formatting | 통과 | 최신 recorded evidence 기준 `cargo fmt --check` pass 기록 |
| Rust compile check | 통과 | 최신 recorded evidence 기준 `cargo check` pass 기록 |
| Rust lint | 통과 | 최신 recorded evidence 기준 `cargo clippy --all-targets --all-features` pass 기록 |
| Mount-free unit tests | 통과 | latest recorded `cargo test --all-targets --all-features`는 142 library tests + 1 binary test 통과이며, recursive literal directory shorthand equivalence, visible recursive glob rejection, recursive bridge discovery guardrail, directory-entry matcher bucket indexing, matcher module split regressions, symlink final-target safety, fd-based xattr/setattr hardening, ScreenFS mount option policy regression, state-lock concurrency regressions, `flush`/`fsync`/`release(flush)` executor-offload regressions, create/open target revalidation regressions, TOCTOU hardening regressions를 포함한다. |
| `visibility.hidden` | source evidence 통과 / repo-local live smoke 통과 | mount-free tests는 direct access `ENOENT`와 listing exclusion을 검증; current smoke는 hidden `.git/config` `stat`/mutation `ENOENT`를 캡처 |
| `visibility.visible` subtree/direct-child carve-out | source evidence 통과 | current source tests는 `/dir`, `/dir/**`, `/dir/*`, `/dir/*.suffix`, cwd/HOME-relative direct-child forms가 recursive bridge discovery 없이 bridge ancestor/listing을 유지하고 hidden sibling을 노출하지 않음을 검증한다. recursive visible glob은 이 행이 아니라 별도 rejection 항목으로 읽는다. |
| `visibility.visible` recursive glob rejection | source evidence 통과 | source tests는 `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, cwd/HOME-relative recursive descendant canonical/shorthand form이 recursive bridge discovery가 필요하다는 메시지와 함께 fail-fast 됨을 검증한다. live smoke stderr는 필요 시 별도 보강한다. |
| bridge-visible traversal/listing | source evidence 통과 | current source tests는 recursive bridge discovery 없이 subtree/direct-child metadata만으로 traversal/listing과 hidden sibling `ENOENT`가 유지됨을 검증한다. |
| directory-entry filtering fast path | source evidence 통과 | matcher source tests는 current path/parent의 family+normalized-anchor candidate bucket 수가 전체 descriptor 수보다 작고 unrelated descendant bucket을 건너뜀을 검증한다. Matcher internals는 grammar/descriptor/index module로 분리되어도 FS visibility tests가 결과 불변과 hidden sibling 비노출을 검증한다. |
| visibility fast-path guardrail | source evidence 통과 | source tests는 hide 불가가 증명될 때만 resolved-target recheck 생략이 가능하고, prior listing success·cross-request direct-path memo·symlink decision cache가 exemption이 아님을 검증한다. request-local resolved-target reuse는 같은 request 범위에서만 허용된다. |
| `mutability.default=writable` | source evidence 통과 / repo-local live smoke 통과 | `docs/artifacts/current-default-writable-smoke-transcript.md`가 default writable write success, readonly override `EROFS`, hidden-before-mutability `ENOENT`를 캡처하며, fast allow가 visibility proof를 대체하지 않는 source tests를 포함한다. |
| `mutability.default=readonly` | source evidence 통과 / repo-local 및 whole-root/chroot live smoke 통과 | current smokes는 readonly default 아래 writable carve-out success, bridge-visible mutation `EROFS`, whole-root `/etc` write `EROFS`를 캡처하며, fast `EROFS`가 hidden-before-`EROFS`를 뒤집지 않는 source tests를 포함한다. |
| `mutability.writable` carve-out | source evidence 통과 / repo-local live smoke 통과 | current smoke는 `/tmp/existing` 및 `/allowed/existing` write success를 캡처 |
| `mutability.readonly` re-block | source evidence 통과 / repo-local live smoke 통과 | current smoke는 `/allowed/reblock/existing` write가 `EROFS`로 막히는 것을 캡처 |
| hidden-before-mutability | source evidence 통과 / repo-local live smoke 통과 | current smoke는 hidden `.git/config` mutation이 readonly보다 먼저 `ENOENT`가 되는 것을 캡처 |
| combined visibility/mutability fast-path precedence | source evidence 통과 | source tests는 visibility block과 mutability block을 분리해도 hidden/non-fully-visible target이 있으면 항상 `ENOENT`가 `EROFS`보다 먼저 유지됨을 검증한다. |
| symlink fully-visible gate | source evidence 통과 | current source tests는 single-hop/multi-hop hidden target, ancestor symlink into hidden subtree, bridge-visible target symlink의 `lookup`/`open`/`opendir`/`access`/`readlink` `ENOENT`와 source-root escape handling을 검증한다. |
| symlink point-of-use fast path | source evidence 통과 | source tests는 target visibility check 생략이 visibility policy가 target을 숨길 수 없을 때만 가능함을 검증하고, hide 가능한 policy에서 resolved final target 기준 point-of-use `ENOENT`가 유지됨을 검증한다. |
| shared recursive-family comparison (`**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`) | source evidence 통과 | requirements/design table은 shared matcher target semantics를 정의하고, latest source tests가 `**/*.pem` cwd-anchor recursive shorthand, explicit root-anchor `/**/*.pem`, `./fixtures/**/*.pem`, `/a/*.txt`, `/a/**/*.txt`를 검증한다. 이 evidence는 `visibility.visible` recursive allowlist가 아니다. |
| anchored direct-child wildcard-all target semantics (`/dir/*`, `./dir/*`, `~/dir/*`) | source evidence 통과 / repo-local live smoke 통과 | requirements/design은 normalized anchor directory의 immediate child 전체와 각 child descendants에만 적용되는 semantics과 `/dir/*.pem` containment를 정의한다. source tests와 `docs/artifacts/current-compatibility-pattern-smoke-transcript.md`가 absolute/relative/home anchored wildcard-all matching을 캡처한다. |
| same-anchor subtree shorthand equivalence (`/dir/**`, `./dir/**`, `~/dir/**`) | source evidence 통과 / repo-local live smoke 통과 | requirements/design은 `/dir/**`가 `/dir` subtree rule과 정확히 동등하고 같은 descriptor/specificity를 공유한다고 정의한다. source tests와 `docs/artifacts/current-compatibility-pattern-smoke-transcript.md`가 absolute/relative/home subtree shorthand matching을 캡처한다. |
| bare slashless cwd-anchor/direct-child target semantics | source evidence 통과 / repo-local live smoke 통과 | `cargo test --all-targets --all-features`와 `current-bare-basename-glob-smoke-transcript.md`가 cwd rebasing, cwd-outside-`source_root` fail-fast, `*.pem`=`./*.pem` equivalence를 검증한다. same-anchor recursive shorthand(`**/*.pem`=`./**/*.pem`) 및 explicit root-anchor `/**/*.pem` distinction은 source tests가 별도로 보강한다. |
| unsupported glob fail-fast | source evidence 통과 / repo-local live smoke 통과 | current source tests는 `*`, `a*b`, `*secret*`, `**/secret?.pem`, `foo/*/bar.pem` 같은 still-unsupported form을 fail-fast로 검증한다. unanchored `**/*`도 source tests와 `current-compatibility-pattern-smoke-transcript.md`에서 직접 fail-fast로 캡처한다. |
| same-specificity conflict fail-fast | source evidence 통과 / repo-local live smoke 통과 | current source tests는 exact-path opposite-polarity conflict와 shorthand/canonical recursive literal directory conflict를 검증하고, `current-bare-basename-glob-smoke-transcript.md`는 `*.pem` vs `./*.pem` same-normalized-specificity conflict를 캡처한다. |
| direct-child no-recursive-traversal performance | source evidence 통과 / live smoke 선택 보강 | source tests는 direct-child visible rule이 nested descendants를 사전 발견하지 않으며 matcher candidate bucket이 unrelated descriptors를 건너뜀을 검증한다. `/tmp/*` live smoke나 `strace` 계측은 필요 시 별도 보강한다. |
| non-visible recursive literal directory shorthand equivalence | source evidence 통과 | latest source tests는 `**/.git/hooks`=`**/.git/hooks/**`, `/repo/**/.git/hooks`=`/repo/**/.git/hooks/**`, `**/node_modules`=`**/node_modules/**`, `**/.git`=`**/.git/**`, `~/**/aaa/hook`=`~/**/aaa/hook/**` equivalence와 hidden `ENOENT`, readonly `EROFS`, writable override, dedup/conflict identity를 검증한다. Config tests는 non-visible scope에서 generic `**/<literal-dir>` single-component shorthand도 수용함을 검증한다. |
| recursive literal directory shorthand no-discovery/scanning guardrail | source evidence 통과 | latest source diff는 shorthand를 canonical recursive literal descriptor로 normalize하는 parser/matcher change만 추가했고, no-discovery grep은 `dynamic_bridge`, `bridge_visible_dirs`, `build_bridge_visible_dirs`, `needs_dynamic`, `startup scan`, `lazy discovery`, `background indexing`, listing-result cache, symlink-decision cache 추가가 없음을 확인했다. Supported shorthand(`**/.git/hooks`, `~/**/aaa/hook`)와 unsupported multi-recursive form(`~/**/bbb/**/ccc`) 구분도 source tests가 검증한다. |
| user-namespace chroot smoke | current whole-root/chroot live smoke 통과 | `docs/artifacts/current-whole-root-chroot-smoke-transcript.md`가 `unshare -r -R <mount>`에서 `/etc/passwd` read, `/root` `ENOENT`, `/tmp` writable carve-out을 캡처 |
| 성능/메모리 측정 | source evidence 통과 / live smoke 선택 보강 | source tests가 startup/lazy recursive bridge discovery 없이 unrelated matcher bucket skip과 recursive literal directory shorthand no-discovery/scanning guardrail을 검증한다. supported shorthand(`**/.git/hooks`, `~/**/aaa/hook`)는 canonical recursive literal subtree와 같은 descriptor/matcher cost를 사용하고, `~/**/bbb/**/ccc` 같은 multi-recursive form은 fail-fast로 남는다. xattr/setattr은 path-string 재해석 대신 confined fd를 사용한다. live performance smoke는 필요 시 별도 보강한다. |

## 문서 변경 검증

문서만 변경했을 때 최소 검증(AGENTS.md 기준):

```bash
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts -maxdepth 3 -type f -print | sort
cargo check
```

current `visibility.visible` contract를 건드렸다면 위 명령에 더해 다음도 반드시 남긴다.

- visible recursive rejection checklist(`**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks/**`, `**/.git/hooks`, `/repo/**/.git/hooks/**`, `/repo/**/.git/hooks`, `./repo/**/.git/hooks/**`, `./repo/**/.git/hooks`, cwd/HOME-relative recursive form)
- non-visible recursive literal directory shorthand checklist(`**/.git/hooks`, `/repo/**/.git/hooks`, `**/node_modules`, `**/.git`, `**/target`, `**/dist`, `**/build`)
- directory-entry filtering fast-path checklist
- direct-child no-recursive-traversal checklist(`/tmp/*` 또는 동등형)
- recursive literal directory shorthand no-discovery/scanning guardrail checklist
- symlink point-of-use fast-path checklist

추가로 `git diff -- README.md AGENTS.md docs .pi` 또는 추적 전 파일의 실제 내용을 확인해 요구사항 누락, 문서 간 모순, 미승인 임시 문구가 없는지 검토한다.
