use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::{
    runner::Runner,
    tui::{Action, components::run_line},
};

#[derive(Default)]
pub(crate) struct History {
    runs: ListState,
}

impl History {
    pub(crate) fn handle_key(&mut self, key: KeyEvent, runner: &Runner) -> Option<Action> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.move_down(runner.runs.len()),
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_down(runner.runs.len())
            }
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Char('r') => {
                return self
                    .selected_index(runner.runs.len())
                    .and_then(|index| runner.runs.get(index))
                    .map(|run| Action::Run(run.task.clone()));
            }
            KeyCode::Char('x') => {
                return self.selected_index(runner.runs.len()).map(Action::Kill);
            }
            _ => {}
        }
        None
    }

    pub(crate) fn render(&mut self, frame: &mut Frame, area: Rect, root: &Path, runner: &Runner) {
        let columns = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);
        let items = runner
            .runs
            .iter()
            .rev()
            .map(|run| ListItem::new(run_line(run.status, &run.task, root)))
            .collect::<Vec<_>>();
        self.normalize_selection(items.len());
        frame.render_stateful_widget(
            List::new(items)
                .block(Block::bordered().title(" Runs "))
                .highlight_symbol("> ")
                .highlight_style(Modifier::BOLD),
            columns[0],
            &mut self.runs,
        );
        let output = self
            .selected_index(runner.runs.len())
            .and_then(|index| runner.runs.get(index))
            .map(|run| run.output.as_str())
            .unwrap_or("");
        frame.render_widget(
            Paragraph::new(output)
                .wrap(Wrap { trim: false })
                .block(Block::bordered().title(" Output ")),
            columns[1],
        );
    }

    fn selected_index(&self, run_count: usize) -> Option<usize> {
        self.runs
            .selected()
            .and_then(|selected| run_count.checked_sub(selected + 1))
    }

    fn move_down(&mut self, run_count: usize) {
        let selected = self.runs.selected().unwrap_or_default();
        self.runs
            .select(Some((selected + 1).min(run_count.saturating_sub(1))));
    }

    fn move_up(&mut self) {
        let selected = self.runs.selected().unwrap_or_default();
        self.runs.select(Some(selected.saturating_sub(1)));
    }

    fn normalize_selection(&mut self, run_count: usize) {
        let selection = (run_count != 0).then(|| {
            self.runs
                .selected()
                .unwrap_or_default()
                .min(run_count.saturating_sub(1))
        });
        self.runs.select(selection);
    }
}
