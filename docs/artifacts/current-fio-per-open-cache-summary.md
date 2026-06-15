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
- `docs/artifacts/current-fio-per-open-cache-summary.png` (2x raster render)

## Provenance

- timestamp_utc: `2026-06-15T02:34:01Z`
- kernel: `7.1.0-rc7-1-spica-git`
- backing_fs_type: `tmpfs`
- fio_version: `fio-3.42-64-g19a6`
- fusermount3_version: `fusermount3 version: 3.18.2`
- screenfs_bin: `/home/spi-ca/Codebase/screenfs/target/release/screenfs`
- screenfs_bin_sha256: `6cbc30137108ace1e7df09c285534465afa74d61b7b2a9bc88e54cf63198aff6`
- git_head: `03bfdeb82ced3041e5876436794d7d5adeda42c8`
- git_worktree_clean: `False`
- dirty worktree note: this smoke was captured from a dirty worktree during active per-open-cache development; inspect `docs/artifacts/current-fio-per-open-cache-env.json` for the authoritative full `git_status_short` listing before treating the binary as reproducible baseline evidence.
- cache_assumption: warm-cache/buffered local run; no cache dropping; fio direct=0 ioengine=sync
- screenfs_command: `/home/spi-ca/Codebase/screenfs/target/release/screenfs /tmp/screenfs-fio-peropen.LJsM6Z/source /tmp/screenfs-fio-peropen.LJsM6Z/mount --visibility-default visible --mutability-default writable`
- provenance note: this checked-in `current-fio-per-open-cache-*` set is one cache-eligible fast-path smoke snapshot, not a before/after comparison; claim-grade artifacts should use separate before/after names with revision, policy-preset, and workload-set provenance.
- Treat the checked-in `current-fio-per-open-cache-*` files in this directory as one warm-cache smoke evidence set, not as claim-grade benchmark proof.

## fio mean completion latency

| job | native mean µs | ScreenFS mean µs | ScreenFS/native | native MiB/s | ScreenFS MiB/s |
| --- | ---: | ---: | ---: | ---: | ---: |
| `seq_write_128k` | 38.132 | 64.954 | 1.70x | 1777.8 | 1600.0 |
| `seq_read_128k` | 32.357 | 110.944 | 3.43x | 3200.0 | 1066.7 |
| `rand_write_4k` | 0.427 | 24.611 | 57.61x | 4367.9 | 151.3 |
| `rand_read_4k` | 0.648 | 24.970 | 38.53x | 4598.9 | 151.5 |
| `sync_write_4k` | 0.898 | 25.450 | 28.36x | 2000.0 | 142.9 |

## Perf-counter evidence

| counter | count | avg ns | total ms |
| --- | ---: | ---: | ---: |
| `fuse_op.read` | 116388 | 1912 | 222.574 |
| `fuse_op.write` | 117403 | 1948 | 228.818 |
| `policy_decision` | 701962 | 273 | 192.122 |
| `read_handle_snapshot` | 116388 | 222 | 25.863 |
| `read_guard_path` | 116388 | 18 | 2.138 |
| `read_io` | 116388 | 1513 | 176.111 |
| `write_handle_snapshot` | 117403 | 209 | 24.541 |
| `write_guard_mutation` | 117403 | 18 | 2.123 |
| `write_io` | 117403 | 1551 | 182.200 |

## Interpretation

- Under the cache-eligible policy, `read_guard_path` and `write_guard_mutation` record only the cheap handle-cache decision segment instead of re-running full policy/path guards.
- Whole-run `policy_decision` still includes non-data-path FUSE work from the fio run, so this smoke uses the split guard counters—not aggregate policy count—to confirm cached read/write behavior.
- This smoke validates that the implementation exercises the intended per-open cache path; correctness remains covered by focused regression tests and full cargo validation, not by this benchmark alone.
- Full-suite validation observed for this change: `cargo test --all-targets --all-features` passed with 169 library tests and 1 main test.

## Focused correctness and regression coverage

- [`../../src/fs/tests/perf.rs`](../../src/fs/tests/perf.rs): `perf_counters_record_data_path_splits_on_success`, `perf_counters_record_data_path_splits_recheck_policy_when_cache_not_safe`, `perf_counters_record_data_path_splits_on_snapshot_guard_and_io_failures`, `perf_counters_keep_fallocate_and_copy_file_range_on_per_call_policy_path`
- [`../../src/fs/tests/data_mutations.rs`](../../src/fs/tests/data_mutations.rs): `cache_eligible_opened_file_read_write_keep_pinned_fd_after_host_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_ancestor_rename`, `cache_eligible_opened_file_read_write_keep_pinned_fd_after_unlink`, `opened_file_read_keeps_pinned_fd_after_host_rename_but_write_fails_closed`, `opened_file_read_write_fail_closed_after_host_rename_into_hidden_subtree`
- [`../../src/fs/tests/symlinks_access_create.rs`](../../src/fs/tests/symlinks_access_create.rs): `cache_eligible_opened_symlink_read_keeps_pinned_fd_after_final_retarget`, `cache_eligible_opened_symlink_read_keeps_pinned_fd_after_ancestor_retarget`, `opened_symlink_read_revalidates_hidden_target_after_retarget`
- [`../../src/fs/tests/mutability.rs`](../../src/fs/tests/mutability.rs): `opened_symlink_write_revalidates_readonly_target_after_retarget`
