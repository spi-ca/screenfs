# ScreenFS 성능 개선 포인트 정리

이 문서는 ScreenFS의 현재 성능 개선 후보를 위험도와 우선순위 기준으로 정리한다. 전제는 기존 correctness 계약을 유지하는 것이다.

## 기본 원칙

- `visibility.hidden`은 계속 `ENOENT`로 처리하고, `mutability`의 `EROFS`보다 우선한다.
- `openat2(RESOLVE_IN_ROOT | RESOLVE_NO_MAGICLINKS)` 기반 confinement는 correctness 핵심이므로 성능 목적으로 대체하지 않는다.
- symlink visibility 판단을 요청 간 공유 cache로 생략하지 않는다.
- state lock을 잡은 채 blocking IO 또는 `spawn_blocking`을 호출하지 않는다.
- 최적화는 benchmark/perf counter로 병목을 확인한 뒤 적용한다.

## 현재 적용/의존 중인 영역

### FUSE transport

```text
kernel <-> ScreenFS
FUSE_OVER_IO_URING
```

`fractal-fuse = 0.4.0`의 FUSE3/io_uring transport는 프로젝트 목표와 일치한다. 이 영역은 유지하고, mount/live smoke에서 협상 여부와 기본 operation 정상 동작을 계속 검증한다.

효과:

- FUSE request/reply transport 병목 완화
- async dispatch 기반 유지
- v1 필수 목표와 정합

## 우선순위 높은 개선 후보

### 1. policy/matcher hot path 계측

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

- debug/perf counter 추가
- rule 수와 path depth를 조합한 micro benchmark 추가
- live smoke에 operation별 counter dump 옵션 추가

### 2. 현재 상태: offset 기반 read/write는 이미 구현됨

`read`/`write`는 이미 `src/fs.rs`의 `read()`/`write()`에서 `FileExt::read_at` / `FileExt::write_at`를 사용한다. `seek + read/write` 전환 자체는 더 이상 TODO가 아니고, 현재 남은 일은 이 경로의 비용을 benchmark/perf counter로 계측하는 것이다.

현재 근거:

- 구현: `src/fs.rs` (`read()`, `write()`)
- 회귀 테스트: `src/fs/tests/data_mutations.rs` (`read_write_offsets_do_not_depend_on_shared_file_position`)
- 상태 잠금 근거: `docs/artifacts/current-state-lock-concurrency-evidence.md` (`Lock rules`, `Low-risk executor-offload boundary`)

다음 질문:

- 작은/큰 IO에서 syscall 비용이 어디서 커지는지
- concurrent reader/writer에서 handle snapshot 비용이 실제 병목인지
- host-side offload 또는 io_uring 검토가 필요한 수준의 data-path 병목이 있는지

### 3. read/write 버퍼 크기별 benchmark

`read_at/write_at`가 적용되어도 syscall은 여전히 sync path일 수 있다. workload에 따라 병목이 다르므로 benchmark가 먼저 필요하다.

측정 축:

- 작은 read/write 다량: syscall overhead 확인
- 큰 sequential read/write: backing FS latency 확인
- mixed/random IO: offset 기반 path의 contention 확인
- concurrent reader/writer: state lock 및 handle snapshot 비용 확인

판단 기준:

- 작은 IO가 지배적이면 batching 또는 syscall 수 감소 후보 검토
- 큰 IO가 지배적이면 backing FS latency와 `spawn_blocking` 가치 검토
- runtime blocking이 확인될 때만 data path offload를 실험

### 4. 현재 상태: flush/fsync/release(flush) sync syscall 분리는 이미 구현됨

`flush`, `fsync`, `release(flush=true)`는 이미 `src/fs.rs`의 `flush()`/`release()`/`fsync()`에서 file-handle snapshot/removal 뒤 state lock 밖으로 sync syscall을 offload한다. 따라서 이 항목의 다음 단계는 구현 자체가 아니라 close/fsync-heavy workload에서의 queueing/tail-latency 계측이다.

현재 근거:

- 구현: `src/fs.rs` (`flush()`, `release()`, `fsync()`, `offload_file_sync()`)
- 회귀 테스트: `src/fs/tests/data_mutations.rs` (`flush_and_fsync_complete_under_compio_runtime`, `release_flush_true_completes_under_compio_runtime_and_removes_handle`, `offload_file_sync_*`)
- 설계/락 규칙: `docs/artifacts/current-state-lock-concurrency-evidence.md` (`Lock rules`, `Low-risk executor-offload boundary`)

관찰 포인트:

- close/fsync-heavy workload에서 blocking pool saturation/backpressure
- sync surface별 latency tail
- errno/join failure mapping이 관측 가능한 regression을 만들지 않는지

## 중간 우선순위 개선 후보

### 5. 현재 상태: 첫 request-local path/source-root reuse는 구현됨

`src/fs/guards.rs`에는 첫 번째 request-local reuse 정리가 이미 들어갔다. `RequestPathResolver`가 단일 FUSE request 범위에서 `source_root_path()` 결과를 재사용하고, mutation coordinate 검사와 opened-target revalidation에서 반복되던 resolved virtual path/source path 변환 helper를 공통화한다.

의미론 가드레일:

- reuse 범위는 같은 request 내부로만 제한한다
- 기존 `resolve_host_path`/`virtual_path_from_source_path` 기반 계산과 errno 매핑을 유지한다
- cross-request symlink decision cache나 host negative cache로 확대하지 않는다
- hidden `ENOENT`, symlink target fully-visible gate, mutability `EROFS` 판단을 cache hit만으로 생략하지 않는다

남은 질문:

- `scripts/bench-screenfs.py`의 `symlink_parent_mkdir_rmdir`처럼 이 surface 주변의 symlink-parent mutation guard/path-resolution path를 겨냥한 mounted workload/probe에서 path/source-root 재계산 횟수가 실제로 얼마나 줄었는지
- allocation/normalization 감소가 wall-clock latency에 의미 있는지
- 추가 reuse 지점을 넓힐 가치가 있는지

증거 요구:

- 현재 변경은 semantics-preserving refactor/reuse로만 문서화한다
- `scripts/bench-screenfs.py`의 `symlink_parent_mkdir_rmdir`는 현재 request-local reuse surface 주변의 symlink-parent mutation guard/path-resolution path를 겨냥한 ScreenFS-only mounted workload/probe로 볼 수 있지만, counter/trace 없이 live FUSE request가 특정 내부 helper를 탔다고 증명하지는 못한다
- speedup 주장은 perf counter, trace, microbenchmark, 또는 변경 surface를 겨냥한 before/after benchmark artifact가 나온 뒤에만 한다
- broad mounted-vs-native harness 결과만으로 이 미세 최적화 효과를 단정하지 않는다

### 6. directory entry attr 생성 비용 분리

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

- hidden entry filtering 결과가 바뀌면 안 됨
- symlink target visibility check 유지
- directory snapshot invalidation과 충돌하면 안 됨

### 7. open_confined/openat2 호출 빈도 분석

`open_confined/openat2`는 confinement correctness 핵심이라 대체는 비추천이다. 대신 호출 빈도와 latency를 측정한다.

대상 workload:

- lookup/open/access/stat 반복
- shell completion/stat-heavy workload
- build tool의 많은 작은 파일 접근
- symlink-heavy path 접근

판단 기준:

- openat2 자체가 병목인지
- policy/matcher 또는 path resolution이 병목인지
- inode/path cache 재사용이 충분한지

### 8. metadata path는 profiling 후 제한적으로 검토

대상:

```text
lookup
getattr
readlink
access
statfs
```

이들은 호출 빈도가 높지만 visibility, symlink, confinement와 밀접하다. 무작정 async/offload하지 말고 특정 함수가 병목으로 확인될 때만 검토한다.

## 신중히 검토할 후보

### 9. negative/hidden path 반복 접근 최적화

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

### 10. state invalidation 범위 점검

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

### 11. state lock 세분화는 계측 전까지 defer

현재 병목 후보는 전역 `Mutex<State>`가 아니라 single consistency-domain `RwLock<State>`다 (`src/fs.rs`의 `state` 필드, `docs/artifacts/current-state-lock-concurrency-evidence.md`). read-only snapshot concurrency와 cross-table atomic invalidation을 위해 현 구조를 유지하고, split은 contention evidence가 있을 때만 검토한다.

현재 근거:

- 구현: `src/fs.rs` (`state: RwLock<State>`)
- 설계 근거: `docs/artifacts/current-state-lock-concurrency-evidence.md` (`Selected low-risk direction`, `Lock rules`)
- 회귀 테스트: `src/fs/tests/state_cache.rs` (`readdirplus_dot_entries_do_not_pin_lookup_refs`, `forget_evicts_non_root_mapping_after_lookup_refs_drop_and_handles_close`, `readdirplus_pins_returned_child_lookup_refs`, `readdirplus_honors_size_budget_and_continues_from_last_cookie`)

검토 전 선행 조건:

1. state lock wait/hold time 계측
2. `readdir`/`readdirplus`와 mutation invalidation의 write-lock 유지 시간 계측
3. 이미 가능한 host IO/sync syscall lock-outside 이동이 추가로 남아 있는지 재확인

주의:

- lock ordering 추가 없이 per-table lock으로 바로 나누지 않는다
- mutation invalidation atomicity를 깨지 않는다
- hidden `ENOENT` precedence, bridge-visible 의미론, page-local `readdirplus` lookup-ref pinning을 바꾸지 않는다

## 장기/선택 후보

### 12. io_uring 기반 host data path 실험

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

판단 기준:

- data path benchmark에서 명확한 병목이 확인될 것
- openat2로 얻은 fd를 안전하게 io_uring/compio file path에 연결할 수 있을 것
- 기존 fd-based confinement와 hidden ENOENT 의미론이 유지될 것

## perf evidence 보강

### 13. mount/live smoke에 perf counters 추가

현재 correctness evidence에 비해 perf evidence가 약하다. live smoke에 operation별 count/latency histogram이 있으면 다음 최적화 우선순위가 명확해진다.

권장 counter:

- FUSE operation별 count/latency
- policy decision count/latency
- matcher candidate count
- `resolved_virtual_path` 호출 수/latency
- `open_confined/openat2` 호출 수/latency
- `read_at/write_at` size bucket별 count/latency
- `readdir`/`readdirplus` entry count와 attr 생성 시간
- invalidation count와 evicted entry 수
- state lock wait/hold time

출력 방식 후보:

- debug log summary
- test-only perf dump
- feature-gated histogram
- benchmark artifact로 저장

## 추천 적용 순서

```text
1. 문서/상태 정합성부터 맞춘다
2. opt-in perf counter와 benchmark surface를 확장하되, request-local reuse는 `scripts/bench-screenfs.py`의 `symlink_parent_mkdir_rmdir` 같은 mounted probe workload도 포함해 본다
3. policy/matcher hot path와 state lock hold time을 계측한다
4. read/write buffer size와 concurrency benchmark를 추가한다
5. 이미 들어간 request-local path/source-root reuse를 `symlink_parent_mkdir_rmdir` 같은 mounted probe workload의 before/after 비교로 계측하고 추가 reuse 지점을 측정한다
6. readdir vs readdirplus attr 비용과 open_confined/openat2 호출 빈도/latency를 측정한다
7. invalidate_after_mutation 범위와 evicted entry 수를 측정한다
8. 위 evidence가 쌓인 뒤에만 negative/hidden path cache, state lock split, host-side io_uring를 검토한다
```

## 피해야 할 최적화

- `openat2` confinement를 일반 `open/openat` 또는 고수준 async open으로 대체
- symlink visibility decision을 cross-request cache로 생략
- hidden path를 `Permission denied`로 노출
- `/dev/null` bind overlay, tmpfs masking, 빈 파일 overlay처럼 이름을 남기는 masking
- recursive background scan/index로 visibility bridge를 추론
- lock을 잡은 채 blocking IO 또는 `spawn_blocking` 실행
- benchmark 없이 metadata/mutation/xattr path를 대규모 async화
