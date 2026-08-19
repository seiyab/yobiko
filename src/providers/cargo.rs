use std::{fs, path::Path};

use toml::Value;

use crate::model::{Task, Workspace};

pub fn workspace(path: &Path) -> Option<Workspace> {
    let _: Value = toml::from_str(&fs::read_to_string(path.join("Cargo.toml")).ok()?).ok()?;
    let tasks = ["check", "test", "build", "fmt", "clippy"]
        .into_iter()
        .map(|command| Task {
            name: command.into(),
            cwd: path.to_path_buf(),
            command: "cargo".into(),
            args: vec![command.into()],
            content: None,
        })
        .collect();
    Some(Workspace {
        dir: path.to_path_buf(),
        provider: "cargo",
        tasks,
    })
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn exposes_standard_cargo_commands() {
        let path = std::env::temp_dir().join(format!(
            "yobiko-cargo-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        fs::write(
            path.join("Cargo.toml"),
            "[package]\nname = \"example\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let workspace = workspace(&path).unwrap();
        assert_eq!(workspace.provider, "cargo");
        assert_eq!(
            workspace
                .tasks
                .iter()
                .map(Task::command_line)
                .collect::<Vec<_>>(),
            [
                "cargo check",
                "cargo test",
                "cargo build",
                "cargo fmt",
                "cargo clippy",
            ]
        );
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn ignores_invalid_manifests() {
        let path = std::env::temp_dir().join(format!(
            "yobiko-cargo-invalid-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        fs::write(path.join("Cargo.toml"), "not toml").unwrap();

        assert!(workspace(&path).is_none());
        fs::remove_dir_all(path).unwrap();
    }
}
