# Follow-up performance before/after summary

This claim-grade-format run uses `--iterations 10 --warmups 3` with reduced fixture sizes and `--matcher-extra-rules 20`. It is evidence for this machine/worktree only; inspect raw JSON samples before making a broad performance claim.

- before: `docs/artifacts/followup-perf/before-4a826e3.json` git `4a826e352c96967da1f8cd589f4b0f341be57617` bin `/tmp/screenfs-before-4a826e3/target/release/screenfs` sha `75e858dfeb104a1064e2a7ab5cfa4adeeee1dea2011b7d8c0b221d2eb0ac9b2b`
- after: `docs/artifacts/followup-perf/after-current.json` git `4a826e352c96967da1f8cd589f4b0f341be57617` bin `target/release/screenfs` sha `2a3335c58ca1419a2e38ee5ca257f1578c98143c901ff6609819da70bf12eb5d` worktree_clean `False`

## Comparable mounted workload tails

| workload | before p50 | after p50 | after/before p50 | before p95 | after p95 | after/before p95 | before p99 | after p99 | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `readdir_lstat` | 0.058503 | 0.039660 | 0.678 | 0.060084 | 0.041059 | 0.683 | 0.060415 | 0.041302 | 0.684 |
| `seq_read` | 0.002905 | 0.002657 | 0.915 | 0.003043 | 0.003025 | 0.994 | 0.003081 | 0.003070 | 0.996 |
| `seq_write` | 0.002162 | 0.002069 | 0.957 | 0.002571 | 0.002519 | 0.980 | 0.002625 | 0.002520 | 0.960 |
| `small_read` | 0.001764 | 0.002004 | 1.136 | 0.002124 | 0.002283 | 1.075 | 0.002317 | 0.002380 | 1.027 |
| `small_stat_open_read` | 0.069462 | 0.045797 | 0.659 | 0.072065 | 0.048610 | 0.675 | 0.073143 | 0.048906 | 0.669 |
| `small_write` | 0.003888 | 0.003392 | 0.872 | 0.004814 | 0.003796 | 0.789 | 0.005271 | 0.003899 | 0.740 |
| `symlink_open_read` | 0.000347 | 0.000198 | 0.570 | 0.000383 | 0.000224 | 0.584 | 0.000384 | 0.000229 | 0.595 |
| `write_fsync_close` | 0.008085 | 0.005722 | 0.708 | 0.009690 | 0.006066 | 0.626 | 0.009781 | 0.006161 | 0.630 |

## ScreenFS-only workload tails

| workload | before p50 | after p50 | after/before p50 | before p95 | after p95 | after/before p95 | before p99 | after p99 | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `hidden_stat_miss` | 0.009080 | 0.006370 | 0.701 | 0.009175 | 0.007044 | 0.768 | 0.009175 | 0.007100 | 0.774 |
| `matcher_hidden_stat_miss` | 0.015278 | 0.009771 | 0.640 | 0.015595 | 0.010564 | 0.677 | 0.015653 | 0.010688 | 0.683 |
| `symlink_parent_mkdir_rmdir` | 0.276760 | 0.181465 | 0.656 | 0.278536 | 0.186168 | 0.668 | 0.278863 | 0.186242 | 0.668 |

## Selected perf counters

| counter | before count | before total ns | after count | after total ns | after/before total |
| --- | ---: | ---: | ---: | ---: | ---: |
| `policy_decision` | 579586 | 832021022 | 579586 | 347165276 | 0.417 |
| `matcher_candidates` | 5200 | 5200 | 5200 | 5200 | 1.000 |
| `matcher_candidate_order.path` | 0 | 0 | 1720428 | 366942460 | 0.000 |
| `resolved_virtual_path_from_path` | 157747 | 522576764 | 157747 | 456866538 | 0.874 |
| `resolved_virtual_path_from_path_component_walk` | 0 | 0 | 157747 | 403697885 | 0.000 |
| `resolved_virtual_path_from_path_canonicalize` | 0 | 0 | 385485 | 333543073 | 0.000 |
| `resolved_virtual_path_from_path_source_root_confinement` | 0 | 0 | 385485 | 32628504 | 0.000 |
| `resolved_virtual_path_from_path_virtual_conversion` | 0 | 0 | 157747 | 45724662 | 0.000 |
| `readdir_directory_scan` | 0 | 0 | 51 | 12099633 | 0.000 |
| `readdir_attr_generation_scan` | 51 | 34204540 | 51 | 1980695 | 0.058 |
| `readdir_candidate_selection` | 0 | 0 | 51 | 499969 | 0.000 |
| `readdir_page_commit` | 0 | 0 | 51 | 3042096 | 0.000 |
| `readdirplus_directory_scan` | 0 | 0 | 27 | 25805254 | 0.000 |
| `readdirplus_attr_generation_scan` | 27 | 52531393 | 27 | 4790289 | 0.091 |
| `readdirplus_candidate_selection` | 0 | 0 | 27 | 1262479 | 0.000 |
| `readdirplus_page_commit` | 0 | 0 | 27 | 3085294 | 0.000 |

## Interpretation

- Some mounted workload samples overlap and this reduced-size run should not be generalized across machines or policies.
- New split counters are present only in the after run; before/after ratios for newly added counters are structural attribution evidence, not speedup evidence.
- `--matcher-extra-rules 20` adds hidden subtree and exact readonly synthetic rules, but this built-in run exercises the hidden subtree miss workload; use custom workloads before claiming readonly-rule-family coverage.
- Use the raw JSON artifacts for sample-level audit: every workload stores all 10 measured samples.
