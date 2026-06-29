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
import random
import shutil
from collections.abc import Callable as AbcCallable
import shlex
import signal
import stat
import statistics
import subprocess
import sys
import tempfile
import threading
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable


MiB = 1024 * 1024
RANDOM_4K_BLOCK_SIZE = 4096
POLICY_OPTION_FLAGS = frozenset(
    {
        "--visibility-default",
        "--hidden",
        "--visible",
        "--mutability-default",
        "--readonly",
        "--writable",
    }
)
POLICY_SHAPING_EXTRA_ARG_FLAGS = frozenset({*POLICY_OPTION_FLAGS, "--config"})
HIDDEN_STAT_MISS_TARGET = "/.screenfs-bench/hidden"
CACHE_CONTROL_CHOICES = ("warm", "posix-fadvise-read-fixture")
CACHE_CONTROL_SCOPES = {
    "warm": "no explicit cache eviction; existing warm-cache behavior",
    "posix-fadvise-read-fixture": "requested non-root read-side cold-cache approximation for benchmark-owned source/.screenfs-bench read fixtures",
}
READ_SIDE_CACHE_CONTROL_WORKLOADS = frozenset({"seq_read", "small_read", "rand_read_4k", "concurrent_rand_read_write_4k"})
POLICY_PRESETS: dict[str, dict[str, Any]] = {
    "fallback-unsafe-policy": {
        "bucket": "fallback-unsafe-policy",
        "description": "Historical default visible/writable policy with hidden and readonly carve-outs.",
        "screenfs_args": [
            "--visibility-default",
            "visible",
            "--hidden",
            "/.screenfs-bench/hidden",
            "--mutability-default",
            "writable",
            "--readonly",
            "/.screenfs-bench/readonly",
        ],
    },
    "fast-path-cache-eligible": {
        "bucket": "fast-path-cache-eligible",
        "description": "Default visible/writable policy with no hidden or readonly carve-outs.",
        "screenfs_args": [
            "--visibility-default",
            "visible",
            "--mutability-default",
            "writable",
        ],
    },
}
DEFAULT_COMPARABLE_WORKLOADS = [
    "seq_read",
    "seq_write",
    "small_read",
    "small_write",
    "write_fsync_close",
    "small_stat_open_read",
    "readdir_lstat",
    "symlink_open_read",
]
PER_OPEN_CACHE_MINIMUM_COMPARABLE_WORKLOADS = [
    "rand_read_4k",
    "rand_write_4k",
    "sync_write_4k",
    "small_open_read_close",
]
READ_WRITE_SURFACE_COMPARABLE_WORKLOADS = [
    "seq_read",
    "seq_write",
    "small_read",
    "small_write",
    "rand_read_4k",
    "rand_write_4k",
]
READ_WRITE_CONCURRENCY_COMPARABLE_WORKLOADS = [
    "concurrent_rand_read_write_4k",
]
METADATA_OPEN_PATH_COMPARABLE_WORKLOADS = [
    "metadata_lookup",
    "metadata_getattr",
    "metadata_open",
    "metadata_readlink",
    "metadata_access",
    "metadata_statfs",
]
OPEN_CONFINED_SURFACE_COMPARABLE_WORKLOADS = [
    "metadata_open",
    "metadata_opendir",
]
SYNC_SURFACE_COMPARABLE_WORKLOADS = [
    "sync_flush_only",
    "sync_fsync_only",
    "sync_release_flush",
]
READ_ONLY_CLOSE_SURFACE_COMPARABLE_WORKLOADS = [
    "read_only_open_close",
    "read_only_open_read_close",
    "write_open_write_close",
    "write_open_fsync_close",
]
DIRECTORY_SURFACE_COMPARABLE_WORKLOADS = [
    "readdir_basic",
    "readdirplus_basic",
]
DIRECTORY_SYMLINK_SURFACE_COMPARABLE_WORKLOADS = [
    "readdir_symlink_visibility",
    "readdirplus_symlink_visibility",
]
MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS = [
    "matcher_descendant_readdir",
    "matcher_descendant_readdirplus",
]
POLICY_HEAVY_COMPARABLE_WORKLOADS = [
    "metadata_lookup",
    "metadata_getattr",
    "metadata_access",
]
POLICY_HEAVY_SCREENFS_ONLY_WORKLOADS = [
    "matcher_hidden_stat_miss",
    "matcher_readonly_access_wok",
]
DEFAULT_SCREENFS_ONLY_WORKLOADS = [
    "hidden_stat_miss",
    "matcher_hidden_stat_miss",
    "symlink_parent_mkdir_rmdir",
]
MUTATION_INVALIDATION_SCREENFS_ONLY_WORKLOADS = [
    "symlink_parent_mkdir_rmdir",
    "pinned_symlink_parent_mkdir_rmdir",
    "subtree_rename_cached_unrelated",
]
WORKLOAD_SETS: dict[str, dict[str, list[str]]] = {
    "default": {
        "comparable": list(DEFAULT_COMPARABLE_WORKLOADS),
        "screenfs_only": list(DEFAULT_SCREENFS_ONLY_WORKLOADS),
    },
    "per-open-cache-minimum": {
        "comparable": list(PER_OPEN_CACHE_MINIMUM_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "read-write-surface": {
        "comparable": list(READ_WRITE_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "read-write-concurrency": {
        "comparable": list(READ_WRITE_CONCURRENCY_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "metadata-open-path": {
        "comparable": list(METADATA_OPEN_PATH_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "open-confined-surface": {
        "comparable": list(OPEN_CONFINED_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "sync-surface": {
        "comparable": list(SYNC_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "read-only-close-surface": {
        "comparable": list(READ_ONLY_CLOSE_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "directory-surface": {
        "comparable": list(DIRECTORY_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "directory-symlink-surface": {
        "comparable": list(DIRECTORY_SYMLINK_SURFACE_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "matcher-descendant-directory": {
        "comparable": list(MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS),
        "screenfs_only": [],
    },
    "policy-heavy-matrix": {
        "comparable": list(POLICY_HEAVY_COMPARABLE_WORKLOADS),
        "screenfs_only": list(POLICY_HEAVY_SCREENFS_ONLY_WORKLOADS),
    },
    "mutation-invalidation": {
        "comparable": [],
        "screenfs_only": list(MUTATION_INVALIDATION_SCREENFS_ONLY_WORKLOADS),
    },
    "all": {
        "comparable": list(
            dict.fromkeys(
                DEFAULT_COMPARABLE_WORKLOADS
                + PER_OPEN_CACHE_MINIMUM_COMPARABLE_WORKLOADS
                + READ_WRITE_SURFACE_COMPARABLE_WORKLOADS
                + READ_WRITE_CONCURRENCY_COMPARABLE_WORKLOADS
                + METADATA_OPEN_PATH_COMPARABLE_WORKLOADS
                + OPEN_CONFINED_SURFACE_COMPARABLE_WORKLOADS
                + SYNC_SURFACE_COMPARABLE_WORKLOADS
                + READ_ONLY_CLOSE_SURFACE_COMPARABLE_WORKLOADS
                + DIRECTORY_SURFACE_COMPARABLE_WORKLOADS
                + DIRECTORY_SYMLINK_SURFACE_COMPARABLE_WORKLOADS
                + MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS
            )
        ),
        "screenfs_only": list(DEFAULT_SCREENFS_ONLY_WORKLOADS),
    },
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--screenfs-bin",
        default="target/release/screenfs",
        help="Path to a prebuilt screenfs binary (default: target/release/screenfs).",
    )
    parser.add_argument(
        "--screenfs-source-root",
        type=Path,
        help="Optional ScreenFS source tree root for provenance when --screenfs-bin points at another checkout. If omitted, the harness tries to infer the source root from the binary path.",
    )
    parser.add_argument(
        "--build",
        action="store_true",
        help="Run cargo build --release before benchmarking. Build time is not measured.",
    )
    parser.add_argument("--iterations", type=int, default=10, help="Measured iterations per workload.")
    parser.add_argument("--warmups", type=int, default=3, help="Warmup iterations per workload.")
    parser.add_argument(
        "--cache-control",
        choices=CACHE_CONTROL_CHOICES,
        default="warm",
        help="Cache handling mode for warmups/measured samples (default: warm). posix-fadvise-read-fixture is a non-root read-side cold-cache approximation, not a strict cold-cache guarantee.",
    )
    parser.add_argument("--read-mib", type=int, default=64, help="Sequential read fixture size in MiB.")
    parser.add_argument("--write-mib", type=int, default=64, help="Sequential write size per iteration in MiB.")
    parser.add_argument(
        "--small-io-bytes",
        type=int,
        default=4096,
        help="Bytes per operation for small-buffer read/write workloads.",
    )
    parser.add_argument(
        "--small-io-ops",
        type=int,
        default=1024,
        help="Small-buffer read/write operations per iteration.",
    )
    parser.add_argument("--sync-bytes", type=int, default=4096, help="Bytes written per fsync workload operation.")
    parser.add_argument("--sync-ops", type=int, default=128, help="Open/write/fsync/close operations per iteration.")
    parser.add_argument("--small-files", type=int, default=2000, help="Small files for stat/open/read workload.")
    parser.add_argument("--dir-entries", type=int, default=5000, help="Directory entries for listing workload.")
    parser.add_argument(
        "--rand-io-ops",
        type=int,
        default=16384,
        help="4KiB random read/write operations per iteration for rand_*_4k workloads.",
    )
    parser.add_argument(
        "--concurrency-workers",
        type=int,
        default=4,
        help="Worker threads per iteration for concurrent_rand_read_write_4k (default: 4).",
    )
    parser.add_argument(
        "--open-read-close-ops",
        type=int,
        default=4096,
        help="Open/read/close operations per iteration for the small_open_read_close workload.",
    )
    parser.add_argument(
        "--metadata-ops",
        type=int,
        default=512,
        help="Metadata operations per iteration for metadata_* workloads. Keep this below the process file-descriptor limit for metadata_open and metadata_opendir.",
    )
    parser.add_argument(
        "--sync-4k-fsync-every",
        type=int,
        default=32,
        help="Call fsync after this many 4KiB writes in the sync_write_4k workload.",
    )
    parser.add_argument(
        "--hidden-misses",
        type=int,
        default=2000,
        help="Repeated hidden-path ENOENT checks for the ScreenFS-only workload.",
    )
    parser.add_argument(
        "--matcher-extra-rules",
        type=int,
        default=0,
        help="Add synthetic hidden/readonly rules plus bridge-visible descendant fixture/rules for matcher-heavy attribution experiments.",
    )
    parser.add_argument(
        "--matcher-misses",
        type=int,
        default=2000,
        help="Repeated hidden-path ENOENT checks for the matcher-heavy ScreenFS-only workload.",
    )
    parser.add_argument(
        "--symlink-parent-mutations",
        type=int,
        default=2000,
        help="Repeated mkdir/rmdir pairs under a visible symlink parent for the ScreenFS-only workload.",
    )
    parser.add_argument(
        "--policy-preset",
        choices=sorted(POLICY_PRESETS),
        default="fallback-unsafe-policy",
        help="Built-in ScreenFS policy preset. Default preserves the historical hidden/readonly benchmark policy.",
    )
    parser.add_argument(
        "--policy-label",
        help="Optional provenance label for the selected policy/matrix entry.",
    )
    parser.add_argument(
        "--workload-set",
        choices=sorted(WORKLOAD_SETS),
        default="default",
        help="Named workload set to run when --workload is not specified.",
    )
    parser.add_argument(
        "--workload",
        action="append",
        default=[],
        help="Run only the named workload(s); repeat as needed. Overrides --workload-set.",
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
        help="Build/use a ScreenFS binary compiled with --features perf-counters and record the stderr summary in JSON/Markdown artifacts.",
    )
    return parser.parse_args()



def require_positive(name: str, value: int) -> None:
    if value <= 0:
        raise SystemExit(f"{name} must be positive")



def ordered_unique(values: list[str]) -> list[str]:
    seen: set[str] = set()
    ordered: list[str] = []
    for value in values:
        if value in seen:
            continue
        seen.add(value)
        ordered.append(value)
    return ordered



def collect_option_values(argv: list[str], option: str) -> list[str]:
    values: list[str] = []
    for index, token in enumerate(argv[:-1]):
        if token == option:
            values.append(argv[index + 1])
    return values



def path_rule_matches_or_contains(rule: str, target: str) -> bool:
    normalized_rule = rule.rstrip("/") or "/"
    normalized_target = target.rstrip("/") or "/"
    if normalized_rule == "/":
        return True
    return normalized_rule == normalized_target or normalized_target.startswith(f"{normalized_rule}/")



def hidden_paths_support_target(hidden_paths: list[str], target: str) -> bool:
    return any(path_rule_matches_or_contains(path, target) for path in hidden_paths)



def run_checked(command: list[str], **kwargs: Any) -> subprocess.CompletedProcess[str]:
    return subprocess.run(command, check=True, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, **kwargs)



def command_output(command: list[str], cwd: Path | None = None) -> str | None:
    try:
        return run_checked(command, cwd=cwd).stdout.strip()
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



def infer_source_root_from_binary(binary: Path) -> Path | None:
    try:
        resolved = binary.resolve()
    except OSError:
        resolved = binary
    for candidate in resolved.parents:
        if (candidate / "Cargo.toml").is_file():
            return candidate
    for candidate in resolved.parents:
        if (candidate / ".git").exists():
            return candidate
    return None



def git_provenance(root: Path | None) -> dict[str, Any]:
    if root is None:
        return {
            "source_root": None,
            "git_revision": None,
            "git_status_porcelain": None,
            "git_worktree_clean": None,
        }
    git_status_porcelain = command_output(["git", "status", "--porcelain"], cwd=root)
    return {
        "source_root": str(root),
        "git_revision": command_output(["git", "rev-parse", "HEAD"], cwd=root),
        "git_status_porcelain": git_status_porcelain,
        "git_worktree_clean": None if git_status_porcelain is None else git_status_porcelain == "",
    }



def resolve_screenfs_source_provenance(screenfs_bin: Path, explicit_root: Path | None) -> dict[str, Any]:
    if explicit_root is not None:
        source_root = explicit_root
        source_root_origin = "cli"
    else:
        source_root = infer_source_root_from_binary(screenfs_bin)
        source_root_origin = "inferred-from-screenfs-bin" if source_root is not None else None
    provenance = git_provenance(source_root)
    provenance["source_root_origin"] = source_root_origin
    return provenance



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
    read_path = root / "read" / "seq.bin"
    write_bytes(read_path, args.read_mib * MiB, chunk)

    concurrency_read_dir = root / "concurrency-read"
    concurrency_read_dir.mkdir()
    for index in range(args.concurrency_workers):
        worker_path = concurrency_read_dir / f"worker-{index:06d}.bin"
        try:
            os.link(read_path, worker_path)
        except OSError:
            shutil.copyfile(read_path, worker_path)

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
    for index in range(args.dir_entries):
        os.symlink("target.txt", symlink_dir / f"link-{index:06d}.txt")

    if args.matcher_extra_rules:
        matcher_dir = root / "matcher-heavy"
        matcher_dir.mkdir()
        descendant_root = matcher_dir / "descendant-root"
        descendant_root.mkdir()
        for index in range(args.matcher_extra_rules):
            hidden_bucket = matcher_dir / f"hidden-{index:04d}"
            hidden_bucket.mkdir()
            (hidden_bucket / "secret.txt").write_text("hidden matcher payload\n", encoding="utf-8")
            visible_bucket = matcher_dir / f"visible-{index:04d}"
            visible_bucket.mkdir()
            (visible_bucket / f"readonly-{index:04d}.txt").write_text("readonly matcher payload\n", encoding="utf-8")
            branch_dir = descendant_root / f"branch-{index:04d}"
            branch_dir.mkdir()
            (branch_dir / f"leaf-{index:04d}.txt").write_text("visible descendant matcher payload\n", encoding="utf-8")

    symlink_parent_dir = root / "symlink-parent"
    symlink_parent_dir.mkdir()
    (symlink_parent_dir / "real").mkdir()
    (symlink_parent_dir / "real" / "pinned-sibling.txt").write_text("pinned sibling\n", encoding="utf-8")
    os.symlink("real", symlink_parent_dir / "alias")

    invalidation_dir = root / "invalidation-tree"
    invalidation_dir.mkdir()
    source_dir = invalidation_dir / "source"
    source_dir.mkdir()
    (source_dir / "child.txt").write_text("subtree child\n", encoding="utf-8")
    unrelated_dir = invalidation_dir / "unrelated"
    unrelated_dir.mkdir()
    for index in range(args.small_files):
        (unrelated_dir / f"cached-{index:06d}.txt").write_text("unrelated\n", encoding="utf-8")

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



def seq_write(root: Path, args: argparse.Namespace, side: str) -> AbcCallable[[], None]:
    path = write_output_path(root, side, "seq-write.bin")
    chunk = b"w" * MiB
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
        try:
            for _ in range(args.write_mib):
                write_all_fd(fd, chunk)
        finally:
            os.close(fd)
    except Exception:
        path.unlink(missing_ok=True)
        raise

    def cleanup() -> None:
        path.unlink(missing_ok=True)

    return cleanup



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



def small_write(root: Path, args: argparse.Namespace, side: str) -> AbcCallable[[], None]:
    path = write_output_path(root, side, "small-write.bin")
    chunk = b"s" * args.small_io_bytes
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
        try:
            for _ in range(args.small_io_ops):
                write_all_fd(fd, chunk)
        finally:
            os.close(fd)
    except Exception:
        path.unlink(missing_ok=True)
        raise

    def cleanup() -> None:
        path.unlink(missing_ok=True)

    return cleanup



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



def rand_read_4k(root: Path, args: argparse.Namespace, side: str) -> None:
    path = root / ".screenfs-bench" / "read" / "seq.bin"
    size = path.stat().st_size
    if size < RANDOM_4K_BLOCK_SIZE:
        raise RuntimeError(f"{side} rand_read_4k fixture is smaller than {RANDOM_4K_BLOCK_SIZE} bytes")
    block_count = size // RANDOM_4K_BLOCK_SIZE
    rng = random.Random(0)
    total = 0
    fd = os.open(path, os.O_RDONLY | os.O_CLOEXEC)
    try:
        for _ in range(args.rand_io_ops):
            offset = rng.randrange(block_count) * RANDOM_4K_BLOCK_SIZE
            data = os.pread(fd, RANDOM_4K_BLOCK_SIZE, offset)
            if len(data) != RANDOM_4K_BLOCK_SIZE:
                raise RuntimeError(f"{side} rand_read_4k short read at offset {offset}: {len(data)}")
            total += len(data)
    finally:
        os.close(fd)
    if total == 0:
        raise RuntimeError(f"{side} rand_read_4k read no data")



def aligned_random_io_size(size_mib: int) -> int:
    return max(RANDOM_4K_BLOCK_SIZE, (size_mib * MiB // RANDOM_4K_BLOCK_SIZE) * RANDOM_4K_BLOCK_SIZE)



def rand_write_4k(root: Path, args: argparse.Namespace, side: str) -> AbcCallable[[], None]:
    path = write_output_path(root, side, "rand-write-4k.bin")
    size = aligned_random_io_size(args.write_mib)
    block_count = max(1, size // RANDOM_4K_BLOCK_SIZE)
    chunk = b"r" * RANDOM_4K_BLOCK_SIZE
    rng = random.Random(0)
    try:
        fd = os.open(path, os.O_RDWR | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
        try:
            os.ftruncate(fd, size)
            for _ in range(args.rand_io_ops):
                offset = rng.randrange(block_count) * RANDOM_4K_BLOCK_SIZE
                written = os.pwrite(fd, chunk, offset)
                if written != RANDOM_4K_BLOCK_SIZE:
                    raise RuntimeError(f"{side} rand_write_4k short write at offset {offset}: {written}")
        finally:
            os.close(fd)
    except Exception:
        path.unlink(missing_ok=True)
        raise

    def cleanup() -> None:
        path.unlink(missing_ok=True)

    return cleanup



def concurrency_read_path(root: Path, worker_index: int) -> Path:
    return root / ".screenfs-bench" / "concurrency-read" / f"worker-{worker_index:06d}.bin"



def concurrency_write_path(root: Path, side: str, worker_index: int) -> Path:
    return write_output_path(root, side, f"concurrent-rand-read-write-4k-worker-{worker_index:06d}.bin")



def concurrent_rand_read_write_4k(root: Path, args: argparse.Namespace, side: str) -> AbcCallable[[], None]:
    read_block_count = max(1, aligned_random_io_size(args.read_mib) // RANDOM_4K_BLOCK_SIZE)
    write_size = aligned_random_io_size(args.write_mib)
    write_block_count = max(1, write_size // RANDOM_4K_BLOCK_SIZE)
    barrier = threading.Barrier(args.concurrency_workers + 1)
    write_paths = [concurrency_write_path(root, side, index) for index in range(args.concurrency_workers)]
    errors: list[BaseException] = []

    def worker(worker_index: int, write_path: Path) -> None:
        read_path = concurrency_read_path(root, worker_index)
        chunk = bytes((worker_index + offset) % 251 for offset in range(RANDOM_4K_BLOCK_SIZE))
        rng = random.Random(worker_index)
        total = 0
        read_fd: int | None = None
        write_fd: int | None = None
        try:
            read_fd = os.open(read_path, os.O_RDONLY | os.O_CLOEXEC)
            write_fd = os.open(write_path, os.O_RDWR | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
            os.ftruncate(write_fd, write_size)
            barrier.wait()
            for _ in range(args.rand_io_ops):
                read_offset = rng.randrange(read_block_count) * RANDOM_4K_BLOCK_SIZE
                data = os.pread(read_fd, RANDOM_4K_BLOCK_SIZE, read_offset)
                if len(data) != RANDOM_4K_BLOCK_SIZE:
                    raise RuntimeError(
                        f"{side} concurrent_rand_read_write_4k short read for worker {worker_index} at offset {read_offset}: {len(data)}"
                    )
                total += len(data)
                write_offset = rng.randrange(write_block_count) * RANDOM_4K_BLOCK_SIZE
                written = os.pwrite(write_fd, chunk, write_offset)
                if written != RANDOM_4K_BLOCK_SIZE:
                    raise RuntimeError(
                        f"{side} concurrent_rand_read_write_4k short write for worker {worker_index} at offset {write_offset}: {written}"
                    )
            if total == 0:
                raise RuntimeError(f"{side} concurrent_rand_read_write_4k worker {worker_index} transferred no data")
        except BaseException as exc:
            errors.append(exc)
            barrier.abort()
        finally:
            if write_fd is not None:
                os.close(write_fd)
            if read_fd is not None:
                os.close(read_fd)

    threads = [
        threading.Thread(target=worker, args=(worker_index, write_path), name=f"screenfs-bench-concurrency-{worker_index}")
        for worker_index, write_path in enumerate(write_paths)
    ]
    try:
        for thread in threads:
            thread.start()
        try:
            barrier.wait()
        except threading.BrokenBarrierError:
            pass
        for thread in threads:
            thread.join()
        if errors:
            raise RuntimeError(str(errors[0])) from errors[0]
    except Exception:
        for write_path in write_paths:
            write_path.unlink(missing_ok=True)
        raise

    def cleanup() -> None:
        for write_path in write_paths:
            write_path.unlink(missing_ok=True)

    return cleanup



def sync_write_4k(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "sync-write-4k.bin")
    chunk = b"y" * RANDOM_4K_BLOCK_SIZE
    writes_since_fsync = 0
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
    try:
        for _ in range(args.sync_ops):
            write_all_fd(fd, chunk)
            writes_since_fsync += 1
            if writes_since_fsync >= args.sync_4k_fsync_every:
                os.fsync(fd)
                writes_since_fsync = 0
        if writes_since_fsync:
            os.fsync(fd)
    finally:
        os.close(fd)
    path.unlink(missing_ok=True)



def sync_flush_only(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "sync-flush-only.bin")
    chunk = b"q" * args.sync_bytes
    try:
        for _ in range(args.sync_ops):
            with path.open("wb", buffering=0) as file:
                file.write(chunk)
                file.flush()
    finally:
        path.unlink(missing_ok=True)



def sync_fsync_only(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "sync-fsync-only.bin")
    chunk = b"z" * args.sync_bytes
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
    try:
        for _ in range(args.sync_ops):
            write_all_fd(fd, chunk)
            os.fsync(fd)
    finally:
        os.close(fd)
        path.unlink(missing_ok=True)



def sync_release_flush(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "sync-release-flush.bin")
    chunk = b"c" * args.sync_bytes
    try:
        for _ in range(args.sync_ops):
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
            try:
                write_all_fd(fd, chunk)
            finally:
                os.close(fd)
    finally:
        path.unlink(missing_ok=True)



def read_only_open_close(root: Path, args: argparse.Namespace, _side: str) -> None:
    for index in range(args.open_read_close_ops):
        fd = os.open(metadata_entry(root, index % args.small_files), os.O_RDONLY | os.O_CLOEXEC)
        os.close(fd)



def read_only_open_read_close(root: Path, args: argparse.Namespace, _side: str) -> None:
    total = 0
    for index in range(args.open_read_close_ops):
        fd = os.open(metadata_entry(root, index % args.small_files), os.O_RDONLY | os.O_CLOEXEC)
        try:
            total += len(os.read(fd, 32))
        finally:
            os.close(fd)
    if total == 0:
        raise RuntimeError("read_only_open_read_close read no data")



def write_open_write_close(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "write-open-write-close.bin")
    payload = b"w" * min(args.sync_bytes, 4096)
    try:
        for _ in range(args.sync_ops):
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
            try:
                write_all_fd(fd, payload)
            finally:
                os.close(fd)
    finally:
        path.unlink(missing_ok=True)



def write_open_fsync_close(root: Path, args: argparse.Namespace, side: str) -> None:
    path = write_output_path(root, side, "write-open-fsync-close.bin")
    payload = b"f" * min(args.sync_bytes, 4096)
    try:
        for _ in range(args.sync_ops):
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_CLOEXEC, 0o600)
            try:
                write_all_fd(fd, payload)
                os.fsync(fd)
            finally:
                os.close(fd)
    finally:
        path.unlink(missing_ok=True)



def small_open_read_close(root: Path, args: argparse.Namespace, _side: str) -> None:
    small_dir = root / ".screenfs-bench" / "small-files"
    total = 0
    for index in range(args.open_read_close_ops):
        entry = small_dir / f"file-{index % args.small_files:06d}.txt"
        with entry.open("rb", buffering=0) as file:
            total += len(file.read(32))
    if total == 0:
        raise RuntimeError("small_open_read_close read no data")



def metadata_entry(root: Path, index: int) -> Path:
    return root / ".screenfs-bench" / "small-files" / f"file-{index:06d}.txt"



def metadata_lookup(root: Path, args: argparse.Namespace, side: str) -> None:
    small_dir = root / ".screenfs-bench" / "small-files"
    hits = 0
    for index in range(args.metadata_ops):
        if (small_dir / f"missing-{index:06d}.txt").exists():
            hits += 1
    if hits:
        raise RuntimeError(f"{side} metadata_lookup unexpectedly found {hits} missing entries")



def metadata_getattr(root: Path, args: argparse.Namespace, _side: str) -> None:
    total = 0
    for index in range(args.metadata_ops):
        total += metadata_entry(root, index % args.small_files).stat().st_size
    if total == 0:
        raise RuntimeError("metadata_getattr saw no size data")



def metadata_open(root: Path, args: argparse.Namespace, _side: str) -> AbcCallable[[], None]:
    try:
        import resource

        soft_limit, _hard_limit = resource.getrlimit(resource.RLIMIT_NOFILE)
    except (ImportError, OSError, ValueError):
        soft_limit = 1024
    reserve_fds = 64
    if args.metadata_ops >= max(1, soft_limit - reserve_fds):
        raise RuntimeError(
            f"metadata_open needs one live fd per metadata op to exclude close/release from timing; "
            f"--metadata-ops={args.metadata_ops} is too high for RLIMIT_NOFILE={soft_limit}"
        )
    fds: list[int] = []
    try:
        for index in range(args.metadata_ops):
            fds.append(os.open(metadata_entry(root, index % args.small_files), os.O_RDONLY | os.O_CLOEXEC))
    except Exception:
        for fd in fds:
            os.close(fd)
        raise
    if not fds:
        raise RuntimeError("metadata_open opened no files")

    def cleanup() -> None:
        for fd in fds:
            os.close(fd)

    return cleanup



def metadata_opendir(root: Path, args: argparse.Namespace, _side: str) -> AbcCallable[[], None]:
    try:
        import resource

        soft_limit, _hard_limit = resource.getrlimit(resource.RLIMIT_NOFILE)
    except (ImportError, OSError, ValueError):
        soft_limit = 1024
    reserve_fds = 64
    if args.metadata_ops >= max(1, soft_limit - reserve_fds):
        raise RuntimeError(
            f"metadata_opendir needs one live directory handle per metadata op to exclude close/releasedir from timing; "
            f"--metadata-ops={args.metadata_ops} is too high for RLIMIT_NOFILE={soft_limit}"
        )

    dir_path = root / ".screenfs-bench" / "dir-entries"
    handles = []
    try:
        for _ in range(args.metadata_ops):
            handles.append(os.scandir(dir_path))
    except Exception:
        for handle in handles:
            handle.close()
        raise
    if not handles:
        raise RuntimeError("metadata_opendir opened no directories")

    def cleanup() -> None:
        for handle in handles:
            handle.close()

    return cleanup



def metadata_readlink(root: Path, args: argparse.Namespace, _side: str) -> None:
    link = root / ".screenfs-bench" / "symlinks" / "link.txt"
    total = 0
    for _ in range(args.metadata_ops):
        total += len(os.readlink(link))
    if total == 0:
        raise RuntimeError("metadata_readlink read no link targets")



def metadata_access(root: Path, args: argparse.Namespace, _side: str) -> None:
    accessible = 0
    for index in range(args.metadata_ops):
        if os.access(metadata_entry(root, index % args.small_files), os.R_OK):
            accessible += 1
    if accessible != args.metadata_ops:
        raise RuntimeError(f"metadata_access expected {args.metadata_ops} readable files, saw {accessible}")



def metadata_statfs(root: Path, args: argparse.Namespace, _side: str) -> None:
    total = 0
    for _ in range(args.metadata_ops):
        total += os.statvfs(root).f_bsize
    if total == 0:
        raise RuntimeError("metadata_statfs saw no filesystem block size")



def small_stat_open_read(root: Path, _args: argparse.Namespace, _side: str) -> None:
    small_dir = root / ".screenfs-bench" / "small-files"
    total = 0
    for entry in sorted(small_dir.iterdir()):
        stat = entry.stat()
        with entry.open("rb", buffering=0) as file:
            total += len(file.read(32)) + stat.st_size
    if total == 0:
        raise RuntimeError("small_stat_open_read read no data")



def readdir_basic(root: Path, _args: argparse.Namespace, _side: str) -> None:
    dir_path = root / ".screenfs-bench" / "dir-entries"
    count = 0
    with os.scandir(dir_path) as entries:
        for entry in entries:
            if not entry.name:
                raise RuntimeError("readdir_basic saw an empty entry name")
            count += 1
    if count == 0:
        raise RuntimeError("readdir_basic saw no entries")



def readdirplus_basic(root: Path, _args: argparse.Namespace, _side: str) -> None:
    dir_path = root / ".screenfs-bench" / "dir-entries"
    count = 0
    with os.scandir(dir_path) as entries:
        for entry in entries:
            entry.stat(follow_symlinks=False)
            count += 1
    if count == 0:
        raise RuntimeError("readdirplus_basic saw no entries")



def readdir_lstat(root: Path, _args: argparse.Namespace, _side: str) -> None:
    readdirplus_basic(root, _args, _side)



def matcher_descendant_directory_root(root: Path) -> Path:
    return root / ".screenfs-bench" / "matcher-heavy" / "descendant-root"



def expected_matcher_descendant_directory_names(args: argparse.Namespace) -> set[str]:
    return {f"branch-{index:04d}" for index in range(args.matcher_extra_rules)}



def matcher_descendant_readdir(root: Path, args: argparse.Namespace, _side: str) -> None:
    if args.matcher_extra_rules <= 0:
        raise RuntimeError("matcher_descendant_readdir requires --matcher-extra-rules > 0")
    dir_path = matcher_descendant_directory_root(root)
    names = set()
    with os.scandir(dir_path) as entries:
        for entry in entries:
            if not entry.is_dir(follow_symlinks=False):
                raise RuntimeError(f"matcher_descendant_readdir expected directory entry: {entry.name}")
            names.add(entry.name)
    expected = expected_matcher_descendant_directory_names(args)
    if names != expected:
        raise RuntimeError(f"matcher_descendant_readdir saw unexpected entries: {sorted(names)}")



def matcher_descendant_readdirplus(root: Path, args: argparse.Namespace, _side: str) -> None:
    if args.matcher_extra_rules <= 0:
        raise RuntimeError("matcher_descendant_readdirplus requires --matcher-extra-rules > 0")
    dir_path = matcher_descendant_directory_root(root)
    seen = set()
    with os.scandir(dir_path) as entries:
        for entry in entries:
            if not entry.stat(follow_symlinks=False).st_mode:
                raise RuntimeError(f"matcher_descendant_readdirplus could not stat: {entry.name}")
            if not entry.is_dir(follow_symlinks=False):
                raise RuntimeError(f"matcher_descendant_readdirplus expected directory entry: {entry.name}")
            seen.add(entry.name)
    expected = expected_matcher_descendant_directory_names(args)
    if seen != expected:
        raise RuntimeError(f"matcher_descendant_readdirplus saw unexpected entries: {sorted(seen)}")



def expected_symlink_directory_names(args: argparse.Namespace) -> set[str]:
    return {
        "target.txt",
        "link.txt",
        *(f"link-{index:06d}.txt" for index in range(args.dir_entries)),
    }



def readdir_symlink_visibility(root: Path, args: argparse.Namespace, _side: str) -> None:
    dir_path = root / ".screenfs-bench" / "symlinks"
    names = set()
    with os.scandir(dir_path) as entries:
        for entry in entries:
            names.add(entry.name)
    expected = expected_symlink_directory_names(args)
    if names != expected:
        raise RuntimeError(f"readdir_symlink_visibility saw unexpected entries: {sorted(names)}")



def readdirplus_symlink_visibility(root: Path, args: argparse.Namespace, _side: str) -> None:
    dir_path = root / ".screenfs-bench" / "symlinks"
    seen: dict[str, bool] = {}
    with os.scandir(dir_path) as entries:
        for entry in entries:
            entry.stat(follow_symlinks=False)
            seen[entry.name] = entry.is_symlink()
    expected = expected_symlink_directory_names(args)
    expected_symlinks = expected - {"target.txt"}
    if set(seen) != expected or seen.get("target.txt") is not False or any(not seen.get(name) for name in expected_symlinks):
        raise RuntimeError(f"readdirplus_symlink_visibility saw unexpected entries: {seen}")



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



def matcher_hidden_stat_miss(root: Path, args: argparse.Namespace, side: str) -> None:
    if args.matcher_extra_rules <= 0:
        return
    matcher_dir = root / ".screenfs-bench" / "matcher-heavy"
    for index in range(args.matcher_misses):
        rule_index = index % args.matcher_extra_rules
        path = matcher_dir / f"hidden-{rule_index:04d}" / "secret.txt"
        try:
            path.stat()
        except FileNotFoundError:
            if side == "mounted":
                continue
            raise
        if side == "mounted":
            raise RuntimeError("matcher_hidden_stat_miss expected ENOENT through ScreenFS")



def matcher_readonly_access_wok(root: Path, args: argparse.Namespace, side: str) -> None:
    if args.matcher_extra_rules <= 0:
        return
    matcher_dir = root / ".screenfs-bench" / "matcher-heavy"
    readonly_paths = [
        matcher_dir / f"visible-{index:04d}" / f"readonly-{index:04d}.txt"
        for index in range(args.matcher_extra_rules)
    ]
    for path in readonly_paths:
        if not path.is_file():
            raise RuntimeError(f"matcher_readonly_access_wok missing readonly fixture: {path}")
    for index in range(args.matcher_misses):
        path = readonly_paths[index % args.matcher_extra_rules]
        writable = os.access(path, os.W_OK)
        if side == "mounted":
            if writable:
                raise RuntimeError("matcher_readonly_access_wok expected W_OK denial through ScreenFS")
            continue
        if not writable:
            raise RuntimeError("matcher_readonly_access_wok expected writable native fixture")



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


def pinned_symlink_parent_mkdir_rmdir(root: Path, args: argparse.Namespace, _side: str) -> None:
    fixture_root = root / ".screenfs-bench" / "symlink-parent"
    real_parent = fixture_root / "real"
    alias_parent = fixture_root / "alias"
    if not alias_parent.is_dir():
        raise RuntimeError("pinned_symlink_parent_mkdir_rmdir missing alias parent")

    pinned_sibling = alias_parent / "pinned-sibling.txt"
    created: list[Path] = []
    try:
        if not pinned_sibling.stat().st_size:
            raise RuntimeError("pinned_symlink_parent_mkdir_rmdir expected pinned sibling data")
        with os.scandir(alias_parent) as entries:
            if not any(entry.name == pinned_sibling.name for entry in entries):
                raise RuntimeError("pinned_symlink_parent_mkdir_rmdir did not see pinned sibling")

        for index in range(args.symlink_parent_mutations):
            name = f"pinned-child-{index:06d}"
            alias_child = alias_parent / name
            real_child = real_parent / name
            if alias_child.exists() or real_child.exists():
                raise RuntimeError(f"pinned_symlink_parent_mkdir_rmdir saw unexpected preexisting path: {alias_child}")
            os.mkdir(alias_child, 0o755)
            created.append(real_child)
            if not alias_child.stat().st_mode:
                raise RuntimeError(f"pinned_symlink_parent_mkdir_rmdir could not stat created child: {alias_child}")
            with os.scandir(alias_parent) as entries:
                if not any(entry.name == name for entry in entries):
                    raise RuntimeError(f"pinned_symlink_parent_mkdir_rmdir did not list created child: {alias_child}")
            os.rmdir(alias_child)
            created.pop()
            if real_child.exists():
                raise RuntimeError(f"pinned_symlink_parent_mkdir_rmdir expected {real_child} to be removed")
            if not pinned_sibling.exists():
                raise RuntimeError("pinned_symlink_parent_mkdir_rmdir lost pinned sibling")
    finally:
        for leftover in reversed(created):
            try:
                os.rmdir(leftover)
            except FileNotFoundError:
                pass


def subtree_rename_cached_unrelated(root: Path, args: argparse.Namespace, _side: str) -> Callable[[], None] | None:
    fixture_root = root / ".screenfs-bench" / "invalidation-tree"
    source = fixture_root / "source"
    target = fixture_root / "target"
    unrelated = fixture_root / "unrelated"
    child = source / "child.txt"
    if not source.is_dir() or not child.is_file() or not unrelated.is_dir():
        raise RuntimeError("subtree_rename_cached_unrelated missing fixture")
    if target.exists():
        raise RuntimeError(f"subtree_rename_cached_unrelated saw unexpected target: {target}")

    for index in range(args.small_files):
        unrelated_path = unrelated / f"cached-{index:06d}.txt"
        if not unrelated_path.stat().st_size:
            raise RuntimeError(f"subtree_rename_cached_unrelated expected unrelated data: {unrelated_path}")
    if not child.stat().st_size:
        raise RuntimeError("subtree_rename_cached_unrelated expected child data")

    os.rename(source, target)
    moved_child = target / "child.txt"
    if not moved_child.is_file():
        if target.exists() and not source.exists():
            os.rename(target, source)
        raise RuntimeError("subtree_rename_cached_unrelated target child missing after rename")

    def cleanup() -> None:
        if target.exists() and not source.exists():
            os.rename(target, source)

    return cleanup


WORKLOADS: dict[str, Callable[[Path, argparse.Namespace, str], Any]] = {
    "seq_read": seq_read,
    "seq_write": seq_write,
    "small_read": small_read,
    "small_write": small_write,
    "write_fsync_close": write_fsync_close,
    "rand_read_4k": rand_read_4k,
    "rand_write_4k": rand_write_4k,
    "concurrent_rand_read_write_4k": concurrent_rand_read_write_4k,
    "sync_write_4k": sync_write_4k,
    "sync_flush_only": sync_flush_only,
    "sync_fsync_only": sync_fsync_only,
    "sync_release_flush": sync_release_flush,
    "read_only_open_close": read_only_open_close,
    "read_only_open_read_close": read_only_open_read_close,
    "write_open_write_close": write_open_write_close,
    "write_open_fsync_close": write_open_fsync_close,
    "small_open_read_close": small_open_read_close,
    "metadata_lookup": metadata_lookup,
    "metadata_getattr": metadata_getattr,
    "metadata_open": metadata_open,
    "metadata_opendir": metadata_opendir,
    "metadata_readlink": metadata_readlink,
    "metadata_access": metadata_access,
    "metadata_statfs": metadata_statfs,
    "small_stat_open_read": small_stat_open_read,
    "readdir_basic": readdir_basic,
    "readdirplus_basic": readdirplus_basic,
    "readdir_lstat": readdir_lstat,
    "matcher_descendant_readdir": matcher_descendant_readdir,
    "matcher_descendant_readdirplus": matcher_descendant_readdirplus,
    "readdir_symlink_visibility": readdir_symlink_visibility,
    "readdirplus_symlink_visibility": readdirplus_symlink_visibility,
    "symlink_open_read": symlink_open_read,
}
SCREENFS_ONLY_WORKLOADS: dict[str, Callable[[Path, argparse.Namespace, str], Any]] = {
    "hidden_stat_miss": hidden_stat_miss,
    "matcher_hidden_stat_miss": matcher_hidden_stat_miss,
    "matcher_readonly_access_wok": matcher_readonly_access_wok,
    "symlink_parent_mkdir_rmdir": symlink_parent_mkdir_rmdir,
    "pinned_symlink_parent_mkdir_rmdir": pinned_symlink_parent_mkdir_rmdir,
    "subtree_rename_cached_unrelated": subtree_rename_cached_unrelated,
}



def resolve_policy(args: argparse.Namespace) -> dict[str, Any]:
    preset = POLICY_PRESETS[args.policy_preset]
    policy_args = list(preset["screenfs_args"])
    if args.matcher_extra_rules:
        policy_args.extend(["--hidden", "/.screenfs-bench/matcher-heavy/descendant-root"])
        for index in range(args.matcher_extra_rules):
            hidden_path = f"/.screenfs-bench/matcher-heavy/hidden-{index:04d}"
            readonly_path = f"/.screenfs-bench/matcher-heavy/visible-{index:04d}/readonly-{index:04d}.txt"
            visible_leaf_path = f"/.screenfs-bench/matcher-heavy/descendant-root/branch-{index:04d}/leaf-{index:04d}.txt"
            policy_args.extend(
                [
                    "--hidden",
                    hidden_path,
                    "--readonly",
                    readonly_path,
                    "--visible",
                    visible_leaf_path,
                ]
            )

    hidden_paths = collect_option_values(policy_args, "--hidden")
    readonly_paths = collect_option_values(policy_args, "--readonly")
    visible_paths = collect_option_values(policy_args, "--visible")
    hidden_paths.extend(collect_option_values(args.extra_screenfs_arg, "--hidden"))
    readonly_paths.extend(collect_option_values(args.extra_screenfs_arg, "--readonly"))
    visible_paths.extend(collect_option_values(args.extra_screenfs_arg, "--visible"))

    manual_policy_override_flags = [arg for arg in args.extra_screenfs_arg if arg in POLICY_SHAPING_EXTRA_ARG_FLAGS]
    suffixes = []
    if args.matcher_extra_rules:
        suffixes.append(f"matcher{args.matcher_extra_rules}")
    if manual_policy_override_flags:
        suffixes.append("extra-policy-args")
    if args.policy_label:
        label = args.policy_label
    elif suffixes:
        label = "-".join([args.policy_preset, *suffixes])
    else:
        label = args.policy_preset

    fast_path_cache_eligible = (
        args.policy_preset == "fast-path-cache-eligible"
        and args.matcher_extra_rules == 0
        and not manual_policy_override_flags
    )
    if fast_path_cache_eligible:
        effective_bucket = "fast-path-cache-eligible"
    elif args.policy_preset == "fallback-unsafe-policy" and args.matcher_extra_rules == 0 and not manual_policy_override_flags:
        effective_bucket = "fallback-unsafe-policy"
    else:
        effective_bucket = "custom-unsafe-policy"

    notes: list[str] = []
    if args.matcher_extra_rules:
        notes.append(
            "matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads"
        )
    if manual_policy_override_flags:
        joined = ", ".join(manual_policy_override_flags)
        notes.append(
            f"extra_screenfs_arg included policy-shaping flags ({joined}); treat the run as a named custom unsafe matrix entry unless independently reviewed"
        )
    if fast_path_cache_eligible:
        notes.append("effective policy remains cache-eligible for the current per-open read/write fast path")

    return {
        "requested_preset": args.policy_preset,
        "requested_bucket": preset["bucket"],
        "effective_bucket": effective_bucket,
        "label": label,
        "description": preset["description"],
        "built_in_screenfs_args": policy_args,
        "hidden_paths": hidden_paths,
        "readonly_paths": readonly_paths,
        "visible_paths": visible_paths,
        "matcher_extra_rules": args.matcher_extra_rules,
        "manual_policy_override_flags": manual_policy_override_flags,
        "extra_screenfs_args": list(args.extra_screenfs_arg),
        "fast_path_cache_eligible": fast_path_cache_eligible,
        "supports_hidden_stat_miss": hidden_paths_support_target(hidden_paths, HIDDEN_STAT_MISS_TARGET),
        "supports_matcher_hidden_stat_miss": args.matcher_extra_rules > 0,
        "supports_matcher_readonly_access_wok": args.matcher_extra_rules > 0,
        "supports_matcher_descendant_directory": args.matcher_extra_rules > 0,
        "notes": notes,
    }



def resolve_workloads(args: argparse.Namespace, policy: dict[str, Any]) -> dict[str, Any]:
    known_workloads = set(WORKLOADS) | set(SCREENFS_ONLY_WORKLOADS)
    explicit = bool(args.workload)
    if explicit:
        requested_names = ordered_unique(args.workload)
        unknown = [name for name in requested_names if name not in known_workloads]
        if unknown:
            raise SystemExit(f"unknown workload(s): {', '.join(unknown)}")
        selection_mode = "explicit"
    else:
        preset = WORKLOAD_SETS[args.workload_set]
        requested_names = preset["comparable"] + preset["screenfs_only"]
        selection_mode = "named-set"

    comparable: list[str] = []
    screenfs_only: list[str] = []
    skipped: list[dict[str, str]] = []
    for name in requested_names:
        reason: str | None = None
        if name in WORKLOADS:
            if name in MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS and not policy["supports_matcher_descendant_directory"]:
                reason = f"{name} requires --matcher-extra-rules > 0"
            elif name in READ_WRITE_CONCURRENCY_COMPARABLE_WORKLOADS and args.concurrency_workers <= 0:
                reason = f"{name} requires --concurrency-workers > 0"
            if reason is None:
                comparable.append(name)
                continue
        else:
            if name == "hidden_stat_miss" and not policy["supports_hidden_stat_miss"]:
                reason = f"{name} requires a hidden /.screenfs-bench/hidden rule in the selected policy"
            elif name == "matcher_hidden_stat_miss" and not policy["supports_matcher_hidden_stat_miss"]:
                reason = f"{name} requires --matcher-extra-rules > 0"
            elif name == "matcher_readonly_access_wok" and not policy["supports_matcher_readonly_access_wok"]:
                reason = f"{name} requires --matcher-extra-rules > 0"
            if reason is None:
                screenfs_only.append(name)
                continue
        if reason is not None:
            if explicit:
                raise SystemExit(reason)
            skipped.append({"name": name, "reason": reason})

    if not comparable and not screenfs_only:
        raise SystemExit("no workloads selected after applying policy/workload filters")

    return {
        "mode": selection_mode,
        "requested_set": None if explicit else args.workload_set,
        "requested_names": requested_names,
        "comparable": comparable,
        "screenfs_only": screenfs_only,
        "skipped": skipped,
    }



def cache_control_scope(method: str) -> str:
    return CACHE_CONTROL_SCOPES.get(method, "unknown")



def default_cache_control_stats(method: str) -> dict[str, Any]:
    return {
        "method": method,
        "scope": cache_control_scope(method),
        "timing_applied": False,
        "support_checked": method == "warm"
        or (getattr(os, "posix_fadvise", None) is not None and getattr(os, "POSIX_FADV_DONTNEED", None) is not None),
        "applications": 0,
        "applied_files": [],
        "skipped": [],
        "errors": [],
        "notes": [],
    }



def ensure_cache_control_supported(args: argparse.Namespace) -> None:
    if getattr(args, "cache_control", "warm") != "posix-fadvise-read-fixture":
        return
    missing: list[str] = []
    if getattr(os, "posix_fadvise", None) is None:
        missing.append("os.posix_fadvise")
    if getattr(os, "POSIX_FADV_DONTNEED", None) is None:
        missing.append("os.POSIX_FADV_DONTNEED")
    if missing:
        raise SystemExit(
            "--cache-control=posix-fadvise-read-fixture requires "
            + ", ".join(missing)
            + "; support must be available before enabling this provenance mode"
        )



def cache_control_candidate_paths(name: str, source_root: Path) -> list[Path]:
    fixture_root = source_root / ".screenfs-bench"
    if name in {"seq_read", "small_read", "rand_read_4k"}:
        return [fixture_root / "read" / "seq.bin"]
    if name == "concurrent_rand_read_write_4k":
        read_root = fixture_root / "concurrency-read"
        if not read_root.is_dir():
            return []
        return sorted(read_root.glob("worker-*.bin"), key=lambda path: path.name)
    return []



def is_write_output_fixture_path(path: Path, source_root: Path) -> bool:
    try:
        relative = path.relative_to(source_root / ".screenfs-bench")
    except ValueError:
        return False
    if not relative.parts:
        return False
    return relative.parts[0] in {"write-native", "write-mounted"}



def select_cache_control_read_fixture_files(name: str, source_root: Path) -> tuple[list[Path], list[dict[str, str]]]:
    selected: list[Path] = []
    skipped: list[dict[str, str]] = []
    seen_inodes: set[tuple[int, int]] = set()
    for path in cache_control_candidate_paths(name, source_root):
        if is_write_output_fixture_path(path, source_root):
            skipped.append({"path": str(path), "reason": "write-output"})
            continue
        try:
            status = path.lstat()
        except OSError as exc:
            skipped.append({"path": str(path), "reason": f"lstat-failed: {exc}"})
            continue
        if stat.S_ISLNK(status.st_mode):
            skipped.append({"path": str(path), "reason": "symlink"})
            continue
        if not stat.S_ISREG(status.st_mode):
            skipped.append({"path": str(path), "reason": "non-regular"})
            continue
        inode_key = (status.st_dev, status.st_ino)
        if inode_key in seen_inodes:
            skipped.append({"path": str(path), "reason": "duplicate-inode"})
            continue
        seen_inodes.add(inode_key)
        selected.append(path)
    return selected, skipped



def prepare_cache_control_stats(name: str, args: argparse.Namespace, cache_control_source_root: Path | None) -> tuple[list[Path], dict[str, Any]]:
    stats = default_cache_control_stats(args.cache_control)
    method = args.cache_control
    if method == "warm":
        stats["notes"] = ["warm mode leaves cache state unchanged before warmups and measured samples"]
        return [], stats
    if method != "posix-fadvise-read-fixture":
        stats["notes"] = [f"unrecognized cache-control method: {method}"]
        return [], stats
    if name not in READ_SIDE_CACHE_CONTROL_WORKLOADS:
        stats["notes"] = ["cache control only applies to read-side fixture workloads"]
        return [], stats
    if cache_control_source_root is None:
        stats["notes"] = ["cache control skipped because no backing source root was provided"]
        return [], stats
    selected, skipped = select_cache_control_read_fixture_files(name, cache_control_source_root)
    stats["applied_files"] = [str(path) for path in selected]
    stats["skipped"] = skipped
    stats["timing_applied"] = bool(selected)
    if selected:
        stats["notes"] = [
            "apply os.posix_fadvise(..., POSIX_FADV_DONTNEED) to selected backing source fixture files before each warmup and measured sample"
        ]
    else:
        stats["notes"] = ["no regular backing source fixture files were selected for cache control"]
    return selected, stats



def apply_cache_control_to_files(paths: list[Path], stats: dict[str, Any]) -> None:
    if not paths:
        return
    stats["applications"] += 1
    open_flags = os.O_RDONLY | os.O_CLOEXEC
    open_flags |= getattr(os, "O_NOFOLLOW", 0)
    for path in paths:
        fd: int | None = None
        try:
            fd = os.open(path, open_flags)
            descriptor_stat = os.fstat(fd)
            if not stat.S_ISREG(descriptor_stat.st_mode):
                raise RuntimeError(f"cache-control target is not a regular file: {path}")
            os.posix_fadvise(fd, 0, 0, os.POSIX_FADV_DONTNEED)
        except (OSError, RuntimeError) as exc:
            stats["errors"].append({"path": str(path), "error": str(exc)})
            raise RuntimeError(f"cache-control failed for {path}: {exc}") from exc
        finally:
            if fd is not None:
                os.close(fd)



def time_one(func: Callable[[Path, argparse.Namespace, str], Any], root: Path, args: argparse.Namespace, side: str) -> float:
    start = time.perf_counter_ns()
    cleanup = func(root, args, side)
    end = time.perf_counter_ns()
    if callable(cleanup):
        cleanup()
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
    func: Callable[[Path, argparse.Namespace, str], Any],
    root: Path,
    args: argparse.Namespace,
    side: str,
    cache_control_source_root: Path | None = None,
) -> dict[str, Any]:
    cache_control_paths, cache_control_stats = prepare_cache_control_stats(name, args, cache_control_source_root)
    for _ in range(args.warmups):
        apply_cache_control_to_files(cache_control_paths, cache_control_stats)
        time_one(func, root, args, side)
    samples = []
    for _ in range(args.iterations):
        apply_cache_control_to_files(cache_control_paths, cache_control_stats)
        samples.append(time_one(func, root, args, side))
    return {
        "name": name,
        "side": side,
        "samples_sec": samples,
        "summary": summarize(samples),
        "cache_control": cache_control_stats,
    }



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
        '<rect width="100%" height="100%" fill="white"/>',
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
        summary = summarize(values)
        minimum = summary["min_sec"]
        q1 = summary["q1_sec"]
        median = summary["median_sec"]
        q3 = summary["q3_sec"]
        maximum = summary["max_sec"]
        p90 = summary["p90_sec"]
        p95 = summary["p95_sec"]
        p99 = summary["p99_sec"]
        cy = top + row_height * index + row_height / 2
        box_top = cy - 9
        box_height = 18
        label = f"{name} ({side})"
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



def benchmark_parameters(args: argparse.Namespace) -> dict[str, Any]:
    return {
        "cache_control": args.cache_control,
        "iterations": args.iterations,
        "warmups": args.warmups,
        "read_mib": args.read_mib,
        "write_mib": args.write_mib,
        "small_io_bytes": args.small_io_bytes,
        "small_io_ops": args.small_io_ops,
        "sync_bytes": args.sync_bytes,
        "sync_ops": args.sync_ops,
        "rand_io_ops": args.rand_io_ops,
        "concurrency_workers": args.concurrency_workers,
        "open_read_close_ops": args.open_read_close_ops,
        "metadata_ops": args.metadata_ops,
        "sync_4k_fsync_every": args.sync_4k_fsync_every,
        "small_files": args.small_files,
        "dir_entries": args.dir_entries,
        "hidden_misses": args.hidden_misses,
        "matcher_extra_rules": args.matcher_extra_rules,
        "matcher_misses": args.matcher_misses,
        "symlink_parent_mutations": args.symlink_parent_mutations,
    }



def benchmark_environment(
    source: Path,
    mounted_filesystem: dict[str, Any],
    harness_repo_root: Path,
) -> dict[str, Any]:
    harness_repo = git_provenance(harness_repo_root)
    return {
        "platform": platform.platform(),
        "python": sys.version.split()[0],
        "uname": " ".join(platform.uname()),
        "harness_repo_root": str(harness_repo_root),
        "git_revision": harness_repo["git_revision"],
        "git_status_porcelain": harness_repo["git_status_porcelain"],
        "git_worktree_clean": harness_repo["git_worktree_clean"],
        "source_filesystem": path_fs_info(source),
        "mount_filesystem": mounted_filesystem,
        "rustc": command_output(["rustc", "--version"]),
        "cargo": command_output(["cargo", "--version"]),
        "fusermount3": command_output(["fusermount3", "--version"]),
    }



def aggregate_cache_control_summary(method: str, *result_sets: list[dict[str, Any]]) -> dict[str, Any]:
    summary = default_cache_control_stats(method)
    result_items = [item for result_set in result_sets for item in result_set]
    if not result_items:
        return summary
    summary["timing_applied"] = any(bool(item.get("cache_control", {}).get("timing_applied")) for item in result_items)
    summary["applications"] = sum(int(item.get("cache_control", {}).get("applications", 0)) for item in result_items)
    summary["applied_files"] = sorted(
        {
            path
            for item in result_items
            for path in item.get("cache_control", {}).get("applied_files", [])
        }
    )
    summary["skipped"] = [
        {
            "workload": item["name"],
            "side": item["side"],
            **skipped,
        }
        for item in result_items
        for skipped in item.get("cache_control", {}).get("skipped", [])
    ]
    summary["errors"] = [
        {
            "workload": item["name"],
            "side": item["side"],
            **error,
        }
        for item in result_items
        for error in item.get("cache_control", {}).get("errors", [])
    ]
    summary["notes"] = list(
        dict.fromkeys(
            note
            for item in result_items
            for note in item.get("cache_control", {}).get("notes", [])
            if note
        )
    )
    return summary



def build_benchmark_result(
    *,
    args: argparse.Namespace,
    timestamp: str,
    harness_command: list[str],
    environment: dict[str, Any],
    workdir: Path,
    source: Path,
    mount: Path,
    policy: dict[str, Any],
    workloads: dict[str, Any],
    screenfs_bin: Path,
    screenfs_binary_sha256: str,
    screenfs_source: dict[str, Any],
    command: list[str],
    stderr_path: Path,
    screenfs_stderr: str,
    perf_summary: dict[str, Any] | None,
    native_results: list[dict[str, Any]],
    mounted_results: list[dict[str, Any]],
    screenfs_only: list[dict[str, Any]],
    comparisons: dict[str, Any],
) -> dict[str, Any]:
    return {
        "schema": "screenfs-benchmark-v2",
        "timestamp": timestamp,
        "harness": {
            "argv": harness_command,
            "command_line": shell_join(harness_command),
        },
        "environment": environment,
        "parameters": benchmark_parameters(args),
        "cache_control": aggregate_cache_control_summary(args.cache_control, native_results, mounted_results, screenfs_only),
        "paths": {"workdir": str(workdir), "source": str(source), "mount": str(mount)},
        "policy": policy,
        "workloads": workloads,
        "screenfs": {
            "binary": str(screenfs_bin),
            "binary_sha256": screenfs_binary_sha256,
            "source": screenfs_source,
            "command": command,
            "stderr_log": str(stderr_path),
            "stderr_preview": screenfs_stderr[-4000:],
            "perf_counters_enabled": args.perf_counters,
            "perf_summary": perf_summary,
        },
        "native": native_results,
        "mounted": mounted_results,
        "screenfs_only": screenfs_only,
        "comparisons": comparisons,
    }



def markdown_report(result: dict[str, Any]) -> str:
    environment = result["environment"]
    screenfs = result["screenfs"]
    source_info = screenfs.get("source") or {}
    git_dirty_status = summarize_git_status(environment.get("git_status_porcelain"))
    source_dirty_status = summarize_git_status(source_info.get("git_status_porcelain"))
    policy = result["policy"]
    workloads = result["workloads"]
    cache_control = result.get("cache_control") or aggregate_cache_control_summary(result["parameters"].get("cache_control", "warm"))

    comparable_names = ", ".join(workloads["comparable"]) if workloads["comparable"] else "(none)"
    screenfs_only_names = ", ".join(workloads["screenfs_only"]) if workloads["screenfs_only"] else "(none)"
    parameter_cache_control = result["parameters"].get("cache_control", "warm")

    provenance_lines = [
        f"- timestamp: `{result['timestamp']}`",
        f"- harness_command_line: `{result['harness']['command_line']}`",
        f"- harness_repo_root: `{environment.get('harness_repo_root') or 'unknown'}`",
        f"- git: `{environment.get('git_revision') or 'unknown'}`",
        f"- git_worktree_clean: `{environment['git_worktree_clean']}`",
        f"- screenfs_bin: `{screenfs['binary']}`",
        f"- screenfs_bin_sha256: `{screenfs['binary_sha256']}`",
        f"- screenfs_source_root: `{source_info.get('source_root') or 'unknown'}`",
        f"- screenfs_source_root_origin: `{source_info.get('source_root_origin') or 'unknown'}`",
        f"- screenfs_source_git: `{source_info.get('git_revision') or 'unknown'}`",
        f"- screenfs_source_git_worktree_clean: `{source_info.get('git_worktree_clean') if source_info.get('git_worktree_clean') is not None else 'unknown'}`",
        f"- policy_preset: `{policy['requested_preset']}`",
        f"- policy_bucket: `{policy['effective_bucket']}`",
        f"- policy_label: `{policy['label']}`",
        f"- fast_path_cache_eligible: `{policy['fast_path_cache_eligible']}`",
        f"- workload_selection: `{workloads['mode']}`",
        f"- workload_set: `{workloads['requested_set'] or 'explicit'}`",
        f"- comparable_workloads: `{comparable_names}`",
        f"- screenfs_only_workloads: `{screenfs_only_names}`",
        f"- cache_control: `{parameter_cache_control}`",
        f"- cache_control_scope: `{cache_control['scope']}`",
        f"- cache_control_timing_applied: `{cache_control['timing_applied']}`",
        f"- cache_control_applications: `{cache_control.get('applications', 0)}`",
        f"- concurrency_workers: `{result['parameters']['concurrency_workers']}`",
        f"- iterations: `{result['parameters']['iterations']}`, warmups: `{result['parameters']['warmups']}`",
    ]
    if git_dirty_status is not None:
        provenance_lines.insert(4, f"- git_dirty_status: `{git_dirty_status}`")
    if source_dirty_status is not None:
        provenance_lines.insert(11, f"- screenfs_source_git_dirty_status: `{source_dirty_status}`")
    if policy["notes"]:
        provenance_lines.insert(16, f"- policy_notes: `{' ; '.join(policy['notes'])}`")
    if cache_control.get("notes"):
        provenance_lines.insert(-2, f"- cache_control_notes: `{' ; '.join(cache_control['notes'])}`")

    lines = [
        "# ScreenFS benchmark result",
        "",
        *provenance_lines,
        "",
        "## Comparable workloads",
        "",
        "| workload | native p50 s | mounted p50 s | ratio mounted/native | mounted p90 s | mounted p95 s | mounted p99 s |",
        "| --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]

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
    if workloads["skipped"]:
        lines.extend(["", "## Skipped workloads", ""])
        for skipped in workloads["skipped"]:
            lines.append(f"- `{skipped['name']}`: {skipped['reason']}")
    perf_summary = screenfs.get("perf_summary")
    lines.extend(["", "## Perf counters", ""])
    if perf_summary:
        lines.extend(
            [
                "Perf counters were enabled and captured from ScreenFS stderr after unmount/termination. They are process-lifetime aggregates and can include warmups plus cleanup work that runs before unmount, even when cleanup is outside the timed latency samples.",
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
            "- Native and mounted timings use the same prepared backing fixture; write workloads use separate native/mounted output files and remove them after each iteration. Workloads that return cleanup callbacks run those deletions outside timed latency samples, but perf counters still include cleanup-side `unlink`/invalidation work before unmount.",
            "- `--cache-control=posix-fadvise-read-fixture` applies `os.posix_fadvise(..., POSIX_FADV_DONTNEED)` to selected `source/.screenfs-bench` read fixtures before each warmup and measured sample for read-side workloads; mounted runs evict backing source paths rather than mounted view paths.",
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
        "rand_io_ops",
        "concurrency_workers",
        "open_read_close_ops",
        "metadata_ops",
        "sync_4k_fsync_every",
        "hidden_misses",
        "matcher_misses",
        "symlink_parent_mutations",
    ]:
        require_positive(name, getattr(args, name))
    if args.matcher_extra_rules < 0:
        raise SystemExit("matcher_extra_rules must be non-negative")

    if hasattr(os, "geteuid") and os.geteuid() == 0:
        raise SystemExit("do not run this benchmark as root or with sudo; ScreenFS evidence must be non-root FUSE evidence")

    policy = resolve_policy(args)
    workload_selection = resolve_workloads(args, policy)
    ensure_cache_control_supported(args)

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
    harness_repo_root_text = command_output(["git", "rev-parse", "--show-toplevel"])
    harness_repo_root = Path(harness_repo_root_text) if harness_repo_root_text else Path(__file__).resolve().parent.parent
    screenfs_source = resolve_screenfs_source_provenance(screenfs_bin, args.screenfs_source_root)
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
    command = [
        str(screenfs_bin),
        str(source),
        str(mount),
        *policy["built_in_screenfs_args"],
        *args.extra_screenfs_arg,
    ]

    proc: subprocess.Popen[str] | None = None
    unmount_result: dict[str, Any] = {"ok": False, "attempts": []}
    try:
        proc = subprocess.Popen(command, text=True, stdout=subprocess.PIPE, stderr=stderr_file)
        wait_for_mount(mount, proc, args.timeout_sec)

        native_results = [
            measure_workload(name, WORKLOADS[name], source, args, "native", cache_control_source_root=source)
            for name in workload_selection["comparable"]
        ]
        mounted_results = [
            measure_workload(name, WORKLOADS[name], mount, args, "mounted", cache_control_source_root=source)
            for name in workload_selection["comparable"]
        ]
        screenfs_only = [
            measure_workload(name, SCREENFS_ONLY_WORKLOADS[name], mount, args, "mounted", cache_control_source_root=source)
            for name in workload_selection["screenfs_only"]
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

        harness_command = harness_argv()
        result = build_benchmark_result(
            args=args,
            timestamp=datetime.now(timezone.utc).isoformat(),
            harness_command=harness_command,
            environment=benchmark_environment(source, mounted_filesystem, harness_repo_root),
            workdir=workdir,
            source=source,
            mount=mount,
            policy=policy,
            workloads=workload_selection,
            screenfs_bin=screenfs_bin,
            screenfs_binary_sha256=screenfs_binary_sha256,
            screenfs_source=screenfs_source,
            command=command,
            stderr_path=stderr_path,
            screenfs_stderr=screenfs_stderr,
            perf_summary=perf_summary,
            native_results=native_results,
            mounted_results=mounted_results,
            screenfs_only=screenfs_only,
            comparisons=comparisons,
        )

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
