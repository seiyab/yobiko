use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    model::Task,
    tui::{
        Action,
        components::{LaunchMode, TaskPicker, render_task_detail},
    },
};

#[derive(Default)]
pub(crate) struct OneShot {
    task_picker: TaskPicker,
}

impl OneShot {
    pub(crate) fn captures_input(&self) -> bool {
        self.task_picker.is_searching()
    }

    pub(crate) fn handle_key(
        &mut self,
        key: KeyEvent,
        root: &Path,
        tasks: &[Task],
    ) -> Option<Action> {
        if !self.task_picker.is_searching() && key.code == KeyCode::Char('e') {
            return self
                .task_picker
                .selected_task(root, tasks)
                .cloned()
                .map(|task| Action::Edit(task, LaunchMode::OneShot));
        }
        self.task_picker
            .handle_key(key, root, tasks)
            .map(Action::SelectOneShot)
    }

    pub(crate) fn render(&mut self, frame: &mut Frame, area: Rect, root: &Path, tasks: &[Task]) {
        let rows =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
        self.task_picker.render(frame, rows[0], root, tasks);
        render_task_detail(
            frame,
            rows[1],
            root,
            self.task_picker.selected_task(root, tasks),
        );
    }
}
