use std::{fs, path::Path};

use toml::Value;

use crate::model::{Task, Workspace};

pub fn workspace(path: &Path) -> Option<Workspace> {
    let config: Value =
        toml::from_str(&fs::read_to_string(path.join("pyproject.toml")).ok()?).ok()?;
    if !path.join("uv.lock").is_file()
        && config.get("tool").and_then(|tool| tool.get("uv")).is_none()
    {
        return None;
    }
    let tasks = vec![Task {
        name: "sync".into(),
        cwd: path.to_path_buf(),
        command: "uv".into(),
        args: vec!["sync".into()],
        content: None,
    }];
    Some(Workspace {
        dir: path.to_path_buf(),
        provider: "uv",
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

    fn temp_project() -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "yobiko-uv-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        path
    }

    #[test]
    fn requires_an_uv_marker() {
        let path = temp_project();
        fs::write(
            path.join("pyproject.toml"),
            "[project]\nname = \"example\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        assert!(workspace(&path).is_none());

        fs::write(path.join("uv.lock"), "").unwrap();
        let workspace = workspace(&path).unwrap();
        assert_eq!(workspace.tasks.len(), 1);
        assert_eq!(workspace.tasks[0].name, "sync");
        assert_eq!(workspace.tasks[0].command_line(), "uv sync");
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn tool_uv_table_is_an_uv_marker() {
        let path = temp_project();
        fs::write(path.join("pyproject.toml"), "[tool.uv]\npackage = false\n").unwrap();

        assert!(workspace(&path).is_some());
        fs::remove_dir_all(path).unwrap();
    }
}
