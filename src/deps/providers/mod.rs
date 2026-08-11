mod aube;
mod bun;
mod bundler;
mod composer;
mod custom;
mod dart;
mod deno;
mod git_submodule;
mod go;
mod npm;
mod pip;
mod pnpm;
mod poetry;
mod uv;
mod yarn;

pub use aube::AubeDepsProvider;
pub use bun::BunDepsProvider;
pub use bundler::BundlerDepsProvider;
pub use composer::ComposerDepsProvider;
pub use custom::CustomDepsProvider;
pub use dart::DartDepsProvider;
pub use deno::DenoDepsProvider;
pub use git_submodule::GitSubmoduleDepsProvider;
pub use go::GoDepsProvider;
pub use npm::NpmDepsProvider;
pub use pip::PipDepsProvider;
pub use pnpm::PnpmDepsProvider;
pub use poetry::PoetryDepsProvider;
pub use uv::UvDepsProvider;
pub use yarn::YarnDepsProvider;

use std::path::{Path, PathBuf};

use glob::glob;

use crate::deps::rule::DepsProviderConfig;

/// Shared base for all deps providers, holding the id, project root, and config.
/// Provides common implementations for `id` and `is_auto`.
#[derive(Debug)]
pub struct ProviderBase {
    pub(crate) id: String,
    pub(crate) project_root: PathBuf,
    pub(crate) config: DepsProviderConfig,
}

impl ProviderBase {
    pub fn new(id: impl Into<String>, project_root: &Path, config: DepsProviderConfig) -> Self {
        Self {
            id: id.into(),
            project_root: project_root.to_path_buf(),
            config,
        }
    }

    pub fn is_auto(&self) -> bool {
        self.config.auto
    }

    /// Returns the effective root directory for resolving sources/outputs.
    /// When `dir` is set in config, returns `project_root/dir`; otherwise `project_root`.
    pub fn config_root(&self) -> PathBuf {
        match &self.config.dir {
            Some(dir) => self.project_root.join(dir),
            None => self.project_root.clone(),
        }
    }

    pub fn sources(&self, default: Vec<PathBuf>) -> Vec<PathBuf> {
        self.config
            .sources
            .as_deref()
            .map(|patterns| self.resolve_path_patterns(patterns))
            .unwrap_or(default)
    }

    pub fn outputs(&self, default: Vec<PathBuf>) -> Vec<PathBuf> {
        self.config
            .outputs
            .as_deref()
            .map(|patterns| self.resolve_path_patterns(patterns))
            .unwrap_or(default)
    }

    pub fn optional_outputs(&self, default: Vec<PathBuf>) -> Vec<PathBuf> {
        if self.config.outputs.is_some() {
            vec![]
        } else {
            default
        }
    }

    fn resolve_path_patterns(&self, patterns: &[String]) -> Vec<PathBuf> {
        let mut paths = vec![];

        for pattern in patterns {
            let path = PathBuf::from(pattern);
            let full_pattern = if path.is_relative() {
                self.config_root().join(pattern)
            } else {
                path
            };

            if pattern.contains('*') || pattern.contains('{') || pattern.contains('?') {
                if let Ok(entries) = glob(full_pattern.to_string_lossy().as_ref()) {
                    paths.extend(entries.flatten());
                }
            } else {
                paths.push(full_pattern);
            }
        }

        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn path_overrides_distinguish_omitted_explicit_and_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let default_source = tmp.path().join("default.lock");
        let default_output = tmp.path().join("default-output");
        let optional_output = tmp.path().join("optional-output");

        let base = ProviderBase::new("test", tmp.path(), DepsProviderConfig::default());
        assert_eq!(
            base.sources(vec![default_source.clone()]),
            vec![default_source.clone()]
        );
        assert_eq!(
            base.outputs(vec![default_output.clone()]),
            vec![default_output.clone()]
        );
        assert_eq!(
            base.optional_outputs(vec![optional_output.clone()]),
            vec![optional_output]
        );

        let explicit = ProviderBase::new(
            "test",
            tmp.path(),
            DepsProviderConfig {
                sources: Some(vec!["custom.lock".into()]),
                outputs: Some(vec!["custom-output".into()]),
                ..Default::default()
            },
        );
        assert_eq!(
            explicit.sources(vec![default_source]),
            vec![tmp.path().join("custom.lock")]
        );
        assert_eq!(
            explicit.outputs(vec![default_output]),
            vec![tmp.path().join("custom-output")]
        );
        assert!(explicit.optional_outputs(vec![]).is_empty());

        let empty = ProviderBase::new(
            "test",
            tmp.path(),
            DepsProviderConfig {
                sources: Some(vec![]),
                outputs: Some(vec![]),
                ..Default::default()
            },
        );
        assert!(empty.sources(vec![tmp.path().join("ignored")]).is_empty());
        assert!(empty.outputs(vec![tmp.path().join("ignored")]).is_empty());
        assert!(
            empty
                .optional_outputs(vec![tmp.path().join("ignored")])
                .is_empty()
        );
    }

    #[test]
    fn path_overrides_resolve_from_dir_and_expand_globs() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("project");
        let work = root.join("packages/app");
        fs::create_dir_all(work.join("inputs")).unwrap();
        fs::write(work.join("inputs/a.lock"), "a").unwrap();
        fs::write(work.join("inputs/b.lock"), "b").unwrap();
        let absolute = tmp.path().join("absolute-output");

        let base = ProviderBase::new(
            "test",
            &root,
            DepsProviderConfig {
                dir: Some("packages/app".into()),
                sources: Some(vec!["inputs/*.lock".into()]),
                outputs: Some(vec![absolute.to_string_lossy().into_owned()]),
                ..Default::default()
            },
        );

        assert_eq!(
            base.sources(vec![]),
            vec![work.join("inputs/a.lock"), work.join("inputs/b.lock")]
        );
        assert_eq!(base.outputs(vec![]), vec![absolute]);
    }
}
