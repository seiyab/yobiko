use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::{model::Workspace, providers};

pub fn scan(root: PathBuf, depth: usize) -> Vec<Workspace> {
    let mut workspaces = Vec::new();
    let walker = WalkBuilder::new(&root)
        .max_depth(Some(depth.saturating_sub(1)))
        .hidden(false)
        .git_ignore(true)
        .require_git(false)
        .git_global(false)
        .git_exclude(false)
        .filter_entry(|entry| !always_ignored(entry.path()))
        .build();
    for entry in walker
        .flatten()
        .filter(|entry| entry.file_type().is_some_and(|t| t.is_dir()))
    {
        workspaces.extend(providers::discover(entry.path()));
    }
    workspaces
}

fn always_ignored(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | "node_modules")
    )
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn respects_gitignore_negation_and_prunes_dependencies() {
        let root = std::env::temp_dir().join(format!(
            "yobiko-scan-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("ignored/kept")).unwrap();
        fs::create_dir_all(root.join("ignored/dropped")).unwrap();
        fs::create_dir_all(root.join("node_modules/package")).unwrap();
        fs::write(root.join(".gitignore"), "ignored/*\n!ignored/kept/\n").unwrap();
        for path in ["ignored/kept", "ignored/dropped", "node_modules/package"] {
            fs::write(
                root.join(path).join("package.json"),
                r#"{"scripts":{"test":"ok"}}"#,
            )
            .unwrap();
        }

        let found = scan(root.clone(), 10);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].dir, root.join("ignored/kept"));
        fs::remove_dir_all(root).unwrap();
    }
}
