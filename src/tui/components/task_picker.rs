use std::path::Path;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::{model::Task, tui::components::relative_dir};

#[derive(Default)]
pub(crate) struct TaskPicker {
    query: String,
    searching: bool,
    state: ListState,
}

impl TaskPicker {
    pub(crate) fn is_searching(&self) -> bool {
        self.searching
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent, tasks: &[Task]) -> Option<Task> {
        if self.searching {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => self.searching = false,
                KeyCode::Backspace => {
                    self.query.pop();
                    self.select_first();
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.query.clear();
                    self.select_first();
                }
                KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.query.push(c);
                    self.select_first();
                }
                _ => {}
            }
            return None;
        }

        match key.code {
            KeyCode::Char('/') => self.searching = true,
            KeyCode::Char('j') | KeyCode::Down => self.move_down(tasks),
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_down(tasks)
            }
            KeyCode::Char('k') | KeyCode::Up => self.move_up(),
            KeyCode::Enter => return self.selected_task(tasks).cloned(),
            _ => {}
        }
        None
    }

    pub(crate) fn selected_task<'a>(&self, tasks: &'a [Task]) -> Option<&'a Task> {
        let selected = self.state.selected().unwrap_or_default();
        filtered_tasks(tasks, &self.query).get(selected).copied()
    }

    pub(crate) fn render(&mut self, frame: &mut Frame, area: Rect, root: &Path, tasks: &[Task]) {
        let block = Block::bordered().title(" Select Task ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let rows = Layout::vertical([Constraint::Length(2), Constraint::Min(1)]).split(inner);
        let cursor = if self.searching { "_" } else { "" };
        frame.render_widget(
            Paragraph::new(format!("Filter[/]: {}{cursor}", self.query)).block(
                Block::new()
                    .borders(Borders::BOTTOM)
                    .border_style(Color::DarkGray),
            ),
            rows[0],
        );
        let filtered = filtered_tasks(tasks, &self.query);
        let items = filtered
            .iter()
            .map(|task| {
                ListItem::new(Line::from(vec![
                    format!("({})", relative_dir(root, &task.cwd).display()).dark_gray(),
                    format!(" [{}] {}  ", task.provider, task.name).into(),
                    task.command_line().into(),
                ]))
            })
            .collect::<Vec<_>>();
        self.normalize_selection(items.len());
        frame.render_stateful_widget(
            List::new(items)
                .highlight_symbol("> ")
                .highlight_style(Style::new().add_modifier(Modifier::BOLD))
                .scroll_padding(2),
            rows[1],
            &mut self.state,
        );
    }

    fn select_first(&mut self) {
        self.state.select(Some(0));
    }

    fn move_down(&mut self, tasks: &[Task]) {
        let last = filtered_tasks(tasks, &self.query).len().saturating_sub(1);
        let selected = self.state.selected().unwrap_or_default();
        self.state.select(Some((selected + 1).min(last)));
    }

    fn move_up(&mut self) {
        let selected = self.state.selected().unwrap_or_default();
        self.state.select(Some(selected.saturating_sub(1)));
    }

    fn normalize_selection(&mut self, item_count: usize) {
        let selection = (!item_count.eq(&0)).then(|| {
            self.state
                .selected()
                .unwrap_or_default()
                .min(item_count.saturating_sub(1))
        });
        self.state.select(selection);
    }
}

fn filtered_tasks<'a>(tasks: &'a [Task], query: &str) -> Vec<&'a Task> {
    tasks
        .iter()
        .filter(|task| {
            query.is_empty()
                || task.name.contains(query)
                || task.provider.contains(query)
                || task.cwd.to_string_lossy().contains(query)
                || task.command.contains(query)
        })
        .collect()
}
