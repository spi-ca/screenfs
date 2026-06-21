# Open-confined fd-path micro-candidate

This directory records a post-`70a3105` `open_confined` / `openat2` follow-up attempt. The candidate changed only `/proc/self/fd/<fd>` path construction inside `RequestPathResolver::resolved_virtual_path_for_open_file()` to avoid the previous `format!("/proc/self/fd/{}", fd)` full-string allocation.

## Result

Rejected / not kept. The implementation preserved the existing `read_link`-based opened-fd target revalidation flow in local tests and review, but same-machine benchmark evidence failed the `open-confined-surface` claim gate and showed regressions. The Rust implementation was reverted; this artifact remains as rejected evidence only.

## Target lane

- active lane: `open_confined` / `openat2` frequency (`smoke` -> candidate under test)
- target claim-gate rows:
  - `--policy-preset fallback-unsafe-policy --workload-set open-confined-surface`
  - `--policy-preset fast-path-cache-eligible --workload-set open-confined-surface`
- attribution/non-regression rows:
  - `--policy-preset fallback-unsafe-policy --workload metadata_opendir`
  - `--policy-preset fallback-unsafe-policy --workload metadata_access` because the same opened-fd helper is also used by the `access` post-open revalidation path

## Gate/counter watchlist

For the two `open-confined-surface` rows, the normal gate requires both `metadata_open` and `metadata_opendir` at after/before `<= 0.90x` median and `<= 0.95x` at p95/p99. Focused `metadata_opendir` / `metadata_access` rows are attribution and non-regression context only; they cannot rescue a failed surface gate.

Counters watched:

- `open_confined_openat2`
- `open_like.pre_open_guard.open`, `open_like.pre_open_guard.opendir`, `open_like.pre_open_guard.access`
- `open_like.post_open_revalidation.open`, `open_like.post_open_revalidation.opendir`, `open_like.post_open_revalidation.access`
- `source_root_path`
- `resolved_virtual_path_from_open_fd`
- `fuse_op.open`, `fuse_op.opendir`, `fuse_op.access`

## Benchmark evidence

The benchmark bundle was collected with a clean detached `HEAD` worktree for before rows and the dirty candidate worktree for after rows. All benchmark rows used `--perf-counters --iterations 10 --warmups 3`.

JSON source-of-truth files:

- `before-clean-head-70a3105-fallback-open-confined-surface.json`
- `after-dirty-worktree-70a3105-fallback-open-confined-surface.json`
- `before-clean-head-70a3105-fast-open-confined-surface.json`
- `after-dirty-worktree-70a3105-fast-open-confined-surface.json`
- `before-clean-head-70a3105-fallback-metadata_opendir.json`
- `after-dirty-worktree-70a3105-fallback-metadata_opendir.json`
- `before-clean-head-70a3105-fallback-metadata_access.json`
- `after-dirty-worktree-70a3105-fallback-metadata_access.json`

Surface after/before ratios:

| row | workload | p50 | p95 | p99 | gate interpretation |
| --- | --- | ---: | ---: | ---: | --- |
| fallback `open-confined-surface` | `metadata_open` | `1.630311x` | `1.648379x` | `1.608566x` | fails / regresses |
| fallback `open-confined-surface` | `metadata_opendir` | `1.419060x` | `1.427326x` | `1.427674x` | fails / regresses |
| fast `open-confined-surface` | `metadata_open` | `1.051272x` | `1.086041x` | `1.103472x` | fails; tail non-regression miss |
| fast `open-confined-surface` | `metadata_opendir` | `1.026205x` | `1.106744x` | `1.081264x` | fails; p95 non-regression miss |

Focused attribution/context ratios:

| row | workload | p50 | p95 | p99 | interpretation |
| --- | --- | ---: | ---: | ---: | --- |
| fallback focused | `metadata_opendir` | `1.523680x` | `1.226561x` | `1.115950x` | regresses; attribution row does not help |
| fallback focused | `metadata_access` | `1.033812x` | `1.014779x` | `1.005681x` | within non-regression, but context only |

Selected perf-counter movement also argued against keeping the change: in the fallback surface row, `resolved_virtual_path_from_open_fd.avg_ns` moved from `768ns` to `1082ns`, `open_like.post_open_revalidation.open.avg_ns` from `2549ns` to `3235ns`, `open_like.post_open_revalidation.opendir.avg_ns` from `2167ns` to `2853ns`, `fuse_op.open.avg_ns` from `15463ns` to `19639ns`, and `fuse_op.opendir.avg_ns` from `11829ns` to `16012ns`.

## Safety constraints checked before rejection

The attempted implementation preserved these intended constraints while it was under test:

- kept `read_link` on `/proc/self/fd/<fd>` as the opened-fd target source
- kept source-root confinement through the existing `virtual_path_from_source_path()` flow
- kept hidden-path precedence as `ENOENT`
- kept opened-fd target revalidation for `open` / `opendir` / `access`
- did not introduce cross-request caching
- kept perf-counter names and count shape comparable

## Validation and review

Validation log: [`validation.log`](validation.log)

Recorded commands all passed before the benchmark rejection:

- `cargo fmt --check`
- `cargo check --features perf-counters`
- `cargo test --all-targets --all-features path::tests::rejects_following_symlinks_outside_source_root_but_allows_link_itself -- --exact`
- `cargo test --all-targets --all-features fs::tests::perf::perf_counters_split_resolved_virtual_path_sources -- --exact`
- `cargo test --all-targets --all-features fs::tests::perf::perf_counters_record_open_like_guard_and_revalidation_splits_on_success -- --exact`
- `cargo test --all-targets --all-features fs::tests::symlinks_access_create::symlink_directory_escape_rejects_opendir_access_and_create_before_side_effects -- --exact`
- `cargo test --all-targets --all-features fs::tests::symlinks_access_create::hidden_access_returns_enoent_for_read_and_write_masks -- --exact`
- `cargo test --all-targets --all-features fs::tests::symlinks_access_create::opened_parent_directory_at_path_rejects_host_rename_replacement -- --exact`

Subagent review after adding `metadata_access` context and validation evidence reported no findings. Subagent security review found no guardrail blocker for the candidate shape. Because the benchmark gate failed, the `open_confined` / `openat2` lane remains `smoke` with this rejected micro-candidate recorded.
