# ScreenFS 설계 문서

이 문서는 ScreenFS의 현재 목표 계약을 구현 관점에서 정리한다. 제품 요구사항은 [`requirements.md`](requirements.md), 코드 경계 요약은 [`architecture.md`](architecture.md), current evidence는 [`operations.md`](operations.md)를 따른다.

## 1. Design goals

- non-root 사용자 권한으로 whole-root FUSE3 view를 제공하고, effective uid 0 실행은 startup error로 거부한다.
- `fusermount3`, `fractal-fuse = 0.4.0`, `FUSE_OVER_IO_URING` 협상 성공을 기준으로 한다.
- 정책 surface는 `visibility`와 `mutability` 두 축만 사용한다.
- hidden `ENOENT`가 mutability `EROFS`보다 항상 먼저 적용된다.
- root-only mount, privileged bind mount, 이름이 남는 masking overlay는 사용하지 않는다.

## 2. Filesystem view model

입력은 `source_root`와 `mount_root`이고 출력은 whole-root consumer가 읽는 virtual root다.

```text
source root: /
mount root:  /tmp/screenfs-root
virtual path /usr/bin/bash -> source path /usr/bin/bash
```

`source_root=/`에서 `mount_root` subtree가 다시 view 안에 보이면 자기참조가 생긴다. 따라서 mount-root source-relative subtree는 internal hidden rule로 취급하고 user policy보다 먼저 적용한다.

## 3. Policy evaluation order

모든 request는 다음 순서를 따른다.

1. inode/name/file handle에서 virtual path를 계산한다.
2. virtual path를 lexical absolute path로 normalize한다.
3. internal mount-root exclusion을 먼저 적용한다.
4. visibility를 `hidden`, `bridge-visible`, `visible` 중 하나로 판정한다.
5. hidden이면 `ENOENT` 또는 listing omission으로 끝낸다.
6. symlink가 관여하면 resolved virtual target이 fully visible인지 point-of-use에서 확인한다.
7. bridge-visible ancestor mutation이면 `EROFS`다.
8. fully visible write-requiring coordinate에만 mutability를 적용한다.
9. 허용되면 confined host filesystem operation으로 위임한다.

핵심 우선순위:

```text
hidden/non-fully-visible ENOENT
  > bridge-visible mutation EROFS
  > mutability readonly EROFS
  > host errno
```

## 4. Visibility design

```yaml
visibility:
  default: visible | hidden
  hidden: []
  visible: []
```

- `default=visible`: hidden rule이 기본 차단이고, 더 구체적인 visible rule이 carve-out이 될 수 있다.
- `default=hidden`: visible rule이 allowlist이고, 더 구체적인 hidden rule이 re-block이 될 수 있다.
- hidden path는 listing에서 빠지고 direct operation은 `ENOENT`다.
- listing 성공, cross-request direct-path memo, symlink decision cache는 later target check 면제 근거가 아니다.

### Bridge-visible ancestor

Bridge-visible은 visible descendant로 도달시키기 위한 ancestor directory 상태다.

허용:

- `lookup`, `getattr`, read/traverse intent `access`
- `opendir`, `readdir`, `readdirplus`

거부:

- create/delete/rename/link/symlink
- write-intent `open`
- `setattr`, xattr mutation, `fallocate`

Bridge-visible listing은 visible child 또는 visible descendant로 이어지는 child만 반환한다. hidden sibling은 노출하지 않는다.

### Current `visibility.visible` subset

| category | 지원 | 구현 제약 |
| --- | --- | --- |
| static subtree bridge | `/dir`, `/dir/**`, `./dir`, `~/dir` | visible target까지 정적 ancestor chain만 사용 |
| direct-child anchor bridge | `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare/cwd/HOME 동등형 | normalized anchor ancestor만 bridge-visible candidate; current directory/parent 기준 immediate child만 평가 |
| recursive visible glob/shorthand | `**/*.pem`, `/**/*.pem`, `/dir/**/*.pem`, `**/.git/hooks`, `/repo/**/.git/hooks` 등 | unsupported/fail-fast; recursive bridge discovery, startup scan, lazy discovery 금지 |

### Current `readdirplus` directory-local visibility batch

- `DirectoryChildVisibilityBatch`는 현재 `readdirplus` scan에서만 쓰는 request/directory-local helper다. `readdir`, direct-path lookup/open/access, mutation guard는 기존 per-entry policy path를 그대로 사용하고, cross-request/handle cache로 승격하지 않는다.
- fast path는 두 shape로만 제한한다: (1) `visibility.default=visible` + no hidden/internal-hidden rules인 `AllVisible`, (2) `visibility.default=hidden` + no hidden/internal-hidden rules + visible subtree descriptor만 있는 parent-local mode. hidden rule, internal hidden rule, non-subtree visible rule처럼 rule-sensitive shape는 즉시 `PerEntry` fallback으로 돌아간다.
- parent-local mode는 현재 parent의 direct child frontier만 분류한다. fully visible child는 바로 통과시키고, bridge-visible child는 directory일 때만 readable로 취급하며, parent의 direct child가 아니어서 helper가 확정할 수 없는 path만 기존 matcher 판정으로 되돌린다.
- 이 batch는 correctness를 약화하지 않는다. hidden `ENOENT` precedence, axis별 most-specific rule wins, symlink target point-of-use 검사, returned `readdirplus` entry의 `stat_child_no_follow()` 뒤 `entry_is_readable()` 재확인, returned symlink target 재확인은 계속 유지된다.

## 5. Mutability design

```yaml
mutability:
  default: writable | readonly
  readonly: []
  writable: []
```

- mutability는 visibility가 fully visible로 증명된 coordinate에만 적용한다.
- `default=writable`: readonly match가 mutation을 `EROFS`로 막고, 더 구체적인 writable이 다시 허용할 수 있다.
- `default=readonly`: writable match가 mutation을 허용하고, 더 구체적인 readonly가 다시 막을 수 있다.
- fast allow/block은 visibility proof를 대체하지 못한다.

Write-requiring coordinate 예:

| Operation | Coordinates |
| --- | --- |
| `create`, `mkdir`, `mknod` | target + parent |
| `unlink`, `rmdir` | removed path + parent |
| write-intent `open`, `write`, `setattr`, xattr mutation, `fallocate` | entry path와 필요 시 resolved target |
| `rename` | source + target + source parent + target parent |
| `link` | source visibility + target + target parent |
| `symlink` | link path + link parent + target visibility checks |
| `copy_file_range` | source visibility + destination path + destination parent |

## 6. Shared rule normalization

Matching은 host canonical path가 아니라 lexical virtual path 기준이다. `visibility.hidden`, `visibility.visible`, `mutability.readonly`, `mutability.writable`는 같은 parser/normalization을 공유한다. 단, `visibility.visible`은 discovery-free subset만 current surface로 허용한다.

Supported family:

- **exact/subtree**: `/dir`, `/dir/**`, `./dir`, `~/dir`; same-anchor `/dir/**`는 `/dir`와 같은 descriptor다.
- **direct-child glob**: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, bare `*.pem`, `.env.*`, `id_*`; bare slashless form은 launch cwd 기준 `./<pattern>` direct-child shorthand다.
- **recursive non-visible glob**: `**/*.pem`, `/**/*.pem`, `./fixtures/**/*.pem`, `/a/**/*.txt`; prefixless form은 cwd anchor의 recursive shorthand다.
- **recursive literal non-visible subtree**: `**/.git`, `**/.git/hooks`, `/repo/**/.git/hooks`, `~/**/aaa/hook`; `**/<literal-dir>` 또는 `<prefix>/**/<literal-tail>`만 허용하고 canonical trailing `/**` descriptor와 동등하게 compile한다.

Conflict and fail-fast:

- same-polarity duplicate는 idempotent다.
- 같은 축의 opposite-polarity same-specificity conflict는 invalid configuration이다.
- overlap/containment를 증명할 수 없는 broader glob 조합은 fail-fast다.
- `HOME` 없음, source-root 밖 cwd/expanded path, `~user`, prefix 내부 wildcard, unanchored `**/*`, `*`, `a*b`, `*secret*`, multi-recursive descendant form은 fail-fast다.
- same-axis opposite-polarity conflict error는 axis label과 normalized descriptor를 함께 출력한다. 현재 메시지 shape는 `hidden and visible rules conflict at the same normalized specificity: hidden rule [...], visible rule [...]`, `readonly and writable rules have overlapping glob targets without provable containment: ...`다.
- `visibility.visible` recursive glob rejection은 이유, offending normalized rule, 대안을 함께 준다. 현재 메시지 shape는 `invalid visible pattern: recursive visible globs are unsupported because they require recursive bridge discovery: visible rule [...]; prefer an explicit subtree visible rule such as /dir or /dir/**`다.
- config/path/glob parse failure도 mount 전에 raw config path, raw offending rule, 또는 normalized rule descriptor를 포함해 보고한다. 예: `failed to parse config <path>: ...`, `rule path resolves outside source_root: <rule>`, `unsupported glob: <rule>`.

## 7. State, cache, and concurrency

- inode/path map, lookup/open refcount, file/dir handle table, directory cookie state는 하나의 consistency domain이다.
- current stance는 `RwLock<State>` 단일 domain이다. per-table lock split은 별도 design/evidence 전까지 하지 않는다.
- state lock을 host filesystem I/O나 blocking syscall 구간에 잡고 있지 않는다.
- `readdirplus` lookup ref 증가는 실제 반환 page child에만, returned-page commit과 같은 write-lock transaction에서 수행한다.
- request-local resolved-target reuse는 허용하지만 cross-request symlink decision cache나 stable listing snapshot cache는 current contract가 아니다.
- current per-open cache는 열린 file handle에 붙는 handle-local `read`/`write` fast path 범위로만 제한하며, cache-eligible `visible`/`writable` policy에서만 반복 guard를 생략한다. hidden/visible carve-out, readonly/writable carve-out, symlink target 재검증처럼 current-path proof가 다시 필요한 policy shape는 계속 full guard path를 재실행한다. broad path/global authorization cache나 negative cache로 확대하지 않는다.

## 8. FUSE and host delegation boundary

- `FUSE_OVER_IO_URING` 요구는 FUSE transport에 한정한다.
- backing filesystem access는 `source_root` confinement와 fd/dirfd-relative syscall을 우선한다.
- Raw symlink target이 lexical visibility 기준으로 fully visible하지만 host resolution에서 `source_root` 밖으로 escape하면 `readlink`는 raw target을 반환할 수 있고, dereference/open/access는 confinement 단계에서 `ENOENT`로 실패한다. 이 정보노출 경계는 current contract로 문서화한다.
- open target과 mutation parent는 fd로 pin하고, mutation 직전 opened parent dirfd가 요청된 virtual parent path에 남아 있는지 best-effort로 재확인한다.
- current per-open read/write cache도 hidden `ENOENT`, readonly `EROFS`, symlink target revalidation, source-root confinement/`openat2` proof를 생략하지 않는다. unsafe policy shape는 계속 full guard path를 재실행한다.
- hidden/readonly/symlink-target concern이 있는 unsafe policy shape에서는 rename/unlink/symlink retarget/ancestor change 뒤 stale authorization이 남지 않도록 기존 per-I/O fail-closed revalidation path를 유지한다.
- cache-eligible default `visible`/`writable` shape에는 숨기거나 readonly로 막을 target authorization concern이 없으므로, 이미 열린 fd의 data I/O는 documented POSIX pinned-fd lifetime semantics를 따른다. 이 경우에도 cached path로 reopen하지 않는다.
- 외부 same-UID actor가 validation 이후 이미 pin된 object를 rename/unlink하면 해당 policy shape의 pinned-fd semantics를 따른다. ScreenFS는 current virtual path membership을 원자적으로 보장하지 않는다.
- `flush`/`fsync`/`release(flush)` offload는 state lock 밖 blocking pool로 sync syscall 실행 위치만 옮기는 low-risk cleanup이다. `read`/`write`는 `FileExt::read_at`/`write_at` 경로를 유지한다. `fallocate`/`copy_file_range`는 current per-open cache 범위에 포함되지 않는다.
- 현재 이 surface의 focused evidence는 [`src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) / `src/fs/tests/perf/**` (`perf_counters_record_data_path_splits_on_success`, `perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe`, `perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures`, `perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path`), [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`cache_eligible_opened_file_read_write_keep_pinned_fd_after_host_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_ancestor_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_unlink`, `opened_file_read_keeps_pinned_fd_after_host_rename_but_write_fails_closed`, `opened_file_read_write_fail_closed_after_host_rename_into_hidden_subtree`), [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`cache_eligible_opened_symlink_read_keeps_pinned_fd_after_final_retarget`, `cache_eligible_opened_symlink_read_keeps_pinned_fd_after_ancestor_retarget`, `opened_symlink_read_revalidates_hidden_target_after_retarget`), [`../src/fs/tests/mutability.rs`](../src/fs/tests/mutability.rs) (`opened_symlink_write_revalidates_readonly_target_after_retarget`), [`artifacts/current-fio-per-open-cache-summary.md`](artifacts/current-fio-per-open-cache-summary.md)와 companion `current-fio-per-open-cache-*` smoke artifact다.

## 9. Validation strategy

변경은 다음 층에서 검증한다.

- Unit: path normalization, matcher conflict/dedup, hidden-before-EROFS, affected-coordinate classification
- Integration without mount: hidden/listing, bridge-visible traversal, symlink target visibility, mutability overrides, config/CLI precedence
- FUSE smoke: real listing/direct access/mutation behavior, mount-root exclusion, whole-root/chroot baseline
- System smoke: `/dev/fuse`, `fusermount3`, FUSE/io_uring environment, unmount cleanup
- Performance: [`benchmarks.md`](benchmarks.md)의 harness와 claim bar
- Per-open read/write cache evidence: focused regression/perf tests와 [`artifacts/current-fio-per-open-cache-summary.md`](artifacts/current-fio-per-open-cache-summary.md) + `current-fio-per-open-cache-*` smoke artifact로 fast path exercised 여부를 확인하되, user-visible speedup claim은 여전히 claim-grade before/after benchmark pair 없이는 하지 않는다.

다이어그램 source of truth는 `docs/diagrams/*.mmd`이고 렌더링 계약은 [`diagrams/README.md`](diagrams/README.md)를 따른다.
