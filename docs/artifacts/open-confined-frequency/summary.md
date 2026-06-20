# Open-confined frequency experiment

This bundle records a rejected `open_confined` / `openat2` frequency experiment. The implementation tested here was reverted and is **not** part of the current code. Treat the files in this directory as rejected/slice-context evidence, not as speedup evidence: the focused fallback `metadata_opendir` slice improved, but the claim-gate `open-confined-surface` rows did not pass.

## Command shape

Focused fallback `metadata_opendir` slice:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.json --output-md docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.md --output-svg docs/artifacts/open-confined-frequency/before-fallback-metadata-opendir.svg
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload metadata_opendir --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.json --output-md docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.md --output-svg docs/artifacts/open-confined-frequency/after-fallback-metadata-opendir.svg
```

Fallback `open-confined-surface` claim-gate row:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/before-fallback-open-confined-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fallback-unsafe-policy --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/after-fallback-open-confined-surface.svg
```

Fast `open-confined-surface` claim-gate row and rerun:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/before-fast-open-confined-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.json --output-md docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.md --output-svg docs/artifacts/open-confined-frequency/after-fast-open-confined-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --policy-preset fast-path-cache-eligible --workload-set open-confined-surface --iterations 10 --warmups 3 --output-json docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.json --output-md docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.md --output-svg docs/artifacts/open-confined-frequency/after-fast-open-confined-surface-rerun.svg
```

## Mounted latency ratios

### Focused fallback `metadata_opendir` slice

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `metadata_opendir` | 0.038813581 | 0.025558982 | 0.658506x | 0.040179177 | 0.027039851 | 0.672982x | 0.040425923 | 0.027283558 | 0.674903x |

### Fallback `open-confined-surface` claim-gate row (final after run)

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `metadata_open` | 0.034295693 | 0.046021497 | 1.341903x | 0.037940272 | 0.054359575 | 1.432767x | 0.038080981 | 0.055240686 | 1.450611x |
| `metadata_opendir` | 0.029568265 | 0.034090065 | 1.152927x | 0.031184014 | 0.034654019 | 1.111275x | 0.031631583 | 0.034700869 | 1.097032x |

### Fast `open-confined-surface` claim-gate row

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `metadata_open` | 0.030705031 | 0.029454260 | 0.959265x | 0.031972831 | 0.037896882 | 1.185284x | 0.032387033 | 0.042176621 | 1.302269x |
| `metadata_opendir` | 0.025421973 | 0.022218359 | 0.873982x | 0.027017682 | 0.023384097 | 0.865511x | 0.027617766 | 0.023454442 | 0.849252x |

### Fast `open-confined-surface` rerun

| workload | before p50 s | after p50 s | after/before p50 | before p95 s | after p95 s | after/before p95 | before p99 s | after p99 s | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `metadata_open` | 0.030705031 | 0.035286133 | 1.149197x | 0.031972831 | 0.036512422 | 1.141983x | 0.032387033 | 0.036849157 | 1.137775x |
| `metadata_opendir` | 0.025421973 | 0.028406195 | 1.117388x | 0.027017682 | 0.029297598 | 1.084386x | 0.027617766 | 0.029405740 | 1.064740x |

## Selected perf-counter deltas

### Focused fallback `metadata_opendir` slice

| counter | before | after | delta | after/before |
| --- | ---: | ---: | ---: | ---: |
| `open_confined_openat2.total_ns` | 21589058 | 9533021 | -12056037 | 0.441567x |
| `source_root_path.total_ns` | 50300139 | 30979325 | -19320814 | 0.615889x |
| `resolved_virtual_path_from_open_fd.total_ns` | 35503324 | 19476992 | -16026332 | 0.548596x |
| `fuse_op.opendir.total_ns` | 107911095 | 53814478 | -54096617 | 0.498693x |
| `stat_child_no_follow.total_ns` | 186528681 | 92706846 | -93821835 | 0.497011x |
| `policy_decision.total_ns` | 19543827 | 13203265 | -6340562 | 0.675572x |

### Fallback `open-confined-surface` claim-gate row (final after run)

| counter | before | after | delta | after/before |
| --- | ---: | ---: | ---: | ---: |
| `open_confined_openat2.total_ns` | 78694556 | 95550731 | 16856175 | 1.214197x |
| `source_root_path.total_ns` | 65994179 | 94203449 | 28209270 | 1.427451x |
| `resolved_virtual_path_from_open_fd.total_ns` | 55772362 | 57926367 | 2154005 | 1.038621x |
| `fuse_op.open.total_ns` | 106399292 | 82603985 | -23795307 | 0.776358x |
| `fuse_op.opendir.total_ns` | 83439059 | 72574601 | -10864458 | 0.869792x |
| `fuse_op.lookup.total_ns` | 373743731 | 463085041 | 89341310 | 1.239044x |
| `fuse_op.getattr.total_ns` | 63619313 | 80285501 | 16666188 | 1.261967x |

### Fast `open-confined-surface` claim-gate row

| counter | before | after | delta | after/before |
| --- | ---: | ---: | ---: | ---: |
| `open_confined_openat2.total_ns` | 68255106 | 55984755 | -12270351 | 0.820228x |
| `source_root_path.total_ns` | 67038812 | 64057284 | -2981528 | 0.955525x |
| `resolved_virtual_path_from_open_fd.total_ns` | 57515448 | 54464718 | -3050730 | 0.946958x |
| `fuse_op.open.total_ns` | 82259994 | 80904296 | -1355698 | 0.983519x |
| `fuse_op.opendir.total_ns` | 65741690 | 60370892 | -5370798 | 0.918305x |
| `fuse_op.lookup.total_ns` | 290230725 | 268424729 | -21805996 | 0.924867x |
| `fuse_op.getattr.total_ns` | 46725254 | 42062546 | -4662708 | 0.900210x |

### Fast `open-confined-surface` rerun context

| counter | before | after | delta | after/before |
| --- | ---: | ---: | ---: | ---: |
| `open_confined_openat2.total_ns` | 68255106 | 83374320 | 15119214 | 1.221510x |
| `source_root_path.total_ns` | 67038812 | 94768338 | 27729526 | 1.413634x |
| `resolved_virtual_path_from_open_fd.total_ns` | 57515448 | 60190188 | 2674740 | 1.046505x |
| `fuse_op.open.total_ns` | 82259994 | 51355935 | -30904059 | 0.624312x |
| `fuse_op.opendir.total_ns` | 65741690 | 49302488 | -16439202 | 0.749943x |
| `fuse_op.lookup.total_ns` | 290230725 | 356025839 | 65795114 | 1.226699x |
| `fuse_op.getattr.total_ns` | 46725254 | 58044312 | 11319058 | 1.242247x |

## Conclusion

- The focused fallback `metadata_opendir` slice improved materially (`metadata_opendir` `0.658506x` p50 / `0.672982x` p95 / `0.674903x` p99) and reduced the attribution counters we care about, including `open_confined_openat2.total_ns` `21589058 -> 9533021`, `source_root_path.total_ns` `50300139 -> 30979325`, and `resolved_virtual_path_from_open_fd.total_ns` `35503324 -> 19476992`.
- Those focused improvements were **not** sufficient to keep the code. The final fallback `open-confined-surface` after run regressed materially on both gate workloads: `metadata_open` `1.341903x` p50 / `1.432767x` p95 / `1.450611x` p99 and `metadata_opendir` `1.152927x` p50 / `1.111275x` p95 / `1.097032x` p99.
- The fast `open-confined-surface` row also failed the gate. The first fast after run kept `metadata_open` tail regressions despite lower focused counters (`metadata_open` `0.959265x` p50 but `1.185284x` p95 and `1.302269x` p99), and the rerun regressed both gate workloads (`metadata_open` `1.149197x` p50, `metadata_opendir` `1.117388x` p50).
- The useful retained outcome is attribution: the focused `metadata_opendir` row and the surface-row counter deltas show where cost can move. But this bundle is rejected/slice context only. It does **not** provide kept `open_confined` speedup evidence, and it should not change the current `smoke` label for this row.
