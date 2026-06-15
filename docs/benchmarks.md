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

When enabled by the Cargo feature, ScreenFS prints a stderr summary at shutdown with FUSE operation latency, policy decision latency, aggregate `matcher_candidates`, `matcher_family_candidates.{subtree,direct_child_glob,recursive}`, `matcher_candidate_order.{path,descendant}` latency plus aggregate and order-labeled duplicate/seen-slot/ancestor-step counts, state lock read/write wait/hold latency, `open_confined_openat2` latency, `stat_child_no_follow` latency, `source_root_path` latency, aggregate `resolved_virtual_path` latency, split `resolved_virtual_path_from_path` / `resolved_virtual_path_from_open_fd` latency, `resolved_virtual_path_from_path_component_walk` / `_canonicalize` / `_source_root_confinement` / `_virtual_conversion`, data-path split `read_handle_snapshot` / `read_guard_path` / `read_io` and `write_handle_snapshot` / `write_guard_mutation` / `write_io`, `file_sync.{flush,fsync,release_flush}` sync-helper latency, `read_size_bucket.*` / `write_size_bucket.*` latency for the timed `read_io`/`write_io` segment only, `readdir_directory_scan` / `readdirplus_directory_scan`, attr-build-only `readdir_attr_generation_scan` / `readdirplus_attr_generation_scan` plus scanned entry counts, `readdir_symlink_visibility` / `readdirplus_symlink_visibility`, `readdir_candidate_selection` / `readdirplus_candidate_selection`, `readdir_page_commit` / `readdirplus_page_commit`, and invalidation/eviction counts. `resolved_virtual_path` is retained as the aggregate of the path-walk and opened-fd helper buckets, while `source_root_path` is reported separately because both flows can reuse it. Use the split helper counters and sub-counters as first-pass attribution only: `resolved_virtual_path_from_path_component_walk` overlaps with the more specific canonicalize/confinement timings instead of forming an additive partition, matcher family/order counters show aggregate candidate shape rather than which specific matcher or rule was hottest, and the directory buckets still do not replace before/after benchmark evidence. Treat these counters as attribution evidence for a benchmark or smoke run, not as standalone performance claims. With `--perf-counters --build`, the benchmark harness builds with `--features perf-counters`, parses the shutdown summary into `screenfs.perf_summary` in the JSON output, and includes the raw summary in the Markdown report. If `--perf-counters` is used with an existing `--screenfs-bin`, that binary must already be built with the `perf-counters` feature.

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

It also runs these ScreenFS-only contract workloads:

| workload | Purpose |
| --- | --- |
| `hidden_stat_miss` | hidden path `ENOENT` path and matcher overhead |
| `matcher_hidden_stat_miss` | optional rule-rich hidden path `ENOENT` probe enabled by `--matcher-extra-rules`; omitted from default runs when that knob is `0`; intended for matcher family / `candidate_order` attribution, not default policy claims |
| `symlink_parent_mkdir_rmdir` | repeated mkdir/rmdir under a visible symlink parent alias as an intended mounted probe for symlink-parent mutation guard/path-resolution paths |

The default `--workload-set default` preserves the historical comparable + ScreenFS-only run above. `--workload-set per-open-cache-minimum` runs `rand_read_4k`, `rand_write_4k`, `sync_write_4k`, and `small_open_read_close`; `--workload-set read-only-close-surface` runs `read_only_open_close`, `read_only_open_read_close`, `write_open_write_close`, and `write_open_fsync_close`; `--workload-set all` combines all comparable sets; and repeated `--workload <name>` overrides the named set with an explicit workload list.

The default policy is intentionally simple but non-empty when `--policy-preset fallback-unsafe-policy` (the default) is selected:

```text
--visibility-default visible
--hidden /.screenfs-bench/hidden
--mutability-default writable
--readonly /.screenfs-bench/readonly
```

Use `--extra-screenfs-arg` for additional one-off policy experiments, but record the full harness command line from the JSON output when comparing results. For matcher-heavy attribution, `--matcher-extra-rules <N>` appends `N` synthetic hidden `/.screenfs-bench/matcher-heavy/hidden-XXXX` subtree rules and exact readonly `/.screenfs-bench/matcher-heavy/visible-XXXX/readonly-XXXX.txt` rules, prepares matching fixture entries, and activates `matcher_hidden_stat_miss`; that built-in matcher workload intentionally probes hidden subtree `ENOENT` misses, while the readonly rules document and populate the policy mix for custom `--extra-screenfs-arg`/external experiments. Use it for stress/attribution experiments, not default-policy claim evidence.

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

Treat [`artifacts/managed-fio-attribution-summary.md`](artifacts/managed-fio-attribution-summary.md), [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json), and the checked-in `managed-fio-attribution-*` companions as supplemental warm-cache attribution evidence only. That rerun uses an explicit custom unsafe policy shape (`--visibility-default visible --hidden /hidden --mutability-default writable --readonly /readonly`), not the official harness `fallback-unsafe-policy` preset, plus the `docs/artifacts/current-fio-attribution.job` fio workload under non-root `fusermount3` semantics. The env JSON records the custom policy label, dirty worktree, helper binary SHA256, and ScreenFS binary SHA256 provenance. Use it to inspect native vs managed passthrough vs ScreenFS floor attribution and the split-counter shape (`read_guard_path` 91.2% of `fuse_op.read`, `write_guard_mutation` 90.8% of `fuse_op.write` in the checked-in summary), not as a before/after optimization claim.

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

### Next active candidate after per-open cache: metadata/open-path fixed overhead

After the current per-open-cache claim closes, the next implementation candidate is fixed-overhead reduction on metadata/open-path operations (`lookup`, `getattr`, `open`, `readlink`, `access`) with `statfs` kept as a non-regression guardrail. Treat the current `small_stat_open_read` comparable workload as exploratory only for this surface: it is not split enough to support claim-grade attribution or acceptance by itself.

The formal harness now exposes separate named workloads for this candidate. Before claiming it, run the workload matrix with these names rather than a mixed aggregate workload:

- `metadata_lookup`
- `metadata_getattr`
- `metadata_open`
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

Acceptance for the full metadata/open-path candidate is concrete. A smaller implementation slice, such as lookup/getattr same-path no-follow attr reuse, may report slice-scoped evidence for the directly affected workloads, but must not claim that the full candidate is complete unless the full criteria below are met. For the attr-reuse slice, record `metadata_lookup` and `metadata_getattr` before/after evidence at minimum, keep `metadata_open`/`metadata_access` as context or non-regression when feasible, and include helper-level evidence such as a `stat_child_no_follow` counter/trace/focused test so the same-path double-stat removal is not inferred from latency alone.

1. run a before/after pair on the same machine, kernel, backing filesystem, cache assumption, and non-root `fusermount3` contract with at least `--iterations 10 --warmups 3`
2. collect dedicated before/after pairs for every primary workload (`metadata_lookup`, `metadata_getattr`, `metadata_open`, `metadata_readlink`, `metadata_access`) under `fast-path-cache-eligible` and at least one unsafe policy row with an explicit `--policy-label`; the unsafe matrix is incomplete unless it also records `fallback-unsafe-policy` and at least one readonly/carve-out-heavy row with `--matcher-extra-rules >= 32` or an explicit note explaining why fewer extra rules were unavoidable
3. a metadata/open-path win may be claimed only when at least 4 of those 5 primary workloads in the targeted policy row show after/before `<= 0.90x` at median and `<= 0.95x` at both p95 and p99; the remaining primary workload and `metadata_statfs` may be neutral but must stay within `<= 1.05x` median and `<= 1.10x` p99
4. the sync guardrail workloads (`sync_flush_only`, `sync_fsync_only`, `sync_release_flush`) must be reported separately; do not roll `flush`, `fsync`, and `release(flush=true)` back into a single `write_fsync_close` acceptance number. When the active candidate is metadata/open-path rather than sync, each sync workload must stay within `<= 1.05x` median and `<= 1.10x` at both p95 and p99
5. `readdir_basic` and `readdirplus_basic` must also stay split. A `readdir` claim is invalid if `fuse_op.readdir` is zero or if all matching `readdir_*` split counters remain zero. A `readdirplus` claim is invalid if `fuse_op.readdirplus`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_attr_generation_entries`, `readdirplus_candidate_selection`, or `readdirplus_page_commit` is zero, because that means the workload failed to exercise the intended path
6. native vs passthrough vs ScreenFS fixed-overhead floor artifacts must use the same raw-sample metric on all three sides. For fio-based raw-lat runs, use raw completion-latency (`clat`) samples everywhere rather than mixing `clat` with total `lat` or fio percentile summaries. A fixed-overhead improvement claim is incomplete unless the ScreenFS/passthrough gap shrinks on that same p50/p95/p99 raw-sample metric


### Post-metadata directory and read-only close follow-up

For the directory/read-only-close follow-up, claim-grade evidence must include both latency and attribution:

- `--policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3` before/after evidence.
- A glob/hidden-heavy directory row, for example `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy`, when claiming matcher-descendant or directory-hot policy improvements.
- Read-only close workloads `read_only_open_close` and `read_only_open_read_close`, plus write-capable guardrails `write_open_write_close` and `write_open_fsync_close`, or an explicitly equivalent artifact.

Directory claims must report `readdir_basic` and `readdirplus_basic` p50/p95/p99 together with `readdir_directory_scan`, `readdir_attr_generation_scan`, `readdir_candidate_selection`, `readdir_page_commit`, `readdirplus_directory_scan`, `readdirplus_attr_generation_scan`, `readdirplus_candidate_selection`, and `readdirplus_page_commit`. If symlink visibility logic changed, the regular-file directory fixture is insufficient unless separate symlink-aware evidence is added.

Read-only close claims must report `fuse_op.flush`, `file_sync.flush`, `fuse_op.release`, `file_sync.fsync`, and read-only/write p50/p95/p99. `FOPEN_NOFLUSH` success is a mounted fact, not an assumption: if `fuse_op.flush` does not drop for read-only handles, document that and rely only on a separately verified read-only `flush()` no-op guard. Do not claim `release(flush=true)` improvement from a run where `file_sync.release_flush` is zero.

## Current harness coverage limits

The current harness is useful for mounted-vs-native comparisons on the listed workloads, but it does not directly isolate all planned performance surfaces.

It does not directly measure:

- matcher bucket/index cost or policy evaluator hot paths
- which specific matcher instance or rule dominated; current matcher family/order counters are aggregate shape signals across matcher invocations
- per-workload or per-request `open_confined_openat2` attribution beyond the whole-run perf summary
- path-by-path mutation invalidation breadth beyond the aggregate invalidation/eviction counters
- internal helper/offload attribution for the small-buffer or sync-surface path without separate counters/traces
- exactly how much time inside `resolved_virtual_path_from_path` sat in overlapping helper sub-steps beyond the current component-walk/canonicalize/confinement/virtual-conversion counters
- exactly which helper inside each current `readdir`/`readdirplus` bucket dominated without additional counters/traces

The `small_read` and `small_write` comparable workloads probe the small-buffer data path end to end. The `write_fsync_close` comparable workload probes the sync surface end to end through repeated open/write/file-fsync/close cycles plus cleanup, but it does not fsync parent directories. These workloads can show mounted-vs-native deltas on those surfaces, but they do not by themselves prove that time moved in a specific internal helper, queueing layer, or offload path, including internal offload helpers; use counters, traces, or focused artifacts when you need attribution.

The `symlink_parent_mkdir_rmdir` ScreenFS-only workload is an intended mounted workload/probe for symlink-parent mutation guard/path-resolution paths around the current `src/fs/guards.rs` request-local reuse area. Each mkdir/rmdir under a visible symlink parent alias is meant to traverse the mounted mutation path and can support before/after investigation of guard/path-resolution behavior. But without counters or traces, it does not prove that a live FUSE request hit a specific internal helper such as `RequestPathResolver`, `guarded_child_mutation`, or `guard_opened_directory_at_path`, and it cannot by itself attribute time to `source_root_path`, `resolved_virtual_path_from_path`, or `resolved_virtual_path_from_open_fd`. It also is not a speedup claim by itself; request-local reuse or canonical source-root reuse changes still need before/after comparisons, tail-latency checks, and preferably raw-sample review on this workload before claiming lower guard/path-resolution overhead.

For claims on the remaining uncovered surfaces, add dedicated perf counters, traces, microbenchmarks, or focused benchmark artifacts alongside the main harness output.

## Guardrails

- Do not use benchmark results to relax hidden `ENOENT`, bridge-visible, symlink target, or mutability precedence semantics.
- Do not treat warm-cache benchmark evidence as cold-cache or storage-device evidence.
- Do not run privileged mount or namespace steps from this harness; ScreenFS benchmark evidence remains non-root FUSE evidence, and the harness exits when run as root.
- Do not check in large generated result files unless they are explicitly accepted as current baseline artifacts.

## Passthrough baseline helper

The managed passthrough attribution baseline lives in [`../contrib/fractal-passthrough`](../contrib/fractal-passthrough). Build it with:

```bash
cargo build --release --manifest-path contrib/fractal-passthrough/Cargo.toml
```

Use it only as a minimal `fractal-fuse = 0.4.0` FUSE floor for native vs passthrough vs ScreenFS attribution; it does not implement ScreenFS visibility/mutability policy or source-root confinement. Use a trusted private scratch source tree without external symlinks, and keep running it as a non-root helper.

For any refreshed native vs passthrough vs ScreenFS attribution artifact, rebuild the managed contrib helper and record its path/SHA in [`artifacts/managed-fio-attribution-env.json`](artifacts/managed-fio-attribution-env.json) and the Markdown summary. The checked-in managed rerun now captures the contrib helper provenance there; if older `current-fio-attribution-*` files are still present locally, treat them as historical provenance rather than the active managed-helper reference.



Request-local guard context acceptance extends the metadata/open-path matrix with explicit evidence for `metadata_readlink` and source-root/open-fd resolver counters. `opendir` reuse is accepted through directory-surface workloads and perf counters unless a dedicated `metadata_opendir` workload is added; do not claim an `opendir` latency speedup from unrelated `readdir` scan improvements. Matcher hot-path claims require glob/matcher-heavy rows that show candidate-order or policy-decision cost moving in the expected direction while preserving the debug `candidate_order` API.
