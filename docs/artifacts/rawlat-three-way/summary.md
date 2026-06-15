# Three-way raw fio clat box plot

This artifact compares native, managed `contrib/fractal-passthrough`, and ScreenFS using the same fio raw completion-latency (`clat`) sample metric on all three sides. It is supplemental fixed-overhead attribution evidence.

## Files

- `docs/artifacts/rawlat-three-way/boxplot-stats.json`
- `docs/artifacts/rawlat-three-way/boxplot.svg`
- `docs/artifacts/rawlat-three-way/env.json`
- `docs/artifacts/rawlat-three-way/native.json`
- `docs/artifacts/rawlat-three-way/native_clat.1.log.gz`
- `docs/artifacts/rawlat-three-way/native_clat.2.log.gz`
- `docs/artifacts/rawlat-three-way/native_clat.3.log.gz`
- `docs/artifacts/rawlat-three-way/native_clat.4.log.gz`
- `docs/artifacts/rawlat-three-way/native_clat.5.log.gz`
- `docs/artifacts/rawlat-three-way/passthrough-fstype.txt`
- `docs/artifacts/rawlat-three-way/passthrough.json`
- `docs/artifacts/rawlat-three-way/passthrough.stderr.log`
- `docs/artifacts/rawlat-three-way/passthrough.stdout.log`
- `docs/artifacts/rawlat-three-way/passthrough_clat.1.log.gz`
- `docs/artifacts/rawlat-three-way/passthrough_clat.2.log.gz`
- `docs/artifacts/rawlat-three-way/passthrough_clat.3.log.gz`
- `docs/artifacts/rawlat-three-way/passthrough_clat.4.log.gz`
- `docs/artifacts/rawlat-three-way/passthrough_clat.5.log.gz`
- `docs/artifacts/rawlat-three-way/screenfs-fstype.txt`
- `docs/artifacts/rawlat-three-way/screenfs.json`
- `docs/artifacts/rawlat-three-way/screenfs.stderr.log`
- `docs/artifacts/rawlat-three-way/screenfs.stdout.log`
- `docs/artifacts/rawlat-three-way/screenfs_clat.1.log.gz`
- `docs/artifacts/rawlat-three-way/screenfs_clat.2.log.gz`
- `docs/artifacts/rawlat-three-way/screenfs_clat.3.log.gz`
- `docs/artifacts/rawlat-three-way/screenfs_clat.4.log.gz`
- `docs/artifacts/rawlat-three-way/screenfs_clat.5.log.gz`

## p50 / p95 / p99 raw clat (µs)

| job | side | samples | p50 | p95 | p99 | mean |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `seq_write_128k` | `native` | 128 | 33.666 | 40.061 | 42.256 | 34.563 |
| `seq_read_128k` | `native` | 128 | 32.223 | 47.594 | 65.015 | 35.318 |
| `rand_write_4k` | `native` | 2038463 | 0.602 | 0.918 | 1.264 | 0.644 |
| `rand_read_4k` | `native` | 3535076 | 0.463 | 0.708 | 0.997 | 0.498 |
| `sync_write_4k` | `native` | 1024 | 1.732 | 3.013 | 3.703 | 1.771 |
| `seq_write_128k` | `passthrough` | 128 | 50.758 | 72.023 | 112.300 | 55.019 |
| `seq_read_128k` | `passthrough` | 128 | 111.001 | 232.221 | 346.288 | 117.688 |
| `rand_write_4k` | `passthrough` | 406938 | 5.840 | 9.493 | 12.396 | 6.318 |
| `rand_read_4k` | `passthrough` | 260629 | 9.686 | 15.609 | 19.291 | 10.668 |
| `sync_write_4k` | `passthrough` | 1024 | 6.261 | 14.174 | 18.860 | 7.895 |
| `seq_write_128k` | `screenfs` | 128 | 151.881 | 210.236 | 257.882 | 143.299 |
| `seq_read_128k` | `screenfs` | 128 | 115.491 | 239.508 | 322.549 | 107.038 |
| `rand_write_4k` | `screenfs` | 72386 | 39.207 | 47.738 | 74.641 | 40.176 |
| `rand_read_4k` | `screenfs` | 81283 | 34.036 | 47.454 | 69.312 | 35.945 |
| `sync_write_4k` | `screenfs` | 1024 | 37.975 | 51.811 | 102.119 | 40.675 |
