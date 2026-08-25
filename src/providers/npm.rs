use std::{fs, path::Path};

use serde_json::Value;

use crate::model::{Task, Workspace};

pub fn workspace(path: &Path) -> Option<Workspace> {
    let package: Value =
        serde_json::from_str(&fs::read_to_string(path.join("package.json")).ok()?).ok()?;
    let scripts = package.get("scripts")?.as_object()?;
    let command = if path.join("pnpm-lock.yaml").exists() {
        "pnpm"
    } else if path.join("yarn.lock").exists() {
        "yarn"
    } else {
        "npm"
    };
    let tasks = scripts
        .iter()
        .map(|(name, content)| Task {
            name: name.clone(),
            cwd: path.to_path_buf(),
            command: command.into(),
            args: vec!["run".into(), name.clone()],
            content: content.as_str().map(str::to_owned),
        })
        .collect();
    Some(Workspace {
        dir: path.to_path_buf(),
        provider: "npm",
        tasks,
    })
}
