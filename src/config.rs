use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::Deserialize;

use crate::cli::{CliArgs, LaunchArgs, MutabilityDefault, VisibilityDefault};
use crate::matcher::{MatcherScope, PathRuleMatcher, RuleDescriptor, mount_root_internal_prefix};
use crate::path::{RuleNormalizationContext, VirtualPath};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicySource {
    Default,
    Cli,
    Config,
}

impl PolicySource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Cli => "cli",
            Self::Config => "config",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityDecision {
    Visible,
    BridgeVisible,
    Hidden,
}

impl VisibilityDecision {
    pub fn is_readable(self) -> bool {
        matches!(self, Self::Visible | Self::BridgeVisible)
    }

    pub fn is_fully_visible(self) -> bool {
        matches!(self, Self::Visible)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub source_root: PathBuf,
    pub mount_root: PathBuf,
    pub internal_hidden_matcher: PathRuleMatcher,
    pub hidden_matcher: PathRuleMatcher,
    pub visible_matcher: PathRuleMatcher,
    pub readonly_matcher: PathRuleMatcher,
    pub writable_matcher: PathRuleMatcher,
    visibility_default: VisibilityDefault,
    mutability_default: MutabilityDefault,
    visibility_source: PolicySource,
    mutability_source: PolicySource,
    hidden_rule_count: usize,
    visible_rule_count: usize,
    readonly_rule_count: usize,
    writable_rule_count: usize,
    pub attr_ttl: Duration,
    pub entry_ttl: Duration,
}

impl RuntimeConfig {
    #[cfg(test)]
    pub(crate) fn from_cli(args: CliArgs) -> Result<Self, String> {
        Self::from_launch(LaunchArgs {
            cli: args,
            config_path: None,
            visibility_default: None,
            mutability_default: None,
        })
    }

    pub fn from_launch(args: LaunchArgs) -> Result<Self, String> {
        let LaunchArgs {
            cli,
            config_path,
            visibility_default,
            mutability_default,
        } = args;
        let context = RuleNormalizationContext::from_environment(&cli.source_root)?;
        let file_config = match config_path.as_ref() {
            Some(path) => load_file_config(path)?,
            None => FileConfig::default(),
        };

        let mut internal_hidden = Vec::new();
        if let Some(prefix) = mount_root_internal_prefix(&cli.source_root, &cli.mount_root) {
            internal_hidden.push(prefix);
        }

        let visibility = resolve_visibility(&cli, visibility_default, file_config.visibility);
        let mutability = resolve_mutability(&cli, mutability_default, file_config.mutability);

        let internal_hidden_matcher = compile_matcher(
            MatcherScope::Hidden,
            std::iter::empty::<&str>(),
            internal_hidden.clone(),
            &context,
        )?;
        let hidden_policy_matcher = compile_matcher(
            MatcherScope::Hidden,
            visibility.hidden.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;
        let hidden_matcher = compile_matcher(
            MatcherScope::Hidden,
            visibility.hidden.iter().map(String::as_str),
            internal_hidden,
            &context,
        )?;
        let visible_matcher = compile_matcher(
            MatcherScope::Visible,
            visibility.visible.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;
        reject_recursive_visible_bridge_discovery(&visible_matcher)?;
        let readonly_matcher = compile_matcher(
            MatcherScope::Readonly,
            mutability.readonly.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;
        let writable_matcher = compile_matcher(
            MatcherScope::Writable,
            mutability.writable.iter().map(String::as_str),
            Vec::new(),
            &context,
        )?;

        validate_opposite_rules(
            "hidden",
            &hidden_policy_matcher,
            "visible",
            &visible_matcher,
        )?;
        validate_opposite_rules("readonly", &readonly_matcher, "writable", &writable_matcher)?;

        Ok(Self {
            source_root: cli.source_root,
            mount_root: cli.mount_root,
            internal_hidden_matcher,
            hidden_matcher,
            visible_matcher,
            readonly_matcher,
            writable_matcher,
            visibility_default: visibility.default,
            mutability_default: mutability.default,
            visibility_source: visibility.source,
            mutability_source: mutability.source,
            hidden_rule_count: visibility.hidden.len(),
            visible_rule_count: visibility.visible.len(),
            readonly_rule_count: mutability.readonly.len(),
            writable_rule_count: mutability.writable.len(),
            attr_ttl: Duration::ZERO,
            entry_ttl: Duration::ZERO,
        })
    }

    pub fn visibility_decision(&self, path: &VirtualPath) -> VisibilityDecision {
        if self.internal_hidden_matcher.matches_path(path) {
            VisibilityDecision::Hidden
        } else if self.is_visible_by_rules(path) {
            VisibilityDecision::Visible
        } else if self.visible_matcher.may_match_descendant_of(path) {
            VisibilityDecision::BridgeVisible
        } else {
            VisibilityDecision::Hidden
        }
    }

    pub fn is_hidden(&self, path: &VirtualPath) -> bool {
        matches!(self.visibility_decision(path), VisibilityDecision::Hidden)
    }

    pub fn is_visible(&self, path: &VirtualPath) -> bool {
        self.visibility_decision(path).is_readable()
    }

    pub fn is_fully_visible(&self, path: &VirtualPath) -> bool {
        self.visibility_decision(path).is_fully_visible()
    }

    pub fn is_bridge_visible(&self, path: &VirtualPath) -> bool {
        matches!(
            self.visibility_decision(path),
            VisibilityDecision::BridgeVisible
        )
    }

    pub fn entry_is_readable(&self, path: &VirtualPath, is_directory: bool) -> bool {
        match self.visibility_decision(path) {
            VisibilityDecision::Visible => true,
            VisibilityDecision::BridgeVisible => {
                is_directory && self.has_visible_bridge_ancestor(path)
            }
            VisibilityDecision::Hidden => false,
        }
    }

    pub fn visibility_blocks_mutation(&self, path: &VirtualPath, resolved: &VirtualPath) -> bool {
        self.is_bridge_visible(path) || self.is_bridge_visible(resolved)
    }

    pub fn has_visible_bridge_ancestor(&self, path: &VirtualPath) -> bool {
        self.visible_matcher.may_match_descendant_of(path)
    }

    pub fn can_skip_symlink_target_visibility_check(&self) -> bool {
        self.visibility_default == VisibilityDefault::Visible
            && self
                .internal_hidden_matcher
                .can_skip_symlink_target_visibility_check()
            && self
                .hidden_matcher
                .can_skip_symlink_target_visibility_check()
    }

    pub fn is_hidden_symlink_target(&self, link_path: &VirtualPath, raw_target: &OsStr) -> bool {
        link_path
            .resolve_symlink_target(raw_target)
            .is_some_and(|target| !self.is_fully_visible(&target))
    }

    pub fn matches_hidden_rule(&self, path: &VirtualPath) -> bool {
        self.hidden_matcher.matches_path(path)
    }

    pub fn matches_visible_rule(&self, path: &VirtualPath) -> bool {
        self.visible_matcher.matches_path(path)
    }

    pub fn matches_readonly_rule(&self, path: &VirtualPath) -> bool {
        self.readonly_matcher.matches_path(path)
    }

    pub fn matches_writable_rule(&self, path: &VirtualPath) -> bool {
        self.writable_matcher.matches_path(path)
    }

    pub fn is_readonly(&self, path: &VirtualPath) -> bool {
        if self.is_hidden(path) {
            return true;
        }
        match choose_axis(
            self.mutability_default == MutabilityDefault::Readonly,
            self.readonly_matcher.best_descriptor(path),
            self.writable_matcher.best_descriptor(path),
        ) {
            AxisChoice::Negative => true,
            AxisChoice::Positive => false,
        }
    }

    pub fn visibility_default(&self) -> VisibilityDefault {
        self.visibility_default
    }

    pub fn mutability_default(&self) -> MutabilityDefault {
        self.mutability_default
    }

    pub fn visibility_source(&self) -> PolicySource {
        self.visibility_source
    }

    pub fn mutability_source(&self) -> PolicySource {
        self.mutability_source
    }

    pub fn hidden_rule_count(&self) -> usize {
        self.hidden_rule_count
    }

    pub fn visible_rule_count(&self) -> usize {
        self.visible_rule_count
    }

    pub fn readonly_rule_count(&self) -> usize {
        self.readonly_rule_count
    }

    pub fn writable_rule_count(&self) -> usize {
        self.writable_rule_count
    }

    fn is_visible_by_rules(&self, path: &VirtualPath) -> bool {
        match choose_axis(
            self.visibility_default == VisibilityDefault::Hidden,
            self.hidden_matcher.best_descriptor(path),
            self.visible_matcher.best_descriptor(path),
        ) {
            AxisChoice::Negative => false,
            AxisChoice::Positive => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AxisChoice {
    Negative,
    Positive,
}

fn choose_axis(
    default_negative: bool,
    negative: Option<&RuleDescriptor>,
    positive: Option<&RuleDescriptor>,
) -> AxisChoice {
    match (negative, positive) {
        (Some(negative), Some(positive))
            if positive.contains_target_set(negative)
                && !negative.contains_target_set(positive) =>
        {
            AxisChoice::Negative
        }
        (Some(negative), Some(positive))
            if negative.contains_target_set(positive)
                && !positive.contains_target_set(negative) =>
        {
            AxisChoice::Positive
        }
        (Some(negative), Some(positive)) if negative.specificity() > positive.specificity() => {
            AxisChoice::Negative
        }
        (Some(_), Some(_)) => AxisChoice::Positive,
        (Some(_), None) => AxisChoice::Negative,
        (None, Some(_)) => AxisChoice::Positive,
        (None, None) if default_negative => AxisChoice::Negative,
        (None, None) => AxisChoice::Positive,
    }
}

#[derive(Debug)]
struct ResolvedVisibility {
    default: VisibilityDefault,
    source: PolicySource,
    hidden: Vec<String>,
    visible: Vec<String>,
}

#[derive(Debug)]
struct ResolvedMutability {
    default: MutabilityDefault,
    source: PolicySource,
    readonly: Vec<String>,
    writable: Vec<String>,
}

fn resolve_visibility(
    cli: &CliArgs,
    cli_default: Option<VisibilityDefault>,
    config_visibility: Option<VisibilityConfig>,
) -> ResolvedVisibility {
    if cli_default.is_some()
        || !cli.visibility_hidden_rules.is_empty()
        || !cli.visibility_visible_rules.is_empty()
    {
        return ResolvedVisibility {
            default: cli_default.unwrap_or(VisibilityDefault::Visible),
            source: PolicySource::Cli,
            hidden: cli.visibility_hidden_rules.clone(),
            visible: cli.visibility_visible_rules.clone(),
        };
    }

    if let Some(config_visibility) = config_visibility {
        return ResolvedVisibility {
            default: config_visibility
                .default
                .unwrap_or(VisibilityDefault::Visible),
            source: PolicySource::Config,
            hidden: config_visibility.hidden,
            visible: config_visibility.visible,
        };
    }

    ResolvedVisibility {
        default: VisibilityDefault::Visible,
        source: PolicySource::Default,
        hidden: Vec::new(),
        visible: Vec::new(),
    }
}

fn resolve_mutability(
    cli: &CliArgs,
    cli_default: Option<MutabilityDefault>,
    config_mutability: Option<MutabilityConfig>,
) -> ResolvedMutability {
    if cli_default.is_some()
        || !cli.mutability_readonly_rules.is_empty()
        || !cli.mutability_writable_rules.is_empty()
    {
        return ResolvedMutability {
            default: cli_default.unwrap_or(MutabilityDefault::Writable),
            source: PolicySource::Cli,
            readonly: cli.mutability_readonly_rules.clone(),
            writable: cli.mutability_writable_rules.clone(),
        };
    }

    if let Some(config_mutability) = config_mutability {
        return ResolvedMutability {
            default: config_mutability
                .default
                .unwrap_or(MutabilityDefault::Writable),
            source: PolicySource::Config,
            readonly: config_mutability.readonly,
            writable: config_mutability.writable,
        };
    }

    ResolvedMutability {
        default: MutabilityDefault::Writable,
        source: PolicySource::Default,
        readonly: Vec::new(),
        writable: Vec::new(),
    }
}

fn compile_matcher<'a, I>(
    scope: MatcherScope,
    rules: I,
    internal_prefixes: Vec<VirtualPath>,
    context: &RuleNormalizationContext,
) -> Result<PathRuleMatcher, String>
where
    I: IntoIterator<Item = &'a str>,
{
    PathRuleMatcher::compile(scope, rules, internal_prefixes, context)
}

fn reject_recursive_visible_bridge_discovery(visible: &PathRuleMatcher) -> Result<(), String> {
    if visible.has_recursive_bridge_discovery_rule() {
        return Err(
            "invalid visible pattern: recursive visible globs are unsupported because they require recursive bridge discovery; prefer an explicit subtree visible rule such as /dir or /dir/**"
                .to_string(),
        );
    }
    Ok(())
}

fn validate_opposite_rules(
    negative_label: &str,
    negative: &PathRuleMatcher,
    positive_label: &str,
    positive: &PathRuleMatcher,
) -> Result<(), String> {
    for negative_rule in negative.descriptors() {
        for positive_rule in positive.descriptors() {
            if same_conflict_coordinate(negative_rule, positive_rule) {
                return Err(format!(
                    "{negative_label} and {positive_label} rules conflict at the same normalized specificity"
                ));
            }
            if negative_rule.has_unproven_overlap_with(positive_rule) {
                return Err(format!(
                    "{negative_label} and {positive_label} rules have overlapping glob targets without provable containment"
                ));
            }
        }
    }
    Ok(())
}

fn same_conflict_coordinate(left: &RuleDescriptor, right: &RuleDescriptor) -> bool {
    left.specificity() == right.specificity()
        && left.anchor() == right.anchor()
        && (left.contains_target_set(right) || right.contains_target_set(left))
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    #[serde(default)]
    visibility: Option<VisibilityConfig>,
    #[serde(default)]
    mutability: Option<MutabilityConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct VisibilityConfig {
    default: Option<VisibilityDefault>,
    #[serde(default)]
    hidden: Vec<String>,
    #[serde(default)]
    visible: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MutabilityConfig {
    default: Option<MutabilityDefault>,
    #[serde(default)]
    readonly: Vec<String>,
    #[serde(default)]
    writable: Vec<String>,
}

fn load_file_config(path: &Path) -> Result<FileConfig, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|err| format!("failed to read config {}: {err}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(FileConfig::default());
    }
    serde_yaml::from_str(&content)
        .map_err(|err| format!("failed to parse config {}: {err}", path.display()))
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
