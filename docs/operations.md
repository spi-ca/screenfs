# 운영 및 검증

이 문서는 목표 계약과 verified evidence를 구분해 관리한다. 정책 surface는 `visibility.*`와 `mutability.*` 두 축뿐이며, current evidence는 current contract baseline만 기록한다.

## 1. Environment assumptions

현재 확인된 개발 환경:

```text
kernel: 7.1.0-rc7-1-spica-git
kernel config artifact: /home/spi-ca/Codebase/packages/managed/linux-spica-git/config.saved.x86_64
kernel config flags: CONFIG_FUSE_IO_URING=y, CONFIG_IO_URING=y
fusermount3: /usr/bin/fusermount3
fusermount3 version: 3.18.2
/dev/fuse: present
project: /home/spi-ca/Codebase/screenfs
fractal-fuse = 0.4.0
```

Kernel config artifact는 FUSE/io_uring 전제의 정적 근거다. Live mount success 또는 session negotiation success로 승격하지 않는다.

## 2. Standard verification commands

문서만 변경했을 때 최소 확인:

```bash
cargo check
find README.md AGENTS.md docs -maxdepth 2 -type f -print
find .pi/agents .pi/skills .pi/prompts .pi/extensions -maxdepth 3 -type f -print | sort
```

코드 변경이 포함되면 추가 확인:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --all-targets --all-features
```

다이어그램 source/config 변경 시:

```bash
for input in docs/diagrams/*.mmd; do
  base="${input%.mmd}"
  bunx @mermaid-js/mermaid-cli -i "$input" -o "${base}.svg" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json
  bunx @mermaid-js/mermaid-cli -i "$input" -o "${base}.png" \
    -c docs/diagrams/mermaid-config.json \
    -p docs/diagrams/puppeteer-config.json \
    --scale 2
done
file docs/diagrams/*.png
```

## 3. Current evidence map

| Evidence | Artifact |
| --- | --- |
| whole-root/chroot live smoke | [`artifacts/current-whole-root-chroot-smoke-transcript.md`](artifacts/current-whole-root-chroot-smoke-transcript.md) |
| default-writable live smoke | [`artifacts/current-default-writable-smoke-transcript.md`](artifacts/current-default-writable-smoke-transcript.md) |
| bare slashless cwd-anchor direct-child smoke | [`artifacts/current-bare-basename-glob-smoke-transcript.md`](artifacts/current-bare-basename-glob-smoke-transcript.md) |
| visible direct-child/subtree compatibility smoke | [`artifacts/current-compatibility-pattern-smoke-transcript.md`](artifacts/current-compatibility-pattern-smoke-transcript.md) |
| FUSE transport contract source evidence | [`artifacts/current-fuse-transport-contract-evidence.md`](artifacts/current-fuse-transport-contract-evidence.md) |
| file data-path async/io_uring feasibility | [`artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md) |
| state-lock concurrency evidence | [`artifacts/current-state-lock-concurrency-evidence.md`](artifacts/current-state-lock-concurrency-evidence.md) |
| TOCTOU hardening evidence | [`artifacts/current-toctou-hardening-evidence.md`](artifacts/current-toctou-hardening-evidence.md) |
| perf-counter baseline summary | [`artifacts/current-perf-counter-baseline-summary.md`](artifacts/current-perf-counter-baseline-summary.md) |
| perf-counter benchmark result | [`artifacts/current-perf-counter-benchmark-result.md`](artifacts/current-perf-counter-benchmark-result.md) |

Latest recorded source-level coverage includes non-root startup rejection, visibility/mutability axis regressions, bare slashless cwd-anchor/direct-child regressions, recursive family coverage, visible recursive rejection, symlink final-target safety, fd-based xattr/setattr hardening, state-lock concurrency regressions, `flush`/`fsync`/`release(flush)` executor-offload regressions, TOCTOU hardening, and large-directory page-limiting behavior. Re-run commands before claiming a new pass in current session.

## 4. Smoke evidence recording rules

A smoke artifact must record enough context to reproduce the claim:

- command line or equivalent execution surface
- source root and mount root
- policy inputs: visibility default/hidden/visible, mutability default/readonly/writable
- `fusermount3 --version`
- relevant `ls`/`stat`/`cat`/mutation commands
- stdout/stderr excerpts
- unmount result
- whether the evidence is source-level, repo-local live, whole-root live, chroot live, or benchmark

Rules:

- Do not mark a smoke as successful unless the mount was alive and follow-up checks ran.
- Mount startup failure is `failed` or `blocked`, not hidden/whole-view/mutability success.
- Environment artifacts and source inspection are evidence, but not substitutes for live smoke when the claim is live behavior.
- Formal performance claims follow [`benchmarks.md`](benchmarks.md); small smoke runs only prove harness plumbing.
- Current contract and archival/pre-removal evidence must not be mixed.

## 5. Policy smoke checklist

### Visibility hidden

```bash
ls /tmp/screenfs-root/home/spi-ca
find /tmp/screenfs-root/home/spi-ca -maxdepth 2 -name .ssh
stat /tmp/screenfs-root/home/spi-ca/.ssh
cat /tmp/screenfs-root/home/spi-ca/.ssh/id_rsa
```

Expected:

- hidden entry is absent from listing
- direct access fails with `No such file or directory`
- no `Permission denied` existence leak for hidden target

### Bridge-visible carve-out

Record in one session:

- parent listing showing bridge entries only
- visible descendant `stat` and read success
- hidden sibling `ENOENT`
- `readdir` and `readdirplus` consistency when relevant
- same directory handle keeps stable resume cookie/shared cookie domain when listing resumes are part of the claim
- `readdirplus` evidence should note that lookup refs pin only the returned page child entries when relevant
- bridge-visible mutation `EROFS`

### Current `visibility.visible` category

Supported:

- exact/subtree: `/dir`, `/dir/**`, `./dir`, `~/dir`
- direct-child: `/dir/*`, `/dir/*.pem`, `/dir/id_*`, `/dir/.env.*`, `*.pem`, `./dir/*`, `~/dir/*`

Unsupported/fail-fast in `visibility.visible`:

```bash
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/dir/**/*.pem'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '**/.git/hooks'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/repo/**/.git/hooks/**'
screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/repo/**/.git/hooks'
```

Expected: startup fail-fast before mount; reason indicates recursive bridge discovery is unsupported for `visibility.visible`.

### Non-visible recursive literal shorthand

For `visibility.hidden`, `mutability.readonly`, `mutability.writable`, shorthand and canonical trailing `/**` form must be equivalent:

```bash
screenfs / /tmp/screenfs-root --hidden '**/.git/hooks'
screenfs / /tmp/screenfs-root --hidden '**/.git/hooks/**'
screenfs / /tmp/screenfs-root --hidden '/repo/**/.git/hooks'
screenfs / /tmp/screenfs-root --readonly '**/node_modules'
screenfs / /tmp/screenfs-root --readonly '**/node_modules/**'
screenfs / /tmp/screenfs-root --writable '**/dist'
screenfs / /tmp/screenfs-root --writable '**/dist/**'
```

Expected:

- canonical/shorthand match directory itself and descendants identically
- hidden gives `ENOENT`, readonly gives `EROFS`, writable carve-out succeeds when host allows
- same-polarity duplicates dedup; opposite-polarity same-specificity conflicts fail-fast
- no recursive bridge discovery, lazy discovery, startup scan, background indexing, listing-result cache, or symlink-decision cache is introduced

### Direct-child no-recursive-traversal

Direct-child visible rule such as `/tmp/*` evaluates immediate children only. Evidence should show:

- no startup or first listing recursive walk of deeper descendants
- current directory/parent based matcher bucket use
- hidden sibling and bridge listing semantics unchanged

Optional trace shape:

```bash
strace -f -e getdents64,newfstatat \
  -o /tmp/screenfs-visible-tmp-star.strace \
  screenfs / /tmp/screenfs-root --visibility-default hidden --visible '/tmp/*'
```

### Symlink point-of-use

Evidence must show:

- target visibility check runs at listing/lookup/getattr/readlink/dereference/open when hide possibility remains
- prior listing success, cross-request direct-path memo, and symlink decision cache are not exemption grounds
- request-local resolved target reuse is limited to a single FUSE request

## 6. Mutability checklist

### `mutability.default=writable`

Record:

- readonly match mutation returns `EROFS`
- non-match visible mutation succeeds when host allows
- more-specific writable carve-out succeeds
- hidden or non-fully-visible symlink target still returns `ENOENT`

### `mutability.default=readonly`

Record:

- default visible mutation returns `EROFS`
- writable carve-out mutation succeeds when host allows
- nested readonly re-block returns `EROFS`
- hidden-before-`EROFS` precedence holds
- create/delete/rename/link/symlink/copy-file coordinate rules are checked, not just final path

## 7. Runtime boundary checks

- Startup as effective uid 0 must fail before mount with a non-root error.
- Open file handles follow documented POSIX fd lifetime semantics: after policy checks, already opened file data operations act on the pinned fd; external same-UID rename/unlink after validation is a residual boundary, not a stronger current-path atomicity guarantee.
- Raw symlink targets that are lexically fully visible but escape `source_root` may be returned by `readlink`; dereference/open/access must fail at confinement with `ENOENT`. Treat this as a documented information-exposure boundary.
- Source-root confinement, point-of-use symlink visibility, and hidden-before-`EROFS` checks must still run before host delegation.

## 8. Whole-view and chroot checks

Whole-view baseline:

```bash
ls /tmp/screenfs-root
ls /tmp/screenfs-root/bin
ls /tmp/screenfs-root/usr
ls /tmp/screenfs-root/lib
ls /tmp/screenfs-root/lib64
ls /tmp/screenfs-root/etc
```

Expected: allowed system paths are visible and usable.

Chroot/user namespace evidence should record the exact supervisor command. ScreenFS only provides the mount view; chroot privilege model and device-node semantics remain outside ScreenFS.

## 9. Performance evidence

- Use [`benchmarks.md`](benchmarks.md) for formal benchmark method and claim bar.
- Benchmark evidence is scoped to the recorded machine, kernel, backing filesystem, policy, and workload.
- Perf counters require a build with `--features perf-counters`; they are attribution evidence, not standalone speed claims.
- Do not use benchmark results to relax hidden `ENOENT`, bridge-visible, symlink target, or mutability precedence semantics.

## 10. Requirement-linked evidence summary

| Requirement area | Current evidence status |
| --- | --- |
| Rust formatting/check/test/clippy | Recorded pass exists; re-run before current-session claim |
| Non-root startup guard | Source inspection verifies CLI startup and `ScreenFs::new` call the shared non-root guard; unit coverage verifies root effective-uid classification. Live root execution is not required for normal non-root validation |
| `visibility.hidden` | Source evidence and live smoke baseline |
| `visibility.visible` subtree/direct-child | Source evidence and compatibility smoke baseline |
| recursive visible rejection | Source evidence; live stderr can be added when needed |
| bridge-visible traversal/listing | Source evidence |
| `readdir`/`readdirplus` bounded page, stable resume cookie/shared cookie domain, returned-page `readdirplus` lookup refs | Source evidence |
| directory-entry filtering fast path | Source evidence |
| symlink fully-visible gate and point-of-use fast path | Source evidence |
| `mutability.default=writable` | Source evidence and live smoke baseline |
| `mutability.default=readonly` | Source evidence and live smoke baseline |
| hidden-before-mutability | Source evidence and live smoke baseline |
| bare slashless/direct-child semantics | Source evidence and live smoke baseline |
| recursive literal non-visible shorthand | Source evidence |
| user-namespace chroot smoke | Whole-root/chroot live smoke baseline |
