use std::path::Path;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph, Wrap},
};

use crate::model::Task;

pub(crate) fn render_task_detail(frame: &mut Frame, area: Rect, root: &Path, task: Option<&Task>) {
    let text = task.map_or_else(String::new, |task| {
        format!(
            "directory: {}\nname: {}\ncommand: {}{}",
            relative_dir(root, &task.cwd).display(),
            task.name,
            task.command_line(),
            task.content
                .as_ref()
                .map_or_else(String::new, |content| format!("\ncontent: {content}"))
        )
    });
    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .block(Block::bordered().title(" Detail ")),
        area,
    );
}

pub(crate) fn relative_dir<'a>(root: &Path, dir: &'a Path) -> &'a Path {
    dir.strip_prefix(root)
        .ok()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| if dir == root { Path::new(".") } else { dir })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displayed_directory_is_relative_to_scan_root() {
        let root = Path::new("/projects/example");
        assert_eq!(
            relative_dir(root, Path::new("/projects/example")),
            Path::new(".")
        );
        assert_eq!(
            relative_dir(root, Path::new("/projects/example/crates/app")),
            Path::new("crates/app")
        );
    }
}
