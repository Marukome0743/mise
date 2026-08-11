use std::collections::BTreeMap;

use eyre::Result;
use serde::{Deserialize, Serialize};

/// List of built-in provider names that have specialized implementations
pub const BUILTIN_PROVIDERS: &[&str] = &[
    "npm",
    "yarn",
    "pnpm",
    "bun",           // Node.js
    "aube",          // Node.js
    "deno",          // Deno
    "go",            // Go
    "pip",           // Python (requirements.txt)
    "poetry",        // Python (poetry)
    "uv",            // Python (uv)
    "bundler",       // Ruby
    "composer",      // PHP
    "dart",          // Dart
    "flutter",       // Flutter
    "git-submodule", // Git
];

/// Configuration for a deps provider (both built-in and custom)
///
/// Built-in providers have auto-detected sources/outputs and default run commands.
/// Custom providers require explicit sources, outputs, and run.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DepsProviderConfig {
    /// Whether to auto-run this provider before mise x/run (default: false)
    #[serde(default)]
    pub auto: bool,
    /// Command to run when stale (required for custom, optional override for built-in)
    pub run: Option<String>,
    /// Files/patterns to check for changes (required for custom, auto-detected for built-in)
    #[serde(default)]
    pub sources: Vec<String>,
    /// Files/directories that should be newer than sources (required for custom, auto-detected for built-in)
    #[serde(default)]
    pub outputs: Vec<String>,
    /// Environment variables to set
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Working directory
    pub dir: Option<String>,
    /// Optional description
    pub description: Option<String>,
    /// Other deps providers that must complete before this one runs
    #[serde(default)]
    pub depends: Vec<String>,
    /// Timeout for the run command (e.g., "30s", "5m", "1h")
    pub timeout: Option<String>,
}

impl DepsProviderConfig {
    pub fn render_strings(
        &mut self,
        mut render: impl FnMut(&str, &str, bool) -> Result<String>,
    ) -> Result<()> {
        if let Some(run) = &mut self.run {
            *run = render("run", run, false)?;
        }
        for (index, source) in self.sources.iter_mut().enumerate() {
            *source = render(&format!("sources[{index}]"), source, true)?;
        }
        for (index, output) in self.outputs.iter_mut().enumerate() {
            *output = render(&format!("outputs[{index}]"), output, true)?;
        }
        for (key, value) in &mut self.env {
            *value = render(&format!("env.{key}"), value, true)?;
        }
        if let Some(dir) = &mut self.dir {
            *dir = render("dir", dir, true)?;
        }
        if let Some(description) = &mut self.description {
            *description = render("description", description, true)?;
        }
        for (index, dependency) in self.depends.iter_mut().enumerate() {
            *dependency = render(&format!("depends[{index}]"), dependency, true)?;
        }
        if let Some(timeout) = &mut self.timeout {
            *timeout = render("timeout", timeout, true)?;
        }
        Ok(())
    }
}

/// Top-level [deps] configuration section
///
/// All providers are configured at the same level:
/// - `[deps.npm]` - built-in npm provider
/// - `[deps.codegen]` - custom provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DepsConfig {
    /// List of provider IDs to disable at runtime
    #[serde(default)]
    pub disable: Vec<String>,
    /// All provider configurations (both built-in and custom)
    #[serde(flatten)]
    pub providers: BTreeMap<String, DepsProviderConfig>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_strings_covers_all_provider_values() {
        let mut config = DepsProviderConfig {
            run: Some("run".into()),
            sources: vec!["source".into()],
            outputs: vec!["output".into()],
            env: BTreeMap::from([("KEY".into(), "value".into())]),
            dir: Some("dir".into()),
            description: Some("description".into()),
            depends: vec!["dependency".into()],
            timeout: Some("timeout".into()),
            ..Default::default()
        };
        let mut fields = Vec::new();

        config
            .render_strings(|field, value, shell_expand| {
                fields.push((field.to_string(), shell_expand));
                Ok(format!("rendered-{value}"))
            })
            .unwrap();

        assert_eq!(config.run.as_deref(), Some("rendered-run"));
        assert_eq!(config.sources, ["rendered-source"]);
        assert_eq!(config.outputs, ["rendered-output"]);
        assert_eq!(config.env["KEY"], "rendered-value");
        assert_eq!(config.dir.as_deref(), Some("rendered-dir"));
        assert_eq!(config.description.as_deref(), Some("rendered-description"));
        assert_eq!(config.depends, ["rendered-dependency"]);
        assert_eq!(config.timeout.as_deref(), Some("rendered-timeout"));
        assert_eq!(
            fields,
            [
                ("run".into(), false),
                ("sources[0]".into(), true),
                ("outputs[0]".into(), true),
                ("env.KEY".into(), true),
                ("dir".into(), true),
                ("description".into(), true),
                ("depends[0]".into(), true),
                ("timeout".into(), true),
            ]
        );
    }
}
