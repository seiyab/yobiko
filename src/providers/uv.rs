use std::{fs, path::Path};

use toml::Value;

use crate::model::{Task, Workspace};

pub fn workspace(path: &Path) -> Option<Workspace> {
    let config: Value =
        toml::from_str(&fs::read_to_string(path.join("pyproject.toml")).ok()?).ok()?;
    let scripts = config.get("project")?.get("scripts")?.as_table()?;
    let tasks = scripts
        .iter()
        .filter_map(|(name, entry)| {
            Some(Task {
                name: name.clone(),
                cwd: path.to_path_buf(),
                command: "uv".into(),
                args: vec!["run".into(), name.clone()],
                content: Some(entry.as_str()?.to_owned()),
            })
        })
        .collect();
    Some(Workspace {
        dir: path.to_path_buf(),
        provider: "uv",
        tasks,
    })
}
