# current readdirplus page/scan attribution smoke

## Scope

This is a measure-only refinement for the still-unfinished `readdirplus` page/scan lane on top of the current worktree, which already includes the kept direct opened-object `stat_child_no_follow()` change from the `open_confined` lane. The current worktree adds directory scan sub-bucket perf counters for both `readdir` and `readdirplus`; this smoke artifact interprets the `readdirplus` side so the next behavior-changing attempt can target the dominant part of `readdirplus_directory_scan` without reopening rejected directions such as returned-entry policy-recheck skipping, bounded candidate selection, or path-join retuning.

This is **not** a speedup claim and has no before/after gate.

## Commands

All rows used the current worktree binary and non-root `scripts/bench-screenfs.py --build --perf-counters --iterations 3 --warmups 1` and wrote JSON/Markdown/SVG companions in this directory.

- `fast-directory-surface.*`: `--policy-preset fast-path-cache-eligible --workload-set directory-surface`
- `fast-readdirplus-20k.*`: `--policy-preset fast-path-cache-eligible --dir-entries 20000 --workload readdirplus_basic`
- `fallback-matcher32-directory-surface.*`: `--policy-preset fallback-unsafe-policy --matcher-extra-rules 32 --policy-label glob-matcher-heavy --workload-set directory-surface`

## New counter surface

The existing aggregate counters remain source-of-truth for broad gate checks. The new sub-buckets are attribution only:

- `readdir_scan.name_child_path_materialization`
- `readdir_scan.scan_visibility`
- `readdir_scan.scan_fallback_attr`
- `readdirplus_scan.name_child_path_materialization`
- `readdirplus_scan.scan_visibility`
- `readdirplus_scan.scan_fallback_attr`
- `readdirplus_scan.returned_attr_hydration`
- `readdirplus_scan.returned_policy_recheck`
- `readdirplus_scan.returned_symlink_visibility`

These sub-buckets are not an additive partition of `readdirplus_directory_scan`, and they should be read as current-worktree attribution rather than isolated no-code-change control: returned-page hydration/recheck happens after bounded candidate selection, while the aggregate scan bucket covers the backing directory walk and scan-time filtering.

## Observed attribution

| row | workload | mounted p50 | mounted/native p50 | notable `readdirplus` counters |
| --- | --- | ---: | ---: | --- |
| fast directory-surface | `readdirplus_basic` | 0.245208s | 35.556x | `directory_scan=127695685ns`, `scan_visibility=91984803ns`, `name_child_path_materialization=12262231ns`, `candidate_selection=9390370ns`, `page_commit=2645048ns` |
| fast focused 20k | `readdirplus_basic` | 1.542030s | 48.918x | `directory_scan=736738341ns`, `scan_visibility=528955527ns`, `name_child_path_materialization=66324685ns`, `candidate_selection=51262968ns`, `page_commit=15608409ns` |
| fallback matcher32 directory-surface | `readdirplus_basic` | 0.330016s | 28.668x | `directory_scan=259412653ns`, `scan_visibility=214514591ns`, `name_child_path_materialization=18125269ns`, `candidate_selection=12306100ns`, `page_commit=3765695ns` |

`scan_fallback_attr` stayed zero in these rows, which means the local backing filesystem supplied dirent types for this fixture. The largest visible sub-bucket is scan-time visibility/policy work, especially in the 20k row.

## Validation / review status

Focused verification before the smoke run:

- `cargo check --features perf-counters`: pass
- `cargo test --features perf-counters perf_counters_record_policy_state_open_and_readdirplus_attr_work`: pass

Final validation/review for the surrounding goal is recorded in the session output and canonical docs updates. This smoke is intended to guide the next behavior-changing attempt, not to establish a latency win.
