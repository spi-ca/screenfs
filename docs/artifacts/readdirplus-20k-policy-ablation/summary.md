# Readdirplus 20k policy/size ablation summary

이 디렉터리는 **current dirty-worktree coverage/current-ablation**만 기록한다. 네 개의 artifact는 같은 revision id (`920c00e4308596bc285b236ae58139b30e24b0cb`)에서 수집됐지만, provenance상 `git_worktree_clean=False` / `screenfs_source_git_worktree_clean=False`였고 `git_status_porcelain`에는 regenerated open-confined/opendir smoke docs, `scripts/bench-screenfs.py`, `src/fs.rs`, `src/fs/guards.rs`, `src/fs/perf.rs`, `src/fs/tests/perf.rs` 같은 open-like attribution/source changes가 포함돼 있다. 따라서 이것은 `readdirplus` no-code-change control이나 before/after speedup claim이 아니라, size/policy를 prior rejected [`../readdirplus-page-scan-path-join/summary.md`](../readdirplus-page-scan-path-join/summary.md)보다 더 정직하게 분리해 읽는 **coverage ablation**이다.

## Covered artifacts

- [`current-fallback-unsafe-policy-directory-surface-5k.md`](current-fallback-unsafe-policy-directory-surface-5k.md)
- [`current-fast-path-cache-eligible-directory-surface-5k.md`](current-fast-path-cache-eligible-directory-surface-5k.md)
- [`current-fallback-unsafe-policy-readdirplus-basic-20k.md`](current-fallback-unsafe-policy-readdirplus-basic-20k.md)
- [`current-fast-path-cache-eligible-readdirplus-basic-20k.md`](current-fast-path-cache-eligible-readdirplus-basic-20k.md)

## Provenance notes

- 모든 companion artifact는 `git_worktree_clean=False`, `screenfs_source_git_worktree_clean=False`를 기록한다.
- companion JSON의 `git_status_porcelain`에는 regenerated `current-open-confined-surface-smoke*` / `current-metadata-opendir-smoke*`, `scripts/bench-screenfs.py`, `src/fs.rs`, `src/fs/guards.rs`, `src/fs/perf.rs`, `src/fs/tests/perf.rs`가 포함돼 있어, 이 bundle을 “같은 code snapshot에서 `readdirplus`만 그대로 둔 control”로 읽지 않는다.
- perf blocks themselves also expose the newer surrounding/source attribution surface (`open_like.pre_open_guard.*`, `open_like.post_open_revalidation.*`, `source_root_path`, `resolved_virtual_path_from_open_fd`).

## Mounted `readdirplus_basic` latency snapshot

| artifact | mounted p50 s | mounted p95 s | mounted p99 s | read |
| --- | ---: | ---: | ---: | --- |
| fallback `directory-surface` 5k | 0.3432486835 | 0.36014199225 | 0.36046965525 | 5k mixed directory row에서 fallback이 fast보다 약간 빠름 |
| fast `directory-surface` 5k | 0.370510508 | 0.38566112995 | 0.38844790759 | 5k에서는 cache-eligible policy가 자동으로 더 빠르지 않음 |
| fallback focused `readdirplus_basic` 20k | 2.219389109 | 2.4015893068 | 2.42237238616 | 20k focused row에서 tail 포함 가장 느림 |
| fast focused `readdirplus_basic` 20k | 1.923972388 | 1.95623135645 | 1.95716670809 | 20k에서는 fast가 fallback보다 분명히 빠름 (`0.866893x` p50) |

## Key `readdirplus_*` perf-counter totals

| artifact | `readdirplus_directory_scan.total_ns` | `readdirplus_attr_generation_scan.total_ns` | `readdirplus_attr_generation_entries` | `readdirplus_candidate_selection.total_ns` | `readdirplus_page_commit.total_ns` |
| --- | ---: | ---: | ---: | ---: | ---: |
| fallback 5k | 276325085 | 38172888 | 5828 | 15799012 | 3923898 |
| fast 5k | 271007630 | 41261806 | 5828 | 18006488 | 4359352 |
| fallback 20k | 1018163675 | 48066025 | 6246 | 58856683 | 13789497 |
| fast 20k | 899635104 | 43509086 | 6246 | 60287368 | 12143782 |

## Interpretation

- **Coverage only**: 네 run 모두 same-revision-id dirty-worktree snapshot이다. 이 디렉터리로는 before/after speedup, kept `readdirplus` optimization, isolated `readdirplus` code effect, 또는 no-code-change control을 주장하지 않는다.
- **20k cost is still directory-scan-dominant**: focused 20k row에서 `readdirplus_directory_scan`은 fallback `1.018s`, fast `0.900s`로, 같은 row의 `candidate_selection`(`0.059s` / `0.060s`)과 `page_commit`(`0.014s` / `0.012s`)보다 훨씬 크다. 네 counter 묶음 기준으로도 scan share는 fallback `89.4%`, fast `88.6%`다.
- **20k cost is policy-sensitive**: 5k에서는 fallback이 fast보다 약간 빠르지만, 20k focused row에서는 fast가 fallback보다 빠르다 (`2.219389s -> 1.923972s` p50). 즉 남은 비용은 단순한 page-commit 문제만이 아니라 큰 directory에서 policy shape와 함께 움직인다.
- **`page_commit` is not the main remaining gap**: 20k에서도 `readdirplus_page_commit.total_ns`는 `12-14ms` 수준이고, `candidate_selection`도 `59-60ms` 수준이다. 현재 bottleneck 해석의 중심은 여전히 scan 쪽이다.
- **Why this ablation is useful**: rejected path-join bundle은 implementation change와 size effect가 섞여 있었다. 이 current ablation은 dirty current snapshot 안에서 5k/20k와 fallback/fast를 갈라 보므로, current evidence에서 size/policy separation을 더 잘 제공한다.
- **Visibility/symlink caveat remains**: 이 묶음은 `directory-surface`와 regular-file-heavy focused `readdirplus_basic` row를 읽는 coverage다. hidden/visible semantics, symlink target visibility, `readdir*_symlink_visibility` surface에 대한 kept speedup claim으로 확장하지 않는다.

## Bottom line

이 summary는 다음을 보여준다: current dirty worktree의 20k `readdirplus` surface는 여전히 `directory_scan` 지배적이고 policy-sensitive하지만, 이 bundle만으로 kept optimization이나 before/after speedup을 주장할 수는 없다. 다음 단계는 여전히 claim-grade before/after pair와 focused large-directory row를 같은 machine/policy contract로 다시 채우는 것이다.
