# Mounted writeback-cache smoke benchmark — 2026-07-03
This artifact records a same-machine mounted smoke/benchmark run for the opt-in `--experimental-writeback-cache` path. It is useful preflight evidence, but it is **not** claim-grade speedup evidence for writeback-cache support because it is a single local run on an uncommitted/dirty worktree and the feature remains experimental-only.
## Environment and binary
- timestamp: `2026-07-03T01:06:41.077656+00:00`
- screenfs binary: `/tmp/screenfs-release-target.gn4OdT/release/screenfs`
- binary sha256: `4dfd6aed0c02f8fb95914205df84225a95795fafd19c3499ba034d7137dd5417`
- kernel: `Linux ari14-lin 7.2.0-rc1-1-spica-git #1 SMP PREEMPT Thu, 02 Jul 2026 12:02:48 +0000 x86_64`
- source filesystem: `tmpfs`
- mounted filesystem: `fuse screenfs`
- fusermount3: `fusermount3 version: 3.18.2`
- rustc: `rustc 1.96.0 (ac68faa20 2026-05-25)`
- cargo: `cargo 1.96.0 (30a34c682 2026-05-25)`
- cache control: `warm`
- iterations/warmups: `10` / `3`
- policy preset: `fast-path-cache-eligible` (`fast-path-cache-eligible`)

## FUSE INIT / payload provenance

- The checked-in ScreenFS code used for this run has no repo-local `init()` override for `ReplyInit`; `rg "ReplyInit|max_write|max_readahead" src -S` found no ScreenFS override.
- The pinned dependency default remains `fractal-fuse = 0.4.0` `ReplyInit::default()` from `fractal-fuse-0.4.0/src/types.rs`: `max_write = 1024 * 1024` and `max_readahead = 1024 * 1024`.
- `fractal-fuse-0.4.0/src/session.rs` caps requested `max_write` at its transport ceiling, replies with `max_readahead = kernel_max_readahead.min(reply.max_readahead)`, and derives `max_pages = (max_write / 4096).max(1)`.
- Local `getconf PAGESIZE` returned `4096`, so the source-inspected default `max_pages` derivation for the unchanged default `max_write` is `256` pages. No large-I/O payload tuning flag or ScreenFS-specific negotiated payload override was used in this run.
- Startup stderr captured by the harness did not include the dependency `FUSE_INIT` trace lines, so this row is kept at preflight/smoke grade rather than claim-grade payload-negotiation evidence.

## Commands
- `baseline` `read-write-surface` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-be0gwo25/source /tmp/screenfs-bench-be0gwo25/mount --visibility-default visible --mutability-default writable`
- `writeback` `read-write-surface` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-0akexbep/source /tmp/screenfs-bench-0akexbep/mount --visibility-default visible --mutability-default writable --experimental-writeback-cache`
- `baseline` `sync-surface` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-_2d65edd/source /tmp/screenfs-bench-_2d65edd/mount --visibility-default visible --mutability-default writable`
- `writeback` `sync-surface` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-vgha_xwk/source /tmp/screenfs-bench-vgha_xwk/mount --visibility-default visible --mutability-default writable --experimental-writeback-cache`
- `baseline` `per-open-cache-minimum` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-gsiojfgp/source /tmp/screenfs-bench-gsiojfgp/mount --visibility-default visible --mutability-default writable`
- `writeback` `per-open-cache-minimum` ScreenFS command: `/tmp/screenfs-release-target.gn4OdT/release/screenfs /tmp/screenfs-bench-1a_8fw5n/source /tmp/screenfs-bench-1a_8fw5n/mount --visibility-default visible --mutability-default writable --experimental-writeback-cache`

## Mounted median comparison

Positive change means the writeback-cache opt-in row was faster than the same-machine baseline mounted row.

### `read-write-surface`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `rand_read_4k` | `0.214916s` | `0.213649s` | `0.994x` | `+0.6%` | `10.43x` | `10.57x` |
| `rand_write_4k` | `0.233748s` | `0.231301s` | `0.990x` | `+1.0%` | `9.60x` | `9.44x` |
| `seq_read` | `0.029650s` | `0.029312s` | `0.989x` | `+1.1%` | `4.64x` | `5.25x` |
| `seq_write` | `0.034768s` | `0.090967s` | `2.616x` | `-161.6%` | `5.40x` | `14.44x` |
| `small_read` | `0.009007s` | `0.009054s` | `1.005x` | `-0.5%` | `12.39x` | `11.49x` |
| `small_write` | `0.020431s` | `0.020964s` | `1.026x` | `-2.6%` | `12.12x` | `15.90x` |

### `sync-surface`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `sync_flush_only` | `0.011272s` | `0.014126s` | `1.253x` | `-25.3%` | `14.93x` | `22.82x` |
| `sync_fsync_only` | `0.003664s` | `0.006753s` | `1.843x` | `-84.3%` | `12.33x` | `33.32x` |
| `sync_release_flush` | `0.009666s` | `0.012647s` | `1.308x` | `-30.8%` | `16.63x` | `30.46x` |

### `per-open-cache-minimum`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `rand_read_4k` | `0.212059s` | `0.148712s` | `0.701x` | `+29.9%` | `10.13x` | `10.52x` |
| `rand_write_4k` | `0.333162s` | `0.232521s` | `0.698x` | `+30.2%` | `13.90x` | `13.32x` |
| `small_open_read_close` | `0.177388s` | `0.191686s` | `1.081x` | `-8.1%` | `4.65x` | `8.03x` |
| `sync_write_4k` | `0.002133s` | `0.002058s` | `0.965x` | `+3.5%` | `12.66x` | `13.52x` |

## Interpretation

- The `read-write-surface` comparable row is mostly flat/noisy for reads and small writes (`-2.6%` to `+1.1%`) except `seq_write`, which regressed in this run (`2.616x` writeback/base, `-161.6%`).
- `sync-surface` regressed across all three sync rows (`-25.3%` to `-84.3%`).
- `per-open-cache-minimum` showed faster random read/write rows (`+29.9%` / `+30.2%`) but slower `small_open_read_close` (`-8.1%`).
- Because results are mixed and the feature is experimental, this artifact supports only the current preflight conclusion: `--experimental-writeback-cache` mounted plumbing works in this environment, but there is no broad performance-improvement claim.

## Source files

- `baseline-per-open-cache-minimum.json`
- `baseline-read-write-surface.json`
- `baseline-sync-surface.json`
- `writeback-per-open-cache-minimum.json`
- `writeback-read-write-surface.json`
- `writeback-sync-surface.json`
- `baseline-per-open-cache-minimum.md`
- `baseline-read-write-surface.md`
- `baseline-sync-surface.md`
- `writeback-per-open-cache-minimum.md`
- `writeback-read-write-surface.md`
- `writeback-sync-surface.md`
