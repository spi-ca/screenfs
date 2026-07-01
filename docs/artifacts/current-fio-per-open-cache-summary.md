# ScreenFS per-open cache fio smoke

This auxiliary warm-cache tmpfs fio smoke runs ScreenFS with default `visible`/`writable` policy so the conservative per-open read/write guard cache is eligible. It is not claim-grade speedup evidence.

## Files

- `docs/artifacts/current-fio-attribution.job`
- `docs/artifacts/current-fio-per-open-cache-native.json`
- `docs/artifacts/current-fio-per-open-cache-screenfs.json`
- `docs/artifacts/current-fio-per-open-cache-env.json`
- `docs/artifacts/current-fio-per-open-cache-perf-split.json`
- `docs/artifacts/current-fio-per-open-cache-screenfs.stderr.log`
- `docs/artifacts/current-fio-per-open-cache-screenfs-fstype.txt`
- `docs/artifacts/current-fio-per-open-cache-summary.svg`
- `docs/artifacts/current-fio-per-open-cache-summary.png`

## Provenance

- timestamp_utc: `2026-07-01T05:18:42Z`
- kernel: `7.2.0-rc1-1-spica-git`
- backing_fs_type: `tmpfs`
- fio_version: `fio-3.42-64-g19a6`
- fusermount3_version: `fusermount3 version: 3.18.2`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `9a7935e61d81818a603cdf2db732726ac83cba83aeb7098f92dd9cbdd19a7378`
- git_head: `bdc2ceafb1c61bbecab37ab28abf2df2be264ca7`
- git_worktree_clean: `False`
- cache_assumption: `warm-cache/buffered local run; no cache dropping; fio direct=0 ioengine=sync`
- screenfs_command: `/home/spi-ca/Codebase/screenfs/target/release/screenfs /tmp/screenfs-fio-peropen.nsfpsdui/source /tmp/screenfs-fio-peropen.nsfpsdui/mount --visibility-default visible --mutability-default writable`
- dirty worktree note: this smoke was captured from a dirty worktree during active benchmark refresh; inspect `docs/artifacts/current-fio-per-open-cache-env.json` for the authoritative full `git_status_short` listing before treating the binary as reproducible baseline evidence.

## fio mean completion latency

| job | native mean µs | ScreenFS mean µs | ScreenFS/native | native MiB/s | ScreenFS MiB/s |
| --- | ---: | ---: | ---: | ---: | ---: |
| `seq_write_128k` | 17.559 | 113.220 | 6.45x | 4000.0 | 941.2 |
| `seq_read_128k` | 26.549 | 57.264 | 2.16x | 4000.0 | 2000.0 |
| `rand_write_4k` | 0.405 | 21.689 | 53.54x | 4397.6 | 171.0 |
| `rand_read_4k` | 0.676 | 12.384 | 18.32x | 4280.0 | 300.9 |
| `sync_write_4k` | 1.497 | 16.819 | 11.24x | 1333.3 | 210.5 |

## Perf-counter evidence

| counter | count | avg ns | total ms |
| --- | ---: | ---: | ---: |
| `fuse_op.read` | 231258 | 962 | 222.623 |
| `fuse_op.write` | 132516 | 1969 | 261.009 |
| `policy_decision` | 496534 | 314 | 156.130 |
| `read_handle_snapshot` | 231258 | 187 | 43.289 |
| `read_guard_path` | 231258 | 13 | 3.137 |
| `read_io` | 231258 | 583 | 134.910 |
| `write_handle_snapshot` | 132516 | 201 | 26.738 |
| `write_guard_mutation` | 132516 | 17 | 2.367 |
| `write_io` | 132516 | 1583 | 209.859 |

## Interpretation

- Under the cache-eligible policy, `read_guard_path` and `write_guard_mutation` record only the cheap handle-cache decision segment instead of re-running full policy/path guards.
- Whole-run `policy_decision` still includes non-data-path FUSE work from the fio run, so this smoke uses the split guard counters—not aggregate policy count—to confirm cached read/write behavior.
- This smoke validates that the implementation exercises the intended per-open cache path; correctness remains covered by focused regression tests and full cargo validation, not by this benchmark alone.
