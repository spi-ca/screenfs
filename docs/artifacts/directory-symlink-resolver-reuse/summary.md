# Directory symlink resolver reuse experiment

This bundle records a rejected request-local resolver reuse experiment for directory symlink visibility checks. The experiment reused one `RequestPathResolver` across symlink target visibility checks inside `collect_child_directory_page()` instead of constructing a resolver for each symlink entry.

## Command shape

Both runs used:

```bash
python3 scripts/bench-screenfs.py --build --perf-counters \
  --workload-set directory-symlink-surface \
  --iterations 10 --warmups 3 --dir-entries 200
```

## Result

| workload | before p50 | after p50 | after/before p50 | before p95 | after p95 | after/before p95 | before p99 | after p99 | after/before p99 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `readdir_symlink_visibility` | 0.002527 | 0.003009 | 1.191x | 0.002733 | 0.003333 | 1.220x | 0.002751 | 0.003344 | 1.215x |
| `readdirplus_symlink_visibility` | 0.014520 | 0.021127 | 1.455x | 0.016361 | 0.021546 | 1.317x | 0.017221 | 0.021596 | 1.254x |

Selected perf-counter totals:

| counter | before | after |
| --- | ---: | ---: |
| `source_root_path.count` | 16205 | 10823 |
| `source_root_path.total_ns` | 18616755 | 22391580 |
| `readdir_symlink_visibility.total_ns` | 1570358 | 1784507 |
| `readdirplus_symlink_visibility.total_ns` | 35467597 | 38407344 |
| `fuse_op.readdir.total_ns` | 3917161 | 5717512 |
| `fuse_op.readdirplus.total_ns` | 52345440 | 61205293 |

## Conclusion

Do not use this bundle as speedup evidence. Although the experiment reduced `source_root_path.count`, same-machine mounted latency and relevant symlink visibility/FUSE totals worsened. The implementation is not kept in the current code. The useful retained work is the `directory-symlink-surface` workload and stronger symlink fanout fixture/checks, which provide smoke coverage for future symlink-directory changes.
