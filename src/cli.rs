use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutabilityFamily {
    SelectiveReadonly,
    ReadonlyRootAllowwrite,
}

impl MutabilityFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelectiveReadonly => "selective-readonly",
            Self::ReadonlyRootAllowwrite => "readonly-root-allowwrite",
        }
    }

    fn parse(raw: &str) -> Result<Self, String> {
        match raw {
            "selective-readonly" => Ok(Self::SelectiveReadonly),
            "readonly-root-allowwrite" => Ok(Self::ReadonlyRootAllowwrite),
            _ => Err(format!(
                "invalid value for --policy-family: {raw} (expected selective-readonly or readonly-root-allowwrite)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub hide_rules: Vec<String>,
    pub readonly_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchArgs {
    pub cli: CliArgs,
    pub config_path: Option<PathBuf>,
    pub policy_family: Option<MutabilityFamily>,
    pub allow_write_rules: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MutabilitySurface {
    Cli,
    Config,
}

impl MutabilitySurface {
    fn family_label(self) -> &'static str {
        match self {
            Self::Cli => "--policy-family",
            Self::Config => "mutability.family",
        }
    }

    fn readonly_label(self) -> &'static str {
        match self {
            Self::Cli => "--readonly-rule",
            Self::Config => "mutability.readonly_rules",
        }
    }

    fn allow_write_label(self) -> &'static str {
        match self {
            Self::Cli => "--allow-write",
            Self::Config => "mutability.allow_write",
        }
    }
}

impl CliArgs {
    #[cfg(test)]
    pub(crate) fn parse_from<I, S>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        Ok(LaunchArgs::parse_from(args)?.cli)
    }

    pub(crate) fn usage() -> String {
        LaunchArgs::usage()
    }

    fn error_with_usage(message: impl Into<String>) -> String {
        format!(
            "{}\n{}\nTry 'screenfs --help' for detailed usage.",
            message.into(),
            Self::usage()
        )
    }

    fn missing_required_arguments(args: &str) -> String {
        Self::error_with_usage(format!("missing required arguments: {args}"))
    }

    fn missing_required_argument(arg: &str) -> String {
        Self::error_with_usage(format!("missing required argument: {arg}"))
    }

    fn missing_value(option: &str) -> String {
        Self::error_with_usage(format!("missing value for {option}"))
    }

    fn unknown_option(option: &str) -> String {
        Self::error_with_usage(format!("unknown option: {option}"))
    }

    fn unexpected_argument(arg: &Path) -> String {
        Self::error_with_usage(format!("unexpected positional argument: {}", arg.display()))
    }

    fn option_value<'a>(
        args: &'a [OsString],
        index: usize,
        option: &str,
    ) -> Result<&'a OsString, String> {
        let Some(value) = args.get(index + 1) else {
            return Err(Self::missing_value(option));
        };
        if value.to_string_lossy().starts_with('-') {
            return Err(Self::missing_value(option));
        }
        Ok(value)
    }
}

impl LaunchArgs {
    const USAGE_LINE: &str = "usage: screenfs <source-root> <mount-root> [--config <path>] [--hide <pattern> ...] [--policy-family <selective-readonly|readonly-root-allowwrite>] [--readonly-rule <pattern> ...] [--allow-write <pattern> ...]";

    pub fn wants_help<I, S>(args: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        args.into_iter().skip(1).any(|arg| {
            let arg: OsString = arg.into();
            matches!(arg.to_string_lossy().as_ref(), "--help" | "-h")
        })
    }

    pub fn parse_from<I, S>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let mut args = args.into_iter().map(Into::into).collect::<Vec<_>>();
        if !args.is_empty() {
            args.remove(0);
        }

        let mut positionals = Vec::new();
        let mut hide_rules = Vec::new();
        let mut readonly_rules = Vec::new();
        let mut allow_write_rules = Vec::new();
        let mut config_path = None;
        let mut policy_family = None;
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].to_string_lossy();
            match arg.as_ref() {
                "--hide" => {
                    let value = CliArgs::option_value(&args, i, "--hide")?;
                    hide_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--readonly-rule" => {
                    let value = CliArgs::option_value(&args, i, "--readonly-rule")?;
                    readonly_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--allow-write" => {
                    let value = CliArgs::option_value(&args, i, "--allow-write")?;
                    allow_write_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--config" => {
                    let value = CliArgs::option_value(&args, i, "--config")?;
                    config_path = Some(PathBuf::from(value));
                    i += 2;
                }
                "--policy-family" => {
                    let value = CliArgs::option_value(&args, i, "--policy-family")?;
                    policy_family = Some(MutabilityFamily::parse(&value.to_string_lossy())?);
                    i += 2;
                }
                "--help" | "-h" => return Err(Self::usage()),
                other if other.starts_with('-') => return Err(CliArgs::unknown_option(other)),
                _ => {
                    positionals.push(PathBuf::from(&args[i]));
                    i += 1;
                }
            }
        }

        let cli = match positionals.len() {
            0 => {
                return Err(CliArgs::missing_required_arguments(
                    "<source-root> <mount-root>",
                ));
            }
            1 => return Err(CliArgs::missing_required_argument("<mount-root>")),
            2 => CliArgs {
                source_root: positionals.remove(0),
                mount_root: positionals.remove(0),
                hide_rules,
                readonly_rules,
            },
            _ => return Err(CliArgs::unexpected_argument(&positionals[2])),
        };

        let launch_args = Self {
            cli,
            config_path,
            policy_family,
            allow_write_rules,
        };
        launch_args.validate_policy_surface()?;
        Ok(launch_args)
    }

    fn validate_policy_surface(&self) -> Result<(), String> {
        if self.has_future_mutability_cli() {
            validate_future_mutability_surface(
                MutabilitySurface::Cli,
                self.policy_family,
                &self.cli.readonly_rules,
                &self.allow_write_rules,
            )?;
        }
        Ok(())
    }

    pub fn usage() -> String {
        Self::USAGE_LINE.to_string()
    }

    pub fn help() -> String {
        "Create a non-root FUSE whole-root view that hides selected paths and applies one mutability policy family per mount.

Usage:
  screenfs <SOURCE_ROOT> <MOUNT_ROOT> [OPTIONS]

Arguments:
  <SOURCE_ROOT>  Backing filesystem root to mirror, typically /
  <MOUNT_ROOT>   Existing directory where the ScreenFS view is mounted

Options:
      --config <PATH>            Load YAML config
                                 CLI mutability options replace the config mutability block
      --hide <PATTERN>           Hide a path or supported glob
                                 Repeatable; hidden paths resolve as ENOENT
      --policy-family <FAMILY>   Select mutability family
                                 [possible values: selective-readonly, readonly-root-allowwrite]
      --readonly-rule <PATTERN>  Mark matching visible paths read-only (EROFS)
                                 Repeatable; primary for selective-readonly, nested re-block for readonly-root-allowwrite
      --allow-write <PATTERN>    Re-enable writes for matching paths
                                 Repeatable; primary for readonly-root-allowwrite, nested carve-out for selective-readonly
  -h, --help                     Show this help text

Mutability families:
  selective-readonly       Default writable view; matching --readonly-rule paths become read-only
  readonly-root-allowwrite Default read-only view; matching --allow-write paths become writable

Rules and precedence:
  - Hidden paths win first: hidden entries stay ENOENT even if a mutability rule also matches.
  - --readonly-rule and --allow-write together require explicit --policy-family and a valid nested ancestor relationship.
  - If --policy-family is omitted, --allow-write implies readonly-root-allowwrite; otherwise the default is selective-readonly.
  - With no CLI mutability options, config mutability.family supplies the family; otherwise CLI replaces config mutability settings.

Examples:
  screenfs / /tmp/screenfs-root --hide /home/me/.ssh --hide '**/*.pem'
  screenfs / /tmp/screenfs-root --policy-family selective-readonly --readonly-rule /etc/ssh
  screenfs / /tmp/screenfs-root --policy-family readonly-root-allowwrite --allow-write /tmp
  screenfs / /tmp/screenfs-root --config screenfs.yaml

  # screenfs.yaml
  mutability:
    family: selective-readonly
    readonly_rules:
      - /etc/ssh

Notes:
  - Run as a non-root user.
  - <MOUNT_ROOT> must already exist.
  - Supported glob inputs share the same normalization contract across --hide, --readonly-rule, and --allow-write.
"
        .to_string()
    }

    pub fn has_future_mutability_options(
        policy_family: Option<MutabilityFamily>,
        readonly_rules: &[String],
        allow_write_rules: &[String],
    ) -> bool {
        policy_family.is_some() || !readonly_rules.is_empty() || !allow_write_rules.is_empty()
    }

    pub fn has_future_mutability_cli(&self) -> bool {
        Self::has_future_mutability_options(
            self.policy_family,
            &self.cli.readonly_rules,
            &self.allow_write_rules,
        )
    }
}

pub(crate) fn validate_future_mutability_surface(
    surface: MutabilitySurface,
    policy_family: Option<MutabilityFamily>,
    readonly_rules: &[String],
    allow_write_rules: &[String],
) -> Result<MutabilityFamily, String> {
    let readonly_label = surface.readonly_label();
    let allow_write_label = surface.allow_write_label();
    if !readonly_rules.is_empty() && !allow_write_rules.is_empty() && policy_family.is_none() {
        return Err(format!(
            "{readonly_label} and {allow_write_label} require explicit {}",
            surface.family_label()
        ));
    }

    let effective_family = match policy_family {
        Some(family) => family,
        None if !allow_write_rules.is_empty() => MutabilityFamily::ReadonlyRootAllowwrite,
        None => MutabilityFamily::SelectiveReadonly,
    };
    match effective_family {
        MutabilityFamily::SelectiveReadonly
            if !allow_write_rules.is_empty() && readonly_rules.is_empty() =>
        {
            Err(format!(
                "{allow_write_label} requires an ancestor {readonly_label} with {} selective-readonly",
                surface.family_label()
            ))
        }
        MutabilityFamily::ReadonlyRootAllowwrite
            if !readonly_rules.is_empty() && allow_write_rules.is_empty() =>
        {
            Err(format!(
                "{readonly_label} requires an ancestor {allow_write_label} with {} readonly-root-allowwrite",
                surface.family_label()
            ))
        }
        _ => Ok(effective_family),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_positionals_after_binary_name() {
        let args =
            LaunchArgs::parse_from(["/tmp/target/debug/screenfs", "/", "/tmp/screenfs-root"])
                .unwrap();
        assert_eq!(args.cli.source_root, PathBuf::from("/"));
        assert_eq!(args.cli.mount_root, PathBuf::from("/tmp/screenfs-root"));
        assert!(args.cli.hide_rules.is_empty());
        assert!(args.cli.readonly_rules.is_empty());
        assert!(args.allow_write_rules.is_empty());
        assert!(args.config_path.is_none());
        assert_eq!(args.policy_family, None);
    }

    #[test]
    fn parses_future_mutability_options_and_config() {
        let args = LaunchArgs::parse_from([
            "screenfs",
            "--config",
            "screenfs.yaml",
            "/",
            "--hide",
            "/home/me/.ssh",
            "/tmp/screenfs-root",
            "--policy-family",
            "readonly-root-allowwrite",
            "--allow-write",
            "/var/tmp",
            "--hide",
            "**/*.pem",
            "--hide",
            "/home/me/.env.*",
            "--allow-write",
            "**/*.lock",
            "--allow-write",
            "/home/me/.env.*",
        ])
        .unwrap();
        assert_eq!(args.cli.source_root, PathBuf::from("/"));
        assert_eq!(args.cli.mount_root, PathBuf::from("/tmp/screenfs-root"));
        assert_eq!(
            args.cli.hide_rules,
            vec!["/home/me/.ssh", "**/*.pem", "/home/me/.env.*"]
        );
        assert!(args.cli.readonly_rules.is_empty());
        assert_eq!(
            args.allow_write_rules,
            vec!["/var/tmp", "**/*.lock", "/home/me/.env.*"]
        );
        assert_eq!(args.config_path, Some(PathBuf::from("screenfs.yaml")));
        assert_eq!(
            args.policy_family,
            Some(MutabilityFamily::ReadonlyRootAllowwrite)
        );
    }

    #[test]
    fn rejects_removed_readonly_option() {
        let err = CliArgs::parse_from([
            "screenfs",
            "--readonly",
            "--hide",
            "/secret",
            "/",
            "/tmp/screenfs-root",
        ])
        .unwrap_err();
        assert!(err.contains("unknown option: --readonly"));
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
    fn reports_unknown_and_missing_option_arguments() {
        let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--bad"]).unwrap_err();
        assert!(err.contains("unknown option: --bad"));

        let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--hide"]).unwrap_err();
        assert!(err.contains("missing value for --hide"));

        let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--hide", "--config"]).unwrap_err();
        assert!(err.contains("missing value for --hide"));

        let err = CliArgs::parse_from(["screenfs", "/", "/mnt", "--readonly-rule"]).unwrap_err();
        assert!(err.contains("missing value for --readonly-rule"));

        let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--allow-write"]).unwrap_err();
        assert!(err.contains("missing value for --allow-write"));

        let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--config"]).unwrap_err();
        assert!(err.contains("missing value for --config"));

        let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--policy-family"]).unwrap_err();
        assert!(err.contains("missing value for --policy-family"));
    }

    #[test]
    fn reports_mutability_conflicts_and_family_mismatch() {
        let err = LaunchArgs::parse_from([
            "screenfs",
            "/",
            "/mnt",
            "--readonly-rule",
            "/logs",
            "--allow-write",
            "/tmp",
        ])
        .unwrap_err();
        assert!(err.contains("require explicit --policy-family"));

        let args = LaunchArgs::parse_from([
            "screenfs",
            "/",
            "/mnt",
            "--policy-family",
            "selective-readonly",
            "--readonly-rule",
            "/logs",
            "--allow-write",
            "/logs/tmp",
        ])
        .unwrap();
        assert_eq!(
            args.policy_family,
            Some(MutabilityFamily::SelectiveReadonly)
        );
        assert_eq!(args.cli.readonly_rules, vec!["/logs"]);
        assert_eq!(args.allow_write_rules, vec!["/logs/tmp"]);

        let args =
            LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--allow-write", "/tmp"]).unwrap();
        assert_eq!(args.policy_family, None);
        assert_eq!(args.allow_write_rules, vec!["/tmp"]);

        let err = LaunchArgs::parse_from([
            "screenfs",
            "/",
            "/mnt",
            "--policy-family",
            "readonly-root-allowwrite",
            "--readonly-rule",
            "/logs",
        ])
        .unwrap_err();
        assert!(err.contains("requires an ancestor --allow-write"));

        let err = LaunchArgs::parse_from(["screenfs", "/", "/mnt", "--policy-family", "bogus"])
            .unwrap_err();
        assert!(err.contains("invalid value for --policy-family"));
        assert!(!LaunchArgs::usage().contains("[--readonly]"));
    }

    #[test]
    fn infers_readonly_root_allowwrite_from_allow_write_rules() {
        let family = validate_future_mutability_surface(
            MutabilitySurface::Cli,
            None,
            &[],
            &["/tmp".to_string()],
        )
        .unwrap();
        assert_eq!(family, MutabilityFamily::ReadonlyRootAllowwrite);
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
    fn help_text_describes_options_and_examples() {
        let help = LaunchArgs::help();
        assert!(help.contains("Usage:"));
        assert!(help.contains("Arguments:"));
        assert!(help.contains("Options:"));
        assert!(help.contains("Mutability families:"));
        assert!(help.contains("Examples:"));
        assert!(help.contains("[possible values: selective-readonly, readonly-root-allowwrite]"));
        assert!(help.contains("Repeatable; hidden paths resolve as ENOENT"));
        assert!(help.contains(
            "--readonly-rule and --allow-write together require explicit --policy-family"
        ));
        assert!(help.contains("screenfs / /tmp/screenfs-root --hide /home/me/.ssh"));
        assert!(help.contains("screenfs / /tmp/screenfs-root --config screenfs.yaml"));
        assert!(help.contains("# screenfs.yaml"));
        assert!(help.contains("mutability:"));
        assert!(help.contains("readonly_rules:"));
    }
}
