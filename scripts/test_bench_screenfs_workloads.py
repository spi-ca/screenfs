#\!/usr/bin/env python3
import argparse
import importlib.util
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).with_name("bench-screenfs.py")
spec = importlib.util.spec_from_file_location("bench_screenfs", MODULE_PATH)
bench = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(bench)


def make_args(**overrides: object) -> argparse.Namespace:
    values: dict[str, object] = {
        "policy_preset": "fallback-unsafe-policy",
        "policy_label": None,
        "matcher_extra_rules": 0,
        "extra_screenfs_arg": [],
        "workload_set": "default",
        "workload": [],
        "read_mib": 1,
        "write_mib": 1,
        "small_io_bytes": 64,
        "small_io_ops": 4,
        "sync_bytes": 64,
        "sync_ops": 3,
        "small_files": 4,
        "dir_entries": 3,
        "rand_io_ops": 4,
        "open_read_close_ops": 5,
        "metadata_ops": 5,
        "sync_4k_fsync_every": 2,
        "hidden_misses": 2,
        "matcher_misses": 2,
        "symlink_parent_mutations": 3,
        "iterations": 1,
        "warmups": 1,
    }
    values.update(overrides)
    return argparse.Namespace(**values)


class PolicyResolutionTests(unittest.TestCase):
    def test_fallback_default_preserves_historical_policy(self) -> None:
        policy = bench.resolve_policy(make_args())

        self.assertEqual(policy["requested_preset"], "fallback-unsafe-policy")
        self.assertEqual(policy["requested_bucket"], "fallback-unsafe-policy")
        self.assertEqual(policy["effective_bucket"], "fallback-unsafe-policy")
        self.assertEqual(policy["label"], "fallback-unsafe-policy")
        self.assertEqual(
            policy["built_in_screenfs_args"],
            [
                "--visibility-default",
                "visible",
                "--hidden",
                "/.screenfs-bench/hidden",
                "--mutability-default",
                "writable",
                "--readonly",
                "/.screenfs-bench/readonly",
            ],
        )
        self.assertEqual(policy["hidden_paths"], ["/.screenfs-bench/hidden"])
        self.assertEqual(policy["readonly_paths"], ["/.screenfs-bench/readonly"])
        self.assertFalse(policy["fast_path_cache_eligible"])
        self.assertTrue(policy["supports_hidden_stat_miss"])
        self.assertFalse(policy["supports_matcher_hidden_stat_miss"])

    def test_fast_path_cache_eligible_preset_has_no_carve_outs(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))

        self.assertEqual(policy["effective_bucket"], "fast-path-cache-eligible")
        self.assertEqual(policy["label"], "fast-path-cache-eligible")
        self.assertEqual(
            policy["built_in_screenfs_args"],
            ["--visibility-default", "visible", "--mutability-default", "writable"],
        )
        self.assertEqual(policy["hidden_paths"], [])
        self.assertEqual(policy["readonly_paths"], [])
        self.assertTrue(policy["fast_path_cache_eligible"])
        self.assertFalse(policy["supports_hidden_stat_miss"])
        self.assertFalse(policy["supports_matcher_hidden_stat_miss"])
        self.assertIn(
            "effective policy remains cache-eligible for the current per-open read/write fast path",
            policy["notes"],
        )

    def test_policy_shaping_extra_args_demote_cache_eligible_preset(self) -> None:
        policy = bench.resolve_policy(
            make_args(
                policy_preset="fast-path-cache-eligible",
                extra_screenfs_arg=["--hidden", "/tmp/custom-hidden"],
            )
        )

        self.assertEqual(policy["effective_bucket"], "custom-unsafe-policy")
        self.assertEqual(policy["label"], "fast-path-cache-eligible-extra-policy-args")
        self.assertEqual(policy["manual_policy_override_flags"], ["--hidden"])
        self.assertEqual(policy["hidden_paths"], ["/tmp/custom-hidden"])
        self.assertFalse(policy["fast_path_cache_eligible"])

    def test_config_extra_arg_demotes_cache_eligible_preset_and_records_provenance(self) -> None:
        policy = bench.resolve_policy(
            make_args(
                policy_preset="fast-path-cache-eligible",
                extra_screenfs_arg=["--config", "screenfs.yaml"],
            )
        )

        self.assertEqual(policy["effective_bucket"], "custom-unsafe-policy")
        self.assertEqual(policy["label"], "fast-path-cache-eligible-extra-policy-args")
        self.assertEqual(policy["manual_policy_override_flags"], ["--config"])
        self.assertFalse(policy["fast_path_cache_eligible"])
        self.assertFalse(policy["supports_hidden_stat_miss"])
        self.assertIn("policy-shaping flags (--config)", policy["notes"][0])

    def test_broader_hidden_rule_supports_hidden_stat_miss_even_with_config(self) -> None:
        policy = bench.resolve_policy(
            make_args(
                policy_preset="fast-path-cache-eligible",
                extra_screenfs_arg=["--config", "screenfs.yaml", "--hidden", "/.screenfs-bench"],
            )
        )

        self.assertEqual(policy["manual_policy_override_flags"], ["--config", "--hidden"])
        self.assertEqual(policy["hidden_paths"], ["/.screenfs-bench"])
        self.assertTrue(policy["supports_hidden_stat_miss"])

    def test_matcher_extra_rules_use_named_custom_unsafe_entry(self) -> None:
        policy = bench.resolve_policy(make_args(matcher_extra_rules=2))

        self.assertEqual(policy["requested_bucket"], "fallback-unsafe-policy")
        self.assertEqual(policy["effective_bucket"], "custom-unsafe-policy")
        self.assertEqual(policy["label"], "fallback-unsafe-policy-matcher2")
        self.assertTrue(policy["supports_matcher_hidden_stat_miss"])
        self.assertIn(
            "matcher_extra_rules appended synthetic hidden/readonly carve-outs and activates matcher_hidden_stat_miss when that workload is selected",
            policy["notes"],
        )


class WorkloadSelectionTests(unittest.TestCase):
    def test_default_named_set_skips_matcher_workload_without_extra_rules(self) -> None:
        policy = bench.resolve_policy(make_args())
        workloads = bench.resolve_workloads(make_args(), policy)

        self.assertEqual(workloads["mode"], "named-set")
        self.assertEqual(workloads["requested_set"], "default")
        self.assertEqual(workloads["comparable"], bench.DEFAULT_COMPARABLE_WORKLOADS)
        self.assertEqual(workloads["screenfs_only"], ["hidden_stat_miss", "symlink_parent_mkdir_rmdir"])
        self.assertEqual(
            workloads["skipped"],
            [{"name": "matcher_hidden_stat_miss", "reason": "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"}],
        )

    def test_per_open_cache_minimum_named_set_selects_only_minimum_comparables(self) -> None:
        args = make_args(policy_preset="fast-path-cache-eligible", workload_set="per-open-cache-minimum")
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["mode"], "named-set")
        self.assertEqual(workloads["requested_set"], "per-open-cache-minimum")
        self.assertEqual(workloads["comparable"], bench.PER_OPEN_CACHE_MINIMUM_COMPARABLE_WORKLOADS)
        self.assertEqual(workloads["screenfs_only"], [])
        self.assertEqual(workloads["skipped"], [])

    def test_next_candidate_named_sets_select_expected_workloads(self) -> None:
        cases = {
            "metadata-open-path": bench.METADATA_OPEN_PATH_COMPARABLE_WORKLOADS,
            "sync-surface": bench.SYNC_SURFACE_COMPARABLE_WORKLOADS,
            "directory-surface": bench.DIRECTORY_SURFACE_COMPARABLE_WORKLOADS,
        }
        policy = bench.resolve_policy(make_args())
        for workload_set, expected in cases.items():
            with self.subTest(workload_set=workload_set):
                workloads = bench.resolve_workloads(make_args(workload_set=workload_set), policy)
                self.assertEqual(workloads["comparable"], expected)
                self.assertEqual(workloads["screenfs_only"], [])
                self.assertEqual(workloads["skipped"], [])

    def test_policy_heavy_matrix_skips_matcher_without_extra_rules(self) -> None:
        args = make_args(workload_set="policy-heavy-matrix")
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], ["metadata_lookup", "metadata_getattr", "metadata_access"])
        self.assertEqual(workloads["screenfs_only"], [])
        self.assertEqual(
            workloads["skipped"],
            [{"name": "matcher_hidden_stat_miss", "reason": "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"}],
        )

    def test_policy_heavy_matrix_includes_matcher_when_rules_exist(self) -> None:
        args = make_args(workload_set="policy-heavy-matrix", matcher_extra_rules=32)
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], ["metadata_lookup", "metadata_getattr", "metadata_access"])
        self.assertEqual(workloads["screenfs_only"], ["matcher_hidden_stat_miss"])
        self.assertEqual(workloads["skipped"], [])

    def test_explicit_workloads_preserve_order_and_enable_matcher_when_rules_exist(self) -> None:
        args = make_args(
            matcher_extra_rules=2,
            workload=["rand_read_4k", "matcher_hidden_stat_miss", "rand_read_4k", "small_open_read_close"],
        )
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["mode"], "explicit")
        self.assertIsNone(workloads["requested_set"])
        self.assertEqual(
            workloads["requested_names"],
            ["rand_read_4k", "matcher_hidden_stat_miss", "small_open_read_close"],
        )
        self.assertEqual(workloads["comparable"], ["rand_read_4k", "small_open_read_close"])
        self.assertEqual(workloads["screenfs_only"], ["matcher_hidden_stat_miss"])
        self.assertEqual(workloads["skipped"], [])

    def test_explicit_incompatible_screenfs_only_workload_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))
        with self.assertRaisesRegex(SystemExit, "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"):
            bench.resolve_workloads(make_args(policy_preset="fast-path-cache-eligible", workload=["matcher_hidden_stat_miss"]), policy)

    def test_explicit_hidden_stat_miss_without_hidden_rule_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))
        with self.assertRaisesRegex(SystemExit, "hidden_stat_miss requires a hidden /\\.screenfs-bench/hidden rule"):
            bench.resolve_workloads(make_args(policy_preset="fast-path-cache-eligible", workload=["hidden_stat_miss"]), policy)


class MarkdownProvenanceTests(unittest.TestCase):
    def test_markdown_report_includes_policy_and_workload_provenance(self) -> None:
        args = make_args(policy_preset="fast-path-cache-eligible", workload_set="per-open-cache-minimum")
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)
        result = {
            "timestamp": "2026-01-01T00:00:00+00:00",
            "harness": {"command_line": "scripts/bench-screenfs.py --policy-preset fast-path-cache-eligible"},
            "environment": {
                "git_revision": "abc123",
                "git_worktree_clean": True,
                "git_status_porcelain": "",
            },
            "screenfs": {
                "binary": "target/release/screenfs",
                "binary_sha256": "sha",
                "perf_summary": None,
            },
            "parameters": {"iterations": 1, "warmups": 1},
            "policy": policy,
            "workloads": workloads,
            "comparisons": {},
            "screenfs_only": [],
        }

        markdown = bench.markdown_report(result)

        self.assertIn("- policy_preset: `fast-path-cache-eligible`", markdown)
        self.assertIn("- policy_bucket: `fast-path-cache-eligible`", markdown)
        self.assertIn("- policy_label: `fast-path-cache-eligible`", markdown)
        self.assertIn("- fast_path_cache_eligible: `True`", markdown)
        self.assertIn("- workload_selection: `named-set`", markdown)
        self.assertIn("- workload_set: `per-open-cache-minimum`", markdown)
        self.assertIn(
            "- comparable_workloads: `rand_read_4k, rand_write_4k, sync_write_4k, small_open_read_close`",
            markdown,
        )
        self.assertIn("- screenfs_only_workloads: `(none)`", markdown)


class WorkloadFunctionTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.args = make_args()
        bench.prepare_fixture(self.root, self.args)

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def test_per_open_cache_minimum_workloads_run_on_small_fixture(self) -> None:
        for workload_name in bench.PER_OPEN_CACHE_MINIMUM_COMPARABLE_WORKLOADS:
            with self.subTest(workload=workload_name):
                bench.WORKLOADS[workload_name](self.root, self.args, "native")

        write_root = self.root / ".screenfs-bench"
        self.assertFalse((write_root / "write-native" / "rand-write-4k.bin").exists())
        self.assertFalse((write_root / "write-native" / "sync-write-4k.bin").exists())

    def test_metadata_sync_and_directory_workloads_run_on_small_fixture(self) -> None:
        for workload_name in (
            bench.METADATA_OPEN_PATH_COMPARABLE_WORKLOADS
            + bench.SYNC_SURFACE_COMPARABLE_WORKLOADS
            + bench.DIRECTORY_SURFACE_COMPARABLE_WORKLOADS
        ):
            with self.subTest(workload=workload_name):
                bench.WORKLOADS[workload_name](self.root, self.args, "native")

    def test_next_candidate_named_sets_execute_selected_workloads(self) -> None:
        for workload_set in ("metadata-open-path", "sync-surface", "directory-surface"):
            args = make_args(workload_set=workload_set)
            policy = bench.resolve_policy(args)
            workloads = bench.resolve_workloads(args, policy)

            self.assertEqual(workloads["screenfs_only"], [])
            self.assertEqual(workloads["skipped"], [])

            for workload_name in workloads["comparable"]:
                with self.subTest(workload_set=workload_set, workload=workload_name):
                    bench.WORKLOADS[workload_name](self.root, args, "native")

    def test_symlink_parent_mutation_workload_leaves_no_children_behind(self) -> None:
        bench.symlink_parent_mkdir_rmdir(self.root, self.args, "native")

        real_dir = self.root / ".screenfs-bench" / "symlink-parent" / "real"
        self.assertEqual(list(real_dir.iterdir()), [])


if __name__ == "__main__":
    unittest.main()
