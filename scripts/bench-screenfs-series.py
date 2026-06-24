#!/usr/bin/env python3
"""Run repeated before/after ScreenFS benchmark series.

This is a thin orchestration wrapper around scripts/bench-screenfs.py. It is
intended for noisy matcher-heavy investigations where a single before/after
pair is not enough evidence.
"""

from __future__ import annotations

import argparse
import json
import shlex
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

SUMMARY_KEYS = ("p50_sec", "p95_sec", "p99_sec")


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Run scripts/bench-screenfs.py as a repeated before/after series and "
            "summarize per-pair after/before ratios."
        )
    )
    parser.add_argument("--bench-script", type=Path, default=Path(__file__).with_name("bench-screenfs.py"))
    parser.add_argument("--before-bin", required=True, type=Path)
    parser.add_argument("--after-bin", required=True, type=Path)
    parser.add_argument("--before-label", default="before")
    parser.add_argument("--after-label", default="after")
    parser.add_argument("--pairs", type=positive_int, default=3)
    parser.add_argument(
        "--order",
        choices=("before-after", "after-before", "alternate"),
        default="before-after",
        help="Run order within each pair. alternate flips odd/even pairs.",
    )
    parser.add_argument("--output-dir", required=True, type=Path)
    parser.add_argument("--stem", default="series")
    parser.add_argument("--manifest-json", type=Path)
    parser.add_argument("--summary-md", type=Path)
    parser.add_argument(
        "--workload",
        action="append",
        help="Workload names to summarize from child JSON. Repeat as needed. Defaults to workloads present in each before/after child JSON pair.",
    )
    parser.add_argument(
        "bench_args",
        nargs=argparse.REMAINDER,
        help="Arguments passed through to bench-screenfs.py after --. Do not include --screenfs-bin, --output-json, --output-md, or --output-svg.",
    )
    args = parser.parse_args(argv)
    if args.bench_args and args.bench_args[0] == "--":
        args.bench_args = args.bench_args[1:]
    reject_managed_args(args.bench_args)
    if args.manifest_json is None:
        args.manifest_json = args.output_dir / f"{args.stem}-manifest.json"
    if args.summary_md is None:
        args.summary_md = args.output_dir / f"{args.stem}-summary.md"
    return args


def positive_int(value: str) -> int:
    parsed = int(value)
    if parsed < 1:
        raise argparse.ArgumentTypeError("value must be >= 1")
    return parsed


def reject_managed_args(bench_args: list[str]) -> None:
    managed = {"--screenfs-bin", "--output-json", "--output-md", "--output-svg"}
    for arg in bench_args:
        if arg in managed or any(arg.startswith(f"{flag}=") for flag in managed):
            raise SystemExit(f"bench-screenfs-series.py manages {arg.split('=')[0]}; remove it from passthrough args")


def pair_order(order: str, pair_index: int) -> tuple[str, str]:
    if order == "after-before":
        return ("after", "before")
    if order == "alternate" and pair_index % 2 == 0:
        return ("after", "before")
    return ("before", "after")


def output_stem(args: argparse.Namespace, pair_index: int, side: str) -> str:
    label = args.before_label if side == "before" else args.after_label
    return f"{args.stem}-pair{pair_index}-{side}-{label}"


def run_command(argv: list[str]) -> None:
    subprocess.run(argv, check=True)


def build_child_argv(args: argparse.Namespace, pair_index: int, side: str) -> tuple[list[str], dict[str, str]]:
    stem = output_stem(args, pair_index, side)
    binary = args.before_bin if side == "before" else args.after_bin
    paths = {
        "json": str(args.output_dir / f"{stem}.json"),
        "md": str(args.output_dir / f"{stem}.md"),
        "svg": str(args.output_dir / f"{stem}.svg"),
    }
    argv = [
        sys.executable,
        str(args.bench_script),
        "--screenfs-bin",
        str(binary),
        *args.bench_args,
        "--output-json",
        paths["json"],
        "--output-md",
        paths["md"],
        "--output-svg",
        paths["svg"],
    ]
    return argv, paths


def result_entry(data: dict[str, Any], name: str) -> dict[str, Any] | None:
    for section in ("mounted", "screenfs_only"):
        for entry in data.get(section, []):
            if entry.get("name") == name:
                return entry
    return None


def summarize_pair(before_path: Path, after_path: Path, workloads: list[str]) -> dict[str, Any]:
    before = json.loads(before_path.read_text())
    after = json.loads(after_path.read_text())
    ratios: dict[str, dict[str, float]] = {}
    missing: list[str] = []
    for workload in workloads:
        before_entry = result_entry(before, workload)
        after_entry = result_entry(after, workload)
        if before_entry is None or after_entry is None:
            missing.append(workload)
            continue
        before_summary = before_entry.get("summary", {})
        after_summary = after_entry.get("summary", {})
        workload_ratios = {}
        for key in SUMMARY_KEYS:
            before_value = before_summary.get(key)
            after_value = after_summary.get(key)
            if not before_value:
                continue
            workload_ratios[key] = after_value / before_value
        if workload_ratios:
            ratios[workload] = workload_ratios
    return {"ratios": ratios, "missing_workloads": missing}


def workload_names(data: dict[str, Any]) -> set[str]:
    names: set[str] = set()
    for section in ("mounted", "screenfs_only"):
        for entry in data.get(section, []):
            name = entry.get("name")
            if isinstance(name, str):
                names.add(name)
    return names


def infer_common_workloads(before_path: Path, after_path: Path) -> list[str]:
    before = json.loads(before_path.read_text())
    after = json.loads(after_path.read_text())
    return sorted(workload_names(before) & workload_names(after))


def build_manifest(args: argparse.Namespace, runs: list[dict[str, Any]], command_line: list[str]) -> dict[str, Any]:
    requested_workloads = list(args.workload or [])
    pair_summaries = []
    summarized_workloads: set[str] = set(requested_workloads)
    for pair_index in range(1, args.pairs + 1):
        before_run = next(run for run in runs if run["pair"] == pair_index and run["side"] == "before")
        after_run = next(run for run in runs if run["pair"] == pair_index and run["side"] == "after")
        before_path = Path(before_run["outputs"]["json"])
        after_path = Path(after_run["outputs"]["json"])
        workloads = requested_workloads or infer_common_workloads(before_path, after_path)
        summarized_workloads.update(workloads)
        summary = summarize_pair(before_path, after_path, workloads)
        pair_summaries.append({"pair": pair_index, **summary})
    return {
        "schema": "screenfs-benchmark-series-v1",
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "command_line": shlex.join(command_line),
        "bench_script": str(args.bench_script),
        "before_bin": str(args.before_bin),
        "after_bin": str(args.after_bin),
        "before_label": args.before_label,
        "after_label": args.after_label,
        "pairs": args.pairs,
        "order": args.order,
        "bench_args": args.bench_args,
        "summarized_workloads": sorted(summarized_workloads),
        "runs": runs,
        "pair_summaries": pair_summaries,
    }


def markdown_report(manifest: dict[str, Any]) -> str:
    lines = [
        "# ScreenFS benchmark series",
        "",
        f"- timestamp: `{manifest['timestamp']}`",
        f"- pairs: `{manifest['pairs']}`",
        f"- order: `{manifest['order']}`",
        f"- before_bin: `{manifest['before_bin']}`",
        f"- after_bin: `{manifest['after_bin']}`",
        f"- bench_args: `{shlex.join(manifest['bench_args'])}`",
        "",
        "## Pair ratios",
        "",
        "After/before ratios are computed from child benchmark JSON mounted or ScreenFS-only summaries.",
        "",
        "| pair | workload | p50 | p95 | p99 |",
        "| ---: | --- | ---: | ---: | ---: |",
    ]
    for pair in manifest["pair_summaries"]:
        for workload, ratios in pair["ratios"].items():
            lines.append(
                "| {pair} | `{workload}` | {p50} | {p95} | {p99} |".format(
                    pair=pair["pair"],
                    workload=workload,
                    p50=format_ratio(ratios.get("p50_sec")),
                    p95=format_ratio(ratios.get("p95_sec")),
                    p99=format_ratio(ratios.get("p99_sec")),
                )
            )
    lines.extend(["", "## Runs", "", "| order | pair | side | json |", "| ---: | ---: | --- | --- |"])
    for index, run in enumerate(manifest["runs"], start=1):
        lines.append(f"| {index} | {run['pair']} | `{run['side']}` | `{run['outputs']['json']}` |")
    lines.append("")
    return "\n".join(lines)


def format_ratio(value: float | None) -> str:
    if value is None:
        return ""
    return f"{value:.3f}x"


def main(argv: list[str] | None = None) -> int:
    command_line = sys.argv if argv is None else [str(Path(__file__)), *argv]
    args = parse_args(argv)
    args.output_dir.mkdir(parents=True, exist_ok=True)
    args.manifest_json.parent.mkdir(parents=True, exist_ok=True)
    args.summary_md.parent.mkdir(parents=True, exist_ok=True)
    runs: list[dict[str, Any]] = []
    for pair_index in range(1, args.pairs + 1):
        for side in pair_order(args.order, pair_index):
            child_argv, outputs = build_child_argv(args, pair_index, side)
            run_command(child_argv)
            runs.append({"pair": pair_index, "side": side, "argv": child_argv, "outputs": outputs})
    manifest = build_manifest(args, runs, command_line)
    args.manifest_json.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    args.summary_md.write_text(markdown_report(manifest))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
