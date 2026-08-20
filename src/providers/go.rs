use std::{fs, path::Path};

use crate::model::{Task, Workspace};

const TASKS: [&str; 4] = ["test", "build", "fmt", "vet"];

pub fn workspace(path: &Path) -> Option<Workspace> {
    let manifest = fs::read_to_string(path.join("go.mod")).ok()?;
    manifest
        .lines()
        .any(|line| line.trim_start().starts_with("module "))
        .then(|| Workspace {
            dir: path.to_path_buf(),
            provider: "go",
            tasks: TASKS
                .into_iter()
                .map(|command| Task {
                    name: command.into(),
                    cwd: path.to_path_buf(),
                    command: "go".into(),
                    args: vec![command.into(), "./...".into()],
                    content: None,
                })
                .collect(),
        })
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn temp_project() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "yobiko-go-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn exposes_standard_go_commands() {
        let path = temp_project();
        fs::write(
            path.join("go.mod"),
            "module example.com/project\n\ngo 1.24\n",
        )
        .unwrap();

        let workspace = workspace(&path).unwrap();
        assert_eq!(workspace.provider, "go");
        assert_eq!(
            workspace
                .tasks
                .iter()
                .map(Task::command_line)
                .collect::<Vec<_>>(),
            [
                "go test ./...",
                "go build ./...",
                "go fmt ./...",
                "go vet ./...",
            ]
        );
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn ignores_invalid_go_mod_files() {
        let path = temp_project();
        fs::write(path.join("go.mod"), "go 1.24\n").unwrap();

        assert!(workspace(&path).is_none());
        fs::remove_dir_all(path).unwrap();
    }
}
