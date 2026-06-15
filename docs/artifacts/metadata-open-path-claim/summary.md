# Metadata/open-path next-candidate measurement bundle

This bundle records current worktree measurements for the next ScreenFS performance candidates: metadata/open path, split sync surfaces, directory surfaces, and a policy-heavy matrix row. It is measurement plumbing and attribution evidence; it is not a before/after optimization claim.

## Files

- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.md`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.png`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.svg`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.md`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.png`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.svg`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.md`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.png`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.svg`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.md`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.png`
- `docs/artifacts/metadata-open-path-claim/current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.svg`
- `docs/artifacts/metadata-open-path-claim/summary.md`
- `docs/artifacts/metadata-open-path-claim/attr-reuse-worktree-19ca971/summary.md` and companion before/after JSON/Markdown/SVG files for the lookup/getattr same-path no-follow attr reuse slice. This bundle is slice-scoped current-worktree evidence, not a full metadata/open-path candidate claim.
- `docs/artifacts/metadata-open-path-claim/guard-context-worktree-19ca971/summary.md` and companion before/after JSON/Markdown/SVG files for the request-local guard context, readlink parent dirfd/attr reuse, and matcher streaming hot-path work. Latency artifacts use non-`perf-counters` release binaries; attribution artifacts use `perf-counters` release binaries.

## Workload medians

| artifact | policy label | workload | native p50 s | mounted p50 s | mounted/native | mounted p95 s | mounted p99 s |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| `current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json` | `fallback-unsafe-policy-matcher32` | `metadata_lookup` | 0.007180 | 0.088711 | 12.356x | 0.089729 | 0.090036 |
| `current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json` | `fallback-unsafe-policy-matcher32` | `metadata_getattr` | 0.007701 | 0.115882 | 15.048x | 0.116572 | 0.116807 |
| `current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json` | `fallback-unsafe-policy-matcher32` | `metadata_access` | 0.006728 | 0.120268 | 17.877x | 0.126365 | 0.127631 |
| `current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json` | `fallback-unsafe-policy-matcher32` | `matcher_hidden_stat_miss` | n/a | 0.036375 | n/a | 0.039873 | 0.040378 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json` | `fast-path-cache-eligible` | `readdir_basic` | 0.000470 | 0.022245 | 47.342x | 0.023619 | 0.023787 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json` | `fast-path-cache-eligible` | `readdirplus_basic` | 0.002921 | 0.174408 | 59.704x | 0.179682 | 0.180309 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_lookup` | 0.007171 | 0.064550 | 9.002x | 0.065487 | 0.065516 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_getattr` | 0.008821 | 0.083709 | 9.490x | 0.087566 | 0.087973 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_open` | 0.007297 | 0.092995 | 12.745x | 0.094704 | 0.095015 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_readlink` | 0.000671 | 0.076775 | 114.416x | 0.078671 | 0.078873 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_access` | 0.006774 | 0.084126 | 12.419x | 0.087765 | 0.088667 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | `fast-path-cache-eligible` | `metadata_statfs` | 0.000803 | 0.005425 | 6.758x | 0.005492 | 0.005500 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json` | `fast-path-cache-eligible` | `sync_flush_only` | 0.000384 | 0.008478 | 22.080x | 0.008772 | 0.008803 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json` | `fast-path-cache-eligible` | `sync_fsync_only` | 0.000160 | 0.002052 | 12.843x | 0.002700 | 0.002994 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json` | `fast-path-cache-eligible` | `sync_release_flush` | 0.000287 | 0.006665 | 23.263x | 0.008181 | 0.008318 |

## Perf-counter acceptance checks

| artifact | selected counters |
| --- | --- |
| `current-worktree-03bfdeb82ced-fallback-unsafe-policy-matcher32-policy-heavy-matrix.json` | fuse_op.lookup: count=139781 avg_ns=19309; fuse_op.getattr: count=13313 avg_ns=23369; fuse_op.access: count=13313 avg_ns=24599; fuse_op.statfs: count=2 avg_ns=3439; matcher_candidates: count=6656; matcher_family_candidates.subtree: count=6656; matcher_family_candidates.direct_child_glob: count=0; matcher_family_candidates.recursive: count=0; matcher_candidate_order.path: count=1477692 avg_ns=232; matcher_candidate_order.descendant: count=492564 avg_ns=110 (empty visible matcher, not rule-rich descendant evidence); matcher_candidate_order_duplicates: count=0; readdir_directory_scan: count=0 avg_ns=0; readdirplus_directory_scan: count=0 avg_ns=0; readdirplus_attr_generation_entries: count=0 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-directory-surface.json` | fuse_op.lookup: count=79929 avg_ns=13256; fuse_op.getattr: count=26651 avg_ns=13799; fuse_op.access: count=1 avg_ns=73119; fuse_op.statfs: count=2 avg_ns=4826; fuse_op.readdir: count=103 avg_ns=3667577; fuse_op.readdirplus: count=28 avg_ns=6639839; readdir_directory_scan: count=103 avg_ns=3005033; readdirplus_directory_scan: count=28 avg_ns=6385090; readdirplus_attr_generation_entries: count=54306 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-metadata-open-path.json` | fuse_op.lookup: count=199685 avg_ns=13802; fuse_op.getattr: count=13313 avg_ns=13431; fuse_op.open: count=13312 avg_ns=14476; fuse_op.readlink: count=13312 avg_ns=15515; fuse_op.access: count=13313 avg_ns=13818; fuse_op.statfs: count=13314 avg_ns=256; fuse_op.flush: count=13312 avg_ns=5141; fuse_op.release: count=13312 avg_ns=1026; file_sync.flush: count=13312 avg_ns=4669; readdir_directory_scan: count=0 avg_ns=0; readdirplus_directory_scan: count=0 avg_ns=0; readdirplus_attr_generation_entries: count=0 |
| `current-worktree-03bfdeb82ced-fast-path-cache-eligible-sync-surface.json` | fuse_op.lookup: count=5153 avg_ns=10100; fuse_op.getattr: count=833 avg_ns=10612; fuse_op.open: count=1638 avg_ns=16158; fuse_op.access: count=1 avg_ns=84457; fuse_op.statfs: count=2 avg_ns=1322; fuse_op.flush: count=1677 avg_ns=3431; fuse_op.fsync: count=832 avg_ns=2795; fuse_op.release: count=1677 avg_ns=544; file_sync.flush: count=1677 avg_ns=3131; file_sync.fsync: count=832 avg_ns=2517; readdir_directory_scan: count=0 avg_ns=0; readdirplus_directory_scan: count=0 avg_ns=0; readdirplus_attr_generation_entries: count=0 |

## Provenance

Each JSON is `screenfs-benchmark-v2` and records `policy`, `workloads`, ScreenFS source provenance, command line, binary SHA256, dirty worktree state, and perf-counter stderr summary.

## Measurement caveats

- `metadata_open` timing uses the harness cleanup hook so descriptor close/release work is executed after the timed sample. Whole-run ScreenFS perf counters still include cleanup `fuse_op.flush` / `file_sync.flush` / `fuse_op.release` events; use the workload timing for open-path latency and the perf counters only as whole-run context.
- The policy-heavy matcher32 row covers `metadata_lookup`, `metadata_getattr`, `metadata_access`, and `matcher_hidden_stat_miss`; it does not cover `metadata_open`, `metadata_readlink`, `readdir_basic`, or `readdirplus_basic` for the unsafe-policy matrix.
- The matcher32 counters show subtree-family candidate volume and path candidate-order traversal work. The `matcher_candidate_order.descendant` count in this row comes from an empty visible matcher (`visible-rules=0`), so it is not rule-rich descendant candidate-order evidence. Duplicate-removal cost is not demonstrated in this artifact because `matcher_candidate_order_duplicates` is zero.
- `sync_release_flush` is a mounted close/release proxy. In the checked-in FUSE run the kernel did not exercise ScreenFS `release(flush=true)`, so `file_sync.release_flush` remains absent from the sync-surface artifact. Treat release-flush attribution as covered by the focused Rust perf-counter test until a mounted driver can force that FUSE flag.
