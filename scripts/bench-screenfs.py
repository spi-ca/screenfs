#!/usr/bin/env python3
"""Run a reproducible ScreenFS performance benchmark.

The harness compares the same prepared fixture through the native backing source
and through a live ScreenFS mount. It records machine-readable JSON and an
optional Markdown summary; benchmark timing does not include building ScreenFS or
fixture creation.
"""

from __future__ import annotations

import argparse
import hashlib
import html
import json
import os
import platform
import shutil
import shlex
import signal
import statistics
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable


MiB = 1024 * 1024


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--screenfs-bin",
        default="target/release/screenfs",
        help="Path to a prebuilt screenfs binary (default: target/release/screenfs).",
    )
    parser.add_argument(
        "--build",
        action="store_true",
        help="Run cargo build --release before benchmarking. Build time is not measured.",
    )
    parser.add_argument("--iterations", type=int, default=10, help="Measured iterations per workload.")
    parser.add_argument("--warmups", type=int, default=3, help="Warmup iterations per workload.")
    parser.add_argument("--read-mib", type=int, default=64, help="Sequential read fixture size in MiB.")
    parser.add_argument("--write-mib", type=int, default=64, help="Sequential write size per iteration in MiB.")
    parser.add_argument("--small-io-bytes", type=int, default=4096, help="Bytes per operation for small-buffer read/write workloads.")
    parser.add_argument("--small-io-ops", type=int, default=1024, help="Small-buffer read/write operations per iteration.")
    parser.add_argument("--sync-bytes", type=int, default=4096, help="Bytes written per fsync workload operation.")
    parser.add_argument("--sync-ops", type=int, default=128, help="Open/write/fsync/close operations per iteration.")
    parser.add_argument("--small-files", type=int, default=2000, help="Small files for stat/open/read workload.")
    parser.add_argument("--dir-entries", type=int, default=5000, help="Directory entries for listing workload.")
    parser.add_argument("--hidden-misses", type=int, default=2000, help="Repeated hidden-path ENOENT checks for the ScreenFS-only workload.")
    parser.add_argument(
        "--symlink-parent-mutations",
        type=int,
        default=2000,
        help="Repeated mkdir/rmdir pairs under a visible symlink parent for the ScreenFS-only workload.",
    )
    parser.add_argument("--output-json", type=Path, help="Write full benchmark result JSON to this path.")
    parser.add_argument("--output-md", type=Path, help="Write Markdown summary to this path.")
    parser.add_argument("--output-svg", type=Path, help="Write an SVG box plot of raw sample timings to this path.")
    parser.add_argument(
        "--keep-workdir",
        action="store_true",
        help="Do not delete the temporary source/mount/result workspace.",
    )
    parser.add_argument(
        "--timeout-sec",
        type=float,
        default=10.0,
        help="Seconds to wait for the ScreenFS mount to become usable.",
    )
    parser.add_argument(
        "--extra-screenfs-arg",
        action="append",
        default=[],
        help="Append one raw argument to the screenfs command; repeat as needed.",
    )
    parser.add_argument(
        "--perf-counters",
        action="store_true",
        help="Enable ScreenFS perf counters with a temporary config and record the stderr summary in JSON/Markdown artifacts.",
    )
    return parser.parse_args()


def require_positive(name: str, value: int) -> None:
    if value <= 0:
        raise SystemExit(f"{name} must be positive")


def reject_perf_config_conflict(args: argparse.Namespace) -> None:
    if not args.perf_counters:
        return
    for extra in args.extra_screenfs_arg:
        if extra == "--config" or extra.startswith("--config="):
            raise SystemExit("--perf-counters cannot be combined with --extra-screenfs-arg --config; the harness must control the temporary perf config")


def run_checked(command: list[str], **kwargs: Any) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, check=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, **kwargs)


def command_output(command: list[str]) -> str | None:
    try:
        return run_checked(command).stdout.strip()
    except (FileNotFoundError, subprocess.CalledProcessError):
        return None


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as file:
            while True:
                chunk = file.read(1024 * 1024)
                if not chunk:
                    break
                digest.update(chunk)
    except OSError as exc:
        raise SystemExit(f"failed to read screenfs binary for sha256: {path}: {exc}") from exc
    return digest.hexdigest()


def summarize_git_status(status: str | None, limit: int = 5) -> str | None:
    if status is None:
        return None
    entries = [line for line in status.splitlines() if line]
    if not entries:
        return None
    summary = "; ".join(entries[:limit])
    if len(entries) > limit:
        summary += f"; ... (+{len(entries) - limit} more)"
    return summary


def shell_join(argv: list[str]) -> str:
    return shlex.join(argv)


def parse_perf_summary(stderr: str) -> dict[str, Any] | None:
    marker = "screenfs perf counters:"
    start = stderr.rfind(marker)
    if start < 0:
        return None
    block = stderr[start:].strip()
    metrics: dict[str, Any] = {}
    for raw_line in block.splitlines()[1:]:
        line = raw_line.strip()
        if not line:
            continue
        if ":" not in line:
            continue
        name, rest = line.split(":", 1)
        values: dict[str, int] = {}
        for token in rest.strip().split():
            if "=" not in token:
                continue
            key, value = token.split("=", 1)
            try:
                values[key] = int(value)
            except ValueError:
                pass
        metrics[name] = values
    return {"raw": block, "metrics": metrics}


def harness_argv() -> list[str]:
    orig_argv = getattr(sys, "orig_argv", None)
    if orig_argv:
        return [str(arg) for arg in orig_argv]
    return [sys.executable, *sys.argv]


def write_all_fd(fd: int, data: bytes) -> None:
    view = memoryview(data)
    while view:
        written = os.write(fd, view)
        if written <= 0:
            raise OSError("short write made no progress")
        view = view[written:]


def write_bytes(path: Path, size: int, chunk: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
    try:
        remaining = size
        while remaining:
            data = chunk[: min(len(chunk), remaining)]
            write_all_fd(fd, data)
            remaining -= len(data)
    finally:
        os.close(fd)
    actual_size = path.stat().st_size
    if actual_size != size:
        raise RuntimeError(f"fixture size mismatch for {path}: expected {size}, got {actual_size}")


def prepare_fixture(source: Path, args: argparse.Namespace) -> None:
    root = source / ".screenfs-bench"
    root.mkdir(parents=True)

    chunk = bytes((i % 251 for i in range(MiB)))
    write_bytes(root / "read" / "seq.bin", args.read_mib * MiB, chunk)

    small_dir = root / "small-files"
    small_dir.mkdir()
    payload = b"screenfs-small-file-payload\n"
    for index in range(args.small_files):
        (small_dir / f"file-{index:06d}.txt").write_bytes(payload)

    dir_path = root / "dir-entries"
    dir_path.mkdir()
    for index in range(args.dir_entries):
        (dir_path / f"entry-{index:06d}.txt").write_text(f"{index}\n", encoding="utf-8")

    symlink_dir = root / "symlinks"
    symlink_dir.mkdir()
    target = symlink_dir / "target.txt"
    target.write_text("visible symlink target\n", encoding="utf-8")
    os.symlink("target.txt", symlink_dir / "link.txt")

    symlink_parent_dir = root / "symlink-parent"
    symlink_parent_dir.mkdir()
    (symlink_parent_dir / "real").mkdir()
    os.symlink("real", symlink_parent_dir / "alias")

    (root / "hidden").mkdir()
    for index in range(args.hidden_misses):
        (root / "hidden" / f"secret-{index:06d}.txt").write_text("hidden\n", encoding="utf-8")
    (root / "readonly").mkdir()
    (root / "write-native").mkdir()
    (root / "write-mounted").mkdir()


def wait_for_mount(mount: Path, proc: subprocess.Popen[str], timeout_sec: float) -> None:
    sentinel = mount / ".screenfs-bench" / "read" / "seq.bin"
    deadline = time.monotonic() + timeout_sec
    while time.monotonic() < deadline:
        if proc.poll() is not None:
            _, stderr = proc.communicate(timeout=1)
            raise RuntimeError(f"screenfs exited before mount became usable: {stderr}")
        try:
            if sentinel.is_file():
                return
        except OSError:
            pass
        time.sleep(0.05)
    raise TimeoutError(f"ScreenFS mount did not expose benchmark sentinel within {timeout_sec}s")


def unmount(mount: Path) -> dict[str, Any]:
    commands = [["fusermount3", "-u", str(mount)]]
    attempts = []
    for command in commands:
        if shutil.which(command[0]) is None:
            attempts.append({"command": command, "status": "missing"})
            continue
        completed = subprocess.run(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        attempts.append(
            {
                "command": command,
                "returncode": completed.returncode,
                "stdout": completed.stdout,
                "stderr": completed.stderr,
            }
        )
        if completed.returncode == 0:
            return {"ok": True, "attempts": attempts}
    return {"ok": False, "attempts": attempts}


def seq_read(root: Path, _args: argparse.Namespace, side: str) -> None:
    path = root / ".screenfs-bench" / "read" / "seq.bin"
    total = 0
    with path.open("rb", buffering=0) as file:
        while True:
            data = file.read(MiB)
            if not data:
                break
            total += len(data)
    if total == 0:
        raise RuntimeError(f"{side} seq_read read no data")


def write_output_path(root: Path, side: str, name: str) -> Path:
    write_dir_name = "write-native" if side == "native" else "write-mounted"
    return root / ".screenfs-bench" / write_dir_name / name


def seq_write(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "seq-write.bin")
    chunk = b"w" * MiB
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
    try:
        for _ in range(args.write_mib):
            write_all_fd(fd, chunk)
    finally:
        os.close(fd)
    path.unlink(missing_ok=True)


def small_read(root: Path, args: argparse.Namespace, side: str) -> None:
    path = root / ".screenfs-bench" / "read" / "seq.bin"
    if path.stat().st_size == 0:
        raise RuntimeError(f"{side} small_read fixture is empty")
    total = 0
    with path.open("rb", buffering=0) as file:
        for _ in range(args.small_io_ops):
            remaining = args.small_io_bytes
            while remaining:
                data = file.read(remaining)
                if data:
                    total += len(data)
                    remaining -= len(data)
                    continue
                file.seek(0)
    if total == 0:
        raise RuntimeError(f"{side} small_read read no data")


def small_write(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "small-write.bin")
    chunk = b"s" * args.small_io_bytes
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
    try:
        for _ in range(args.small_io_ops):
            write_all_fd(fd, chunk)
    finally:
        os.close(fd)
    path.unlink(missing_ok=True)


def write_fsync_close(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "write-fsync-close.bin")
    chunk = b"f" * args.sync_bytes
    try:
        for _ in range(args.sync_ops):
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
            try:
                write_all_fd(fd, chunk)
                os.fsync(fd)
            finally:
                os.close(fd)
            path.unlink(missing_ok=True)
    finally:
        path.unlink(missing_ok=True)


def small_stat_open_read(root: Path, _args: argparse.Namespace, _side: str) -> None:
    small_dir = root / ".screenfs-bench" / "small-files"
    total = 0
    for entry in sorted(small_dir.iterdir()):
        stat = entry.stat()
        with entry.open("rb", buffering=0) as file:
            total += len(file.read(32)) + stat.st_size
    if total == 0:
        raise RuntimeError("small_stat_open_read read no data")


def readdir_lstat(root: Path, _args: argparse.Namespace, _side: str) -> None:
    dir_path = root / ".screenfs-bench" / "dir-entries"
    count = 0
    with os.scandir(dir_path) as entries:
        for entry in entries:
            entry.stat(follow_symlinks=False)
            count += 1
    if count == 0:
        raise RuntimeError("readdir_lstat saw no entries")


def symlink_open_read(root: Path, _args: argparse.Namespace, _side: str) -> None:
    path = root / ".screenfs-bench" / "symlinks" / "link.txt"
    with path.open("rb", buffering=0) as file:
        if not file.read(64):
            raise RuntimeError("symlink_open_read read no data")


def hidden_stat_miss(root: Path, args: argparse.Namespace, side: str) -> None:
    for index in range(args.hidden_misses):
        path = root / ".screenfs-bench" / "hidden" / f"secret-{index:06d}.txt"
        try:
            path.stat()
        except FileNotFoundError:
            if side == "mounted":
                continue
            raise
        if side == "mounted":
            raise RuntimeError("hidden_stat_miss expected ENOENT through ScreenFS")


def symlink_parent_mkdir_rmdir(root: Path, args: argparse.Namespace, _side: str) -> None:
    fixture_root = root / ".screenfs-bench" / "symlink-parent"
    real_parent = fixture_root / "real"
    alias_parent = fixture_root / "alias"
    if not alias_parent.is_dir():
        raise RuntimeError("symlink_parent_mkdir_rmdir missing alias parent")

    created: list[Path] = []
    try:
        for index in range(args.symlink_parent_mutations):
            name = f"bench-child-{index:06d}"
            alias_child = alias_parent / name
            real_child = real_parent / name
            if alias_child.exists() or real_child.exists():
                raise RuntimeError(f"symlink_parent_mkdir_rmdir saw unexpected preexisting path: {alias_child}")
            os.mkdir(alias_child, 0o755)
            created.append(real_child)
            if not real_child.is_dir():
                raise RuntimeError(f"symlink_parent_mkdir_rmdir expected directory at {real_child}")
            os.rmdir(alias_child)
            created.pop()
            if real_child.exists():
                raise RuntimeError(f"symlink_parent_mkdir_rmdir expected {real_child} to be removed")
    finally:
        for leftover in reversed(created):
            try:
                os.rmdir(leftover)
            except FileNotFoundError:
                pass


WORKLOADS: dict[str, Callable[[Path, argparse.Namespace, str], None]] = {
    "seq_read": seq_read,
    "seq_write": seq_write,
    "small_read": small_read,
    "small_write": small_write,
    "write_fsync_close": write_fsync_close,
    "small_stat_open_read": small_stat_open_read,
    "readdir_lstat": readdir_lstat,
    "symlink_open_read": symlink_open_read,
}
SCREENFS_ONLY_WORKLOADS: dict[str, Callable[[Path, argparse.Namespace, str], None]] = {
    "hidden_stat_miss": hidden_stat_miss,
    "symlink_parent_mkdir_rmdir": symlink_parent_mkdir_rmdir,
}


def time_one(func: Callable[[Path, argparse.Namespace, str], None], root: Path, args: argparse.Namespace, side: str) -> float:
    start = time.perf_counter_ns()
    func(root, args, side)
    end = time.perf_counter_ns()
    return (end - start) / 1_000_000_000


def percentile(sorted_values: list[float], percent: float) -> float:
    if not sorted_values:
        raise ValueError("percentile needs at least one value")
    if len(sorted_values) == 1:
        return sorted_values[0]
    position = (len(sorted_values) - 1) * percent
    lower = int(position)
    upper = min(lower + 1, len(sorted_values) - 1)
    fraction = position - lower
    return sorted_values[lower] + (sorted_values[upper] - sorted_values[lower]) * fraction


def summarize(values: list[float]) -> dict[str, float]:
    ordered = sorted(values)
    return {
        "min_sec": ordered[0],
        "q1_sec": percentile(ordered, 0.25),
        "p50_sec": percentile(ordered, 0.50),
        "median_sec": statistics.median(ordered),
        "q3_sec": percentile(ordered, 0.75),
        "p90_sec": percentile(ordered, 0.90),
        "p95_sec": percentile(ordered, 0.95),
        "p99_sec": percentile(ordered, 0.99),
        "mean_sec": statistics.mean(ordered),
        "max_sec": ordered[-1],
        "stdev_sec": statistics.stdev(ordered) if len(ordered) > 1 else 0.0,
    }


def measure_workload(
    name: str,
    func: Callable[[Path, argparse.Namespace, str], None],
    root: Path,
    args: argparse.Namespace,
    side: str,
) -> dict[str, Any]:
    for _ in range(args.warmups):
        time_one(func, root, args, side)
    samples = [time_one(func, root, args, side) for _ in range(args.iterations)]
    return {"name": name, "side": side, "samples_sec": samples, "summary": summarize(samples)}


def path_fs_info(path: Path) -> dict[str, Any]:
    statvfs = os.statvfs(path)
    return {
        "path": str(path),
        "findmnt": command_output(["findmnt", "-T", str(path), "-no", "FSTYPE,SOURCE,OPTIONS"]),
        "stat_f_type": command_output(["stat", "-f", "-c", "%T", str(path)]),
        "block_size": statvfs.f_bsize,
        "fragment_size": statvfs.f_frsize,
    }


def render_box_plot_svg(result: dict[str, Any]) -> str:
    series = []
    for item in result["native"]:
        series.append((item["name"], "native", item["samples_sec"], "#94a3b8"))
    for item in result["mounted"]:
        series.append((item["name"], "mounted", item["samples_sec"], "#2563eb"))
    for item in result["screenfs_only"]:
        series.append((item["name"], "mounted-only", item["samples_sec"], "#16a34a"))

    max_value = max(max(values) for _, _, values, _ in series)
    scale_max = max_value * 1.10 if max_value > 0 else 1.0
    width = 1200
    top = 70
    left = 210
    right = 40
    row_height = 34
    bottom = 70
    plot_width = width - left - right
    height = top + bottom + row_height * len(series)

    def x(value: float) -> float:
        return left + (value / scale_max) * plot_width

    def text(value: str) -> str:
        return html.escape(value, quote=True)

    lines = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
        "<style>",
        "text{font-family:Inter,Pretendard,Arial,sans-serif;fill:#111827;font-size:13px}",
        ".axis{stroke:#64748b;stroke-width:1}.grid{stroke:#e2e8f0;stroke-width:1}.median{stroke:#111827;stroke-width:2}.whisker{stroke:#475569;stroke-width:1.5}.sample{fill:#0f172a;opacity:.35}",
        "</style>",
        f'<text x="{left}" y="30" font-size="20" font-weight="700">ScreenFS benchmark box plot</text>',
        f'<text x="{left}" y="50">seconds; lower is better; box=min/q1/p50/q3/max; p90/p95/p99 markers shown; timestamp {text(result["timestamp"])}</text>',
    ]
    tick_count = 5
    for tick in range(tick_count + 1):
        value = scale_max * tick / tick_count
        xpos = x(value)
        lines.append(f'<line class="grid" x1="{xpos:.2f}" y1="{top - 10}" x2="{xpos:.2f}" y2="{height - bottom + 10}"/>')
        lines.append(f'<text x="{xpos:.2f}" y="{height - 25}" text-anchor="middle">{value:.4f}</text>')
    lines.append(f'<line class="axis" x1="{left}" y1="{height - bottom + 10}" x2="{width - right}" y2="{height - bottom + 10}"/>')

    for index, (name, side, values, color) in enumerate(series):
        ordered = sorted(values)
        minimum = ordered[0]
        q1 = percentile(ordered, 0.25)
        median = percentile(ordered, 0.50)
        q3 = percentile(ordered, 0.75)
        maximum = ordered[-1]
        p90 = percentile(ordered, 0.90)
        p95 = percentile(ordered, 0.95)
        p99 = percentile(ordered, 0.99)
        cy = top + index * row_height + row_height / 2
        box_top = cy - 9
        box_height = 18
        label = f"{name} [{side}]"
        lines.append(f'<text x="12" y="{cy + 4:.2f}">{text(label)}</text>')
        lines.append(f'<line class="whisker" x1="{x(minimum):.2f}" y1="{cy:.2f}" x2="{x(maximum):.2f}" y2="{cy:.2f}"/>')
        lines.append(f'<line class="whisker" x1="{x(minimum):.2f}" y1="{cy - 7:.2f}" x2="{x(minimum):.2f}" y2="{cy + 7:.2f}"/>')
        lines.append(f'<line class="whisker" x1="{x(maximum):.2f}" y1="{cy - 7:.2f}" x2="{x(maximum):.2f}" y2="{cy + 7:.2f}"/>')
        box_x = x(q1)
        box_w = max(1.0, x(q3) - x(q1))
        lines.append(f'<rect x="{box_x:.2f}" y="{box_top:.2f}" width="{box_w:.2f}" height="{box_height}" fill="{color}" opacity="0.28" stroke="{color}"/>')
        lines.append(f'<line class="median" x1="{x(median):.2f}" y1="{box_top:.2f}" x2="{x(median):.2f}" y2="{box_top + box_height:.2f}"/>')
        for sample in values:
            lines.append(f'<circle class="sample" cx="{x(sample):.2f}" cy="{cy:.2f}" r="2.5"/>')
        for value, label, marker_color, offset in [
            (p90, "p90", "#f59e0b", -10),
            (p95, "p95", "#ef4444", 0),
            (p99, "p99", "#7c3aed", 10),
        ]:
            xpos = x(value)
            points = f"{xpos:.2f},{cy - 13 + offset / 10:.2f} {xpos - 4:.2f},{cy - 5 + offset / 10:.2f} {xpos + 4:.2f},{cy - 5 + offset / 10:.2f}"
            lines.append(f'<polygon points="{points}" fill="{marker_color}"><title>{label} {value:.6f}s</title></polygon>')
        lines.append(f'<text x="{width - right}" y="{cy + 4:.2f}" text-anchor="end">p50 {median:.6f}s p95 {p95:.6f}s</text>')
    lines.append("</svg>")
    return "\n".join(lines)


def markdown_report(result: dict[str, Any]) -> str:
    environment = result["environment"]
    screenfs = result["screenfs"]
    git_dirty_status = summarize_git_status(environment.get("git_status_porcelain"))

    lines = [
        "# ScreenFS benchmark result",
        "",
        f"- timestamp: `{result['timestamp']}`",
        f"- harness_command_line: `{result['harness']['command_line']}`",
        f"- git: `{environment.get('git_revision') or 'unknown'}`",
        f"- git_worktree_clean: `{environment['git_worktree_clean']}`",
        f"- screenfs_bin: `{screenfs['binary']}`",
        f"- screenfs_bin_sha256: `{screenfs['binary_sha256']}`",
        f"- iterations: `{result['parameters']['iterations']}`, warmups: `{result['parameters']['warmups']}`",
        "",
        "## Comparable workloads",
        "",
        "| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    if git_dirty_status is not None:
        lines.insert(5, f"- git_dirty_status: `{git_dirty_status}`")

    comparable = result["comparisons"]
    for name, comparison in comparable.items():
        lines.append(
            "| {name} | {native:.6f} | {mounted:.6f} | {ratio:.3f} | {p90:.6f} | {p95:.6f} | {p99:.6f} |".format(
                name=name,
                native=comparison["native_median_sec"],
                mounted=comparison["mounted_median_sec"],
                ratio=comparison["mounted_over_native_median"],
                p90=comparison["mounted_p90_sec"],
                p95=comparison["mounted_p95_sec"],
                p99=comparison["mounted_p99_sec"],
            )
        )
    lines.extend(["", "## ScreenFS-only workloads", "", "| workload | mounted median s | mounted p90 s | mounted p95 s | mounted p99 s |", "| --- | ---: | ---: | ---: | ---: |"])
    for item in result["screenfs_only"]:
        summary = item["summary"]
        lines.append(f"| {item['name']} | {summary['median_sec']:.6f} | {summary['p90_sec']:.6f} | {summary['p95_sec']:.6f} | {summary['p99_sec']:.6f} |")
    perf_summary = screenfs.get("perf_summary")
    lines.extend(["", "## Perf counters", ""])
    if perf_summary:
        lines.extend(
            [
                "Perf counters were enabled and captured from ScreenFS stderr after unmount/termination.",
                "",
                "```text",
                perf_summary.get("raw", ""),
                "```",
            ]
        )
    else:
        lines.append(
            "Perf counters were not enabled or no `screenfs perf counters:` summary was captured."
        )

    lines.extend(
        [
            "",
            "## Notes",
            "",
            "- Fixture creation, build time, and mount startup are not included in workload timings.",
            "- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration.",
            "- Interpret results as warm-cache local evidence unless the run environment records separate cache-control steps.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> int:
    args = parse_args()
    for name in [
        "iterations",
        "warmups",
        "read_mib",
        "write_mib",
        "small_io_bytes",
        "small_io_ops",
        "sync_bytes",
        "sync_ops",
        "small_files",
        "dir_entries",
        "hidden_misses",
        "symlink_parent_mutations",
    ]:
        require_positive(name, getattr(args, name))

    reject_perf_config_conflict(args)

    if hasattr(os, "geteuid") and os.geteuid() == 0:
        raise SystemExit("do not run this benchmark as root or with sudo; ScreenFS evidence must be non-root FUSE evidence")

    if args.build:
        build_command = ["cargo", "build", "--release"]
        if args.perf_counters:
            build_command.extend(["--features", "perf-counters"])
        print("building release binary...", file=sys.stderr)
        run_checked(build_command)

    screenfs_bin = Path(args.screenfs_bin)
    if not screenfs_bin.exists():
        raise SystemExit(f"screenfs binary not found: {screenfs_bin}; run with --build or pass --screenfs-bin")
    screenfs_binary_sha256 = sha256_file(screenfs_bin)
    if shutil.which("fusermount3") is None:
        raise SystemExit("fusermount3 not found; ScreenFS benchmark requires non-root FUSE3 cleanup")

    workdir = Path(tempfile.mkdtemp(prefix="screenfs-bench-"))
    source = workdir / "source"
    mount = workdir / "mount"
    source.mkdir()
    mount.mkdir()
    prepare_fixture(source, args)

    stderr_path = workdir / "screenfs.stderr.log"
    stderr_file = stderr_path.open("w", encoding="utf-8")
    screenfs_prefix_args: list[str] = []
    perf_config_path: Path | None = None
    if args.perf_counters:
        perf_config_path = workdir / "screenfs-perf.yaml"
        perf_config_path.write_text("perf:\n  enabled: true\n", encoding="utf-8")
        screenfs_prefix_args.extend(["--config", str(perf_config_path)])
    command = [
        str(screenfs_bin),
        str(source),
        str(mount),
        *screenfs_prefix_args,
        "--visibility-default",
        "visible",
        "--hidden",
        "/.screenfs-bench/hidden",
        "--mutability-default",
        "writable",
        "--readonly",
        "/.screenfs-bench/readonly",
        *args.extra_screenfs_arg,
    ]

    proc: subprocess.Popen[str] | None = None
    unmount_result: dict[str, Any] = {"ok": False, "attempts": []}
    try:
        proc = subprocess.Popen(command, text=True, stdout=subprocess.PIPE, stderr=stderr_file)
        wait_for_mount(mount, proc, args.timeout_sec)

        native_results = [measure_workload(name, func, source, args, "native") for name, func in WORKLOADS.items()]
        mounted_results = [measure_workload(name, func, mount, args, "mounted") for name, func in WORKLOADS.items()]
        screenfs_only = [
            measure_workload(name, func, mount, args, "mounted")
            for name, func in SCREENFS_ONLY_WORKLOADS.items()
        ]

        mounted_filesystem = path_fs_info(mount)
        mounted_by_name = {item["name"]: item for item in mounted_results}
        comparisons = {}
        for native in native_results:
            mounted_item = mounted_by_name[native["name"]]
            native_median = native["summary"]["median_sec"]
            mounted_median = mounted_item["summary"]["median_sec"]
            comparisons[native["name"]] = {
                "native_median_sec": native_median,
                "mounted_median_sec": mounted_median,
                "mounted_over_native_median": mounted_median / native_median if native_median else None,
                "native_p50_sec": native["summary"]["p50_sec"],
                "mounted_p50_sec": mounted_item["summary"]["p50_sec"],
                "native_p90_sec": native["summary"]["p90_sec"],
                "mounted_p90_sec": mounted_item["summary"]["p90_sec"],
                "native_p95_sec": native["summary"]["p95_sec"],
                "mounted_p95_sec": mounted_item["summary"]["p95_sec"],
                "native_p99_sec": native["summary"]["p99_sec"],
                "mounted_p99_sec": mounted_item["summary"]["p99_sec"],
            }

        unmount_result = unmount(mount)
        if proc.poll() is None:
            try:
                proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                proc.send_signal(signal.SIGTERM)
                try:
                    proc.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait(timeout=3)
        stderr_file.close()
        proc = None

        try:
            screenfs_stderr = stderr_path.read_text(encoding="utf-8", errors="replace")
        except OSError:
            screenfs_stderr = ""
        perf_summary = parse_perf_summary(screenfs_stderr)
        if args.perf_counters and perf_summary is None:
            raise RuntimeError("--perf-counters was enabled but no 'screenfs perf counters:' summary was captured from ScreenFS stderr")

        git_status_porcelain = command_output(["git", "status", "--porcelain"])
        harness_command = harness_argv()
        result = {
            "schema": "screenfs-benchmark-v1",
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "harness": {
                "argv": harness_command,
                "command_line": shell_join(harness_command),
            },
            "environment": {
                "platform": platform.platform(),
                "python": sys.version.split()[0],
                "uname": " ".join(platform.uname()),
                "git_revision": command_output(["git", "rev-parse", "HEAD"]),
                "git_status_porcelain": git_status_porcelain,
                "git_worktree_clean": git_status_porcelain == "",
                "source_filesystem": path_fs_info(source),
                "mount_filesystem": mounted_filesystem,
                "rustc": command_output(["rustc", "--version"]),
                "cargo": command_output(["cargo", "--version"]),
                "fusermount3": command_output(["fusermount3", "--version"]),
            },
            "parameters": {
                "iterations": args.iterations,
                "warmups": args.warmups,
                "read_mib": args.read_mib,
                "write_mib": args.write_mib,
                "small_io_bytes": args.small_io_bytes,
                "small_io_ops": args.small_io_ops,
                "sync_bytes": args.sync_bytes,
                "sync_ops": args.sync_ops,
                "small_files": args.small_files,
                "dir_entries": args.dir_entries,
                "hidden_misses": args.hidden_misses,
                "symlink_parent_mutations": args.symlink_parent_mutations,
            },
            "paths": {"workdir": str(workdir), "source": str(source), "mount": str(mount)},
            "screenfs": {
                "binary": str(screenfs_bin),
                "binary_sha256": screenfs_binary_sha256,
                "command": command,
                "stderr_log": str(stderr_path),
                "stderr_preview": screenfs_stderr[-4000:],
                "perf_counters_enabled": args.perf_counters,
                "perf_config": str(perf_config_path) if perf_config_path else None,
                "perf_summary": perf_summary,
            },
            "native": native_results,
            "mounted": mounted_results,
            "screenfs_only": screenfs_only,
            "comparisons": comparisons,
        }

        if args.output_json:
            args.output_json.parent.mkdir(parents=True, exist_ok=True)
            args.output_json.write_text(json.dumps(result, indent=2, sort_keys=True), encoding="utf-8")
        else:
            print(json.dumps(result, indent=2, sort_keys=True))
        if args.output_md:
            args.output_md.parent.mkdir(parents=True, exist_ok=True)
            args.output_md.write_text(markdown_report(result), encoding="utf-8")
        if args.output_svg:
            args.output_svg.parent.mkdir(parents=True, exist_ok=True)
            args.output_svg.write_text(render_box_plot_svg(result), encoding="utf-8")
        return 0
    finally:
        if proc is not None:
            unmount_result = unmount(mount)
            if proc.poll() is None:
                try:
                    proc.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    proc.send_signal(signal.SIGTERM)
                    try:
                        proc.wait(timeout=3)
                    except subprocess.TimeoutExpired:
                        proc.kill()
                        proc.wait(timeout=3)
        if not stderr_file.closed:
            stderr_file.close()
        if not unmount_result.get("ok"):
            print(f"warning: unmount did not report success: {unmount_result}", file=sys.stderr)
        if args.keep_workdir or not unmount_result.get("ok"):
            print(f"kept benchmark workdir: {workdir}", file=sys.stderr)
        else:
            shutil.rmtree(workdir, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(main())
