mod mise;
mod npm;
mod uv;

use std::path::Path;

use crate::model::Workspace;

pub fn discover(path: &Path) -> Vec<Workspace> {
    [
        npm::workspace(path),
        mise::workspace(path),
        uv::workspace(path),
    ]
    .into_iter()
    .flatten()
    .collect()
}
