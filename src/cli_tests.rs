//! Regression tests for CLI parsing, help text, and removed-option rejection.

use super::*;

// Test cases are grouped by the behavior named in each function.
#[test]
fn parses_positionals_after_binary_name() {
    let args =
        LaunchArgs::parse_from(["/tmp/target/debug/screenfs", "/", "/tmp/screenfs-root"]).unwrap();
    assert_eq!(args.cli.source_root, PathBuf::from("/"));
    assert_eq!(args.cli.mount_root, PathBuf::from("/tmp/screenfs-root"));
    assert!(args.cli.visibility_hidden_rules.is_empty());
    assert!(args.cli.visibility_visible_rules.is_empty());
    assert!(args.cli.mutability_readonly_rules.is_empty());
    assert!(args.cli.mutability_writable_rules.is_empty());
    assert!(args.config_path.is_none());
    assert_eq!(args.visibility_default, None);
    assert_eq!(args.mutability_default, None);
    assert!(!args.experimental_writeback_cache);
}

#[test]
fn parses_two_axis_policy_options_and_config() {
    let args = LaunchArgs::parse_from([
        "screenfs",
        "--config",
        "screenfs.yaml",
        "/",
        "--hidden",
        "/home/me/.ssh",
        "/tmp/screenfs-root",
        "--visibility-default",
        "visible",
        "--visible",
        "/home/me/project",
        "--hidden",
        "**/*.pem",
        "--mutability-default",
        "readonly",
        "--writable",
        "/var/tmp",
        "--readonly",
        "**/.git/hooks/**",
        "--experimental-writeback-cache",
    ])
    .unwrap();
    assert_eq!(args.cli.source_root, PathBuf::from("/"));
    assert_eq!(args.cli.mount_root, PathBuf::from("/tmp/screenfs-root"));
    assert_eq!(
        args.cli.visibility_hidden_rules,
        vec!["/home/me/.ssh", "**/*.pem"]
    );
    assert_eq!(args.cli.visibility_visible_rules, vec!["/home/me/project"]);
    assert_eq!(args.cli.mutability_readonly_rules, vec!["**/.git/hooks/**"]);
    assert_eq!(args.cli.mutability_writable_rules, vec!["/var/tmp"]);
    assert_eq!(args.config_path, Some(PathBuf::from("screenfs.yaml")));
    assert_eq!(args.visibility_default, Some(VisibilityDefault::Visible));
    assert_eq!(args.mutability_default, Some(MutabilityDefault::Readonly));
    assert!(args.experimental_writeback_cache);
}

#[test]
fn rejects_removed_policy_options_as_unknown() {
    let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--removed-policy-option", "/tmp"])
        .unwrap_err();
    assert!(err.contains("unknown option: --removed-policy-option"));
    assert!(err.contains("Try 'screenfs --help' for detailed usage."));
}

#[test]
fn reports_missing_required_positionals() {
    let err = CliArgs::parse_from(["screenfs"]).unwrap_err();
    assert!(err.contains("missing required arguments: <source-root> <mount-root>"));

    let err = CliArgs::parse_from(["screenfs", "/"]).unwrap_err();
    assert!(err.contains("missing required argument: <mount-root>"));
}

#[test]
fn compact_usage_marks_repeatable_policy_options_as_repeatable_flags() {
    let usage = CliArgs::usage();
    assert!(usage.contains("[--hidden <pattern>]..."), "{usage}");
    assert!(usage.contains("[--visible <pattern>]..."), "{usage}");
    assert!(usage.contains("[--readonly <pattern>]..."), "{usage}");
    assert!(usage.contains("[--writable <pattern>]..."), "{usage}");
    assert!(
        usage.contains("[--experimental-writeback-cache]"),
        "{usage}"
    );
    assert!(!usage.contains("--hidden <pattern> ..."), "{usage}");
}

#[test]
fn reports_unknown_and_missing_option_arguments() {
    let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--bad"]).unwrap_err();
    assert!(err.contains("unknown option: --bad"));

    for option in [
        "--hidden",
        "--visible",
        "--readonly",
        "--writable",
        "--config",
        "--visibility-default",
        "--mutability-default",
    ] {
        let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", option]).unwrap_err();
        assert!(err.contains(&format!("missing value for {option}")));
    }
}

#[test]
fn rejects_invalid_defaults() {
    let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--visibility-default", "bogus"])
        .unwrap_err();
    assert!(err.contains("invalid value for --visibility-default"));

    let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--mutability-default", "bogus"])
        .unwrap_err();
    assert!(err.contains("invalid value for --mutability-default"));
}

#[test]
fn reports_unexpected_extra_positionals() {
    let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "/extra"]).unwrap_err();
    assert!(err.contains("unexpected positional argument: /extra"));
}

#[test]
fn detects_help_flag() {
    assert!(LaunchArgs::wants_help(["screenfs", "--help"]));
    assert!(LaunchArgs::wants_help(["screenfs", "/", "/mnt", "-h"]));
    assert!(!LaunchArgs::wants_help(["screenfs", "/", "/mnt"]));
}

#[test]
fn help_text_describes_two_axis_policy() {
    let help = LaunchArgs::help();
    assert!(help.contains("Usage:"));
    assert!(help.contains("Policy axes:"));
    assert!(help.contains("--visibility-default <DEFAULT>"));
    assert!(help.contains("--mutability-default <DEFAULT>"));
    assert!(help.contains("--hidden <PATTERN>"));
    assert!(help.contains("--visible <PATTERN>"));
    assert!(help.contains("--readonly <PATTERN>"));
    assert!(help.contains("--writable <PATTERN>"));
    assert!(help.contains("--experimental-writeback-cache"));
    assert!(help.contains("writeback-cache"));
    assert!(help.contains("bridge-visible"));
    assert!(help.contains("visibility:"));
    assert!(help.contains("mutability:"));
    assert!(!help.contains("family"));
    assert!(!help.contains("allow-write"));
}
