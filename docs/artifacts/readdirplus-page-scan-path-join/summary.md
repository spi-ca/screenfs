# Readdirplus page/scan path-join experiment

This bundle records a rejected path-join experiment for `readdirplus` page/scan work. The code changes from the experiment were reverted and are **not** part of the current implementation. Treat the files in this directory as rejected-experiment evidence, not as speedup evidence.

## Command shape

5k directory-surface row:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.json --output-md docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/before-directory-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set directory-surface --iterations 10 --warmups 3 --output-json docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.json --output-md docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/after-directory-surface.svg
```

20k focused row:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.json --output-md docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/before-readdirplus-20k.svg
python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.json --output-md docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-page-scan-path-join/after-readdirplus-20k.svg
```

## Mounted latency ratios

### 5k directory-surface

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `readdir_basic` | 0.075869842 | 0.118200714 | 1.557941x | 0.076828568 | 0.121768885 | 1.584943x | 0.076998361 | 0.122790496 | 1.594716x |
| `readdirplus_basic` | 0.373800450 | 0.367979436 | 0.984427x | 0.381594285 | 0.383095192 | 1.003933x | 0.383678076 | 0.384689038 | 1.002635x |

### 20k focused `readdirplus_basic`

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `readdirplus_basic` | 2.272758733 | 2.355125795 | 1.036241x | 2.449739771 | 2.393600615 | 0.977084x | 2.457400398 | 2.396959332 | 0.975404x |

## Selected perf-counter totals

### 5k directory-surface regression counters

| counter | before | after | after/before |
| --- | ---: | ---: | ---: |
| `fuse_op.readdir.total_ns` | 1329339492 | 1677204661 | 1.261683x |
| `readdir_directory_scan.total_ns` | 1140828556 | 1441604026 | 1.263647x |
| `readdir_candidate_selection.total_ns` | 73614264 | 96890478 | 1.316192x |
| `readdir_page_commit.total_ns` | 157966876 | 204327309 | 1.293482x |
| `fuse_op.readdirplus.total_ns` | 433216664 | 531887310 | 1.227763x |
| `readdirplus_directory_scan.total_ns` | 348488862 | 434796375 | 1.247662x |
| `readdirplus_candidate_selection.total_ns` | 22063679 | 29110242 | 1.319374x |
| `readdirplus_page_commit.total_ns` | 6738508 | 7360319 | 1.092277x |

### 20k focused row counter improvements that still did not clear latency

| counter | before | after | after/before |
| --- | ---: | ---: | ---: |
| `fuse_op.readdirplus.total_ns` | 1167339461 | 954248167 | 0.817456x |
| `readdirplus_directory_scan.total_ns` | 1076329283 | 873491363 | 0.811547x |
| `readdirplus_attr_generation_scan.total_ns` | 55792580 | 51203451 | 0.917747x |
| `readdirplus_candidate_selection.total_ns` | 62968025 | 52057191 | 0.826724x |
| `readdirplus_page_commit.total_ns` | 19660325 | 16454664 | 0.836948x |

## Conclusion

- The experiment is rejected because the 5k `directory-surface` run regressed badly on `readdir_basic` (`1.557941x` p50, `1.584943x` p95, `1.594716x` p99).
- The 20k focused `readdirplus_basic` row still regressed at p50 (`1.036241x`) even though p95/p99 improved (`0.977084x` / `0.975404x`) and the `readdirplus_*` counters above moved down.
- The code changes were reverted, the implementation was not kept, and this artifact directory should only be cited as rejected evidence for the remaining `readdirplus` page/scan gap.
