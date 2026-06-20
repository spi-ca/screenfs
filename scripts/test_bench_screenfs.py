#!/usr/bin/env python3
import argparse
import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

MODULE_PATH = Path(__file__).with_name("bench-screenfs.py")
spec = importlib.util.spec_from_file_location("bench_screenfs", MODULE_PATH)
bench = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(bench)


def make_args(**overrides: object) -> argparse.Namespace:
    values: dict[str, object] = {
        "read_mib": 1,
        "write_mib": 1,
        "small_io_bytes": 64,
        "small_io_ops": 4,
        "sync_bytes": 64,
        "sync_ops": 3,
        "rand_io_ops": 4,
        "concurrency_workers": 4,
        "open_read_close_ops": 5,
        "metadata_ops": 5,
        "sync_4k_fsync_every": 2,
        "small_files": 4,
        "dir_entries": 3,
        "hidden_misses": 2,
        "matcher_extra_rules": 0,
        "matcher_misses": 2,
        "symlink_parent_mutations": 3,
        "iterations": 1,
        "warmups": 1,
        "cache_control": "warm",
        "perf_counters": False,
    }
    values.update(overrides)
    return argparse.Namespace(**values)


class ParseArgsTests(unittest.TestCase):
    def test_parse_args_defaults_cache_control_to_warm(self) -> None:
        with mock.patch.object(sys, "argv", ["bench-screenfs.py"]):
            args = bench.parse_args()

        self.assertEqual(args.cache_control, "warm")

    def test_parse_args_accepts_cache_control_override(self) -> None:
        with mock.patch.object(
            sys,
            "argv",
            [
                "bench-screenfs.py",
                "--cache-control",
                "posix-fadvise-read-fixture",
                "--concurrency-workers",
                "7",
                "--workload-set",
                "read-write-concurrency",
            ],
        ):
            args = bench.parse_args()

        self.assertEqual(args.cache_control, "posix-fadvise-read-fixture")
        self.assertEqual(args.concurrency_workers, 7)
        self.assertEqual(args.workload_set, "read-write-concurrency")


class CacheControlSupportTests(unittest.TestCase):
    def test_ensure_cache_control_supported_allows_default_warm_mode(self) -> None:
        bench.ensure_cache_control_supported(make_args())

    def test_ensure_cache_control_supported_fails_when_posix_fadvise_is_unavailable(self) -> None:
        with mock.patch.object(bench.os, "posix_fadvise", None, create=True), mock.patch.object(
            bench.os,
            "POSIX_FADV_DONTNEED",
            None,
            create=True,
        ):
            with self.assertRaises(SystemExit) as ctx:
                bench.ensure_cache_control_supported(make_args(cache_control="posix-fadvise-read-fixture"))

        self.assertIn("--cache-control=posix-fadvise-read-fixture requires", str(ctx.exception))


class PerfSummaryTests(unittest.TestCase):
    def test_time_one_runs_cleanup_after_timed_body(self) -> None:
        events: list[str] = []

        def workload(_root: Path, _args: argparse.Namespace, _side: str):
            events.append("body")

            def cleanup() -> None:
                events.append("cleanup")

            return cleanup

        elapsed = bench.time_one(workload, Path("/tmp"), make_args(), "native")

        self.assertGreaterEqual(elapsed, 0)
        self.assertEqual(events, ["body", "cleanup"])

    def test_parse_perf_summary_extracts_metrics(self) -> None:
        stderr = "mounting screenfs\nscreenfs perf counters:\n  fuse_op.lookup: count=2 total_ns=10 avg_ns=5 max_ns=6\n  matcher_candidates: count=7\n  invalidations: count=1 invalidated_entries=2 evicted_entries=1 scanned_entries=3\n"
        summary = bench.parse_perf_summary(stderr)
        self.assertIsNotNone(summary)
        assert summary is not None
        self.assertIn("screenfs perf counters:", summary["raw"])
        self.assertEqual(summary["metrics"]["fuse_op.lookup"]["count"], 2)
        self.assertEqual(summary["metrics"]["matcher_candidates"]["count"], 7)
        self.assertEqual(summary["metrics"]["invalidations"]["evicted_entries"], 1)
        self.assertEqual(summary["metrics"]["invalidations"]["scanned_entries"], 3)

    def test_markdown_report_includes_perf_summary(self) -> None:
        result = {
            "timestamp": "2026-01-01T00:00:00+00:00",
            "harness": {"command_line": "bench"},
            "environment": {
                "git_revision": "abc",
                "git_worktree_clean": True,
                "git_status_porcelain": "",
            },
            "policy": {
                "requested_preset": "fallback-unsafe-policy",
                "effective_bucket": "fallback-unsafe-policy",
                "label": "fallback-unsafe-policy",
                "fast_path_cache_eligible": False,
                "notes": [],
            },
            "workloads": {
                "mode": "named-set",
                "requested_set": "default",
                "comparable": [],
                "screenfs_only": [],
                "skipped": [],
            },
            "screenfs": {
                "binary": "target/release/screenfs",
                "binary_sha256": "sha",
                "perf_summary": {"raw": "screenfs perf counters:\n  matcher_candidates: count=1", "metrics": {}},
            },
            "parameters": {"cache_control": "warm", "iterations": 1, "warmups": 1, "concurrency_workers": 4},
            "cache_control": {
                "method": "warm",
                "scope": bench.cache_control_scope("warm"),
                "timing_applied": False,
                "support_checked": True,
                "notes": [
                    "cache-control selection is recorded in provenance and support-checked, but this helper skeleton does not yet apply cache eviction inside workload timing"
                ],
            },
            "comparisons": {},
            "screenfs_only": [],
        }
        markdown = bench.markdown_report(result)
        self.assertIn("- cache_control: `warm`", markdown)
        self.assertIn("- cache_control_timing_applied: `False`", markdown)
        self.assertIn("- concurrency_workers: `4`", markdown)
        self.assertIn("## Perf counters", markdown)
        self.assertIn("matcher_candidates: count=1", markdown)

    def test_markdown_report_includes_policy_notes_and_skipped_workloads(self) -> None:
        result = {
            "timestamp": "2026-01-01T00:00:00+00:00",
            "harness": {"command_line": "bench"},
            "environment": {
                "git_revision": "abc",
                "git_worktree_clean": True,
                "git_status_porcelain": "",
            },
            "policy": {
                "requested_preset": "fast-path-cache-eligible",
                "effective_bucket": "custom-unsafe-policy",
                "label": "fast-path-cache-eligible-extra-policy-args",
                "fast_path_cache_eligible": False,
                "notes": ["extra_screenfs_arg included policy-shaping flags (--config)"],
            },
            "workloads": {
                "mode": "named-set",
                "requested_set": "default",
                "comparable": ["seq_read"],
                "screenfs_only": [],
                "skipped": [{"name": "hidden_stat_miss", "reason": "hidden workload disabled"}],
            },
            "screenfs": {
                "binary": "target/release/screenfs",
                "binary_sha256": "sha",
                "perf_summary": None,
            },
            "parameters": {"cache_control": "warm", "iterations": 1, "warmups": 1, "concurrency_workers": 4},
            "comparisons": {},
            "screenfs_only": [],
        }

        markdown = bench.markdown_report(result)

        self.assertIn("- policy_notes: `extra_screenfs_arg included policy-shaping flags (--config)`", markdown)
        self.assertIn("## Skipped workloads", markdown)
        self.assertIn("- `hidden_stat_miss`: hidden workload disabled", markdown)

    def test_markdown_report_includes_screenfs_source_provenance(self) -> None:
        result = {
            "timestamp": "2026-01-01T00:00:00+00:00",
            "harness": {"command_line": "bench --screenfs-source-root /tmp/before"},
            "environment": {
                "harness_repo_root": "/home/spi-ca/Codebase/screenfs",
                "git_revision": "harness-rev",
                "git_worktree_clean": True,
                "git_status_porcelain": "",
            },
            "policy": {
                "requested_preset": "fast-path-cache-eligible",
                "effective_bucket": "fast-path-cache-eligible",
                "label": "fast-path-cache-eligible",
                "fast_path_cache_eligible": True,
                "notes": [],
            },
            "workloads": {
                "mode": "named-set",
                "requested_set": "per-open-cache-minimum",
                "comparable": [],
                "screenfs_only": [],
                "skipped": [],
            },
            "screenfs": {
                "binary": "/tmp/before/target/release/screenfs",
                "binary_sha256": "sha",
                "source": {
                    "source_root": "/tmp/before",
                    "source_root_origin": "cli",
                    "git_revision": "before-rev",
                    "git_status_porcelain": " M src/fs.rs",
                    "git_worktree_clean": False,
                },
                "perf_summary": None,
            },
            "parameters": {"cache_control": "warm", "iterations": 1, "warmups": 1, "concurrency_workers": 4},
            "comparisons": {},
            "screenfs_only": [],
        }

        markdown = bench.markdown_report(result)

        self.assertIn("- harness_repo_root: `/home/spi-ca/Codebase/screenfs`", markdown)
        self.assertIn("- screenfs_source_root: `/tmp/before`", markdown)
        self.assertIn("- screenfs_source_root_origin: `cli`", markdown)
        self.assertIn("- screenfs_source_git: `before-rev`", markdown)
        self.assertIn("- screenfs_source_git_worktree_clean: `False`", markdown)
        self.assertIn("- screenfs_source_git_dirty_status: ` M src/fs.rs`", markdown)


class SourceProvenanceTests(unittest.TestCase):
    def test_infer_source_root_from_binary_prefers_cargo_workspace_root(self) -> None:
        with tempfile.TemporaryDirectory() as tempdir:
            repo_root = Path(tempdir) / "before"
            binary = repo_root / "target" / "release" / "screenfs"
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b"screenfs")
            (repo_root / "Cargo.toml").write_text("[package]\nname = \"screenfs\"\nversion = \"0.0.0\"\n", encoding="utf-8")

            inferred = bench.infer_source_root_from_binary(binary)

            self.assertEqual(inferred, repo_root)


class ResultAssemblyTests(unittest.TestCase):
    def test_build_benchmark_result_records_provenance_shape(self) -> None:
        args = make_args(
            matcher_extra_rules=2,
            concurrency_workers=6,
            cache_control="posix-fadvise-read-fixture",
            extra_screenfs_arg=["--config", "matrix.yaml"],
            perf_counters=True,
        )
        policy = {
            "requested_preset": "fallback-unsafe-policy",
            "effective_bucket": "custom-unsafe-policy",
            "manual_policy_override_flags": ["--config"],
        }
        workloads = {
            "mode": "named-set",
            "requested_set": "default",
            "requested_names": ["seq_read"],
            "comparable": ["seq_read"],
            "screenfs_only": ["hidden_stat_miss"],
            "skipped": [],
        }
        stderr = "x" * 4500 + "tail"
        result = bench.build_benchmark_result(
            args=args,
            timestamp="2026-01-01T00:00:00+00:00",
            harness_command=["python3", "scripts/bench-screenfs.py", "--policy-preset", "fallback-unsafe-policy"],
            environment={"git_revision": "abc123", "git_worktree_clean": True, "git_status_porcelain": ""},
            workdir=Path("/tmp/workdir"),
            source=Path("/tmp/source"),
            mount=Path("/tmp/mount"),
            policy=policy,
            workloads=workloads,
            screenfs_bin=Path("target/release/screenfs"),
            screenfs_binary_sha256="sha256",
            screenfs_source={
                "source_root": "/tmp/before",
                "source_root_origin": "cli",
                "git_revision": "before-rev",
                "git_status_porcelain": " M src/fs.rs",
                "git_worktree_clean": False,
            },
            command=["target/release/screenfs", "/tmp/source", "/tmp/mount"],
            stderr_path=Path("/tmp/workdir/screenfs.stderr.log"),
            screenfs_stderr=stderr,
            perf_summary={"raw": "screenfs perf counters:", "metrics": {}},
            native_results=[],
            mounted_results=[],
            screenfs_only=[],
            comparisons={},
        )

        self.assertEqual(result["schema"], "screenfs-benchmark-v2")
        self.assertEqual(result["policy"]["manual_policy_override_flags"], ["--config"])
        self.assertEqual(result["workloads"]["screenfs_only"], ["hidden_stat_miss"])
        self.assertEqual(result["parameters"]["cache_control"], "posix-fadvise-read-fixture")
        self.assertEqual(result["parameters"]["matcher_extra_rules"], 2)
        self.assertEqual(result["parameters"]["concurrency_workers"], 6)
        self.assertFalse(result["cache_control"]["timing_applied"])
        self.assertTrue(result["screenfs"]["perf_counters_enabled"])
        self.assertEqual(result["screenfs"]["stderr_preview"], stderr[-4000:])
        self.assertEqual(result["screenfs"]["command"], ["target/release/screenfs", "/tmp/source", "/tmp/mount"])
        self.assertEqual(result["screenfs"]["source"]["source_root"], "/tmp/before")
        self.assertEqual(result["screenfs"]["source"]["source_root_origin"], "cli")


if __name__ == "__main__":
    unittest.main()
