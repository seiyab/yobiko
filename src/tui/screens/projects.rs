use std::path::Path;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, List, ListItem},
};

use crate::{model::Workspace, tui::components::relative_dir};

#[derive(Default)]
pub(super) struct Projects;

impl Projects {
    pub(super) fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        root: &Path,
        workspaces: &[Workspace],
    ) {
        let items = workspaces
            .iter()
            .map(|workspace| {
                ListItem::new(format!(
                    "{} ({}, {} tasks)",
                    relative_dir(root, &workspace.dir).display(),
                    workspace.provider,
                    workspace.tasks.len()
                ))
            })
            .collect::<Vec<_>>();
        frame.render_widget(
            List::new(items).block(Block::bordered().title(" Projects ")),
            area,
        );
    }
}
