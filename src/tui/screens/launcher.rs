use std::path::Path;

use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, ListItem},
};

use crate::{
    model::Task,
    runner::Runner,
    tui::components::{TaskPicker, render_task_detail, run_line},
};

#[derive(Default)]
pub(super) struct Launcher {
    task_picker: TaskPicker,
}

impl Launcher {
    pub(super) fn captures_input(&self) -> bool {
        self.task_picker.is_searching()
    }

    pub(super) fn handle_key(&mut self, key: KeyEvent, tasks: &[Task]) -> Option<Task> {
        self.task_picker.handle_key(key, tasks)
    }

    pub(super) fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        root: &Path,
        tasks: &[Task],
        runner: &Runner,
    ) {
        let rows =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
        let columns = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[1]);
        self.task_picker.render(frame, rows[0], root, tasks);
        render_runs(frame, columns[0], root, runner);
        render_task_detail(
            frame,
            columns[1],
            root,
            self.task_picker.selected_task(tasks),
        );
    }
}

fn render_runs(frame: &mut Frame, area: Rect, root: &Path, runner: &Runner) {
    let items = runner
        .runs
        .iter()
        .rev()
        .map(|run| ListItem::new(run_line(run.status, &run.task, root)))
        .collect::<Vec<_>>();
    frame.render_widget(
        List::new(items).block(Block::bordered().title(" Runs ")),
        area,
    );
}
