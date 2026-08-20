use std::path::Path;

use crate::model::{Task, Workspace};

const COMPOSE_FILES: [&str; 4] = [
    "compose.yaml",
    "compose.yml",
    "docker-compose.yaml",
    "docker-compose.yml",
];

const TASKS: [(&str, &[&str]); 7] = [
    ("up", &["up"]),
    ("up-detached", &["up", "--wait"]),
    ("down", &["down"]),
    ("build", &["build"]),
    ("pull", &["pull"]),
    ("ps", &["ps"]),
    ("logs", &["logs"]),
];

pub fn workspace(path: &Path) -> Option<Workspace> {
    COMPOSE_FILES
        .iter()
        .any(|file| path.join(file).is_file())
        .then(|| Workspace {
            dir: path.to_path_buf(),
            provider: "docker-compose",
            tasks: TASKS
                .iter()
                .map(|(name, args)| Task {
                    name: (*name).into(),
                    provider: "docker-compose",
                    cwd: path.to_path_buf(),
                    command: "docker".into(),
                    args: std::iter::once("compose".into())
                        .chain(args.iter().map(|arg| (*arg).into()))
                        .collect(),
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

    fn temp_project(label: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "yobiko-docker-compose-{label}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn recognizes_compose_filenames() {
        for file in COMPOSE_FILES {
            let path = temp_project(file);
            fs::write(path.join(file), "services: {}\n").unwrap();

            let workspace = workspace(&path).unwrap();
            assert_eq!(workspace.provider, "docker-compose");
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[test]
    fn exposes_standard_compose_commands() {
        let path = temp_project("commands");
        fs::write(path.join("compose.yaml"), "services: {}\n").unwrap();

        let workspace = workspace(&path).unwrap();
        assert_eq!(
            workspace
                .tasks
                .iter()
                .map(Task::command_line)
                .collect::<Vec<_>>(),
            [
                "docker compose up",
                "docker compose up --wait",
                "docker compose down",
                "docker compose build",
                "docker compose pull",
                "docker compose ps",
                "docker compose logs",
            ]
        );
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn ignores_directories_without_a_compose_file() {
        let path = temp_project("missing");

        assert!(workspace(&path).is_none());
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn creates_one_workspace_when_multiple_compose_files_exist() {
        let path = temp_project("multiple");
        fs::write(path.join("compose.yaml"), "services: {}\n").unwrap();
        fs::write(path.join("docker-compose.yml"), "services: {}\n").unwrap();

        assert_eq!(workspace(&path).unwrap().tasks.len(), TASKS.len());
        fs::remove_dir_all(path).unwrap();
    }
}
