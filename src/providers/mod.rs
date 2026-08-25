mod cargo;
mod docker_compose;
mod go;
mod mise;
mod npm;
mod uv;

use std::path::Path;

use crate::model::Workspace;

pub fn discover(path: &Path) -> Vec<Workspace> {
    [
        cargo::workspace(path),
        docker_compose::workspace(path),
        go::workspace(path),
        npm::workspace(path),
        mise::workspace(path),
        uv::workspace(path),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    #[test]
    fn discovers_compose_alongside_other_providers() {
        let path = std::env::temp_dir().join(format!(
            "yobiko-provider-discovery-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&path).unwrap();
        fs::write(path.join("compose.yaml"), "services: {}\n").unwrap();
        fs::write(path.join("package.json"), r#"{"scripts":{"test":"ok"}}"#).unwrap();

        let providers = discover(&path)
            .into_iter()
            .map(|workspace| workspace.provider)
            .collect::<Vec<_>>();
        assert_eq!(providers, ["docker-compose", "npm"]);
        fs::remove_dir_all(path).unwrap();
    }
}
