use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::{
    model::{Diagnostic, Workspace},
    providers,
};

pub struct ScanResult {
    pub workspaces: Vec<Workspace>,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn scan(root: PathBuf, depth: usize) -> ScanResult {
    let mut workspaces = Vec::new();
    let mut diagnostics = Vec::new();
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
        let (found, errors) = providers::discover(entry.path());
        workspaces.extend(found);
        diagnostics.extend(errors);
    }
    workspaces.sort_by_key(|workspace| workspace.provider != "yobiko");
    ScanResult {
        workspaces,
        diagnostics,
    }
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
        assert_eq!(found.workspaces.len(), 1);
        assert_eq!(found.workspaces[0].dir, root.join("ignored/kept"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn puts_native_workspaces_before_discovered_providers() {
        let root = std::env::temp_dir().join(format!(
            "yobiko-scan-order-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        fs::write(
            root.join("nested/yobiko.toml"),
            "[tasks.test]\nrun = \"true\"\n",
        )
        .unwrap();

        let found = scan(root.clone(), 10);
        assert_eq!(found.workspaces.len(), 2);
        assert_eq!(found.workspaces[0].provider, "yobiko");
        assert_eq!(found.workspaces[1].provider, "cargo");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn collects_all_native_manifest_diagnostics() {
        let root = std::env::temp_dir().join(format!(
            "yobiko-scan-diagnostics-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("yobiko.toml"), "not toml").unwrap();
        fs::write(root.join("nested/yobiko.toml"), "also not toml").unwrap();

        let found = scan(root.clone(), 10);
        assert!(found.workspaces.is_empty());
        assert_eq!(found.diagnostics.len(), 2);
        fs::remove_dir_all(root).unwrap();
    }
}
