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

### 2. data path 정리: offset 기반 read/write

현재 `seek + read/write` 형태는 file cursor 공유와 concurrent request에 불리하다.

우선 변경 후보:

```text
seek + read/write
-> FileExt::read_at / FileExt::write_at
```

기대 효과:

- file cursor 공유 문제 제거
- concurrent read/write 안정성 개선
- 이후 `spawn_blocking` 또는 io_uring data path 실험이 쉬워짐

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

### 4. flush/fsync/release flush blocking 분리

대상:

```text
flush
fsync
release(flush=true)
```

이들은 backing FS 상태에 따라 오래 block될 수 있다.

권장 패턴:

```text
1. state lock에서 필요한 handle/file clone만 확보
2. state lock 해제
3. sync_all/fsync/fdatasync 수행
4. 필요 시 state lock 재획득 후 상태 갱신
```

주의:

- lock을 잡은 채 `spawn_blocking` 금지
- threadpool saturation/backpressure 필요
- errno 변환을 공통 helper로 통일

## 중간 우선순위 개선 후보

### 5. resolved_virtual_path / host_path 비용 절감

symlink-heavy workload에서 `resolved_virtual_path()`와 `host_path` 계산이 반복될 수 있다.

허용 가능한 방향:

- single-request 내부에서 이미 계산한 resolved path 재사용
- 같은 request 안에서 visibility/mutability 판단에 필요한 path 변환 중복 제거
- path allocation/normalization 횟수 계측

금지/주의:

- cross-request symlink decision cache는 현재 계약상 위험하므로 금지
- symlink target visibility check를 cache hit만으로 생략하지 않는다
- rename/create/unlink 이후 invalidation 문제를 단순 cache로 우회하지 않는다

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

### 11. state lock 세분화

현재 전역 `Mutex<State>`가 병목일 수 있다.

가능한 분리 축:

```text
inode map lock
file handle lock
directory handle lock
policy/cache lock
```

주의:

- lock ordering 규칙 필요
- mutation invalidation 복잡도 증가
- deadlock과 stale state 리스크 증가

권장 순서:

1. lock hold time 계측
2. data/directory blocking IO를 lock 밖으로 이동
3. 그래도 contention이 확인될 때만 세분화 검토

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
1. perf counter/benchmark 기반 만들기
2. policy/matcher hot path와 state lock hold time 계측
3. read/write buffer size별 benchmark 추가
4. resolved_virtual_path/host_path single-request reuse 지점 확인
5. readdir vs readdirplus 비용 분리 측정
6. open_confined/openat2 호출 빈도와 latency 측정
7. invalidate_after_mutation 범위와 evicted entry 수 확인
8. 병목 확인 후 data path offload 또는 directory snapshot 분리 검토
9. 필요할 때만 negative/hidden path TTL cache와 state lock 세분화 실험
10. data path 병목이 명확할 때만 host io_uring 실험
```

## 피해야 할 최적화

- `openat2` confinement를 일반 `open/openat` 또는 고수준 async open으로 대체
- symlink visibility decision을 cross-request cache로 생략
- hidden path를 `Permission denied`로 노출
- `/dev/null` bind overlay, tmpfs masking, 빈 파일 overlay처럼 이름을 남기는 masking
- recursive background scan/index로 visibility bridge를 추론
- lock을 잡은 채 blocking IO 또는 `spawn_blocking` 실행
- benchmark 없이 metadata/mutation/xattr path를 대규모 async화
