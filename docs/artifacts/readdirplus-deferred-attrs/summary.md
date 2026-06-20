# Readdirplus deferred attr-generation experiment

This bundle records a same-machine before/after experiment for deferring `readdirplus` child attribute generation until after candidate page selection. The implementation keeps `readdir` behavior unchanged and still generates and revalidates attributes for every committed `readdirplus` page entry before replying.

## Commands

Baseline and after runs used the official benchmark harness with perf counters:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-surface --iterations 10 --warmups 3 --dir-entries 5000 --output-json docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.json --output-md docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.md --output-svg docs/artifacts/readdirplus-deferred-attrs/before-directory-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --workload-set directory-surface --iterations 10 --warmups 3 --dir-entries 5000 --output-json docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.json --output-md docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.md --output-svg docs/artifacts/readdirplus-deferred-attrs/after-directory-surface.svg
python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.json --output-md docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-deferred-attrs/before-readdirplus-20k.svg
python3 scripts/bench-screenfs.py --build --perf-counters --workload readdirplus_basic --iterations 5 --warmups 2 --dir-entries 20000 --output-json docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.json --output-md docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.md --output-svg docs/artifacts/readdirplus-deferred-attrs/after-readdirplus-20k.svg
```

## Results

| run | workload | mounted p50 before | mounted p50 after | p50 ratio | mounted p95 before | mounted p95 after | p95 ratio |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| directory-surface 5k | `readdir_basic` | 0.070411 | 0.054051 | 0.768 | 0.071321 | 0.055994 | 0.785 |
| directory-surface 5k | `readdirplus_basic` | 0.463973 | 0.334404 | 0.721 | 0.478372 | 0.350182 | 0.732 |
| readdirplus 20k | `readdirplus_basic` | 2.391141 | 2.458977 | 1.028 | 2.441640 | 2.494807 | 1.022 |

Attribution moved in the intended direction:

| run | before `readdirplus_attr_generation_entries` | after `readdirplus_attr_generation_entries` | before attr total ns | after attr total ns | before directory scan total ns | after directory scan total ns |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| directory-surface 5k | 139868 | 5828 | 112753590 | 37427780 | 438197292 | 288113522 |
| readdirplus 20k | 327378 | 5082 | 202932867 | 40751062 | 922414448 | 843851072 |

## Conclusion

The change is kept as an attribution-backed reduction in unnecessary `readdirplus` attr generation for non-returned entries. It is **not** a broad latency speedup claim: the 5k mixed directory surface improved in this same-machine pair (`readdirplus_basic` p50 0.721x, p95 0.732x), but the 20k focused row regressed slightly (`1.028x` p50, `1.022x` p95). Use this bundle as scoped evidence for reduced attr work and as input for future readdirplus page-work tuning, not as standalone claim-grade latency proof across directory sizes.
