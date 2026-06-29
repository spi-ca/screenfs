---
name: "run-screenfs-benchmarks"
description: "Run and record the ScreenFS formal performance benchmark harness"
version: 1
created: "2026-06-14"
updated: "2026-06-14"
---
## When to Use
Use when measuring ScreenFS performance, comparing native backing filesystem vs ScreenFS mount, or producing benchmark evidence for performance changes in this repo.

## Procedure
1. Build release binary first with cargo build --release, or pass --build to the harness when build time should stay outside measured workloads.
2. Run scripts/bench-screenfs.py as a regular non-root user; never use sudo. Use fusermount3-capable environment with FUSE_OVER_IO_URING support.
3. For smoke-checking the harness, use small sizes such as --iterations 1 --warmups 1 --read-mib 1 --write-mib 1 --small-files 10 --dir-entries 10 --hidden-misses 10 and write outputs under /tmp.
4. For formal evidence, use docs/benchmarks.md defaults or larger sizes and write --output-json plus --output-md to an agreed artifact path.
5. Preserve the JSON as source of truth; use Markdown only as a human summary. Record command line, workload sizes, environment, git status, and cache assumptions.
6. For `DirectoryChildVisibilityBatch`, `readdirplus_scan.scan_visibility`, matcher-heavy, or visible-descendant changes, follow `docs/benchmarks.md` gates before claiming speed: use the series wrapper/repeated or interleaved before-after runs when required, include same-binary controls when the lane is noise-sensitive, and cover `--workload-set matcher-descendant-directory` for rule-sensitive descendant changes.
7. For `open_confined` / open-like / `metadata_opendir` changes, collect both `fallback-unsafe-policy` and `fast-path-cache-eligible` `--workload-set open-confined-surface` before/after rows when making a speed claim. For opendir-specific work or regressions, also run an explicit `--workload metadata_opendir` row with `--perf-counters` for attribution.
8. Treat counter-only reductions as slice/smoke evidence unless mounted latency clears the documented gate. Do not promote parent-local or matcher-heavy evidence to a broad `readdirplus` claim, and do not promote open-like counter movement to an `open_confined` or `metadata_opendir` speed claim, without the matching matrix.
9. After code changes, run cargo test --all-targets --all-features and compare before/after median ratios and raw-sample variance for affected workloads.

## Pitfalls
- Do not treat warm-cache local benchmark results as cold-cache or storage-device evidence unless cache-control steps are explicitly recorded.
- Do not use benchmark results to justify relaxing hidden ENOENT, bridge-visible, symlink target, or mutability precedence semantics.
- Do not check in large generated result files unless explicitly accepted as current baseline artifacts.
- Do not reuse the fast-policy/cache-eligible `readdirplus` claim gate for broader rule-sensitive, matcher-heavy, or descendant visibility changes.
- Do not claim an opendir/open-confined speedup from readdir, matcher, or counter-only movement; use the documented open-confined and explicit `metadata_opendir` rows.
- If unmount fails, the harness preserves the workdir; inspect and clean it manually only after confirming the mount is gone.

## Verification
1. scripts/bench-screenfs.py --help works and python3 -m py_compile scripts/bench-screenfs.py passes.
2. A small smoke run produces non-empty JSON and Markdown outputs and reports mount_filesystem stat_f_type=fuse.
3. cargo test --all-targets --all-features passes after any implementation changes.