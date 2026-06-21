# Readdirplus bounded candidate-heap attempt

This directory records a post-`70a3105` retry for the `readdirplus` page/scan lane. The candidate replaced the `BTreeMap<Vec<u8>, DirEntryInfo>` candidate selection in `ScreenFs::collect_child_directory_page` with a bounded max-heap plus name-keyed retained-entry set, preserving lexicographic output and old name-deduplication semantics while trying to reduce large-directory candidate-selection overhead.

## Result

Rejected / not kept. Focused tests and review showed the candidate could preserve ordering/resume semantics after adding name-deduplication and paired `readdir` coverage, but same-machine benchmark evidence failed the claim gate. The 20k focused row improved at p50 only, missed p95/p99, and paired `directory-surface` rows regressed. The Rust implementation was reverted; this artifact remains rejected evidence only.

## Lane and gate rows

- lane: `readdirplus` page/scan
- previous scope label: `slice`
- target: focused 20k `readdirplus_basic` gap without changing directory semantics

Rows collected under [`worktree-70a31053d688/`](worktree-70a31053d688/) with clean `HEAD` before rows and dirty candidate after rows, all using `--perf-counters --iterations 10 --warmups 3`:

| row | purpose | gate read |
| --- | --- | --- |
| `--policy-preset fast-path-cache-eligible --workload-set directory-surface` | paired directory evidence | `readdirplus_basic` claim rows need `<= 0.90x` p50 and `<= 0.95x` p95/p99; paired `readdir_basic` must stay within `<= 1.05x` p50 and `<= 1.10x` p95/p99 |
| `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface` | matcher-adjacent directory non-regression | same row gate as above |
| `--policy-preset fast-path-cache-eligible --workload readdirplus_basic --dir-entries 20000` | focused large-directory retry | must clear `<= 0.90x` p50 and `<= 0.95x` p95/p99 before promotion beyond attempt/slice scope |

## Benchmark evidence

JSON source-of-truth files:

- `worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-directory-surface.json`
- `worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-directory-surface.json`
- `worktree-70a31053d688/before-70a31053d688-glob-matcher-heavy-directory-surface.json`
- `worktree-70a31053d688/after-worktree-70a31053d688-glob-matcher-heavy-directory-surface.json`
- `worktree-70a31053d688/before-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.json`
- `worktree-70a31053d688/after-worktree-70a31053d688-fast-path-cache-eligible-readdirplus-basic-20k.json`

After/before latency ratios:

| row | workload | p50 | p95 | p99 | interpretation |
| --- | --- | ---: | ---: | ---: | --- |
| fast `directory-surface` | `readdir_basic` | `1.042241x` | `1.063237x` | `1.069269x` | non-regression passes but worsens |
| fast `directory-surface` | `readdirplus_basic` | `1.062710x` | `1.090198x` | `1.096719x` | claim gate fails / regresses |
| matcher32 `directory-surface` | `readdir_basic` | `1.480425x` | `1.467177x` | `1.466719x` | non-regression fails badly |
| matcher32 `directory-surface` | `readdirplus_basic` | `1.137827x` | `1.085680x` | `1.065619x` | claim gate fails / regresses |
| fast 20k focused | `readdirplus_basic` | `0.882012x` | `0.998063x` | `1.002933x` | p50 improves, but p95/p99 miss the gate |

Focused 20k counters showed mixed attribution rather than a keepable claim: `fuse_op.readdirplus.avg_ns` improved (`32425271ns -> 29994587ns`) and `readdirplus_directory_scan.avg_ns` improved (`30149283ns -> 27953586ns`), but `readdirplus_candidate_selection.avg_ns` regressed (`2059673ns -> 2908967ns`). Combined with failed p95/p99 and paired `directory-surface` regressions, the implementation is rejected.

## Counters watched

- `fuse_op.readdirplus`
- `readdirplus_directory_scan`
- `readdirplus_attr_generation_scan`
- `readdirplus_attr_generation_entries`
- `readdirplus_candidate_selection`
- `readdirplus_page_commit`
- paired `readdir_*` counters on `directory-surface` rows

## Safety constraints checked before rejection

The attempted implementation preserved these intended constraints while it was under test:

- hidden filtering, hidden `ENOENT` precedence, bridge-visible semantics, and readonly `EROFS` behavior unchanged
- symlink target visibility checks unchanged
- `readdirplus` attr revalidation for returned entries unchanged
- stable resume cookies / shared cookie domain maintained
- returned-page-only `readdirplus` lookup-ref pinning maintained
- `readdir` / `readdirplus` returned entry ordering preserved in focused small-page tests

## Local validation and review

Local validation passed before benchmark rejection:

- `cargo fmt --check`
- `cargo check --features perf-counters`
- `cargo test readdir_keeps_lexicographic_pages_with_small_child_budget -- --nocapture`
- `cargo test readdirplus_keeps_lexicographic_pages_with_small_child_budget -- --nocapture`
- `cargo test readdirplus_honors_size_budget_and_continues_from_last_cookie -- --nocapture`
- `cargo test readdir_honors_size_budget_and_last_cookie_continuation -- --nocapture`
- `cargo test --features perf-counters perf_counters_record_policy_state_open_and_readdirplus_attr_work -- --nocapture`

Initial review found a name-deduplication compatibility issue and missing paired `readdir` test coverage; both were fixed before benchmarking. Follow-up reviewer reported no findings. Security review found no guardrail blocker. Because the benchmark gate failed, the `readdirplus` lane remains `slice` with this bounded candidate-heap attempt recorded as rejected evidence.
