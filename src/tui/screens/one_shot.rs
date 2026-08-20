use std::path::Path;

use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    model::Task,
    tui::components::{TaskPicker, render_task_detail},
};

#[derive(Default)]
pub(crate) struct OneShot {
    task_picker: TaskPicker,
}

impl OneShot {
    pub(crate) fn captures_input(&self) -> bool {
        self.task_picker.is_searching()
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent, tasks: &[Task]) -> Option<Task> {
        self.task_picker.handle_key(key, tasks)
    }

    pub(crate) fn render(&mut self, frame: &mut Frame, area: Rect, root: &Path, tasks: &[Task]) {
        let rows =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
        self.task_picker.render(frame, rows[0], root, tasks);
        render_task_detail(frame, rows[1], root, self.task_picker.selected_task(tasks));
    }
}
