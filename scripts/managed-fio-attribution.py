#!/usr/bin/env python3
"""Run and render the managed fio attribution artifact set.

This script is intentionally repo-local tooling for docs/artifacts/managed-fio-attribution-*.
It runs native fio, the contrib fractal passthrough helper, and ScreenFS, then writes
JSON/log/provenance plus Markdown/SVG/2x PNG summary artifacts.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from xml.sax.saxutils import escape

REPO = Path(__file__).resolve().parents[1]
ARTIFACTS = REPO / "docs" / "artifacts"
DEFAULT_PREFIX = ARTIFACTS / "managed-fio-attribution"
JOB_NAMES = ["seq_write_128k", "seq_read_128k", "rand_write_4k", "rand_read_4k", "sync_write_4k"]
SERIES = ["native", "passthrough", "screenfs"]
COLORS = {"native": "#7f8c8d", "passthrough": "#2d7dd2", "screenfs": "#d1495b"}


def run(cmd: list[str], *, cwd: Path = REPO, stdout=None, stderr=None, check: bool = True) -> subprocess.CompletedProcess:
    print("+", " ".join(cmd), file=sys.stderr)
    return subprocess.run(cmd, cwd=cwd, text=True, stdout=stdout, stderr=stderr, check=check)


def out(cmd: list[str], *, cwd: Path = REPO) -> str:
    return subprocess.check_output(cmd, cwd=cwd, text=True).strip()


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def ensure_clean_unmount(mount: Path) -> None:
    subprocess.run(["fusermount3", "-u", str(mount)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, text=True)


def wait_mounted(mount: Path) -> None:
    for _ in range(50):
        r = subprocess.run(["findmnt", "--target", str(mount)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, text=True)
        if r.returncode == 0:
            return
        import time
        time.sleep(0.1)
    raise RuntimeError(f"mount did not appear at {mount}")


def write_fstype(mount: Path, output: Path) -> None:
    text = out(["findmnt", "--target", str(mount), "--noheadings", "--output", "FSTYPE,OPTIONS"])
    output.write_text(text + "\n", encoding="utf-8")


def clat(job: dict) -> dict:
    op = "read" if job["jobname"] in {"seq_read_128k", "rand_read_4k"} else "write"
    return job[op]["clat_ns"]


def clat_us(job: dict, key: str) -> float:
    c = clat(job)
    if key == "mean":
        return float(c["mean"]) / 1000.0
    pct = c.get("percentile", {})
    # fio percentile keys are stringified floats, e.g. "95.000000".
    return float(pct[f"{float(key):.6f}"]) / 1000.0


def fio_by_job(path: Path) -> dict[str, dict]:
    data = json.loads(path.read_text(encoding="utf-8"))
    return {job["jobname"]: job for job in data["jobs"]}


def parse_perf(stderr_path: Path, output: Path) -> dict:
    text = stderr_path.read_text(encoding="utf-8", errors="replace")
    counters: dict[str, dict] = {}
    for line in text.splitlines():
        m = re.match(r"\s*([^:]+): count=(\d+)(?: total_ns=(\d+) avg_ns=(\d+) max_ns=(\d+))?", line)
        if not m:
            continue
        item: dict[str, int] = {"count": int(m.group(2))}
        if m.group(3):
            item.update(total_ns=int(m.group(3)), avg_ns=int(m.group(4)), max_ns=int(m.group(5)))
        counters[m.group(1)] = item
    parsed = {"counters": counters, "raw": text}
    output.write_text(json.dumps(parsed, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return parsed


def ratio(a: float, b: float) -> float:
    return b / a if a else float("nan")


def render_summary_svg(results: dict[str, dict[str, dict]], output: Path) -> None:
    width, height = 1200, 700
    left, top = 210, 95
    row_h = 88
    max_us = max(clat_us(results[s][j], "mean") for s in SERIES for j in JOB_NAMES) * 1.15
    plot_w = 840
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">', '<rect width="100%" height="100%" fill="white"/>', '<style>text{font-family:Arial,sans-serif;font-size:15px}.title{font-size:24px;font-weight:700}.label{font-weight:700}.small{font-size:13px;fill:#555}</style>', '<text class="title" x="40" y="45">Managed fio attribution summary</text>', '<text class="small" x="40" y="70">Mean completion latency (µs), native vs managed passthrough vs ScreenFS</text>']
    for i, job in enumerate(JOB_NAMES):
        y = top + i * row_h
        parts.append(f'<text class="label" x="40" y="{y + 25}">{escape(job)}</text>')
        for n, s in enumerate(SERIES):
            val = clat_us(results[s][job], "mean")
            bar = max(1, val / max_us * plot_w)
            by = y + 4 + n * 22
            parts.append(f'<rect x="{left}" y="{by}" width="{bar:.1f}" height="16" fill="{COLORS[s]}"/>')
            parts.append(f'<text x="{left + bar + 8:.1f}" y="{by + 13}">{s}: {val:.3f}</text>')
    parts.append(f'<text class="small" x="{left}" y="{height - 35}">Generated by scripts/managed-fio-attribution.py</text>')
    parts.append('</svg>')
    output.write_text("\n".join(parts) + "\n", encoding="utf-8")


def percentile_us(clat_ns: dict, percentile: float) -> float:
    pct = {float(k): float(v) / 1000.0 for k, v in clat_ns.get("percentile", {}).items()}
    if percentile in pct:
        return pct[percentile]
    if not pct:
        return float(clat_ns.get("mean", 0)) / 1000.0
    keys = sorted(pct)
    if percentile <= keys[0]:
        return pct[keys[0]]
    if percentile >= keys[-1]:
        return pct[keys[-1]]
    lower = max(k for k in keys if k < percentile)
    upper = min(k for k in keys if k > percentile)
    span = upper - lower
    weight = (percentile - lower) / span if span else 0.0
    return pct[lower] + (pct[upper] - pct[lower]) * weight


def render_boxplot_svg(results: dict[str, dict[str, dict]], output: Path) -> None:
    width, height = 1600, 900
    left, right, top, bottom = 120, 60, 90, 120
    plot_w, plot_h = width - left - right, height - top - bottom
    y_max = 250.0
    def y(v: float) -> float:
        return top + plot_h - min(v, y_max) / y_max * plot_h
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">', '<rect width="100%" height="100%" fill="white"/>', '<style>text{font-family:Arial,sans-serif;font-size:15px}.title{font-size:24px;font-weight:700}.axis{stroke:#333;stroke-width:1}.grid{stroke:#ddd;stroke-width:1}.small{font-size:12px;fill:#555}</style>', '<text class="title" x="120" y="45">Managed fio attribution clat distribution</text>', '<text class="small" x="120" y="68">p1/p25/p50/p75/p99 whisker box, p95 orange dot, mean black diamond; y-axis fixed 0..250 µs</text>']
    for tick in range(0, 251, 50):
        yy = y(tick)
        parts.append(f'<line class="grid" x1="{left}" y1="{yy:.1f}" x2="{width-right}" y2="{yy:.1f}"/>')
        parts.append(f'<text x="45" y="{yy+5:.1f}">{tick}</text>')
    parts.append(f'<line class="axis" x1="{left}" y1="{top}" x2="{left}" y2="{height-bottom}"/>')
    parts.append(f'<line class="axis" x1="{left}" y1="{height-bottom}" x2="{width-right}" y2="{height-bottom}"/>')
    group_w = plot_w / len(JOB_NAMES)
    box_w = 28
    for i, job in enumerate(JOB_NAMES):
        gx = left + group_w * (i + 0.5)
        parts.append(f'<text text-anchor="middle" x="{gx}" y="{height-70}" transform="rotate(-20 {gx} {height-70})">{escape(job)}</text>')
        for n, s in enumerate(SERIES):
            x = gx + (n - 1) * 40
            c = clat(results[s][job])
            vals = {"p1": percentile_us(c, 1.0), "p25": percentile_us(c, 25.0), "p50": percentile_us(c, 50.0), "p75": percentile_us(c, 75.0), "p95": percentile_us(c, 95.0), "p99": percentile_us(c, 99.0), "mean": float(c["mean"]) / 1000}
            parts.append(f'<line x1="{x}" y1="{y(vals["p1"]):.1f}" x2="{x}" y2="{y(vals["p99"]):.1f}" stroke="{COLORS[s]}" stroke-width="2"/>')
            parts.append(f'<rect x="{x-box_w/2}" y="{y(vals["p75"]):.1f}" width="{box_w}" height="{max(1, y(vals["p25"])-y(vals["p75"])):.1f}" fill="{COLORS[s]}" fill-opacity="0.25" stroke="{COLORS[s]}"/>')
            parts.append(f'<line x1="{x-box_w/2}" y1="{y(vals["p50"]):.1f}" x2="{x+box_w/2}" y2="{y(vals["p50"]):.1f}" stroke="{COLORS[s]}" stroke-width="3"/>')
            parts.append(f'<circle cx="{x}" cy="{y(vals["p95"]):.1f}" r="4" fill="#f28e2b"/>')
            my = y(vals["mean"])
            parts.append(f'<polygon points="{x},{my-5:.1f} {x+5},{my:.1f} {x},{my+5:.1f} {x-5},{my:.1f}" fill="#111"/>')
    legend_x = width - right - 280
    for n, s in enumerate(SERIES):
        ly = top + n * 24
        parts.append(f'<rect x="{legend_x}" y="{ly-12}" width="16" height="16" fill="{COLORS[s]}"/>')
        parts.append(f'<text x="{legend_x+24}" y="{ly+2}">{s}</text>')
    parts.append('</svg>')
    output.write_text("\n".join(parts) + "\n", encoding="utf-8")


def render_png(svg: Path, png: Path, scale: int) -> None:
    # Query width/height from the generated root element and rasterize at scale.
    text = svg.read_text(encoding="utf-8")[:300]
    w = int(re.search(r'width="(\d+)"', text).group(1))
    h = int(re.search(r'height="(\d+)"', text).group(1))
    run(["rsvg-convert", "-w", str(w * scale), "-h", str(h * scale), str(svg), "-o", str(png)])


def write_summary(results: dict[str, dict[str, dict]], perf: dict, env: dict, prefix: Path) -> None:
    md = prefix.with_name(prefix.name + "-summary.md")
    lines = ["# Managed passthrough fio attribution: native vs contrib passthrough vs ScreenFS", "", "This artifact reruns the supplemental fio attribution with the managed `contrib/fractal-passthrough` helper. It is attribution evidence, not claim-grade before/after optimization evidence.", "", "## Measured setup", "", f"- Passthrough implementation: managed `contrib/fractal-passthrough` helper built at `{env['passthrough_binary']}`", "- Policy caveat: this is an explicit custom unsafe shape, not the official harness `fallback-unsafe-policy` preset, because the carve-outs target `/hidden` and `/readonly` instead of `/.screenfs-bench/**`", f"- Mount/provenance caveats: non-root FUSE run with `{env.get('fusermount3_version','unknown')}`; `managed-fio-attribution-env.json` records command lines, binary SHA256 values, and dirty worktree status", "- Passthrough safety caveat: `contrib/fractal-passthrough` is only a minimal FUSE floor. It does not implement ScreenFS source-root confinement, visibility/mutability policy, or external-symlink safety checks, so use only a trusted private scratch tree with no external symlinks and do not treat it as a policy-equivalent baseline", "", "## Files", ""]
    outputs = [Path(p).name for p in env["outputs"]]
    for name in outputs:
        lines.append(f"- `docs/artifacts/{name}`")
    lines += ["", "## Results", "", "| job | native mean clat µs | passthrough mean clat µs | ScreenFS mean clat µs | passthrough/native | ScreenFS/passthrough |", "| --- | ---: | ---: | ---: | ---: | ---: |"]
    for job in JOB_NAMES:
        n = clat_us(results["native"][job], "mean")
        p = clat_us(results["passthrough"][job], "mean")
        s = clat_us(results["screenfs"][job], "mean")
        lines.append(f"| `{job}` | {n:.3f} | {p:.3f} | {s:.3f} | {ratio(n,p):.2f}x | {ratio(p,s):.2f}x |")
    counters = perf["counters"]
    split_rows = []
    for name, parent in [("read_handle_snapshot", "fuse_op.read"), ("read_guard_path", "fuse_op.read"), ("read_io", "fuse_op.read"), ("write_handle_snapshot", "fuse_op.write"), ("write_guard_mutation", "fuse_op.write"), ("write_io", "fuse_op.write")]:
        c = counters.get(name, {})
        p = counters.get(parent, {})
        share = (c.get("total_ns", 0) / p.get("total_ns", 1) * 100) if p.get("total_ns") else 0
        split_rows.append((name, c.get("count", 0), c.get("avg_ns", 0), share, parent))
    lines += ["", "## ScreenFS perf split excerpt", "", "`managed-fio-attribution-perf-split.json` preserves the full parsed stderr counter set, including count-only metrics such as `matcher_candidates`, `readdir*_attr_generation_entries`, and `invalidations`.", "", "| counter | count | avg ns | share |", "| --- | ---: | ---: | ---: |"]
    for name, count, avg, share, parent in split_rows:
        lines.append(f"| `{name}` | {count} | {avg} | {share:.1f}% of `{parent}` |")
    lines += ["", "## Boxplot style contract", "", "`managed-fio-attribution-boxplot.svg` is intentionally kept in the original fixed-scale style so refreshed artifacts remain visually comparable across runs:", "", "- canvas: `1600x900` SVG, with `3200x1800` PNG raster output", "- y-axis: linear completion latency in µs, fixed `0..250` tick range", "- series colors: native gray, managed passthrough blue, ScreenFS red", "- markers: p95 orange dot, mean black diamond; box is p25/p50/p75 and whiskers are p1/p99", "- rendering: regenerate PNG from SVG with `scripts/managed-fio-attribution.py --render-only --png-scale 2` or `rsvg-convert -w 3200 -h 1800 docs/artifacts/managed-fio-attribution-boxplot.svg -o docs/artifacts/managed-fio-attribution-boxplot.png`", "", "Do not switch this artifact to log scale or a different layout unless the summary and README explicitly call out the visual break from prior artifacts.", "", "## Provenance", "", "See `managed-fio-attribution-env.json` for command lines, binary SHA256 values, git status, policy args, and source/mount paths captured during the run.", ""]
    md.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--prefix", type=Path, default=DEFAULT_PREFIX)
    parser.add_argument("--job", type=Path, default=ARTIFACTS / "current-fio-attribution.job")
    parser.add_argument("--png-scale", type=int, default=2)
    parser.add_argument("--render-only", action="store_true", help="Regenerate summary/boxplot SVG/PNG/MD from existing JSON/log artifacts.")
    args = parser.parse_args()
    prefix = args.prefix
    ARTIFACTS.mkdir(parents=True, exist_ok=True)

    native_json = prefix.with_name(prefix.name + "-native.json")
    passthrough_json = prefix.with_name(prefix.name + "-passthrough.json")
    screenfs_json = prefix.with_name(prefix.name + "-screenfs.json")
    pass_stderr = prefix.with_name(prefix.name + "-passthrough.stderr.log")
    pass_stdout = prefix.with_name(prefix.name + "-passthrough.stdout.log")
    screen_stderr = prefix.with_name(prefix.name + "-screenfs.stderr.log")
    screen_stdout = prefix.with_name(prefix.name + "-screenfs.stdout.log")
    perf_split = prefix.with_name(prefix.name + "-perf-split.json")
    env_json = prefix.with_name(prefix.name + "-env.json")

    if not args.render_only:
        if os.geteuid() == 0:
            raise SystemExit("refusing to run managed fio attribution as root")
        run(["cargo", "build", "--release", "--features", "perf-counters"])
        run(["cargo", "build", "--release", "--manifest-path", "contrib/fractal-passthrough/Cargo.toml"])
        screenfs_bin = REPO / "target" / "release" / "screenfs"
        passthrough_bin = REPO / "contrib" / "fractal-passthrough" / "target" / "release" / "fractal_passthrough"
        with tempfile.TemporaryDirectory(prefix="screenfs-managed-attr.") as td:
            root = Path(td)
            source = root / "source"
            native_dir = source / "native"
            pass_dir = source / "passthrough"
            screen_dir = source / "screenfs"
            for d in [native_dir, pass_dir, screen_dir, source / "hidden", source / "readonly"]:
                d.mkdir(parents=True, exist_ok=True)
            pass_mount = root / "passthrough-mount"
            screen_mount = root / "screenfs-mount"
            pass_mount.mkdir(); screen_mount.mkdir()
            run(["fio", "--directory", str(native_dir), "--output-format=json", "--output", str(native_json), str(args.job)])
            with pass_stdout.open("w", encoding="utf-8") as so, pass_stderr.open("w", encoding="utf-8") as se:
                p = subprocess.Popen([str(passthrough_bin), str(source), str(pass_mount)], cwd=REPO, text=True, stdout=so, stderr=se)
                try:
                    wait_mounted(pass_mount)
                    write_fstype(pass_mount, prefix.with_name(prefix.name + "-passthrough-fstype.txt"))
                    run(["fio", "--directory", str(pass_mount / "passthrough"), "--output-format=json", "--output", str(passthrough_json), str(args.job)])
                finally:
                    ensure_clean_unmount(pass_mount)
                    p.wait(timeout=10)
            screen_cmd = [str(screenfs_bin), str(source), str(screen_mount), "--visibility-default", "visible", "--hidden", "/hidden", "--mutability-default", "writable", "--readonly", "/readonly"]
            with screen_stdout.open("w", encoding="utf-8") as so, screen_stderr.open("w", encoding="utf-8") as se:
                p = subprocess.Popen(screen_cmd, cwd=REPO, text=True, stdout=so, stderr=se)
                try:
                    wait_mounted(screen_mount)
                    write_fstype(screen_mount, prefix.with_name(prefix.name + "-screenfs-fstype.txt"))
                    run(["fio", "--directory", str(screen_mount / "screenfs"), "--output-format=json", "--output", str(screenfs_json), str(args.job)])
                finally:
                    ensure_clean_unmount(screen_mount)
                    p.wait(timeout=10)
            outputs = [f"docs/artifacts/{prefix.name}-{suffix}" for suffix in ["env.json", "native.json", "passthrough-fstype.txt", "passthrough.json", "passthrough.stderr.log", "passthrough.stdout.log", "perf-split.json", "screenfs-fstype.txt", "screenfs.json", "screenfs.stderr.log", "screenfs.stdout.log", "summary.md", "summary.png", "summary.svg", "boxplot.png", "boxplot.svg"]]
            env = {
                "timestamp_utc": dt.datetime.now(dt.timezone.utc).isoformat().replace("+00:00", "Z"),
                "kernel": out(["uname", "-r"]),
                "fio_version": out(["fio", "--version"]),
                "fusermount3_version": out(["fusermount3", "--version"]),
                "git_head": out(["git", "rev-parse", "HEAD"]),
                "git_status_short": out(["git", "status", "--short"]),
                "git_worktree_clean": out(["git", "status", "--short"]) == "",
                "passthrough_source": "contrib/fractal-passthrough",
                "passthrough_binary": str(passthrough_bin),
                "passthrough_binary_sha256": sha256(passthrough_bin),
                "screenfs_bin": str(screenfs_bin),
                "screenfs_bin_sha256": sha256(screenfs_bin),
                "source_root": str(source),
                "commands": {"native_fio": ["fio", "--directory", str(native_dir), "--output-format=json", "--output", str(native_json), str(args.job)], "passthrough_mount": [str(passthrough_bin), str(source), str(pass_mount)], "passthrough_fio": ["fio", "--directory", str(pass_mount / "passthrough"), "--output-format=json", "--output", str(passthrough_json), str(args.job)], "screenfs_mount": screen_cmd, "screenfs_fio": ["fio", "--directory", str(screen_mount / "screenfs"), "--output-format=json", "--output", str(screenfs_json), str(args.job)]},
                "outputs": outputs,
                "policy_shape": "custom-unsafe-policy",
                "policy_label": "managed-fio-root-carveouts",
                "policy_note": "Explicit fio attribution policy uses --hidden /hidden and --readonly /readonly; it is not the official harness fallback-unsafe-policy preset, whose carve-outs live under /.screenfs-bench/.",
                "screenfs_policy_args": ["--visibility-default", "visible", "--hidden", "/hidden", "--mutability-default", "writable", "--readonly", "/readonly"],
            }
            env_json.write_text(json.dumps(env, indent=2) + "\n", encoding="utf-8")
    else:
        env = json.loads(env_json.read_text(encoding="utf-8"))

    results = {"native": fio_by_job(native_json), "passthrough": fio_by_job(passthrough_json), "screenfs": fio_by_job(screenfs_json)}
    perf = parse_perf(screen_stderr, perf_split)
    render_summary_svg(results, prefix.with_name(prefix.name + "-summary.svg"))
    render_boxplot_svg(results, prefix.with_name(prefix.name + "-boxplot.svg"))
    write_summary(results, perf, env, prefix)
    render_png(prefix.with_name(prefix.name + "-summary.svg"), prefix.with_name(prefix.name + "-summary.png"), args.png_scale)
    render_png(prefix.with_name(prefix.name + "-boxplot.svg"), prefix.with_name(prefix.name + "-boxplot.png"), args.png_scale)

    # Keep provenance in sync with the final rendered artifact set and any
    # generator-tool changes made before this script is run.
    env["git_status_short"] = out(["git", "status", "--short"])
    env["git_worktree_clean"] = env["git_status_short"] == ""
    env_json.write_text(json.dumps(env, indent=2) + "\n", encoding="utf-8")

    print(f"wrote {prefix.name} artifacts", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
