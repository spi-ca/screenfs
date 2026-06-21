# ScreenFS benchmarks

This document defines the formal benchmark workflow for ScreenFS performance work. It is separate from correctness smoke evidence: benchmark results are only performance evidence for the recorded machine, kernel, backing filesystem, policy, and workload.

## Method

The repo-local harness is [`scripts/bench-screenfs.py`](../scripts/bench-screenfs.py). It follows the common recommendations used by Criterion/hyperfine/fio-style benchmarking:

- do not include build time, fixture creation, or mount startup in measured workload timing
- run warmup iterations before measured iterations
- keep every raw sample and report summary statistics, not a single run
- compare ScreenFS against the same workload on the native backing source when possible
- record environment metadata, the harness command line, workload sizes, policy inputs, binary SHA, and dirty/worktree state with the result
- treat cache state explicitly; the default harness evidence is warm-cache local evidence unless a run records separate cache-control steps

The harness intentionally uses only Python standard-library operations plus the built `screenfs` binary. It does not add benchmark-only Rust dependencies to the product crate. For lower-level storage work, use external tools such as `fio` only with a separately recorded job file and environment notes.

For optimization priorities and deferred performance candidates, see [`performance-roadmap.md`](performance-roadmap.md). This document remains the measurement workflow and evidence contract.

ScreenFS also has optional internal attribution counters for benchmark/debug attribution. They are compiled and enabled only when the binary is built with `--features perf-counters`; default builds do not include the instrumentation code or config surface. A perf-enabled binary should be used only for benchmark/smoke runs that explicitly need attribution evidence, not for normal production use.

When enabled by the Cargo feature, ScreenFS prints a stderr summary at shutdown with FUSE operation latency, policy decision latency, aggregate `matcher_candidates`, `matcher_family_candidates.{subtree,direct_child_glob,recursive}`, `matcher_candidate_order.{path,descendant}` latency plus aggregate and order-labeled duplicate/seen-slot/ancestor-step counts, state lock read/write wait/hold latency, `open_confined_openat2` latency, `open_like.pre_open_guard.*` / `open_like.post_open_revalidation.*` open-like surrounding-cost latency, `stat_child_no_follow` latency, `source_root_path` latency, aggregate `resolved_virtual_path` latency, split `resolved_virtual_path_from_path` / `resolved_virtual_path_from_open_fd` latency, `resolved_virtual_path_from_path_component_walk` / `_canonicalize` / `_source_root_confinement` / `_virtual_conversion`, data-path split `read_handle_snapshot` / `read_guard_path` / `read_io` and `write_handle_snapshot` / `write_guard_mutation` / `write_io`, `file_sync.{flush,fsync,release_flush}` sync-helper latency, `read_size_bucket.*` / `write_size_bucket.*` latency for the timed `read_io`/`write_io` segment only, `readdir_directory_scan` / `readdirplus_directory_scan`, attr-build-only `readdir_attr_generation_scan` / `readdirplus_attr_generation_scan` plus scanned entry counts, `readdir_symlink_visibility` / `readdirplus_symlink_visibility`, `readdir_candidate_selection` / `readdirplus_candidate_selection`, `readdir_page_commit` / `readdirplus_page_commit`, and invalidation/eviction/scanned-entry counts. `resolved_virtual_path` is retained as the aggregate of the path-walk and opened-fd helper buckets, while `source_root_path` is reported separately because both flows can reuse it. Use the split helper counters and sub-counters as first-pass attribution only: `resolved_virtual_path_from_path_component_walk` overlaps with the more specific canonicalize/confinement timings instead of forming an additive partition, matcher family/order counters show aggregate candidate shape rather than which specific matcher or rule was hottest, `open_like.pre_open_guard.*` / `open_like.post_open_revalidation.*` show surrounding guard/revalidation cost around the `open_confined_openat2` lane rather than standalone speedup evidence, and the directory buckets still do not replace before/after benchmark evidence. The current `open_confined`/readlink attribution surface keeps aggregate `stat_child_no_follow` and also emits root-path `stat_child_no_follow.host_fstat`, non-root `stat_child_no_follow.parent_open` / `stat_child_no_follow.directory_revalidation` / `stat_child_no_follow.host_fstatat`, shared `stat_child_no_follow.attr_conversion`, plus `stat_child_no_follow_context.*` labels such as `path_guard_or_metadata`; focused readlink perf coverage also checks `stat_child_no_follow_context.readlink_pre_open`. These splits do not relax `openat2` confinement, opened-fd revalidation, hidden `ENOENT`, symlink target visibility, or the no-cross-request-cache rule. Treat these counters as attribution evidence for a benchmark or smoke run, not as standalone performance claims. The shutdown summary is process-lifetime aggregate evidence: it includes warmups and any workload cleanup that runs before unmount, while latency sample tables time only the measured workload body. With `--perf-counters --build`, the benchmark harness builds with `--features perf-counters`, parses the shutdown summary into `screenfs.perf_summary` in the JSON output, and includes the raw summary in the Markdown report. If `--perf-counters` is used with an existing `--screenfs-bin`, that binary must already be built with the `perf-counters` feature.

## Workloads

The default benchmark creates a temporary source tree and runs these workloads through both the native source path and the ScreenFS mount path:

| workload | Purpose |
| --- | --- |
| `seq_read` | already-open data path read throughput surface |
| `seq_write` | write syscall/data path overhead, without timing fixture setup |
| `small_read` | repeated small-buffer reads from the sequential-read fixture to probe small read syscall/data-path overhead |
| `small_write` | repeated small-buffer writes to a side-specific output file, removed after each iteration |
| `write_fsync_close` | repeated open/write/file-fsync/close/cleanup cycles as an end-to-end sync-surface probe; it does not fsync parent directories |
| `small_stat_open_read` | metadata-heavy small-file stat/open/read path |
| `readdir_lstat` | directory listing plus metadata path |
| `symlink_open_read` | visible symlink dereference path |

Additional comparable workloads used by named sets include `rand_read_4k`, `rand_write_4k`, and `concurrent_rand_read_write_4k`; the concurrency row is the mixed random 4KiB read/write probe for `--workload-set read-write-concurrency` and scales per-iteration worker count with `--concurrency-workers`.

When `--matcher-extra-rules > 0`, the harness also exposes these matcher-descendant comparable workloads:

| workload | Purpose |
| --- | --- |
| `matcher_descendant_readdir` | scans the synthetic hidden `/.screenfs-bench/matcher-heavy/descendant-root` directory and expects the bridge-visible `branch-XXXX` descendants created by visible leaf carve-outs |
| `matcher_descendant_readdirplus` | same fixture as `matcher_descendant_readdir`, but forces per-entry `stat`/directory classification through the mounted path |

It also runs these ScreenFS-only contract workloads:

| workload | Purpose |
| --- | --- |
| `hidden_stat_miss` | hidden path `ENOENT` path and matcher overhead |
| `matcher_hidden_stat_miss` | optional rule-rich hidden path `ENOENT` probe enabled by `--matcher-extra-rules`; omitted from default runs when that knob is `0`; intended for matcher family / `candidate_order` attribution, not default policy claims |
| `symlink_parent_mkdir_rmdir` | repeated mkdir/rmdir under a visible symlink parent alias as an intended mounted probe for symlink-parent mutation guard/path-resolution paths |

The default `--workload-set default` preserves the historical comparable + ScreenFS-only run above. `--workload-set per-open-cache-minimum` runs `rand_read_4k`, `rand_write_4k`, `sync_write_4k`, and `small_open_read_close`; `--workload-set read-write-surface` runs `seq_read`, `seq_write`, `small_read`, `small_write`, `rand_read_4k`, and `rand_write_4k`; `--workload-set read-write-concurrency` runs `concurrent_rand_read_write_4k`; `--workload-set metadata-open-path` runs `metadata_lookup`, `metadata_getattr`, `metadata_open`, `metadata_readlink`, `metadata_access`, and `metadata_statfs`; `--workload-set open-confined-surface` runs `metadata_open` and `metadata_opendir`; `--workload-set sync-surface` runs `sync_flush_only`, `sync_fsync_only`, and `sync_release_flush`; `--workload-set read-only-close-surface` runs `read_only_open_close`, `read_only_open_read_close`, `write_open_write_close`, and `write_open_fsync_close`; `--workload-set directory-surface` runs `readdir_basic` and `readdirplus_basic`; `--workload-set directory-symlink-surface` runs `readdir_symlink_visibility` and `readdirplus_symlink_visibility`; `--workload-set matcher-descendant-directory` runs `matcher_descendant_readdir` and `matcher_descendant_readdirplus`; `--workload-set policy-heavy-matrix` runs `metadata_lookup`, `metadata_getattr`, `metadata_access`, and, when `--matcher-extra-rules > 0`, `matcher_hidden_stat_miss`; `--workload-set mutation-invalidation` runs `symlink_parent_mkdir_rmdir`, `pinned_symlink_parent_mkdir_rmdir`, and `subtree_rename_cached_unrelated`; `--workload-set all` combines all comparable sets, including `matcher_descendant_readdir` and `matcher_descendant_readdirplus`; those matcher-descendant workloads are skipped unless `--matcher-extra-rules > 0`; and repeated `--workload <name>` overrides the named set with an explicit workload list.

The default policy is intentionally simple but non-empty when `--policy-preset fallback-unsafe-policy` (the default) is selected:

```text
--visibility-default visible
--hidden /.screenfs-bench/hidden
--mutability-default writable
--readonly /.screenfs-bench/readonly
```

Use `--extra-screenfs-arg` for additional one-off policy experiments, but record the full harness command line from the JSON output when comparing results. For matcher-heavy attribution, `--matcher-extra-rules <N>` appends a hidden rule for `/.screenfs-bench/matcher-heavy/descendant-root`, `N` synthetic hidden `/.screenfs-bench/matcher-heavy/hidden-XXXX` subtree rules, `N` exact readonly `/.screenfs-bench/matcher-heavy/visible-XXXX/readonly-XXXX.txt` rules, and `N` visible leaf carve-outs at `/.screenfs-bench/matcher-heavy/descendant-root/branch-XXXX/leaf-XXXX.txt`. The fixture shape matches that policy: a hidden descendant root, hidden subtree buckets, readonly visible buckets, and non-empty descendant branches that become bridge-visible only because the visible leaf carve-outs exist. `matcher_hidden_stat_miss`, `matcher_descendant_readdir`, and `matcher_descendant_readdirplus` therefore require `--matcher-extra-rules > 0`; without it, the descendant workloads are filtered out of named sets or fail fast when requested explicitly. Use this knob for stress/attribution experiments, not default-policy claim evidence.

The official repo-local harness therefore now exposes explicit policy/workload selection and records it in the JSON/Markdown provenance. Keep these buckets separate:

- `--policy-preset fast-path-cache-eligible`: `--visibility-default visible --mutability-default writable` with no hidden/readonly carve-outs; this is the only built-in policy shape that can exercise the current read/write fast path as claim-grade speedup evidence.
- `--policy-preset fallback-unsafe-policy` (default): the historical hidden/readonly mounted policy, useful for general visibility/mutability coverage but not a cache-eligible claim policy.
- additional unsafe shapes (for example symlink-target-sensitive policy mixes or extra policy args) should use their own named matrix entry via `--policy-label`, not be merged into the cache-eligible bucket.

## Running

Run as a regular user only; do not use `sudo`. Build once, then benchmark the release binary:

```bash
cargo build --release
scripts/bench-screenfs.py \
  --iterations 10 \
  --warmups 3 \
  --output-json docs/artifacts/current-benchmark-result.json \
  --output-md docs/artifacts/current-benchmark-result.md \
  --output-svg docs/artifacts/current-benchmark-boxplot.svg
```

Or let the harness build first, without including build time in measured workloads:

```bash
scripts/bench-screenfs.py --build --output-json /tmp/screenfs-bench.json --output-md /tmp/screenfs-bench.md
```

On this environment, the default release run is typically around a minute. Use the smaller sizing knobs below for quick harness smoke checks only, not for performance evidence.

Useful sizing options:

```text
--read-mib 64
--write-mib 64
--small-io-bytes 4096
--small-io-ops 1024
--rand-io-ops 16384
--concurrency-workers 4
--cache-control warm
--open-read-close-ops 4096
--sync-bytes 4096
--sync-ops 128
--sync-4k-fsync-every 32
--small-files 2000
--dir-entries 5000
--hidden-misses 2000
--matcher-extra-rules 0
--matcher-misses 2000
--symlink-parent-mutations 2000
--iterations 10
--warmups 3
```

### Cache-control modes

- `--cache-control warm` is the default and leaves cache state unchanged.
- `--cache-control posix-fadvise-read-fixture` is a **non-root read-side cold-cache approximation**. It calls `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` before each warmup and measured sample for selected benchmark-owned backing-source regular files under `source/.screenfs-bench`.
- Supported read-side workloads are currently `seq_read`, `small_read`, `rand_read_4k`, and `concurrent_rand_read_write_4k`. Write-side workloads still run in the same benchmark matrix, but they do not receive cache-control timing application.
- Mounted samples evict the backing source fixture files rather than mounted-path aliases.
- This mode does **not** reset write-side state, dentry/inode/device caches, or the whole host page cache, and it does **not** authorize privileged `drop_caches` steps. Treat it as documented approximation coverage only, not literal strict cold-cache evidence.
- The current checked-in approximation bundle is [`artifacts/read-write-cold-cache-approx/summary.md`](artifacts/read-write-cold-cache-approx/summary.md).

Use larger values for stable release evidence.

For claim-grade per-open-cache before/after pairs, start from the cache-eligible preset and minimum workload set, then raise sizing/iteration knobs as needed:

```bash
scripts/bench-screenfs.py \
  --policy-preset fast-path-cache-eligible \
  --workload-set per-open-cache-minimum \
  --iterations 10 \
  --warmups 3 \
  --output-json /tmp/screenfs-per-open-cache.json \
  --output-md /tmp/screenfs-per-open-cache.md
```

## Formal performance-claim bar

Small smoke runs, such as `--iterations 1 --warmups 1`, only prove that the harness, mount, workload plumbing, and result serialization work. Do not use them as evidence for a performance claim.

For a claim-grade before/after comparison, use at least:

```text
--iterations 10
--warmups 3
```

Then inspect the JSON raw samples for the affected workload, not just the Markdown table. A claim should report p50/median plus p90/p95/p99 tail latency, raw-sample spread or variance, and whether samples overlap enough to make the result inconclusive. If repeated runs disagree, record the result as inconclusive instead of selecting the favorable run.

For the active follow-up surfaces in [`performance-roadmap.md`](performance-roadmap.md), “claim-grade” means a real before/after pair on the same machine, kernel, backing filesystem, policy, workload sizing, and cache assumptions. Use deeper attribution only to explain the delta after the benchmark pair exists: for `resolved_virtual_path_from_path`, matcher-family/`candidate_order`, or `readdir`/`readdirplus` attr/symlink/page-selection work, do not substitute smoke-only counters for the before/after harness evidence.

## Reading results

The JSON output is the source of truth. The Markdown output is a human-readable summary, and the optional SVG output is a box plot of raw sample timings. The harness records the full benchmark command line, git worktree clean/dirty state, and the `screenfs` binary SHA256 so human review does not lose dirty/uncommitted binary provenance. Summaries include p50/median, p90, p95, and p99. Comparable workloads include a `mounted_over_native_median` ratio:

- `1.0`: ScreenFS median approximately matches native path for this workload
- `>1.0`: ScreenFS mount is slower than native path
- `<1.0`: ScreenFS mount measured faster; treat this cautiously and inspect sample variance/cache effects

Do not compare results from different machines, kernels, storage devices, CPU governors, or policy inputs unless those differences are the subject of the experiment.

## Evidence requirements for performance changes

Any new or expanded performance claim must be backed by benchmark and/or perf-counter evidence that directly measures the changed surface. Reuse the repo-local harness where it fits, but do not treat unrelated workload wins as proof for a different hot path.

When a performance change is proposed or merged, record at least:

- git revision, whether the worktree was clean, and a visible dirty-status summary in the human-readable report when the worktree was not clean
- benchmark command line used for the run
- JSON result path or attached result
- `screenfs` binary path plus binary SHA256/provenance, especially when comparing dirty or otherwise uncommitted binaries
- kernel, `fusermount3`, rustc/cargo, backing filesystem, CPU/storage notes when relevant
- whether the binary was built with `--features perf-counters` and the stderr counter summary when used for attribution
- for resolved-path hot-path claims, whether attribution came only from the retained aggregate `resolved_virtual_path` line, from the split `source_root_path` / `resolved_virtual_path_from_path` / `resolved_virtual_path_from_open_fd` helper counters, or from the current `resolved_virtual_path_from_path_*` sub-counters, plus the relevant helper-focused smoke output
- for matcher hot-path claims, whether attribution came only from aggregate `matcher_candidates` or from the current matcher-family / `matcher_candidate_order` counters or traces; aggregate candidate count alone does not prove allocation or duplicate-removal cost, and current family/order counters are still whole-run aggregate signals
- for `readdir`/`readdirplus` claims, whether attribution came only from broad `readdir*_attr_generation_scan` lines or from the current attr-build / symlink-visibility / candidate-selection / page-commit counters; even the split directory counters do not replace before/after workload evidence
- workload sizes, warmups, iterations, and cache-control assumptions
- before/after p50/median ratios, p90/p95/p99 tail latency, and raw-sample variance for the affected workload
- separate post-change correctness validation command and result, typically `cargo test --all-targets --all-features`; record that in final evidence alongside the benchmark because the harness does not run correctness validation for you

When the changed surface is `resolved_virtual_path` or adjacent guard/path-resolution work, also record:

- correctness regression coverage from [`../src/path_tests.rs`](../src/path_tests.rs) (`rejects_following_symlinks_outside_source_root_but_allows_link_itself`)
- symlink escape/`ENOENT` coverage from [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`symlink_to_outside_source_root_stays_visible_but_following_ops_return_enoent`, `symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects`)
- perf-counter smoke/helper attribution from [`../src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) (`perf_counters_split_resolved_virtual_path_sources` for the split counters, plus `perf_counters_record_data_path_splits_on_success` for size-bucket coverage)
- when using the current `resolved_virtual_path_from_path_*` sub-counters, note exactly which sub-steps were cited and avoid claiming more precision than those counters actually provide

When the changed surface is matcher/index work, also record:

- which matcher families were present in the tested policy mix, and whether `--matcher-extra-rules` was used to create a rule-rich mounted workload
- whether the current `matcher_family_candidates.*` / `matcher_candidate_order.*` counters were sufficient, or whether extra traces/microbenchmarks were needed to isolate `candidate_order` / `descendant_candidate_order` allocation or duplicate-removal work instead of only aggregate matcher volume
- a before/after benchmark pair for the workload that actually exercises the matcher-heavy path; do not rely on an unrelated workload win

When the changed surface is `readdir`/`readdirplus`, also record:

- whether the evidence isolates directory scan, attr build, symlink target visibility checks, page candidate selection, and returned-page commit separately with the current split counters, or only reports a broader subset
- that bounded page behavior, stable resume cookies/shared cookie domain, and returned-page-only `readdirplus` lookup-ref pinning remained unchanged
- the specific directory-heavy workload and size budget used for the before/after comparison

For host-side async/io_uring experiments, also follow the selective data-path evidence rules in [`docs/operations.md`](operations.md) and [`docs/artifacts/current-file-data-path-async-feasibility.md`](artifacts/current-file-data-path-async-feasibility.md).

### Additional gate for the implemented per-open read/write cache

The current implementation is intentionally conservative: only cache-eligible `visible`/`writable` policy shapes skip repeated `read`/`write` guard work. Policy mixes that can still change the current-path proof—such as hidden/visible carve-outs, readonly/writable carve-outs, or symlink-target-sensitive paths—continue to re-run guards, and `fallocate`/`copy_file_range` remain on the per-call policy path. This is a handle-local fast path only; it is not a broad/global authorization cache, negative cache, or reopen-by-cached-path scheme.

Treat [`artifacts/managed-fio-attribution-summary.md`](artifacts/managed-fio-attribution-summary.md), [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json), and the checked-in `managed-fio-attribution-*` companions as supplemental warm-cache attribution evidence only. That rerun uses an explicit custom unsafe policy shape (`--visibility-default visible --hidden /hidden --mutability-default writable --readonly /readonly`), not the official harness `fallback-unsafe-policy` preset, plus the `docs/artifacts/current-fio-attribution.job` fio workload under non-root `fusermount3` semantics. The env JSON records the custom policy label, dirty worktree, helper binary SHA256, and ScreenFS binary SHA256 provenance. Use it to inspect native vs managed passthrough vs ScreenFS floor attribution and the split-counter shape (`read_guard_path` 95.3% of `fuse_op.read`, `write_guard_mutation` 94.8% of `fuse_op.write` in the checked-in summary), not as a before/after optimization claim.

For the active per-open-cache goal, the checked-in claim-grade before/after pair is [`artifacts/per-open-cache-claim/summary.md`](artifacts/per-open-cache-claim/summary.md) with the `before-*` / `after-*` JSON, Markdown, SVG, and PNG companions in [`artifacts/per-open-cache-claim/`](artifacts/per-open-cache-claim/). That pair measures `--policy-preset fast-path-cache-eligible --workload-set per-open-cache-minimum --iterations 10 --warmups 3` under the same non-root `fusermount3` FUSE contract, and each JSON captures dirty-worktree status, ScreenFS source provenance, command line, filesystem/kernel notes, and binary SHA256 provenance. Read the checked-in pair as workload-scoped rather than a blanket win: the current p50 summary reports after/before `0.832x` for `rand_read_4k`, `0.714x` for `rand_write_4k`, `0.738x` for `sync_write_4k`, and `0.996x` for `small_open_read_close`.

Keep [`artifacts/current-fio-per-open-cache-summary.md`](artifacts/current-fio-per-open-cache-summary.md) and the `current-fio-per-open-cache-*` companions as smoke-only fast-path confirmation; they are still useful for split-counter sanity checks, but they are not the claim-grade before/after artifact.

For this surface, claim-grade evidence must be a before/after pair per policy preset, not a single mixed-policy run. The current official harness policy above injects hidden/readonly rules, so by itself it belongs to the unsafe fallback bucket rather than the cache-eligible fast-path bucket. The minimum first workload set for the before/after policy matrix is available as `--workload-set per-open-cache-minimum`:

- `rand_read_4k`
- `rand_write_4k`
- `sync_write_4k`
- `small_open_read_close`

Later expand the same matrix with:

- open/stat-heavy workload
- `readdir`/`readdirplus` workload
- policy-heavy workload (for example matcher-rich or carve-out-rich policy mixes)

Store claim-grade per-open-cache artifacts in a non-`current-*` before/after layout so runs are not overwritten or mixed. Include revision, policy preset, and workload-set provenance in every filename. For example:

```text
docs/artifacts/per-open-cache-claim/
  before-<rev>-<policy-preset>-<workload-set>.json
  before-<rev>-<policy-preset>-<workload-set>.md
  after-<rev>-<policy-preset>-<workload-set>.json
  after-<rev>-<policy-preset>-<workload-set>.md
  env-before-<rev>-<policy-preset>-<workload-set>.json
  env-after-<rev>-<policy-preset>-<workload-set>.json
```

Keep `current-fio-per-open-cache-*` only as the latest smoke snapshot; do not reuse that prefix for claim-grade before/after evidence.

When updating this surface or claiming a speedup on it, record dedicated correctness evidence in addition to the benchmark pair:

- post-open visible→hidden transition coverage (for example rename into a hidden subtree or hidden ancestor alias) proving unsafe-policy `read`/`write` does not continue on stale authorization and instead falls back to the existing `ENOENT` revalidation path before data I/O
- post-open writable→readonly transition coverage proving unsafe-policy `write` does not continue on stale authorization and instead falls back to the existing `EROFS` revalidation path before data I/O
- final-component symlink retarget and ancestor symlink retarget coverage split by policy shape: cache-eligible default `visible`/`writable` handles keep pinned-fd behavior without cached-path reopen, while hidden/readonly target concern cases revalidate and fail closed
- leaf rename, ancestor rename, and unlink coverage proving which cases retain documented pinned-fd behavior and which cases become current-path authorization failures, without silently extending cache beyond the documented policy shape
- focused regression/perf coverage from [`../src/fs/tests/perf.rs`](../src/fs/tests/perf.rs) (`perf_counters_record_data_path_splits_on_success`, `perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe`, `perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures`, `perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path`), [`../src/fs/tests/data_mutations.rs`](../src/fs/tests/data_mutations.rs) (`cache_eligible_opened_file_read_write_keep_pinned_fd_after_host_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_ancestor_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_unlink`, `opened_file_read_keeps_pinned_fd_after_host_rename_but_write_fails_closed`, `opened_file_read_write_fail_closed_after_host_rename_into_hidden_subtree`), [`../src/fs/tests/symlinks_access_create.rs`](../src/fs/tests/symlinks_access_create.rs) (`cache_eligible_opened_symlink_read_keeps_pinned_fd_after_final_retarget`, `cache_eligible_opened_symlink_read_keeps_pinned_fd_after_ancestor_retarget`, `opened_symlink_read_revalidates_hidden_target_after_retarget`), and [`../src/fs/tests/mutability.rs`](../src/fs/tests/mutability.rs) (`opened_symlink_write_revalidates_readonly_target_after_retarget`)
- the normal post-change correctness validation command/result (typically `cargo test --all-targets --all-features`) alongside the before/after benchmark result

The current repo-local benchmark harness does not directly exercise all of those post-open mutation semantics, so dedicated regression tests are required; benchmark wins alone are insufficient.

### Metadata/open-path umbrella contract after per-open cache

With the current checked-in per-open-cache claim, metadata/open-path fixed overhead remains one lane in the completed all-five-lane post-metadata follow-up checkpoint and in any future refreshed attempt, not the sole follow-up implementation target. The historical five-lane model captured under [`Current next-priority candidate gates`](#current-next-priority-candidate-gates) and [`artifacts/current-next-performance-candidates.md`](artifacts/current-next-performance-candidates.md) is `open_confined` / `openat2`, `readdirplus` page/scan, matcher-heavy policy path, read/write follow-up beyond the completed fallback small-I/O baseline, and mutation invalidation breadth-first. For refreshed unfinished-work ordering, keep `open_confined` / `openat2` first; read the already-implemented `lookup`/`getattr` same-path no-follow attr-reuse slice as current dirty-worktree slice evidence under this umbrella; then keep the later unfinished `readdirplus`/matcher/read-write/mutation lanes after that. This section defines the umbrella contract for the metadata/open-path row: fixed-overhead reduction on metadata/open-path operations (`lookup`, `getattr`, `open`, `readlink`, `access`) with `statfs` kept as a non-regression guardrail. Treat the current `small_stat_open_read` comparable workload as exploratory only for this surface: it is not split enough to support claim-grade attribution or acceptance by itself.

The formal harness now exposes separate named workloads for this candidate. Before claiming it, run the workload matrix with these names rather than a mixed aggregate workload:

- `metadata_lookup`
- `metadata_getattr`
- `metadata_open`
- `metadata_opendir` (focused opendir/open-confined probe; attribution/non-regression context rather than one of the five primary metadata acceptance workloads)
- `metadata_readlink`
- `metadata_access`
- `metadata_statfs`
- `sync_flush_only`
- `sync_fsync_only`
- `sync_release_flush`
- `readdir_basic`
- `readdirplus_basic`

Do not collapse these surfaces back into `small_stat_open_read`, `write_fsync_close`, or a single directory workload when deciding acceptance. If a future harness change breaks isolation for one of `lookup`/`getattr`/`open`/`readlink`/`access`/`statfs`, document that harness gap first and treat the candidate as not ready for implementation or measurement claims.

Store claim-grade artifacts for this candidate under a dedicated non-`current-*` directory so the baseline and follow-up runs are not overwritten. Use:

```text
docs/artifacts/metadata-open-path-claim/
  before-<rev>-<policy-label>-<workload>.json
  before-<rev>-<policy-label>-<workload>.md
  before-<rev>-<policy-label>-<workload>.svg
  before-<rev>-<policy-label>-<workload>.png
  after-<rev>-<policy-label>-<workload>.json
  after-<rev>-<policy-label>-<workload>.md
  after-<rev>-<policy-label>-<workload>.svg
  after-<rev>-<policy-label>-<workload>.png
  env-before-<rev>-<policy-label>-<workload>.json
  env-after-<rev>-<policy-label>-<workload>.json
```

For native/passthrough/ScreenFS fixed-overhead floor artifacts, keep the same `<rev>-<policy-label>-<workload>` stem and append the side explicitly (for example `rawlat-<rev>-<policy-label>-<workload>-native.json`, `rawlat-...-passthrough.json`, `rawlat-...-screenfs.json`, plus matching raw-sample logs). When `--matcher-extra-rules <N>` is used, include that cardinality in the policy label itself (for example `<policy-label>-matcher32`) so readonly-heavy and matcher-heavy runs are distinguishable in filenames without opening the JSON.

Acceptance for the full metadata/open-path candidate is concrete. The `lookup`/`getattr` same-path no-follow attr-reuse slice is already implemented in the current code path (`src/fs/guards.rs` known-attr flow through `reply_entry_for_path()`, `attr_for_path()`, and `guard_read_path_with_known_attr()`), with focused perf coverage in `src/fs/tests/perf.rs` and slice-scoped before/after evidence in [`artifacts/metadata-open-path-claim/attr-reuse-worktree-19ca971/summary.md`](artifacts/metadata-open-path-claim/attr-reuse-worktree-19ca971/summary.md). Read that bundle as dirty-worktree current-slice evidence only, not as a full metadata/open-path candidate claim. For the attr-reuse slice, record `metadata_lookup` and `metadata_getattr` before/after evidence at minimum, keep `metadata_open`/`metadata_access` as context or non-regression when feasible, and include helper-level evidence such as a `stat_child_no_follow` counter/trace/focused test so the same-path double-stat removal is not inferred from latency alone.

1. run a before/after pair on the same machine, kernel, backing filesystem, cache assumption, and non-root `fusermount3` contract with at least `--iterations 10 --warmups 3`
2. collect dedicated before/after pairs for every primary workload (`metadata_lookup`, `metadata_getattr`, `metadata_open`, `metadata_readlink`, `metadata_access`) under `fast-path-cache-eligible` and at least one unsafe policy row with an explicit `--policy-label`; the unsafe matrix is incomplete unless it also records `fallback-unsafe-policy` and at least one readonly/carve-out-heavy row with `--matcher-extra-rules >= 32` or an explicit note explaining why fewer extra rules were unavoidable
3. a metadata/open-path win may be claimed only when at least 4 of those 5 primary workloads in the targeted policy row show after/before `<= 0.90x` at median and `<= 0.95x` at both p95 and p99; the remaining primary workload and `metadata_statfs` may be neutral but must stay within `<= 1.05x` median and `<= 1.10x` p99
4. the sync guardrail workloads (`sync_flush_only`, `sync_fsync_only`, `sync_release_flush`) must be reported separately; do not roll `flush`, `fsync`, and `release(flush=true)` back into a single `write_fsync_close` acceptance number. When the active candidate is metadata/open-path rather than sync, each sync workload must stay within `<= 1.05x` median and `<= 1.10x` at both p95 and p99
5. `readdir_basic` and `readdirplus_basic` must also stay split. A `readdir` claim is invalid if `fuse_op.readdir` is zero or if all matching `readdir_*` split counters remain zero. A `readdirplus` claim is invalid if `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, or `readdirplus_page_commit` is zero, because that means the workload failed to exercise the intended path
6. native vs passthrough vs ScreenFS fixed-overhead floor artifacts must use the same raw-sample metric on all three sides. For fio-based raw-lat runs, use raw completion-latency (`clat`) samples everywhere rather than mixing `clat` with total `lat` or fio percentile summaries. A fixed-overhead improvement claim is incomplete unless the ScreenFS/passthrough gap shrinks on that same p50/p95/p99 raw-sample metric

### Sync-surface slice claims

A sync-specific slice may be claimed separately from the full metadata/open-path candidate when it touches only `flush()` / `fsync()` / `release(flush=true)` helper cost. Use same-machine before/after rows with `--workload-set sync-surface --iterations 10 --warmups 3` under the targeted policy preset and keep the companion policy preset as context. A narrow sync claim must show the targeted primary rows (`sync_flush_only` and/or `sync_fsync_only`) at after/before `<= 0.90x` median and `<= 0.95x` at p95/p99. Non-target sync rows and companion-policy rows must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99. `sync_release_flush` remains context unless `file_sync.release_flush` is non-zero in both before and after rows. Keep `file_sync.*`, `fuse_op.flush`, `fuse_op.fsync`, `fuse_op.release`, and state lock counters in the artifact.


### Post-metadata directory and read-only close follow-up

The current post-metadata directory/read-only-close implementation has a dedicated before/after bundle at [`artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md`](artifacts/post-metadata-follow-up-claim/worktree-3cb95ba/summary.md). Treat that bundle as scoped to directory-surface and read-only-close workloads only; do not merge it with metadata/open-path or per-open-cache claims.

For any refreshed directory/read-only-close follow-up, claim-grade evidence must still include both latency and attribution:

- `--policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3` before/after evidence.
- For matcher-descendant claims, a dedicated matcher row such as `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set matcher-descendant-directory`; for broader directory-hot policy claims, keep a separate `directory-surface` row.
- Read-only close workloads `read_only_open_close` and `read_only_open_read_close`, plus write-capable guardrails `write_open_write_close` and `write_open_fsync_close`, or an explicitly equivalent artifact.

Directory claims must report `readdir_basic` and `readdirplus_basic` p50/p95/p99 together with `readdir_directory_scan`, `readdir_attr_generation_scan`, `readdir_candidate_selection`, `readdir_page_commit`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_candidate_selection`, and `readdirplus_page_commit`. If symlink visibility logic changed, the regular-file directory fixture is insufficient; add `--workload-set directory-symlink-surface` or equivalent evidence that exercises `readdir_symlink_visibility` and `readdirplus_symlink_visibility`.

Read-only close claims must report `fuse_op.flush`, `file_sync.flush`, `fuse_op.release`, `file_sync.fsync`, and read-only/write p50/p95/p99. `FOPEN_NOFLUSH` success is a mounted fact, not an assumption: if `fuse_op.flush` does not drop for read-only handles, document that and rely only on a separately verified read-only `flush()` no-op guard. Do not claim `release(flush=true)` improvement from a run where `file_sync.release_flush` is zero.

## Current next-priority candidate gates

The evidence snapshot and scope labels for these five rows live in [`artifacts/current-next-performance-candidates.md`](artifacts/current-next-performance-candidates.md). The `70a3105` all-five retry is now a completed historical checkpoint with rejected implementations documented under dedicated artifacts; these gates remain the source of truth for future refreshed attempts on the same five rows. For refreshed unfinished-work ordering, keep `open_confined` / `openat2` first; the already-implemented `lookup`/`getattr` same-path no-follow attr-reuse slice is tracked separately above as slice-scoped dirty-worktree evidence rather than as a future row in these five gates. The five historical rows below remain the source-of-truth gates for the later `readdirplus`/matcher/read-write/mutation lanes plus the leading `open_confined` row. Unless a row says otherwise, the minimum claim-grade floor is a same-machine before/after pair with matching cache assumptions and at least:

```text
--iterations 10
--warmups 3
```

Scope labels:

- `smoke`: workload plumbing plus non-zero counters only; not claim-grade
- `slice`: claim-grade pair proves a narrow helper/workload effect, but not the full candidate
- `claim`: the full row-specific gates below are satisfied
- `rejected`: before/after evidence or tail behavior failed the row, so the implementation is not kept

### `readdirplus` page/scan

- Current label: `slice`
- Current caveat: treat [`artifacts/readdirplus-page-scan-path-join/summary.md`](artifacts/readdirplus-page-scan-path-join/summary.md) as rejected page/scan retuning only: some focused 20k counters/tails improved, but 5k `readdir_basic` regressed badly (`1.557941x` p50), 20k `readdirplus_basic` p50 still regressed (`1.036241x`), and the implementation was reverted. Also treat [`artifacts/readdirplus-bounded-candidate-heap/summary.md`](artifacts/readdirplus-bounded-candidate-heap/summary.md) as rejected bounded candidate-selection evidence: 20k `readdirplus_basic` p50 improved (`0.882012x`) but p95/p99 missed (`0.998063x`/`1.002933x`), fast `directory-surface` `readdirplus_basic` regressed (`1.062710x`), matcher32 `readdir_basic` regressed (`1.480425x`), and the code was reverted/not kept.
- Required before/after rows:
  - `--policy-preset fast-path-cache-eligible --workload-set directory-surface`
  - `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface` when claiming directory-hot policy or matcher-adjacent improvements
  - a focused large-directory `--workload readdirplus_basic` row (for example the existing 20k-entry shape or an explicitly recorded equivalent) for page/scan claims across directory sizes
  - `--workload-set directory-symlink-surface` if symlink visibility logic changed
- Required counters:
  - `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, `readdirplus_page_commit`
  - paired `readdir` non-regression rows must also report `fuse_op.readdir`, `readdir_directory_scan`, `readdir_attr_generation_scan`, `readdir_candidate_selection`, `readdir_page_commit`
- Claim gate:
  - every claimed `readdirplus_basic` size/policy row must show after/before `<= 0.90x` median and `<= 0.95x` at p95 and p99
  - paired `readdir_basic` rows, and any required symlink rows, must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - attr-only or counter-only movement without those latency rows remains `slice`, not `claim`

### read/write follow-up beyond fallback small-I/O baseline

- Current label: `smoke`
- Current caveat: [`artifacts/read-write-small-io-guard-reuse/summary.md`](artifacts/read-write-small-io-guard-reuse/summary.md) is a completed fallback small-I/O claim baseline. The same bundle's `fast-path-cache-eligible` row is context/non-regression only. [`artifacts/read-write-followup-current/summary.md`](artifacts/read-write-followup-current/summary.md) adds current fast/fallback default rows, alternate same-default-`64MiB` sequential-size rows for a reduced small/random mix, and separate `256MiB` larger-sequential coverage rows. [`artifacts/read-write-storage-backed/summary.md`](artifacts/read-write-storage-backed/summary.md) adds local `/home/...` btrfs warm-cache storage-backed coverage for both `read-write-surface` and `read-write-concurrency` / `concurrent_rand_read_write_4k` with `--concurrency-workers 4`. [`artifacts/read-write-cold-cache-approx/summary.md`](artifacts/read-write-cold-cache-approx/summary.md) now adds local btrfs **non-root read-side cold-cache approximation** coverage via `--cache-control posix-fadvise-read-fixture` for fast/fallback surface and concurrency rows. Its JSON-backed cache-control stats record method `posix-fadvise-read-fixture`, `42` applications on each surface row, `14` on each concurrency row, read-side-only fixture selection from `source/.screenfs-bench/read/seq.bin` or `source/.screenfs-bench/concurrency-read/worker-000000.bin`, and mounted runs evicting those backing source files rather than mounted aliases. The split-counter shape still stays near-zero on the fast rows (warm-cache default `16ns` / `16ns`, 256MiB `13ns` / `14ns`, storage-backed surface `17ns` / `19ns`, storage-backed concurrency `21ns` / `20ns` for `read_guard_path` / `write_guard_mutation`), while fallback rows remain guard-heavy (warm-cache default `18105ns` / `24486ns`, storage-backed surface `24827ns` / `31047ns`, storage-backed concurrency `34768ns` / `46630ns`). These checked-in storage-backed/concurrency/cold-cache-approx artifacts are still dirty-worktree local coverage only, not before/after claims, and the approximation does not reset write-side, dentry, inode, or device caches. Strict literal cold-cache remains intentionally unclaimed under the current non-root harness contract.
- Required before/after rows:
  - `--policy-preset fast-path-cache-eligible --workload-set read-write-surface` when claiming a separate fast-policy read/write win
  - `--policy-preset fallback-unsafe-policy --workload-set read-write-surface` as baseline/non-regression context when the same change also touches fallback/unsafe policy handling
  - keep `seq_read` / `seq_write` inside the claim-grade matrix, or record an explicitly equivalent same-machine row, when claiming larger-sequential movement
  - storage-backed and concurrency coverage now exist in [`artifacts/read-write-storage-backed/summary.md`](artifacts/read-write-storage-backed/summary.md), and non-root read-side cold-cache approximation coverage now exists in [`artifacts/read-write-cold-cache-approx/summary.md`](artifacts/read-write-cold-cache-approx/summary.md), but any claim that depends on those conditions still needs its own same-machine before/after row with matching cache/storage/concurrency provenance; `posix-fadvise-read-fixture` is approximation coverage only and this contract still does not authorize privileged `drop_caches` runs
  - if the change touches the cache-eligible handle fast path, keep `--workload-set per-open-cache-minimum` as a non-regression/context row rather than substituting it for `read-write-surface`
- Required counters:
  - `read_handle_snapshot`, `read_guard_path`, `read_io`, `write_handle_snapshot`, `write_guard_mutation`, `write_io`
  - `read_size_bucket.*` / `write_size_bucket.*`
- Claim gate:
  - the checked-in fallback row remains baseline/context only for this follow-up lane
  - any separate fast-policy `read-write-surface` claim must show at least 3 of `small_read`, `small_write`, `rand_read_4k`, `rand_write_4k` at after/before `<= 0.90x` median and `<= 0.95x` at p95/p99
  - the remaining small-I/O row, plus `seq_read` and `seq_write`, must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99 on the targeted row
  - before implementation, choose one read/write sub-axis as the targeted claim row and record that choice in the attempt artifact; coverage-only rows cannot be promoted after the fact
  - any larger-sequential claim must use matching same-machine `seq_read` and/or `seq_write` rows (for example the existing `256MiB` shape) and the targeted sequential row(s) must show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99; non-target small/random rows in the same matrix must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - any storage-backed claim must use matching local-storage provenance before/after rows and apply the same workload-specific threshold: targeted `read-write-surface` small-I/O rows follow the 3-of-4 small-I/O gate above, targeted sequential rows follow the larger-sequential gate above, and non-target rows remain non-regression context
  - any concurrency claim must use matching `read-write-concurrency` / `concurrent_rand_read_write_4k` rows with the same `--concurrency-workers` value and show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99; companion surface rows touched by the same change must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - any cold-cache-sensitive claim under this non-root harness must use matching `--cache-control posix-fadvise-read-fixture` before/after rows and may claim only read-side approximation movement. Targeted read-side rows must show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99, while write-side or non-target rows remain non-regression context. The approximation remains read-side-only and is not literal `drop_caches`
  - the checked-in storage-backed bundle and `posix-fadvise-read-fixture` bundle are coverage only; if only split counters move or only the completed fallback bundle plus approximation coverage exists, keep the result at `smoke` or `slice`
- Current checked-in baseline: the fallback row in [`artifacts/read-write-small-io-guard-reuse/summary.md`](artifacts/read-write-small-io-guard-reuse/summary.md) satisfies the old small-I/O gate with `small_read` `0.552451x`, `small_write` `0.800006x`, and `rand_read_4k` `0.888775x`, while `rand_write_4k` remains within non-regression (`0.905193x` p50, `0.984318x` p95, `1.002145x` p99). Treat that row as baseline/context here rather than a second independent fast-policy or storage/concurrency claim. Treat [`artifacts/read-write-followup-current/summary.md`](artifacts/read-write-followup-current/summary.md) and [`artifacts/read-write-storage-backed/summary.md`](artifacts/read-write-storage-backed/summary.md) separately as current coverage only, not as additional claim-grade rows.

### mutation invalidation breadth-first

- Current label: `smoke`
- Current caveat: treat the checked-in `mutation-invalidation-range-scan` bundle as rejected/stale evidence for the current workload scope. Also treat [`artifacts/mutation-invalidation-breadth-index/summary.md`](artifacts/mutation-invalidation-breadth-index/summary.md) as rejected/slice-context only: it reduced `invalidations.scanned_entries` (`15392 -> 572` on the set, `10764 -> 52` on the focused subtree row) but failed the latency gate (`symlink_parent_mkdir_rmdir` p50 `1.239281x`, `pinned_symlink_parent_mkdir_rmdir` p50 `1.295362x`, set `subtree_rename_cached_unrelated` p50 `1.439480x`, focused subtree p50 `1.027902x`), so the child-index Rust implementation was reverted/not kept. The active follow-up lane is invalidation-breadth-first, not eviction-first. Before another implementation attempt, refresh a focused current-shape `subtree_rename_cached_unrelated` baseline and target `invalidations.scanned_entries` reduction; mounted post-`FORGET` observability is needed only when the claim explicitly includes eviction breadth, otherwise document the absence as limitation/rejected/coverage artifact.
- Required before/after rows:
  - `--workload-set mutation-invalidation`
  - a fresh focused `--workload subtree_rename_cached_unrelated` row on the current workload shape when claiming subtree breadth or ordered-range wins
  - add a separate mounted post-`FORGET` driver or equivalent evidence only if the claim depends on eviction breadth rather than invalidation breadth alone
- Required counters:
  - `invalidations.count`, `invalidations.invalidated_entries`, `invalidations.evicted_entries`, `invalidations.scanned_entries` (`scanned_entries` is the required breadth-first improvement signal; `evicted_entries` is observability/context unless eviction breadth is part of the claim)
  - `state_write_lock_hold`
  - any workload-specific directory counters needed to prove the mounted row actually exercised the intended listing path (for example `readdirplus_*` in the pinned row)
- Claim gate:
  - a breadth optimization must reduce the intended invalidation-breadth counter (`invalidations.scanned_entries` or an explicitly tighter replacement) on the targeted row
  - at least 2 of `symlink_parent_mkdir_rmdir`, `pinned_symlink_parent_mkdir_rmdir`, and `subtree_rename_cached_unrelated` must show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99, and the remaining row must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - mounted post-`FORGET` observability must either come from a dedicated driver/equivalent when eviction breadth is claimed, or be explicitly documented as limitation/rejected/coverage artifact when the claim stays breadth-only
  - if counters improve but latency remains inconclusive, keep the result at `slice`; if tails regress materially, mark the experiment `rejected`

### `open_confined` / `openat2` frequency

- Current label: `smoke`
- Current caveat: the checked-in open-confined smokes are fallback-only, and the rejected [`artifacts/open-confined-frequency/summary.md`](artifacts/open-confined-frequency/summary.md) bundle did not clear the surface gate. The regenerated [`artifacts/current-open-confined-surface-smoke.md`](artifacts/current-open-confined-surface-smoke.md) and [`artifacts/current-metadata-opendir-smoke.md`](artifacts/current-metadata-opendir-smoke.md) show coverage for `open_like.pre_open_guard.*` / `open_like.post_open_revalidation.*` around the `open_confined_openat2` lane, and the focused `metadata_opendir` smoke still isolates the row (`readdir*` counters stay zero), but neither those split counters nor the rejected bundle proves `openat2` dominates the remaining opendir cost or upgrades the row beyond `smoke`. Current fallback smokes instead make `stat_child_no_follow.directory_revalidation` the next investigation/design target inside this lane: it is `15618029/19736380ns` of `stat_child_no_follow` in the focused `metadata_opendir` smoke and `40361300/71934271ns` in `open-confined-surface`, while remaining attribution only rather than a speedup claim. Also treat [`artifacts/open-confined-fd-path/summary.md`](artifacts/open-confined-fd-path/summary.md) as rejected micro-helper evidence only: the `/proc/self/fd/<fd>` builder was behavior-safe, but fallback `open-confined-surface` regressed (`metadata_open` p50 `1.630311x`, `metadata_opendir` p50 `1.419060x`), fast tails missed non-regression, focused `metadata_opendir` regressed, and the implementation was reverted/not kept.
- Required before/after rows:
  - `--policy-preset fallback-unsafe-policy --workload-set open-confined-surface`
  - `--policy-preset fast-path-cache-eligible --workload-set open-confined-surface`
  - add explicit `--workload metadata_opendir` when the claim is opendir-specific, and keep `metadata_lookup` / `metadata_getattr` / `metadata_access` rows as non-regression context when the same change also touches broader metadata/open-path guards
- Required counters:
  - `open_confined_openat2`, `open_like.pre_open_guard.*`, `open_like.post_open_revalidation.*`, `source_root_path`, `resolved_virtual_path_from_open_fd`, and aggregate `stat_child_no_follow`
  - the current split counter surface also includes root-path `stat_child_no_follow.host_fstat`, non-root `stat_child_no_follow.parent_open` / `stat_child_no_follow.directory_revalidation` / `stat_child_no_follow.host_fstatat`, shared `stat_child_no_follow.attr_conversion`, and `stat_child_no_follow_context.*` labels such as `path_guard_or_metadata`; focused readlink perf coverage also checks `stat_child_no_follow_context.readlink_pre_open`
  - `stat_child_no_follow.directory_revalidation` is expensive because opened-dirfd current-path validation resolves both `resolved_virtual_path_for_open_file(file)` and `resolved_virtual_path(path, true)` before `fstatat`; any design must preserve `guard_opened_directory_at_path_with_resolver()` semantics or prove a same-request equivalent
  - those sub-counters must not be used to justify relaxing `openat2` confinement, opened-fd revalidation, hidden `ENOENT`, symlink target visibility, or the no-cross-request-cache contract
  - `fuse_op.open`, `fuse_op.opendir`
  - the regenerated [`artifacts/current-open-confined-surface-smoke.md`](artifacts/current-open-confined-surface-smoke.md) and [`artifacts/current-metadata-opendir-smoke.md`](artifacts/current-metadata-opendir-smoke.md) remain smoke/attribution evidence, not speedup evidence by themselves, and not permission to skip validation
  - `readdir*` counters should stay zero on the focused open-confined rows unless the workload definition itself changed
- Claim gate:
  - before code changes on this lane, keep focused perf/correctness coverage for opened-dirfd revalidation and the `stat_child_no_follow.directory_revalidation` split; before any claim, keep a dedicated before/after `open-confined-surface` pair and add explicit `metadata_opendir` when the question is opendir-specific
  - both `metadata_open` and `metadata_opendir` in the targeted row must show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99
  - companion metadata/open-path rows touched by the same change must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - a broader metadata/open-path bundle without an independent `open-confined-surface` pair does not elevate this row beyond `smoke` or `slice`
  - an explicit `metadata_opendir` improvement or counter reduction without a passing `open-confined-surface` pair remains attribution/slice context only; it does not rescue a row where `metadata_open` or `metadata_opendir` fails the surface gate

### matcher-heavy policy path

- Current label: `smoke`
- Current caveat: [`artifacts/current-policy-heavy-matrix-smoke.md`](artifacts/current-policy-heavy-matrix-smoke.md) remains metadata-heavy context, while [`artifacts/current-matcher-descendant-directory-smoke.md`](artifacts/current-matcher-descendant-directory-smoke.md) is dedicated smoke/coverage for the built-in non-empty visible descendant fixture. The current descendant smoke exercises `matcher_descendant_readdir` / `matcher_descendant_readdirplus` and keeps non-zero `matcher_candidate_order.descendant`, `readdir_directory_scan`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_candidate_selection`, and `readdirplus_page_commit` counters, but it is not a before/after pair and does not upgrade this row beyond `smoke`. The rejected [`artifacts/matcher-descendant-combined-probe/summary.md`](artifacts/matcher-descendant-combined-probe/summary.md) bundle preserves a combined before/after attempt only as rejected evidence: the policy-heavy metadata rows stayed near-neutral-to-regressive, `matcher_descendant_readdir` improved only marginally and missed p99 non-regression, `matcher_descendant_readdirplus` regressed, the state gate failed, and the Rust optimization was reverted/not kept. Also treat [`artifacts/matcher-rank-zero-early-stop/summary.md`](artifacts/matcher-rank-zero-early-stop/summary.md) as rejected path-side evidence only: winner semantics were preserved, but `policy-heavy-matrix` missed the improvement gate (`matcher_hidden_stat_miss` p50 `1.008929x`, metadata rows neutral/regressive), the companion `matcher-descendant-directory` row regressed badly (`matcher_descendant_readdir` p50 `1.314779x`, `matcher_descendant_readdirplus` p50 `1.469457x`), and the implementation was reverted/not kept.
- Required before/after rows:
  - `--workload-set policy-heavy-matrix --matcher-extra-rules 32 --policy-label <explicit-matcher32-label>`
  - `--workload-set matcher-descendant-directory --matcher-extra-rules 32 --policy-label glob-matcher-heavy` when claiming descendant-policy behavior; that row must exercise `matcher_descendant_readdir` and/or `matcher_descendant_readdirplus` on the built-in non-empty visible descendant fixture
  - keep `--workload-set directory-surface --matcher-extra-rules 32 --policy-label glob-matcher-heavy` as a separate row when claiming broader directory-hot policy behavior beyond the dedicated matcher-descendant workloads
  - add `--workload-set directory-symlink-surface` or an explicitly equivalent row if descendant/symlink visibility logic changed
  - if no suitable mounted descendant row exists yet, add focused trace/microbenchmark/counter evidence that isolates non-empty visible descendant matching before claiming descendant movement
- Required counters:
  - `matcher_candidates`, `matcher_family_candidates.*`, `matcher_candidate_order.path`, `matcher_candidate_order.descendant`, `matcher_candidate_order_duplicates`, `matcher_candidate_order_seen_slots`, `matcher_candidate_order_ancestor_steps`
  - any targeted `fuse_op.*` rows for the claimed workload family
- Claim gate:
  - the matcher-heavy row must show after/before `<= 0.90x` median and `<= 0.95x` at p95/p99 for `matcher_hidden_stat_miss` plus at least one claimed metadata or directory workload in the same policy row
  - fallback/no-matcher rows and fast-path rows must stay within `<= 1.05x` median and `<= 1.10x` at p95/p99
  - a descendant-specific claim is invalid if `matcher_candidate_order.descendant` only comes from an empty visible matcher; require either the dedicated `matcher-descendant-directory` row above or focused trace/microbenchmark/counter evidence that exercises the same path
  - a duplicate-removal claim is invalid if `matcher_candidate_order_duplicates` stays zero in the claimed row

## Current harness coverage limits

The current harness is useful for mounted-vs-native comparisons on the listed workloads, but it does not directly isolate all planned performance surfaces.

It does not directly measure:

- matcher bucket/index cost or policy evaluator hot paths
- which specific matcher instance or rule dominated; current matcher family/order counters are aggregate shape signals across matcher invocations
- per-workload or per-request `open_confined_openat2` attribution beyond the whole-run perf summary
- path-by-path mutation invalidation breadth beyond the aggregate invalidation/eviction/scanned-entry counters
- internal helper/offload attribution for the small-buffer or sync-surface path without separate counters/traces
- exactly how much time inside `resolved_virtual_path_from_path` sat in overlapping helper sub-steps beyond the current component-walk/canonicalize/confinement/virtual-conversion counters
- exactly which helper inside each current `readdir`/`readdirplus` bucket dominated without additional counters/traces

The `small_read` and `small_write` comparable workloads probe the small-buffer data path end to end. The `write_fsync_close` comparable workload probes the sync surface end to end through repeated open/write/file-fsync/close cycles plus cleanup, but it does not fsync parent directories. These workloads can show mounted-vs-native deltas on those surfaces, but they do not by themselves prove that time moved in a specific internal helper, queueing layer, or offload path, including internal offload helpers; use counters, traces, or focused artifacts when you need attribution.

The `symlink_parent_mkdir_rmdir` ScreenFS-only workload is an intended mounted workload/probe for symlink-parent mutation guard/path-resolution paths around the current `src/fs/guards.rs` request-local reuse area. Each mkdir/rmdir under a visible symlink parent alias is meant to traverse the mounted mutation path and can support before/after investigation of guard/path-resolution behavior. But without counters or traces, it does not prove that a live FUSE request hit a specific internal helper such as `RequestPathResolver`, `guarded_child_mutation`, or `guard_opened_directory_at_path`, and it cannot by itself attribute time to `source_root_path`, `resolved_virtual_path_from_path`, or `resolved_virtual_path_from_open_fd`. It also is not a speedup claim by itself; request-local reuse or canonical source-root reuse changes still need before/after comparisons, tail-latency checks, and preferably raw-sample review on this workload before claiming lower guard/path-resolution overhead.

For claims on the remaining uncovered surfaces, add dedicated perf counters, traces, microbenchmarks, or focused benchmark artifacts alongside the main harness output.

## Guardrails

- Do not use benchmark results to relax hidden `ENOENT`, bridge-visible, symlink target, or mutability precedence semantics.
- Do not treat warm-cache benchmark evidence as cold-cache or storage-device evidence.
- Do not treat `--cache-control posix-fadvise-read-fixture` as write-side, dentry/inode/device-cache, or literal `drop_caches` evidence; it is a documented non-root read-side approximation only.
- Do not run privileged mount or namespace steps from this harness; ScreenFS benchmark evidence remains non-root FUSE evidence, and the harness exits when run as root.
- Do not check in large generated result files unless they are explicitly accepted as current baseline artifacts.

## Passthrough baseline helper

The managed passthrough attribution baseline lives in [`../contrib/fractal-passthrough`](../contrib/fractal-passthrough). Build it with:

```bash
cargo build --release --manifest-path contrib/fractal-passthrough/Cargo.toml
```

Use it only as a minimal `fractal-fuse = 0.4.0` FUSE floor for native vs passthrough vs ScreenFS attribution; it does not implement ScreenFS visibility/mutability policy or source-root confinement. Use a trusted private scratch source tree without external symlinks, and keep running it as a non-root helper.

For any refreshed native vs passthrough vs ScreenFS attribution artifact, rebuild the managed contrib helper and record its path/SHA in [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json) and the Markdown summary. The checked-in managed rerun now captures the contrib helper provenance there; if older `current-fio-attribution-*` files are still present locally, treat them as historical provenance rather than the active managed-helper reference.



Request-local guard context acceptance extends the metadata/open-path matrix with explicit evidence for `metadata_readlink` and source-root/open-fd resolver counters. `opendir` reuse can be measured through the dedicated `metadata_opendir` workload and `open-confined-surface` set together with the surrounding-cost `open_like.pre_open_guard.*` / `open_like.post_open_revalidation.*` counters; do not claim an `opendir` latency speedup from unrelated `readdir` scan improvements or from those split counters alone. Matcher hot-path claims require glob/matcher-heavy rows that show candidate-order or policy-decision cost moving in the expected direction while preserving the debug `candidate_order` API.
