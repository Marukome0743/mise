use crate::Result;
use crate::config::Config;
use crate::file;
use crate::task::Task;
use clap::ValueHint;
use eyre::bail;
use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use xx::file::display_path;

/// Generates shims to run mise tasks
///
/// By default, this will build shims like ./bin/<task>. These can be paired with `mise generate bootstrap`
/// so contributors to a project can execute mise tasks without installing mise into their system.
/// When a parent and nested task both exist, the parent stub is written to `<parent>/_default`.
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment, after_long_help = AFTER_LONG_HELP)]
pub struct TaskStubs {
    /// Directory to create task stubs inside of
    #[clap(long, short, verbatim_doc_comment, default_value="bin", value_hint=ValueHint::DirPath)]
    dir: PathBuf,

    /// Path to a mise bin to use when running the task stub.
    ///
    /// Use `--mise-bin=./bin/mise` to use a mise bin generated from `mise generate bootstrap`
    #[clap(long, short, verbatim_doc_comment, default_value = "mise")]
    mise_bin: PathBuf,
}

impl TaskStubs {
    pub async fn run(self) -> eyre::Result<()> {
        let config = Config::get().await?;
        let tasks = config.tasks().await?;
        let task_paths = tasks.values().map(Task::name_to_path).collect::<Vec<_>>();
        let paths = resolve_stub_paths(&self.dir, &task_paths)?;
        let stubs = tasks
            .values()
            .zip(task_paths)
            .zip(paths)
            .map(|((task, legacy_path), path)| {
                Ok(TaskStub {
                    task,
                    legacy_path: self.dir.join(legacy_path),
                    path,
                    output: self.generate(task)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let migrations = validate_stub_paths(&self.dir, &stubs)?;

        for path in migrations {
            file::remove_file(path)?;
        }
        for stub in stubs {
            if let Some(parent) = stub.path.parent() {
                file::create_dir_all(parent)?;
            }
            file::write(&stub.path, &stub.output)?;
            file::make_executable(&stub.path)?;
            miseprintln!("Wrote to {}", display_path(&stub.path));
        }
        Ok(())
    }

    fn generate(&self, task: &Task) -> Result<String> {
        let mise_bin = self.mise_bin.to_string_lossy();
        let mise_bin = shell_words::quote(&mise_bin);
        let display_name = &task.display_name;
        let script = format!(
            r#"
#!/bin/sh
exec {mise_bin} run {display_name} "$@"
"#
        );
        Ok(script.trim().to_string())
    }
}

struct TaskStub<'a> {
    task: &'a Task,
    legacy_path: PathBuf,
    path: PathBuf,
    output: String,
}

fn resolve_stub_paths(dir: &Path, task_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let base_paths = task_paths
        .iter()
        .map(|path| dir.join(path))
        .collect::<Vec<_>>();
    let paths = base_paths
        .iter()
        .enumerate()
        .map(|(index, path)| {
            if base_paths.iter().enumerate().any(|(other_index, other)| {
                index != other_index && other != path && other.starts_with(path)
            }) {
                path.join("_default")
            } else {
                path.clone()
            }
        })
        .collect::<Vec<_>>();

    let mut seen = HashSet::new();
    for path in &paths {
        if !seen.insert(path) {
            bail!(
                "multiple tasks map to task stub path {}",
                display_path(path)
            );
        }
    }
    Ok(paths)
}

fn validate_stub_paths(dir: &Path, stubs: &[TaskStub<'_>]) -> Result<Vec<PathBuf>> {
    let mut migrations = HashSet::new();
    for stub in stubs.iter().filter(|stub| stub.legacy_path != stub.path) {
        match fs::symlink_metadata(&stub.legacy_path) {
            Ok(metadata) if metadata.file_type().is_file() => {
                if file::read_to_string(&stub.legacy_path)? != stub.output {
                    bail!(
                        "cannot create nested task stubs because {} is not the generated stub for task {}",
                        display_path(&stub.legacy_path),
                        stub.task.display_name
                    );
                }
                migrations.insert(stub.legacy_path.clone());
            }
            Ok(metadata) if metadata.file_type().is_dir() => {}
            Ok(_) => bail!(
                "cannot create nested task stubs because {} is not a directory",
                display_path(&stub.legacy_path)
            ),
            Err(err) if matches!(err.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {}
            Err(err) => return Err(err.into()),
        }
    }

    for stub in stubs {
        match fs::symlink_metadata(&stub.path) {
            Ok(metadata) if metadata.file_type().is_dir() => bail!(
                "cannot write task stub because {} is a directory",
                display_path(&stub.path)
            ),
            Ok(_) => {}
            Err(err) if matches!(err.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {}
            Err(err) => return Err(err.into()),
        }
        for parent in stub.path.ancestors().skip(1) {
            match fs::symlink_metadata(parent) {
                Ok(metadata) if metadata.file_type().is_dir() || migrations.contains(parent) => {}
                Ok(_) => bail!(
                    "cannot create task stub directory because {} is not a directory",
                    display_path(parent)
                ),
                Err(err)
                    if matches!(err.kind(), ErrorKind::NotFound | ErrorKind::NotADirectory) => {}
                Err(err) => return Err(err.into()),
            }
            if parent == dir {
                break;
            }
        }
    }
    Ok(migrations.into_iter().collect())
}

static AFTER_LONG_HELP: &str = color_print::cstr!(
    r#"<bold><underline>Examples:</underline></bold>

    $ <bold>mise tasks add test -- echo 'running tests'</bold>
    $ <bold>mise generate task-stubs</bold>
    $ <bold>./bin/test</bold>
    running tests
"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_parent_and_nested_task_paths() {
        let paths = resolve_stub_paths(
            Path::new("bin"),
            &[
                PathBuf::from("foo"),
                PathBuf::from("foo/bar"),
                PathBuf::from("foo/bar/baz"),
                PathBuf::from("foobar"),
            ],
        )
        .unwrap();

        assert_eq!(
            paths,
            [
                PathBuf::from("bin/foo/_default"),
                PathBuf::from("bin/foo/bar/_default"),
                PathBuf::from("bin/foo/bar/baz"),
                PathBuf::from("bin/foobar"),
            ]
        );
    }

    #[test]
    fn rejects_duplicate_resolved_paths() {
        let err = resolve_stub_paths(
            Path::new("bin"),
            &[PathBuf::from("foo"), PathBuf::from("foo/_default")],
        )
        .unwrap_err();

        assert!(err.to_string().contains("bin/foo/_default"));
    }
}
