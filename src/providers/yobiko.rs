use std::{fs, io::ErrorKind, path::Path};

use toml::Value;

use crate::model::{Diagnostic, Task, Workspace};

pub fn workspace(path: &Path) -> Result<Option<Workspace>, Diagnostic> {
    let manifest_path = path.join("yobiko.toml");
    let source = match fs::read_to_string(&manifest_path) {
        Ok(source) => source,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(diagnostic(&manifest_path, error.to_string())),
    };
    let manifest: Value =
        toml::from_str(&source).map_err(|error| diagnostic(&manifest_path, error.to_string()))?;
    let root = manifest
        .as_table()
        .ok_or_else(|| diagnostic(&manifest_path, "manifest must be a TOML table"))?;
    if let Some(key) = root.keys().find(|key| key.as_str() != "tasks") {
        return Err(diagnostic(
            &manifest_path,
            format!("unknown top-level key `{key}`"),
        ));
    }
    let Some(tasks) = root.get("tasks") else {
        return Ok(None);
    };
    let tasks = tasks
        .as_table()
        .ok_or_else(|| diagnostic(&manifest_path, "`tasks` must be a table"))?;
    let mut parsed = Vec::with_capacity(tasks.len());
    for (name, definition) in tasks {
        let definition = definition.as_table().ok_or_else(|| {
            diagnostic(
                &manifest_path,
                format!("task `{name}` must be defined as a table"),
            )
        })?;
        if let Some(key) = definition.keys().find(|key| key.as_str() != "run") {
            return Err(diagnostic(
                &manifest_path,
                format!("unknown key `{key}` in task `{name}`"),
            ));
        }
        let run = definition
            .get("run")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                diagnostic(
                    &manifest_path,
                    format!("task `{name}` requires a string `run` value"),
                )
            })?;
        if run.trim().is_empty() {
            return Err(diagnostic(
                &manifest_path,
                format!("task `{name}` has an empty `run` value"),
            ));
        }
        parsed.push(Task {
            name: name.clone(),
            provider: "yobiko",
            cwd: path.to_path_buf(),
            command: "sh".into(),
            args: vec!["-c".into(), run.into()],
            content: Some(run.into()),
        });
    }
    parsed.sort_by(|left, right| left.name.cmp(&right.name));
    if parsed.is_empty() {
        return Ok(None);
    }
    Ok(Some(Workspace {
        dir: path.to_path_buf(),
        provider: "yobiko",
        tasks: parsed,
    }))
}

fn diagnostic(path: &Path, message: impl Into<String>) -> Diagnostic {
    Diagnostic {
        path: path.to_path_buf(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn temp_dir() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "yobiko-native-provider-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn loads_shell_tasks_alphabetically() {
        let path = temp_dir();
        fs::write(
            path.join("yobiko.toml"),
            "[tasks.zed]\nrun = \"printf z\"\n[tasks.alpha]\nrun = \"cargo test && cargo fmt\"\n",
        )
        .unwrap();
        let workspace = workspace(&path).unwrap().unwrap();
        assert_eq!(workspace.provider, "yobiko");
        assert_eq!(workspace.tasks[0].name, "alpha");
        assert_eq!(workspace.tasks[0].command, "sh");
        assert_eq!(workspace.tasks[0].args, ["-c", "cargo test && cargo fmt"]);
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn accepts_empty_manifests_without_a_workspace() {
        let path = temp_dir();
        fs::write(path.join("yobiko.toml"), "").unwrap();
        assert!(workspace(&path).unwrap().is_none());
        fs::write(path.join("yobiko.toml"), "[tasks]\n").unwrap();
        assert!(workspace(&path).unwrap().is_none());
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rejects_the_entire_manifest_for_invalid_tasks() {
        let path = temp_dir();
        fs::write(
            path.join("yobiko.toml"),
            "[tasks.valid]\nrun = \"true\"\n[tasks.invalid]\nruns = \"false\"\n",
        )
        .unwrap();
        let error = workspace(&path).unwrap_err();
        assert!(error.message.contains("unknown key `runs`"));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn rejects_empty_commands_and_unknown_top_level_keys() {
        let path = temp_dir();
        fs::write(path.join("yobiko.toml"), "[tasks.test]\nrun = \"  \"\n").unwrap();
        assert!(
            workspace(&path)
                .unwrap_err()
                .message
                .contains("empty `run`")
        );

        fs::write(path.join("yobiko.toml"), "version = 1\n").unwrap();
        assert!(
            workspace(&path)
                .unwrap_err()
                .message
                .contains("unknown top-level key `version`")
        );
        fs::remove_dir_all(path).unwrap();
    }
}
