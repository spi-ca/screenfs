# Mounted writeback-cache rerun — 2026-07-03

This rerun repeats the same-machine baseline vs `--experimental-writeback-cache` mounted benchmark after the first mounted preflight run. It remains preflight evidence, not a broad writeback-cache performance claim.

## Environment and binary

- timestamp: `2026-07-03T01:54:32.626774+00:00`
- screenfs binary: `/tmp/screenfs-release-target.gn4OdT/release/screenfs`
- binary sha256: `4dfd6aed0c02f8fb95914205df84225a95795fafd19c3499ba034d7137dd5417`
- kernel: `Linux ari14-lin 7.2.0-rc1-1-spica-git #1 SMP PREEMPT Thu, 02 Jul 2026 12:02:48 +0000 x86_64 `
- source filesystem: `tmpfs tmpfs rw,nosuid,nodev,relatime,lazytime,inode64,huge=within_size`
- mounted filesystem: `fuse screenfs rw,nosuid,nodev,relatime,user_id=1000,group_id=1000`
- fusermount3: `fusermount3 version: 3.18.2`
- rustc: `rustc 1.96.0 (ac68faa20 2026-05-25)`
- cargo: `cargo 1.96.0 (30a34c682 2026-05-25)`
- cache control: `warm`
- iterations/warmups: `10` / `3`
- policy preset: `fast-path-cache-eligible` (`fast-path-cache-eligible`)

## Mounted median comparison

Positive change means the writeback-cache opt-in row was faster than the same-machine baseline mounted row.

### `read-write-surface`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `rand_read_4k` | `0.158986s` | `0.168399s` | `1.059x` | `-5.9%` | `10.86x` | `8.24x` |
| `rand_write_4k` | `0.275788s` | `0.302865s` | `1.098x` | `-9.8%` | `15.34x` | `12.63x` |
| `seq_read` | `0.023808s` | `0.026781s` | `1.125x` | `-12.5%` | `2.83x` | `4.59x` |
| `seq_write` | `0.019170s` | `0.074411s` | `3.882x` | `-288.2%` | `4.91x` | `11.59x` |
| `small_read` | `0.007085s` | `0.009008s` | `1.271x` | `-27.1%` | `12.97x` | `11.64x` |
| `small_write` | `0.015847s` | `0.020892s` | `1.318x` | `-31.8%` | `16.44x` | `13.66x` |

### `sync-surface`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `sync_flush_only` | `0.011276s` | `0.013857s` | `1.229x` | `-22.9%` | `17.37x` | `24.56x` |
| `sync_fsync_only` | `0.003835s` | `0.006840s` | `1.784x` | `-78.4%` | `16.37x` | `36.74x` |
| `sync_release_flush` | `0.009825s` | `0.012684s` | `1.291x` | `-29.1%` | `20.78x` | `32.54x` |

### `per-open-cache-minimum`

| workload | baseline mounted median | writeback mounted median | writeback/base | change | baseline mounted/native | writeback mounted/native |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `rand_read_4k` | `0.213425s` | `0.143254s` | `0.671x` | `+32.9%` | `9.97x` | `10.53x` |
| `rand_write_4k` | `0.337035s` | `0.224522s` | `0.666x` | `+33.4%` | `14.22x` | `13.08x` |
| `small_open_read_close` | `0.249140s` | `0.193470s` | `0.777x` | `+22.3%` | `6.44x` | `8.28x` |
| `sync_write_4k` | `0.002838s` | `0.001835s` | `0.647x` | `+35.3%` | `16.75x` | `12.38x` |

## Interpretation

- This rerun strengthens the first run's negative signal for `seq_write` and `sync-surface`: `seq_write` regressed again (`3.882x` writeback/base), and all sync rows regressed (`1.229x` to `1.784x` writeback/base).
- `per-open-cache-minimum` improved in this rerun across all four rows, including random read/write (`+32.9%` / `+33.4%`).
- `read-write-surface` as a set regressed in this rerun (`-5.9%` to `-31.8%` except the large `seq_write` regression), so the feature still has no broad speedup claim.

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
