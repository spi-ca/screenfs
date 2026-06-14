#!/usr/bin/env python3
import importlib.util
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("bench-screenfs.py")
spec = importlib.util.spec_from_file_location("bench_screenfs", MODULE_PATH)
bench = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(bench)


class PerfSummaryTests(unittest.TestCase):
    def test_parse_perf_summary_extracts_metrics(self) -> None:
        stderr = "mounting screenfs\nscreenfs perf counters:\n  fuse_op.lookup: count=2 total_ns=10 avg_ns=5 max_ns=6\n  matcher_candidates: count=7\n  invalidations: count=1 invalidated_entries=2 evicted_entries=1\n"
        summary = bench.parse_perf_summary(stderr)
        self.assertIsNotNone(summary)
        assert summary is not None
        self.assertIn("screenfs perf counters:", summary["raw"])
        self.assertEqual(summary["metrics"]["fuse_op.lookup"]["count"], 2)
        self.assertEqual(summary["metrics"]["matcher_candidates"]["count"], 7)
        self.assertEqual(summary["metrics"]["invalidations"]["evicted_entries"], 1)

    def test_markdown_report_includes_perf_summary(self) -> None:
        result = {
            "timestamp": "2026-01-01T00:00:00+00:00",
            "harness": {"command_line": "bench"},
            "environment": {
                "git_revision": "abc",
                "git_worktree_clean": True,
                "git_status_porcelain": "",
            },
            "screenfs": {
                "binary": "target/release/screenfs",
                "binary_sha256": "sha",
                "perf_summary": {"raw": "screenfs perf counters:\n  matcher_candidates: count=1", "metrics": {}},
            },
            "parameters": {"iterations": 1, "warmups": 1},
            "comparisons": {},
            "screenfs_only": [],
        }
        markdown = bench.markdown_report(result)
        self.assertIn("## Perf counters", markdown)
        self.assertIn("matcher_candidates: count=1", markdown)


if __name__ == "__main__":
    unittest.main()
