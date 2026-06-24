#\!/usr/bin/env python3
import argparse
import importlib.util
import os
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
        "concurrency_workers": 4,
        "open_read_close_ops": 5,
        "metadata_ops": 5,
        "sync_4k_fsync_every": 2,
        "hidden_misses": 2,
        "matcher_misses": 2,
        "symlink_parent_mutations": 3,
        "iterations": 1,
        "warmups": 1,
        "cache_control": "warm",
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
        self.assertFalse(policy["supports_matcher_readonly_access_wok"])

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
        self.assertFalse(policy["supports_matcher_readonly_access_wok"])
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
        self.assertEqual(policy["hidden_paths"], ["/.screenfs-bench/hidden", "/.screenfs-bench/matcher-heavy/descendant-root", "/.screenfs-bench/matcher-heavy/hidden-0000", "/.screenfs-bench/matcher-heavy/hidden-0001"])
        self.assertEqual(
            policy["visible_paths"],
            [
                "/.screenfs-bench/matcher-heavy/descendant-root/branch-0000/leaf-0000.txt",
                "/.screenfs-bench/matcher-heavy/descendant-root/branch-0001/leaf-0001.txt",
            ],
        )
        self.assertTrue(policy["supports_matcher_hidden_stat_miss"])
        self.assertTrue(policy["supports_matcher_readonly_access_wok"])
        self.assertTrue(policy["supports_matcher_descendant_directory"])
        self.assertIn(
            "matcher_extra_rules appended synthetic hidden/readonly rules plus hidden/visible descendant directory carve-outs for matcher_hidden_stat_miss, matcher_readonly_access_wok, and matcher_descendant_readdir* attribution workloads",
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
            "read-write-surface": bench.READ_WRITE_SURFACE_COMPARABLE_WORKLOADS,
            "read-write-concurrency": bench.READ_WRITE_CONCURRENCY_COMPARABLE_WORKLOADS,
            "metadata-open-path": bench.METADATA_OPEN_PATH_COMPARABLE_WORKLOADS,
            "open-confined-surface": bench.OPEN_CONFINED_SURFACE_COMPARABLE_WORKLOADS,
            "sync-surface": bench.SYNC_SURFACE_COMPARABLE_WORKLOADS,
            "directory-surface": bench.DIRECTORY_SURFACE_COMPARABLE_WORKLOADS,
            "directory-symlink-surface": bench.DIRECTORY_SYMLINK_SURFACE_COMPARABLE_WORKLOADS,
        }
        policy = bench.resolve_policy(make_args())
        for workload_set, expected in cases.items():
            with self.subTest(workload_set=workload_set):
                workloads = bench.resolve_workloads(make_args(workload_set=workload_set), policy)
                self.assertEqual(workloads["comparable"], expected)
                self.assertEqual(workloads["screenfs_only"], [])
                self.assertEqual(workloads["skipped"], [])

    def test_mutation_invalidation_named_set_selects_pinned_workloads(self) -> None:
        policy = bench.resolve_policy(make_args())
        workloads = bench.resolve_workloads(make_args(workload_set="mutation-invalidation"), policy)

        self.assertEqual(workloads["comparable"], [])
        self.assertEqual(workloads["screenfs_only"], bench.MUTATION_INVALIDATION_SCREENFS_ONLY_WORKLOADS)
        self.assertEqual(workloads["skipped"], [])

    def test_policy_heavy_matrix_skips_matcher_without_extra_rules(self) -> None:
        args = make_args(workload_set="policy-heavy-matrix")
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], ["metadata_lookup", "metadata_getattr", "metadata_access"])
        self.assertEqual(workloads["screenfs_only"], [])
        self.assertEqual(
            workloads["skipped"],
            [
                {"name": "matcher_hidden_stat_miss", "reason": "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"},
                {"name": "matcher_readonly_access_wok", "reason": "matcher_readonly_access_wok requires --matcher-extra-rules > 0"},
            ],
        )

    def test_policy_heavy_matrix_includes_matcher_when_rules_exist(self) -> None:
        args = make_args(workload_set="policy-heavy-matrix", matcher_extra_rules=32)
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], ["metadata_lookup", "metadata_getattr", "metadata_access"])
        self.assertEqual(workloads["screenfs_only"], ["matcher_hidden_stat_miss", "matcher_readonly_access_wok"])
        self.assertEqual(workloads["skipped"], [])

    def test_all_named_set_skips_matcher_descendant_workloads_without_rules(self) -> None:
        args = make_args(workload_set="all")
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(
            workloads["skipped"],
            [
                {"name": "matcher_descendant_readdir", "reason": "matcher_descendant_readdir requires --matcher-extra-rules > 0"},
                {"name": "matcher_descendant_readdirplus", "reason": "matcher_descendant_readdirplus requires --matcher-extra-rules > 0"},
                {"name": "matcher_hidden_stat_miss", "reason": "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"},
            ],
        )
        self.assertNotIn("matcher_descendant_readdir", workloads["comparable"])
        self.assertNotIn("matcher_descendant_readdirplus", workloads["comparable"])

    def test_all_named_set_includes_matcher_descendant_workloads_when_rules_exist(self) -> None:
        args = make_args(workload_set="all", matcher_extra_rules=32)
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], bench.WORKLOAD_SETS["all"]["comparable"])
        self.assertEqual(workloads["screenfs_only"], bench.DEFAULT_SCREENFS_ONLY_WORKLOADS)
        self.assertEqual(workloads["skipped"], [])
        self.assertTrue(
            all(name in workloads["comparable"] for name in bench.MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS)
        )

    def test_matcher_descendant_named_set_requires_matcher_rules(self) -> None:
        args = make_args(workload_set="matcher-descendant-directory")
        policy = bench.resolve_policy(args)
        with self.assertRaisesRegex(SystemExit, "no workloads selected after applying policy/workload filters"):
            bench.resolve_workloads(args, policy)

    def test_matcher_descendant_named_set_selects_directory_workloads_when_rules_exist(self) -> None:
        args = make_args(workload_set="matcher-descendant-directory", matcher_extra_rules=32)
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["comparable"], bench.MATCHER_DESCENDANT_DIRECTORY_COMPARABLE_WORKLOADS)
        self.assertEqual(workloads["screenfs_only"], [])
        self.assertEqual(workloads["skipped"], [])

    def test_explicit_workloads_preserve_order_and_enable_matcher_when_rules_exist(self) -> None:
        args = make_args(
            matcher_extra_rules=2,
            workload=[
                "rand_read_4k",
                "matcher_hidden_stat_miss",
                "matcher_readonly_access_wok",
                "rand_read_4k",
                "small_open_read_close",
            ],
        )
        policy = bench.resolve_policy(args)
        workloads = bench.resolve_workloads(args, policy)

        self.assertEqual(workloads["mode"], "explicit")
        self.assertIsNone(workloads["requested_set"])
        self.assertEqual(
            workloads["requested_names"],
            ["rand_read_4k", "matcher_hidden_stat_miss", "matcher_readonly_access_wok", "small_open_read_close"],
        )
        self.assertEqual(workloads["comparable"], ["rand_read_4k", "small_open_read_close"])
        self.assertEqual(workloads["screenfs_only"], ["matcher_hidden_stat_miss", "matcher_readonly_access_wok"])
        self.assertEqual(workloads["skipped"], [])

    def test_explicit_incompatible_screenfs_only_workload_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))
        with self.assertRaisesRegex(SystemExit, "matcher_hidden_stat_miss requires --matcher-extra-rules > 0"):
            bench.resolve_workloads(make_args(policy_preset="fast-path-cache-eligible", workload=["matcher_hidden_stat_miss"]), policy)

    def test_explicit_matcher_readonly_access_workload_without_rules_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))
        with self.assertRaisesRegex(SystemExit, "matcher_readonly_access_wok requires --matcher-extra-rules > 0"):
            bench.resolve_workloads(make_args(policy_preset="fast-path-cache-eligible", workload=["matcher_readonly_access_wok"]), policy)

    def test_explicit_hidden_stat_miss_without_hidden_rule_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args(policy_preset="fast-path-cache-eligible"))
        with self.assertRaisesRegex(SystemExit, "hidden_stat_miss requires a hidden /\\.screenfs-bench/hidden rule"):
            bench.resolve_workloads(make_args(policy_preset="fast-path-cache-eligible", workload=["hidden_stat_miss"]), policy)

    def test_explicit_matcher_descendant_workload_without_rules_is_blocked(self) -> None:
        policy = bench.resolve_policy(make_args())
        with self.assertRaisesRegex(SystemExit, "matcher_descendant_readdir requires --matcher-extra-rules > 0"):
            bench.resolve_workloads(make_args(workload=["matcher_descendant_readdir"]), policy)


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
            "parameters": {"iterations": 1, "warmups": 1, "concurrency_workers": 4},
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
        self.assertIn("- concurrency_workers: `4`", markdown)


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
                cleanup = bench.WORKLOADS[workload_name](self.root, self.args, "native")
                if callable(cleanup):
                    cleanup()

        write_root = self.root / ".screenfs-bench"
        self.assertFalse((write_root / "write-native" / "rand-write-4k.bin").exists())
        self.assertFalse((write_root / "write-native" / "sync-write-4k.bin").exists())

    def test_metadata_sync_and_directory_workloads_run_on_small_fixture(self) -> None:
        for workload_name in (
            bench.READ_WRITE_SURFACE_COMPARABLE_WORKLOADS
            + bench.METADATA_OPEN_PATH_COMPARABLE_WORKLOADS
            + bench.OPEN_CONFINED_SURFACE_COMPARABLE_WORKLOADS
            + bench.SYNC_SURFACE_COMPARABLE_WORKLOADS
            + bench.DIRECTORY_SURFACE_COMPARABLE_WORKLOADS
            + bench.DIRECTORY_SYMLINK_SURFACE_COMPARABLE_WORKLOADS
        ):
            with self.subTest(workload=workload_name):
                cleanup = bench.WORKLOADS[workload_name](self.root, self.args, "native")
                if callable(cleanup):
                    cleanup()

    def test_next_candidate_named_sets_execute_selected_workloads(self) -> None:
        for workload_set in (
            "read-write-surface",
            "read-write-concurrency",
            "metadata-open-path",
            "open-confined-surface",
            "sync-surface",
            "directory-surface",
            "directory-symlink-surface",
        ):
            args = make_args(workload_set=workload_set)
            policy = bench.resolve_policy(args)
            workloads = bench.resolve_workloads(args, policy)

            self.assertEqual(workloads["screenfs_only"], [])
            self.assertEqual(workloads["skipped"], [])

            for workload_name in workloads["comparable"]:
                with self.subTest(workload_set=workload_set, workload=workload_name):
                    cleanup = bench.WORKLOADS[workload_name](self.root, args, "native")
                    if callable(cleanup):
                        cleanup()

    def test_read_write_concurrency_workload_uses_per_worker_files_and_cleans_up(self) -> None:
        args = make_args(workload_set="read-write-concurrency", concurrency_workers=3, rand_io_ops=2)
        with tempfile.TemporaryDirectory() as tempdir:
            root = Path(tempdir)
            bench.prepare_fixture(root, args)
            policy = bench.resolve_policy(args)
            workloads = bench.resolve_workloads(args, policy)

            self.assertEqual(workloads["comparable"], ["concurrent_rand_read_write_4k"])
            cleanup = bench.WORKLOADS["concurrent_rand_read_write_4k"](root, args, "native")
            self.assertTrue(callable(cleanup))

            read_root = root / ".screenfs-bench" / "concurrency-read"
            self.assertEqual(
                sorted(path.name for path in read_root.iterdir()),
                ["worker-000000.bin", "worker-000001.bin", "worker-000002.bin"],
            )
            self.assertTrue(
                all(
                    (root / ".screenfs-bench" / "write-native" / f"concurrent-rand-read-write-4k-worker-{index:06d}.bin").exists()
                    for index in range(args.concurrency_workers)
                )
            )

            cleanup()
            self.assertTrue(
                all(
                    not (root / ".screenfs-bench" / "write-native" / f"concurrent-rand-read-write-4k-worker-{index:06d}.bin").exists()
                    for index in range(args.concurrency_workers)
                )
            )

    def test_matcher_descendant_directory_fixture_and_workloads_run(self) -> None:
        args = make_args(matcher_extra_rules=3, workload_set="matcher-descendant-directory")
        with tempfile.TemporaryDirectory() as tempdir:
            root = Path(tempdir)
            bench.prepare_fixture(root, args)
            policy = bench.resolve_policy(args)
            workloads = bench.resolve_workloads(args, policy)

            descendant_root = root / ".screenfs-bench" / "matcher-heavy" / "descendant-root"
            self.assertEqual(sorted(path.name for path in descendant_root.iterdir()), ["branch-0000", "branch-0001", "branch-0002"])

            for workload_name in workloads["comparable"]:
                with self.subTest(workload=workload_name):
                    cleanup = bench.WORKLOADS[workload_name](root, args, "native")
                    if callable(cleanup):
                        cleanup()

    def test_matcher_readonly_access_workload_uses_visible_readonly_fixture(self) -> None:
        args = make_args(matcher_extra_rules=3, matcher_misses=5)
        with tempfile.TemporaryDirectory() as tempdir:
            root = Path(tempdir)
            bench.prepare_fixture(root, args)

            matcher_dir = root / ".screenfs-bench" / "matcher-heavy"
            self.assertEqual(
                sorted(path.name for path in matcher_dir.iterdir() if path.name.startswith("visible-")),
                ["visible-0000", "visible-0001", "visible-0002"],
            )
            bench.matcher_readonly_access_wok(root, args, "native")

    def test_matcher_readonly_access_workload_rejects_missing_mounted_fixture(self) -> None:
        args = make_args(matcher_extra_rules=1, matcher_misses=1)
        with tempfile.TemporaryDirectory() as tempdir:
            root = Path(tempdir)
            with self.assertRaisesRegex(RuntimeError, "missing readonly fixture"):
                bench.matcher_readonly_access_wok(root, args, "mounted")

    def test_measure_workload_applies_read_fixture_cache_control_before_each_warmup_and_sample(self) -> None:
        args = make_args(cache_control="posix-fadvise-read-fixture", warmups=2, iterations=3)
        calls: list[list[str]] = []

        def workload(_root: Path, _args: argparse.Namespace, _side: str) -> None:
            return None

        def record(paths: list[Path], stats: dict[str, object]) -> None:
            calls.append([str(path) for path in paths])
            stats["applications"] = int(stats.get("applications", 0)) + 1

        with mock.patch.object(bench, "apply_cache_control_to_files", side_effect=record):
            result = bench.measure_workload(
                "seq_read",
                workload,
                self.root,
                args,
                "native",
                cache_control_source_root=self.root,
            )

        expected = str(self.root / ".screenfs-bench" / "read" / "seq.bin")
        self.assertEqual(calls, [[expected], [expected], [expected], [expected], [expected]])
        self.assertEqual(result["cache_control"]["applications"], 5)
        self.assertEqual(result["cache_control"]["applied_files"], [expected])
        self.assertTrue(result["cache_control"]["timing_applied"])

    def test_measure_workload_uses_source_root_for_mounted_cache_control(self) -> None:
        args = make_args(cache_control="posix-fadvise-read-fixture", warmups=1, iterations=1)
        mount = self.root / "mount-view"
        mount.mkdir()
        calls: list[list[str]] = []

        def workload(_root: Path, _args: argparse.Namespace, _side: str) -> None:
            return None

        def record(paths: list[Path], stats: dict[str, object]) -> None:
            calls.append([str(path) for path in paths])
            stats["applications"] = int(stats.get("applications", 0)) + 1

        with mock.patch.object(bench, "apply_cache_control_to_files", side_effect=record):
            result = bench.measure_workload(
                "seq_read",
                workload,
                mount,
                args,
                "mounted",
                cache_control_source_root=self.root,
            )

        expected = str(self.root / ".screenfs-bench" / "read" / "seq.bin")
        mounted_path = str(mount / ".screenfs-bench" / "read" / "seq.bin")
        self.assertEqual(calls, [[expected], [expected]])
        self.assertEqual(result["cache_control"]["applied_files"], [expected])
        self.assertNotIn(mounted_path, result["cache_control"]["applied_files"])

    def test_select_cache_control_read_fixture_files_excludes_symlinks_non_regular_and_write_outputs(self) -> None:
        read_root = self.root / ".screenfs-bench" / "concurrency-read"
        symlink_path = read_root / "worker-999998.bin"
        os.symlink("worker-000000.bin", symlink_path)
        non_regular_path = read_root / "worker-999999.bin"
        non_regular_path.mkdir()
        duplicate_source = read_root / "worker-000000.bin"
        write_native = self.root / ".screenfs-bench" / "write-native" / "worker-000000.bin"
        write_native.write_bytes(b"native-write")
        write_mounted = self.root / ".screenfs-bench" / "write-mounted" / "worker-000000.bin"
        write_mounted.write_bytes(b"mounted-write")

        with mock.patch.object(
            bench,
            "cache_control_candidate_paths",
            return_value=[
                duplicate_source,
                duplicate_source,
                symlink_path,
                non_regular_path,
                write_native,
                write_mounted,
            ],
        ):
            selected, skipped = bench.select_cache_control_read_fixture_files(
                "concurrent_rand_read_write_4k", self.root
            )

        self.assertTrue(all(path.is_file() for path in selected))
        self.assertTrue(all(path.parent == read_root for path in selected))
        self.assertNotIn(write_native, selected)
        self.assertNotIn(write_mounted, selected)
        skipped_by_path = {item["path"]: item["reason"] for item in skipped}
        self.assertEqual(skipped_by_path[str(symlink_path)], "symlink")
        self.assertEqual(skipped_by_path[str(non_regular_path)], "non-regular")
        self.assertEqual(skipped_by_path[str(duplicate_source)], "duplicate-inode")

    def test_symlink_parent_mutation_workload_leaves_no_children_behind(self) -> None:
        bench.symlink_parent_mkdir_rmdir(self.root, self.args, "native")

        real_dir = self.root / ".screenfs-bench" / "symlink-parent" / "real"
        self.assertEqual([path.name for path in real_dir.iterdir()], ["pinned-sibling.txt"])

    def test_pinned_symlink_parent_mutation_workload_leaves_no_children_behind(self) -> None:
        bench.pinned_symlink_parent_mkdir_rmdir(self.root, self.args, "native")

        real_dir = self.root / ".screenfs-bench" / "symlink-parent" / "real"
        self.assertEqual([path.name for path in real_dir.iterdir()], ["pinned-sibling.txt"])

    def test_subtree_rename_cached_unrelated_returns_cleanup_for_source_tree(self) -> None:
        cleanup = bench.subtree_rename_cached_unrelated(self.root, self.args, "native")

        fixture_root = self.root / ".screenfs-bench" / "invalidation-tree"
        self.assertFalse((fixture_root / "source").exists())
        self.assertTrue((fixture_root / "target" / "child.txt").is_file())
        self.assertIsNotNone(cleanup)
        assert cleanup is not None
        cleanup()
        self.assertTrue((fixture_root / "source" / "child.txt").is_file())
        self.assertFalse((fixture_root / "target").exists())


if __name__ == "__main__":
    unittest.main()
