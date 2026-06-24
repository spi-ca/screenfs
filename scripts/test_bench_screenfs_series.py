#!/usr/bin/env python3
import argparse
import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

MODULE_PATH = Path(__file__).with_name("bench-screenfs-series.py")
spec = importlib.util.spec_from_file_location("bench_screenfs_series", MODULE_PATH)
series = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(series)


class SeriesTests(unittest.TestCase):
    def test_pair_order_supports_alternating(self) -> None:
        self.assertEqual(series.pair_order("before-after", 2), ("before", "after"))
        self.assertEqual(series.pair_order("after-before", 1), ("after", "before"))
        self.assertEqual(series.pair_order("alternate", 1), ("before", "after"))
        self.assertEqual(series.pair_order("alternate", 2), ("after", "before"))

    def test_parse_args_accepts_arbitrary_summary_workload_names(self) -> None:
        args = series.parse_args(
            [
                "--before-bin",
                "/tmp/before",
                "--after-bin",
                "/tmp/after",
                "--output-dir",
                "/tmp/out",
                "--workload",
                "metadata_open",
                "--",
                "--iterations",
                "1",
            ]
        )

        self.assertEqual(args.workload, ["metadata_open"])

    def test_rejects_managed_passthrough_args(self) -> None:
        with self.assertRaises(SystemExit):
            series.reject_managed_args(["--screenfs-bin", "target/release/screenfs"])
        with self.assertRaises(SystemExit):
            series.reject_managed_args(["--output-json=/tmp/out.json"])

    def test_build_child_argv_adds_outputs_and_binary(self) -> None:
        args = argparse.Namespace(
            bench_script=Path("scripts/bench-screenfs.py"),
            before_bin=Path("/tmp/before"),
            after_bin=Path("/tmp/after"),
            before_label="base",
            after_label="candidate",
            output_dir=Path("/tmp/out"),
            stem="matcher",
            bench_args=["--iterations", "1", "--workload-set", "policy-heavy-matrix"],
        )
        argv, outputs = series.build_child_argv(args, 2, "after")

        self.assertIn("/tmp/after", argv)
        self.assertIn("--iterations", argv)
        self.assertEqual(outputs["json"], "/tmp/out/matcher-pair2-after-candidate.json")
        self.assertEqual(outputs["md"], "/tmp/out/matcher-pair2-after-candidate.md")
        self.assertEqual(outputs["svg"], "/tmp/out/matcher-pair2-after-candidate.svg")

    def test_summarize_pair_reads_mounted_and_screenfs_only(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            before = {
                "mounted": [
                    {"name": "metadata_lookup", "summary": {"p50_sec": 2.0, "p95_sec": 4.0, "p99_sec": 5.0}}
                ],
                "screenfs_only": [
                    {"name": "matcher_hidden_stat_miss", "summary": {"p50_sec": 10.0, "p95_sec": 20.0, "p99_sec": 25.0}}
                ],
            }
            after = {
                "mounted": [
                    {"name": "metadata_lookup", "summary": {"p50_sec": 1.0, "p95_sec": 2.0, "p99_sec": 2.5}}
                ],
                "screenfs_only": [
                    {"name": "matcher_hidden_stat_miss", "summary": {"p50_sec": 9.0, "p95_sec": 18.0, "p99_sec": 20.0}}
                ],
            }
            before_path = root / "before.json"
            after_path = root / "after.json"
            before_path.write_text(json.dumps(before))
            after_path.write_text(json.dumps(after))

            summary = series.summarize_pair(
                before_path,
                after_path,
                ["metadata_lookup", "matcher_hidden_stat_miss", "missing_workload"],
            )

        self.assertEqual(summary["ratios"]["metadata_lookup"]["p50_sec"], 0.5)
        self.assertEqual(summary["ratios"]["matcher_hidden_stat_miss"]["p99_sec"], 0.8)
        self.assertEqual(summary["missing_workloads"], ["missing_workload"])

    def test_infer_common_workloads_uses_child_json_when_no_workload_requested(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            before_path = root / "before.json"
            after_path = root / "after.json"
            before_path.write_text(json.dumps({"mounted": [{"name": "metadata_lookup"}], "screenfs_only": [{"name": "only_before"}]}))
            after_path.write_text(json.dumps({"mounted": [{"name": "metadata_lookup"}], "screenfs_only": [{"name": "only_after"}]}))

            self.assertEqual(series.infer_common_workloads(before_path, after_path), ["metadata_lookup"])

    def test_main_runs_children_and_writes_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            bench_script = root / "bench.py"
            bench_script.write_text("# fake")
            before_bin = root / "before"
            after_bin = root / "after"
            before_bin.write_text("before")
            after_bin.write_text("after")
            output_dir = root / "out"
            custom_dir = root / "custom" / "nested"

            def fake_run(argv: list[str]) -> None:
                json_path = Path(argv[argv.index("--output-json") + 1])
                multiplier = 2.0 if "after" in json_path.name else 1.0
                json_path.parent.mkdir(parents=True, exist_ok=True)
                json_path.write_text(
                    json.dumps(
                        {
                            "mounted": [
                                {
                                    "name": "metadata_lookup",
                                    "summary": {
                                        "p50_sec": multiplier,
                                        "p95_sec": multiplier * 2,
                                        "p99_sec": multiplier * 3,
                                    },
                                }
                            ],
                            "screenfs_only": [],
                        }
                    )
                )
                Path(argv[argv.index("--output-md") + 1]).write_text("md")
                Path(argv[argv.index("--output-svg") + 1]).write_text("svg")

            with mock.patch.object(series, "run_command", side_effect=fake_run):
                rc = series.main(
                    [
                        "--bench-script",
                        str(bench_script),
                        "--before-bin",
                        str(before_bin),
                        "--after-bin",
                        str(after_bin),
                        "--pairs",
                        "1",
                        "--output-dir",
                        str(output_dir),
                        "--stem",
                        "test",
                        "--manifest-json",
                        str(custom_dir / "manifest.json"),
                        "--summary-md",
                        str(custom_dir / "summary.md"),
                        "--",
                        "--iterations",
                        "1",
                    ]
                )

            self.assertEqual(rc, 0)
            manifest = json.loads((custom_dir / "manifest.json").read_text())
            self.assertEqual(len(manifest["runs"]), 2)
            self.assertTrue(manifest["command_line"].startswith(str(MODULE_PATH)))
            self.assertIn("--before-bin", manifest["command_line"])
            self.assertIn(str(before_bin), manifest["command_line"])
            self.assertEqual(manifest["pair_summaries"][0]["ratios"]["metadata_lookup"]["p50_sec"], 2.0)
            summary = (custom_dir / "summary.md").read_text()
            self.assertIn("2.000x", summary)


if __name__ == "__main__":
    unittest.main()
