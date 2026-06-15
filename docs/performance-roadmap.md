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

### Per-open read/write handle cache (implemented conservative fast path)

현재 상태:

- 초기 conservative per-open handle cache는 이미 열린 regular-file handle의 `read`/`write` fast path에 구현돼 있다.
- handle에는 `read`/`write` fast path용 cache bit만 붙고, cache hit으로 반복 guard를 생략하는 범위는 cache-eligible `visible`/`writable` policy로 제한된다. hidden carve-out, visible carve-out, readonly/writable carve-out, symlink target revalidation처럼 current-path proof가 다시 필요한 policy shape는 계속 기존 per-I/O guard를 재실행한다.
- scope는 handle-local fast path뿐이다. broad path cache, global authorization cache, negative cache, symlink decision cache, stable listing cache는 포함하지 않는다.
- data I/O는 기존 opened fd로 계속 수행하고 cached path로 reopen하지 않는다.
- `fallocate`와 `copy_file_range`는 현재 구현 범위 밖이며 계속 per-call guard path를 탄다.

현재 evidence:

- 구현 source: [`../src/fs.rs`](../src/fs.rs) (`open_file_io_guard_cache()`, `open()`, `read()`, `write()`), [`../src/fs/state.rs`](../src/fs/state.rs) (`FileIoGuardCache`, opened file-handle snapshot state)
- supplemental managed attribution: [`artifacts/managed-fio-attribution-summary.md`](artifacts/managed-fio-attribution-summary.md), [`artifacts/managed-fio-attribution-perf-split.json`](artifacts/managed-fio-attribution-perf-split.json), [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json). 이 rerun은 non-root `fusermount3` mount에서 `/hidden` + `/readonly` carve-out을 쓰는 `custom-unsafe-policy` / `managed-fio-root-carveouts` shape와 `current-fio-attribution.job` fio workload를 사용한 warm-cache attribution이며, official harness `fallback-unsafe-policy` preset도 claim-grade before/after pair도 아니다.
- 현재 claim-grade before/after artifact: [`artifacts/per-open-cache-claim/summary.md`](artifacts/per-open-cache-claim/summary.md)와 companion [`artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json`](artifacts/per-open-cache-claim/before-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json), [`artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json`](artifacts/per-open-cache-claim/after-worktree-03bfdeb82ced-fast-path-cache-eligible-per-open-cache-minimum.json), 대응 Markdown/SVG/PNG. 이 pair는 `--policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum --iterations 10 --warmups 3`를 사용했고, JSON provenance에 ScreenFS source provenance, dirty worktree 상태, `fusermount3`/kernel/filesystem 메타데이터가 들어 있다.
- 현재 smoke artifact: [`artifacts/current-fio-per-open-cache-summary.md`](artifacts/current-fio-per-open-cache-summary.md)와 companion [`artifacts/current-fio-per-open-cache-native.json`](artifacts/current-fio-per-open-cache-native.json), [`artifacts/current-fio-per-open-cache-screenfs.json`](artifacts/current-fio-per-open-cache-screenfs.json), [`artifacts/current-fio-per-open-cache-env.json`](artifacts/current-fio-per-open-cache-env.json), [`artifacts/current-fio-per-open-cache-perf-split.json`](artifacts/current-fio-per-open-cache-perf-split.json), [`artifacts/current-fio-per-open-cache-screenfs.stderr.log`](artifacts/current-fio-per-open-cache-screenfs.stderr.log), [`artifacts/current-fio-per-open-cache-screenfs-fstype.txt`](artifacts/current-fio-per-open-cache-screenfs-fstype.txt). 이들은 split counter smoke 확인용으로만 읽고 claim-grade pair와 섞지 않는다.
- 현재 official harness는 `--policy-preset`, `--policy-label`, `--workload-set`를 제공하지만, 기본 mounted policy는 여전히 hidden/readonly rule을 함께 주입하는 `fallback-unsafe-policy`다. per-open cache claim-grade 비교에서는 `fast-path-cache-eligible` preset과 별도 named unsafe matrix entry를 분리해서 사용해야 한다.
- managed contrib helper provenance는 [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json)에 기록돼 있다.
- focused regression/perf tests: [`../src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) (`perf_counters_record_data_path_splits_on_success`, `perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe`, `perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures`, `perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path`)
- focused correctness tests: [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`cache_eligible_opened_file_read_write_keep_pinned_fd_after_host_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_ancestor_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_unlink`, `opened_file_read_keeps_pinned_fd_after_host_rename_but_write_fails_closed`, `opened_file_read_write_fail_closed_after_host_rename_into_hidden_subtree`), [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`cache_eligible_opened_symlink_read_keeps_pinned_fd_after_final_retarget`, `cache_eligible_opened_symlink_read_keeps_pinned_fd_after_ancestor_retarget`, `opened_symlink_read_revalidates_hidden_target_after_retarget`), [`../src/fs/tests/mutability.rs`](../src/fs/tests/mutability.rs) (`opened_symlink_write_revalidates_readonly_target_after_retarget`)

지속 제약:

- hidden `ENOENT` precedence, readonly `EROFS`, symlink target revalidation, source-root confinement/`openat2` proof를 유지한다.
- cache hit은 policy shape가 hidden/readonly/symlink-target authorization concern을 제거한 cache-eligible default `visible`/`writable` case로만 제한한다. 이 case에서는 already-open fd에 대해 documented POSIX pinned-fd lifetime semantics를 유지하고 cached path로 reopen하지 않는다.
- hidden/visible carve-out, readonly/writable carve-out, symlink target authorization concern이 있는 unsafe policy shape에서는 cache hit이 current-path proof를 대체하지 않으며 기존 per-I/O fail-closed revalidation path를 계속 사용한다.
- documented POSIX fd lifetime boundary를 강화하거나 약화하지 않는다.

남은 질문:

- 이 구현에 대한 user-visible speedup 설명은 여전히 [`benchmarks.md`](benchmarks.md)의 claim-grade before/after pair와 별도 correctness validation(`cargo test --all-targets --all-features` 등) 없이는 할 수 없다.
- claim-grade artifact는 `current-fio-per-open-cache-*`를 덮어쓰지 말고 `before|after + rev + policy-preset + workload-set` provenance를 분리한 이름/layout으로 남겨야 한다. 현재 checked-in pair는 [`artifacts/per-open-cache-claim/`](artifacts/per-open-cache-claim/) 아래 그 규칙을 따른다.
- 첫 before/after matrix의 최소 workload set은 harness `--workload-set per-open-cache-minimum`(`rand_read_4k`, `rand_write_4k`, `sync_write_4k`, `small_open_read_close`)이고, 이후 open/stat-heavy, `readdir`/`readdirplus`, policy-heavy case로 확장해야 한다.
- cache-eligible policy와 fallback policy 각각에서 small-I/O/tail-latency variance가 어떻게 나타나는지
- 현재 focused coverage 외에 더 복잡한 multi-actor mounted workload에서 추가 artifact나 회귀 테스트를 더 남길 필요가 있는지

### Active next candidate: metadata/open-path fixed overhead

per-open cache claim-grade pair가 이미 체크인된 현재 기준에서, 다음 implementation candidate는 `lookup`/`getattr`/`open`/`readlink`/`access` 중심의 metadata/open-path fixed overhead 축소다. `statfs`는 improvement target이라기보다 non-regression guardrail로 유지한다. 현재 `small_stat_open_read`만으로는 이 surface를 claim-grade로 분리할 수 없으므로, 구현이나 측정 주장보다 먼저 split workload contract와 artifact naming을 고정한다.

문서 gate와 acceptance 기준:

- claim-grade 측정 전제는 [`benchmarks.md`](benchmarks.md)의 새 `metadata/open-path fixed overhead` contract다. 현재 harness는 `metadata_lookup`, `metadata_getattr`, `metadata_open`, `metadata_readlink`, `metadata_access`, `metadata_statfs`, `sync_flush_only`, `sync_fsync_only`, `sync_release_flush`, `readdir_basic`, `readdirplus_basic` workload names를 제공한다. 향후 이 중 하나라도 실제 path를 분리하지 못하면 먼저 harness gap으로 기록하고, 그 전까지는 exploratory smoke만 허용한다.
- artifact는 `docs/artifacts/metadata-open-path-claim/` 아래 `before|after-<rev>-<policy-label>-<workload>.*`와 `env-before|after-...json` 형식으로 남긴다. native/passthrough/ScreenFS raw-lat floor artifact도 같은 stem을 공유하고 side suffix를 붙인다.
- policy matrix는 최소 `fast-path-cache-eligible`, `fallback-unsafe-policy`, 그리고 explicit `--policy-label` + `--matcher-extra-rules >= 32`를 포함한 readonly/carve-out-heavy row를 요구한다. `--matcher-extra-rules`를 썼으면 policy label과 filename stem에 `matcher<N>`를 포함해 matcher-rich/read-only-heavy provenance가 사람이 읽히도록 남긴다.
- acceptance는 target policy row에서 primary metadata workloads 5개 중 최소 4개가 after/before `<= 0.90x` median, `<= 0.95x` p95/p99를 만족하고, 남은 1개와 `metadata_statfs`는 `<= 1.05x` median / `<= 1.10x` p99 안에 머무를 때만 통과로 읽는다.
- sync surface는 `write_fsync_close` 하나로 대체하지 않는다. `flush`, `fsync`, `release(flush=true)`는 각각 `sync_flush_only`, `sync_fsync_only`, `sync_release_flush`로 분리해 p95/p99까지 기록하고, metadata/open-path candidate 단계에서는 각 workload가 `<= 1.05x` median / `<= 1.10x` p95/p99 non-regression budget 안에 남아야 한다.
- `readdir`와 `readdirplus`도 분리해서 본다. `readdirplus` acceptance 또는 non-regression 판정은 `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, `readdirplus_page_commit`이 모두 non-zero일 때만 유효하다. `readdir` acceptance 또는 non-regression 판정도 `fuse_op.readdir`가 0이거나 대응 `readdir_*` split counter가 모두 0이면 workload miss로 처리하고, [`benchmarks.md`](benchmarks.md)의 source-of-truth gate와 동일하게 판단한다.
- native/passthrough/ScreenFS floor comparison은 같은 raw-sample metric을 써야 한다. 현재 fio supplemental raw-lat 계열을 refresh할 때도 `clat` raw samples를 세 side 모두에서 유지하고, percentile bucket이나 `lat` 혼합으로 fixed-overhead gap을 주장하지 않는다.

현재 측정 plumbing/evidence:

- next-candidate measurement bundle: [`artifacts/metadata-open-path-claim/summary.md`](artifacts/metadata-open-path-claim/summary.md)와 companion JSON/Markdown/SVG/PNG. 이 bundle은 현재 worktree의 metadata/open-path, sync surface, directory surface, policy-heavy matrix row를 측정한 attribution evidence이며 before/after optimization claim은 아니다.
- coverage caveat: checked-in fast-path-cache-eligible row에는 metadata/open-path와 directory surface evidence가 있지만, 현재 policy-heavy matcher32 row는 `metadata_lookup`, `metadata_getattr`, `metadata_access`, `matcher_hidden_stat_miss`만 포함한다. 따라서 unsafe-policy row에서 `metadata_open`, `metadata_readlink`, `readdir_basic`, `readdirplus_basic`까지 같은 강도로 입증됐다고 쓰지 않는다. 해당 workload가 필요하면 먼저 matrix row를 채우거나 gap으로 남긴다.
- matcher caveat: matcher32 artifact의 현재 rule-rich 신호는 subtree-heavy synthetic probe에서 `matcher_candidate_order.path` 호출량과 path-order ancestor/seen-slot work를 보여주는 attribution이다. 이 row의 `matcher_candidate_order.descendant`는 `visible-rules=0`인 empty visible matcher 호출 비용이므로 rule-rich descendant candidate-order 근거로 쓰지 않는다. `matcher_candidate_order_duplicates`가 0인 현 artifact만으로 duplicate-removal 비용 또는 broad matcher-family 재배치 효과를 주장하지 않는다.
- 현재 mounted `sync_release_flush` workload는 kernel이 `release(flush=true)`를 실제로 주지 않아 `file_sync.release_flush`를 태우지 못한 known measurement gap으로 남기고, 해당 counter 자체는 focused Rust perf test로 검증한다.
- raw three-way FUSE fixed-overhead floor: [`artifacts/rawlat-three-way/summary.md`](artifacts/rawlat-three-way/summary.md), [`artifacts/rawlat-three-way/boxplot-stats.json`](artifacts/rawlat-three-way/boxplot-stats.json), [`artifacts/rawlat-three-way/boxplot.svg`](artifacts/rawlat-three-way/boxplot.svg), [`artifacts/rawlat-three-way/boxplot.png`](artifacts/rawlat-three-way/boxplot.png). 이 artifact는 native/passthrough/ScreenFS 모두에서 fio raw `clat` sample metric을 사용한다.

이 candidate는 correctness guardrail을 바꾸지 않는다. hidden `ENOENT`, bridge-visible ancestor semantics, symlink target fully-visible gate, `openat2` confinement, readonly `EROFS`, stable resume cookie/shared cookie domain, returned-page-only `readdirplus` lookup-ref pinning은 그대로 유지한다. metadata parent dirfd나 resolved-path 결과를 cross-request cache/pool로 재사용하지 않고, request-local reuse라도 mutation 직전 opened-target revalidation을 유지한다.

#### Partial step: lookup/getattr same-path no-follow attr reuse

첫 implementation slice는 `lookup`/`getattr`에서 같은 `VirtualPath`에 대해 `stat_child_no_follow(path, ino)`를 두 번 호출하는 fixed overhead를 줄이는 것이다. 이 단계는 전체 metadata/open-path candidate의 partial step이며, 직접 효과는 `metadata_lookup`과 `metadata_getattr` 중심으로 판단한다. 전체 candidate acceptance의 primary 5개 중 4개 개선 기준은 유지하되, 이 slice만으로 `metadata_open`, `metadata_readlink`, `metadata_access`까지 완료됐다고 주장하지 않는다.

구현 계약:

- `reply_entry_for_path()`와 `attr_for_path()`는 먼저 같은 path의 no-follow `stat_child_no_follow(path, ino)`를 얻고, 그 성공한 attr를 `guard_hidden_path`/`guard_read_path` 계열에 known child attr로 전달해 visibility 판단에서 같은 stat를 반복하지 않는다.
- known attr API는 전제를 이름이나 타입에 드러낸다. 예: `guard_read_path_with_known_attr(path, known_child_attr)` 또는 `guard_hidden_path(path, known_child_attr: Option<&FileAttr>)`.
- 주입 가능한 attr는 반드시 같은 `VirtualPath`에 대한 `stat_child_no_follow()` 결과여야 한다. symlink target resolved attr, opened fd attr, parent dir attr, 다른 child path attr는 child path guard에 재사용하지 않는다.
- no-follow attr는 final component의 type/readability 판단에만 재사용하고, symlink target fully-visible gate, ancestor symlink 해소, source-root confinement, opened-fd target revalidation, mutability 판단을 대체하지 않는다.
- injected stat가 없거나 stat 수집이 실패한 경로에서는 기존 `visible_for_entry()` fallback 의미론을 유지한다. stat 실패를 새 hidden `ENOENT` 근거로 승격하지 않고, 기존 `is_fully_visible()`/bridge-visible 판단과 후속 host syscall errno 흐름을 유지한다.
- `guard_read_path(path)`의 기존 호출부는 안전한 no-known-attr 경로를 유지하거나 전용 helper로 분리한다.
- `fallocate`/`copy_file_range` 같은 opened-fd mutator/data movement path는 attr injection으로 우회하지 않으며, per-call guard와 opened-target revalidation 요구를 계속 별도 guardrail로 유지한다.

측정/증거 요구:

- 정적 diff로 `reply_entry_for_path()`/`attr_for_path()`의 같은-path double stat 제거를 확인한다.
- perf attribution에는 `stat_child_no_follow` 또는 인접 helper 호출/latency counter, trace, focused test 중 하나를 추가해 double stat 제거를 직접 설명한다. whole-workload latency만으로 helper 감소를 단정하지 않는다.
- before/after benchmark는 최소 `metadata_lookup`, `metadata_getattr`를 포함하고, 가능하면 `metadata_open`, `metadata_access`도 non-regression/context로 함께 기록한다.
- policy-first guard 재구성은 이 slice 이후 남는 비용을 perf counter로 확인한 뒤 별도 후속 최적화로 다룬다. `visibility_decision(path)`을 무조건 선행하지 말고, Visible/Hidden policy-only fast 판정 가능성, BridgeVisible의 attr/type 필요성, glob-heavy matcher 비용과 host stat 절감 tradeoff를 분리해 본다.


#### Next slices: request-local guard context, readlink, and matcher streaming

The next metadata/open-path work must keep the lookup/getattr attr-reuse slice separate from broader request-local reuse. A request-local guard context is allowed to carry only values derived inside the same FUSE request: the canonical `source_root_path()` result, optional same-`VirtualPath` no-follow child attrs, and opened parent/object fd resolution results that still pass current-path validation. It must not become a cross-request path, visibility, symlink, or policy cache.

Readlink optimization contract:

- `readlink()` may use a readlink-specific helper that opens and validates the parent directory once, then uses that same parent dirfd for `fstatat(AT_SYMLINK_NOFOLLOW)` and `readlinkat`.
- The no-follow attr from that helper may be passed to the read guard only for the same symlink `VirtualPath`; it does not replace `check_hidden_symlink_target()` or symlink target fully-visible policy.
- Parent dirfd reuse must still call the same current-path validation used by other backing helpers before issuing host syscalls. If validation fails, the operation fails closed with the existing `ENOENT` behavior.

Open/access/opendir request-local reuse contract:

- `open()`, `access()`, and `opendir()` may share one request-local resolver between pre-open guards and opened-target revalidation so the canonical source root is not recomputed inside the same request.
- `open_confined` still uses `openat2(RESOLVE_IN_ROOT | RESOLVE_NO_MAGICLINKS)` and must not rely on cached policy to skip opened-target revalidation.
- Mutating opens and writable access still perform mutability checks with hidden-before-`EROFS` precedence.
- `opendir` is part of this scope even when the formal mounted latency matrix reports it through directory-surface workloads; perf counters must show whether source-root/open-fd resolution work moved or decreased.

Matcher streaming contract:

- `PathRuleMatcher::best_descriptor()` is the hot path and may use an allocation-free streaming best-candidate helper.
- `candidate_order()` and descendant candidate-order APIs remain the debug/metrics source of truth and must keep their current `Vec` order, dedup, and metric behavior.
- The streaming helper must return the same winner as `candidate_order(path).into_iter().find(|descriptor| descriptor.matches_path(path))`, including mixed exact/subtree, direct-child glob, recursive glob, and recursive literal families. Ties continue to follow existing `match_order`/specificity precedence.

Additional evidence for these slices:

- Static diff must show readlink parent dirfd validation is not repeated between guard and `readlinkat`, and that `open()`/`access()` share a request-local resolver through pre-open and opened-target checks.
- Matcher tests must compare streaming `best_descriptor()` results to the existing candidate-order winner across specificity, conflict, glob, recursive, and mixed-family cases.
- Benchmark artifacts must include `metadata_readlink` and the policy rows needed for fast-path-cache-eligible, fallback-unsafe-policy, and glob/matcher-heavy claims.

### Sync surface

`flush`, `fsync`, `release(flush=true)`는 이미 [`../src/fs.rs`](../src/fs.rs)의 `flush()`/`release()`/`fsync()`에서 file-handle snapshot/removal 뒤 state lock 밖으로 sync syscall을 offload한다.

현재 evidence:

- 구현: [`../src/fs.rs`](../src/fs.rs) (`flush()`, `release()`, `fsync()`, `offload_file_sync()`), [`../src/fs/perf.rs`](../src/fs/perf.rs) (`file_sync.flush`, `file_sync.fsync`, `file_sync.release_flush` attribution counters)
- 회귀 테스트: [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`flush_and_fsync_complete_under_compio_runtime`, `release_flush_true_completes_under_compio_runtime_and_removes_handle`, `offload_file_sync_preserves_closure_errno`, `offload_file_sync_completes_without_ambient_compio_runtime`, `offload_file_sync_maps_closure_panic_to_eio`), [`../src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) (`perf_counters_record_file_sync_splits`)
- sync surface 범위/후속 조건: [`artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md)
- 설계/락 규칙: [`artifacts/current-state-lock-concurrency-evidence.md`](artifacts/current-state-lock-concurrency-evidence.md)

남은 질문:

- close/fsync-heavy workload에서 blocking pool saturation/backpressure가 있는지
- sync surface별 tail latency가 어떻게 나타나는지
- errno/join failure mapping이 관측 가능한 regression을 만들지 않는지

### Request-local path/source-root reuse

현재 구현에는 이미 [`../src/fs/guards.rs`](../src/fs/guards.rs)의 request-local reuse 정리가 들어가 있다. `RequestPathResolver`가 단일 FUSE request 범위에서 canonicalized `source_root_path()` 결과를 재사용하고, mutation coordinate 검사와 opened-target revalidation에서 반복되던 resolved virtual path/source path 변환 helper를 공통화한다. 현재 perf summary는 `source_root_path`, `resolved_virtual_path_from_path`, `resolved_virtual_path_from_open_fd`를 별도로 내보내며, aggregate `resolved_virtual_path`도 path/open-fd helper 합계로 유지한다. 여기에 `resolved_virtual_path_from_path_component_walk`, `_canonicalize`, `_source_root_confinement`, `_virtual_conversion`까지 추가로 내보내지만, 이 하위 counter들은 additive partition이 아니라 helper attribution 보조선으로 읽어야 한다. Path resolution은 [`../src/path.rs`](../src/path.rs)의 `resolve_host_path_from_canonical_source_root()`를 통해 per-call `source_root.canonicalize()` 중복을 피하면서 같은 confinement/`ENOENT` 의미론을 유지하고, component traversal도 inner walk에서 `Component` `Vec`를 만들지 않는 iterator 기반 정리를 유지한다.

현재 evidence:

- 구현: [`../src/fs/guards.rs`](../src/fs/guards.rs) (`RequestPathResolver`, `source_root()`, `resolved_virtual_path()`, `resolved_virtual_path_for_open_file()`, `guard_mutation_coordinates()`), [`../src/path.rs`](../src/path.rs) (`resolve_host_path()`, `resolve_host_path_from_canonical_source_root()`)
- 집중 회귀 테스트: [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`mutation_under_visible_symlink_parent_uses_resolved_parent`, `symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent`, `deleted_source_root_resolved_path_fails_closed`, `symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects`)
- path helper 회귀 테스트: [`../src/path_tests.rs`](../src/path_tests.rs) (`rejects_following_symlinks_outside_source_root_but_allows_link_itself`)
- current operation/TOCTOU 근거: [`artifacts/current-toctou-hardening-evidence.md`](artifacts/current-toctou-hardening-evidence.md) (pinned `source_root`, opened-target revalidation, resolved-parent mutation path)
- mounted probe coverage 한계: [`benchmarks.md`](benchmarks.md) (`symlink_parent_mkdir_rmdir`, Current harness coverage limits)

가드레일:

- reuse 범위는 같은 request 내부로만 제한한다.
- canonical source root reuse가 필요해도 `resolve_host_path_from_canonical_source_root()`/`virtual_path_from_source_path` 기반 계산과 errno 매핑을 유지한다.
- cross-request symlink decision cache나 host negative cache로 확대하지 않는다.
- hidden `ENOENT`, symlink target fully-visible gate, mutability `EROFS` 판단을 cache hit만으로 생략하지 않는다.
- path walk cleanup이 들어가더라도 component를 미리 `Vec`로 모으는 방식으로 되돌리지 않는다.

증거 요구:

- 현재 변경은 semantics-preserving refactor/reuse, canonical source-root reuse, 또는 attribution split로만 문서화한다.
- `resolved_virtual_path` hot path claim은 retained aggregate `resolved_virtual_path` line만 단독으로 인용하지 말고 `source_root_path`, `resolved_virtual_path_from_path`, `resolved_virtual_path_from_open_fd`와 함께 남긴다. 현재 `resolved_virtual_path_from_path_*` 하위 counter를 쓸 때도 component walk, canonicalize, confinement, virtual conversion 중 무엇을 인용했는지 명시하고 additive partition처럼 과장하지 않는다.
- correctness evidence에는 [`../src/path_tests.rs`](../src/path_tests.rs) (`rejects_following_symlinks_outside_source_root_but_allows_link_itself`)와 [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent`, `symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects`)가 포함돼야 한다.
- helper attribution evidence에는 [`../src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) (`perf_counters_split_resolved_virtual_path_sources` for the split and detailed counters, 필요 시 `perf_counters_record_data_path_splits_on_success`)가 포함돼야 한다.
- `scripts/bench-screenfs.py`의 `symlink_parent_mkdir_rmdir`는 이 surface 주변의 symlink-parent mutation guard/path-resolution path를 겨냥한 ScreenFS-only mounted workload/probe지만, counter/trace 없이 live FUSE request가 특정 내부 helper를 탔다고 증명하지는 못한다.
- user-visible speedup 주장은 [`benchmarks.md`](benchmarks.md)의 claim-grade bar(최소 `--iterations 10 --warmups 3`)를 만족하는 같은 machine/policy/workload의 before/after benchmark artifact와, 필요 시 perf counter·trace·microbenchmark가 함께 나온 뒤에만 한다.
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
- matcher family별 후보 수(exact/subtree, direct-child glob, recursive non-visible glob, recursive literal non-visible subtree)
- `candidate_order`/`descendant_candidate_order`의 `Vec` allocation 및 duplicate-removal 비용이 의미 있는지
- hidden precedence와 most-specific rule wins 계산 비용

권장 시작점:

- 현재 opt-in perf counters의 `matcher_candidates`, `matcher_family_candidates.*`, `matcher_candidate_order.{path,descendant}`, aggregate 및 order-labeled `matcher_candidate_order_duplicates`, `matcher_candidate_order_seen_slots`, `matcher_candidate_order_ancestor_steps`
- `scripts/bench-screenfs.py --matcher-extra-rules <N>` rule-rich mounted probe (`matcher_hidden_stat_miss` 포함)
- 위 aggregate signal만으로 부족할 때 rule 수와 path depth를 조합한 microbenchmark
- 필요한 경우에만 live smoke/benchmark artifact를 재생성해 현재 counter surface를 남긴다

최적화 후보(계측 후에만):

- matcher family split 결과를 본 뒤에만 bucket/index 재배치 검토
- candidate set/order 불변성을 유지하는 allocation reuse류 미세 최적화는 검토 가능하지만, 최종 후보 집합과 `order_rank` 순서 및 dedup 결과가 기존과 같아야 한다.
- `candidate_order`/`descendant_candidate_order`의 duplicate-removal 단순화는 counter/trace가 실제 duplicate cost를 보여줄 때만 검토한다. 현재 matcher32 checked-in artifact처럼 duplicate counter가 0인 경우에는 duplicate-removal 최적화 근거로 쓰지 않는다.

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


### Post-metadata follow-up: directory, matcher-descendant, and read-only close

After the metadata/open-path guard-context work, the next active follow-up is directory-surface and read-only close fixed overhead. This work must not weaken hidden filtering, bridge-visible directory-only listing, stable resume cookies, returned-page-only `readdirplus` lookup-ref pinning, or write-capable sync semantics.

Directory optimization contract:

- `readdir` may avoid building a full `FileAttr` for entries when the kernel directory entry type is sufficient for visibility decisions. If the host returns `DT_UNKNOWN`, or if symlink/directory status is otherwise needed, fall back to the existing no-follow metadata path.
- `readdirplus` still needs returned entry attrs, but may reduce repeated scan/page work with a handle-local bounded scan buffer or progress reuse. This must remain invalidated by the existing mutation invalidation paths; it must not become a persistent full-directory listing cache.
- Candidate selection may replace the current `BTreeMap` top-N shape only if page ordering, bounded FUSE `size`, stable cookies, and hidden-entry filtering before returned-page commit remain identical. `readdirplus` lookup refs are pinned only for children in the committed returned page.
- If a change touches symlink visibility inside directory iteration, the regular-file-only `readdir_basic`/`readdirplus_basic` fixture is not enough; add focused tests or a symlink-aware workload/trace that exercises `readdir*_symlink_visibility`.

Matcher-descendant contract:

- Directory-heavy bridge-visible checks use `visible_matcher.may_match_descendant_of(path)`, not only `best_descriptor()`. Any optimization here must preserve current boolean results, short-circuit behavior, most-specific rule wins, and hidden/visible precedence for exact, subtree, direct-child glob, recursive glob, and recursive literal families.
- Debug/metrics APIs that expose descendant candidate-order shape must keep their current ordering/dedup semantics unless the metric contract is explicitly updated with equivalent evidence.
- Claim-grade matcher evidence requires a glob/hidden-heavy directory row or an equivalent focused workload that actually exercises descendant candidate logic; the existing matcher32 metadata artifact is not sufficient when `matcher_candidate_order.descendant` comes only from an empty visible matcher.

Read-only open-close sync contract:

- `FOPEN_NOFLUSH` may be set only for handles opened without write intent. Treat `O_WRONLY`, `O_RDWR`, `O_TRUNC`, `O_CREAT`, and create-returned handles as write-capable.
- If mounted evidence shows `FOPEN_NOFLUSH` does not suppress `FUSE_FLUSH`, a handle-local `flush_needs_sync`/`write_intent` bit may make read-only `flush()` skip `sync_all()`. This must be based only on the handle's open/create intent and must not skip `release()` cleanup.
- Write-capable handles keep existing `flush()` and `release(flush=true)` sync behavior. Explicit `fsync()`/`fdatasync()` remains honored for all handles.
- `sync_release_flush` remains a known mounted measurement gap when the kernel does not send `release(flush=true)`; do not claim release-flush improvement from a run where `file_sync.release_flush` is zero.

Evidence required for this follow-up:

- Before/after `directory-surface` benchmark under `fast-path-cache-eligible` and a glob/hidden-heavy policy row, with `readdir_basic` and `readdirplus_basic` p50/p95/p99 plus directory split counters.
- Read-only and write-capable open/close workloads or equivalent artifacts that report `fuse_op.flush`, `file_sync.flush`, `fuse_op.release`, `file_sync.fsync`, and read-only/write p50/p95/p99.
- Correctness coverage for visibility filtering, bridge-visible page boundaries, stable cookies across pages, returned-page-only `readdirplus` lookup refs, directory handle invalidation, read-only release cleanup, write/create flush behavior, and explicit fsync.

### 4. Directory entry attr cost

`readdirplus`는 entry마다 metadata/attr 생성을 요구하므로 큰 directory에서 비쌀 수 있다.

측정 항목:

- `readdir` 비용
- `readdirplus` 비용
- entry당 `symlink_metadata` 비용
- attr 변환 비용
- symlink target fully-visible gate 비용
- page candidate selection / returned-page commit / lookup-ref pinning 비용
- page size별 total latency와 tail latency

권장 접근:

```text
현재 `readdir_directory_scan` / `readdirplus_directory_scan`, attr-build-only `readdir_attr_generation_scan` / `readdirplus_attr_generation_scan`, `readdir*_{symlink_visibility,candidate_selection,page_commit}`를 먼저 읽는다
그 뒤에도 불충분할 때만 더 미세한 trace 또는 counter를 추가한다
opendir/readdir에서 state lock으로 inode/path만 짧게 확인한다
host read_dir + metadata 수집은 lock 밖에서 수행한다
결과 반영 시에만 state lock을 재획득한다
```

주의:

- hidden entry filtering 결과가 바뀌면 안 된다.
- symlink target visibility check를 유지한다.
- host `read_dir`/metadata 수집을 state lock 밖으로 옮겨도 기존 dirfd/openat2-confined access 경계를 유지해야 하며, 문자열 path 재조합 기반의 unconstrained path walk로 바꾸면 안 된다.
- stable resume cookie/shared cookie domain과 returned-page `readdirplus` lookup-ref pinning을 유지해야 한다.
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

현재 `--features perf-counters`로 빌드한 binary에서 자동으로 켜지는 opt-in counter:

- FUSE operation별 `fuse_op.<operation>` count/latency
- policy decision count/latency, aggregate `matcher_candidates`, `matcher_family_candidates.{subtree,direct_child_glob,recursive}`
- `matcher_candidate_order.{path,descendant}` latency와 aggregate/order-labeled `matcher_candidate_order_duplicates`, `matcher_candidate_order_seen_slots`, `matcher_candidate_order_ancestor_steps`
- state lock read/write wait/hold count/latency
- `open_confined_openat2` count/latency
- `stat_child_no_follow` count/latency
- `source_root_path` count/latency
- aggregate `resolved_virtual_path` count/latency (retained sum of `resolved_virtual_path_from_path` + `resolved_virtual_path_from_open_fd`)
- `resolved_virtual_path_from_path` / `resolved_virtual_path_from_open_fd` count/latency
- `resolved_virtual_path_from_path_component_walk`, `_canonicalize`, `_source_root_confinement`, `_virtual_conversion`
- data-path split `read_handle_snapshot`, `read_guard_path`, `read_io`, `write_handle_snapshot`, `write_guard_mutation`, `write_io`
- `read_size_bucket.<bucket>` / `write_size_bucket.<bucket>` count/latency for the timed `read_io`/`write_io` segment only
- `readdir_directory_scan`, attr-build-only `readdir_attr_generation_scan` plus `readdir_attr_generation_entries`, `readdir_symlink_visibility`, `readdir_candidate_selection`, `readdir_page_commit`
- `readdirplus_directory_scan`, attr-build-only `readdirplus_attr_generation_scan` plus `readdirplus_attr_generation_entries`, `readdirplus_symlink_visibility`, `readdirplus_candidate_selection`, `readdirplus_page_commit`
- mutation invalidation count, invalidated entries, evicted entries

출력은 ScreenFS 종료 시 stderr summary다. 이 summary는 benchmark/smoke run의 내부 attribution 보조 evidence이며, 단독 performance claim 근거가 아니다. 기본 build에는 instrumentation code/config surface가 포함되지 않는다. `scripts/bench-screenfs.py --perf-counters --build`는 `--features perf-counters`로 빌드하며, 종료 후 summary를 JSON `screenfs.perf_summary`와 Markdown report에 기록한다. matcher-heavy attribution이 필요하면 `scripts/bench-screenfs.py --matcher-extra-rules <N>`이 synthetic hidden/readonly rule set과 `matcher_hidden_stat_miss` workload를 함께 준비한다.

남은 counter/backlog:

- 현재 whole-run aggregate를 matcher instance/decision axis 또는 workload별로 더 쪼개야 할지 claim-grade evidence가 필요할 때만 검토한다
- current `resolved_virtual_path_from_path_*` 또는 `readdir*` bucket으로도 부족한 경우에만 trace/counter를 더 세분화한다
- histogram 또는 benchmark artifact 저장 형식의 추가 구조화

## Measurement-guided optimization notes

Perf-enabled benchmark evidence should drive optimization order. The historical smoke baseline summarized in [`artifacts/current-perf-counter-baseline-summary.md`](artifacts/current-perf-counter-baseline-summary.md) showed aggregate `resolved_virtual_path` attribution (`count=145166`, `total_ns=394848335`) and `policy_decision` (`count=171732`, `total_ns=183484872`) as broader hot surfaces than `open_confined_openat2` (`count=98298`, `total_ns=47566036`). That baseline was enough to justify deeper attribution, not a user-visible speedup claim. Current code still retains aggregate `resolved_virtual_path`, additionally emits `stat_child_no_follow`, `source_root_path`, `resolved_virtual_path_from_path`, `resolved_virtual_path_from_open_fd`, `resolved_virtual_path_from_path_*` sub-counters, matcher family/`matcher_candidate_order` counters, read/write data-path split counters, and split `readdir`/`readdirplus` attr/symlink/candidate-selection/page-commit buckets. It also uses request-local canonical source-root reuse in `ScreenFs::resolved_virtual_path()`/`RequestPathResolver` via `resolve_host_path_from_canonical_source_root()` so per-call `source_root.canonicalize()` is avoided without changing confinement or `ENOENT` semantics; the path-walk cleanup keeps iterator-based component traversal rather than materializing a component `Vec`. The next step remains measurement-first: read the expanded attribution surface, regenerate smoke artifacts when they predate the current counter surface, and still require claim-grade before/after benchmark pairs before describing any speedup. The checked-in smoke artifacts remain smoke-only evidence, not claim evidence, while [`artifacts/per-open-cache-claim/summary.md`](artifacts/per-open-cache-claim/summary.md) is the current claim-grade pair for the active per-open-cache goal. [`artifacts/current-perf-counter-benchmark-result.json`](artifacts/current-perf-counter-benchmark-result.json), [`artifacts/current-perf-counter-benchmark-result.md`](artifacts/current-perf-counter-benchmark-result.md), and [`artifacts/current-perf-counter-benchmark-result.svg`](artifacts/current-perf-counter-benchmark-result.svg) predate the read/write data-path split counters; use [`artifacts/managed-fio-attribution-summary.md`](artifacts/managed-fio-attribution-summary.md), [`artifacts/managed-fio-attribution-perf-split.json`](artifacts/managed-fio-attribution-perf-split.json), and [`artifacts/managed-fio-attribution-screenfs.stderr.log`](artifacts/managed-fio-attribution-screenfs.stderr.log) for current supplemental data-path split attribution evidence.

## Recommended order

```text
1. 문서/상태 정합성을 유지한다
2. active next candidate인 metadata/open-path fixed-overhead contract부터 고정한다 (split workload additions, policy matrix, artifact naming, raw-sample metric contract)
3. claim-grade before/after benchmark pair가 필요한 metadata/open-path + sync + `readdir`/`readdirplus` workload를 먼저 채운다 (`--matcher-extra-rules` 같은 focused policy knobs 포함)
4. opt-in perf counter와 benchmark surface를 유지·확장하고 현재 expanded attribution(`stat_child_no_follow`, `source_root_path`, `resolved_virtual_path_from_path_*`, `matcher_family_candidates.*`, `matcher_candidate_order.*`, `readdir*`/`readdirplus*` split buckets)을 먼저 읽는다
5. 현재 checked-in smoke/benchmark artifact가 필요한 counter surface를 못 담으면 재생성 계획부터 세운다
6. policy/matcher hot path와 state lock hold time을 계측한다
7. request-local canonical source-root reuse 결과를 mounted probe workload와 before/after 비교로 계측한다
8. read/write buffer size와 concurrency benchmark를 보강한다
9. readdir vs readdirplus attr 비용과 open_confined/openat2 호출 빈도/latency를 측정한다
10. invalidate_after_mutation 범위와 evicted entry 수를 측정한다
11. current counters로도 부족한 경우에만 더 세분화한 trace/counter를 추가한다
12. evidence가 쌓인 뒤에만 negative/hidden path cache, state lock split, host-side io_uring를 검토한다
```

## Non-goals

- `openat2` confinement를 일반 `open/openat` 또는 고수준 async open으로 대체
- symlink visibility decision을 cross-request cache로 생략
- hidden path를 `Permission denied`로 노출
- `/dev/null` bind overlay, tmpfs masking, 빈 파일 overlay처럼 이름을 남기는 masking
- recursive background scan/index로 visibility bridge를 추론
- lock을 잡은 채 blocking IO 또는 `spawn_blocking` 실행
- benchmark 없이 metadata/mutation/xattr path를 대규모 async화
