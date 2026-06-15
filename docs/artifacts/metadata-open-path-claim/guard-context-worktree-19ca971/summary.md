# Request-local guard context metadata/open-path evidence

Generated: 2026-06-15T20:51:23.468680+00:00

This bundle compares detached `HEAD` before binaries against the current dirty worktree after binaries on the same machine with the same harness parameters. Latency claim rows use non-`perf-counters` release binaries with `--iterations 10 --warmups 3 --metadata-ops 4096`; attribution rows use `perf-counters` release binaries with `--iterations 10 --warmups 3 --metadata-ops 1024`. Static validation output is captured in `validation.log`.

## Latency ratios (after / before; lower is better)

### fast-path-cache-eligible

| workload | p50 ratio | p95 ratio | p99 ratio | p50 before -> after (s) |
|---|---:|---:|---:|---:|
| `metadata_lookup` | 0.850 | 0.852 | 0.852 | 0.347583 -> 0.295485 |
| `metadata_getattr` | 0.858 | 0.840 | 0.838 | 0.452467 -> 0.388141 |
| `metadata_open` | 0.921 | 0.880 | 0.875 | 0.457995 -> 0.421639 |
| `metadata_readlink` | 0.865 | 0.840 | 0.834 | 0.389969 -> 0.337201 |
| `metadata_access` | 0.915 | 0.912 | 0.915 | 0.453971 -> 0.415391 |

### fallback-unsafe-policy

| workload | p50 ratio | p95 ratio | p99 ratio | p50 before -> after (s) |
|---|---:|---:|---:|---:|
| `metadata_lookup` | 0.803 | 0.790 | 0.789 | 0.468193 -> 0.375964 |
| `metadata_getattr` | 0.817 | 0.840 | 0.859 | 0.614519 -> 0.501853 |
| `metadata_open` | 0.879 | 0.884 | 0.889 | 0.623411 -> 0.547716 |
| `metadata_readlink` | 0.794 | 0.798 | 0.798 | 0.591145 -> 0.469135 |
| `metadata_access` | 0.868 | 0.870 | 0.870 | 0.614495 -> 0.533484 |

### glob-matcher-heavy

| workload | p50 ratio | p95 ratio | p99 ratio | p50 before -> after (s) |
|---|---:|---:|---:|---:|
| `metadata_lookup` | 0.789 | 0.755 | 0.748 | 0.479448 -> 0.378071 |
| `metadata_getattr` | 0.785 | 0.769 | 0.767 | 0.643219 -> 0.505008 |
| `metadata_open` | 0.879 | 0.870 | 0.871 | 0.642305 -> 0.564848 |
| `metadata_readlink` | 0.812 | 0.821 | 0.820 | 0.608618 -> 0.494487 |
| `metadata_access` | 0.833 | 0.817 | 0.821 | 0.643425 -> 0.535858 |

Interpretation: all five primary metadata/open-path workloads improve at p50/p95/p99 across the fast-path-cache-eligible, fallback-unsafe-policy, and glob/matcher-heavy rows. `metadata_lookup` improved after reusing the initial child stat failure in the read guard instead of re-statting missing entries; older lookup-only supplemental reruns below are retained as historical noise context from before that final fix.

Historical fast-path lookup-only supplemental reruns (`--metadata-ops 4096`, lookup only):

| run | p50 (s) | p95 (s) | p99 (s) |
|---|---:|---:|---:|
| `before` | 0.365671 | 0.380844 | 0.381158 |
| `after` | 0.364196 | 0.370917 | 0.371630 |
| `before2` | 0.356269 | 0.361996 | 0.362017 |
| `after2` | 0.357178 | 0.372343 | 0.374525 |

## Perf-counter attribution

### fast-path-cache-eligible

| counter | count before -> after | total_ns before -> after |
|---|---:|---:|
| `source_root_path` | 505869 -> 279559 | 1671309376 -> 982735795 |
| `resolved_virtual_path_from_open_fd` | 505869 -> 279559 | 912156041 -> 522294529 |
| `open_confined_openat2` | 505870 -> 279560 | 552775116 -> 337235852 |
| `policy_decision` | 772116 -> 545806 | 238214004 -> 182822071 |
| `matcher_candidate_order.path` | 2316348 -> 1637418 | 268701037 -> 195375717 |
| `matcher_candidate_order.descendant` | 772116 -> 545806 | 86991988 -> 68467646 |
| `stat_child_no_follow` | 0 -> 252935 | 0 -> 3109360359 |

### fallback-unsafe-policy

| counter | count before -> after | total_ns before -> after |
|---|---:|---:|
| `source_root_path` | 758804 -> 279559 | 2513095605 -> 1000492567 |
| `resolved_virtual_path_from_open_fd` | 505869 -> 279559 | 941372889 -> 534331841 |
| `open_confined_openat2` | 505870 -> 279560 | 531524363 -> 336599493 |
| `policy_decision` | 798740 -> 572430 | 333409460 -> 257835323 |
| `matcher_candidate_order.path` | 2396220 -> 1717290 | 387755740 -> 289709445 |
| `matcher_candidate_order.descendant` | 798740 -> 572430 | 93054852 -> 74902388 |
| `stat_child_no_follow` | 0 -> 252935 | 0 -> 3192402200 |

### glob-matcher-heavy

| counter | count before -> after | total_ns before -> after |
|---|---:|---:|
| `source_root_path` | 758804 -> 279559 | 2516352012 -> 1009298624 |
| `resolved_virtual_path_from_open_fd` | 505869 -> 279559 | 930134868 -> 533838229 |
| `open_confined_openat2` | 505870 -> 279560 | 561686879 -> 358923546 |
| `policy_decision` | 798740 -> 572430 | 564613901 -> 469500257 |
| `matcher_candidate_order.path` | 2396220 -> 1717290 | 641823933 -> 517203407 |
| `matcher_candidate_order.descendant` | 798740 -> 572430 | 92523906 -> 74270654 |
| `stat_child_no_follow` | 0 -> 252935 | 0 -> 3338071692 |

Notes: after-only `stat_child_no_follow` exists because this work adds the counter; compare it as attribution, not as a before/after reduction counter. `open_confined_openat2` count decreases rather than increasing. `source_root_path`, `resolved_virtual_path_from_open_fd`, policy decision, and matcher candidate-order totals decrease across policy rows.
