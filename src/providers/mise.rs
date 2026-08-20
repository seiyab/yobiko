use std::{fs, path::Path};

use toml::Value;

use crate::model::{Task, Workspace};

pub fn workspace(path: &Path) -> Option<Workspace> {
    let config: Value = toml::from_str(&fs::read_to_string(path.join("mise.toml")).ok()?).ok()?;
    let tasks = config.get("tasks")?.as_table()?;
    let tasks = tasks
        .iter()
        .map(|(name, definition)| Task {
            name: name.clone(),
            provider: "mise",
            cwd: path.to_path_buf(),
            command: "mise".into(),
            args: vec!["run".into(), name.clone()],
            content: content(definition),
        })
        .collect();
    Some(Workspace {
        dir: path.to_path_buf(),
        provider: "mise",
        tasks,
    })
}

fn content(value: &Value) -> Option<String> {
    if let Some(value) = value.as_str() {
        return Some(value.into());
    }
    if let Some(values) = value.as_array() {
        return Some(
            values
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
    let table = value.as_table()?;
    table.get("run").and_then(content).or_else(|| {
        table
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_owned)
    })
}
