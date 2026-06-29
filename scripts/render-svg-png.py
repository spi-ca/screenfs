#!/usr/bin/env python3
"""Render an SVG artifact to PNG at a requested integer scale using rsvg-convert."""

from __future__ import annotations

import argparse
import re
import subprocess
import tempfile
from pathlib import Path


def svg_size(svg: Path) -> tuple[int, int]:
    head = svg.read_text(encoding="utf-8", errors="replace")[:500]
    width = re.search(r'\bwidth="(\d+)"', head)
    height = re.search(r'\bheight="(\d+)"', head)
    if not width or not height:
        raise SystemExit(f"could not read integer width/height from {svg}")
    return int(width.group(1)), int(height.group(1))


def with_white_background(svg: Path) -> str:
    text = svg.read_text(encoding="utf-8", errors="replace")
    if '<rect width="100%" height="100%" fill="white"/>' in text[:1000]:
        return text
    return re.sub(r"(<svg\b[^>]*>)", r"\1\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>", text, count=1)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("svg", type=Path)
    parser.add_argument("png", type=Path)
    parser.add_argument("--scale", type=int, default=2)
    args = parser.parse_args()
    width, height = svg_size(args.svg)
    args.png.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile("w", suffix=".svg", encoding="utf-8", delete=False) as tmp:
        tmp.write(with_white_background(args.svg))
        tmp_path = Path(tmp.name)
    try:
        subprocess.run(
            ["rsvg-convert", "-w", str(width * args.scale), "-h", str(height * args.scale), str(tmp_path), "-o", str(args.png)],
            check=True,
        )
    finally:
        tmp_path.unlink(missing_ok=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
