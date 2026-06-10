use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliArgs {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub readonly: bool,
    pub hide_rules: Vec<String>,
    pub readonly_rules: Vec<String>,
}

impl CliArgs {
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
        let mut readonly = false;
        let mut hide_rules = Vec::new();
        let mut readonly_rules = Vec::new();
        let mut i = 0;
        while i < args.len() {
            let arg = args[i].to_string_lossy();
            match arg.as_ref() {
                "--readonly" => {
                    readonly = true;
                    i += 1;
                }
                "--hide" => {
                    let Some(value) = args.get(i + 1) else {
                        return Err(Self::missing_value("--hide"));
                    };
                    if value.to_string_lossy().starts_with('-') {
                        return Err(Self::missing_value("--hide"));
                    }
                    hide_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--readonly-rule" => {
                    let Some(value) = args.get(i + 1) else {
                        return Err(Self::missing_value("--readonly-rule"));
                    };
                    if value.to_string_lossy().starts_with('-') {
                        return Err(Self::missing_value("--readonly-rule"));
                    }
                    readonly_rules.push(value.to_string_lossy().into_owned());
                    i += 2;
                }
                "--help" | "-h" => return Err(Self::usage()),
                other if other.starts_with('-') => return Err(Self::unknown_option(other)),
                _ => {
                    positionals.push(PathBuf::from(&args[i]));
                    i += 1;
                }
            }
        }

        match positionals.len() {
            0 => Err(Self::missing_required_arguments(
                "<source-root> <mount-root>",
            )),
            1 => Err(Self::missing_required_argument("<mount-root>")),
            2 => Ok(Self {
                source_root: positionals.remove(0),
                mount_root: positionals.remove(0),
                readonly,
                hide_rules,
                readonly_rules,
            }),
            _ => Err(Self::unexpected_argument(&positionals[2])),
        }
    }

    pub fn usage() -> String {
        "usage: holefs <source-root> <mount-root> [--readonly] [--hide <pattern> ...] [--readonly-rule <pattern> ...]".to_string()
    }

    fn missing_required_arguments(args: &str) -> String {
        format!("missing required arguments: {args}\n{}", Self::usage())
    }

    fn missing_required_argument(arg: &str) -> String {
        format!("missing required argument: {arg}\n{}", Self::usage())
    }

    fn missing_value(option: &str) -> String {
        format!("missing value for {option}\n{}", Self::usage())
    }

    fn unknown_option(option: &str) -> String {
        format!("unknown option: {option}\n{}", Self::usage())
    }

    fn unexpected_argument(arg: &Path) -> String {
        format!(
            "unexpected positional argument: {}\n{}",
            arg.display(),
            Self::usage()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_positionals_after_binary_name() {
        let args =
            CliArgs::parse_from(["/tmp/target/debug/holefs", "/", "/tmp/holefs-root"]).unwrap();
        assert_eq!(args.source_root, PathBuf::from("/"));
        assert_eq!(args.mount_root, PathBuf::from("/tmp/holefs-root"));
        assert!(!args.readonly);
        assert!(args.hide_rules.is_empty());
        assert!(args.readonly_rules.is_empty());
    }

    #[test]
    fn parses_readonly_and_repeated_hide_and_readonly_rules() {
        let args = CliArgs::parse_from([
            "holefs",
            "--readonly",
            "/",
            "--hide",
            "/home/me/.ssh",
            "/tmp/holefs-root",
            "--readonly-rule",
            "/var/log",
            "--hide",
            "**/*.pem",
            "--readonly-rule",
            "**/*.lock",
        ])
        .unwrap();
        assert_eq!(args.source_root, PathBuf::from("/"));
        assert_eq!(args.mount_root, PathBuf::from("/tmp/holefs-root"));
        assert!(args.readonly);
        assert_eq!(args.hide_rules, vec!["/home/me/.ssh", "**/*.pem"]);
        assert_eq!(args.readonly_rules, vec!["/var/log", "**/*.lock"]);
    }

    #[test]
    fn reports_missing_required_positionals() {
        let err = CliArgs::parse_from(["holefs"]).unwrap_err();
        assert!(err.contains("missing required arguments: <source-root> <mount-root>"));

        let err = CliArgs::parse_from(["holefs", "/"]).unwrap_err();
        assert!(err.contains("missing required argument: <mount-root>"));
    }

    #[test]
    fn reports_unknown_and_missing_option_arguments() {
        let err = CliArgs::parse_from(["holefs", "/", "/mnt", "--bad"]).unwrap_err();
        assert!(err.contains("unknown option: --bad"));

        let err = CliArgs::parse_from(["holefs", "/", "/mnt", "--hide"]).unwrap_err();
        assert!(err.contains("missing value for --hide"));

        let err = CliArgs::parse_from(["holefs", "/", "/mnt", "--hide", "--readonly"]).unwrap_err();
        assert!(err.contains("missing value for --hide"));

        let err = CliArgs::parse_from(["holefs", "/", "/mnt", "--readonly-rule"]).unwrap_err();
        assert!(err.contains("missing value for --readonly-rule"));

        let err =
            CliArgs::parse_from(["holefs", "/", "/mnt", "--readonly-rule", "--hide"]).unwrap_err();
        assert!(err.contains("missing value for --readonly-rule"));
    }

    #[test]
    fn reports_unexpected_extra_positionals() {
        let err = CliArgs::parse_from(["holefs", "/", "/mnt", "/extra"]).unwrap_err();
        assert!(err.contains("unexpected positional argument: /extra"));
    }
}
