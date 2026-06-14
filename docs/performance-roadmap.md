# ScreenFS performance roadmap

이 문서는 ScreenFS 성능 개선 후보를 현재 상태, 위험도, 적용 순서 기준으로 정리한다. 성능 측정 절차와 claim 기준은 [`benchmarks.md`](benchmarks.md)가 source of truth이고, 운영/검증 evidence는 [`operations.md`](operations.md)와 `docs/artifacts/**`가 source of truth다.

## Guardrails

성능 최적화는 기존 correctness 계약을 바꾸지 않는다.

- `visibility.hidden`은 계속 `ENOENT`로 처리하고, `mutability`의 `EROFS`보다 우선한다.
- `openat2(RESOLVE_IN_ROOT | RESOLVE_NO_MAGICLINKS)` 기반 confinement는 correctness 핵심이므로 성능 목적으로 대체하지 않는다.
- symlink visibility 판단을 요청 간 공유 cache로 생략하지 않는다.
- state lock을 잡은 채 blocking IO 또는 `spawn_blocking`을 호출하지 않는다.
- 최적화는 benchmark, perf counter, trace, 또는 microbenchmark로 병목을 확인한 뒤 적용한다.
- benchmark 결과는 hidden `ENOENT`, bridge-visible, symlink target, mutability precedence 의미론을 완화하는 근거로 쓰지 않는다.

## Current baseline

### FUSE transport

ScreenFS는 `fractal-fuse = 0.4.0`의 FUSE3/io_uring transport 목표를 유지한다. 이 항목은 future TODO가 아니라 현재 baseline contract다.

```text
kernel <-> ScreenFS
FUSE_OVER_IO_URING
```

기대 효과는 FUSE request/reply transport 병목 완화와 async dispatch 기반 유지다. 이 영역은 mount/live smoke에서 협상 여부와 기본 operation 정상 동작을 계속 확인한다.

현재 evidence:

- transport contract source/local no-fallback 근거: [`artifacts/current-fuse-transport-contract-evidence.md`](artifacts/current-fuse-transport-contract-evidence.md)
- 운영/환경/라이브 smoke 분리 기준과 artifact quick map: [`operations.md`](operations.md)
- current live smoke baseline 예시: [`artifacts/current-whole-root-chroot-smoke-transcript.md`](artifacts/current-whole-root-chroot-smoke-transcript.md), [`artifacts/current-default-writable-smoke-transcript.md`](artifacts/current-default-writable-smoke-transcript.md) (`io_uring=required` mount transcript)
- benchmark 경계: [`benchmarks.md`](benchmarks.md) (mount startup/transport negotiation 자체는 benchmark timing에 넣지 않으며, benchmark 결과는 transport correctness 증거로 대체하지 않음)

### File data path

`read`/`write`는 이미 [`../src/fs.rs`](../src/fs.rs)의 `read()`/`write()`에서 `FileExt::read_at` / `FileExt::write_at`를 사용한다. `seek + read/write` 전환 자체는 더 이상 TODO가 아니다.

현재 evidence:

- 구현: [`../src/fs.rs`](../src/fs.rs) (`read()`, `write()`)
- 회귀 테스트: [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`read_write_offsets_do_not_depend_on_shared_file_position`)
- 데이터 경로 범위/후속 조건: [`artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md)
- 상태 잠금/lock-outside-IO 근거: [`artifacts/current-state-lock-concurrency-evidence.md`](artifacts/current-state-lock-concurrency-evidence.md)

남은 질문:

- 작은/큰 IO에서 syscall 비용이 어디서 커지는지
- concurrent reader/writer에서 handle snapshot 비용이 실제 병목인지
- host-side offload 또는 io_uring 검토가 필요한 수준의 data-path 병목이 있는지

### Sync surface

`flush`, `fsync`, `release(flush=true)`는 이미 [`../src/fs.rs`](../src/fs.rs)의 `flush()`/`release()`/`fsync()`에서 file-handle snapshot/removal 뒤 state lock 밖으로 sync syscall을 offload한다.

현재 evidence:

- 구현: [`../src/fs.rs`](../src/fs.rs) (`flush()`, `release()`, `fsync()`, `offload_file_sync()`)
- 회귀 테스트: [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`flush_and_fsync_complete_under_compio_runtime`, `release_flush_true_completes_under_compio_runtime_and_removes_handle`, `offload_file_sync_preserves_closure_errno`, `offload_file_sync_completes_without_ambient_compio_runtime`, `offload_file_sync_maps_closure_panic_to_eio`)
- sync surface 범위/후속 조건: [`artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md)
- 설계/락 규칙: [`artifacts/current-state-lock-concurrency-evidence.md`](artifacts/current-state-lock-concurrency-evidence.md)

남은 질문:

- close/fsync-heavy workload에서 blocking pool saturation/backpressure가 있는지
- sync surface별 tail latency가 어떻게 나타나는지
- errno/join failure mapping이 관측 가능한 regression을 만들지 않는지

### Request-local path/source-root reuse

현재 구현에는 이미 [`../src/fs/guards.rs`](../src/fs/guards.rs)의 첫 번째 request-local reuse 정리가 들어가 있다. `RequestPathResolver`가 단일 FUSE request 범위에서 `source_root_path()` 결과를 재사용하고, mutation coordinate 검사와 opened-target revalidation에서 반복되던 resolved virtual path/source path 변환 helper를 공통화한다.

현재 evidence:

- 구현: [`../src/fs/guards.rs`](../src/fs/guards.rs) (`RequestPathResolver`, `source_root()`, `resolved_virtual_path()`, `resolved_virtual_path_for_open_file()`, `guard_mutation_coordinates()`)
- 집중 회귀 테스트: [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`mutation_under_visible_symlink_parent_uses_resolved_parent`)
- current operation/TOCTOU 근거: [`artifacts/current-toctou-hardening-evidence.md`](artifacts/current-toctou-hardening-evidence.md) (pinned `source_root`, opened-target revalidation, resolved-parent mutation path)
- mounted probe coverage 한계: [`benchmarks.md`](benchmarks.md) (`symlink_parent_mkdir_rmdir`, Current harness coverage limits)

가드레일:

- reuse 범위는 같은 request 내부로만 제한한다.
- 기존 `resolve_host_path`/`virtual_path_from_source_path` 기반 계산과 errno 매핑을 유지한다.
- cross-request symlink decision cache나 host negative cache로 확대하지 않는다.
- hidden `ENOENT`, symlink target fully-visible gate, mutability `EROFS` 판단을 cache hit만으로 생략하지 않는다.

증거 요구:

- 현재 변경은 semantics-preserving refactor/reuse로만 문서화한다.
- `scripts/bench-screenfs.py`의 `symlink_parent_mkdir_rmdir`는 이 surface 주변의 symlink-parent mutation guard/path-resolution path를 겨냥한 ScreenFS-only mounted workload/probe지만, counter/trace 없이 live FUSE request가 특정 내부 helper를 탔다고 증명하지는 못한다.
- speedup 주장은 perf counter, trace, microbenchmark, 또는 변경 surface를 겨냥한 before/after benchmark artifact가 나온 뒤에만 한다.
- broad mounted-vs-native harness 결과만으로 이 미세 최적화 효과를 단정하지 않는다.

### State lock stance

현재 병목 후보는 전역 `Mutex<State>`가 아니라 single consistency-domain `RwLock<State>`다. read-only snapshot concurrency와 cross-table atomic invalidation을 위해 현 구조를 유지하고, split은 contention evidence가 있을 때만 검토한다.

현재 evidence:

- 구현: [`../src/fs.rs`](../src/fs.rs) (`state: RwLock<State>`)
- 설계/계약 근거: [`artifacts/current-state-lock-concurrency-evidence.md`](artifacts/current-state-lock-concurrency-evidence.md), [`operations.md`](operations.md), [`design.md`](design.md)
- 회귀 테스트: [`../src/fs/tests/state_cache.rs`](../src/fs/tests/state_cache.rs) (`read_only_state_snapshots_can_run_concurrently`, `readdirplus_pins_only_returned_page_child_lookup_refs`, `successful_mutations_invalidate_parent_snapshots_and_reused_exact_paths`)
- 관련 data/sync surface 테스트: [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`read_write_offsets_do_not_depend_on_shared_file_position`, `flush_and_fsync_complete_under_compio_runtime`, `release_flush_true_completes_under_compio_runtime_and_removes_handle`)

검토 전 선행 조건:

1. state lock wait/hold time 계측
2. `readdir`/`readdirplus`와 mutation invalidation의 write-lock 유지 시간 계측
3. 이미 가능한 host IO/sync syscall lock-outside 이동이 추가로 남아 있는지 재확인

## Measure-first backlog

### 1. Policy/matcher hot path

대상:

```text
visibility_decision
mutability_decision
is_fully_visible
```

확인할 것:

- operation별 호출 빈도
- 함수별 누적/평균/percentile latency
- rule 수가 많을 때 matcher bucket/index가 실제 후보군을 충분히 줄이는지
- hidden precedence와 most-specific rule wins 계산 비용

권장 시작점:

- opt-in debug/perf counter
- rule 수와 path depth를 조합한 microbenchmark
- live smoke에 operation별 counter dump 옵션

### 2. Read/write buffer size and concurrency

`read_at/write_at`가 적용되어도 syscall은 여전히 sync path일 수 있다. workload에 따라 병목이 다르므로 benchmark가 먼저 필요하다.

측정 축:

- 작은 read/write 다량: syscall overhead 확인
- 큰 sequential read/write: backing FS latency 확인
- mixed/random IO: offset 기반 path contention 확인
- concurrent reader/writer: state lock 및 handle snapshot 비용 확인

현재 harness surface:

- `seq_read`, `seq_write`
- `small_read`, `small_write`

판단 기준:

- 작은 IO가 지배적이면 batching 또는 syscall 수 감소 후보 검토
- 큰 IO가 지배적이면 backing FS latency와 host-side offload 가치 검토
- runtime blocking이 확인될 때만 data path offload를 실험

### 3. Sync-heavy surface

현재 harness surface:

- `write_fsync_close`: repeated open/write/file-fsync/close/cleanup end-to-end probe

주의:

- parent directory `fsync`는 포함하지 않는다.
- internal offload helper attribution은 counter/trace 없이는 증명하지 못한다.
- tmpfs 결과를 storage-backed fsync evidence로 일반화하지 않는다.

### 4. Directory entry attr cost

`readdirplus`는 entry마다 metadata/attr 생성을 요구하므로 큰 directory에서 비쌀 수 있다.

측정 항목:

- `readdir` 비용
- `readdirplus` 비용
- entry당 `symlink_metadata` 비용
- attr 변환 비용
- page size별 total latency와 tail latency

권장 접근:

```text
opendir/readdir에서 state lock으로 inode/path만 짧게 확인
host read_dir + metadata 수집은 lock 밖에서 수행
결과 반영 시에만 state lock 재획득
```

주의:

- hidden entry filtering 결과가 바뀌면 안 된다.
- symlink target visibility check를 유지한다.
- host `read_dir`/metadata 수집을 state lock 밖으로 옮겨도 기존 dirfd/openat2-confined access 경계를 유지해야 하며, 문자열 path 재조합 기반의 unconstrained path walk로 바꾸면 안 된다.
- directory snapshot invalidation과 충돌하면 안 된다.

### 5. open_confined/openat2 call frequency

`open_confined/openat2`는 confinement correctness 핵심이라 대체하지 않는다. 대신 호출 빈도와 latency를 측정한다.

대상 workload:

- lookup/open/access/stat 반복
- shell completion/stat-heavy workload
- build tool의 많은 작은 파일 접근
- symlink-heavy path 접근

판단 기준:

- openat2 자체가 병목인지
- policy/matcher 또는 path resolution이 병목인지
- inode/path cache 재사용이 충분한지

### 6. Metadata path profiling

대상:

```text
lookup
getattr
readlink
access
statfs
```

이들은 호출 빈도가 높지만 visibility, symlink, confinement와 밀접하다. 무작정 async/offload하지 말고 특정 함수가 병목으로 확인될 때만 검토한다.

### 7. Mutation invalidation breadth

mutation 후 `invalidate_after_mutation()`이 필요한 범위보다 넓게 지우면 inode/path cache 재생성 비용이 커질 수 있다.

확인할 것:

- mutation 종류별 invalidation 범위
- parent/child/sibling까지 지워지는지
- directory snapshot 재생성 빈도
- repeated lookup/getattr에서 cache miss 증가 여부

개선 방향:

- mutation별 최소 invalidation 범위 문서화
- perf counter로 invalidation count와 evicted entry 수 기록
- correctness test로 stale visibility/mutability 상태가 남지 않는지 검증

## Deferred or high-risk candidates

### Negative/hidden path repeated access cache

shell/build 도구가 같은 hidden path를 반복 stat할 수 있다.

위험:

- negative cache는 rename/create/unlink와 invalidation이 어렵다.
- hidden/visible policy 변경 또는 ancestor 변화와 충돌할 수 있다.
- 잘못 구현하면 존재 여부 또는 policy 상태를 노출할 수 있다.

검토 가능한 제한안:

- 매우 짧은 TTL
- policy-only 결정으로 제한
- host FS negative 결과와 섞지 않기
- mutation 후 즉시 폐기
- 기본 off, debug/experimental로 시작

### State lock split

`RwLock<State>` split은 계측 전까지 defer한다.

주의:

- lock ordering 추가 없이 per-table lock으로 바로 나누지 않는다.
- mutation invalidation atomicity를 깨지 않는다.
- hidden `ENOENT` precedence, bridge-visible 의미론, page-local `readdirplus` lookup-ref pinning을 바꾸지 않는다.

### Host-side io_uring data path

프로젝트 목표의 핵심은 우선 FUSE transport의 `FUSE_OVER_IO_URING`이다. host filesystem IO 전체를 io_uring으로 바꾸는 것은 별도 장기 과제다.

적절한 범위:

```text
read
write
possibly fsync/fallocate/copy_file_range
```

비추천 범위:

```text
lookup/getattr/readlink/opendir/readdir
mutation/xattr/open_confined/openat2 대체
userspace path resolution cache로 confinement 대체
```

진입 조건:

- data path benchmark에서 명확한 병목이 확인될 것
- openat2로 얻은 fd를 안전하게 io_uring/compio file path에 연결할 수 있을 것
- 기존 fd-based confinement와 hidden `ENOENT` 의미론이 유지될 것

## Instrumentation status and backlog

현재 `--features perf-counters`로 빌드한 binary에서 `perf.enabled: true` config로 켤 수 있는 opt-in counter:

- FUSE operation별 `fuse_op.<operation>` count/latency
- policy decision count/latency와 matcher candidate count
- state lock read/write wait/hold count/latency
- `open_confined_openat2` count/latency
- `resolved_virtual_path` count/latency
- `read_size_bucket.<bucket>` / `write_size_bucket.<bucket>` count/latency
- `readdir_attr_generation_scan` count/latency plus `readdir_attr_generation_entries` scanned entry count
- `readdirplus_attr_generation_scan` count/latency plus `readdirplus_attr_generation_entries` scanned entry count
- mutation invalidation count, invalidated entries, evicted entries

출력은 ScreenFS 종료 시 stderr summary다. 이 summary는 benchmark/smoke run의 내부 attribution 보조 evidence이며, 단독 performance claim 근거가 아니다. 기본 build에는 instrumentation code/config surface가 포함되지 않는다. `scripts/bench-screenfs.py --perf-counters --build`는 `--features perf-counters`로 빌드하고 임시 config로 `perf.enabled`를 켜며, 종료 후 summary를 JSON `screenfs.perf_summary`와 Markdown report에 기록한다.

남은 후보 counter:

- matcher candidate를 matcher family별로 더 세분화
- histogram 또는 benchmark artifact 저장 형식의 추가 구조화

## Measurement-guided optimization notes

Perf-enabled benchmark evidence should drive optimization order. The smoke baseline summarized in [`artifacts/current-perf-counter-baseline-summary.md`](artifacts/current-perf-counter-baseline-summary.md) showed `resolved_virtual_path` as a high-attribution helper (`count=145166`, `total_ns=394848335`) and `policy_decision` as another broad hot path (`count=171732`, `total_ns=183484872`) compared with `open_confined_openat2` (`count=98298`, `total_ns=47566036`). Low-risk follow-ups keep `ScreenFs::resolved_virtual_path()` on the same single-`source_root_path()` pattern already used by `RequestPathResolver`, and avoid recomputing visible-descendant bridge checks after `visibility_decision()` has already returned `BridgeVisible`. Both preserve `resolve_host_path`/`virtual_path_from_source_path` errno and visibility semantics. The smoke before/after artifact does not support a user-visible speedup claim; treat these as attribution-guided cleanup only, and require claim-grade before/after samples before claiming performance improvement. The checked-in current smoke artifacts are [`artifacts/current-perf-counter-benchmark-result.json`](artifacts/current-perf-counter-benchmark-result.json), [`artifacts/current-perf-counter-benchmark-result.md`](artifacts/current-perf-counter-benchmark-result.md), and [`artifacts/current-perf-counter-benchmark-result.svg`](artifacts/current-perf-counter-benchmark-result.svg).

## Recommended order

```text
1. 문서/상태 정합성을 유지한다
2. opt-in perf counter와 benchmark surface를 확장한다
3. policy/matcher hot path와 state lock hold time을 계측한다
4. read/write buffer size와 concurrency benchmark를 보강한다
5. request-local path/source-root reuse를 mounted probe workload와 before/after 비교로 계측한다
6. readdir vs readdirplus attr 비용과 open_confined/openat2 호출 빈도/latency를 측정한다
7. invalidate_after_mutation 범위와 evicted entry 수를 측정한다
8. evidence가 쌓인 뒤에만 negative/hidden path cache, state lock split, host-side io_uring를 검토한다
```

## Non-goals

- `openat2` confinement를 일반 `open/openat` 또는 고수준 async open으로 대체
- symlink visibility decision을 cross-request cache로 생략
- hidden path를 `Permission denied`로 노출
- `/dev/null` bind overlay, tmpfs masking, 빈 파일 overlay처럼 이름을 남기는 masking
- recursive background scan/index로 visibility bridge를 추론
- lock을 잡은 채 blocking IO 또는 `spawn_blocking` 실행
- benchmark 없이 metadata/mutation/xattr path를 대규모 async화
