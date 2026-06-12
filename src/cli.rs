use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VisibilityDefault {
    Visible,
    Hidden,
}

impl VisibilityDefault {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Hidden => "hidden",
        }
    }

    fn parse(option: &str, raw: &str) -> Result<Self, String> {
        match raw {
            "visible" => Ok(Self::Visible),
            "hidden" => Ok(Self::Hidden),
            _ => Err(format!(
                "invalid value for {option}: {raw} (expected visible or hidden)"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MutabilityDefault {
    Writable,
    Readonly,
}

impl MutabilityDefault {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Writable => "writable",
            Self::Readonly => "readonly",
        }
    }

    fn parse(option: &str, raw: &str) -> Result<Self, String> {
        match raw {
            "writable" => Ok(Self::Writable),
            "readonly" => Ok(Self::Readonly),
            _ => Err(format!(
                "invalid value for {option}: {raw} (expected writable or readonly)"
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub visibility_hidden_rules: Vec<String>,
    pub visibility_visible_rules: Vec<String>,
    pub mutability_readonly_rules: Vec<String>,
    pub mutability_writable_rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchArgs {
    pub cli: CliArgs,
    pub config_path: Option<PathBuf>,
    pub visibility_default: Option<VisibilityDefault>,
    pub mutability_default: Option<MutabilityDefault>,
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
    const USAGE_LINE: &str = "usage: screenfs <source-root> <mount-root> [--config <path>] [--visibility-default <visible|hidden>] [--hidden <pattern> ...] [--visible <pattern> ...] [--mutability-default <writable|readonly>] [--readonly <pattern> ...] [--writable <pattern> ...]";

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
        let mut visibility_hidden_rules = Vec::new();
        let mut visibility_visible_rules = Vec::new();
        let mut mutability_readonly_rules = Vec::new();
        let mut mutability_writable_rules = Vec::new();
        let mut config_path = None;
        let mut visibility_default = None;
        let mut mutability_default = None;
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].to_string_lossy();
            match arg.as_ref() {
                "--hidden" => {
                    let value = CliArgs::option_value(&args, i, "--hidden")?;
                    visibility_hidden_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--visible" => {
                    let value = CliArgs::option_value(&args, i, "--visible")?;
                    visibility_visible_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--readonly" => {
                    let value = CliArgs::option_value(&args, i, "--readonly")?;
                    mutability_readonly_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--writable" => {
                    let value = CliArgs::option_value(&args, i, "--writable")?;
                    mutability_writable_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--visibility-default" => {
                    let value = CliArgs::option_value(&args, i, "--visibility-default")?;
                    visibility_default = Some(VisibilityDefault::parse(
                        "--visibility-default",
                        &value.to_string_lossy(),
                    )?);
                    i += 2;
                }
                "--mutability-default" => {
                    let value = CliArgs::option_value(&args, i, "--mutability-default")?;
                    mutability_default = Some(MutabilityDefault::parse(
                        "--mutability-default",
                        &value.to_string_lossy(),
                    )?);
                    i += 2;
                }
                "--config" => {
                    let value = CliArgs::option_value(&args, i, "--config")?;
                    config_path = Some(PathBuf::from(value));
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
                visibility_hidden_rules,
                visibility_visible_rules,
                mutability_readonly_rules,
                mutability_writable_rules,
            },
            _ => return Err(CliArgs::unexpected_argument(&positionals[2])),
        };

        Ok(Self {
            cli,
            config_path,
            visibility_default,
            mutability_default,
        })
    }

    pub fn usage() -> String {
        Self::USAGE_LINE.to_string()
    }

    pub fn help() -> String {
        "Create a non-root FUSE whole-root view with independent visibility and mutability policy axes.

Usage:
  screenfs <SOURCE_ROOT> <MOUNT_ROOT> [OPTIONS]

Arguments:
  <SOURCE_ROOT>  Backing filesystem root to mirror, typically /
  <MOUNT_ROOT>   Existing directory where the ScreenFS view is mounted

Options:
      --config <PATH>                    Load YAML config
                                         CLI visibility options replace the config visibility block;
                                         CLI mutability options replace the config mutability block
      --visibility-default <DEFAULT>     Default visibility for paths with no matching rule
                                         [possible values: visible, hidden]
      --hidden <PATTERN>                 Hide a path or supported glob as ENOENT
                                         Repeatable; opposite of --visible
      --visible <PATTERN>                Make a hidden/default-hidden path visible again
                                         Repeatable; ancestors are bridge-visible for traversal/listing
      --mutability-default <DEFAULT>     Default mutability for visible paths with no matching rule
                                         [possible values: writable, readonly]
      --readonly <PATTERN>               Return EROFS for matching visible paths
                                         Repeatable; opposite of --writable
      --writable <PATTERN>               Allow mutation for matching visible paths
                                         Repeatable; opposite of --readonly
  -h, --help                             Show this help text

Policy axes:
  visibility.default  visible | hidden; hidden results are ENOENT before mutability is checked
  visibility.hidden   paths/globs to hide
  visibility.visible  paths/globs to reveal; ancestors are bridge-visible when needed
  mutability.default  writable | readonly
  mutability.readonly paths/globs to make read-only
  mutability.writable paths/globs to make writable

Rules and precedence:
  - Visibility is evaluated first. Hidden paths stay ENOENT even if writable or readonly also matches.
  - Within each axis, the most-specific matching rule wins; no match uses that axis default.
  - Same-axis opposite rules with the same normalized anchor and specificity are invalid.
  - Bridge-visible ancestors support stat/traversal/listing only so visible carve-outs are reachable without exposing siblings.
  - Supported glob inputs share the same normalization contract across hidden, visible, readonly, and writable rules.

Examples:
  screenfs / /tmp/screenfs-root --hidden /home/me/.ssh --hidden '/**/*.pem'
  screenfs / /tmp/screenfs-root --hidden /home --visible /home/me/project
  screenfs / /tmp/screenfs-root --mutability-default readonly --writable . --readonly '**/.git/hooks/**'
  screenfs / /tmp/screenfs-root --config screenfs.yaml

  # screenfs.yaml
  visibility:
    default: visible
    hidden:
      - ~/.ssh
      - '/**/*.pem'
    visible:
      - ./fixtures/public.pem
  mutability:
    default: readonly
    writable:
      - .
      - /tmp
    readonly:
      - '**/.git/hooks/**'

Notes:
  - Run as a non-root user.
  - <MOUNT_ROOT> must already exist.
"
        .to_string()
    }

    pub fn has_visibility_cli(&self) -> bool {
        self.visibility_default.is_some()
            || !self.cli.visibility_hidden_rules.is_empty()
            || !self.cli.visibility_visible_rules.is_empty()
    }

    pub fn has_mutability_cli(&self) -> bool {
        self.mutability_default.is_some()
            || !self.cli.mutability_readonly_rules.is_empty()
            || !self.cli.mutability_writable_rules.is_empty()
    }
}

#[cfg(test)]
#[path = "cli_tests.rs"]
mod tests;
